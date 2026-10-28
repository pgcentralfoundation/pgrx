//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Wrapper for Postgres 'varlena' type, over Rust types of a fixed size (ie, `impl Copy`)
use crate::{FromDatum, IntoDatum, PgMemoryContexts, StringInfo, pg_sys, rust_regtypein};
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, ReturnsError, ReturnsRef, SqlMappingRef, SqlTranslatable,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::{ptr, slice};

/// A Rust type of a fixed size, stored in PostgreSQL as the payload of a `varlena`.
///
/// A `PgVarlena<T>` holds its value in Rust. Reading one from a Datum copies the payload into a
/// Rust heap allocation aligned for `T`, because PostgreSQL places the payload one or four bytes
/// past the varlena header without aligning it, and `Deref` and `DerefMut` lend references to
/// that copy. The value keeps no pointer into PostgreSQL memory, so it may outlive the Datum,
/// memory context, or SPI connection it was read from, and changing it never writes to that
/// Datum. [`PgVarlena::into_pg`] and [`IntoDatum`] allocate a new varlena holding the value in
/// `CurrentMemoryContext`.
///
/// Generally, `PgVarlena` is meant to be used in conjunction with pgrx's `PostgresType` derive macro
/// to provide transparent mapping of fixed-size Rust types as Postgres datums.
///
/// ## Padding
///
/// The payload holds every byte of the value, padding bytes included, as they are. A value read
/// from a Datum keeps the padding bytes it was read with. A value built in Rust has unspecified
/// padding: [`PgVarlena::new`], `PgVarlena::from`, `Default`, and assigning a whole `T` through
/// `DerefMut` (`*v = T { .. }`) copy a typed `T`, which can leave any bytes, such as stale stack
/// contents, in its padding. PostgreSQL writes those bytes to disk and WAL, and comparisons of
/// stored bytes, such as `record_image_eq` (`*=`) and `REFRESH MATERIALIZED VIEW CONCURRENTLY`,
/// then treat equal values as different.
///
/// For a type whose stored bytes matter, use a `#[repr(C)]` type without padding, adding
/// explicit padding fields where alignment would leave gaps. Otherwise start from
/// [`PgVarlena::new_zeroed`] and write the fields one at a time.
///
/// ## Example
///
/// ```rust
/// use std::str::FromStr;
///
/// use pgrx::prelude::*;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Copy, Clone, PostgresType, Serialize, Deserialize)]
/// #[pgvarlena_inoutfuncs]
/// struct MyType {
///    a: f32,
///    b: f32,
///    c: i64
/// }
///
/// impl PgVarlenaInOutFuncs for MyType {
///     fn input(input: &core::ffi::CStr) -> PgVarlena<Self> {
///         let mut iter = input.to_str().unwrap().split(',');
///         let (a, b, c) = (iter.next(), iter.next(), iter.next());
///
///         PgVarlena::from(MyType {
///             a: f32::from_str(a.unwrap()).expect("a is not a valid f32"),
///             b: f32::from_str(b.unwrap()).expect("b is not a valid f32"),
///             c: i64::from_str(c.unwrap()).expect("c is not a valid i64"),
///         })
///     }
///
///     fn output(&self, buffer: &mut pgrx::StringInfo) {
///         buffer.push_str(&format!("{},{},{}", self.a, self.b, self.c));
///     }
/// }
///
/// #[pg_extern]
/// fn do_a_thing(mut input: PgVarlena<MyType>) -> PgVarlena<MyType> {
///     input.c += 99;  // changes the Rust copy; the returned Datum is a new varlena
///     input
/// }
/// ```
pub struct PgVarlena<T>
where
    T: Copy + Sized,
{
    /// Boxed because a typed move of `T` need not keep padding bytes. Moving a `PgVarlena` moves
    /// only this pointer, so the value's padding bytes change only when a typed value is
    /// written over them through `DerefMut`.
    value: Box<T>,
    /// Payload bytes that followed the `T` in the Datum this value was read from.
    tail: Box<[u8]>,
}

