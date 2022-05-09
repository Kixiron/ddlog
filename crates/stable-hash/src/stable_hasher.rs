use crate::StablyHashed;
use core::hash::Hasher;

/// Stable hashing for values
///
/// All `StableHasher` implementations must use big endian
/// representations for integers, no matter the target platform
pub trait StableHasher: Hasher {
    fn finalize(&mut self) -> StablyHashed;
}

impl<T> StableHasher for &mut T
where
    T: StableHasher + ?Sized,
{
    #[inline]
    fn finalize(&mut self) -> crate::StablyHashed {
        T::finalize(self)
    }
}

impl<T> StableHasher for Box<T>
where
    T: StableHasher + ?Sized,
{
    #[inline]
    fn finalize(&mut self) -> crate::StablyHashed {
        T::finalize(self)
    }
}
