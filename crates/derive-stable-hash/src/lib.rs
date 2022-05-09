use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, spanned::Spanned, Data, DeriveInput, Error, Index, Result, TypeParam};

/// Derive the `StableHash` trait for a type
///
/// - Structs will have each field hashed in the order they're declared in
/// - Enums will have their discriminant hashed before hashing all of the
///   current variant's fields in the order they're declared in
///
/// # Remarks
///
/// Unions are not supported and will cause a compile error if the derive
/// is used on them.  
/// If the given struct or enum has generic type parameters a `StableHash`
/// bound will be automatically added for each of the to the generated
/// `StableHash` implementation.
///
#[proc_macro_derive(StableHash)]
pub fn derive_stable_hash(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(tokens as DeriveInput);

    derive_stable_hash_inner(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_stable_hash_inner(mut input: DeriveInput) -> Result<TokenStream> {
    let innards: TokenStream = match &input.data {
        // Hash all fields of the struct
        Data::Struct(strct) => strct
            .fields
            .iter()
            .map(|field| {
                let (ident, ty) = (&field.ident, &field.ty);
                quote! {
                    <#ty as ::stable_hash::StableHash>::stable_hash(&self.#ident, state);
                }
            })
            .collect(),

        Data::Enum(enm) => {
            // Hash all the fields of each enum variant
            let variants = enm.variants.iter().map(|variant| {
                let (fields, hashes): (Vec<_>, Vec<_>) = variant
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(idx, field)| {
                        let (field_pattern, ident) = if let Some(ident) = &field.ident {
                            (quote! { #ident: ref #ident }, ident.clone())
                        } else {
                            let idx = Index {
                                index: idx
                                    .try_into()
                                    .expect("more than 2^32 fields on enum variant"),
                                span: field.span(),
                            };
                            let ident = format_ident!("_{}", idx);

                            (quote! { #idx: ref #ident }, ident)
                        };

                        let ty = &field.ty;
                        let hash = quote! {
                            <#ty as ::stable_hash::StableHash>::stable_hash(#ident, state);
                        };

                        (field_pattern, hash)
                    })
                    .unzip();

                let variant = &variant.ident;
                quote! {
                    Self::#variant { #(#fields),* } => { #(#hashes)* }
                }
            });

            // Hash the enum's discriminant and then the data within it
            quote! {
                <::core::mem::Discriminant<_> as ::stable_hash::StableHash>::stable_hash(
                    &::core::mem::discriminant(self),
                    state,
                );

                // We dereference `self` so that uninhabited enums
                // can be properly matched on
                match *self {
                    #(#variants)*
                }
            }
        }

        // We don't support deriving for unions
        Data::Union(_) => {
            return Err(Error::new(
                input.span(),
                "Deriving StableHash is only supported for structs and enums, not unions",
            ));
        }
    };

    // Add StableHash bounds to all generics on the struct/enum
    let clauses = input
        .generics
        .type_params()
        .map(
            |TypeParam {
                 ident: ty_param, ..
             }|
             -> syn::WherePredicate {
                parse_quote! { #ty_param : ::stable_hash::StableHash }
            },
        )
        .collect::<Vec<_>>();
    input
        .generics
        .make_where_clause()
        .predicates
        .extend(clauses);

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::stable_hash::StableHash for #ident #ty_generics
            #where_clause
        {
            fn stable_hash<H>(&self, state: &mut H)
            where
                H: ::stable_hash::StableHasher,
            {
                #innards
            }
        }
    })
}