impl<T> PgVarlena<T>
where
    T: Copy + Sized,
{
    /// Create a new `PgVarlena` holding `T::default()`. For a type without a `Default`
    /// implementation, build the value and convert it with `PgVarlena::from` instead.
    ///
    /// The value's padding bytes are unspecified, and [`into_pg`](PgVarlena::into_pg) stores them
    /// as they are; see [Padding](PgVarlena#padding).
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use pgrx::prelude::*;
    ///
    /// #[derive(Copy, Clone, Default)]
    /// struct MyType {
    ///    a: f32,
    ///    b: f32,
    ///    c: i64
    /// }
    ///
    /// let mut v = PgVarlena::<MyType>::new();
    /// v.a = 42.0;
    /// v.b = 0.424242;
    /// v.c = 42;
    /// ```
    pub fn new() -> Self
    where
        T: Default,
    {
        Self::from(T::default())
    }

    /// Create a new `PgVarlena` whose value is all zero bytes, padding included, as
    /// `PgVarlena::new` did before it required `T: Default`.
    ///
    /// This is how to store zero padding for a type that has padding. Writing a field through
    /// `DerefMut`, such as `v.a = 1`, changes only that field's bytes, so the padding stays zero
    /// in the payload that [`into_pg`](PgVarlena::into_pg) stores. Assigning a whole `T`, such as
    /// `*v = T { .. }`, or a field whose own type has padding, copies a typed value and leaves
    /// that padding unspecified; see [Padding](PgVarlena#padding).
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use pgrx::prelude::*;
    ///
    /// #[derive(Copy, Clone)]
    /// #[repr(C)]
    /// struct Reading {
    ///     id: u64,
    ///     flag: u8, // followed by 7 padding bytes
    /// }
    ///
    /// // SAFETY: all-zero bytes are a valid Reading.
    /// let mut v = unsafe { PgVarlena::<Reading>::new_zeroed() };
    /// v.id = 7;
    /// v.flag = 1;
    /// ```
    ///
    /// ## Safety
    ///
    /// All-zero bytes must be a valid value of `T`. That holds for integers, floats, `bool`, and
    /// raw pointers, and for arrays and structs made only of them. It does not hold for
    /// references, `NonZero` integers, function pointers, or an enum without a zero
    /// discriminant, among others.
    pub unsafe fn new_zeroed() -> Self {
        // SAFETY: the caller promises that all-zero bytes are a valid T.
        let value = unsafe { Box::<T>::new_zeroed().assume_init() };
        PgVarlena { value, tail: Box::default() }
    }

    /// Read a `PgVarlena` from a non-null varlena `pg_sys::Datum`. As
    /// `FromDatum for PgVarlena<T> where T: Copy + Sized` is already implemented, it is unlikely
    /// that this function will need to be called directly.
    ///
    /// The payload is copied into the returned value, which does not borrow `datum`. A compressed
    /// or external value is first detoasted into a temporary copy in `CurrentMemoryContext`,
    /// which is freed before returning. Payload bytes after the `T` are kept, and
    /// [`PgVarlena::into_pg`] writes them back after the value.
    ///
    /// ## Safety
    ///
    /// `datum` must point to a live varlena, possibly packed, compressed, or external, whose
    /// payload starts with the bytes of a valid `T`, such as a value that
    /// `PgVarlena::<T>::into_pg` produced. The call must run on the PostgreSQL backend thread.
    ///
    /// ## Panics
    ///
    /// Panics if the payload is shorter than `T`.
    pub unsafe fn from_datum(datum: pg_sys::Datum) -> Self {
        let ptr = datum.cast_mut_ptr::<pg_sys::varlena>();
        // SAFETY: the caller supplies a live varlena, whose header these macros read.
        let toasted = unsafe {
            pg_sys::VARATT_IS_COMPRESSED!(ptr).is_true()
                || pg_sys::VARATT_IS_EXTERNAL!(ptr).is_true()
        };
        if !toasted {
            // SAFETY: a varlena that is neither compressed nor external holds its whole payload
            // inline, and the caller promises that payload starts with a valid T.
            return unsafe { Self::from_inline_varlena(ptr) };
        }
        // SAFETY: for a compressed or external varlena, pg_detoast_datum_packed returns a new
        // plain copy allocated in CurrentMemoryContext, which only this block uses and frees.
        unsafe {
            let detoasted = pg_sys::pg_detoast_datum_packed(ptr);
            let value = Self::from_inline_varlena(detoasted);
            if detoasted != ptr {
                pg_sys::pfree(detoasted.cast());
            }
            value
        }
    }

    /// Copy the payload of a plain varlena, with a one- or four-byte header.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a live varlena that is neither compressed nor external and whose
    /// payload starts with the bytes of a valid `T`.
    unsafe fn from_inline_varlena(ptr: *mut pg_sys::varlena) -> Self {
        // SAFETY: the caller's live varlena header describes its inline payload.
        let (len, payload) = unsafe {
            (pg_sys::VARSIZE_ANY_EXHDR!(ptr).get() as usize, pg_sys::VARDATA_ANY!(ptr).get())
        };
        let payload = payload.cast::<u8>().cast_const();
        let size = size_of::<T>();
        assert!(len >= size, "varlena payload of {len} bytes cannot hold a {size}-byte value");

        let mut value = Box::<T>::new_uninit();
        // SAFETY: the payload has `len >= size` initialized bytes and cannot overlap the new
        // Rust allocation. The byte copies need no alignment and keep the payload's padding bytes.
        // The caller promises the first `size` bytes are a valid T, so `value` holds one after
        // the copy. The tail slice covers the remaining `len - size` payload bytes, and its
        // pointer is non-null and aligned for u8 even when that length is zero.
        let (value, tail) = unsafe {
            ptr::copy_nonoverlapping(payload, value.as_mut_ptr().cast::<u8>(), size);
            (value.assume_init(), Box::from(slice::from_raw_parts(payload.add(size), len - size)))
        };
        PgVarlena { value, tail }
    }

    /// Allocate a varlena holding this value in `CurrentMemoryContext`, and return it to pass to a
    /// Postgres function. The varlena belongs to that memory context; Rust does not free it.
    ///
    /// The payload is the bytes of the value, padding bytes included, followed by any payload
    /// bytes that came after the `T` in the Datum this value was read from. It gets a one-byte
    /// header when it fits one. Padding of a value built in Rust is unspecified; see
    /// [Padding](PgVarlena#padding).
    ///
    /// This method is also used by the `IntoDatum for PgVarlena<T> where T: Copy + Sized`
    /// implementation.
    pub fn into_pg(self) -> *mut pg_sys::varlena {
        let size = size_of::<T>();
        let payload_len =
            size.checked_add(self.tail.len()).expect("varlena payload length overflows usize");
        let total = payload_len
            .checked_add(pg_sys::VARHDRSZ as usize)
            .expect("varlena length overflows usize");

        // palloc0 raises an ERROR for any size above MaxAllocSize, so the header lengths
        // below fit C `int`.
        // SAFETY: palloc0 returns zeroed storage, aligned for any varlena header, or raises
        // an ERROR instead of returning NULL.
        let ptr = unsafe { pg_sys::palloc0(total) }.cast::<pg_sys::varlena>();

        // SAFETY: the new allocation has room for a four-byte header and `payload_len` bytes,
        // with no other reference to it. The short branch checks that its encoded size fits
        // the one-byte header. VARDATA_ANY reads the header just written, and the byte copies
        // fill exactly `payload_len` bytes after it, with no alignment requirement. `value` is
        // a valid T, initialized except perhaps for padding bytes, which an untyped copy may
        // read.
        unsafe {
            if payload_len + pg_sys::VARHDRSZ_SHORT as usize <= pg_sys::VARATT_SHORT_MAX as usize {
                pg_sys::SET_VARSIZE_SHORT!(
                    ptr,
                    (payload_len + pg_sys::VARHDRSZ_SHORT as usize) as i32
                )
                .get();
            } else {
                pg_sys::SET_VARSIZE_4B!(ptr, total as i32).get();
            }
            let data = pg_sys::VARDATA_ANY!(ptr).get().cast::<u8>();
            ptr::copy_nonoverlapping(ptr::from_ref::<T>(&self.value).cast::<u8>(), data, size);
            ptr::copy_nonoverlapping(self.tail.as_ptr(), data.add(size), self.tail.len());
        }
        ptr
    }
}

