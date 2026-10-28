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
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

/// Wraps a Postgres `varlena *`, presenting it as if it's a Rust type of a fixed size.
///
/// PostgreSQL does not align a varlena's payload for `T`: the payload follows a one- or
/// four-byte header, so it starts one or four bytes past an allocation, or wherever a tuple
/// placed the value. `PgVarlena` therefore copies the payload into an aligned Rust allocation
/// and lends out references to that copy only. Changes made through `DerefMut` or `AsMut` are
/// copied back into the varlena when it is handed to PostgreSQL by [`PgVarlena::into_pg`] or
/// [`IntoDatum`].
///
/// A `PgVarlena` created from a Postgres-provided `Datum` never writes to that Datum's memory.
/// Its first mutable borrow allocates a new varlena in `CurrentMemoryContext` instead.
///
/// Generally, `PgVarlena` is meant to be used in conjunction with pgrx's `PostgresType` derive macro
/// to provide transparent mapping of fixed-size Rust types as Postgres datums.
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
///         let mut result = PgVarlena::<MyType>::new();
///         result.a = f32::from_str(a.unwrap()).expect("a is not a valid f32");
///         result.b = f32::from_str(b.unwrap()).expect("b is not a valid f32");
///         result.c = i64::from_str(c.unwrap()).expect("c is not a valid i64");
///
///         result
///     }
///
///     fn output(&self, buffer: &mut pgrx::StringInfo) {
///         buffer.push_str(&format!("{},{},{}", self.a, self.b, self.c));
///     }
/// }
///
/// #[pg_extern]
/// fn do_a_thing(mut input: PgVarlena<MyType>) -> PgVarlena<MyType> {
///     input.c += 99;  // performs a copy-on-write
///     input
/// }
/// ```
pub struct PgVarlena<T>
where
    T: Copy + Sized,
{
    /// Aligned copy of the payload. Every `&T` and `&mut T` borrows this copy.
    value: Box<T>,
    /// The varlena that `into_pg` returns.
    varlena: NonNull<pg_sys::varlena>,
    /// This value allocated `varlena`, so it may write the payload and must free it.
    owned: bool,
}

