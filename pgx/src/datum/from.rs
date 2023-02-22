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
use core::ffi::CStr;
use std::num::NonZeroUsize;

/// If converting a Datum to a Rust type fails, this is the set of possible reasons why.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum TryFromDatumError {
    #[error("Postgres type {datum_type} {datum_oid} is not compatible with the Rust type {rust_type} {rust_oid}")]
    IncompatibleTypes {
        rust_type: &'static str,
        rust_oid: pg_sys::Oid,
        datum_type: String,
        datum_oid: pg_sys::Oid,
    },

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
    /// a `Result` around an `Option`, as a Datum can be null.  It's intended to be used in
    /// situations where the caller needs to know whether the type conversion succeeded or failed.
    ///
    /// ## Safety
    ///
    /// Same caveats as `FromDatum::from_datum(...)`
    #[inline]
    unsafe fn try_from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        type_oid: pg_sys::Oid,
    ) -> Result<Option<Self>, TryFromDatumError>
    where
        Self: Sized + IntoDatum,
    {
        if !is_binary_coercible::<Self>(type_oid) {
            Err(TryFromDatumError::IncompatibleTypes {
                rust_type: std::any::type_name::<Self>(),
                rust_oid: Self::type_oid(),
                datum_type: lookup_type_name(type_oid),
                datum_oid: type_oid,
            })
        } else {
            Ok(FromDatum::from_polymorphic_datum(datum, is_null, type_oid))
        }
    }

    /// A version of `try_from_datum` that switches to the given context to convert from Datum
    #[inline]
    unsafe fn try_from_datum_in_memory_context(
        memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        type_oid: pg_sys::Oid,
    ) -> Result<Option<Self>, TryFromDatumError>
    where
        Self: Sized + IntoDatum,
    {
        if !is_binary_coercible::<Self>(type_oid) {
            Err(TryFromDatumError::IncompatibleTypes {
                rust_type: std::any::type_name::<Self>(),
                rust_oid: Self::type_oid(),
                datum_type: lookup_type_name(type_oid),
                datum_oid: type_oid,
            })
        } else {
            Ok(FromDatum::from_datum_in_memory_context(memory_context, datum, is_null, type_oid))
        }
    }
}

fn is_binary_coercible<T: IntoDatum>(type_oid: pg_sys::Oid) -> bool {
    T::is_compatible_with(type_oid) || unsafe { pg_sys::IsBinaryCoercible(type_oid, T::type_oid()) }
}

/// Retrieves a Postgres type name given its Oid
pub(crate) fn lookup_type_name(oid: pg_sys::Oid) -> String {
    unsafe {
        // SAFETY: nothing to concern ourselves with other than just calling into Postgres FFI
        // and Postgres will raise an ERROR if we pass it an invalid Oid, so it'll never return a null
        let cstr_name = pg_sys::format_type_extended(oid, -1, 0);
        let cstr = CStr::from_ptr(cstr_name);
        let typname = cstr.to_string_lossy().to_string();
        pg_sys::pfree(cstr_name as _); // don't leak the palloc'd cstr_name
        typname
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

impl FromDatum for pg_sys::Oid {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<pg_sys::Oid> {
        if is_null {
            None
        } else {
            datum
                .value()
                .try_into()
                .ok()
                .map(|uint| unsafe { pg_sys::Oid::from_u32_unchecked(uint) })
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
        _typoid: pg_sys::Oid,
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
impl<'a> FromDatum for &'a core::ffi::CStr {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<&'a CStr> {
        if is_null || datum.is_null() {
            None
        } else {
            Some(core::ffi::CStr::from_ptr(datum.cast_mut_ptr()))
        }
    }
}

/// for bytea
impl<'a> FromDatum for &'a [u8] {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
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
        _typoid: pg_sys::Oid,
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
        typoid: pg_sys::Oid,
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

/// for VOID -- always converts to `Some(())`, even if the "is_null" argument is true
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
        _typoid: pg_sys::Oid,
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
