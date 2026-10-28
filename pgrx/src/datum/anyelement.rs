//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use super::TryFromDatumError;
use crate::{FromDatum, IntoDatum, PgMemoryContexts, pg_sys};

/// The [`anyelement` polymorphic pseudo-type][anyelement].
///
/// An `AnyElement` holds a value and the type PostgreSQL resolved for it. Read the value with
/// [`AnyElement::value`], which checks that the Rust type can read that type.
///
/// The handle does not borrow the memory that holds a pass-by-reference value, which may be
/// freed once the call that received the handle returns. An element that
/// [`AnyArray`](crate::AnyArray) iteration yields points into the array that the `AnyArray`
/// holds, which may be a detoasted copy in the `CurrentMemoryContext` in which that handle was
/// made, freed when that context is reset. `AnyElement` is `Copy` and `'static`, and the compiler
/// does not stop it, or a borrowed value read from it, from outliving the memory it points into,
/// for example when it is kept for a later call. Use both only while that memory is live, and
/// read into a type that owns its data, such as `String`, to keep a value longer.
///
/// [SPI](crate::spi) frees a query's results when the connection closes, so it copies an
/// `AnyElement` that it returns into the parent of the memory context that is current when the
/// value is read. Inside [`Spi::connect`](crate::spi::Spi::connect), that parent is
/// `TopTransactionContext`, so the copy stays allocated until the transaction ends. To read many
/// values in one transaction, read them as an owned type such as `String`, which keeps no copy.
///
// rustdoc doesn't directly support a warning block: https://github.com/rust-lang/rust/issues/73935
/// **Warning**: Calling [`FromDatum::from_datum`] with this type will unconditionally panic. Call
/// [`FromDatum::from_polymorphic_datum`] with a type ID instead.
///
/// [anyelement]: https://www.postgresql.org/docs/current/extend-type-system.html#EXTEND-TYPES-POLYMORPHIC
#[derive(Debug, Clone, Copy)]
pub struct AnyElement {
    datum: pg_sys::Datum,
    typoid: pg_sys::Oid,
}

impl AnyElement {
    /// The value.
    pub fn datum(&self) -> pg_sys::Datum {
        self.datum
    }

    /// The value's type, or InvalidOid if PostgreSQL did not resolve it, as for a call through
    /// [`fn_call`](crate::fn_call::fn_call), which passes no expression tree, or for an element of
    /// an [`AnyArray`](crate::AnyArray) whose type is InvalidOid.
    pub fn oid(&self) -> pg_sys::Oid {
        self.typoid
    }

    /// Reads the value as `T` if `T` can read the value's type.
    ///
    /// This applies the type check of [`FromDatum::try_from_datum`], which trusts `T`'s
    /// [`IntoDatum::type_oid`] and [`IntoDatum::is_compatible_with`] to name only types that
    /// `T`'s [`FromDatum`] implementation can read. It accepts a domain over a type that `T`
    /// reads, and it accepts a composite type or an anonymous record as `PgHeapTuple`.
    ///
    /// A `T` that borrows, such as `&'a str` or `Array<'a, U>`, points into the value's memory or
    /// into a detoasted copy in `CurrentMemoryContext`. Do not keep it after that memory is freed;
    /// see [`AnyElement`].
    ///
    /// # Errors
    ///
    /// Returns [`TryFromDatumError::IncompatibleTypes`] if `T` cannot read the value's type,
    /// including when [`AnyElement::oid`] is InvalidOid.
    ///
    /// # Panics
    ///
    /// `Vec<U>` panics if the array contains a NULL; read `Vec<Option<U>>` or `Array<U>` instead.
    /// `&str` and `String` panic, in a database whose encoding is not UTF-8, if the text is not
    /// valid UTF-8 or, for most encodings, not ASCII. [`IntoDatum::type_oid`] can raise an ERROR
    /// for some `T`; see [`FromDatum::try_from_datum`].
    #[inline]
    pub fn value<T: FromDatum + IntoDatum>(&self) -> Result<Option<T>, TryFromDatumError> {
        // SAFETY: try_from_datum reads the datum only if `T` can read `self.typoid`, and it
        // rejects InvalidOid, for which the datum's type is unknown. For a valid `typoid`, the
        // caller of the unsafe FromDatum constructor established that the datum is a non-NULL
        // value of that type. That the datum's memory is still live is required but not checked;
        // the type documentation describes that gap.
        unsafe { T::try_from_datum(self.datum, false, self.typoid) }
    }

