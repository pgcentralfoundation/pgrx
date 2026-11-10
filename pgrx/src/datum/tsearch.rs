//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Full-text search types: `tsvector` and `tsquery`.
//!
//! [`TsVector`] and [`TsQuery`] are unsized views over a detoasted varlena, like `str`.
//! [`TsVectorBuf`] and [`TsQueryBuf`] own a copy in Rust memory, like `String`, and dereference to
//! the view.
//!
//! Operations that call Postgres raise a Postgres error, as a panic, when Postgres rejects the
//! input or the result.
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(
    unused_unsafe,
    reason = "docs.rs builds replace generated C macros with shells, some of which need no unsafe"
)]
mod item;
mod lexeme;

use crate::callconv::{Arg, ArgAbi};
use crate::datum::{FromDatum, IntoDatum};
use crate::nullable::Nullable;
use crate::pg_sys::PgTryBuilder;
use crate::pg_sys::errcodes::PgSqlErrorCode;
use crate::pg_sys::panic::CaughtError;
use crate::{direct_function_call, direct_function_call_as_datum, pg_sys};
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::ops::Deref;
use core::ptr::{self, NonNull};
use core::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::ffi::{CStr, CString};
use std::panic::UnwindSafe;

pub use item::{Item, Items, Operand, Operator, WeightSet};
pub use lexeme::{InvalidWeight, Lexeme, Lexemes, Position, Weight};

/// Size of the varlena header and the `size` field that precede the entries of both types.
const HEADER_SIZE: usize = 8;

type PgFunction = unsafe fn(pg_sys::FunctionCallInfo) -> pg_sys::Datum;

const _: () = assert!(
    size_of::<pg_sys::TSVectorData>() == HEADER_SIZE
        && size_of::<pg_sys::TSQueryData>() == HEADER_SIZE
);

/// Copies `bytes` into 8-aligned storage, zero-padding the last word.
fn words_from_bytes(bytes: &[u8]) -> Box<[u64]> {
    let mut words = vec![0_u64; bytes.len().div_ceil(8)].into_boxed_slice();
    // SAFETY: `words` has at least `bytes.len()` bytes and is disjoint from `bytes`.
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), words.as_mut_ptr().cast::<u8>(), bytes.len());
    }
    words
}

/// Like [`words_from_bytes`], with the varlena header set to the length of `image`.
fn varlena_words(image: &[u8]) -> Box<[u64]> {
    assert!(image.len() >= 4, "image holds a varlena header");
    let mut words = words_from_bytes(image);
    // SAFETY: the storage is 8-aligned and holds at least the four header bytes.
    unsafe {
        pg_sys::SET_VARSIZE_4B!(words.as_mut_ptr().cast::<pg_sys::varlena>(), image.len() as i32)
            .get();
    }
    words
}

/// The error returned when parsing a `tsvector` or `tsquery` from text fails.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct ParseError(String);

fn parse_error<T>(e: CaughtError) -> Result<T, ParseError> {
    match e {
        CaughtError::PostgresError(ref report) => Err(ParseError(report.message().to_string())),
        e => e.rethrow(),
    }
}

/// Reports whether `s` can be passed to Postgres as a C string in the database encoding.
///
/// Rust strings are UTF-8 and are passed without conversion. Postgres input functions expect a
/// C string that is valid in the database encoding, and find character boundaries by its rules.
/// In an encoding that only extends ASCII, non-ASCII UTF-8 bytes do not meet that expectation.
fn encodable(s: &str) -> bool {
    !matches!(*crate::UTF8DATABASE, crate::Utf8Compat::Ascii) || s.is_ascii()
}

/// Runs `f`, turning the Postgres errors that malformed input raises into a [`ParseError`].
/// Other errors propagate.
fn catch_parse_errors<T>(f: impl FnOnce() -> T + UnwindSafe) -> Result<T, ParseError> {
    PgTryBuilder::new(|| Ok(f()))
        .catch_when(PgSqlErrorCode::ERRCODE_SYNTAX_ERROR, parse_error)
        .catch_when(PgSqlErrorCode::ERRCODE_PROGRAM_LIMIT_EXCEEDED, parse_error)
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE, parse_error)
        .execute()
}

