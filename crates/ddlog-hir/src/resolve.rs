use crate::hir::Item;
use ddlog_diagnostics::{Diagnostic, FileId, IStr, Interner, Label};
use ddlog_syntax::{
    ast::{
        prefixed::nodes::{AstGenerics, AstItem, AstPath, AstStructFields},
        AstNode, AstToken,
    },
    queries::SourceDatabase,
};
use ddlog_utils::ArcSlice;
use imbl::OrdMap;
use std::{
    fmt::{self, Debug},
    num::{NonZeroU16, NonZeroU32},
};

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: SymbolDatabase {
    fn lower_file(&self, file: FileId) -> (ArcSlice<Item>, ArcSlice<Diagnostic>);

    fn lower_item(&self, item: ItemId) -> (Option<Item>, ArcSlice<Diagnostic>);
}

fn lower_file(db: &dyn HirDatabase, file: FileId) -> (ArcSlice<Item>, ArcSlice<Diagnostic>) {
    let items = db.top_level_items(file);
    let (mut hir_items, mut errors) = (Vec::with_capacity(items.len()), Vec::new());

    for &item in items.iter() {
        let (item, item_errors) = db.lower_item(item);

        if let Some(item) = item {
            hir_items.push(item);
        }
        errors.extend(item_errors.iter().cloned());
    }

    (ArcSlice::from(hir_items), ArcSlice::from(errors))
}

enum Type {
    Enum,
    Aggregate,
}

