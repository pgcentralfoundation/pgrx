//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! for converting a pg_sys::Datum and a corresponding "is_null" bool into a typed Option
#![allow(clippy::manual_map, clippy::map_clone, clippy::into_iter_on_ref)]
use super::UnboxDatum;
use crate::{
    AllocatedByPostgres, IntoDatum, PgBox, PgMemoryContexts, pg_sys, varlena, varlena_to_byte_slice,
};
use alloc::ffi::CString;
use core::ffi::CStr;
use std::num::NonZeroUsize;

/// If converting a Datum to a Rust type fails, this is the set of possible reasons why.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum TryFromDatumError {
    #[error(
        "Postgres type {datum_type} (Oid({datum_oid})) is not compatible with the Rust type {rust_type} (Oid({rust_oid}))"
    )]
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

/// Convert a `(pg_sys::Datum, is_null:bool)` pair into a Rust type
///
/// Default implementations are provided for the common Rust types.
///
/// If implementing this, also implement `IntoDatum` for the reverse
/// conversion.
pub trait FromDatum: Sized {
    /// Should a type OID be fetched when calling `from_datum`?
    const GET_TYPOID: bool = false;

    /// Does `from_polymorphic_datum` return a value that points into the Datum's memory but has
    /// no lifetime that keeps that memory alive?
    ///
    /// Code that can free the Datum's memory while the value lives reads such a type with
    /// [`FromDatum::from_datum_in_memory_context`], which must then return a copy.
    /// [`PgHeapTuple::get_by_index`](crate::heap_tuple::PgHeapTuple::get_by_index) does, because
    /// a tuple frees its memory when it drops or when an attribute is set.
    const POINTS_INTO_DATUM: bool = false;

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
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<Self> {
        FromDatum::from_polymorphic_datum(datum, is_null, pg_sys::Oid::INVALID)
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
    ) -> Option<Self>;

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
    /// Same caveats as `FromDatum::from_datum(...)`. The returned value may point into
    /// `memory_context`, as a `&str` does, so the context must stay live while the value is used.
    /// [`PgMemoryContexts::Transient`] and [`PgMemoryContexts::Owned`] contexts are deleted before
    /// this returns. Pass them only for a type that copies its value out, such as `String`.
    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self> {
        memory_context.switch_to(|_| FromDatum::from_polymorphic_datum(datum, is_null, typoid))
    }

    /// `try_from_datum` is a convenience wrapper around `FromDatum::from_datum` that returns a
    /// a `Result` around an `Option`, as a Datum can be null.  It's intended to be used in
    /// situations where the caller needs to know whether the type conversion succeeded or failed.
    ///
    /// ## Errors
    ///
    /// Returns [`TryFromDatumError::IncompatibleTypes`] if `type_oid` is InvalidOid, or if
    /// [`IntoDatum::is_compatible_with`] rejects `type_oid` and it either names no type or is not
    /// binary coercible to [`IntoDatum::type_oid`].
    ///
    /// [`IntoDatum::type_oid`] itself can raise an ERROR: `PgBox<T>` and `#[derive(PostgresType)]`
    /// types look their type up by name among the extension's types, and the extension may have no
    /// type of that name.
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
        Self: IntoDatum,
    {
        check_compatible::<Self>(type_oid)?;
        Ok(FromDatum::from_polymorphic_datum(datum, is_null, type_oid))
    }

    /// A version of `try_from_datum` that switches to the given context to convert from Datum
    ///
    /// ## Errors
    ///
    /// The same as [`FromDatum::try_from_datum`].
    ///
    /// ## Safety
    ///
    /// Same caveats as [`FromDatum::from_datum_in_memory_context`], including how long
    /// `memory_context` must stay live.
    #[inline]
    unsafe fn try_from_datum_in_memory_context(
        memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        type_oid: pg_sys::Oid,
    ) -> Result<Option<Self>, TryFromDatumError>
    where
        Self: IntoDatum,
    {
        check_compatible::<Self>(type_oid)?;
        Ok(FromDatum::from_datum_in_memory_context(memory_context, datum, is_null, type_oid))
    }
}

