// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{pg_sys, FromDatum, IntoDatum, PgMemoryContexts};
use std::num::NonZeroUsize;

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
/// [Internal] is simply a wrapper around a [pg_sys::Datum], which when retreived via `::get/get_mut()`
/// is simply cast to a pointer of `T`, returning the respective reference.
///
/// ## Safety
///
/// We make no guarantees about what the internal Datum actually points to in memory, so it is your
/// responsibility to ensure that what you're casting it to is really what it is.
#[repr(transparent)]
pub struct Internal(Option<NonZeroUsize>);

impl Internal {
    /// Construct a new Internal from any type.  The value will be dropped when the
    /// [PgMemoryContexts::CurrentMemoryContext] is deleted
    #[inline]
    pub fn new<T>(t: T) -> Self {
        Self(Some(unsafe {
            // SAFETY: `leak_and_drop_on_delete()` will always give us a non-zero (non-null) pointer
            NonZeroUsize::new_unchecked(
                PgMemoryContexts::CurrentMemoryContext.leak_and_drop_on_delete(t) as usize,
            )
        }))
    }

    /// Return a reference to the memory pointed to by this [Internal], cast as `T`.
    ///
    /// ## Safety
    ///
    /// We cannot guarantee that the contained datum points to memory that is really `T`.  This is
    /// your responsibility.
    #[inline]
    pub unsafe fn get<T>(&self) -> Option<&T> {
        match self.0 {
            None => None,
            Some(datum) => (datum.get() as *const T).as_ref(),
        }
    }

    /// Return a mutable reference to the memory pointed to by this [Internal], cast as `T`.
    ///
    /// ## Safety
    ///
    /// We cannot guarantee that the contained datum points to memory that is really `T`.  This is
    /// your responsibility.
    #[inline]
    pub unsafe fn get_mut<T>(&self) -> Option<&mut T> {
        match self.0 {
            None => None,
            Some(datum) => (datum.get() as *mut T).as_mut(),
        }
    }
}

impl FromDatum for Internal {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<Internal> {
        Some(Internal(if is_null {
            None
        } else {
            // SAFETY:  Postgres shouldn't be sending us a zero-value Datum when is_null is true
            assert!(datum != 0);
            Some(NonZeroUsize::new_unchecked(datum))
        }))
    }
}

impl IntoDatum for Internal {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        self.0.map(|datum| datum.get() as pg_sys::Datum)
    }

    #[inline]
    fn type_oid() -> pg_sys::Oid {
        pg_sys::INTERNALOID
    }
}
