//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Provides a safe wrapper around Postgres' `pg_sys::RelationData` struct
use crate::{FromDatum, IntoDatum, PgTupleDesc, direct_function_call, name_data_to_str, pg_sys};
use core::ffi::CStr;
use core::ptr::NonNull;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, ReturnsError, ReturnsRef, SqlMappingRef, SqlTranslatable,
};
use std::os::raw::c_char;

/// A relation open in the relcache.
///
/// While a relation is open, PostgreSQL rebuilds its relcache entry in place whenever it processes
/// an invalidation for it, overwriting the `RelationData` and the `pg_class` row in `rd_rel`. That
/// can happen inside any call into PostgreSQL, including a statement this backend runs on the open
/// relation, such as `ALTER TABLE ... RENAME`. So `PgRelation` returns copies of what it reads,
/// never references into the entry.
pub struct PgRelation {
    ptr: NonNull<pg_sys::RelationData>,
    need_close: bool,
    lockmode: Option<pg_sys::LOCKMODE>,
}

impl PgRelation {
    /// Wrap a Postgres-provided `pg_sys::Relation`.
    ///
    /// It is assumed that Postgres will later `RelationClose()` the provided relation pointer.
    /// As such, it is not closed when this instance is dropped
    ///
    /// ## Safety
    ///
    /// `ptr` must be a relation that Postgres keeps open for as long as this instance exists.
    ///
    /// ## Panics
    ///
    /// Panics if `ptr` is NULL.
    pub unsafe fn from_pg(ptr: pg_sys::Relation) -> Self {
        PgRelation::new(ptr, false, None)
    }

    /// Wrap a Postgres-provided `pg_sys::Relation`.
    ///
    /// The provided `Relation` will be closed via `pg_sys::RelationClose` when this instance is dropped
    ///
    /// ## Safety
    ///
    /// `ptr` must be an open relation whose relcache reference this instance takes over, such as
    /// one returned by `pg_sys::RelationIdGetRelation`.
    ///
    /// ## Panics
    ///
    /// Panics if `ptr` is NULL.
    pub unsafe fn from_pg_owned(ptr: pg_sys::Relation) -> Self {
        PgRelation::new(ptr, true, None)
    }

    fn new(ptr: pg_sys::Relation, need_close: bool, lockmode: Option<pg_sys::LOCKMODE>) -> Self {
        let ptr = NonNull::new(ptr).expect("relation pointer is NULL");
        PgRelation { ptr, need_close, lockmode }
    }

    /// Given a relation oid, use `pg_sys::RelationIdGetRelation()` to open the relation
    ///
    /// If the specified relation oid was recently deleted, this function will panic.
    ///
    /// Additionally, the relation is closed via `pg_sys::RelationClose()` when this instance is
    /// dropped.
    ///
    /// ## Safety
    ///
    /// The caller should already have at least AccessShareLock on the relation ID, else there are
    /// nasty race conditions.
    ///
    /// As such, this function is unsafe as we cannot guarantee that this requirement is true.
    pub unsafe fn open(oid: pg_sys::Oid) -> Self {
        let rel = pg_sys::RelationIdGetRelation(oid);
        if rel.is_null() {
            // relation was recently deleted
            panic!("Cannot open relation with oid={oid:?}");
        }

        PgRelation::new(rel, true, None)
    }

    /// relation_open - open any relation by relation OID
    ///
    /// If lockmode is not "NoLock", the specified kind of lock is
    /// obtained on the relation.
    ///
    /// An error is raised if the relation does not exist.
    ///
    /// # Safety
    ///
    /// Using an inappropriate lockmode, such as using NoLock on a relation
    /// that has not been previously locked, may result in undefined behavior.
    ///
    /// NB: a "relation" is anything with a pg_class entry.  The caller is
    /// expected to check whether the relkind is something it can handle.
    ///
    /// The opened relation is automatically closed via `pg_sys::relation_close()`
    /// when this instance is dropped
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe fn with_lock(oid: pg_sys::Oid, lockmode: pg_sys::LOCKMODE) -> Self {
        // SAFETY: the caller passes a `lockmode` that is enough for how the relation is used,
        // and NoLock only for a relation it has already locked. relation_open raises an ERROR if
        // the relation does not exist. Otherwise it returns the relation open with one relcache
        // reference and, unless `lockmode` is NoLock, that lock taken. This value owns both, and
        // Drop releases them with relation_close.
        PgRelation::new(unsafe { pg_sys::relation_open(oid, lockmode) }, true, Some(lockmode))
    }