/// Returns [`TryFromDatumError::IncompatibleTypes`] unless `T` can read a value of type
/// `type_oid`, as [`FromDatum::try_from_datum`] describes.
fn check_compatible<T: IntoDatum>(type_oid: pg_sys::Oid) -> Result<(), TryFromDatumError> {
    if is_binary_coercible::<T>(type_oid) { Ok(()) } else { Err(incompatible::<T>(type_oid)) }
}

fn incompatible<T: IntoDatum>(type_oid: pg_sys::Oid) -> TryFromDatumError {
    TryFromDatumError::IncompatibleTypes {
        rust_type: std::any::type_name::<T>(),
        rust_oid: T::type_oid(),
        datum_type: lookup_type_name(type_oid),
        datum_oid: type_oid,
    }
}

/// Can `T` read a value of type `type_oid`? Returns false, without raising an ERROR, for
/// InvalidOid and for an OID that names no type.
fn is_binary_coercible<T: IntoDatum>(type_oid: pg_sys::Oid) -> bool {
    // InvalidOid names no type, but it equals `T::type_oid()` when T's type does not exist, as
    // for `Array<Array<i32>>` (int4[] has no array type), and IsBinaryCoercible accepts it for
    // `anyelement`.
    if type_oid == pg_sys::Oid::INVALID {
        return false;
    }
    T::is_compatible_with(type_oid)
        // SAFETY: both functions take the OIDs by value and only look them up. get_typtype
        // returns '\0' for an OID with no pg_type row, for which IsBinaryCoercible would raise
        // an ERROR.
        || unsafe {
            pg_sys::get_typtype(type_oid) != 0 && pg_sys::IsBinaryCoercible(type_oid, T::type_oid())
        }
}

/// Retrieves a Postgres type name given its Oid: `-` for InvalidOid and `???` for an OID that
/// names no type.
pub(crate) fn lookup_type_name(oid: pg_sys::Oid) -> String {
    // The bindings type the flag as u32. format_type_extended takes bits16 before PG19 and
    // uint16 since; both are u16.
    const ALLOW_INVALID: u16 = {
        assert!(pg_sys::FORMAT_TYPE_ALLOW_INVALID <= u16::MAX as u32);
        pg_sys::FORMAT_TYPE_ALLOW_INVALID as u16
    };
    unsafe {
        // SAFETY: format_type_extended takes the OID by value. FORMAT_TYPE_ALLOW_INVALID makes
        // it name an invalid or unknown OID instead of raising an ERROR, and without
        // FORMAT_TYPE_INVALID_AS_NULL it always returns a palloc'd C string, never null.
        let cstr_name = pg_sys::format_type_extended(oid, -1, ALLOW_INVALID);
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
        if is_null { None } else { Some(datum) }
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
            // DatumGetObjectId casts the Datum's stored value to Oid; it does not
            // treat those bits as a pointer to an unsigned integer and dereference it.
            // SAFETY: FromDatum's caller provides a valid non-null datum of this PostgreSQL type.
            // Where this conversion is a guarded native call, the guard panics off the
            // backend thread.
            #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
            let value = unsafe { pg_sys::DatumGetObjectId!(datum).get() };
            Some(pg_sys::Oid::from(value))
        }
    }
}

