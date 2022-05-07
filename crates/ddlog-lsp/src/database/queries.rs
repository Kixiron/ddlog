use crate::providers::{document_symbols, hir};
use ddlog_diagnostics::{Diagnostic, FileId, Rope};
use ddlog_hir::hir::Item as HirItem;
use ddlog_syntax::{
    ast::nodes::{
        BracketedStructField, EnumDef, EnumVariant, FunctionArg, FunctionDef, StructDef,
        VariantStructField,
    },
    validation, RuleCtx, SyntaxNode,
};
use ddlog_utils::{Arc, ArcSlice};
use std::fmt::Debug;
use tower_lsp::lsp_types::{DocumentSymbol, Url};

#[salsa::query_group(SessionDatabase)]
pub trait Session {
    #[salsa::input]
    fn session(&self) -> Arc<crate::Session>;

    #[salsa::transparent]
    fn file_id(&self, url: &Url) -> FileId;
}

fn file_id(session: &dyn Session, url: &Url) -> FileId {
    FileId::new(session.session().interner().get_or_intern(url.as_str()))
}

#[salsa::query_group(SourceDatabase)]
pub trait Source: Session {
    #[salsa::input]
    fn file_source(&self, file: FileId) -> Rope;

    fn parsed(&self, file: FileId) -> (SyntaxNode, ArcSlice<Diagnostic>);

    fn parse_diagnostics(&self, file: FileId) -> ArcSlice<Diagnostic>;

    fn syntax(&self, file: FileId) -> SyntaxNode;
}

fn parsed(source: &dyn Source, file: FileId) -> (SyntaxNode, ArcSlice<Diagnostic>) {
    let source_text = source.file_source(file);
    let session = source.session();

    let (parsed, cache) = ddlog_syntax::parse(file, &source_text.to_string(), session.node_cache());
    session.give_node_cache(cache);

    let (root, diagnostics) = parsed.into_parts();

    (root, ArcSlice::new(diagnostics))
}

fn syntax(source: &dyn Source, file: FileId) -> SyntaxNode {
    source.parsed(file).0
}

fn parse_diagnostics(source: &dyn Source, file: FileId) -> ArcSlice<Diagnostic> {
    source.parsed(file).1
}

#[salsa::query_group(ValidationDatabase)]
pub trait Validation: Source {
    fn validation_diagnostics(&self, file: FileId) -> ArcSlice<Diagnostic>;
}

fn validation_diagnostics(validation: &dyn Validation, file: FileId) -> ArcSlice<Diagnostic> {
    let mut ctx = RuleCtx::new(
        file,
        validation.file_source(file),
        validation.session().interner().clone(),
    );

    validation::run_validators(&validation.syntax(file), &mut ctx);

    ArcSlice::new(ctx.diagnostics)
}

#[salsa::query_group(SymbolsDatabase)]
pub trait HirStore: Source {
    #[salsa::invoke(hir::hir_items)]
    fn items(&self, file: FileId) -> ArcSlice<HirItem>;
}

#[salsa::query_group(DocumentSymbolsDatabase)]
pub trait DocumentSymbols: HirStore {
    #[salsa::invoke(document_symbols::document_symbols)]
    fn document_symbols(&self, file: FileId) -> ArcSlice<DocumentSymbol>;

    #[salsa::invoke(document_symbols::document_function)]
    fn document_function(&self, file: FileId, function: FunctionDef) -> DocumentSymbol;

    #[salsa::invoke(document_symbols::document_function_arg)]
    fn document_function_arg(
        &self,
        file: FileId,
        arg: FunctionArg,
    ) -> Option<ArcSlice<DocumentSymbol>>;

    #[salsa::invoke(document_symbols::document_struct)]
    fn document_struct(&self, file: FileId, strct: StructDef) -> DocumentSymbol;

    #[salsa::invoke(document_symbols::document_struct_field)]
    fn document_struct_field(
        &self,
        file: FileId,
        field: BracketedStructField,
    ) -> Option<DocumentSymbol>;

    #[salsa::invoke(document_symbols::document_enum)]
    fn document_enum(&self, file: FileId, enumeration: EnumDef) -> DocumentSymbol;

    #[salsa::invoke(document_symbols::document_enum_variant)]
    fn document_enum_variant(&self, file: FileId, variant: EnumVariant) -> Option<DocumentSymbol>;

    #[salsa::invoke(document_symbols::document_variant_field)]
    fn document_variant_field(
        &self,
        file: FileId,
        field: VariantStructField,
    ) -> Option<DocumentSymbol>;
}

#[salsa::query_group(DiagnosticsDatabase)]
pub trait Diagnostics: Source + Validation {
    fn diagnostics(&self, file: FileId) -> ArcSlice<Diagnostic>;
}

fn diagnostics(diagnostics: &dyn Diagnostics, file: FileId) -> ArcSlice<Diagnostic> {
    let (parse_diagnostics, validation_diagnostics) = (
        diagnostics.parse_diagnostics(file),
        diagnostics.validation_diagnostics(file),
    );

    let mut diagnostics =
        Vec::with_capacity(parse_diagnostics.len() + validation_diagnostics.len());
    diagnostics.extend(parse_diagnostics.iter().cloned());
    diagnostics.extend(validation_diagnostics.iter().cloned());

    diagnostics.sort_by_key(|diagnostic| diagnostic.primary_span());
    diagnostics.dedup();

    ArcSlice::new(diagnostics)
}
