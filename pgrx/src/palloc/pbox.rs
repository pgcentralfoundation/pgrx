//! Lifetime-bound boxed allocations in a Postgres memory context.
//!
//! [`PBox<'mcx, T>`] is the safe, lifetime-bound analogue of [`alloc::boxed::Box`]
//! for memory allocated inside a [`MemCx<'mcx>`]. Unlike `Box`, the value is
//! reclaimed when the memory context is reset — not when `PBox` is dropped.
//!
//! # Examples
//!
//! Allocating a zero-initialized slice and reading it back:
//!
//! ```no_run
//! use pgrx::memcx;
//! use pgrx::palloc::PBox;
//! use core::mem::MaybeUninit;
//!
//! memcx::current_context(|cx| {
//!     let s = PBox::<[MaybeUninit<u32>]>::new_zeroed_slice_in(8, cx)
//!         .expect("OOM");
//!     // SAFETY: zero is a valid bit pattern for u32.
//!     let s: PBox<[u32]> = unsafe { s.assume_init() };
//!     assert_eq!(s.len(), 8);
//!     assert!(s.iter().all(|&n| n == 0));
//! });
//! ```
//!
//! Copying from an existing slice:
//!
//! ```no_run
//! use pgrx::memcx;
//! use pgrx::palloc::PBox;
//!
//! memcx::current_context(|cx| {
//!     let src = [1u32, 2, 3, 4];
//!     let dst: PBox<[u32]> = PBox::from_slice_in(&src, cx).expect("OOM");
//!     assert_eq!(&*dst, &src);
//! });
//! ```

use crate::callconv::{BoxRet, FcInfo};
use crate::datum::{BorrowDatum, Datum};
use crate::layout::PassBy;
use crate::memcx::{MemCx, OutOfMemory};
use crate::pg_sys;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use pgrx_sql_entity_graph::metadata::{
    ArgumentError, ReturnsError, ReturnsRef, SqlMappingRef, SqlTranslatable,
};

/** As [`Box<T, A>`][stdbox] where `A` is a [`MemCx`]


[stdbox]: alloc::boxed::Box

Lifetime escape is statically rejected. The following snippet must fail to type-check because the `MemCx`-bound lifetime cannot outlive the context it was borrowed from:

```compile_fail
use pgrx::memcx::MemCx;
use pgrx::palloc::PBox;

fn smuggle<'long, 'short>(cx: &MemCx<'short>) -> PBox<'long, [u32]> {
    PBox::<[core::mem::MaybeUninit<u32>]>::new_uninit_slice_in(4, cx)
        .map(|p| unsafe { p.assume_init() })
        .unwrap()
}
```
*/
#[repr(transparent)]
pub struct PBox<'mcx, T: ?Sized> {
    ptr: NonNull<T>,
    _cx: PhantomData<MemCx<'mcx>>,
}

impl<'mcx, T: ?Sized> PBox<'mcx, T> {
    // # Safety
    // The same constraints as [`Box::from_raw`]
    // - you assert the pointer was allocated in the `MemCx`
    // - you assert the pointer may be freed by `pfree`
    pub unsafe fn from_raw_in(ptr: NonNull<T>, _cx: &MemCx<'mcx>) -> PBox<'mcx, T> {
        PBox { ptr, _cx: PhantomData }
    }
}

impl<'mcx, T: Sized> PBox<'mcx, T> {
    #[track_caller]
    pub fn new_in(val: T, memcx: &MemCx<'mcx>) -> Self {
        PBox::try_new_in(val, memcx).unwrap()
    }

