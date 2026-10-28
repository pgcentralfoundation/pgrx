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
use crate::{FromDatum, IntoDatum, pg_sys};

/// The [`anyelement` polymorphic pseudo-type][anyelement].
///
/// An `AnyElement` holds a value and the type PostgreSQL resolved for it. Read the value with
/// [`AnyElement::value`], which checks that the Rust type can read that type.
///
/// The handle does not borrow the memory that holds a pass-by-reference value: the arguments of
/// the call that received it, the array that [`AnyArray`](crate::AnyArray) iteration read it
/// from or that array's untoasted copy in `CurrentMemoryContext`, or the result of the SPI query
/// that returned it. That memory may be freed once the call returns, the memory context is reset,
/// or the SPI connection closes, so a handle from
/// [`Spi::get_one`](crate::spi::Spi::get_one) already refers to freed memory. `AnyElement` is
/// `Copy` and `'static`, and the compiler does not stop it, or a borrowed value read from it,
/// from outliving that memory. Use both only while that memory is live, and read into a type
/// that owns its data, such as `String`, to keep a value longer.
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
