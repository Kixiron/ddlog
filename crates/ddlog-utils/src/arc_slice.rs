use core::slice;
use std::{
    fmt::{self, Debug},
    ops::Deref,
};
use triomphe::ThinArc;

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ArcSlice<T> {
    slice: ThinArc<(), T>,
}

impl<T> ArcSlice<T> {
    #[inline]
    pub fn new<I>(slice: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        Self {
            slice: ThinArc::from_header_and_iter((), slice.into_iter()),
        }
    }

    #[inline]
    pub fn empty() -> Self {
        // FIXME: Make sure empty ArcSlices don't allocate
        Self::new(Vec::new())
    }

    /// Creates a new [`ArcSlice`] from an iterator of unknown size. Whenever possible
    /// [`ArcSlice::new()`] should be used as it's much more efficient.
    #[inline]
    pub fn from_dynamic<I>(slice: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        // We need to collect into a `Vec` in order to create an exactly sized iterator
        Self::new(slice.into_iter().collect::<Vec<_>>())
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.slice.slice
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.as_slice().iter()
    }
}

impl<T> From<Vec<T>> for ArcSlice<T> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Self::new(vec)
    }
}

impl<T> FromIterator<T> for ArcSlice<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_dynamic(iter)
    }
}

impl<'a, T> IntoIterator for &'a ArcSlice<T> {
    type IntoIter = slice::Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<T> AsRef<[T]> for ArcSlice<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> Deref for ArcSlice<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> Clone for ArcSlice<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            slice: self.slice.clone(),
        }
    }
}

impl<T> Debug for ArcSlice<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}
