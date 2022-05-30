use stable_hash::StableHash;

#[derive(StableHash)]
struct Foo(Option<usize>);

fn main() {}