macro_rules! tsearch_type {
    (
        $(#[$dst_meta:meta])* $dst:ident,
        $(#[$buf_meta:meta])* $buf:ident,
        $sql:literal, $oid:ident, $in:ident, $out:ident, $cmp:ident
    ) => {
        $(#[$dst_meta])*
        #[repr(transparent)]
        pub struct $dst([u8]);

        $(#[$buf_meta])*
        #[derive(Clone)]
        pub struct $buf(Box<[u64]>);

        impl $dst {
            /// Borrows the varlena at `ptr`.
            ///
            /// # Safety
            ///
            /// `ptr` must satisfy the invariants documented on the type and stay valid and
            /// unmodified for `'a`.
            pub(crate) unsafe fn from_varlena<'a>(ptr: NonNull<pg_sys::varlena>) -> &'a Self {
                debug_assert!(unsafe { pg_sys::VARATT_IS_4B_U!(ptr.as_ptr()).get() } != 0);
                let len = unsafe { pg_sys::VARSIZE_4B!(ptr.as_ptr()).get() } as usize;
                let bytes = ptr::slice_from_raw_parts(ptr.as_ptr().cast::<u8>(), len);
                // SAFETY: `Self` is `repr(transparent)` over `[u8]`; the caller guarantees `len`
                // initialized bytes behind `ptr` that live for `'a`.
                unsafe { &*(bytes as *const Self) }
            }

            /// Returns the underlying varlena pointer.
            pub fn as_varlena_ptr(&self) -> *const pg_sys::varlena {
                self.0.as_ptr().cast()
            }

            /// Returns true if the value contains no items.
            pub fn is_empty(&self) -> bool {
                self.header_size() == 0
            }

            fn header_size(&self) -> i32 {
                let bytes = self.0.get(4..8).expect("size field within a tsearch varlena");
                i32::from_ne_bytes(bytes.try_into().unwrap())
            }

            /// Copies the varlena into `CurrentMemoryContext`.
            pub(crate) fn to_datum_copy(&self) -> pg_sys::Datum {
                // SAFETY: the allocation has `self.0.len()` bytes and is disjoint from `self`.
                unsafe {
                    let dest = pg_sys::palloc(self.0.len()).cast::<u8>();
                    ptr::copy_nonoverlapping(self.0.as_ptr(), dest, self.0.len());
                    pg_sys::Datum::from(dest)
                }
            }
        }

        impl $buf {
            /// Calls `f` and copies its result into Rust memory.
            ///
            /// # Safety
            ///
            /// `args` must be valid arguments for `f`, and `f` must return a fresh varlena of
            /// this type, which is freed here.
            pub(crate) unsafe fn call_owned(
                f: PgFunction,
                args: &[Option<pg_sys::Datum>],
            ) -> Self {
                // SAFETY: the caller guarantees the call and the result's ownership.
                unsafe {
                    let datum =
                        direct_function_call_as_datum(f, args).expect("function returned null");
                    let buf = Self::copy_from(datum.cast_mut_ptr());
                    pg_sys::pfree(datum.cast_mut_ptr());
                    buf
                }
            }

            /// Copies the (possibly toasted) value at `ptr` into Rust memory.
            ///
            /// # Safety
            ///
            /// `ptr` must point to a valid varlena of this type.
            pub(crate) unsafe fn copy_from(ptr: *const pg_sys::varlena) -> Self {
                // SAFETY: the caller guarantees a valid varlena. `pg_detoast_datum` returns a
                // 4-byte-header varlena of `VARSIZE` bytes, either `ptr` or a fresh copy.
                unsafe {
                    let detoasted = pg_sys::pg_detoast_datum(ptr.cast_mut());
                    let size = pg_sys::VARSIZE_4B!(detoasted).get() as usize;
                    let words = words_from_bytes(core::slice::from_raw_parts(
                        detoasted.cast::<u8>(),
                        size,
                    ));
                    if !ptr::eq(detoasted, ptr) {
                        pg_sys::pfree(detoasted.cast());
                    }
                    Self(words)
                }
            }
        }

        impl $dst {
            /// Calls `f` with the text form in the database encoding, if it is valid UTF-8.
            fn with_text<R>(&self, f: impl FnOnce(&str) -> R) -> Result<R, fmt::Error> {
                // SAFETY: `self` is a valid varlena of this type. The output function returns a
                // NUL-terminated string allocated by `palloc`, which is freed after `f` returns.
                unsafe {
                    let arg = pg_sys::Datum::from(self.as_varlena_ptr());
                    let out = direct_function_call_as_datum(pg_sys::$out, &[Some(arg)])
                        .ok_or(fmt::Error)?;
                    let result = CStr::from_ptr(out.cast_mut_ptr()).to_str().map(f);
                    pg_sys::pfree(out.cast_mut_ptr());
                    result.map_err(|_| fmt::Error)
                }
            }

            fn fmt_debug(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.with_text(|text| f.debug_tuple(name).field(&text).finish())?
            }
        }

        impl fmt::Display for $dst {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.with_text(|text| f.pad(text))?
            }
        }

        impl fmt::Debug for $dst {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.fmt_debug(stringify!($dst), f)
            }
        }

        impl fmt::Display for $buf {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&**self, f)
            }
        }

        impl fmt::Debug for $buf {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.fmt_debug(stringify!($buf), f)
            }
        }

        impl Ord for $dst {
            fn cmp(&self, other: &Self) -> Ordering {
                // SAFETY: both are valid varlenas of this type with 4-byte headers, so the
                // function reads them in place and frees nothing.
                let result = unsafe {
                    direct_function_call::<i32>(
                        pg_sys::$cmp,
                        &[
                            Some(pg_sys::Datum::from(self.as_varlena_ptr())),
                            Some(pg_sys::Datum::from(other.as_varlena_ptr())),
                        ],
                    )
                };
                result.expect("comparison function returned null").cmp(&0)
            }
        }

        impl PartialOrd for $dst {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for $dst {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == Ordering::Equal
            }
        }

        impl Eq for $dst {}

        impl Ord for $buf {
            fn cmp(&self, other: &Self) -> Ordering {
                (**self).cmp(&**other)
            }
        }

        impl PartialOrd for $buf {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for $buf {
            fn eq(&self, other: &Self) -> bool {
                **self == **other
            }
        }

        impl Eq for $buf {}

        impl PartialOrd<$dst> for $buf {
            fn partial_cmp(&self, other: &$dst) -> Option<Ordering> {
                (**self).partial_cmp(other)
            }
        }

        impl PartialEq<$dst> for $buf {
            fn eq(&self, other: &$dst) -> bool {
                **self == *other
            }
        }

        impl PartialOrd<$buf> for $dst {
            fn partial_cmp(&self, other: &$buf) -> Option<Ordering> {
                self.partial_cmp(&**other)
            }
        }

        impl PartialEq<$buf> for $dst {
            fn eq(&self, other: &$buf) -> bool {
                *self == **other
            }
        }

        impl Default for $buf {
            /// Returns the empty value.
            fn default() -> Self {
                Self(varlena_words(&[0; HEADER_SIZE]))
            }
        }

        impl FromStr for $buf {
            type Err = ParseError;

            /// Parses the text form with the Postgres input function.
            ///
            /// Postgres reports malformed input as a [`ParseError`]. Other errors, such as an
            /// out-of-memory condition, propagate as Postgres errors. Non-ASCII input is an error
            /// when the database encoding is not UTF-8 but extends ASCII, as in `EUC_JP`.
            fn from_str(s: &str) -> Result<Self, ParseError> {
                if s.is_empty() {
                    return Ok(Self::default());
                }
                if !encodable(s) {
                    return Err(ParseError(
                        "text is not ASCII and the database encoding is not UTF-8".into(),
                    ));
                }
                let cstring = CString::new(s).map_err(|_| ParseError("embedded NUL".into()))?;
                catch_parse_errors(|| {
                    // SAFETY: `cstring` is NUL-terminated and outlives the call.
                    unsafe {
                        let arg = pg_sys::Datum::from(cstring.as_ptr());
                        Self::call_owned(pg_sys::$in, &[Some(arg)])
                    }
                })
            }
        }

        impl Serialize for $dst {
            /// Serializes the text form.
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str(self)
            }
        }

        impl Serialize for $buf {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                (**self).serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $buf {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
            }
        }

        impl Deref for $buf {
            type Target = $dst;

            fn deref(&self) -> &$dst {
                let ptr = self.0.as_ptr().cast::<pg_sys::varlena>().cast_mut();
                // SAFETY: the storage is 8-aligned, holds a copy of a 4-byte-header varlena of
                // `VARSIZE` bytes, and is never mutated.
                unsafe { $dst::from_varlena(NonNull::new_unchecked(ptr)) }
            }
        }

        impl Borrow<$dst> for $buf {
            fn borrow(&self) -> &$dst {
                self
            }
        }

        impl AsRef<$dst> for $buf {
            fn as_ref(&self) -> &$dst {
                self
            }
        }

        impl ToOwned for $dst {
            type Owned = $buf;

            fn to_owned(&self) -> $buf {
                $buf(words_from_bytes(&self.0))
            }
        }

        impl FromDatum for $buf {
            unsafe fn from_polymorphic_datum(
                datum: pg_sys::Datum,
                is_null: bool,
                _typoid: pg_sys::Oid,
            ) -> Option<Self> {
                if is_null {
                    None
                } else {
                    // SAFETY: the caller supplies a datum of this type.
                    Some(unsafe { Self::copy_from(datum.cast_mut_ptr()) })
                }
            }
        }

        impl IntoDatum for $buf {
            fn into_datum(self) -> Option<pg_sys::Datum> {
                Some(self.deref().to_datum_copy())
            }

            fn type_oid() -> pg_sys::Oid {
                pg_sys::$oid
            }
        }

        impl IntoDatum for &$dst {
            fn into_datum(self) -> Option<pg_sys::Datum> {
                Some(self.to_datum_copy())
            }

            fn type_oid() -> pg_sys::Oid {
                pg_sys::$oid
            }
        }

        // `Bytea` takes the same route, but a short (1-byte header) varlena is rewritten to a
        // 4-byte header here, because the C functions use `VARSIZE`.
        unsafe impl<'fcx> ArgAbi<'fcx> for &'fcx $dst {
            unsafe fn unbox_arg_unchecked(arg: Arg<'_, 'fcx>) -> Self {
                // SAFETY: Postgres supplied a datum of this type. `pg_detoast_datum` returns the
                // argument itself or a copy in `CurrentMemoryContext`; both outlive `'fcx`.
                unsafe {
                    let datum: pg_sys::Datum = arg
                        .unbox_arg_using_from_datum::<pg_sys::Datum>()
                        .expect(concat!($sql, " argument must not be null"));
                    let varlena = pg_sys::pg_detoast_datum(datum.cast_mut_ptr());
                    let ptr = NonNull::new(varlena).expect("pg_detoast_datum returned null");
                    $dst::from_varlena(ptr)
                }
            }

            unsafe fn unbox_nullable_arg(arg: Arg<'_, 'fcx>) -> Nullable<Self> {
                if arg.is_null() {
                    Nullable::Null
                } else {
                    // SAFETY: the argument is not null.
                    Nullable::Valid(unsafe { Self::unbox_arg_unchecked(arg) })
                }
            }
        }

        crate::impl_sql_translatable!($dst, $sql);
        crate::impl_sql_translatable!($buf, $sql);
    };
}

