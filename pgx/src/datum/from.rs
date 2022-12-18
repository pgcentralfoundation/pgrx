/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! for converting a pg_sys::Datum and a corresponding "is_null" bool into a typed Option

use crate::{
    pg_sys, text_to_rust_str_unchecked, varlena_to_byte_slice, AllocatedByPostgres, IntoDatum,
    PgBox, PgMemoryContexts,
};
use pgx_pg_sys::{Datum, Oid};
use std::ffi::CStr;
use std::num::NonZeroUsize;

/// If converting a Datum to a Rust type fails, this is the set of possible reasons why.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum TryFromDatumError {
    #[error("The specified type of the Datum is not compatible with the desired Rust type.")]
    IncompatibleTypes,

    #[error("We were asked to convert a Datum that is NULL (but flagged as \"not null\")")]
    NullDatumPointer,

    #[error("The specified attribute number `{0}` is not present")]
    NoSuchAttributeNumber(NonZeroUsize),

    #[error("The specified attribute name `{0}` is not present")]
    NoSuchAttributeName(String),
}

/// Convert a `(pg_sys::Datum, is_null:bool` pair into a Rust type
///
/// Default implementations are provided for the common Rust types.
///
/// If implementing this, also implement `IntoDatum` for the reverse
/// conversion.
pub trait FromDatum {
    /// Should a type OID be fetched when calling `from_datum`?
    const GET_TYPOID: bool = false;

    /// ## Safety
    ///
    /// This method is inherently unsafe as the `datum` argument can represent an arbitrary
    /// memory address in the case of pass-by-reference Datums.  Referencing that memory address
    /// can cause Postgres to crash if it's invalid.
    ///
    /// If the `(datum, is_null)` pair comes from Postgres, it's generally okay to consider this
    /// a safe call (ie, wrap it in `unsafe {}`) and move on with life.
    ///
    /// If, however, you're providing an arbitrary datum value, it needs to be considered unsafe
    /// and that unsafeness should be propagated through your API.
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<Self>
    where
        Self: Sized,
    {
        FromDatum::from_polymorphic_datum(datum, is_null, pg_sys::InvalidOid)
    }

    /// Like `from_datum` for instantiating polymorphic types
    /// which require preserving the dynamic type metadata.
    ///
    /// ## Safety
    ///
    /// Same caveats as `FromDatum::from_datum(...)`.
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized;

    /// Default implementation switched to the specified memory context and then simply calls
    /// `FromDatum::from_datum(...)` from within that context.
    ///
    /// For certain Datums (such as `&str`), this is likely not enough and this function
    /// should be overridden in the type's trait implementation.
    ///
    /// The intent here is that the returned Rust type, which might be backed by a pass-by-reference
    /// Datum, be copied into the specified memory context, and then the Rust type constructed from
    /// that pointer instead.
    ///
    /// ## Safety
    ///
    /// Same caveats as `FromDatum::from_datum(...)`
    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        memory_context.switch_to(|_| FromDatum::from_polymorphic_datum(datum, is_null, typoid))
    }

    /// `try_from_datum` is a convenience wrapper around `FromDatum::from_datum` that returns a
    /// a `Result` instead of an `Option`.  It's intended to be used in situations where
    /// the caller needs to know whether the type conversion succeeded or failed.
    ///
    /// ## Safety
    ///
    /// Same caveats as `FromDatum::from_datum(...)`
    #[inline]
    unsafe fn try_from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        type_oid: pg_sys::Oid,
    ) -> Result<Self, TryFromDatumError>
    where
        Self: Sized + IntoDatum,
    {
        if !Self::is_compatible_with(type_oid) {
            Err(TryFromDatumError::IncompatibleTypes)
        } else {
            Ok(FromDatum::from_polymorphic_datum(datum, is_null, type_oid)
                .ok_or(TryFromDatumError::NullDatumPointer)?)
        }
    }

    /// A version of `try_from_datum` that switches to the given context to convert from Datum
    #[inline]
    unsafe fn try_from_datum_in_memory_context(
        memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        type_oid: pg_sys::Oid,
    ) -> Result<Self, TryFromDatumError>
    where
        Self: Sized + IntoDatum,
    {
        if !Self::is_compatible_with(type_oid) {
            Err(TryFromDatumError::IncompatibleTypes)
        } else {
            Ok(FromDatum::from_datum_in_memory_context(memory_context, datum, is_null, type_oid)
                .ok_or(TryFromDatumError::NullDatumPointer)?)
        }
    }
}

