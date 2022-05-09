use derive_stable_hash::StableHash;

#[derive(StableHash)]
struct Foo;

#[derive(StableHash)]
enum Bar {}

fn main() {}
