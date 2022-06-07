#[cfg(not(feature = "std"))]
use core::arch;
use core::{
    mem::{self, MaybeUninit},
    ptr, slice,
};

#[inline]
pub(crate) fn abort() -> ! {
    // std
    #[cfg(feature = "std")]
    std::process::abort();

    // no_std + WASM
    #[cfg(all(not(feature = "std"), target_family = "wasm"))]
    arch::wasm::unreachable();

    // no_std + x86/x86_64
    #[cfg(all(not(feature = "std"), any(target_arch = "x86", target_arch = "x86_64")))]
    unsafe {
        arch::asm!("ud2", options(nomem, preserves_flags, nostack));
    }

    // no_std + arm/aarch64
    #[cfg(all(
        not(feature = "std"),
        any(target_arch = "arm", target_arch = "aarch64")
    ))]
    unsafe {
        arch::asm!("brk #0x1", options(nomem, preserves_flags, nostack));
    }

    // TODO: Inline asm is unstable for mips
    // // no_std + mips/mips64
    // #[cfg(all(
    //     not(feature = "std"),
    //     any(target_arch = "mips", target_arch = "mips64"),
    // ))]
    // unsafe {
    //     arch::asm!("break", options(nomem, preserves_flags, nostack));
    // }

    // no_std + powerpc/powerpc64
    #[cfg(all(
        not(feature = "std"),
        any(target_arch = "powerpc", target_arch = "powerpc64"),
    ))]
    unsafe {
        arch::asm!("trap", options(nomem, preserves_flags, nostack));
    }

    // Fallback to a double drop, this has worse
    // codegen than any of the other options
    #[cfg(not(any(
        feature = "std",
        target_family = "wasm",
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
        // target_arch = "mips",
        // target_arch = "mips64",
        target_arch = "powerpc",
        target_arch = "powerpc64",
    )))]
    {
        struct PanicOnDrop;
        impl Drop for PanicOnDrop {
            #[inline(always)]
            fn drop(&mut self) {
                panic!("")
            }
        }

        let _panic_on_drop = PanicOnDrop;
        panic!("")
    }
}

#[inline]
pub(crate) fn copy_to_uninit_slice<'a, T>(this: &'a mut [MaybeUninit<T>], src: &[T]) -> &'a mut [T]
where
    T: Copy,
{
    #[cfg(feature = "nightly")]
    return MaybeUninit::write_slice(this, src);

    // Stable polyfill
    #[cfg(not(feature = "nightly"))]
    {
        // SAFETY: &[T] and &[MaybeUninit<T>] have the same layout
        let uninit_src: &[MaybeUninit<T>] = unsafe { mem::transmute(src) };
        this.copy_from_slice(uninit_src);

        // SAFETY: Valid elements have just been copied into `this` so it is initialized
        unsafe { slice::from_raw_parts_mut(this.as_mut_ptr().cast::<T>(), this.len()) }
    }
}

#[inline]
pub(crate) fn clone_to_uninit_slice<'a, T>(this: &'a mut [MaybeUninit<T>], src: &[T]) -> &'a mut [T]
where
    T: Clone,
{
    #[cfg(feature = "nightly")]
    return MaybeUninit::write_slice_cloned(this, src);

    // Stable polyfill
    #[cfg(not(feature = "nightly"))]
    {
        // unlike copy_from_slice this does not call clone_from_slice on the slice
        // this is because `MaybeUninit<T: Clone>` does not implement Clone.

        // Pre-poop our pants
        struct Guard<'a, T> {
            slice: &'a mut [MaybeUninit<T>],
            initialized: usize,
        }

        impl<'a, T> Drop for Guard<'a, T> {
            fn drop(&mut self) {
                // SAFETY: this raw slice will contain only initialized objects
                // that's why, it is allowed to drop it.
                unsafe {
                    let initialized_part = slice::from_raw_parts_mut(
                        self.slice.as_mut_ptr().cast::<T>(),
                        self.initialized,
                    );

                    ptr::drop_in_place(initialized_part);
                }
            }
        }

        assert_eq!(
            this.len(),
            src.len(),
            "destination and source slices have different lengths",
        );
        // NOTE: We need to explicitly slice them to the same length
        // for bounds checking to be elided, and the optimizer will
        // generate memcpy for simple cases (for example T = u8).
        let len = this.len();
        let src = &src[..len];

        // guard is needed b/c panic might happen during a clone
        let mut guard = Guard {
            slice: this,
            initialized: 0,
        };

        for (idx, src) in src.iter().enumerate() {
            guard.slice[idx].write(src.clone());
            guard.initialized += 1;
        }

        mem::forget(guard);

        // SAFETY: Valid elements have just been written into `this` so it is initialized
        unsafe { slice::from_raw_parts_mut(this.as_mut_ptr().cast::<T>(), len) }
    }
}
