#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(maybe_uninit_write_slice))]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

mod error;
mod utils;

pub use error::{ArcError, ArcResult};

use core::{
    alloc::Layout,
    fmt::{self, Debug},
    mem::{needs_drop, ManuallyDrop, MaybeUninit},
    ops::Deref,
    ptr::{self, addr_of, addr_of_mut, NonNull},
    slice,
    sync::atomic::{AtomicUsize, Ordering},
};

#[repr(transparent)]
pub struct ArcSlice<T, A: Atomic = AtomicUsize> {
    inner: NonNull<ArcInner<[T; 0], usize, A>>,
}

impl<T, A> ArcSlice<T, A>
where
    A: Atomic,
{
    // TODO: Allow creating empty `ArcSlice`s without allocating
    // const EMPTY: ArcInner<[T; 0], usize, A> = ArcInner {
    //     data: 0,
    //     counter: A::ONE,
    //     value: [],
    // };
    //
    // #[inline]
    // pub const fn empty() -> Self {
    //     // Safety: References cannot be null and `NonNull::from(&T)` isn't const
    //     let inner = unsafe { NonNull::new_unchecked(&Self::EMPTY as *const _ as *mut _) };
    //
    //     // Safety: We've created a valid arc and we pinky promise to never drop it
    //     let arc = unsafe { Arc::from_inner(inner) };
    //
    //     Self {
    //         slice: ManuallyDrop::new(arc),
    //     }
    // }

    #[inline]
    #[track_caller]
    pub fn copy_from_slice(slice: &[T]) -> Self
    where
        T: Copy,
    {
        Self::try_copy_from_slice(slice).expect("failed to create arc slice from slice")
    }

    #[inline]
    pub fn try_copy_from_slice(slice: &[T]) -> ArcResult<Self>
    where
        T: Copy,
    {
        let length = slice.len();
        let mut uninit = Self::new_uninit(length)?;

        unsafe {
            utils::copy_to_uninit_slice(uninit.as_mut_slice(), slice);

            // Safety: All elements within `uninit` are valid
            Ok(Self::assume_init(uninit))
        }
    }

    #[inline]
    #[track_caller]
    pub fn clone_from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        Self::try_clone_from_slice(slice).expect("failed to create arc slice from slice")
    }

    #[inline]
    pub fn try_clone_from_slice(slice: &[T]) -> ArcResult<Self>
    where
        T: Clone,
    {
        let length = slice.len();
        let mut uninit = Self::new_uninit(length)?;

        unsafe {
            utils::clone_to_uninit_slice(uninit.as_mut_slice(), slice);

            // Safety: All elements within `uninit` are valid
            Ok(Self::assume_init(uninit))
        }
    }

    #[inline]
    #[track_caller]
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self::try_from_vec(vec).expect("failed to create arc slice from vec")
    }

    #[inline]
    pub fn try_from_vec(vec: Vec<T>) -> ArcResult<Self> {
        let length = vec.len();
        let vec = {
            // TODO: We'd rather use `Vec::into_raw_parts()` here, rust/#65816
            let mut vec = ManuallyDrop::new(vec);
            let capacity = vec.capacity();
            let ptr = vec.as_mut_ptr();

            // Safety: It's valid to reinterpret a `Vec<T>` as a `Vec<MaybeUninit<T>>`
            unsafe { Vec::<MaybeUninit<T>>::from_raw_parts(ptr.cast(), length, capacity) }
        };

        let uninit = Self::new_uninit(length)?;
        unsafe {
            // Safety: `vec` is filled with valid elements up to `length` and `uninit`
            //         is allocated up to `length`
            ptr::copy_nonoverlapping(vec.as_ptr(), uninit.as_mut_ptr(), length);

            // Safety: All elements within `uninit` are valid
            Ok(Self::assume_init(uninit))
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        // Safety: inner is a valid `ArcInner`
        unsafe { *ArcInner::data(self.inner) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        // Safety: We know our inner value is valid
        unsafe { ArcInner::value(self.inner).cast() }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // Safety: Our allocated slice has exactly `len` elements
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }
}

impl<T, A> ArcSlice<T, A>
where
    A: Atomic,
{
    #[inline]
    fn counter(&self) -> &A {
        // Safety: We know our inner value is valid
        unsafe { &*ArcInner::counter(self.inner) }
    }

    #[inline]
    fn as_mut_ptr(&self) -> *mut T {
        // Safety: We know our inner value is valid
        unsafe { ArcInner::value_mut(self.inner).cast() }
    }

    #[inline]
    unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        // Safety: Our allocated slice has exactly `len` elements
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }

    #[inline]
    unsafe fn assume_init(this: ArcSlice<MaybeUninit<T>, A>) -> Self {
        let this = ManuallyDrop::new(this);

        Self {
            inner: this.inner.cast(),
        }
    }

    #[inline]
    fn new_uninit(length: usize) -> ArcResult<ArcSlice<MaybeUninit<T>, A>> {
        // Create the layout of the trailing slice
        let slice_layout = match Layout::array::<T>(length) {
            Ok(layout) => layout,
            Err(_) => return Err(ArcError::AllocError),
        };

        // Create the layout for the arc
        let layout = ArcInner::<[T; 0], usize, A>::layout(slice_layout)?;
        // The layout's size can never be zero
        debug_assert_ne!(layout.size(), 0);

        // Allocate the memory for the arc
        // Safety: `layout` never has a size of zero
        let inner = match NonNull::new(unsafe { alloc::alloc::alloc(layout) }) {
            Some(ptr) => ptr.cast::<ArcInner<[MaybeUninit<T>; 0], usize, A>>(),
            None => return Err(ArcError::AllocError),
        };

        // Initialize the allocation with the provided data
        // Safety: `inner` is a valid pointer to an allocated `ArcInner`
        unsafe {
            ArcInner::data_mut(inner).write(length);
            ArcInner::counter_mut(inner).write(A::ONE);
            // Note: We don't initialize the slice since this is a `MaybeUninit`
        }

        Ok(ArcSlice { inner })
    }

    #[inline(never)]
    unsafe fn drop_slow(&mut self) {
        let len = self.len();

        // Statically check if the contained types need dropping and if so, drop them
        // Safety: inner points to a valid `ArcInner` that has not yet been dropped
        unsafe {
            // Note: `usize` doesn't need any drop code run

            if needs_drop::<A>() {
                ptr::drop_in_place(ArcInner::counter_mut(self.inner));
            }

            if needs_drop::<T>() {
                let slice_ptr = ArcInner::value_mut(self.inner).cast::<T>();
                let slice = ptr::slice_from_raw_parts_mut(slice_ptr, len);
                ptr::drop_in_place(slice);
            }
        }

        let slice_layout = Layout::array::<T>(len).unwrap_or_else(|_| {
            if cfg!(debug_assertions) {
                unreachable!("the arc slice was created, therefore its layout is valid")
            } else {
                // Safety: The arc slice was created and therefore its layout must
                //         be creatable
                unsafe { core::hint::unreachable_unchecked() }
            }
        });

        // Build the layout we allocated the arc with
        let layout = ArcInner::<[T; 0], usize, A>::layout(slice_layout).unwrap_or_else(|_| {
            if cfg!(debug_assertions) {
                unreachable!("the arc was created, therefore its layout is valid")
            } else {
                // Safety: The arc was created and therefore its layout must
                //         be creatable
                unsafe { core::hint::unreachable_unchecked() }
            }
        });

        // Deallocate the arc's backing data
        // Safety: The backing memory has not yet been deallocated
        unsafe { alloc::alloc::dealloc(self.inner.as_ptr().cast(), layout) };
    }
}

