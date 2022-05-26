use ddlog_syntax::queries::SourceDatabase;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: SourceDatabase {}