tsearch_type! {
    /// A `tsvector` value.
    ///
    /// An unsized view over a detoasted varlena that includes its header. A `&TsVector` always
    /// points to
    ///
    /// 1. a varlena with a 4-byte uncompressed header, never packed, compressed or external;
    /// 2. exactly `VARSIZE` initialized bytes, produced by Postgres or copied from such a value;
    /// 3. memory aligned to at least 4 bytes.
    ///
    /// A `&TsVector` argument borrows from the function call. Use [`TsVectorBuf`] to keep a value
    /// beyond that, or to read one from SPI or a heap tuple.
    TsVector,
    /// An owned copy of a `tsvector` in Rust memory.
    ///
    /// The copy does not depend on any memory context, so it may outlive SPI connections and
    /// transactions. It dereferences to [`TsVector`].
    TsVectorBuf,
    "tsvector", TSVECTOROID, tsvectorin, tsvectorout, tsvector_cmp
}

tsearch_type! {
    /// A `tsquery` value.
    ///
    /// An unsized view over a detoasted varlena that includes its header. A `&TsQuery` always
    /// points to
    ///
    /// 1. a varlena with a 4-byte uncompressed header, never packed, compressed or external;
    /// 2. exactly `VARSIZE` initialized bytes, produced by Postgres or copied from such a value;
    /// 3. memory aligned to at least 4 bytes.
    ///
    /// A `&TsQuery` argument borrows from the function call. Use [`TsQueryBuf`] to keep a value
    /// beyond that, or to read one from SPI or a heap tuple.
    TsQuery,
    /// An owned copy of a `tsquery` in Rust memory.
    ///
    /// The copy does not depend on any memory context, so it may outlive SPI connections and
    /// transactions. It dereferences to [`TsQuery`].
    TsQueryBuf,
    "tsquery", TSQUERYOID, tsqueryin, tsqueryout, tsquery_cmp
}