impl<T> Eq for PgVarlena<T> where T: Eq + Copy + Sized {}
impl<T> PartialEq for PgVarlena<T>
where
    T: PartialEq + Copy + Sized,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T> Ord for PgVarlena<T>
where
    T: Ord + Copy + Sized,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl<T> PartialOrd for PgVarlena<T>
where
    T: Ord + Copy + Sized,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.as_ref().cmp(other.as_ref()))
    }
}

impl<T> Hash for PgVarlena<T>
where
    T: Hash + Copy + Sized,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<T> Deref for PgVarlena<T>
where
    T: Copy + Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for PgVarlena<T>
where
    T: Copy + Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> AsRef<T> for PgVarlena<T>
where
    T: Copy + Sized,
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for PgVarlena<T>
where
    T: Copy + Sized,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Default for PgVarlena<T>
where
    T: Default + Copy,
{
    /// Create a `PgVarlena` holding `T::default()`, as [`PgVarlena::new`] does. Its padding
    /// bytes are unspecified; see [Padding](PgVarlena#padding).
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<T> for PgVarlena<T>
where
    T: Copy + Sized,
{
    /// Wrap `value`. A typed `T` carries only its fields, so the padding bytes stored for it
    /// are unspecified; see [Padding](PgVarlena#padding).
    fn from(value: T) -> Self {
        PgVarlena { value: Box::new(value), tail: Box::default() }
    }
}

impl<T> From<PgVarlena<T>> for Option<pg_sys::Datum>
where
    T: Copy + Sized,
{
    fn from(val: PgVarlena<T>) -> Self {
        Some(val.into_pg().into())
    }
}

impl<T> IntoDatum for PgVarlena<T>
where
    T: Copy + Sized,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.into_pg().into())
    }

    fn type_oid() -> pg_sys::Oid {
        rust_regtypein::<T>()
    }
}

