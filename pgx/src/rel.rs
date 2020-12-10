// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! Provides a safe wrapper around Postgres' `pg_sys::RelationData` struct
use crate::{
    direct_function_call, name_data_to_str, pg_sys, FromDatum, IntoDatum, PgBox, PgList,
    PgTupleDesc,
};
use std::ops::Deref;

pub struct PgRelation {
    boxed: PgBox<pg_sys::RelationData>,
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
    /// This method is unsafe as we cannot ensure that this relation will later be closed by Postgres
    pub unsafe fn from_pg(ptr: pg_sys::Relation) -> Self {
        PgRelation {
            boxed: PgBox::from_pg(ptr),
            need_close: false,
            lockmode: None,
        }
    }

    /// Wrap a Postgres-provided `pg_sys::Relation`.
    ///
    /// The provided `Relation` will be closed via `pg_sys::RelationClose` when this instance is dropped
    pub fn from_pg_owned(ptr: pg_sys::Relation) -> Self {
        PgRelation {
            boxed: PgBox::from_pg(ptr),
            need_close: true,
            lockmode: None,
        }
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
            panic!("Cannot open relation with oid={}", oid);
        }

        PgRelation {
            boxed: PgBox::from_pg(rel),
            need_close: true,
            lockmode: None,
        }
    }

    /// relation_open - open any relation by relation OID
    ///
    /// If lockmode is not "NoLock", the specified kind of lock is
    /// obtained on the relation.  (Generally, NoLock should only be
    /// used if the caller knows it has some appropriate lock on the
    /// relation already.)
    ///
    /// An error is raised if the relation does not exist.
    ///
    /// NB: a "relation" is anything with a pg_class entry.  The caller is
    /// expected to check whether the relkind is something it can handle.
    ///
    /// The opened relation is automatically closed via `pg_sys::relation_close()`
    /// when this instance is dropped
    pub fn with_lock(oid: pg_sys::Oid, lockmode: pg_sys::LOCKMODE) -> Self {
        unsafe {
            PgRelation {
                boxed: PgBox::from_pg(pg_sys::relation_open(oid, lockmode)),
                need_close: true,
                lockmode: Some(lockmode),
            }
        }
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
        match direct_function_call::<pg_sys::Oid>(pg_sys::to_regclass, vec![relname.into_datum()]) {
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
            match direct_function_call::<pg_sys::Oid>(
                pg_sys::to_regclass,
                vec![relname.into_datum()],
            ) {
                Some(oid) => Ok(PgRelation::with_lock(
                    oid,
                    pg_sys::AccessShareLock as pg_sys::LOCKMODE,
                )),
                None => Err("no such relation"),
            }
        }
    }

    /// RelationGetRelationName
    ///            Returns the rel's name.
    ///
    /// Note that the name is only unique within the containing namespace.
    pub fn name(&self) -> &str {
        let rd_rel = unsafe { self.boxed.rd_rel.as_ref() }.unwrap();
        name_data_to_str(&rd_rel.relname)
    }

    /// RelationGetRelid
    ///          Returns the OID of the relation
    #[inline]
    pub fn oid(&self) -> pg_sys::Oid {
        let rel = &self.boxed;
        rel.rd_id
    }

    /// RelationGetNamespace
    ///            Returns the rel's namespace OID.
    pub fn namespace_oid(&self) -> pg_sys::Oid {
        let rd_rel: PgBox<pg_sys::FormData_pg_class> = PgBox::from_pg(self.boxed.rd_rel);
        rd_rel.relnamespace
    }

    /// What is the name of the namespace in which this relation is located?
    pub fn namespace(&self) -> &str {
        unsafe { std::ffi::CStr::from_ptr(pg_sys::get_namespace_name(self.namespace_oid())) }
            .to_str()
            .expect("unable to convert namespace name to UTF8")
    }

    /// If this `PgRelation` represents an index, return the `PgRelation` for the heap
    /// relation to which it is attached
    pub fn heap_relation(&self) -> Option<PgRelation> {
        let rd_index: PgBox<pg_sys::FormData_pg_index> = PgBox::from_pg(self.boxed.rd_index);
        if rd_index.is_null() {
            None
        } else {
            Some(unsafe { PgRelation::open(rd_index.indrelid) })
        }
    }

    /// Return an iterator of indices, as `PgRelation`s, attached to this relation
    pub fn indicies(
        &self,
        lockmode: pg_sys::LOCKMODE,
    ) -> impl std::iter::Iterator<Item = PgRelation> {
        let list = PgList::<pg_sys::Oid>::from_pg(unsafe {
            pg_sys::RelationGetIndexList(self.boxed.as_ptr())
        });

        list.iter_oid()
            .filter(|oid| *oid != pg_sys::InvalidOid)
            .map(|oid| PgRelation::with_lock(oid, lockmode))
            .collect::<Vec<PgRelation>>()
            .into_iter()
    }

    /// Returned a wrapped `PgTupleDesc`
    ///
    /// The returned `PgTupleDesc` is tied to the lifetime of this `PgRelation` instance.
    ///
    /// ```rust,no_run
    /// use pgx::{PgRelation, pg_sys};
    /// let oid = 42;   // a valid pg_class "oid" value
    /// let relation = unsafe { PgRelation::from_pg(pg_sys::RelationIdGetRelation(oid) ) };
    /// let tupdesc = relation.tuple_desc();
    ///
    /// // assert that the tuple descriptor has 12 attributes
    /// assert_eq!(tupdesc.len(), 12);
    /// ```
    pub fn tuple_desc(&self) -> PgTupleDesc {
        PgTupleDesc::from_relation(&self)
    }

    /// Number of tuples in this relation (not always up-to-date)
    pub fn reltuples(&self) -> Option<f32> {
        let reltuples = unsafe { self.boxed.rd_rel.as_ref() }
            .expect("rd_rel is NULL")
            .reltuples;

        if reltuples == 0f32 {
            None
        } else {
            Some(reltuples)
        }
    }

    pub fn is_table(&self) -> bool {
        let rd_rel: &pg_sys::FormData_pg_class =
            unsafe { self.boxed.rd_rel.as_ref().expect("rd_rel is NULL") };
        rd_rel.relkind == pg_sys::RELKIND_RELATION as i8
    }

    pub fn is_matview(&self) -> bool {
        let rd_rel: &pg_sys::FormData_pg_class =
            unsafe { self.boxed.rd_rel.as_ref().expect("rd_rel is NULL") };
        rd_rel.relkind == pg_sys::RELKIND_MATVIEW as i8
    }

    pub fn is_index(&self) -> bool {
        let rd_rel: &pg_sys::FormData_pg_class =
            unsafe { self.boxed.rd_rel.as_ref().expect("rd_rel is NULL") };
        rd_rel.relkind == pg_sys::RELKIND_INDEX as i8
    }

    pub fn is_view(&self) -> bool {
        let rd_rel: &pg_sys::FormData_pg_class =
            unsafe { self.boxed.rd_rel.as_ref().expect("rd_rel is NULL") };
        rd_rel.relkind == pg_sys::RELKIND_VIEW as i8
    }

    pub fn is_sequence(&self) -> bool {
        let rd_rel: &pg_sys::FormData_pg_class =
            unsafe { self.boxed.rd_rel.as_ref().expect("rd_rel is NULL") };
        rd_rel.relkind == pg_sys::RELKIND_SEQUENCE as i8
    }

    pub fn is_composite_type(&self) -> bool {
        let rd_rel: &pg_sys::FormData_pg_class =
            unsafe { self.boxed.rd_rel.as_ref().expect("rd_rel is NULL") };
        rd_rel.relkind == pg_sys::RELKIND_COMPOSITE_TYPE as i8
    }

    pub fn is_foreign_table(&self) -> bool {
        let rd_rel: &pg_sys::FormData_pg_class =
            unsafe { self.boxed.rd_rel.as_ref().expect("rd_rel is NULL") };
        rd_rel.relkind == pg_sys::RELKIND_FOREIGN_TABLE as i8
    }

    pub fn is_partitioned_table(&self) -> bool {
        let rd_rel: &pg_sys::FormData_pg_class =
            unsafe { self.boxed.rd_rel.as_ref().expect("rd_rel is NULL") };
        rd_rel.relkind == pg_sys::RELKIND_PARTITIONED_TABLE as i8
    }

    pub fn is_toast_value(&self) -> bool {
        let rd_rel: &pg_sys::FormData_pg_class =
            unsafe { self.boxed.rd_rel.as_ref().expect("rd_rel is NULL") };
        rd_rel.relkind == pg_sys::RELKIND_TOASTVALUE as i8
    }

    /// ensures that the returned `PgRelation` is closed by Rust when it is dropped
    pub fn to_owned(mut self) -> Self {
        self.need_close = true;
        self
    }
}

