mod queries;

pub use queries::{Diagnostics, DocumentSymbols, Session, Source, Validation};

use queries::{
    DiagnosticsDatabase, DocumentSymbolsDatabase, SessionDatabase, SourceDatabase,
    ValidationDatabase,
};
use salsa::{Database, ParallelDatabase, Snapshot, Storage};
use std::fmt::{self, Debug};

#[salsa::database(
    SessionDatabase,
    SourceDatabase,
    ValidationDatabase,
    DocumentSymbolsDatabase,
    DiagnosticsDatabase
)]
#[derive(Default)]
pub struct DDlogDatabase {
    storage: Storage<Self>,
}

impl Debug for DDlogDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DDlogDatabase").finish()
    }
}

impl Database for DDlogDatabase {}

impl ParallelDatabase for DDlogDatabase {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(Self {
            storage: self.storage.snapshot(),
        })
    }
}