// The default `from_datum_in_memory_context` suffices: the value copies its payload, so it
// does not depend on the memory context the Datum lives in.
impl<T> FromDatum for PgVarlena<T>
where
    T: Copy + Sized,
{
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<Self> {
        if is_null { None } else { Some(PgVarlena::<T>::from_datum(datum)) }
    }
}

#[doc(hidden)]
pub unsafe fn cbor_encode<T>(input: T) -> *const pg_sys::varlena
where
    T: Serialize,
{
    let mut serialized = StringInfo::new();

    serialized.push_bytes(&[0u8; pg_sys::VARHDRSZ as usize]); // reserve space for the header
    serde_cbor::to_writer(&mut serialized, &input).expect("failed to encode as CBOR");

    let size = serialized.len();
    let varlena = serialized.into_char_ptr();
    // SAFETY: StringInfo owns this aligned PostgreSQL allocation, reserves an
    // initialized header prefix, and bounds its total size by MaxAllocSize.
    // into_char_ptr transfers the buffer to its live memory context without
    // freeing it, so the header remains writable and its length fits C int.
    unsafe {
        pg_sys::SET_VARSIZE_4B!(varlena as *mut pg_sys::varlena, size as i32).get();
    }

    varlena as *const pg_sys::varlena
}

/// Decodes a value that [`cbor_encode`] wrote. Panics if the payload is not a CBOR `T`.
///
/// # Safety
///
/// `input` must point to a live varlena, which may be toasted. The returned value may borrow from
/// `input`, or from a detoasted copy allocated in `CurrentMemoryContext`, for `'de`, so both must
/// stay allocated for `'de`.
#[doc(hidden)]
pub unsafe fn cbor_decode<'de, T>(input: *mut pg_sys::varlena) -> T
where
    T: Deserialize<'de>,
{
    let varlena = pg_sys::pg_detoast_datum_packed(input);
    // SAFETY: the caller supplies a valid CBOR varlena; detoasting keeps the
    // complete packed allocation live for both header and payload accesses.
    let (len, data) = unsafe {
        (pg_sys::VARSIZE_ANY_EXHDR!(varlena).get() as usize, pg_sys::VARDATA_ANY!(varlena).get())
    };
    let slice = std::slice::from_raw_parts(data as *const u8, len);
    serde_cbor::from_slice(slice).expect("failed to decode CBOR")
}

