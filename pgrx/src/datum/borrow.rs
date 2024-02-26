#![deny(unsafe_op_in_unsafe_fn)]
use super::*;
use core::ffi;

/// Converts `&Datum` to `&T`
///
/// Only for types which can be losslessly referenced from a borrow of a Datum, without unboxing.
/// For instance, you may not borrow `&String` from `&Datum<'_>`: that just doesn't work as it
/// requires a full unboxing to be performed. You may, however, claim `&str` from the datum.
/// Implementers are expected to uphold this obligation.
///
/// Note that this is unaware of "detoasting".
// TODO: Implement the DetoastDatum this is supposed to combine with.
pub unsafe trait BorrowDatum {
    /// Lossless zero-unboxing reference conversion.
    ///
    /// # Safety
    /// * The Datum must not be an SQL Null.
    /// * For pass-by-reference types, this Datum must point to a valid instance of that type.
    unsafe fn borrow_from<'dat>(datum: &'dat Datum<'_>) -> &'dat Self;

    /// Lossless zero-unboxing mutable reference conversion.
    ///
    /// # Safety
    /// * The Datum must not be an SQL Null.
    /// * For pass-by-reference types, this Datum must point to a valid instance of that type.
    ///
    /// **Design Note:** For pass-by-value types, if you yield two successive `&mut Datum<'_>`,
    /// and the underlying source is e.g. an ArrayType or one of the many Tuples of Postgres,
    /// then merely yielding two `&mut Datum<'_>`s in a row is unsound. This is because the bytes
    /// of one Datum can overlap with the next. Uh... whoops?
    unsafe fn borrow_mut_from<'dat>(datum: &'dat mut Datum<'_>) -> &'dat mut Self;
}

macro_rules! borrow_by_value {
    ($($value_ty:ty),*) => {
        $(
            unsafe impl BorrowDatum for $value_ty {
                /// Directly cast `&Datum as &Self`
                unsafe fn borrow_from<'dat>(datum: &'dat Datum<'_>) -> &'dat Self {
                    unsafe { &*(datum as *const Datum<'_> as *const Self) }
                }
                /// Directly cast `&mut Datum as &mut Self`
                unsafe fn borrow_mut_from<'dat>(datum: &'dat mut Datum<'_>) -> &'dat mut Self {
                    unsafe { &mut *(datum as *mut Datum<'_> as *mut Self) }
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
    /// Treat `&Datum` as `&*const ffi::c_char` and then deref-reborrow it,
    /// with the same constraints as [`ffi::CStr::from_ptr`].
    unsafe fn borrow_from<'dat>(datum: &'dat Datum<'_>) -> &'dat Self {
        unsafe { ffi::CStr::from_ptr(*datum.0.cast_mut_ptr()) }
    }

    /// Treat `&mut Datum` as `&mut *mut ffi::c_char` and then deref-reborrow it,
    /// with the same constraints as [`ffi::CStr::from_ptr`].
    unsafe fn borrow_mut_from<'dat>(datum: &'dat mut Datum<'_>) -> &'dat mut Self {
        let char_ptr: *mut ffi::c_char = datum.0.cast_mut_ptr();
        let len = unsafe { ffi::CStr::from_ptr(char_ptr).to_bytes().len() };
        let slice_ptr = core::ptr::slice_from_raw_parts_mut(char_ptr, len + 1);
        unsafe { &mut *(slice_ptr as *mut ffi::CStr) }
    }
}
