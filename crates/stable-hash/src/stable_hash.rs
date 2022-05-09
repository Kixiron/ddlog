use ddlog_utils::{Arc, ArcSlice};

use crate::StableHasher;
use core::mem::Discriminant;
use std::{borrow::Cow, rc::Rc};

pub trait StableHash {
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher;

    #[inline]
    fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
        for elem in slice {
            elem.stable_hash(state);
        }
    }
}

impl<T> StableHash for Discriminant<T> {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        core::hash::Hash::hash(self, state);
    }
}

impl StableHash for bool {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_u8(*self as u8);
    }

    #[inline]
    fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
        state.write(bytemuck::cast_slice(slice));
    }
}

impl StableHash for u8 {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_u8(*self);
    }

    #[inline]
    fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
        state.write(slice);
    }
}

impl StableHash for u16 {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_u16(*self);
    }

    #[inline]
    fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
        if cfg!(target_endian = "little") {
            for &elem in slice {
                state.write_u16(elem);
            }
        } else {
            state.write(bytemuck::cast_slice(slice));
        }
    }
}

impl StableHash for u32 {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_u32(*self);
    }

    #[inline]
    fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
        if cfg!(target_endian = "little") {
            for &elem in slice {
                state.write_u32(elem);
            }
        } else {
            state.write(bytemuck::cast_slice(slice));
        }
    }
}

impl StableHash for u64 {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_u64(*self);
    }

    #[inline]
    fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
        if cfg!(target_endian = "little") {
            for &elem in slice {
                state.write_u64(elem);
            }
        } else {
            state.write(bytemuck::cast_slice(slice));
        }
    }
}

impl StableHash for u128 {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_u128(*self);
    }

    #[inline]
    fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
        if cfg!(target_endian = "little") {
            for &elem in slice {
                state.write_u128(elem);
            }
        } else {
            state.write(bytemuck::cast_slice(slice));
        }
    }
}

impl StableHash for usize {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_usize(*self);
    }

    #[inline]
    fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
        if cfg!(target_ptr_width = "64") {
            state.write(bytemuck::cast_slice(slice));
        } else {
            for &elem in slice {
                state.write_usize(elem);
            }
        }
    }
}

impl StableHash for i8 {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_i8(*self);
    }

    #[inline]
    fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
        state.write(bytemuck::cast_slice(slice));
    }
}

// TODO: I'd like to do slice impls for signed numbers, but I'm
//       not sure how sign extension will interact with that
impl StableHash for i16 {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_i16(*self);
    }
}

impl StableHash for i32 {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_i32(*self);
    }
}

impl StableHash for i64 {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_i64(*self);
    }
}

impl StableHash for i128 {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_i128(*self);
    }
}

impl StableHash for isize {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write_isize(*self);
    }
}

impl<T> StableHash for Box<T>
where
    T: StableHash + ?Sized,
{
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        T::stable_hash(self, state);
    }
}

impl<T> StableHash for [T]
where
    T: StableHash,
{
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        T::stable_hash_slice(self, state);
    }
}

impl<T> StableHash for Vec<T>
where
    T: StableHash,
{
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        T::stable_hash_slice(self, state);
    }
}

impl StableHash for str {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        self.as_bytes().stable_hash(state);
    }
}

impl StableHash for String {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        self.as_bytes().stable_hash(state);
    }
}

impl<T> StableHash for Cow<'_, T>
where
    T: StableHash + ToOwned + ?Sized,
{
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        self.as_ref().stable_hash(state);
    }
}

impl<T> StableHash for Rc<T>
where
    T: StableHash + ?Sized,
{
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        self.as_ref().stable_hash(state);
    }
}

impl<T> StableHash for Arc<T>
where
    T: StableHash + ?Sized,
{
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        self.as_ref().stable_hash(state);
    }
}

impl<T> StableHash for ArcSlice<T>
where
    T: StableHash,
{
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        T::stable_hash_slice(self.as_ref(), state);
    }
}

impl StableHash for () {
    #[inline]
    fn stable_hash<H>(&self, _state: &mut H)
    where
        H: StableHasher,
    {
    }

    #[inline]
    fn stable_hash_slice<H>(_slice: &[Self], _state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
    }
}

macro_rules! stable_hash_tuple {
    ($($($generic:ident)+),+ $(,)?) => {
        $(
            impl<$($generic),+> StableHash for ($($generic,)+)
            where
                $($generic: StableHash,)+
            {
                #[inline]
                fn stable_hash<Hasher>(&self, state: &mut Hasher)
                where
                    Hasher: StableHasher,
                {
                    #[allow(non_snake_case)]
                    let ($($generic,)+) = self;
                    $($generic.stable_hash(state);)+
                }
            }
        )+
    };
}

stable_hash_tuple! {
    A,
    A B,
    A B C,
    A B C D,
    A B C D E,
    A B C D E F,
    A B C D E F G,
    A B C D E F G H,
    A B C D E F G H I,
    A B C D E F G H I J,
    A B C D E F G H I J K,
    A B C D E F G H I J K L,
    A B C D E F G H I J K L M,
    A B C D E F G H I J K L M N,
    A B C D E F G H I J K L M N O,
    A B C D E F G H I J K L M N O P,
}

macro_rules! stable_hash_nonzero {
    ($($nonzero:ident = $inner:ty),+ $(,)?) => {
        $(
            impl StableHash for core::num::$nonzero {
                #[inline]
                fn stable_hash<H>(&self, state: &mut H)
                where
                    H: StableHasher,
                {
                    self.get().stable_hash(state);
                }

                #[inline]
                fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
                where
                    H: StableHasher,
                {
                    // Assertions for our `bytemuck::cast_slice()` polyfill
                    const _: () = assert!(
                        core::mem::size_of::<core::num::$nonzero>() != 0
                            && core::mem::size_of::<core::num::$nonzero>() == core::mem::size_of::<$inner>()
                            && core::mem::align_of::<core::num::$nonzero>() == core::mem::align_of::<$inner>(),
                    );

                    // Safety: The nonzero and inner types have the same repr
                    // FIXME: Replace this with a bytemuck call once
                    //        https://github.com/Lokathor/bytemuck/issues/105
                    //        is fixed
                    let inner = unsafe {
                        core::slice::from_raw_parts::<$inner>(
                            slice.as_ptr().cast(),
                            slice.len(),
                        )
                    };

                    <$inner as StableHash>::stable_hash_slice(inner, state);
                }
            }
        )+
    };
}

stable_hash_nonzero! {
    NonZeroU8 = u8,
    NonZeroU16 = u16,
    NonZeroU32 = u32,
    NonZeroU64 = u64,
    NonZeroU128 = u128,
    NonZeroUsize = usize,
    NonZeroI8 = i8,
    NonZeroI16 = i16,
    NonZeroI32 = i32,
    NonZeroI64 = i64,
    NonZeroI128 = i128,
    NonZeroIsize = isize,
}
