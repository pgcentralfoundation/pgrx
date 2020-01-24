//! for converting primitive types into Datums
//!
//! Primitive types can never be null, so we do a direct
//! cast of the primitive type to pg_sys::Datum

use crate::{pg_sys, rust_str_to_text_p, PgBox};

/// Convert a Rust type into a `pg_sys::Datum`.
///
/// Default implementations are provided for the common Rust types.
///
/// If implementing this, also implement `FromDatum<T>` for the reverse
/// conversion.
///
/// Note that any conversions that need to allocate memory (ie, for a `varlena *` representation
/// of a Rust type, that memory **must** be allocated within a [PgMemoryContext]
pub trait IntoDatum<T> {
    fn into_datum(self) -> Option<pg_sys::Datum>;
}

/// for bool
impl IntoDatum<bool> for bool {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some((if self { 1 } else { 0 }) as pg_sys::Datum)
    }
}

/// for smallint
impl IntoDatum<i16> for i16 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }
}

/// for integer
impl IntoDatum<i32> for i32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }
}

/// for oid
impl IntoDatum<u32> for u32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }
}

/// for bigint
impl IntoDatum<i64> for i64 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }
}

/// for real
impl IntoDatum<f32> for f32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.to_bits() as pg_sys::Datum)
    }
}

/// for double precision
impl IntoDatum<f64> for f64 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.to_bits() as pg_sys::Datum)
    }
}

/// for text, varchar
impl<'a> IntoDatum<&'a str> for &'a str {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(rust_str_to_text_p(&self).convert_to_datum())
    }
}

impl IntoDatum<String> for String {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(rust_str_to_text_p(&self).convert_to_datum())
    }
}

/// for cstring
///
/// ## Safety
///
/// The `&CStr` better be allocated by Postgres
impl<'a> IntoDatum<&'a std::ffi::CStr> for &'a std::ffi::CStr {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.as_ptr() as pg_sys::Datum)
    }
}

/// for NULL -- always converts to `None`
impl IntoDatum<()> for () {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        None
    }
}

/// for user types
impl<T> IntoDatum<PgBox<T>> for PgBox<T> {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        if self.is_null() {
            None
        } else {
            Some(self.convert_to_datum())
        }
    }
}