impl TsVector {
    /// Returns true if the vector satisfies `query`, like the `@@` operator.
    pub fn matches(&self, query: &TsQuery) -> bool {
        // SAFETY: both are valid varlenas of the types the function expects.
        unsafe {
            direct_function_call::<bool>(
                pg_sys::ts_match_vq,
                &[
                    Some(pg_sys::Datum::from(self.as_varlena_ptr())),
                    Some(pg_sys::Datum::from(query.as_varlena_ptr())),
                ],
            )
        }
        .expect("ts_match_vq returned null")
    }

    /// Concatenates two vectors, like the `||` operator.
    ///
    /// Postgres raises an error if the result is too long for a `tsvector`.
    pub fn concat(&self, other: &TsVector) -> TsVectorBuf {
        // SAFETY: `tsvector_concat` returns a fresh varlena.
        unsafe {
            TsVectorBuf::call_owned(
                pg_sys::tsvector_concat,
                &[
                    Some(pg_sys::Datum::from(self.as_varlena_ptr())),
                    Some(pg_sys::Datum::from(other.as_varlena_ptr())),
                ],
            )
        }
    }
}

impl TsQuery {
    /// Combines two queries with AND, like the `&&` operator.
    pub fn and(&self, other: &TsQuery) -> TsQueryBuf {
        self.binary(pg_sys::tsquery_and, other)
    }

