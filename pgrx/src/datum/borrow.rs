#![deny(unsafe_op_in_unsafe_fn)]
use super::*;
use crate::layout::PassBy;
use core::{ffi, mem, ptr};

/// Types which can be "borrowed from" [`&Datum<'_>`] via simple cast, deref, or slicing
///
/// # Safety
/// Despite its pleasant-sounding name, this implements a fairly low-level detail.
/// It exists to allow other code to use that nice-sounding BorrowDatum bound.
/// Outside of the pgrx library, it is probably incorrect to call and rely on this:
/// instead use the convenience functions available in `pgrx::datum`.
// TODO: implement those.
///
/// Its behavior is trusted for ABI details, and it should not be implemented if any doubt
/// exists of whether the type would be suitable for passing via Postgres.
pub unsafe trait BorrowDatum {
    /// The "native" passing convention for this type.
    ///
    /// - `PassBy::Value` implies [`mem::size_of<T>()`][size_of] <= [`mem::size_of::<Datum>()`][Datum].
    /// - `PassBy::Ref` means the pointee will occupy at least 1 byte for variable-sized types.
    ///
    /// Note that this means a zero-sized type is inappropriate for `BorrowDatum`.
    const PASS: PassBy;

    /// Cast a pointer to this blob of bytes to a pointer to this type.
    ///
    /// This is not a simple `ptr.cast()` because it may be *unsizing*, which may require
    /// reading varlena headers. For all fixed-size types, `ptr.cast()` should be correct.
    ///
    /// # Safety
    /// - This must be correctly invoked for the pointee type, as it may deref and read one or more
    ///   bytes in its implementation in order to read the inline metadata and unsize the type.
    ///
    /// ## For Implementers
    /// Reading the **first** byte pointed to is permitted if `T::PASS = PassBy::Ref`, assuming you
    /// are implementing a varlena type. As the other dynamic length type, CStr also does this.
    /// This function
    /// - must NOT mutate the pointee
    /// - must return a pointer with provenance to the entire allocation
    ///
    /// Do not attempt to handle pass-by-value versus pass-by-ref in this fn's body!
    /// A caller may be in a context where all types are handled by-reference, for instance.
    unsafe fn point_from(ptr: *mut u8) -> *mut Self;

    /// Cast a pointer to aligned varlena headers to this type
    ///
    /// This version allows you to assume the pointer is aligned to, and readable for, 4 bytes.
    /// This optimization is not required. When in doubt, avoid implementing it, and rely on your
    /// `point_from` implementation alone.
    ///
    /// # Safety
    /// - This must be correctly invoked for the pointee type, as it may deref.
    /// - This must be 4-byte aligned!
    unsafe fn point_from_align4(ptr: *mut u32) -> *mut Self {
        debug_assert!(ptr.is_aligned());
        unsafe { <Self as BorrowDatum>::point_from(ptr.cast()) }
    }

    /// Optimization for borrowing the referent
    unsafe fn borrow_unchecked<'dat>(ptr: *const u8) -> &'dat Self {
        unsafe { &*Self::point_from(ptr.cast_mut()) }
    }
}

macro_rules! borrow_by_value {
    ($($value_ty:ty),*) => {
        $(
            unsafe impl BorrowDatum for $value_ty {
                const PASS: PassBy = if mem::size_of::<Self>() <= mem::size_of::<Datum>() {
                    PassBy::Value
                } else {
                    PassBy::Ref
                };

                unsafe fn point_from(ptr: *mut u8) -> *mut Self {
                    ptr.cast()
                }
            }
        )*
    }
}

borrow_by_value! {
    i8, i16, i32, i64, bool, f32, f64, pg_sys::Oid, Date, Time, Timestamp, TimestampWithTimeZone
}

/// It is rare to pass CStr via Datums, but not unheard of
unsafe impl BorrowDatum for ffi::CStr {
    const PASS: PassBy = PassBy::Ref;

    unsafe fn point_from(ptr: *mut u8) -> *mut Self {
        let char_ptr: *mut ffi::c_char = ptr.cast();
        let len = unsafe { ffi::CStr::from_ptr(char_ptr).to_bytes_with_nul().len() };
        ptr::slice_from_raw_parts_mut(char_ptr, len) as *mut Self
    }

    unsafe fn borrow_unchecked<'dat>(ptr: *const u8) -> &'dat Self {
        let char_ptr: *const ffi::c_char = ptr.cast();
        unsafe { ffi::CStr::from_ptr(char_ptr) }
    }
}
