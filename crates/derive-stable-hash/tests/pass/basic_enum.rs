use derive_stable_hash::StableHash;

#[derive(StableHash)]
enum Foo {
    Bar,
    Baz(u8),
    Bing(Box<Self>),
    Bong { whizz: usize, bang: Vec<Self> },
}

fn main() {}
