/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! for converting primitive types into Datums
//!
//! Primitive types can never be null, so we do a direct
//! cast of the primitive type to pg_sys::Datum

use crate::{pg_sys, rust_regtypein, PgBox, PgOid, WhoAllocated};
use pgx_pg_sys::{Datum, Oid};

/// Convert a Rust type into a `pg_sys::Datum`.
///
/// Default implementations are provided for the common Rust types.
///
/// If implementing this, also implement `FromDatum` for the reverse
/// conversion.
///
/// Note that any conversions that need to allocate memory (ie, for a `varlena *` representation
/// of a Rust type, that memory **must** be allocated within a [`PgMemoryContexts`](crate::PgMemoryContexts).
pub trait IntoDatum {
    fn into_datum(self) -> Option<pg_sys::Datum>;
    fn type_oid() -> pg_sys::Oid;

    fn composite_type_oid(&self) -> Option<Oid> {
        None
    }
    fn array_type_oid() -> pg_sys::Oid {
        unsafe { pg_sys::get_array_type(Self::type_oid()) }
    }

    /// Is a Datum of this type compatible with another Postgres type?
    ///
    /// An example of this are the Postgres `text` and `varchar` types, which are both
    /// technically compatible from a Rust type perspective.  They're both represented in Rust as
    /// `String` (or `&str`), but the underlying Postgres types are different.
    ///
    /// If implementing this yourself, you likely want to follow a pattern like this:
    ///
    /// ```rust,no_run
    /// # use pgx::*;
    /// # #[repr(transparent)]
    /// # struct FooType(String);
    /// # impl pgx::IntoDatum for FooType {      
    ///     fn is_compatible_with(other: pg_sys::Oid) -> bool {
    ///         // first, if our type is the other type, then we're compatible
    ///         Self::type_oid() == other
    ///
    ///         // and here's the other type we're compatible with
    ///         || other == pg_sys::VARCHAROID
    ///     }
    ///
    /// #    fn into_datum(self) -> Option<pg_sys::Datum> {
    /// #        todo!()
    /// #    }
    /// #
    /// #    fn type_oid() -> pg_sys::Oid {
    /// #        pg_sys::TEXTOID
    /// #    }
    /// # }
    /// ```
    #[inline]
    fn is_compatible_with(other: pg_sys::Oid) -> bool {
        Self::type_oid() == other
    }

    /// Is a Datum of this type pass by value or pass by reference?
    ///
    /// We provide a hardcoded list of known Postgres types that are pass by value,
    /// but you are free to implement this yourself for custom types.
    #[inline]
    fn is_pass_by_value() -> bool
    where
        Self: 'static,
    {
        let my_type = std::any::TypeId::of::<Self>();
        my_type == std::any::TypeId::of::<i8>()
            || my_type == std::any::TypeId::of::<i16>()
            || my_type == std::any::TypeId::of::<i32>()
            || my_type == std::any::TypeId::of::<i64>()
            || my_type == std::any::TypeId::of::<u8>()
            || my_type == std::any::TypeId::of::<u16>()
            || my_type == std::any::TypeId::of::<u32>()
            || my_type == std::any::TypeId::of::<u64>()
            || my_type == std::any::TypeId::of::<f32>()
            || my_type == std::any::TypeId::of::<f64>()
            || my_type == std::any::TypeId::of::<bool>()
            || my_type == std::any::TypeId::of::<()>()
            || my_type == std::any::TypeId::of::<crate::Time>()
            || my_type == std::any::TypeId::of::<crate::TimeWithTimeZone>()
            || my_type == std::any::TypeId::of::<crate::Timestamp>()
            || my_type == std::any::TypeId::of::<crate::TimestampWithTimeZone>()
            || my_type == std::any::TypeId::of::<crate::Date>()
            || my_type == std::any::TypeId::of::<PgOid>()
            || my_type == std::any::TypeId::of::<pg_sys::Datum>()
            || my_type == std::any::TypeId::of::<Option<pg_sys::Datum>>()
    }
}

/// for supporting NULL as the None value of an Option<T>
impl<T> IntoDatum for Option<T>
where
    T: IntoDatum,
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
impl IntoDatum for bool {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(if self { 1 } else { 0 }))
    }

    fn type_oid() -> u32 {
        pg_sys::BOOLOID
    }
}

/// for "char"
impl IntoDatum for i8 {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self))
    }

    fn type_oid() -> u32 {
        pg_sys::CHAROID
    }
}

/// for smallint
impl IntoDatum for i16 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self))
    }

    fn type_oid() -> u32 {
        pg_sys::INT2OID
    }
}

/// for integer
impl IntoDatum for i32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self))
    }

    fn type_oid() -> u32 {
        pg_sys::INT4OID
    }
}

