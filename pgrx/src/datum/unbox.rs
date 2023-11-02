use super::uuid::Uuid;
use super::Datum;
use crate::prelude::*;
use crate::varlena::{text_to_rust_str_unchecked, varlena_to_byte_slice};
use alloc::ffi::CString;
use core::ffi::CStr;
/// Directly convert a Datum into this type
pub unsafe trait UnboxDatum {
    type As<'dat>;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat>;
}

macro_rules! unbox_int {
    ($($int_ty:ty),*) => {
        $(
            unsafe impl UnboxDatum for $int_ty {
                type As<'dat> = $int_ty;
                unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
                    datum.0.value() as $int_ty
                }
            }
        )*
    }
}

unbox_int! {
    i8, i16, i32, i64
}

unsafe impl UnboxDatum for bool {
    type As<'dat> = bool;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        datum.0.value() != 0
    }
}

unsafe impl UnboxDatum for f32 {
    type As<'dat> = f32;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        f32::from_bits(datum.0.value() as u32)
    }
}

unsafe impl UnboxDatum for f64 {
    type As<'dat> = f64;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        f64::from_bits(datum.0.value() as u64)
    }
}

unsafe impl UnboxDatum for str {
    type As<'dat> = &'dat str;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        unsafe { text_to_rust_str_unchecked(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for &str {
    type As<'dat> = &'dat str;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        unsafe { text_to_rust_str_unchecked(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for CStr {
    type As<'dat> = &'dat CStr;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        unsafe { CStr::from_ptr(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for &CStr {
    type As<'dat> = &'dat CStr;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        unsafe { CStr::from_ptr(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for [u8] {
    type As<'dat> = &'dat [u8];
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        unsafe { varlena_to_byte_slice(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for &[u8] {
    type As<'dat> = &'dat [u8];
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        unsafe { varlena_to_byte_slice(datum.0.cast_mut_ptr()) }
    }
}

unsafe impl UnboxDatum for pg_sys::Oid {
    type As<'dat> = pg_sys::Oid;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        unsafe { pg_sys::Oid::from_u32_unchecked(datum.0.value() as u32) }
    }
}

unsafe impl UnboxDatum for String {
    type As<'dat> = String;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        unsafe { str::unbox(datum) }.to_owned()
    }
}

unsafe impl UnboxDatum for CString {
    type As<'dat> = CString;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        unsafe { CStr::unbox(datum) }.to_owned()
    }
}

unsafe impl UnboxDatum for pg_sys::Datum {
    type As<'dat> = pg_sys::Datum;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        datum.0
    }
}

unsafe impl UnboxDatum for Uuid {
    type As<'dat> = Uuid;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        Uuid::from_bytes(datum.0.cast_mut_ptr::<[u8; 16]>().read())
    }
}

unsafe impl UnboxDatum for Date {
    type As<'dat> = Date;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        Date::try_from(i32::unbox(datum)).unwrap_unchecked()
    }
}

unsafe impl UnboxDatum for Time {
    type As<'dat> = Time;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        Time::try_from(i64::unbox(datum)).unwrap_unchecked()
    }
}

unsafe impl UnboxDatum for Timestamp {
    type As<'dat> = Timestamp;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        Timestamp::try_from(i64::unbox(datum)).unwrap_unchecked()
    }
}

unsafe impl UnboxDatum for TimestampWithTimeZone {
    type As<'dat> = TimestampWithTimeZone;
    unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
        TimestampWithTimeZone::try_from(i64::unbox(datum)).unwrap_unchecked()
    }
}

macro_rules! unbox_with_fromdatum {
    ($($from_ty:ty,)*) => {
        $(
            unsafe impl UnboxDatum for $from_ty {
                type As<'dat> = $from_ty;
                unsafe fn unbox<'dat>(datum: Datum<'dat>) -> Self::As<'dat> {
                    Self::from_datum(datum.0, false).unwrap()
                }
            }
        )*
    }
}

unbox_with_fromdatum! {
    TimeWithTimeZone, AnyNumeric, char, pg_sys::Point, Interval, pg_sys::BOX,
}

unsafe impl UnboxDatum for PgHeapTuple<'_, crate::AllocatedByRust> {
    type As<'dat> = PgHeapTuple<'dat, AllocatedByRust>;
    unsafe fn unbox<'dat>(d: Datum<'dat>) -> Self::As<'dat> {
        PgHeapTuple::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<T: FromDatum + UnboxDatum> UnboxDatum for Array<'_, T> {
    type As<'dat> = Array<'dat, T>;
    unsafe fn unbox<'dat>(d: Datum<'dat>) -> Self::As<'dat> {
        Array::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<const P: u32, const S: u32> UnboxDatum for Numeric<P, S> {
    type As<'dat> = Numeric<P, S>;
    unsafe fn unbox<'dat>(d: Datum<'dat>) -> Self::As<'dat> {
        Numeric::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<T> UnboxDatum for PgBox<T, AllocatedByPostgres> {
    type As<'dat> = PgBox<T>;
    unsafe fn unbox<'dat>(d: Datum<'dat>) -> Self::As<'dat> {
        PgBox::from_datum(d.0, false).unwrap()
    }
}

unsafe impl<'de, T> UnboxDatum for T
where
    T: PostgresType + serde::Deserialize<'de>,
{
    type As<'dat> = T;
    unsafe fn unbox<'dat>(d: Datum<'dat>) -> Self::As<'dat> {
        T::from_datum(d.0, false).unwrap()
    }
}
