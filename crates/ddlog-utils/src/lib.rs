//! Various utilities shared between various ddlog crates

#[macro_use]
mod bvec;
mod arc_slice;
mod either;
mod hasher;
mod option_ext;
pub mod strings;

pub use arc_slice::ArcSlice;
pub use either::Either;
pub use hasher::ConsistentHasher;
pub use option_ext::OptionExt;
pub use triomphe::Arc;