    pub fn try_new_in(val: T, memcx: &MemCx<'mcx>) -> Result<Self, OutOfMemory> {
        const { assert!(align_of::<T>() <= size_of::<pg_sys::Datum>()) };
        let ptr = memcx.alloc_bytes(size_of::<T>())?.cast();
        // SAFETY: We were guaranteed an appropriately sized allocation to write to, and we have asserted our alignment maximum was upheld
        unsafe { ptr.write(val) };
        Ok(PBox { ptr, _cx: PhantomData })
    }
}

impl<'mcx, T: ?Sized> PBox<'mcx, T> {
    /// Raw pointer to the boxed value. Does not dereference.
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr() as *const T
    }
    /// Raw mutable pointer to the boxed value. Does not dereference.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<'mcx, T> PBox<'mcx, [MaybeUninit<T>]> {
    /// Allocate a slice of `len` uninitialized `T` inside `cx`.
    ///
    /// Mirrors [`alloc::boxed::Box::new_uninit_slice`] but the resulting slice is bound to the lifetime of `cx`.
    pub fn new_uninit_slice_in(len: usize, cx: &MemCx<'mcx>) -> Result<Self, OutOfMemory> {
        let layout = core::alloc::Layout::array::<T>(len).map_err(|_| OutOfMemory::new())?;
        let raw = cx.alloc_layout(layout)?;
        // SAFETY: `raw` is non-null, sized by `layout`; MaybeUninit<T> needs no initialization, so building the fat pointer over `len` elements is sound.
        let slice_ptr: *mut [MaybeUninit<T>] =
            core::ptr::slice_from_raw_parts_mut(raw.as_ptr().cast::<MaybeUninit<T>>(), len);
        let ptr = unsafe { NonNull::new_unchecked(slice_ptr) };
        Ok(PBox { ptr, _cx: PhantomData })
    }

    /// Allocate a slice of `len` zero-initialized `T` (kept as `MaybeUninit<T>`) inside `cx`. Caller can transmute via [`assume_init`] when zero is a valid bit pattern for `T`.
    pub fn new_zeroed_slice_in(len: usize, cx: &MemCx<'mcx>) -> Result<Self, OutOfMemory> {
        let layout = core::alloc::Layout::array::<T>(len).map_err(|_| OutOfMemory::new())?;
        let raw = cx.alloc_layout_zeroed(layout)?;
        // SAFETY: `raw` is non-null, sized by `layout`; MaybeUninit<T> needs no init.
        let slice_ptr: *mut [core::mem::MaybeUninit<T>] = core::ptr::slice_from_raw_parts_mut(
            raw.as_ptr().cast::<core::mem::MaybeUninit<T>>(),
            len,
        );
        let ptr = unsafe { NonNull::new_unchecked(slice_ptr) };
        Ok(PBox { ptr, _cx: PhantomData })
    }

    /// # Safety
    /// Caller asserts every element of the slice has been initialized.
    pub unsafe fn assume_init(self) -> PBox<'mcx, [T]> {
        let p = self.ptr.as_ptr() as *mut [T];
        // SAFETY: caller upholds the init invariant; the data pointer was already non-null (it came from PBox::new_uninit_slice_in / new_zeroed_slice_in).
        let ptr = unsafe { NonNull::new_unchecked(p) };
        // Avoid running any (non-existent) Drop on `self`.
        core::mem::forget(self);
        PBox { ptr, _cx: PhantomData }
    }

    /// Like [`new_uninit_slice_in`] but permits allocations above Postgres's 1 GiB `MaxAllocSize`. Only use for genuinely huge Rust-side scratch buffers — values returned to Postgres still cap at 1 GiB regardless of how they were allocated.
    ///
    /// [`new_uninit_slice_in`]: PBox::new_uninit_slice_in
    pub fn new_huge_uninit_slice_in(len: usize, cx: &MemCx<'mcx>) -> Result<Self, OutOfMemory> {
        let layout = core::alloc::Layout::array::<T>(len).map_err(|_| OutOfMemory::new())?;
        let raw = cx.alloc_huge_layout(layout)?;
        // SAFETY: same as new_uninit_slice_in.
        let slice_ptr: *mut [MaybeUninit<T>] =
            core::ptr::slice_from_raw_parts_mut(raw.as_ptr().cast::<MaybeUninit<T>>(), len);
        let ptr = unsafe { NonNull::new_unchecked(slice_ptr) };
        Ok(PBox { ptr, _cx: PhantomData })
    }

    /// Zeroed counterpart of [`new_huge_uninit_slice_in`].
    pub fn new_huge_zeroed_slice_in(len: usize, cx: &MemCx<'mcx>) -> Result<Self, OutOfMemory> {
        let layout = core::alloc::Layout::array::<T>(len).map_err(|_| OutOfMemory::new())?;
        let raw = cx.alloc_huge_layout_zeroed(layout)?;
        // SAFETY: same as new_zeroed_slice_in.
        let slice_ptr: *mut [MaybeUninit<T>] =
            core::ptr::slice_from_raw_parts_mut(raw.as_ptr().cast::<MaybeUninit<T>>(), len);
        let ptr = unsafe { NonNull::new_unchecked(slice_ptr) };
        Ok(PBox { ptr, _cx: PhantomData })
    }
}

