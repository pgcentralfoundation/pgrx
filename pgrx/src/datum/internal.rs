//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::{FromDatum, IntoDatum, PgMemoryContexts, pg_sys};

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
/// [Internal] is a wrapper around an `Option<pg_sys::Datum>`, which when retrieved via
/// `::get/get_mut()` is cast to a pointer of `T`, returning the respective reference.
///
/// ## Safety
///
/// An `Internal` is an untyped pointer, and safe code can build one from any [`pg_sys::Datum`]
/// with `Internal::from`. The `unsafe` methods that return a reference to the pointee require
/// that, for as long as the reference is live:
///
/// - the datum points to a valid, properly aligned `T`;
/// - the pointee is not dropped or freed. [`Internal::new`], [`Internal::insert`], and the
///   `get_or_insert*` methods store a value that is dropped and freed when the memory context
///   that was current at the time is next reset or deleted;
/// - nothing writes to the pointee while a `&T` is live, and nothing else reads or writes it
///   while a `&mut T` is live, including through another `Internal` that holds the same pointer.
#[derive(Default)]
pub struct Internal(Option<pg_sys::Datum>);

impl Internal {
    /// Construct a new Internal from any type.
    ///
    /// The value will be dropped when the [PgMemoryContexts::CurrentMemoryContext] is next reset
    /// or deleted.
    #[inline(always)]
    pub fn new<T>(t: T) -> Self {
        Self(Some(pg_sys::Datum::from(
            PgMemoryContexts::CurrentMemoryContext.leak_and_drop_on_delete(t),
        )))
    }

    /// Returns true if this holds a non-null pointer. Returns false for SQL `NULL` and for a null
    /// pointer, which PostgreSQL passes for some `internal` arguments that are not SQL `NULL`.
    #[inline(always)]
    pub fn initialized(&self) -> bool {
        self.0.is_some_and(|datum| !datum.is_null())
    }

    /// Return a reference to the memory pointed to by this [`Internal`], as `Some(&T)`, unless the
    /// backing datum is null, then `None`.
    ///
    /// ## Safety
    ///
    /// A non-null datum must meet the [requirements](Internal#safety) for a `&T` for as long as
    /// the returned reference is live.
    #[inline(always)]
    pub unsafe fn get<T>(&self) -> Option<&T> {
        self.0.and_then(|datum| (datum.cast_mut_ptr::<T>() as *const T).as_ref())
    }

    /// Initializes the internal with `value`, then returns a mutable reference to it.
    ///
    /// This does not drop the previous pointee. A value that [`Internal::new`] or this method
    /// stored is dropped when the memory context that was current at the time is next reset or
    /// deleted.
    ///
    /// See also [`Internal::get_or_insert`], which doesn’t update the value if already initialized.
    ///
    /// ## Safety
    ///
    /// `value` is dropped when [PgMemoryContexts::CurrentMemoryContext] is next reset or deleted.
    /// The returned reference must not be used after that, and anything `value` borrows must stay
    /// valid until then.
    #[inline(always)]
    pub unsafe fn insert<T>(&mut self, value: T) -> &mut T {
        let datum = pg_sys::Datum::from(
            PgMemoryContexts::CurrentMemoryContext.leak_and_drop_on_delete(value),
        );
        let ptr = self.0.insert(datum);
        &mut *(ptr.cast_mut_ptr::<T>())
    }

    /// Return a reference to the memory pointed to by this [`Internal`], as `Some(&mut T)`, unless the
    /// backing datum is null, then `None`.
    ///
    /// ## Safety
    ///
    /// A non-null datum must meet the [requirements](Internal#safety) for a `&mut T` for as long
    /// as the returned reference is live.
    #[inline(always)]
    pub unsafe fn get_mut<T>(&mut self) -> Option<&mut T> {
        self.0.and_then(|datum| (datum.cast_mut_ptr::<T>()).as_mut())
    }

    /// Initializes the internal with `value` if it is not initialized, then returns a mutable reference to
    /// the contained value.
    ///
    /// See also [`Internal::insert`], which updates the value even if the option already contains Some.
    ///
    /// ## Safety
    ///
    /// See [`Internal::get_or_insert_with`].
    #[inline(always)]
    pub unsafe fn get_or_insert<T>(&mut self, value: T) -> &mut T {
        self.get_or_insert_with(|| value)
    }

    /// Initializes the internal with a default if it is not initialized, then returns a mutable reference
    /// to the contained value.
    ///
    /// See also [`Internal::insert`], which updates the value even if the option already contains Some.
    ///
    /// ## Safety
    ///
    /// See [`Internal::get_or_insert_with`].
    #[inline(always)]
    pub unsafe fn get_or_insert_default<T>(&mut self) -> &mut T
    where
        T: Default,
    {
        self.get_or_insert_with(|| T::default())
    }

    /// Inserts a value computed from `f` into the internal if it is not initialized, then returns a
    /// mutable reference to the contained value.
    ///
    /// ## Safety
    ///
    /// If this `Internal` is [initialized](Internal::initialized), its datum must meet the
    /// [requirements](Internal#safety) for a `&mut T` for as long as the returned reference is
    /// live. Otherwise the requirements of [`Internal::insert`] apply to the value from `f`.
    #[inline(always)]
    pub unsafe fn get_or_insert_with<F, T>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        match self.0 {
            // SAFETY: the caller ensures an initialized datum meets the requirements for `&mut T`.
            Some(datum) if !datum.is_null() => unsafe { &mut *datum.cast_mut_ptr::<T>() },
            // SAFETY: the caller meets `insert`'s requirements for the value from `f`.
            _ => unsafe { self.insert(f()) },
        }
    }

    /// Returns the contained `Option<pg_sys::Datum>`
    #[inline(always)]
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
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<Internal> {
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

crate::impl_sql_translatable!(Internal, "internal");
