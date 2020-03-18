//! for converting primitive types into Datums
//!
//! Primitive types can never be null, so we do a direct
//! cast of the primitive type to pg_sys::Datum

use crate::{direct_function_call, pg_sys, rust_str_to_text_p, PgBox, PgOid};

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
    fn type_oid() -> pg_sys::Oid;
    fn array_type_oid() -> pg_sys::Oid {
        unsafe { pg_sys::get_array_type(Self::type_oid()) }
    }
}

/// for supporting NULL as the None value of an Option<T>
impl<T> IntoDatum<Option<T>> for Option<T>
where
    T: IntoDatum<T>,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        match self {
            Some(t) => t.into_datum(),
            None => None,
        }
    }

    fn type_oid() -> u32 {
        T::type_oid()
    }
}

/// for bool
impl IntoDatum<bool> for bool {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some((if self { 1 } else { 0 }) as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::BOOLOID
    }
}

/// for smallint
impl IntoDatum<i16> for i16 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::INT2OID
    }
}

/// for integer
impl IntoDatum<i32> for i32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::INT4OID
    }
}

/// for oid
impl IntoDatum<u32> for u32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::OIDOID
    }
}

/// for bigint
impl IntoDatum<i64> for i64 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::INT8OID
    }
}

/// for real
impl IntoDatum<f32> for f32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.to_bits() as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::FLOAT4OID
    }
}

/// for double precision
impl IntoDatum<f64> for f64 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.to_bits() as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::FLOAT8OID
    }
}

impl IntoDatum<PgOid> for PgOid {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        match self {
            PgOid::InvalidOid => None,
            oid => Some(oid.value() as pg_sys::Datum),
        }
    }

    fn type_oid() -> u32 {
        pg_sys::OIDOID
    }
}

/// for text, varchar
impl<'a> IntoDatum<&'a str> for &'a str {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let varlena = rust_str_to_text_p(&self);
        if varlena.is_null() {
            None
        } else {
            Some(varlena.into_pg() as pg_sys::Datum)
        }
    }

    fn type_oid() -> u32 {
        pg_sys::TEXTOID
    }
}

impl IntoDatum<String> for String {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        self.as_str().into_datum()
    }

    fn type_oid() -> u32 {
        pg_sys::TEXTOID
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

    fn type_oid() -> u32 {
        pg_sys::CSTRINGOID
    }
}

/// for NULL -- always converts to `None`
impl IntoDatum<()> for () {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        None
    }

    fn type_oid() -> u32 {
        pg_sys::BOOLOID
    }
}

/// for user types
impl<T> IntoDatum<PgBox<T>> for PgBox<T> {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        if self.is_null() {
            None
        } else {
            Some(self.into_pg() as pg_sys::Datum)
        }
    }

    fn type_oid() -> u32 {
        let type_name = std::any::type_name::<T>();
        unsafe {
            direct_function_call::<pg_sys::Oid>(pg_sys::regtypein, vec![type_name.into_datum()])
                .expect("unable to lookup type oid")
        }
    }
}
