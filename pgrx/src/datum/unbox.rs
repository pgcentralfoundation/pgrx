use super::Datum;
use crate::varlena::text_to_rust_str_unchecked;
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
