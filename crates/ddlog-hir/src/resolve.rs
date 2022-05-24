use ddlog_diagnostics::{Diagnostic, FileId, IStr, Interner, Label, Span};
use ddlog_syntax::ast::{
    nodes::Root,
    prefixed::nodes::{AstItem, AstStructFields},
    AstNode,
};
use ddlog_utils::OptionExt;
use std::{
    borrow::Cow,
    collections::{BTreeMap, VecDeque},
};

macro_rules! declare_primitives {
    ($($primitive:ident),* $(,)?) => {
        const LENGTH: usize = [$({ let $primitive = (); }),*].len();

        struct Primitives {
            $($primitive: IStr,)*
        }

        impl Primitives {
            pub fn new(interner: &Interner) -> Self {
                Self {
                    $($primitive: interner.get_or_intern_static(stringify!($primitive)),)*
                }
            }

            pub fn iter(&self) -> core::array::IntoIter<IStr, LENGTH> {
                [$(self.$primitive),*].into_iter()
            }
        }
    };
}

declare_primitives! {
    str,
    bool,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    usize,
    isize,
}

pub struct Resolver<'a> {
    file: FileId,
    interner: &'a Interner,
    primitives: Primitives,
}

impl<'a> Resolver<'a> {
    pub fn new(file: FileId, interner: &'a Interner) -> Self {
        Self {
            file,
            interner,
            primitives: Primitives::new(interner),
        }
    }

    pub fn resolve(&mut self, root: &Root) -> Vec<Diagnostic> {
        let (mut errors, mut items) = (Vec::new(), BTreeMap::new());
        for name in self.primitives.iter() {
            items
                .insert(name, Symbol::Primitive { name })
                .debug_unwrap_none();
        }

        // let mut needs_resolution = VecDeque::new();

        for item in root.items() {
            match &*item {
                AstItem::ClauseDef(_) => todo!(),
                AstItem::ConstDef(_) => todo!(),
                AstItem::EnumDef(_) => todo!(),
                AstItem::ImplBlock(_) => todo!(),
                AstItem::RelationDef(_) => todo!(),
                AstItem::TypeAlias(_) => todo!(),
                AstItem::UseDef(_) => todo!(),

                AstItem::FunctionDef(func) => {
                    if let Some(name) = func.ident() {
                        let span = Span::from_text_range(func.signature_span(true), self.file);

                        let symbol = Symbol::Fn { name, span };
                        if let Some(shadowed) = items.insert(name, symbol) {
                            // Shadowing primitives is ok for user types
                            if !shadowed.is_primitive() {
                                let shadowed_span = shadowed
                                    .span()
                                    .expect("everything should have a span except primitives");

                                let error = Diagnostic::error()
                                    .with_message(format!(
                                        "re-declared {} when it was already in scope",
                                        self.interner.resolve(name),
                                    ))
                                    .with_message_span(span)
                                    .with_label(
                                        Label::primary(shadowed_span)
                                            .with_message("initially declared here"),
                                    );
                                errors.push(error);
                            }
                        }
                    }

                    // TODO: Even if there's an error within the current struct's name
                    // we can still do symbol resolution on its fields
                }

                AstItem::StructDef(strct) => {
                    if let Some(name) = strct.ident() {
                        let span = Span::from_text_range(strct.signature_span(), self.file);
                        // let fields: Vec<_> = strct
                        //     .fields()
                        //     .map(|fields| {
                        //         debug_assert!(fields.len() <= u16::MAX as usize);
                        //
                        //         match &*fields {
                        //             AstStructFields::BracketedStructFields(fields) => fields
                        //                 .fields()
                        //                 .map(|field| {
                        //                     let ty =
                        //
                        //                     (
                        //                         field.ident().map(FieldIdx::Ident),
                        //                         ty,
                        //                     )
                        //                 })
                        //                 .collect(),
                        //
                        //             AstStructFields::TupleStructFields(fields) => fields
                        //                 .fields()
                        //                 .enumerate()
                        //                 .map(|(idx, field)| {
                        //                     (
                        //                         Some(FieldIdx::Idx(idx as u16)),
                        //                         todo!(),
                        //                     )
                        //                 })
                        //                 .collect(),
                        //         }
                        //     })
                        //     .unwrap_or_default();

                        let symbol = Symbol::Struct { name, span };
                        if let Some(shadowed) = items.insert(name, symbol) {
                            // Shadowing primitives is ok for user types
                            if !shadowed.is_primitive() {
                                let shadowed_span = shadowed
                                    .span()
                                    .expect("everything should have a span except primitives");

                                let error = Diagnostic::error()
                                    .with_message(format!(
                                        "re-declared {} when it was already in scope",
                                        self.interner.resolve(name),
                                    ))
                                    .with_message_span(span)
                                    .with_label(
                                        Label::primary(shadowed_span)
                                            .with_message("initially declared here"),
                                    );
                                errors.push(error);
                            }
                        }
                    }

                    // TODO: Even if there's an error within the current struct's name
                    // we can still do symbol resolution on its fields
                }
            }
        }

        dbg!(&items);
        errors
    }
}

#[derive(Debug)]
enum Symbol {
    Primitive { name: IStr },
    Fn { name: IStr, span: Span },
    Struct { name: IStr, span: Span },
}

#[derive(Debug)]
enum FieldIdx {
    Idx(u16),
    Ident(IStr),
}

impl Symbol {
    /// Returns `true` if the symbol is a [`Primitive`].
    ///
    /// [`Primitive`]: Symbol::Primitive
    #[must_use]
    const fn is_primitive(&self) -> bool {
        matches!(self, Self::Primitive { .. })
    }

    #[must_use]
    const fn span(&self) -> Option<Span> {
        match *self {
            Self::Primitive { .. } => None,
            Self::Fn { span, .. } | Self::Struct { span, .. } => Some(span),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::resolve::Resolver;
    use ddlog_diagnostics::{FileId, Interner};
    use ddlog_syntax::NodeCache;

    const SOURCE: &str = "
struct Bar {
    baz: usize,
}

fn foo(bar: Bar) {}
    ";

    #[test]
    fn resolve_test() {
        let interner = Interner::new();
        let file = FileId::new(interner.get_or_intern_static("tests/resolve_test.ddlog"));
        let cache = NodeCache::from_interner(interner);
        let (parsed, cache) = ddlog_syntax::parse(file, SOURCE, cache);

        let mut resolver = Resolver::new(file, cache.interner());
        let errors = resolver.resolve(parsed.root());
        dbg!(errors);
    }
}