fn lower_item(db: &dyn HirDatabase, item: ItemId) -> (Option<Item>, ArcSlice<Diagnostic>) {
    let file = db.item_file(item);
    let (mut symbols, mut errors) = (db.top_level_symbols(file), Vec::new());

    // The generics declared by the item shadow any types by the same name
    symbols.add_generics(
        db.item_generics(item)
            .iter()
            .enumerate()
            .filter_map(|(idx, &name)| {
                idx.try_into()
                    .ok()
                    .map(|idx| (name, GenericId::new(item, idx)))
            }),
    );

    match db.item_ast(item) {
        AstItem::StructDef(strct) => {
            if let Some(fields) = strct.fields() {
                match &*fields {
                    AstStructFields::BracketedStructFields(fields) => {
                        for field in fields.fields() {
                            // TODO: Handle missing data and all types
                            let name = field.ident().unwrap();
                            let ty = field.ty().unwrap();
                            let generic = ty.as_generic_type().unwrap();
                            let path = generic.path().unwrap();

                            if let Some(ty) = symbols.lookup_type_path(&*path) {
                                dbg!(ty);
                            } else {
                                // TODO: Better error, string distance lookup for suggestions, etc.
                                let error = Diagnostic::error()
                                    .with_message("could not find type in scope")
                                    .with_label(Label::primary(path.span(file)).with_message(
                                        format!(
                                            "could not find the type '{}' within the current scope",
                                            db.parser_cache().interner().resolve(path.head()
                                                .unwrap()
                                                .interned()
                                            ),
                                        ),
                                    ));

                                errors.push(error);
                            }
                        }
                    }

                    AstStructFields::TupleStructFields(_) => todo!(),
                }
            }

            (None, ArcSlice::from(errors))
        }

        AstItem::ClauseDef(_)
        | AstItem::ConstDef(_)
        | AstItem::EnumDef(_)
        | AstItem::FunctionDef(_)
        | AstItem::ImplBlock(_)
        | AstItem::RelationDef(_)
        | AstItem::TypeAlias(_)
        | AstItem::UseDef(_) => (None, ArcSlice::empty()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GenericId {
    parent: ItemId,
    idx: u8,
}

impl GenericId {
    const fn new(parent: ItemId, idx: u8) -> Self {
        Self { parent, idx }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolScope {
    types: OrdMap<IStr, TypeSymbol>,
    modules: OrdMap<IStr, ModuleId>,
    variables: OrdMap<IStr, VarSymbol>,
}

impl SymbolScope {
    fn new() -> Self {
        Self {
            types: OrdMap::new(),
            modules: OrdMap::new(),
            variables: OrdMap::new(),
        }
    }

    fn add_primitives(&mut self, interner: &Interner) {
        let primitives = ["bool", "u8", "i8", "usize", "isize"];

        for (idx, primitive) in primitives.into_iter().enumerate() {
            let name = interner.get_or_intern_static(primitive);
            let id = PrimId(NonZeroU16::new(u16::try_from(idx).unwrap() + 1).unwrap());

            self.types.insert(
                name,
                TypeSymbol::new(id.into(), name, TypeSymbolKind::Primitive),
            );
        }
    }

    /// Declares a variable within the current scope, returning an error if it shadows a
    /// previously-declared variable
    fn declare_var<I>(&mut self, id: I, name: IStr, kind: VarSymbolKind) -> Option<Diagnostic>
    where
        I: Into<VarId>,
    {
        self.variables
            .insert(name, VarSymbol::new(id.into(), name, kind))
            .map(|shadowed| todo!(" Build an error for disallowed shadowing"))
    }

    /// Declares a type within the current scope, returning an error if it shadows a
    /// previously-declared type
    fn declare_type<I>(&mut self, id: I, name: IStr, kind: TypeSymbolKind) -> Option<Diagnostic>
    where
        I: Into<TypeId>,
    {
        self.types
            .insert(name, TypeSymbol::new(id.into(), name, kind))
            .map(|shadowed| todo!("Build an error for disallowed shadowing"))
    }

    /// Adds generics to the current scope, shadowing any types by the same name
    fn add_generics<G>(&mut self, generics: G)
    where
        G: IntoIterator<Item = (IStr, GenericId)>,
    {
        generics.into_iter().for_each(|(name, id)| {
            self.types.insert(
                name,
                TypeSymbol::new(id.into(), name, TypeSymbolKind::Generic),
            );
        });
    }

    fn lookup_type_path<'a>(&'a self, path: &AstPath) -> Option<&'a TypeSymbol> {
        // TODO: Full paths & generics
        self.types.get(&path.head().unwrap().interned())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimId(NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeId {
    Item(ItemId),
    Primitive(PrimId),
    Generic(GenericId),
}

impl From<ItemId> for TypeId {
    #[inline]
    fn from(item: ItemId) -> Self {
        Self::Item(item)
    }
}

impl From<PrimId> for TypeId {
    #[inline]
    fn from(prim: PrimId) -> Self {
        Self::Primitive(prim)
    }
}

impl From<GenericId> for TypeId {
    #[inline]
    fn from(generic: GenericId) -> Self {
        Self::Generic(generic)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeSymbol {
    id: TypeId,
    name: IStr,
    kind: TypeSymbolKind,
}

impl TypeSymbol {
    pub const fn new(id: TypeId, name: IStr, kind: TypeSymbolKind) -> Self {
        Self { id, name, kind }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeSymbolKind {
    Enum,
    Struct,
    Generic,
    Primitive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarId {
    Item(ItemId),
    // TODO: Statements and expressions
}

impl From<ItemId> for VarId {
    #[inline]
    fn from(item: ItemId) -> Self {
        Self::Item(item)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarSymbol {
    id: VarId,
    name: IStr,
    kind: VarSymbolKind,
}

impl VarSymbol {
    pub const fn new(id: VarId, name: IStr, kind: VarSymbolKind) -> Self {
        Self { id, name, kind }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarSymbolKind {
    Fn,
    Const,
    Binding,
    Relation,
}

#[salsa::query_group(SymbolDatabaseStorage)]
pub trait SymbolDatabase: SourceDatabase {
    /// The ast defining the given item
    #[salsa::interned]
    fn item_ast_inner(&self, file: FileId, ast: AstItem) -> ItemId;
    fn item_ast(&self, item: ItemId) -> AstItem;
    fn item_file(&self, item: ItemId) -> FileId;

    /// Get all of the symbols defined by the given item
    fn item_symbol(&self, item: ItemId) -> Option<Symbol>;
    fn item_name(&self, item: ItemId) -> Option<IStr>;

    fn top_level_items(&self, file: FileId) -> ArcSlice<ItemId>;

    // TODO: Split the symbol space where it makes sense to so that
    //       functions don't clash with types
    // TODO: This may not be as deterministic as we'd like, `IStr` has
    //       no deterministic properties since `IStr`'s internal repr
    //       is influenced by concurrency, user-driven factors, etc.
    fn top_level_symbols_inner(&self, file: FileId) -> (SymbolScope, ArcSlice<Diagnostic>);
    fn top_level_symbols(&self, file: FileId) -> SymbolScope;
    fn top_level_symbol_diags(&self, file: FileId) -> ArcSlice<Diagnostic>;

    fn item_generics_inner(&self, item: ItemId) -> (ArcSlice<IStr>, ArcSlice<Diagnostic>);
    fn item_generics(&self, item: ItemId) -> ArcSlice<IStr>;
    fn item_generics_diags(&self, item: ItemId) -> ArcSlice<Diagnostic>;
}

fn item_ast(db: &dyn SymbolDatabase, item: ItemId) -> AstItem {
    db.lookup_item_ast_inner(item).1
}

fn item_file(db: &dyn SymbolDatabase, item: ItemId) -> FileId {
    db.lookup_item_ast_inner(item).0
}

fn top_level_items(db: &dyn SymbolDatabase, file: FileId) -> ArcSlice<ItemId> {
    ArcSlice::new(
        db.syntax_root(file)
            .items()
            .map(|item| db.item_ast_inner(file, item.into_owned())),
    )
}

fn top_level_symbols_inner(
    db: &dyn SymbolDatabase,
    file: FileId,
) -> (SymbolScope, ArcSlice<Diagnostic>) {
    // FIXME: We want to iterate in lexical order so that our errors make sense to users
    let (mut symbols, mut errors) = (SymbolScope::new(), Vec::new());
    // Add primitives to the current scope
    symbols.add_primitives(db.parser_cache().interner());

    for &item_id in db.top_level_items(file).iter() {
        if let Some(symbol) = db.item_symbol(item_id) {
            let (id, name) = (symbol.id, symbol.name);

            let error = match symbol.kind {
                SymbolKind::Fn => symbols.declare_var(id, name, VarSymbolKind::Fn),
                SymbolKind::Const => symbols.declare_var(id, name, VarSymbolKind::Const),
                // TODO: Should relations declare types too?
                SymbolKind::Relation => symbols.declare_var(id, name, VarSymbolKind::Relation),
                SymbolKind::Enum => symbols.declare_type(id, name, TypeSymbolKind::Enum),
                SymbolKind::Struct => symbols.declare_type(id, name, TypeSymbolKind::Struct),
            };

            if let Some(error) = error {
                errors.push(error);
            }
        }
    }

    (symbols, ArcSlice::new(errors))
}

fn top_level_symbols(db: &dyn SymbolDatabase, file: FileId) -> SymbolScope {
    db.top_level_symbols_inner(file).0
}

fn top_level_symbol_diags(db: &dyn SymbolDatabase, file: FileId) -> ArcSlice<Diagnostic> {
    db.top_level_symbols_inner(file).1
}

fn item_symbol(db: &dyn SymbolDatabase, item: ItemId) -> Option<Symbol> {
    let (name, kind) = match db.item_ast(item) {
        AstItem::FunctionDef(func) => {
            let name = func.ident().unwrap();
            (name, SymbolKind::Fn)
        }

        AstItem::RelationDef(relation) => {
            let name = relation.ident().unwrap();
            (name, SymbolKind::Relation)
        }

        AstItem::StructDef(r#struct) => {
            let name = r#struct.ident().unwrap();
            (name, SymbolKind::Struct)
        }

        AstItem::EnumDef(r#enum) => {
            let name = r#enum.ident().unwrap();
            (name, SymbolKind::Enum)
        }

        AstItem::ConstDef(_) | AstItem::TypeAlias(_) | AstItem::UseDef(_) => todo!(),

        AstItem::ClauseDef(_) | AstItem::ImplBlock(_) => return None,
    };

    Some(Symbol::new(item, name, kind))
}

fn item_name(db: &dyn SymbolDatabase, item: ItemId) -> Option<IStr> {
    db.item_symbol(item).map(|symbol| symbol.name)
}

fn item_generics_inner(
    db: &dyn SymbolDatabase,
    item: ItemId,
) -> (ArcSlice<IStr>, ArcSlice<Diagnostic>) {
    let (mut generics, mut diagnostics) = (Vec::new(), Vec::new());
    let (file, ast) = db.lookup_item_ast_inner(item);

    match ast {
        AstItem::FunctionDef(func) => {
            if let Some(args) = func.generics() {
                process_generics(&*args, file, &mut generics, &mut diagnostics);
            }
        }

        AstItem::StructDef(strct) => {
            if let Some(args) = strct.generics() {
                process_generics(&*args, file, &mut generics, &mut diagnostics);
            }
        }

        // TODO: Generics for these items
        AstItem::TypeAlias(_) | AstItem::EnumDef(_) | AstItem::ImplBlock(_) => {}

        AstItem::ClauseDef(_)
        | AstItem::ConstDef(_)
        | AstItem::RelationDef(_)
        | AstItem::UseDef(_) => {}
    }

    (ArcSlice::from(generics), ArcSlice::from(diagnostics))
}

fn item_generics(db: &dyn SymbolDatabase, item: ItemId) -> ArcSlice<IStr> {
    db.item_generics_inner(item).0
}

fn item_generics_diags(db: &dyn SymbolDatabase, item: ItemId) -> ArcSlice<Diagnostic> {
    db.item_generics_inner(item).1
}

fn process_generics(
    args: &AstGenerics,
    file: FileId,
    generics: &mut Vec<IStr>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for generic in args.generics() {
        if let Some(ty) = generic.ty() {
            if let Some(ty) = ty.as_generic_type() {
                if let Some(args) = ty.generics() {
                    let error = Diagnostic::error()
                        .with_message("generics cannot have generic parameters")
                        .with_label(Label::primary(args.span(file)));
                    diagnostics.push(error);
                }

                if let Some(path) = ty.path() {
                    if let Some(colon) = path.double_colon() {
                        let error = Diagnostic::error()
                            .with_message("generic parameters cannot be prefixed with '::'")
                            .with_label(Label::primary(path.span(file)))
                            .with_label(
                                Label::secondary(colon.span(file))
                                    .with_message("try removing this ::"),
                            );
                        diagnostics.push(error);
                    }

                    if path.tails().count() != 0 {
                        let error = Diagnostic::error()
                            .with_message("item paths cannot be generic parameters")
                            .with_label(Label::primary(path.span(file)));
                        diagnostics.push(error);
                    }

                    if let Some(head) = path.head() {
                        generics.push(head.interned());
                    } else {
                        // TODO: Is this possible?
                    }
                }
            } else {
                let error = Diagnostic::error()
                    .with_message(
                        "tuples and function types are not allowed to be generic parameters",
                    )
                    .with_label(Label::primary(ty.span(file)));
                diagnostics.push(error);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FnSig {
    args: Vec<(IStr, ItemId)>,
    ret: ItemId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    id: ItemId,
    name: IStr,
    kind: SymbolKind,
}

impl Symbol {
    pub const fn new(id: ItemId, name: IStr, kind: SymbolKind) -> Self {
        Self { id, name, kind }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SymbolKind {
    Fn,
    Enum,
    Const,
    Struct,
    Relation,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ItemId(salsa::InternId);

impl salsa::InternKey for ItemId {
    #[inline]
    fn from_intern_id(id: salsa::InternId) -> Self {
        Self(id)
    }

    #[inline]
    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

impl Debug for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ItemId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::resolve::{HirDatabase, HirDatabaseStorage, SymbolDatabase, SymbolDatabaseStorage};
    use ddlog_diagnostics::{DiagnosticConfig, FileCache, FileId, Interner};
    use ddlog_syntax::queries::{
        ParserCache, ParserCacheDatabase, ParserCacheDatabaseStorage, SourceDatabase,
        SourceDatabaseStorage,
    };

    const SOURCE: &str = "
struct Bar {
    baz: usize,
}

fn foo(bar: Bar) {}
    ";

    #[salsa::database(
        HirDatabaseStorage,
        SourceDatabaseStorage,
        SymbolDatabaseStorage,
        ParserCacheDatabaseStorage
    )]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<TestDb>,
    }

    impl salsa::Database for TestDb {}

    impl salsa::ParallelDatabase for TestDb {
        fn snapshot(&self) -> salsa::Snapshot<TestDb> {
            salsa::Snapshot::new(TestDb {
                storage: self.storage.snapshot(),
            })
        }
    }

    #[test]
    fn resolve_test() {
        let interner = Interner::new();
        let file = FileId::new(interner.get_or_intern_static("tests/resolve_test.ddlog"));

        let config = DiagnosticConfig::new();
        let mut file_cache = FileCache::new(interner.clone());
        file_cache.add_str(file, SOURCE);

        let mut db = TestDb::default();
        db.set_parser_cache(ParserCache::new(interner.clone()));
        db.set_file_source(file, SOURCE.into());

        println!("Symbols: {:#?}", db.top_level_symbols(file));
        {
            let mut stdout = std::io::stdout().lock();
            for error in db.top_level_symbol_diags(file).iter().cloned() {
                error
                    .emit_to(&config, &mut file_cache, &mut stdout)
                    .unwrap();
            }
        }

        for &item in db.top_level_items(file).iter() {
            let (generics, errors) = db.item_generics_inner(item);
            let mut errors = errors.to_vec();

            let (_item_hir, lowering_errors) = db.lower_item(item);
            errors.extend_from_slice(&*lowering_errors);

            if !generics.is_empty() {
                let name = db.item_name(item).unwrap();

                println!(
                    "generics for {}: {:?}",
                    interner.resolve(name),
                    generics
                        .iter()
                        .map(|&generic| interner.resolve(generic))
                        .collect::<Vec<_>>(),
                );
            }

            {
                let mut stdout = std::io::stdout().lock();
                for error in errors.iter().cloned() {
                    error
                        .emit_to(&config, &mut file_cache, &mut stdout)
                        .unwrap();
                }
            }
        }
    }
}