/// for oid
impl IntoDatum for u32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self))
    }

    fn type_oid() -> u32 {
        pg_sys::OIDOID
    }
}

/// for bigint
impl IntoDatum for i64 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self))
    }

    fn type_oid() -> u32 {
        pg_sys::INT8OID
    }
}

/// for real
impl IntoDatum for f32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.to_bits().into())
    }

    fn type_oid() -> u32 {
        pg_sys::FLOAT4OID
    }
}

/// for double precision
impl IntoDatum for f64 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.to_bits().into())
    }

    fn type_oid() -> u32 {
        pg_sys::FLOAT8OID
    }
}

impl IntoDatum for PgOid {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        match self {
            PgOid::InvalidOid => None,
            oid => Some(oid.value().into()),
        }
    }

    fn type_oid() -> u32 {
        pg_sys::OIDOID
    }
}

/// for text, varchar
impl<'a> IntoDatum for &'a str {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        self.as_bytes().into_datum()
    }

    fn type_oid() -> u32 {
        pg_sys::TEXTOID
    }

    #[inline]
    fn is_compatible_with(other: Oid) -> bool {
        Self::type_oid() == other || other == pg_sys::VARCHAROID
    }
}

impl IntoDatum for String {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        self.as_str().into_datum()
    }

    fn type_oid() -> u32 {
        pg_sys::TEXTOID
    }

    #[inline]
    fn is_compatible_with(other: Oid) -> bool {
        Self::type_oid() == other || other == pg_sys::VARCHAROID
    }
}

impl IntoDatum for &String {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        self.as_str().into_datum()
    }

    fn type_oid() -> u32 {
        pg_sys::TEXTOID
    }

    #[inline]
    fn is_compatible_with(other: Oid) -> bool {
        Self::type_oid() == other || other == pg_sys::VARCHAROID
    }
}

impl IntoDatum for char {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        self.to_string().into_datum()
    }

    fn type_oid() -> u32 {
        pg_sys::VARCHAROID
    }

    #[inline]
    fn is_compatible_with(other: Oid) -> bool {
        Self::type_oid() == other || other == pg_sys::VARCHAROID
    }
}

/// for cstring
///
/// ## Safety
///
/// The `&CStr` better be allocated by Postgres
impl<'a> IntoDatum for &'a std::ffi::CStr {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.as_ptr().into())
    }

    fn type_oid() -> u32 {
        pg_sys::CSTRINGOID
    }
}

impl<'a> IntoDatum for &'a crate::cstr_core::CStr {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.as_ptr().into())
    }

    fn type_oid() -> u32 {
        pg_sys::CSTRINGOID
    }
}

/// for bytea
impl<'a> IntoDatum for &'a [u8] {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let len = pg_sys::VARHDRSZ + self.len();
        unsafe {
            // SAFETY:  palloc gives us a valid pointer if if there's not enough memory it'll raise an error
            let varlena = pg_sys::palloc(len) as *mut pg_sys::varlena;

            // SAFETY: `varlena` can properly cast into a `varattrib_4b` and all of what it contains is properly
            // allocated thanks to our call to `palloc` above
            let varattrib_4b = varlena
                .cast::<pg_sys::varattrib_4b>()
                .as_mut()
                .unwrap_unchecked()
                .va_4byte
                .as_mut();
            varattrib_4b.va_header = <usize as TryInto<u32>>::try_into(len)
                .expect("Rust string too large for a Postgres varlena datum")
                << 2u32;

            // SAFETY: src and dest pointers are valid and are exactly `self.len()` bytes long
            std::ptr::copy_nonoverlapping(
                self.as_ptr().cast(),
                varattrib_4b.va_data.as_mut_ptr(),
                self.len(),
            );

            Some(Datum::from(varlena))
        }
    }

    #[inline]
    fn type_oid() -> u32 {
        pg_sys::BYTEAOID
    }
}

impl IntoDatum for Vec<u8> {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        (&self[..]).into_datum()
    }

    #[inline]
    fn type_oid() -> u32 {
        pg_sys::BYTEAOID
    }
}

/// for NULL -- always converts to `None`
impl IntoDatum for () {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        None
    }

    fn type_oid() -> u32 {
        pg_sys::BOOLOID
    }
}

/// for user types
impl<T, AllocatedBy: WhoAllocated> IntoDatum for PgBox<T, AllocatedBy> {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        if self.is_null() {
            None
        } else {
            Some(self.into_pg().into())
        }
    }

    fn type_oid() -> pg_sys::Oid {
        rust_regtypein::<T>()
    }
}

impl IntoDatum for pg_sys::Datum {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self)
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::INT8OID
    }
}
