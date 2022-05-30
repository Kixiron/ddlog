use stable_hash::StableHash;

#[derive(StableHash)]
union Foo {
    bar: u32,
    baz: usize,
    bing: u8,
}

fn main() {}
