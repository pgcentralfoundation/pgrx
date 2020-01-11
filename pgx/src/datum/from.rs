//! for converting a pg_sys::Datum and a corresponding "is_null" bool into a typed Option

use crate::{pg_sys, text_to_rust_str_unchecked, PgBox};
use std::ffi::CStr;

/// Convert a `(pg_sys::Datum, is_null:bool)` tuple into a Rust type
///
/// Default implementations are provided for the common Rust types.
///
/// If implementing this, also implement `IntoDatum<T>` for the reverse
/// conversion.
pub trait FromDatum<T> {
    fn from_datum(datum: pg_sys::Datum, is_null: bool, typoid: pg_sys::Oid) -> Option<T>;
}

/// for bool
impl FromDatum<bool> for bool {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<bool> {
        if is_null {
            None
        } else {
            Some(datum != 0)
        }
    }
}

/// for char
impl FromDatum<i8> for i8 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i8> {
        if is_null {
            None
        } else {
            Some(datum as i8)
        }
    }
}

/// for smallint
impl FromDatum<i16> for i16 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i16> {
        if is_null {
            None
        } else {
            Some(datum as i16)
        }
    }
}

/// for integer
impl FromDatum<i32> for i32 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i32> {
        if is_null {
            None
        } else {
            Some(datum as i32)
        }
    }
}

/// for bigint
impl FromDatum<i64> for i64 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i64> {
        if is_null {
            None
        } else {
            Some(datum as i64)
        }
    }
}

/// for real
impl FromDatum<f32> for f32 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<f32> {
        if is_null {
            None
        } else {
            Some(f32::from_bits(datum as u32))
        }
    }
}

/// for double precision
impl FromDatum<f64> for f64 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<f64> {
        if is_null {
            None
        } else {
            Some(f64::from_bits(datum as u64))
        }
    }
}

/// for text, varchar
impl<'a> FromDatum<&'a str> for &'a str {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<&'a str> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a varlena Datum was flagged as non-null but the datum is zero");
        } else {
            let varlena = unsafe { pg_sys::pg_detoast_datum(datum as *mut pg_sys::varlena) };
            Some(unsafe { text_to_rust_str_unchecked(varlena) })
        }
    }
}
impl<'a> FromDatum<&'a str> for str {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<&'a str> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a varlena Datum was flagged as non-null but the datum is zero");
        } else {
            let varlena = unsafe { pg_sys::pg_detoast_datum(datum as *mut pg_sys::varlena) };
            Some(unsafe { text_to_rust_str_unchecked(varlena) })
        }
    }
}

impl FromDatum<String> for String {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<String> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a varlena Datum was flagged as non-null but the datum is zero");
        } else {
            let varlena = unsafe { pg_sys::pg_detoast_datum(datum as *mut pg_sys::varlena) };
            Some(unsafe { text_to_rust_str_unchecked(varlena) }.to_string())
        }
    }
}

/// for cstring
impl<'a> FromDatum<&'a std::ffi::CStr> for &'a std::ffi::CStr {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<&'a CStr> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a cstring Datum was flagged as non-null but the datum is zero");
        } else {
            Some(unsafe { std::ffi::CStr::from_ptr(datum as *const std::os::raw::c_char) })
        }
    }
}

/// for NULL -- always converts to a `None`, even if the is_null argument is false
impl FromDatum<()> for () {
    #[inline]
    fn from_datum(_datum: pg_sys::Datum, _is_null: bool, _: pg_sys::Oid) -> Option<()> {
        None
    }
}

/// for user types
impl<T> FromDatum<PgBox<T>> for PgBox<T> {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<PgBox<T>> {
        if is_null {
            None
        } else if datum == 0 {
            panic!(
                "user type {} Datum was flagged as non-null but the datum is zero",
                std::any::type_name::<T>()
            );
        } else {
            Some(PgBox::<T>::from_pg(datum as *mut T))
        }
    }
}
