//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use super::{ArrayIntoIterator, TryFromDatumError};
use crate::{AnyElement, Array, FromDatum, IntoDatum, PgMemoryContexts, pg_sys};
use core::ptr::NonNull;
use std::iter::FusedIterator;

/// The [`anyarray` polymorphic pseudo-type][anyarray].
///
/// An `AnyArray` holds an array value and the type PostgreSQL resolved for it. Read the value
/// with [`AnyArray::value`], which checks that the Rust type can read that type. Iterating over
/// `&AnyArray` yields each element as an [`AnyElement`], which points into the array or into an
/// untoasted copy of it that the iteration allocates in `CurrentMemoryContext`.
///
/// The handle does not borrow the memory that holds the array, which may be freed once the call
/// that received the handle returns. [SPI](crate::spi) copies an `AnyArray` that a query returns
/// out of the query's results, which it frees when the connection closes. `AnyArray` is `Copy`
/// and `'static`, and the compiler does not stop it, its elements, or a borrowed value read from
/// them, from outliving the memory they point into, for example when one is kept for a later
/// call. Use them only while that memory is live, and read into a type that owns its data, such
/// as `Vec<i32>` or `Vec<String>`, to keep a value longer.
///
// rustdoc doesn't directly support a warning block: https://github.com/rust-lang/rust/issues/73935
/// **Warning**: Calling [`FromDatum::from_datum`] with this type will unconditionally panic. Call
/// [`FromDatum::from_polymorphic_datum`] with a type ID instead.
///
/// [anyarray]: https://www.postgresql.org/docs/current/extend-type-system.html#EXTEND-TYPES-POLYMORPHIC
#[derive(Debug, Clone, Copy)]
pub struct AnyArray {
    datum: pg_sys::Datum,
    typoid: pg_sys::Oid,
}

impl AnyArray {
    /// The array value.
    pub fn datum(&self) -> pg_sys::Datum {
        self.datum
    }

    /// The array's type, or InvalidOid if PostgreSQL did not resolve it, as for a call through
    /// [`fn_call`](crate::fn_call::fn_call), which passes no expression tree.
    pub fn oid(&self) -> pg_sys::Oid {
        self.typoid
    }

    /// Reads the array as `T` if `T` can read the array's type.
    ///
    /// This applies the type check of [`FromDatum::try_from_datum`], which trusts `T`'s
    /// [`IntoDatum::type_oid`] and [`IntoDatum::is_compatible_with`] to name only types that
    /// `T`'s [`FromDatum`] implementation can read. It accepts a domain over a type that `T`
    /// reads, and it accepts an array of a composite type or of anonymous records as
    /// `Array<PgHeapTuple>`.
    ///
    /// A `T` that borrows, such as `Array<'a, U>`, points into the array's memory or into a
    /// detoasted copy in `CurrentMemoryContext`. Do not keep it after that memory is freed; see
    /// [`AnyArray`].
    ///
    /// # Errors
    ///
    /// Returns [`TryFromDatumError::IncompatibleTypes`] if `T` cannot read the array's type,
    /// including when [`AnyArray::oid`] is InvalidOid.
    ///
    /// # Panics
    ///
    /// `Vec<U>` panics if the array contains a NULL; read `Vec<Option<U>>` or `Array<U>` instead.
    /// [`IntoDatum::type_oid`] can raise an ERROR for some `T`; see
    /// [`FromDatum::try_from_datum`].
    #[inline]
    pub fn value<T: FromDatum + IntoDatum>(&self) -> Result<Option<T>, TryFromDatumError> {
        // SAFETY: try_from_datum reads the datum only if `T` can read `self.typoid`, and it
        // rejects InvalidOid, for which the datum's type is unknown. For a valid `typoid`, the
        // caller of the unsafe FromDatum constructor established that the datum is a non-NULL
        // value of that type. That the datum's memory is still live is required but not checked;
        // the type documentation describes that gap.
        unsafe { T::try_from_datum(self.datum, false, self.typoid) }
    }

    /// Reads the array as `T` without checking that `T` can read the array's type.
    ///
    /// Prefer [`AnyArray::value`]. Call this as `AnyArray::into::<T>(&array)`, because
    /// `array.into()` on an owned `AnyArray` resolves to [`Into::into`].
    ///
    /// # Safety
    ///
    /// - `T` must be able to read a value of type [`AnyArray::oid`], or of the array's actual type
    ///   if that is InvalidOid. `Array<U>` and `Vec<U>` take the element layout from the array
    ///   and read each element as `U`, so reading an `int4[]` as `Array<PgHeapTuple>` treats each
    ///   integer as a pointer.
    /// - The memory that holds the array must still be live; see [`AnyArray`].
    /// - A returned `T` that borrows must not outlive that memory or `CurrentMemoryContext`.
    #[inline]
    pub unsafe fn into<T: FromDatum>(&self) -> Option<T> {
        // SAFETY: the caller ensures that `T` can read this live datum's type.
        unsafe { T::from_polymorphic_datum(self.datum, false, self.typoid) }
    }
}

