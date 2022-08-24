use crate::datum::{Array, FromDatum};
use crate::pg_sys;
use core::ptr::NonNull;
use pgx_pg_sys::*;

extern "C" {
    pub fn pgx_ARR_NELEMS(arrayType: *mut ArrayType) -> i32;
    pub fn pgx_ARR_NULLBITMAP(arrayType: *mut ArrayType) -> *mut bits8;
}

#[inline]
pub unsafe fn get_arr_data_ptr<T>(arr: *mut ArrayType) -> *mut T {
    extern "C" {
        pub fn pgx_ARR_DATA_PTR(arrayType: *mut ArrayType) -> *mut u8;
    }
    pgx_ARR_DATA_PTR(arr) as *mut T
}

#[inline]
pub fn get_arr_nelems(arr: *mut ArrayType) -> i32 {
    unsafe { pgx_ARR_NELEMS(arr) }
}

#[inline]
pub fn get_arr_nullbitmap<'a>(arr: *mut ArrayType) -> &'a [bits8] {
    unsafe {
        let len = (pgx_ARR_NELEMS(arr) + 7) / 8;
        std::slice::from_raw_parts(pgx_ARR_NULLBITMAP(arr), len as usize)
    }
}

#[inline]
pub fn get_arr_nullbitmap_mut<'a>(arr: *mut ArrayType) -> &'a mut [u8] {
    unsafe {
        let len = (pgx_ARR_NELEMS(arr) + 7) / 8;
        std::slice::from_raw_parts_mut(pgx_ARR_NULLBITMAP(arr), len as usize)
    }
}

#[inline]
pub fn get_arr_hasnull(arr: *mut ArrayType) -> bool {
    // copied from array.h
    unsafe { (*arr).dataoffset != 0 }
}

#[inline]
pub fn get_arr_dims<'a>(arr: *mut ArrayType) -> &'a [i32] {
    extern "C" {
        pub fn pgx_ARR_DIMS(arrayType: *mut ArrayType) -> *mut i32;
    }
    unsafe {
        let len = (*arr).ndim;
        std::slice::from_raw_parts(pgx_ARR_DIMS(arr), len as usize)
    }
}

/// Handle describing a bare, "untyped" pointer to an array,
/// offering safe accessors to the various fields of one.
#[repr(transparent)]
#[derive(Debug)]
pub struct RawArray {
    at: NonNull<ArrayType>,
}

#[deny(unsafe_op_in_unsafe_fn)]
impl RawArray {
    // General implementation notes:
    // RawArray is not Copy or Clone, making it harder to misuse versus *mut ArrayType.
    // But this also offers safe accessors to the fields, like &ArrayType,
    // so it requires validity assertions in order to be constructed.

    /// Returns a handle to the raw array header.
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that all of the following is true:
    /// * The pointer must be properly aligned.
    /// * It must be "dereferenceable" in the sense defined in [the std documentation].
    /// * The pointer must point to an initialized instance of `ArrayType`.
    /// * You aren't going to alias the data like mad.
    ///
    /// It should be noted as RawArray is not inherently lifetime-bound, it can be racy and unsafe!
    ///
    /// [the std documentation]: core::ptr#safety
    pub unsafe fn from_raw(at: NonNull<ArrayType>) -> RawArray {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a mutable reference, as we're going to treat this like one.
        RawArray { at }
    }

    /// # Safety
    ///
    /// Array must have been constructed from an ArrayType pointer.
    pub unsafe fn from_array<T: FromDatum>(arr: Array<T>) -> Option<RawArray> {
        let array_type = arr.into_array_type() as *mut _;
        Some(RawArray {
            at: NonNull::new(array_type)?,
        })
    }

    /// Returns the inner raw pointer to the ArrayType.
    pub fn into_raw(self) -> NonNull<ArrayType> {
        self.at
    }

    /// Get the number of dimensions.
    pub fn ndims(&self) -> libc::c_int {
        // SAFETY: Validity asserted on construction.
        unsafe {
            (*self.at.as_ptr()).ndim
            // FIXME: While this is a c_int, the max ndim is normally 6
            // While the value can be set higher, it is... unlikely
            // that it is going to actually challenge even 16-bit pointer widths.
            // It would be preferable to return a usize instead,
            // however, PGX has trouble with that, unfortunately.
            as _
        }
    }

    pub fn oid(&self) -> pg_sys::Oid {
        // SAFETY: Validity asserted on construction.
        unsafe { (*self.at.as_ptr()).elemtype }
    }

    /// Gets the offset to the ArrayType's data.
    /// Note that this should not be "taken literally".
    pub fn data_offset(&self) -> libc::c_int {
        // SAFETY: Validity asserted on construction.
        unsafe { (*self.at.as_ptr()).dataoffset }
    }
}
