use crate::database::{DDlogDatabase, Session, Symbols};
use lspower::lsp::{DocumentSymbolResponse, Url};
use salsa::Snapshot;

pub(crate) fn nested_symbols(
    snapshot: Snapshot<DDlogDatabase>,
    url: &Url,
) -> Option<DocumentSymbolResponse> {
    let session = snapshot.session();
    let file = session.file_id(url);

    let symbols = snapshot.document_symbols(file);
    if symbols.is_empty() {
        None
    } else {
        Some(DocumentSymbolResponse::Nested(symbols.to_vec()))
    }
}