impl<'mcx, T> PBox<'mcx, [T]> {
    /// Number of elements in the slice.
    pub fn len(&self) -> usize {
        self.ptr.len()
    }

    /// `true` if the slice has zero elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'mcx, T: Copy> PBox<'mcx, [T]> {
    /// Allocate a slice of `src.len()` elements in `cx` and copy `src` into it.
    pub fn from_slice_in(src: &[T], cx: &MemCx<'mcx>) -> Result<Self, OutOfMemory> {
        let mut uninit = PBox::<[MaybeUninit<T>]>::new_uninit_slice_in(src.len(), cx)?;
        // SAFETY: `uninit.len() == src.len()` by construction. T: Copy, so a bytewise copy fully initializes every element with no Drop concerns.
        unsafe {
            let dst = (uninit.as_mut_ptr() as *mut MaybeUninit<T>).cast::<T>();
            core::ptr::copy_nonoverlapping(src.as_ptr(), dst, src.len());
            Ok(uninit.assume_init())
        }
    }
}

impl<'mcx, T> PBox<'mcx, [T]> {
    /// Allocate a slice of `iter.len()` elements in `cx` and write each element produced by `iter`. Requires `ExactSizeIterator` so the allocation size is known upfront with no resizing.
    ///
    /// On panic during iteration, the initialized prefix is dropped exactly once. The underlying memory remains in `cx` until the context resets.
    ///
    /// # Errors
    ///
    /// - `Err(OutOfMemory)` if the initial allocation fails.
    /// - `Err(OutOfMemory)` if `iter` yields **fewer** items than its
    ///   `ExactSizeIterator::len()` advertised. This is technically a contract violation by the iterator rather than a true OOM, but
    ///   reusing `OutOfMemory` avoids introducing a new error type for an edge case `ExactSizeIterator` callers shouldn't hit. In this case the initialized prefix is dropped via the RAII guard before the error returns; the buffer itself stays in `cx`.
    ///
    /// If `iter` yields **more** items than promised, the surplus items are dropped at the loop boundary and the function returns `Ok` with the first `len` elements.
    pub fn from_iter_in<I>(iter: I, cx: &MemCx<'mcx>) -> Result<Self, OutOfMemory>
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        let iter = iter.into_iter();
        let len = iter.len();
        let mut uninit = PBox::<[MaybeUninit<T>]>::new_uninit_slice_in(len, cx)?;

        struct Guard<T> {
            base: *mut T,
            initialized: usize,
        }
        impl<T> Drop for Guard<T> {
            fn drop(&mut self) {
                if self.initialized > 0 {
                    // SAFETY: the first `initialized` elements were written via ptr::write and are valid T.
                    unsafe {
                        let s = core::slice::from_raw_parts_mut(self.base, self.initialized);
                        core::ptr::drop_in_place(s);
                    }
                }
            }
        }

        // Get a thin pointer to the first element of the uninit buffer.
        let base: *mut T = uninit.as_mut_ptr().cast::<T>();
        let mut guard = Guard { base, initialized: 0 };

        let mut count = 0usize;
        for item in iter {
            if count >= len {
                // Iterator over-yielded vs ExactSizeIterator promise. Stop here;
                // we'll fail below since `count < len`.
                break;
            }
            // SAFETY: `count < len`, so `base.add(count)` is in-bounds for the freshly allocated slice. The slot is uninit MaybeUninit<T>; writing a T is sound.
            unsafe { core::ptr::write(base.add(count), item) };
            count += 1;
            guard.initialized = count;
        }

        if count < len {
            // Iterator under-yielded. Drop the initialized prefix via Guard,
            // and return an error. The buffer itself stays in `cx`.
            return Err(OutOfMemory::new());
        }

        // All elements initialized successfully — disarm the guard and assume_init.
        core::mem::forget(guard);
        // SAFETY: every slot in the slice was written above (count == len).
        Ok(unsafe { uninit.assume_init() })
    }
}