impl FromDatum for pg_sys::TransactionId {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<Self> {
        if is_null { None } else { datum.value().try_into().ok().map(Self::from_inner) }
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
            // SAFETY: FromDatum's caller provides a valid non-null datum of this PostgreSQL type.
            // Where this conversion is a guarded native call, the guard panics off the
            // backend thread.
            #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
            let value = unsafe { pg_sys::DatumGetBool!(datum).get() };
            Some(value)
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
            // SAFETY: FromDatum's caller provides a valid non-null datum of this PostgreSQL type.
            // Where this conversion is a guarded native call, the guard panics off the
            // backend thread.
            #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
            let value = unsafe { pg_sys::DatumGetChar!(datum).get() };
            Some(value as i8)
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
            // SAFETY: FromDatum's caller provides a valid non-null datum of this PostgreSQL type.
            // Where this conversion is a guarded native call, the guard panics off the
            // backend thread.
            #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
            let value = unsafe { pg_sys::DatumGetInt16!(datum).get() };
            Some(value)
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
            // SAFETY: FromDatum's caller provides a valid non-null datum of this PostgreSQL type.
            // Where this conversion is a guarded native call, the guard panics off the
            // backend thread.
            #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
            let value = unsafe { pg_sys::DatumGetInt32!(datum).get() };
            Some(value)
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
            // SAFETY: FromDatum's caller provides a valid non-null datum of this PostgreSQL type.
            // Where this conversion is a guarded native call, the guard panics off the
            // backend thread.
            #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
            let value = unsafe { pg_sys::DatumGetUInt32!(datum).get() };
            Some(value)
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
            // SAFETY: FromDatum's caller provides a valid non-null datum of this PostgreSQL type.
            // Where this conversion is a guarded native call, the guard panics off the
            // backend thread.
            #[allow(unused_unsafe, reason = "pure where it is a macro or translated")]
            let value = unsafe { pg_sys::DatumGetInt64!(datum).get() };
            Some(value)
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
        if is_null { None } else { Some(f32::from_bits(datum.value() as _)) }
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
            let value = if size_of::<i64>() <= size_of::<pg_sys::Datum>() {
                f64::from_bits(datum.value() as _)
            } else {
                *(datum.cast_mut_ptr() as *const _)
            };
            Some(value)
        }
    }
}

/// For Postgres text and varchar
///
/// Note that while these conversions are inherently unsafe, they still enforce
/// UTF-8 correctness, so they may panic if you use PGRX with a database
/// that has non-UTF-8 data. The details of this are subject to change.
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

            // Postgres supports non-UTF-8 character sets. Rust supports them... less.
            // There are a lot of options for how to proceed in response.
            // This could return None, but Rust programmers may assume UTF-8,
            // so panics align more with informing programmers of wrong assumptions.
            // In order to reduce optimization loss here, let's call into a memoized function.
            Some(convert_varlena_to_str_memoized(varlena))
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
                Some(convert_varlena_to_str_memoized(varlena))
            })
        }
    }
}

/// Borrow a text value as `str`, panicking if the database encoding lets it hold bytes that are
/// not UTF-8 and it does.
///
/// # Safety
///
/// The same as [`text_to_rust_str_unchecked`], except that the payload need not be UTF-8.
///
/// [`text_to_rust_str_unchecked`]: crate::varlena::text_to_rust_str_unchecked
// This is not marked inline on purpose, to allow it to be in a single code section
// which is then branch-predicted on every time by the CPU.
pub(crate) unsafe fn convert_varlena_to_str_memoized<'a>(
    varlena: *const pg_sys::varlena,
) -> &'a str {
    match *crate::UTF8DATABASE {
        crate::Utf8Compat::Yes => varlena::text_to_rust_str_unchecked(varlena),
        crate::Utf8Compat::Maybe => varlena::text_to_rust_str(varlena)
            .expect("datums converted to &str should be valid UTF-8"),
        crate::Utf8Compat::Ascii => {
            let bytes = varlena_to_byte_slice(varlena);
            if bytes.is_ascii() {
                core::str::from_utf8_unchecked(bytes)
            } else {
                panic!(
                    "datums converted to &str should be valid UTF-8, database encoding is only UTF-8 compatible for ASCII"
                )
            }
        }
    }
}

/// For Postgres text, varchar, or any `pg_sys::varlena`-based type
///
/// This returns a **copy**, allocated and managed by Rust, of the underlying `varlena` Datum
///
/// Note that while these conversions are inherently unsafe, they still enforce
/// UTF-8 correctness, so they may panic if you use PGRX with a database
/// that has non-UTF-8 data. The details of this are subject to change.
impl FromDatum for String {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<String> {
        if is_null || datum.is_null() {
            None
        } else {
            let varlena = pg_sys::pg_detoast_datum_packed(datum.cast_mut_ptr());
            let converted_varlena = convert_varlena_to_str_memoized(varlena);
            let ret_string = converted_varlena.to_owned();

            // If the datum is EXTERNAL or COMPRESSED, then detoasting creates a pfree-able chunk
            // that needs to be freed. We can free it because `to_owned` above creates a Rust copy
            // of the string.
            if pg_sys::VARATT_IS_1B_E!(datum.cast_mut_ptr::<pg_sys::varlena>()).get() != 0
                || pg_sys::VARATT_IS_4B_C!(datum.cast_mut_ptr::<pg_sys::varlena>()).get() != 0
            {
                pg_sys::pfree(varlena.cast());
            }

            Some(ret_string)
        }
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
impl<'a> FromDatum for &'a CStr {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<&'a CStr> {
        if is_null || datum.is_null() { None } else { Some(CStr::from_ptr(datum.cast_mut_ptr())) }
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
            let copy =
                memory_context.switch_to(|_| CStr::from_ptr(pg_sys::pstrdup(datum.cast_mut_ptr())));

