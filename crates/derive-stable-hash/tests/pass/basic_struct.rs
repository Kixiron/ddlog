use derive_stable_hash::StableHash;

#[derive(StableHash)]
struct Foo {
    bar: usize,
    baz: Vec<u32>,
}

fn main() {}