impl FromDatum for AnyArray {
    const GET_TYPOID: bool = true;

    /// You should **never** call this function to make this type; it will unconditionally panic.
    /// For polymorphic types such as this one, you must use [`FromDatum::from_polymorphic_datum`]
    /// and pass a type ID.
    #[inline]
    unsafe fn from_datum(_datum: pg_sys::Datum, _is_null: bool) -> Option<AnyArray> {
        panic!(
            "Can't create a polymorphic type using from_datum, call FromDatum::from_polymorphic_datum instead"
        )
    }

    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<AnyArray> {
        if is_null { None } else { Some(AnyArray { datum, typoid }) }
    }

    /// Copies the array into `memory_context`, detoasting it, so that the handle stays valid
    /// after the memory that holds `datum` is freed, as SPI frees a query's results when its
    /// connection closes.
    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<AnyArray> {
        if is_null {
            return None;
        }
        let datum = match NonNull::new(datum.cast_mut_ptr::<pg_sys::varlena>()) {
            // SAFETY: the caller passes a live array value and a memory context to copy it into.
            // pg_detoast_datum_copy pallocs an untoasted copy of the array in the current context,
            // which switch_to sets to `memory_context` and restores when the closure returns.
            Some(array) => unsafe {
                memory_context.switch_to(|_| {
                    pg_sys::Datum::from(pg_sys::pg_detoast_datum_copy(array.as_ptr()))
                })
            },
            // There is nothing to copy; reading the handle treats a null pointer as no array.
            None => datum,
        };
        Some(AnyArray { datum, typoid })
    }
}

impl IntoDatum for AnyArray {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.datum)
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::ANYARRAYOID
    }
}

crate::impl_sql_translatable!(AnyArray, "anyarray");

impl<'a> IntoIterator for &'a AnyArray
where
    Self: 'a,
{
    type Item = Option<AnyElement>;
    type IntoIter = AnyArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        // Each AnyElement points into the array's data and can outlive this iterator. An Array
        // frees a detoasted copy that it made itself when it drops, so detoast here instead: the
        // Array then reads the copy without owning it, and the copy stays allocated in
        // CurrentMemoryContext.
        let datum = match NonNull::new(self.datum.cast_mut_ptr::<pg_sys::varlena>()) {
            // SAFETY: the handle holds an array value, which must still be live; see the type
            // documentation. pg_detoast_datum returns an untoasted array with a 4-byte header
            // unchanged, and otherwise pallocs such a copy in CurrentMemoryContext.
            Some(array) => pg_sys::Datum::from(unsafe { pg_sys::pg_detoast_datum(array.as_ptr()) }),
            // Array reads a null pointer as no array, so the iterator yields nothing.
            None => self.datum,
        };
        // SAFETY: `datum` is the handle's array or an untoasted copy of it, so Array does not
        // copy it again, and Array takes the element layout from the array header.
        // get_element_type takes the OID by value and returns InvalidOid for an OID that names no
        // array type.
        unsafe {
            AnyArrayIterator {
                inner: Array::<pg_sys::Datum>::from_polymorphic_datum(datum, false, self.typoid)
                    .map(|a| a.into_iter()),
                typelem: pg_sys::get_element_type(self.typoid),
            }
        }
    }
}

pub struct AnyArrayIterator<'a> {
    inner: Option<ArrayIntoIterator<'a, pg_sys::Datum>>,
    typelem: pg_sys::Oid,
}

impl<'a> Iterator for AnyArrayIterator<'a> {
    type Item = Option<AnyElement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.as_mut().and_then(|i| {
            i.next().map(|d| match d {
                // SAFETY: `d` is a non-NULL element of the array, so its type is `typelem` unless
                // that is InvalidOid. A pass-by-reference element points into the array that
                // `into_iter` read, which does not belong to this iterator.
                Some(d) => unsafe { AnyElement::from_polymorphic_datum(d, false, self.typelem) },
                None => None,
            })
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            Some(inner) => inner.size_hint(),
            None => (0, Some(0)),
        }
    }
}

impl<'a> ExactSizeIterator for AnyArrayIterator<'a> {}
impl<'a> FusedIterator for AnyArrayIterator<'a> {}