            Some(copy)
        }
    }
}

impl FromDatum for CString {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<CString> {
        if is_null || datum.is_null() {
            None
        } else {
            Some(CStr::from_ptr(datum.cast_mut_ptr()).to_owned())
        }
    }

    /// Nonsensical because CString is always allocated within Rust memory.
    unsafe fn from_datum_in_memory_context(
        _memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_polymorphic_datum(datum, is_null, _typoid)
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
///
/// `from_polymorphic_datum` boxes the Datum's pointer, which can refer to a PostgreSQL struct
/// passed as `internal`. `from_datum_in_memory_context` copies `size_of::<T>()` bytes into the
/// memory context instead.
///
/// `try_from_datum` and `try_from_datum_in_memory_context` also return
/// [`TryFromDatumError::IncompatibleTypes`] for a type whose values a copy of `size_of::<T>()`
/// bytes cannot hold, as [`UnboxDatum::can_unbox`] decides for array elements: a variable-length
/// type, a type passed by value, or a fixed-length type of another size.
impl<T> FromDatum for PgBox<T, AllocatedByPostgres> {
    const POINTS_INTO_DATUM: bool = true;

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
        assert!(
            align_of::<T>() <= pg_sys::MAXIMUM_ALIGNOF as usize,
            "palloc cannot align a copy for {}",
            std::any::type_name::<T>()
        );
        memory_context.switch_to(|context| {
            if is_null || datum.is_null() {
                None
            } else {
                // SAFETY: the caller passes a Datum that points to a `T`, so its
                // `size_of::<T>()` bytes are readable. The copy is a palloc chunk holding those
                // bytes, which palloc aligns to MAXALIGN, enough for `T` as asserted above, and it
                // does not point into the Datum, as `POINTS_INTO_DATUM` requires.
                let copied = context.copy_ptr_into(datum.cast_mut_ptr(), std::mem::size_of::<T>());
                Some(PgBox::<T>::from_pg(copied))
            }
        })
    }

    unsafe fn try_from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        type_oid: pg_sys::Oid,
    ) -> Result<Option<Self>, TryFromDatumError>
    where
        Self: IntoDatum,
    {
        check_pgbox_layout::<T>(type_oid)?;
        Ok(unsafe { Self::from_polymorphic_datum(datum, is_null, type_oid) })
    }

    unsafe fn try_from_datum_in_memory_context(
        memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        type_oid: pg_sys::Oid,
    ) -> Result<Option<Self>, TryFromDatumError>
    where
        Self: IntoDatum,
    {
        check_pgbox_layout::<T>(type_oid)?;
        Ok(unsafe { Self::from_datum_in_memory_context(memory_context, datum, is_null, type_oid) })
    }
}

/// Checks that `PgBox<T>` can read a value of type `type_oid`: that the types are compatible,
/// and that [`UnboxDatum::can_unbox`] accepts the type's length and pass-by-value flag.
fn check_pgbox_layout<T>(type_oid: pg_sys::Oid) -> Result<(), TryFromDatumError> {
    check_compatible::<PgBox<T>>(type_oid)?;
    let (mut typlen, mut typbyval) = (0, false);
    // SAFETY: get_typlenbyval takes the OID by value and writes both out-parameters. It raises
    // an ERROR for an OID that names no type, which check_compatible has already rejected.
    unsafe { pg_sys::get_typlenbyval(type_oid, &mut typlen, &mut typbyval) };
    if <PgBox<T> as UnboxDatum>::can_unbox(typlen, typbyval) {
        Ok(())
    } else {
        Err(incompatible::<PgBox<T>>(type_oid))
    }
}