    /// Given a relation name, use `pg_sys::to_regclass` to look up its oid, and then
    /// `pg_sys::RelationIdGetRelation()` to open the relation.
    ///
    /// If the specified relation name is not found, we return an `Err(&str)`.
    ///
    /// If the specified relation was recently deleted, this function will panic.
    ///
    /// Additionally, the relation is closed via `pg_sys::RelationClose()` when this instance is
    /// dropped.
    ///
    /// ## Safety
    ///
    /// The caller should already have at least AccessShareLock on the relation ID, else there are
    /// nasty race conditions.
    ///
    /// As such, this function is unsafe as we cannot guarantee that this requirement is true.
    pub unsafe fn open_with_name(relname: &str) -> std::result::Result<Self, &'static str> {
        match direct_function_call::<pg_sys::Oid>(pg_sys::to_regclass, &[relname.into_datum()]) {
            Some(oid) => Ok(PgRelation::open(oid)),
            None => Err("no such relation"),
        }
    }

    /// Given a relation name, use `pg_sys::to_regclass` to look up its oid, and then
    /// open it with an AccessShareLock
    ///
    /// If the specified relation name is not found, we return an `Err(&str)`.
    ///
    /// If the specified relation was recently deleted, this function will panic.
    ///
    /// Additionally, the relation is closed via `pg_sys::RelationClose()` when this instance is
    /// dropped.
    pub fn open_with_name_and_share_lock(relname: &str) -> std::result::Result<Self, &'static str> {
        unsafe {
            match direct_function_call::<pg_sys::Oid>(pg_sys::to_regclass, &[relname.into_datum()])
            {
                Some(oid) => {
                    Ok(PgRelation::with_lock(oid, pg_sys::AccessShareLock as pg_sys::LOCKMODE))
                }
                None => Err("no such relation"),
            }
        }
    }

    /// The relcache entry, for passing to PostgreSQL functions.
    ///
    /// PostgreSQL can rewrite what this points to, as described on [`PgRelation`]. Copy fields
    /// out of it rather than holding references to them across calls into PostgreSQL.
    #[inline]
    pub fn as_ptr(&self) -> pg_sys::Relation {
        self.ptr.as_ptr()
    }

    /// RelationGetRelationName
    ///            Returns a copy of the rel's name.
    ///
    /// Note that the name is only unique within the containing namespace.
    pub fn name(&self) -> String {
        // SAFETY: class_form points to this open relation's pg_class row; relname is copied
        // out before PostgreSQL can rewrite it.
        let relname = unsafe { (*self.class_form()).relname };
        name_data_to_str(&relname).to_owned()
    }

    /// RelationGetRelid
    ///          Returns the OID of the relation
    #[inline]
    pub fn oid(&self) -> pg_sys::Oid {
        // SAFETY: the relation stays open and initialized for this access.
        pg_sys::Oid::from(unsafe { pg_sys::RelationGetRelid!(self.as_ptr()).get() })
    }

    /// RelationGetNamespace
    ///            Returns the rel's namespace OID.
    pub fn namespace_oid(&self) -> pg_sys::Oid {
        // SAFETY: class_form points to this open relation's pg_class row.
        unsafe { (*self.class_form()).relnamespace }
    }

    /// What is the name of the namespace in which this relation is located?
    pub fn namespace(&self) -> String {
        let nspid = self.namespace_oid();
        // SAFETY: get_namespace_name accepts any OID. It returns NULL or a NUL-terminated copy
        // palloc'd in CurrentMemoryContext that nothing else references.
        let name = unsafe { pg_sys::get_namespace_name(nspid) };
        assert!(!name.is_null(), "namespace {nspid:?} does not exist");
        // SAFETY: name is live until the pfree, which comes after the copy.
        unsafe {
            let copy = CStr::from_ptr(name).to_str().map(str::to_owned);
            pg_sys::pfree(name.cast());
            copy
        }
        .expect("unable to convert namespace name to UTF8")
    }

    /// If this `PgRelation` represents an index, return the `PgRelation` for the heap
    /// relation to which it is attached
    ///
    /// # Safety
    ///
    /// This does not perform locking, even though Postgres requires it.
    /// Callers must first call `with_lock` before calling this.
    pub unsafe fn heap_relation(&self) -> Option<PgRelation> {
        // SAFETY: the relation is open, so its RelationData is live.
        let rd_index = unsafe { (*self.as_ptr()).rd_index };
        if rd_index.is_null() {
            return None;
        }
        // SAFETY: a non-NULL rd_index points to this open index's pg_index row.
        Some(unsafe { PgRelation::open((*rd_index).indrelid) })
    }

    /// Return an iterator of indices, as `PgRelation`s, attached to this relation
    #[cfg(feature = "cshim")]
    pub fn indices(
        &self,
        lockmode: pg_sys::LOCKMODE,
    ) -> impl std::iter::Iterator<Item = PgRelation> {
        use crate::PgList;
        // SAFETY: we know self.ptr is a valid pointer as we created it
        let list =
            unsafe { PgList::<pg_sys::Oid>::from_pg(pg_sys::RelationGetIndexList(self.as_ptr())) };

        list.iter_oid()
            .filter(|oid| *oid != pg_sys::Oid::INVALID)
            .map(|oid| unsafe { PgRelation::with_lock(oid, lockmode) })
            .collect::<Vec<PgRelation>>()
            .into_iter()
    }

    /// Returned a wrapped `PgTupleDesc`
    ///
    /// The returned `PgTupleDesc` is tied to the lifetime of this `PgRelation` instance.
    ///
    /// ```rust,no_run
    /// use pgrx::{PgRelation, pg_sys};
    /// # let example_pg_class_oid = |i| pg_sys::Oid::from(i);
    /// let oid = example_pg_class_oid(42); // a valid pg_class "oid" value
    /// let relation = unsafe { PgRelation::from_pg(pg_sys::RelationIdGetRelation(oid) ) };
    /// let tupdesc = relation.tuple_desc();
    ///
    /// // assert that the tuple descriptor has 12 attributes
    /// assert_eq!(tupdesc.len(), 12);
    /// ```
    pub fn tuple_desc(&self) -> PgTupleDesc<'_> {
        PgTupleDesc::from_relation(self)
    }

    /// Point at the cached pg_class row, which callers must copy fields out of rather than
    /// borrow; see [`PgRelation`].
    fn class_form(&self) -> *const pg_sys::FormData_pg_class {
        // SAFETY: the relation is open, so its RelationData is live.
        let form = unsafe { pg_sys::RelationGetForm!(self.as_ptr()).get() };
        assert!(!form.is_null(), "rd_rel is NULL");
        form
    }

    fn relkind(&self) -> c_char {
        // SAFETY: class_form points to this open relation's pg_class row.
        unsafe { (*self.class_form()).relkind }
    }

    /// Number of tuples in this relation (not always up-to-date)
    pub fn reltuples(&self) -> Option<f32> {
        // SAFETY: class_form points to this open relation's pg_class row.
        let reltuples = unsafe { (*self.class_form()).reltuples };

        if reltuples == 0f32 { None } else { Some(reltuples) }
    }

    pub fn is_table(&self) -> bool {
        self.relkind() == pg_sys::RELKIND_RELATION as c_char
    }

    pub fn is_matview(&self) -> bool {
        self.relkind() == pg_sys::RELKIND_MATVIEW as c_char
    }

    pub fn is_index(&self) -> bool {
        self.relkind() == pg_sys::RELKIND_INDEX as c_char
    }

    pub fn is_view(&self) -> bool {
        self.relkind() == pg_sys::RELKIND_VIEW as c_char
    }

    pub fn is_sequence(&self) -> bool {
        self.relkind() == pg_sys::RELKIND_SEQUENCE as c_char
    }

    pub fn is_composite_type(&self) -> bool {
        self.relkind() == pg_sys::RELKIND_COMPOSITE_TYPE as c_char
    }

    pub fn is_foreign_table(&self) -> bool {
        self.relkind() == pg_sys::RELKIND_FOREIGN_TABLE as c_char
    }

    pub fn is_partitioned_table(&self) -> bool {
        self.relkind() == pg_sys::RELKIND_PARTITIONED_TABLE as c_char
    }

    pub fn is_toast_value(&self) -> bool {
        self.relkind() == pg_sys::RELKIND_TOASTVALUE as c_char
    }

    /// ensures that the returned `PgRelation` is closed by Rust when it is dropped
    pub fn to_owned(mut self) -> Self {
        self.need_close = true;
        self
    }

    /// Record a heap scan in this relation's PostgreSQL statistics.
    pub fn count_heap_scan(&mut self) {
        // SAFETY: this initialized relation stays open while PostgreSQL updates its statistics.
        unsafe { pg_sys::pgstat_count_heap_scan!(self.as_ptr()) }
    }

    /// Record a heap getnext in this relation's PostgreSQL statistics.
    pub fn count_heap_getnext(&mut self) {
        // SAFETY: this initialized relation stays open while PostgreSQL updates its statistics.
        unsafe { pg_sys::pgstat_count_heap_getnext!(self.as_ptr()) }
    }

    /// Record a heap fetch in this relation's PostgreSQL statistics.
    pub fn count_heap_fetch(&mut self) {
        // SAFETY: this initialized relation stays open while PostgreSQL updates its statistics.
        unsafe { pg_sys::pgstat_count_heap_fetch!(self.as_ptr()) }
    }

    /// Record an index scan in this relation's PostgreSQL statistics.
    pub fn count_index_scan(&mut self) {
        // SAFETY: this initialized relation stays open while PostgreSQL updates its statistics.
        unsafe { pg_sys::pgstat_count_index_scan!(self.as_ptr()) }
    }

    /// Record a buffer read in this relation's PostgreSQL statistics.
    pub fn count_buffer_read(&mut self) {
        // SAFETY: this initialized relation stays open while PostgreSQL updates its statistics.
        unsafe { pg_sys::pgstat_count_buffer_read!(self.as_ptr()) }
    }

    /// Record a buffer hit in this relation's PostgreSQL statistics.
    pub fn count_buffer_hit(&mut self) {
        // SAFETY: this initialized relation stays open while PostgreSQL updates its statistics.
        unsafe { pg_sys::pgstat_count_buffer_hit!(self.as_ptr()) }
    }

    /// Add returned index tuples to this relation's PostgreSQL statistics.
    pub fn count_index_tuples(&mut self, n: i64) {
        // SAFETY: this initialized relation stays open while PostgreSQL updates its statistics.
        unsafe {
            pg_sys::pgstat_count_index_tuples!(
                self.as_ptr(),
                pg_sys::__pgrx_c_macros::CValue::<pg_sys::__pgrx_c_types::int64>::new(n as _)
            )
        }
    }
}