impl Clone for PgRelation {
    /// Same as calling `PgRelation::with_lock(AccessShareLock)` on the underlying relation id
    fn clone(&self) -> Self {
        PgRelation::with_lock(self.rd_id, pg_sys::AccessShareLock as pg_sys::LOCKMODE)
    }
}

impl FromDatum for PgRelation {
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _typoid: u32) -> Option<PgRelation> {
        if is_null {
            None
        } else {
            Some(PgRelation::with_lock(
                datum as pg_sys::Oid,
                pg_sys::AccessShareLock as pg_sys::LOCKMODE,
            ))
        }
    }
}

impl IntoDatum for PgRelation {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.oid() as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::REGCLASSOID
    }
}

impl Deref for PgRelation {
    type Target = PgBox<pg_sys::RelationData>;

    fn deref(&self) -> &Self::Target {
        &self.boxed
    }
}

impl Drop for PgRelation {
    fn drop(&mut self) {
        if !self.boxed.is_null() {
            if self.need_close {
                match self.lockmode {
                    None => unsafe { pg_sys::RelationClose(self.boxed.as_ptr()) },
                    Some(lockmode) => unsafe {
                        pg_sys::relation_close(self.boxed.as_ptr(), lockmode)
                    },
                }
            }
        }
    }
}