unsafe impl<'mcx, T> BoxRet for PBox<'mcx, T>
where
    T: ?Sized + BorrowDatum,
{
    unsafe fn box_into<'fcx>(self, fcinfo: &mut FcInfo<'fcx>) -> Datum<'fcx> {
        let datum = match T::PASS {
            PassBy::Value => {
                // start with a zeroed Datum, just to minimize funny business
                let mut datum = pg_sys::Datum::null();
                // SAFETY: Due to BorrowDatum, this type has a definite size less than a Datum, and PBox must have an initialized pointee, so a copy is sound-by-construction.
                unsafe {
                    let size = size_of_val(&*self.ptr.as_ptr());
                    debug_assert!(size <= size_of::<pg_sys::Datum>());
                    // using `BorrowDatum::point_from` handles endianness
                    let datum_ptr = T::point_from(NonNull::from_mut(&mut datum).cast::<u8>());
                    datum_ptr.cast::<u8>().copy_from_nonoverlapping(self.ptr.cast(), size);
                }
                datum
            }
            PassBy::Ref => pg_sys::Datum::from(self.ptr.cast::<u8>().as_ptr()),
        };
        // SAFETY: by proxy, BorrowDatum is an `unsafe trait` so the above impl must be correct
        unsafe { fcinfo.return_raw_datum(datum) }
    }
}

/// SAFETY: SQL has no "pointers" so by-val and by-ref calling conventions are identical,and all `PBox` truly does is enable pass-by-ref returns of unsized values.
unsafe impl<'mcx, T> SqlTranslatable for PBox<'mcx, T>
where
    T: SqlTranslatable + ?Sized,
{
    const TYPE_IDENT: &'static str = T::TYPE_IDENT;
    const TYPE_ORIGIN: pgrx_sql_entity_graph::metadata::TypeOrigin = T::TYPE_ORIGIN;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = T::ARGUMENT_SQL;
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> = T::RETURN_SQL;
}

impl<'mcx, T> Deref for PBox<'mcx, T>
where
    T: BorrowDatum + ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: by construction
        unsafe { self.ptr.as_ref() }
    }
}

impl<'mcx, T> DerefMut for PBox<'mcx, T>
where
    T: BorrowDatum + ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: by construction
        unsafe { self.ptr.as_mut() }
    }
}

impl<'mcx, T> Deref for PBox<'mcx, [T]> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        // SAFETY: `self.ptr` is a valid fat pointer to an initialized [T] slice (the Sized constructors only return PBox<[T]> after assume_init or from_slice_in/from_iter_in, all of which uphold initialization).
        unsafe { self.ptr.as_ref() }
    }
}

impl<'mcx, T> DerefMut for PBox<'mcx, [T]> {
    fn deref_mut(&mut self) -> &mut [T] {
        // SAFETY: same as Deref, plus we hold `&mut self` so aliasing is exclusive.
        unsafe { self.ptr.as_mut() }
    }
}