/// for pg_sys::Datum
impl FromDatum for pg_sys::Datum {
    #[inline]
    unsafe fn from_polymorphic_datum(
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
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<bool> {
        if is_null {
            None
        } else {
            Some(datum.value() != 0)
        }
    }
}

/// for `"char"`
impl FromDatum for i8 {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<i8> {
        if is_null {
            None
        } else {
            Some(datum.value() as _)
        }
    }
}

/// for smallint
impl FromDatum for i16 {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<i16> {
        if is_null {
            None
        } else {
            Some(datum.value() as _)
        }
    }
}

/// for integer
impl FromDatum for i32 {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<i32> {
        if is_null {
            None
        } else {
            Some(datum.value() as _)
        }
    }
}

/// for oid
impl FromDatum for u32 {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<u32> {
        if is_null {
            None
        } else {
            Some(datum.value() as _)
        }
    }
}

/// for bigint
impl FromDatum for i64 {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<i64> {
        if is_null {
            None
        } else {
            Some(datum.value() as _)
        }
    }
}

/// for real
impl FromDatum for f32 {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<f32> {
        if is_null {
            None
        } else {
            Some(f32::from_bits(datum.value() as _))
        }
    }
}

/// for double precision
impl FromDatum for f64 {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<f64> {
        if is_null {
            None
        } else {
            Some(f64::from_bits(datum.value() as _))
        }
    }
}

/// for text, varchar
impl<'a> FromDatum for &'a str {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<&'a str> {
        if is_null || datum.is_null() {
            None
        } else {
            let varlena = pg_sys::pg_detoast_datum_packed(datum.cast_mut_ptr());
            Some(text_to_rust_str_unchecked(varlena))
        }
    }

    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: u32,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null || datum.is_null() {
            None
        } else {
            memory_context.switch_to(|_| {
                // this gets the varlena Datum copied into this memory context
                let detoasted = pg_sys::pg_detoast_datum_copy(datum.cast_mut_ptr());

                // and we need to unpack it (if necessary), which will decompress it too
                let varlena = pg_sys::pg_detoast_datum_packed(detoasted);

                // and now we return it as a &str
                Some(text_to_rust_str_unchecked(varlena))
            })
        }
    }
}

/// for text, varchar, or any `pg_sys::varlena`-based type
///
/// This returns a **copy**, allocated and managed by Rust, of the underlying `varlena` Datum
impl FromDatum for String {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<String> {
        FromDatum::from_polymorphic_datum(datum, is_null, typoid).map(|s: &str| s.to_owned())
    }
}

impl FromDatum for char {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<char> {
        FromDatum::from_polymorphic_datum(datum, is_null, typoid)
            .and_then(|s: &str| s.chars().next())
    }
}

/// for cstring
impl<'a> FromDatum for &'a std::ffi::CStr {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<&'a CStr> {
        if is_null || datum.is_null() {
            None
        } else {
            Some(std::ffi::CStr::from_ptr(datum.cast_mut_ptr()))
        }
    }
}

impl<'a> FromDatum for &'a crate::cstr_core::CStr {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<&'a crate::cstr_core::CStr> {
        if is_null || datum.is_null() {
            None
        } else {
            Some(crate::cstr_core::CStr::from_ptr(datum.cast_mut_ptr()))
        }
    }
}

/// for bytea
impl<'a> FromDatum for &'a [u8] {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: u32,
    ) -> Option<&'a [u8]> {
        if is_null || datum.is_null() {
            None
        } else {
            let varlena = pg_sys::pg_detoast_datum_packed(datum.cast_mut_ptr());
            Some(varlena_to_byte_slice(varlena))
        }
    }

    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: u32,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null || datum.is_null() {
            None
        } else {
            memory_context.switch_to(|_| {
                // this gets the varlena Datum copied into this memory context
                let detoasted = pg_sys::pg_detoast_datum_copy(datum.cast_mut_ptr());

                // and we need to unpack it (if necessary), which will decompress it too
                let varlena = pg_sys::pg_detoast_datum_packed(detoasted);

                // and now we return it as a &[u8]
                Some(varlena_to_byte_slice(varlena))
            })
        }
    }
}

impl FromDatum for Vec<u8> {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: u32,
    ) -> Option<Vec<u8>> {
        if is_null || datum.is_null() {
            None
        } else {
            // Vec<u8> conversion is initially the same as for &[u8]
            let bytes: Option<&[u8]> = FromDatum::from_polymorphic_datum(datum, is_null, typoid);

            match bytes {
                // but then we need to convert it into an owned Vec where the backing
                // data is allocated by Rust
                Some(bytes) => Some(bytes.into_iter().map(|b| *b).collect::<Vec<u8>>()),
                None => None,
            }
        }
    }
}

impl FromDatum for () {
    #[inline]
    unsafe fn from_polymorphic_datum(
        _datum: pg_sys::Datum,
        _is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<()> {
        Some(())
    }
}

/// for user types
impl<T> FromDatum for PgBox<T, AllocatedByPostgres> {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<Self> {
        if is_null || datum.is_null() {
            None
        } else {
            Some(PgBox::<T>::from_pg(datum.cast_mut_ptr()))
        }
    }

    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: u32,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        memory_context.switch_to(|context| {
            if is_null || datum.is_null() {
                None
            } else {
                let copied = context.copy_ptr_into(datum.cast_mut_ptr(), std::mem::size_of::<T>());
                Some(PgBox::<T>::from_pg(copied))
            }
        })
    }
}

impl<T> FromDatum for Option<T>
where
    T: FromDatum,
{
    unsafe fn from_polymorphic_datum(datum: Datum, is_null: bool, typoid: Oid) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            Some(None)
        } else {
            Some(T::from_polymorphic_datum(datum, is_null, typoid))
        }
    }
}
