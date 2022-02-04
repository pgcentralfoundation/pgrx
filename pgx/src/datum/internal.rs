// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{pg_sys, FromDatum, IntoDatum, PgMemoryContexts};

/// Represents Postgres' `internal` data type, which is documented as:
///
///    The internal pseudo-type is used to declare functions that are meant only to be called
///    internally by the database system, and not by direct invocation in an SQL query. If a
///    function has at least one internal-type argument then it cannot be called from SQL. To
///    preserve the type safety of this restriction it is important to follow this coding rule: do
///    not create any function that is declared to return internal unless it has at least one
///    internal argument.
///
/// ## Implementation Notes
///
/// [Internal] is a wrapper around an `Option<pg_sys::Datum>`, which when retreived via
/// `::get/get_mut()` is cast to a pointer of `T`, returning the respective reference.
///
/// ## Safety
///
/// We make no guarantees about what the internal [pg_sys::Datum] actually points to in memory, so
/// it is your responsibility to ensure that what you're casting it to is really what it is.
#[derive(Default)]
pub struct Internal(Option<pg_sys::Datum>);

impl Internal {
    /// Construct a new Internal from any type.  
    ///
    /// The value will be dropped when the [PgMemoryContexts::CurrentMemoryContext] is deleted.
    #[inline]
    pub fn new<T>(t: T) -> Self {
        Self(Some(
            PgMemoryContexts::CurrentMemoryContext.leak_and_drop_on_delete(t) as pg_sys::Datum,
        ))
    }

    /// Returns true if the internal value is initialized. If false, this is a null pointer.
    #[inline]
    pub fn initialized(&self) -> bool {
        self.0.is_some()
    }

    /// Return a reference to the memory pointed to by this [`Internal`], as `Some(&T)`, unless the
    /// backing datum is null, then `None`.
    ///
    /// ## Safety
    ///
    /// We cannot guarantee that the contained datum points to memory that is really `T`.  This is
    /// your responsibility.
    #[inline]
    pub unsafe fn get<T>(&self) -> Option<&T> {
        self.0.and_then(|datum| (datum as *const T).as_ref())
    }

    /// Initializes the internal with `value`, then returns a mutable reference to it.
    ///
    /// If the Internal is already initialized with a value, the old value is dropped.
    ///
    /// See also [`Internal::get_or_insert`], which doesn’t update the value if already initialized.
    ///
    /// ## Safety
    ///
    /// We cannot guarantee that the contained datum points to memory that is really `T`.  This is
    /// your responsibility.
    #[inline]
    pub unsafe fn insert<T>(&mut self, value: T) -> &mut T {
        let datum =
            PgMemoryContexts::CurrentMemoryContext.leak_and_drop_on_delete(value) as pg_sys::Datum;
        let ptr = self.0.insert(datum);
        &mut *(*ptr as *mut T)
    }

    /// Return a reference to the memory pointed to by this [`Internal`], as `Some(&mut T)`, unless the
    /// backing datum is null, then `None`.
    ///
    /// ## Safety
    ///
    /// We cannot guarantee that the contained datum points to memory that is really `T`.  This is
    /// your responsibility.
    #[inline]
    pub unsafe fn get_mut<T>(&self) -> Option<&mut T> {
        self.0.and_then(|datum| (datum as *mut T).as_mut())
    }

    /// Initializes the internal with `value` if it is not initialized, then returns a mutable reference to
    /// the contained value.
    ///
    /// See also [`Internal::insert`], which updates the value even if the option already contains Some.
    ///
    /// ## Safety
    ///
    /// We cannot guarantee that the contained datum points to memory that is really `T`.  This is
    /// your responsibility.
    pub unsafe fn get_or_insert<T>(&mut self, value: T) -> &mut T {
        let ptr = self.0.get_or_insert({
            let datum = PgMemoryContexts::CurrentMemoryContext.leak_and_drop_on_delete(value)
                as pg_sys::Datum;
            datum
        });
        &mut *(*ptr as *mut T)
    }

    /// Initializes the internal with a default if it is not initialized, then returns a mutable reference
    /// to the contained value.
    ///
    /// See also [`Internal::insert`], which updates the value even if the option already contains Some.
    ///
    /// ## Safety
    ///
    /// We cannot guarantee that the contained datum points to memory that is really `T`.  This is
    /// your responsibility.
    pub unsafe fn get_or_insert_default<T>(&mut self) -> &mut T
    where
        T: Default,
    {
        let ptr = self.0.get_or_insert({
            let default = T::default();
            let datum = PgMemoryContexts::CurrentMemoryContext.leak_and_drop_on_delete(default)
                as pg_sys::Datum;
            datum
        });
        &mut *(*ptr as *mut T)
    }

    /// Inserts a value computed from `f` into the internal if it is `None`, then returns a mutable reference
    /// to the contained value.
    ///
    /// ## Safety
    ///
    /// We cannot guarantee that the contained datum points to memory that is really `T`.  This is
    /// your responsibility.
    pub unsafe fn get_or_insert_with<F, T>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        let ptr = self.0.get_or_insert_with(|| {
            let result = f();
            let datum = PgMemoryContexts::CurrentMemoryContext.leak_and_drop_on_delete(result)
                as pg_sys::Datum;
            datum
        });
        &mut *(*ptr as *mut T)
    }

    /// Returns the contained `Option<pg_sys::Datum>`
    #[inline]
    pub fn unwrap(self) -> Option<pg_sys::Datum> {
        self.0
    }
}

impl From<Option<pg_sys::Datum>> for Internal {
    #[inline]
    fn from(datum: Option<pg_sys::Datum>) -> Self {
        Internal(datum)
    }
}

impl FromDatum for Internal {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<Internal> {
        Some(Internal(if is_null { None } else { Some(datum) }))
    }
}

impl IntoDatum for Internal {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        self.0
    }

    #[inline]
    fn type_oid() -> pg_sys::Oid {
        pg_sys::INTERNALOID
    }
}