    /// Combines two queries with OR, like the `||` operator.
    pub fn or(&self, other: &TsQuery) -> TsQueryBuf {
        self.binary(pg_sys::tsquery_or, other)
    }

    /// Negates the query, like the `!!` operator.
    pub fn not(&self) -> TsQueryBuf {
        // SAFETY: `tsquery_not` returns a fresh copy.
        unsafe {
            TsQueryBuf::call_owned(
                pg_sys::tsquery_not,
                &[Some(pg_sys::Datum::from(self.as_varlena_ptr()))],
            )
        }
    }

    /// Requires `other` to follow this query immediately, like the `<->` operator.
    pub fn followed_by(&self, other: &TsQuery) -> TsQueryBuf {
        self.binary(pg_sys::tsquery_phrase, other)
    }

    /// Requires `other` to follow this query at exactly `distance` positions, like `<N>`.
    ///
    /// Postgres raises an error if `distance` is outside `0..=16384`.
    pub fn followed_by_at(&self, other: &TsQuery, distance: i32) -> TsQueryBuf {
        // SAFETY: `tsquery_phrase_distance` returns a fresh copy.
        unsafe {
            TsQueryBuf::call_owned(
                pg_sys::tsquery_phrase_distance,
                &[
                    Some(pg_sys::Datum::from(self.as_varlena_ptr())),
                    Some(pg_sys::Datum::from(other.as_varlena_ptr())),
                    Some(pg_sys::Datum::from(distance)),
                ],
            )
        }
    }

    fn binary(&self, f: PgFunction, other: &TsQuery) -> TsQueryBuf {
        // SAFETY: these functions copy their arguments and return a fresh copy, even when an
        // operand is empty.
        unsafe {
            TsQueryBuf::call_owned(
                f,
                &[
                    Some(pg_sys::Datum::from(self.as_varlena_ptr())),
                    Some(pg_sys::Datum::from(other.as_varlena_ptr())),
                ],
            )
        }
    }
}

impl core::ops::BitOr<&TsVector> for &TsVector {
    type Output = TsVectorBuf;

    fn bitor(self, other: &TsVector) -> TsVectorBuf {
        self.concat(other)
    }
}

impl core::ops::BitAnd<&TsQuery> for &TsQuery {
    type Output = TsQueryBuf;

    fn bitand(self, other: &TsQuery) -> TsQueryBuf {
        self.and(other)
    }
}

impl core::ops::BitOr<&TsQuery> for &TsQuery {
    type Output = TsQueryBuf;

    fn bitor(self, other: &TsQuery) -> TsQueryBuf {
        self.or(other)
    }
}

impl core::ops::Not for &TsQuery {
    type Output = TsQueryBuf;

    fn not(self) -> TsQueryBuf {
        TsQuery::not(self)
    }
}

impl core::ops::BitAnd for TsQueryBuf {
    type Output = TsQueryBuf;

    fn bitand(self, other: TsQueryBuf) -> TsQueryBuf {
        &*self & &*other
    }
}

impl core::ops::BitOr for TsQueryBuf {
    type Output = TsQueryBuf;

    fn bitor(self, other: TsQueryBuf) -> TsQueryBuf {
        &*self | &*other
    }
}

impl core::ops::Not for TsQueryBuf {
    type Output = TsQueryBuf;

    fn not(self) -> TsQueryBuf {
        !&*self
    }
}

#[cfg(test)]
mod test_support {
    pub(super) struct XorShift(pub u64);

    impl XorShift {
        pub fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }

        pub fn below(&mut self, n: usize) -> usize {
            (self.next() % n as u64) as usize
        }
    }
}