/// [`cbor_decode`] for a `T` that borrows nothing from `input`.
///
/// # Safety
///
/// `input` must point to a live varlena, which may be toasted.
#[doc(hidden)]
pub unsafe fn cbor_decode_owned<T>(input: *mut pg_sys::varlena) -> T
where
    T: DeserializeOwned,
{
    // SAFETY: the caller passes a live varlena, and a `T` that borrows nothing cannot outlive it.
    unsafe { cbor_decode(input) }
}

#[doc(hidden)]
#[deprecated(since = "0.12.0", note = "just use the FromDatum impl")]
pub unsafe fn cbor_decode_into_context<'de, T>(
    mut memory_context: PgMemoryContexts,
    input: *mut pg_sys::varlena,
) -> T
where
    T: Deserialize<'de>,
{
    memory_context.switch_to(|_| {
        // this gets the varlena Datum copied into this memory context
        let varlena = pg_sys::pg_detoast_datum_copy(input);
        cbor_decode(varlena)
    })
}

unsafe impl<T> SqlTranslatable for PgVarlena<T>
where
    T: SqlTranslatable + Copy,
{
    const TYPE_IDENT: &'static str = T::TYPE_IDENT;
    const TYPE_ORIGIN: pgrx_sql_entity_graph::metadata::TypeOrigin = T::TYPE_ORIGIN;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = T::ARGUMENT_SQL;
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> = T::RETURN_SQL;
}

#[cfg(test)]
mod tests {
    use super::PgVarlena;
    use core::mem::size_of;
    use core::num::NonZeroU32;
    use core::{ptr, slice};

    #[test]
    fn constructors_hold_valid_values() {
        // A type whose all-zero bytes are not a valid value.
        assert_eq!(*PgVarlena::from(NonZeroU32::MIN), NonZeroU32::MIN);
        assert_eq!(*PgVarlena::<&str>::default(), "");
        assert_eq!(*PgVarlena::<(u8, i64)>::new(), (0, 0));
        // SAFETY: all-zero bytes are a valid [u32; 4].
        assert_eq!(*unsafe { PgVarlena::<[u32; 4]>::new_zeroed() }, [0; 4]);
    }

    /// Fill part of the stack with nonzero bytes, so that padding lost to a typed copy does
    /// not happen to read as zero.
    #[inline(never)]
    fn dirty_stack() {
        core::hint::black_box([0xa5_u8; 4096]);
    }

    /// A value started with `new_zeroed` keeps zero padding through field writes and moves, in
    /// the bytes that `into_pg` copies into the varlena.
    #[test]
    fn new_zeroed_keeps_padding_zero_through_field_writes() {
        /// One field byte, three padding bytes, then four field bytes. Rust may pass it by
        /// value as two scalars, which drops the padding bytes.
        #[derive(Copy, Clone)]
        #[repr(C)]
        struct Padded {
            a: u8,
            b: u32,
        }

        #[inline(never)]
        fn build() -> PgVarlena<Padded> {
            // SAFETY: all-zero bytes are a valid Padded.
            let mut v = unsafe { PgVarlena::<Padded>::new_zeroed() };
            v.a = 0xa1;
            v.b = 0xb2c3_d4e5;
            v
        }

        dirty_stack();
        let v = core::hint::black_box(build());
        // SAFETY: new_zeroed initialized every byte of the value, the field writes changed
        // only field bytes, and moving a PgVarlena does not copy the value as a Padded.
        let bytes = unsafe {
            slice::from_raw_parts(
                ptr::from_ref::<Padded>(&v.value).cast::<u8>(),
                size_of::<Padded>(),
            )
        };
        assert_eq!(bytes[0], 0xa1);
        assert_eq!(bytes[1..4], [0; 3], "padding bytes");
        assert_eq!(bytes[4..], 0xb2c3_d4e5_u32.to_ne_bytes());
    }
}
