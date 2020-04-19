//! for converting a pg_sys::Datum and a corresponding "is_null" bool into a typed Option

use crate::{
    pg_sys, text_to_rust_str_unchecked, vardata_any, varsize_any_exhdr, void_mut_ptr, PgBox,
};
use std::ffi::CStr;

/// Convert a `(pg_sys::Datum, is_null:bool, type_oid:pg_sys::Oid)` tuple into a Rust type
///
/// Default implementations are provided for the common Rust types.
///
/// If implementing this, also implement `IntoDatum` for the reverse
/// conversion.
pub trait FromDatum {
    /// ## Safety
    ///
    /// This method is inherently unsafe as the `datum` argument can represent an arbitrary
    /// memory address in the case of pass-by-reference Datums.  Referencing that memory address
    /// can cause Postgres to crash if it's invalid.
    ///
    /// If the `(datum, is_null)` tuple comes from Postgres, it's generally okay to consider this
    /// a safe call (ie, wrap it in `unsafe {}`) and move on with life.
    ///
    /// If, however, you're providing an arbitrary datum value, it needs to be considered unsafe
    /// and that unsafeness should be propagated through your API.
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, typoid: pg_sys::Oid) -> Option<Self>
    where
        Self: Sized;
}

/// for pg_sys::Datum
impl FromDatum for pg_sys::Datum {
    #[inline]
    unsafe fn from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<pg_sys::Datum> {
        if is_null {
            None
        } else {
            Some(datum)
        }
    }
}

/// for bool
impl FromDatum for bool {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<bool> {
        if is_null {
            None
        } else {
            Some(datum != 0)
        }
    }
}

/// for char
impl FromDatum for i8 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i8> {
        if is_null {
            None
        } else {
            Some(datum as i8)
        }
    }
}

/// for smallint
impl FromDatum for i16 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i16> {
        if is_null {
            None
        } else {
            Some(datum as i16)
        }
    }
}

/// for integer
impl FromDatum for i32 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i32> {
        if is_null {
            None
        } else {
            Some(datum as i32)
        }
    }
}

/// for oid
impl FromDatum for u32 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<u32> {
        if is_null {
            None
        } else {
            Some(datum as u32)
        }
    }
}

/// for bigint
impl FromDatum for i64 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<i64> {
        if is_null {
            None
        } else {
            Some(datum as i64)
        }
    }
}

/// for real
impl FromDatum for f32 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<f32> {
        if is_null {
            None
        } else {
            Some(f32::from_bits(datum as u32))
        }
    }
}

/// for double precision
impl FromDatum for f64 {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<f64> {
        if is_null {
            None
        } else {
            Some(f64::from_bits(datum as u64))
        }
    }
}

/// for text, varchar
impl<'a> FromDatum for &'a str {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<&'a str> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a varlena Datum was flagged as non-null but the datum is zero");
        } else {
            let varlena = pg_sys::pg_detoast_datum(datum as *mut pg_sys::varlena);
            Some(text_to_rust_str_unchecked(varlena))
        }
    }
}

/// for text, varchar, or any `pg_sys::varlena`-based type
///
/// This returns a **copy**, allocated and managed by Rust, of the underlying `varlena` Datum
impl FromDatum for String {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<String> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a varlena Datum was flagged as non-null but the datum is zero");
        } else {
            let varlena = datum as *mut pg_sys::varlena;
            let detoasted = pg_sys::pg_detoast_datum(varlena);
            let len = varsize_any_exhdr(detoasted);
            let data = vardata_any(detoasted);

            let result =
                std::str::from_utf8_unchecked(std::slice::from_raw_parts(data as *mut u8, len))
                    .to_owned();

            if detoasted != varlena {
                pg_sys::pfree(detoasted as void_mut_ptr);
            }

            Some(result)
        }
    }
}

/// for cstring
impl<'a> FromDatum for &'a std::ffi::CStr {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<&'a CStr> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a cstring Datum was flagged as non-null but the datum is zero");
        } else {
            Some(std::ffi::CStr::from_ptr(
                datum as *const std::os::raw::c_char,
            ))
        }
    }
}

/// for NULL -- always converts to a `None`, even if the is_null argument is false
impl FromDatum for () {
    #[inline]
    unsafe fn from_datum(_datum: pg_sys::Datum, _is_null: bool, _: pg_sys::Oid) -> Option<()> {
        None
    }
}

/// for user types
impl<T> FromDatum for PgBox<T> {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<PgBox<T>> {
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