impl<T> PgVarlena<T>
where
    T: Copy + Sized,
{
    /// Create a new `PgVarlena` representing a Rust type.  The backing varlena is allocated
    /// by Postgres and initially zero'd (using `pg_sys::palloc0`).  Unless `.into_pg()` is called,
    /// the Postgres-allocated memory will follow Rust's drop semantics.
    ///
    /// The initial value is all zero bytes, so `T` must accept all-zero bytes as a valid value.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use pgrx::prelude::*;
    ///
    /// #[derive(Copy, Clone)]
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
    pub fn new() -> Self {
        // SAFETY: the varlena payload starts out as these same zero bytes. `T` must accept
        // them, as documented above; nothing in the type system enforces it.
        let value = unsafe { Box::<T>::new_zeroed().assume_init() };
        PgVarlena { value, varlena: Self::alloc_varlena(), owned: true }
    }

    /// Allocate a zeroed varlena with room for a `T` in `CurrentMemoryContext`.
    fn alloc_varlena() -> NonNull<pg_sys::varlena> {
        let size_of = std::mem::size_of::<T>();

        // palloc0 raises an ERROR for any size above MaxAllocSize, so the header writes
        // below never see a length that overflows C `int`.
        // SAFETY: palloc0 returns zeroed storage, aligned for any varlena header, or raises
        // an ERROR instead of returning NULL.
        let ptr =
            unsafe { pg_sys::palloc0(pg_sys::VARHDRSZ as usize + size_of) as *mut pg_sys::varlena };

        // SAFETY: palloc0 provides live aligned storage including the writable header
        // prefix, with no competing borrow. The short branch checks that its encoded
        // size fits the one-byte header; the other branch writes the four-byte header.
        unsafe {
            if size_of + pg_sys::VARHDRSZ_SHORT as usize <= pg_sys::VARATT_SHORT_MAX as usize {
                // we can use the short header size
                pg_sys::SET_VARSIZE_SHORT!(ptr, (size_of + pg_sys::VARHDRSZ_SHORT as usize) as i32)
                    .get();
            } else {
                // gotta use the full 4-byte header
                pg_sys::SET_VARSIZE_4B!(ptr, (size_of + pg_sys::VARHDRSZ as usize) as i32).get();
            }
        }

        NonNull::new(ptr).expect("palloc0 returned NULL")
    }

    /// Construct a `PgVarlena` from a known-to-be-non-null `pg_sys::Datum`.  As
    /// `FromDatum for PgVarlena<T> where T: Copy + Sized` is already implemented, it is unlikely
    /// that this function will need to be called directly.
    ///
    /// The provided datum is automatically detoasted, including a packed one-byte header, and
    /// its payload is copied into an aligned `T`. If detoasting allocated a copy, the returned
    /// `PgVarlena` owns and frees it; otherwise it borrows `datum`.
    ///
    /// ## Safety
    ///
    /// `datum` must point to a live varlena, possibly packed, compressed, or external, whose
    /// detoasted payload starts with the bytes of a valid `T`, such as a value that
    /// `PgVarlena::<T>::into_pg` produced. Unless the returned value is mutated first,
    /// [`PgVarlena::into_pg`] hands back `datum` itself, which must still be live wherever that
    /// pointer is used.
    ///
    /// ## Panics
    ///
    /// Panics if the detoasted payload is shorter than `T`.
    pub unsafe fn from_datum(datum: pg_sys::Datum) -> Self {
        let ptr = pg_sys::pg_detoast_datum(datum.cast_mut_ptr());
        // SAFETY: the caller's valid varlena datum was detoasted into live storage with an
        // initialized four-byte header whose size describes the complete allocation.
        let (len, payload) = unsafe {
            (pg_sys::VARSIZE_ANY_EXHDR!(ptr).get() as usize, pg_sys::VARDATA_ANY!(ptr).get())
        };
        assert!(
            len >= std::mem::size_of::<T>(),
            "varlena payload of {len} bytes cannot hold a {}-byte value",
            std::mem::size_of::<T>()
        );

        let mut value = Box::<T>::new_uninit();
        // SAFETY: the payload has at least size_of::<T>() initialized bytes, checked above, and
        // the new Rust allocation cannot overlap it. A byte copy has no alignment requirement
        // and keeps the payload's padding bytes. The caller promises those bytes are a valid `T`.
        let value = unsafe {
            ptr::copy_nonoverlapping(
                payload.cast::<u8>(),
                value.as_mut_ptr().cast::<u8>(),
                std::mem::size_of::<T>(),
            );
            value.assume_init()
        };

        PgVarlena {
            value,
            varlena: NonNull::new(ptr).expect("pg_detoast_datum returned NULL"),
            // A detoasted copy is ours to free; otherwise we borrow the caller's datum.
            owned: ptr != datum.cast_mut_ptr(),
        }
    }

    /// Use when you need to pass the backing `*mut pg_sys::varlena` to a Postgres function.
    ///
    /// This method is also used by the `IntoDatum for PgVarlena<T> where T: Copy + Sized`
    /// implementation.
    pub fn into_pg(mut self) -> *mut pg_sys::varlena {
        let ptr = self.varlena.as_ptr();
        if self.owned {
            // SAFETY: this value allocated `ptr` (or owns its detoasted copy), whose payload has
            // room for a `T`, and nothing else references it. The untyped byte copy writes the
            // aligned value, which may have changed, to the unaligned payload.
            unsafe {
                ptr::copy_nonoverlapping(
                    ptr::from_ref::<T>(&self.value).cast::<u8>(),
                    pg_sys::VARDATA_ANY!(ptr).get().cast::<u8>(),
                    std::mem::size_of::<T>(),
                );
            }
            // the varlena now belongs to the caller; don't pfree it on drop
            self.owned = false;
        }
        ptr
    }
}

/// `pg_sys::pfree` a `PgVarlena` if we allocated it, instead of Postgres
impl<T> Drop for PgVarlena<T>
where
    T: Copy + Sized,
{
    fn drop(&mut self) {
        if self.owned {
            // SAFETY: this value allocated the varlena with palloc, or received it from
            // pg_detoast_datum, and has not handed it to the caller.
            unsafe { pg_sys::pfree(self.varlena.as_ptr().cast()) }
        }
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

impl<T> Default for PgVarlena<T>
where
    T: Default + Copy,
{
    fn default() -> Self {
        let mut ptr = Self::new();
        *ptr = T::default();
        ptr
    }
}

/// Does a copy-on-write if the backing varlena pointer is borrowed
impl<T> AsMut<T> for PgVarlena<T>
where
    T: Copy + Sized,
{
    fn as_mut(&mut self) -> &mut T {
        if !self.owned {
            // Never write through a borrowed Datum. `into_pg` fills the new payload.
            self.varlena = Self::alloc_varlena();
            self.owned = true;
        }
        &mut self.value
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

    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<Self> {
        if is_null {
            None
        } else {
            memory_context.switch_to(|_| {
                // this gets the varlena Datum copied into this memory context
                let detoasted = pg_sys::pg_detoast_datum_copy(datum.cast_mut_ptr());

                // and we need to unpack it (if necessary), which will decompress it too
                let varlena = pg_sys::pg_detoast_datum_packed(detoasted);

                // and now we return it as a &str
                Some(PgVarlena::<T>::from_datum(varlena.into()))
            })
        }
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