impl Clone for PgRelation {
    /// Same as calling `PgRelation::with_lock(AccessShareLock)` on the underlying relation id
    fn clone(&self) -> Self {
        unsafe { PgRelation::with_lock(self.oid(), pg_sys::AccessShareLock as pg_sys::LOCKMODE) }
    }
}

impl FromDatum for PgRelation {
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<PgRelation> {
        if is_null {
            None
        } else {
            // the `PgRelation` SQL type is `REGCLASS`, which is just an `OID`, so that's how
            // we'll get the value.
            let oid = pg_sys::Oid::from_datum(datum, false)?;
            Some(PgRelation::with_lock(oid, pg_sys::AccessShareLock as pg_sys::LOCKMODE))
        }
    }
}

impl IntoDatum for PgRelation {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self.oid()))
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::REGCLASSOID
    }
}

impl Drop for PgRelation {
    fn drop(&mut self) {
        if self.need_close {
            // SAFETY: `open`, `from_pg_owned` and `with_lock` set need_close, and each gives this
            // value one relcache reference to the open relation, which RelationClose releases.
            // `with_lock` also records in `lockmode` the lock it took, which relation_close
            // releases with the reference. Safe `to_owned` can set need_close on a relation
            // that this value does not own; that is a known soundness bug, because Drop then
            // releases a reference that belongs to someone else.
            match self.lockmode {
                None => unsafe { pg_sys::RelationClose(self.as_ptr()) },
                Some(lockmode) => unsafe { pg_sys::relation_close(self.as_ptr(), lockmode) },
            }
        }
    }
}

unsafe impl SqlTranslatable for PgRelation {
    const TYPE_IDENT: &'static str = crate::pgrx_resolved_type!(PgRelation);
    const TYPE_ORIGIN: pgrx_sql_entity_graph::metadata::TypeOrigin =
        pgrx_sql_entity_graph::metadata::TypeOrigin::External;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> =
        Ok(SqlMappingRef::literal("regclass"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("regclass")));
}
