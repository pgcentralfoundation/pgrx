use super::uuid::Uuid;
use super::Datum;
use crate::prelude::*;
use crate::varlena::{text_to_rust_str_unchecked, varlena_to_byte_slice};
use alloc::ffi::CString;
use core::ffi::CStr;

/// Directly convert a Datum into this type
///
/// Previously, pgrx used FromDatum exclusively, which captures conversion into T, but
/// leaves ambiguous a number of possibilities and allows large swathes of behavior to
/// be folded under a single trait. This provided certain beneficial ergonomics at first,
/// but eventually behavior was incorrectly folded under FromDatum, which provided the
/// illusion of soundness.
///
/// UnboxDatum should only be implemented for a type that CAN be directly converted from a Datum,
/// and it doesn't say whether it can be used directly or if it should be detoasted via MemCx.
/// It's currently just a possibly-temporary shim to make pgrx work.
///
/// # Safety
/// This trait is used to bound the lifetime of certain types: thus the associated type must be
/// this type but "infected" by the Datum's lifetime. By implementing this, you verify that you
/// are implementing this in the way that satisfies that lifetime constraint. There isn't really
/// a good way to constrain lifetimes correctly without forcing from-Datum types to go through a
/// wrapper type bound by the lifetime of the Datum. And what would you use as the bound, hmm?
// pub unsafe trait UnboxDatum {
//     // TODO: Currently, this doesn't actually get used to identify all the cases where the Datum
//     // is actually a pointer type. However, it has been noted that Postgres does yield nullptr
//     // on occasion, even when they say something is not supposed to be nullptr. As it is common
//     // for Postgres to init [Datum<'_>] with palloc0, it is reasonable to assume nullptr is a risk,
//     // even if `is_null == false`.
//     //
//     // Wait, what are you about, Jubilee? In some cases, the chance of nullness doesn't exist!
//     // This is because we are materializing the datums from e.g. pointers to an Array, which
//     // requires you to have had a valid base pointer into an ArrayType to start!
//     // That's why you started using this goofy GAT scheme in the first place!
//     type As<'dat>;
//     /// Convert from `Datum<'dat>` to `T<'dat>`
//     ///
//     /// # Safety
//     /// Due to the absence of an `is_null` parameter, this does not validate "SQL nullness" of
//     /// the Datum in question. The intention is that this core fn eschews an additional branch.
//     /// Just... don't use it if it might be null?
//     ///
//     /// This also should not be used as the primary conversion mechanism if it requires a MemCx,
//     /// as this is intended primarily to be used in cases where the datum is guaranteed to be
//     /// detoasted already.
//     unsafe fn unbox(datum: Datum<'dat>) -> Self;
// }

pub unsafe trait UnboxDatumNoGat<'dat> {
    /// Convert from `Datum<'dat>` to `T<'dat>`
    ///
    /// # Safety
    /// Due to the absence of an `is_null` parameter, this does not validate "SQL nullness" of
    /// the Datum in question. The intention is that this core fn eschews an additional branch.
    /// Just... don't use it if it might be null?
    ///
    /// This also should not be used as the primary conversion mechanism if it requires a MemCx,
    /// as this is intended primarily to be used in cases where the datum is guaranteed to be
    /// detoasted already.
    unsafe fn unbox(datum: Datum<'dat>) -> Self;
}

macro_rules! unbox_int {
    ($($int_ty:ty),*) => {
        $(
            unsafe impl<'dat> UnboxDatumNoGat<'dat> for $int_ty {
                unsafe fn unbox(datum: Datum<'dat>) -> Self {
                    datum.0.value() as $int_ty
                }
            }
        )*
    }
}

unbox_int! {
    i8, i16, i32, i64
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for bool {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        datum.0.value() != 0
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for f32 {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        f32::from_bits(datum.0.value() as u32)
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for f64 {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        f64::from_bits(datum.0.value() as u64)
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for &'dat str {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        unsafe { text_to_rust_str_unchecked(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for &'dat CStr {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        unsafe { CStr::from_ptr(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for &'dat [u8] {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        unsafe { varlena_to_byte_slice(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for pg_sys::Oid {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        pg_sys::Oid::from(datum.0.value() as u32)
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for String {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        unsafe { <&'dat str>::unbox(datum) }.to_owned()
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for CString {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        unsafe { <&'dat CStr>::unbox(datum) }.to_owned()
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for pg_sys::Datum {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        datum.0
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for Uuid {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        Uuid::from_bytes(datum.0.cast_mut_ptr::<[u8; 16]>().read())
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for Date {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        Date::try_from(i32::unbox(datum)).unwrap_unchecked()
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for Time {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        Time::try_from(i64::unbox(datum)).unwrap_unchecked()
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for Timestamp {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        Timestamp::try_from(i64::unbox(datum)).unwrap_unchecked()
    }
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for TimestampWithTimeZone {
    #[inline]
    unsafe fn unbox(datum: Datum<'dat>) -> Self {
        TimestampWithTimeZone::try_from(i64::unbox(datum)).unwrap_unchecked()
    }
}

macro_rules! unbox_with_fromdatum {
    ($($from_ty:ty,)*) => {
        $(
            unsafe impl<'dat> UnboxDatumNoGat<'dat> for $from_ty {
                unsafe fn unbox(datum: Datum<'dat>) -> $from_ty {
                    Self::from_datum(datum.0, false).unwrap()
                }
            }
        )*
    }
}

unbox_with_fromdatum! {
    TimeWithTimeZone, AnyNumeric, char, pg_sys::Point, Interval, pg_sys::BOX,
}

unsafe impl<'dat> UnboxDatumNoGat<'dat> for PgHeapTuple<'_, crate::AllocatedByRust> {
    #[inline]
    unsafe fn unbox(d: Datum<'dat>) -> Self {
        PgHeapTuple::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<'dat, T: FromDatum + UnboxDatumNoGat<'dat>> UnboxDatumNoGat<'dat> for Array<'dat, T> {
    unsafe fn unbox(d: Datum<'dat>) -> Array<'dat, T> {
        Array::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<'dat, T: FromDatum + UnboxDatumNoGat<'dat>> UnboxDatumNoGat<'dat>
    for VariadicArray<'dat, T>
{
    unsafe fn unbox(d: Datum<'dat>) -> VariadicArray<'dat, T> {
        VariadicArray::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<'dat, const P: u32, const S: u32> UnboxDatumNoGat<'dat> for Numeric<P, S> {
    #[inline]
    unsafe fn unbox(d: Datum<'dat>) -> Self {
        Numeric::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<'dat, T> UnboxDatumNoGat<'dat> for PgBox<T, AllocatedByPostgres> {
    #[inline]
    unsafe fn unbox(d: Datum<'dat>) -> Self {
        PgBox::from_datum(d.0, false).unwrap()
    }
}