impl<T, A> Deref for ArcSlice<T, A>
where
    A: Atomic,
{
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, A> AsRef<[T]> for ArcSlice<T, A>
where
    A: Atomic,
{
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, A> Debug for ArcSlice<T, A>
where
    T: Debug,
    A: Atomic,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}

impl<T, A> Clone for ArcSlice<T, A>
where
    A: Atomic,
{
    #[inline]
    fn clone(&self) -> Self {
        // Using a relaxed ordering is alright here, as knowledge of the
        // original reference prevents other threads from erroneously deleting
        // the object.
        //
        // As explained in the [Boost documentation][1], Increasing the
        // reference counter can always be done with memory_order_relaxed: New
        // references to an object can only be formed from an existing
        // reference, and passing an existing reference from one thread to
        // another must already provide any required synchronization.
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        let old_size = self.counter().fetch_add(<A::Raw>::ONE, Ordering::Relaxed);

        // However we need to guard against massive refcounts in case someone
        // is `mem::forget`ing Arcs. If we don't do this the count can overflow
        // and users will use-after free. We racily saturate to `isize::MAX` on
        // the assumption that there aren't ~2 billion threads incrementing
        // the reference count at once. This branch will never be taken in
        // any realistic program.
        //
        // We abort because such a program is incredibly degenerate, and we
        // don't care to support it.
        if old_size > <A::Raw>::MAX_REFCOUNT {
            utils::abort();
        }

        Self { inner: self.inner }
    }
}

impl<T, A> Drop for ArcSlice<T, A>
where
    A: Atomic,
{
    fn drop(&mut self) {
        let counter = self.counter();

        // Because `fetch_sub` is already atomic, we do not need to synchronize
        // with other threads unless we are going to delete the object
        if !counter.fetch_sub(<A::Raw>::ONE, Ordering::Release).is_one() {
            return;
        }

        // This fence is needed to prevent reordering of use of the data and
        // deletion of the data.  Because it is marked `Release`, the decreasing
        // of the reference count synchronizes with this `Acquire` fence. This
        // means that use of the data happens before decreasing the reference
        // count, which happens before this fence, which happens before the
        // deletion of the data.
        //
        // As explained in the [Boost documentation][1],
        //
        // > It is important to enforce any possible access to the object in one
        // > thread (through an existing reference) to *happen before* deleting
        // > the object in a different thread. This is achieved by a "release"
        // > operation after dropping a reference (any access to the object
        // > through this reference must obviously happened before), and an
        // > "acquire" operation before deleting the object.
        //
        // In particular, while the contents of an Arc are usually immutable, it's
        // possible to have interior writes to something like a Mutex<T>. Since a
        // Mutex is not acquired when it is deleted, we can't rely on its
        // synchronization logic to make writes in thread A visible to a destructor
        // running in thread B.
        //
        // Also note that the Acquire fence here could probably be replaced with an
        // Acquire load, which could improve performance in highly-contended
        // situations. See [2].
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        // [2]: (https://github.com/rust-lang/rust/pull/41714)
        counter.load(Ordering::Acquire);

        unsafe { self.drop_slow() };
    }
}

// TODO: Hybrid arc?
// TODO: Utilize custom allocators
#[repr(transparent)]
pub struct Arc<T: ?Sized, D = (), A: Atomic = AtomicUsize> {
    inner: NonNull<ArcInner<T, D, A>>,
}

impl<T, D, A> Arc<T, D, A>
where
    A: Atomic,
{
    #[inline]
    #[track_caller]
    pub fn new(value: T, data: D) -> Self {
        Self::try_new(value, data).expect("failed to create new arc")
    }

    #[inline]
    pub fn try_new(value: T, data: D) -> ArcResult<Self> {
        // Create the layout for the arc
        let layout = ArcInner::<T, D, A>::layout(Layout::new::<T>())?;
        // The layout's size can never be zero
        debug_assert_ne!(layout.size(), 0);

        // Allocate the memory for the arc
        // Safety: `layout` never has a size of zero
        let inner = match NonNull::new(unsafe { alloc::alloc::alloc(layout) }) {
            Some(ptr) => ptr.cast::<ArcInner<T, D, A>>(),
            None => return Err(ArcError::AllocError),
        };

        // Initialize the allocation with the provided data
        // Safety: `inner` is a valid pointer to an allocated `ArcInner`
        unsafe {
            ArcInner::data_mut(inner).write(data);
            ArcInner::counter_mut(inner).write(A::ONE);
            ArcInner::value_mut(inner).write(value);
        }

        Ok(Self { inner })
    }

    #[inline]
    pub fn data(&self) -> &D {
        // Safety: The data is valid and initialized
        unsafe { &*ArcInner::data(self.inner) }
    }
}

impl<T, D, A> Arc<T, D, A>
where
    T: ?Sized,
    A: Atomic,
{
    /// Creates a new arc from the given pointer
    ///
    /// # Safety
    ///
    /// There must be enough references to the inner arc to not cause
    /// a double-free upon dropping the created arc
    ///
    #[inline]
    const unsafe fn from_inner(inner: NonNull<ArcInner<T, D, A>>) -> Self {
        Self { inner }
    }

    #[inline]
    fn counter(&self) -> &A {
        // Safety: The counter is valid and initialized
        unsafe { &*ArcInner::counter(self.inner) }
    }

    #[inline]
    fn value(&self) -> &T {
        // Safety: The value is valid and initialized
        unsafe { &*ArcInner::value(self.inner) }
    }

    /// Fully drops and deallocates the current `Arc`
    ///
    /// # Safety
    ///
    /// - We must have exclusive access to the current `Arc`
    /// - The current `Arc` must not have been previously freed
    /// - The `inner` pointer of the current `Arc` must point to valid
    ///   memory that has not yet been dropped
    #[inline(never)]
    unsafe fn drop_slow(&mut self) {
        // Statically check if the contained types need dropping and if so, drop them
        // Safety: inner points to a valid `ArcInner` that has not yet been dropped
        let value_layout = unsafe {
            if needs_drop::<D>() {
                ptr::drop_in_place(ArcInner::data_mut(self.inner));
            }

            if needs_drop::<A>() {
                ptr::drop_in_place(ArcInner::counter_mut(self.inner));
            }

            // Get the layout of our inner value
            let value_ptr = ArcInner::value_mut(self.inner);
            let value_layout = Layout::for_value::<T>(&*value_ptr);
            ptr::drop_in_place(value_ptr);

            value_layout
        };

        // Build the layout we allocated the arc with
        let layout = ArcInner::<T, D, A>::layout(value_layout).unwrap_or_else(|_| {
            if cfg!(debug_assertions) {
                unreachable!("the arc was created, therefore its layout is valid")
            } else {
                // Safety: The arc was created and therefore its layout must
                //         be creatable
                unsafe { core::hint::unreachable_unchecked() }
            }
        });

        // Deallocate the arc's backing data
        // Safety: The backing memory has not yet been deallocated
        unsafe { alloc::alloc::dealloc(self.inner.as_ptr().cast(), layout) };
    }
}

impl<T, D, A> Deref for Arc<T, D, A>
where
    T: ?Sized,
    A: Atomic,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<T, D, A> Clone for Arc<T, D, A>
where
    T: ?Sized,
    A: Atomic,
{
    #[inline]
    fn clone(&self) -> Self {
        // Using a relaxed ordering is alright here, as knowledge of the
        // original reference prevents other threads from erroneously deleting
        // the object.
        //
        // As explained in the [Boost documentation][1], Increasing the
        // reference counter can always be done with memory_order_relaxed: New
        // references to an object can only be formed from an existing
        // reference, and passing an existing reference from one thread to
        // another must already provide any required synchronization.
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        let old_size = self.counter().fetch_add(<A::Raw>::ONE, Ordering::Relaxed);

        // However we need to guard against massive refcounts in case someone
        // is `mem::forget`ing Arcs. If we don't do this the count can overflow
        // and users will use-after free. We racily saturate to `isize::MAX` on
        // the assumption that there aren't ~2 billion threads incrementing
        // the reference count at once. This branch will never be taken in
        // any realistic program.
        //
        // We abort because such a program is incredibly degenerate, and we
        // don't care to support it.
        if old_size > <A::Raw>::MAX_REFCOUNT {
            utils::abort();
        }

        unsafe { Self::from_inner(self.inner) }
    }
}

impl<T, D, A> Drop for Arc<T, D, A>
where
    T: ?Sized,
    A: Atomic,
{
    #[inline]
    fn drop(&mut self) {
        let counter = self.counter();

        // Because `fetch_sub` is already atomic, we do not need to synchronize
        // with other threads unless we are going to delete the object
        if !counter.fetch_sub(<A::Raw>::ONE, Ordering::Release).is_one() {
            return;
        }

        // This fence is needed to prevent reordering of use of the data and
        // deletion of the data.  Because it is marked `Release`, the decreasing
        // of the reference count synchronizes with this `Acquire` fence. This
        // means that use of the data happens before decreasing the reference
        // count, which happens before this fence, which happens before the
        // deletion of the data.
        //
        // As explained in the [Boost documentation][1],
        //
        // > It is important to enforce any possible access to the object in one
        // > thread (through an existing reference) to *happen before* deleting
        // > the object in a different thread. This is achieved by a "release"
        // > operation after dropping a reference (any access to the object
        // > through this reference must obviously happened before), and an
        // > "acquire" operation before deleting the object.
        //
        // In particular, while the contents of an Arc are usually immutable, it's
        // possible to have interior writes to something like a Mutex<T>. Since a
        // Mutex is not acquired when it is deleted, we can't rely on its
        // synchronization logic to make writes in thread A visible to a destructor
        // running in thread B.
        //
        // Also note that the Acquire fence here could probably be replaced with an
        // Acquire load, which could improve performance in highly-contended
        // situations. See [2].
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        // [2]: (https://github.com/rust-lang/rust/pull/41714)
        counter.load(Ordering::Acquire);

        unsafe { self.drop_slow() };
    }
}

#[repr(C)]
struct ArcInner<T: ?Sized, D, A> {
    data: D,
    counter: A,
    value: T,
}

impl<T: ?Sized, D, A> ArcInner<T, D, A> {
    /// Create a layout for an `ArcInner` containing the given types along with
    /// a provided layout for the `value` field of `ArcInner`
    fn layout(value: Layout) -> ArcResult<Layout> {
        let data = Layout::new::<D>();
        let counter = Layout::new::<A>();

        // Note: We use `match` here to reduce the amount of IR that's generated
        //       since everything to do with allocation tends to be rather
        //       heavily monomorphized
        let layout = match data.extend(counter) {
            Ok((layout, _)) => layout,
            Err(_) => return Err(ArcError::LayoutError),
        };
        let layout = match layout.extend(value) {
            Ok((layout, _)) => layout,
            Err(_) => return Err(ArcError::LayoutError),
        };

        Ok(layout.pad_to_align())
    }

    /// Gets a pointer to the data field of the current arc
    ///
    /// # Safety
    ///
    /// `this` must be a valid pointer to an `ArcInner<T, D, A>`
    #[inline]
    unsafe fn data(this: NonNull<Self>) -> *const D {
        unsafe { addr_of!((*this.as_ptr()).data) }
    }

    /// Gets a mutable pointer to the data field of the current arc
    ///
    /// # Safety
    ///
    /// `this` must be a valid pointer to an `ArcInner<T, D, A>`
    #[inline]
    unsafe fn data_mut(this: NonNull<Self>) -> *mut D {
        unsafe { addr_of_mut!((*this.as_ptr()).data) }
    }

    /// Gets a pointer to the counter field of the current arc
    ///
    /// # Safety
    ///
    /// `this` must be a valid pointer to an `ArcInner<T, D, A>`
    #[inline]
    unsafe fn counter(this: NonNull<Self>) -> *const A {
        unsafe { addr_of!((*this.as_ptr()).counter) }
    }

    /// Gets a mutable pointer to the counter field of the current arc
    ///
    /// # Safety
    ///
    /// `this` must be a valid pointer to an `ArcInner<T, D, A>`
    #[inline]
    unsafe fn counter_mut(this: NonNull<Self>) -> *mut A {
        unsafe { addr_of_mut!((*this.as_ptr()).counter) }
    }

    /// Gets a pointer to the value field of the current arc
    ///
    /// # Safety
    ///
    /// `this` must be a valid pointer to an `ArcInner<T, D, A>`
    #[inline]
    unsafe fn value(this: NonNull<Self>) -> *const T {
        unsafe { addr_of!((*this.as_ptr()).value) }
    }

    /// Gets a mutable pointer to the value field of the current arc
    ///
    /// # Safety
    ///
    /// `this` must be a valid pointer to an `ArcInner<T, D, A>`
    #[inline]
    unsafe fn value_mut(this: NonNull<Self>) -> *mut T {
        unsafe { addr_of_mut!((*this.as_ptr()).value) }
    }
}

pub trait Atomic: sealed::Sealed + Sized {
    type Raw: Copy + RawAtomic;

    const ONE: Self;

    fn fetch_add(&self, val: Self::Raw, order: Ordering) -> Self::Raw;

    fn fetch_sub(&self, val: Self::Raw, ordering: Ordering) -> Self::Raw;

    fn load(&self, order: Ordering) -> Self::Raw;
}

pub trait RawAtomic: sealed::Sealed + Sized + Copy + PartialEq + Eq + PartialOrd + Ord {
    const ONE: Self;
    const MAX_REFCOUNT: Self;

    #[inline]
    fn is_one(&self) -> bool {
        *self == Self::ONE
    }
}

impl Atomic for AtomicUsize {
    type Raw = usize;

    #[allow(clippy::declare_interior_mutable_const)]
    const ONE: Self = Self::new(1);

    #[inline]
    fn fetch_add(&self, val: Self::Raw, order: Ordering) -> Self::Raw {
        AtomicUsize::fetch_add(self, val, order)
    }

    #[inline]
    fn fetch_sub(&self, val: Self::Raw, order: Ordering) -> Self::Raw {
        AtomicUsize::fetch_sub(self, val, order)
    }

    #[inline]
    fn load(&self, order: Ordering) -> Self::Raw {
        AtomicUsize::load(self, order)
    }
}

impl RawAtomic for usize {
    const ONE: Self = 1;

    /// A soft limit on the amount of references that may be made to an `Arc`.
    ///
    /// Going above this limit will abort your program (although not
    /// necessarily) at _exactly_ `MAX_REFCOUNT + 1` references.
    const MAX_REFCOUNT: Self = isize::MAX as usize;
}

mod sealed {
    use core::sync::atomic::AtomicUsize;

    pub trait Sealed {}

    macro_rules! sealed {
        ($($ty:ident),* $(,)?) => {
            $(impl Sealed for $ty {})*
        };
    }

    sealed! {
        usize,
        AtomicUsize,
    }
}
