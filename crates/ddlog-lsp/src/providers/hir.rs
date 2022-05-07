use crate::database::HirStore;
use ddlog_diagnostics::FileId;
use ddlog_hir::{hir::Item as HirItem, lowering::HirBuilder};
use ddlog_syntax::{ast::nodes::Root, SyntaxNodeExt};
use ddlog_utils::ArcSlice;

pub fn hir_items(store: &dyn HirStore, file: FileId) -> ArcSlice<HirItem> {
    let session = store.session();
    let interner = session.interner();
    let uri = file.to_str(interner);
    tracing::debug!(file = uri, "building hir for file");

    let items = HirBuilder::new(interner).lower_items(store.syntax(file).to::<Root>().items());

    ArcSlice::new(items)
}