    /// Reads the value as `T` without checking that `T` can read the value's type.
    ///
    /// Prefer [`AnyElement::value`]. Call this as `AnyElement::into::<T>(&element)`, because
    /// `element.into()` on an owned `AnyElement` resolves to [`Into::into`].
    ///
    /// # Safety
    ///
    /// - `T` must be able to read a value of type [`AnyElement::oid`], or of the value's actual
    ///   type if that is InvalidOid. Reading an `int4` as `String` treats the integer as a
    ///   pointer.
    /// - The memory that holds a pass-by-reference value must still be live; see [`AnyElement`].
    /// - A returned `T` that borrows must not outlive that memory or `CurrentMemoryContext`.
    #[inline]
    pub unsafe fn into<T: FromDatum>(&self) -> Option<T> {
        // SAFETY: the caller ensures that `T` can read this live datum's type.
        unsafe { T::from_polymorphic_datum(self.datum, false, self.typoid) }
    }
}

impl FromDatum for AnyElement {
    const GET_TYPOID: bool = true;
    // `value` reads the Datum's memory, and `from_datum_in_memory_context` copies it.
    const POINTS_INTO_DATUM: bool = true;

    /// You should **never** call this function to make this type; it will unconditionally panic.
    /// For polymorphic types such as this one, you must use [`FromDatum::from_polymorphic_datum`]
    /// and pass a type ID.
    #[inline]
    unsafe fn from_datum(_datum: pg_sys::Datum, _is_null: bool) -> Option<AnyElement> {
        panic!(
            "Can't create a polymorphic type using from_datum, call FromDatum::from_polymorphic_datum instead"
        )
    }

    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<AnyElement> {
        if is_null { None } else { Some(AnyElement { datum, typoid }) }
    }

    /// Copies a pass-by-reference value into `memory_context`, detoasting a varlena, so that the
    /// handle stays valid after the memory that holds `datum` is freed, as SPI frees a query's
    /// results when its connection closes.
    ///
    /// Raises an ERROR if no type has the OID `typoid`, including InvalidOid, because the value's
    /// size is then unknown.
    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<AnyElement> {
        if is_null {
            return None;
        }
        let mut typlen = 0;
        let mut typbyval = false;
        // SAFETY: get_typlenbyval takes the OID by value and writes both out-parameters, or
        // raises an ERROR if no type has that OID.
        unsafe { pg_sys::get_typlenbyval(typoid, &mut typlen, &mut typbyval) };
        if typbyval || datum.is_null() {
            // The value is in the Datum itself, or there is nothing to copy.
            return Some(AnyElement { datum, typoid });
        }
        // SAFETY: the caller passes a live value of type `typoid` and a memory context to copy it
        // into. A pass-by-reference value is a pointer to `typlen` bytes, to a varlena if
        // `typlen` is -1, or to a C string if it is -2. pg_detoast_datum_copy and datumCopy palloc
        // the copy in the current context, which switch_to sets to `memory_context` and restores
        // when the closure returns.
        let datum = unsafe {
            memory_context.switch_to(|_| {
                if typlen == -1 {
                    pg_sys::Datum::from(pg_sys::pg_detoast_datum_copy(datum.cast_mut_ptr()))
                } else {
                    pg_sys::datumCopy(datum, false, typlen.into())
                }
            })
        };
        Some(AnyElement { datum, typoid })
    }
}

impl IntoDatum for AnyElement {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.datum)
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::ANYELEMENTOID
    }
}

crate::impl_sql_translatable!(AnyElement, "anyelement");
