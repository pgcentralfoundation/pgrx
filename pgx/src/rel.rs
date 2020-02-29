//! Provides a safe wrapper around Postgres' `pg_sys::RelationData` struct
use crate::{name_data_to_str, pg_sys, FromDatum, IntoDatum, PgBox, PgTupleDesc};
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
    pub unsafe fn from_pg(ptr: pg_sys::Relation) -> PgRelation {
        PgRelation {
            boxed: PgBox::from_pg(ptr),
            need_close: false,
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
    pub unsafe fn open(oid: pg_sys::Oid) -> PgRelation {
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
    pub fn with_lock(oid: pg_sys::Oid, lockmode: pg_sys::LOCKMODE) -> PgRelation {
        unsafe {
            PgRelation {
                boxed: PgBox::from_pg(pg_sys::relation_open(oid, lockmode)),
                need_close: true,
                lockmode: Some(lockmode),
            }
        }
    }

    /// RelationGetRelationName
    ///		Returns the rel's name.
    ///
    /// Note that the name is only unique within the containing namespace.
    pub fn name(&self) -> &str {
        let rd_rel = unsafe { self.boxed.rd_rel.as_ref() }.unwrap();
        name_data_to_str(&rd_rel.relname)
    }

    /// RelationGetRelid
    ///	    Returns the OID of the relation
    pub fn oid(&self) -> pg_sys::Oid {
        let rel = &self.boxed;
        rel.rd_id
    }

    /// RelationGetNamespace
    ///		Returns the rel's namespace OID.
    pub fn namespace_oid(&self) -> pg_sys::Oid {
        let rd_rel: PgBox<pg_sys::FormData_pg_class> = PgBox::from_pg(self.boxed.rd_rel);
        rd_rel.relnamespace
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
}

impl Clone for PgRelation {
    /// Same as calling `PgRelation::open()` on the underlying relation id
    fn clone(&self) -> Self {
        unsafe { PgRelation::open(self.rd_id) }
    }
}

impl FromDatum<PgRelation> for PgRelation {
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _typoid: u32) -> Option<PgRelation> {
        if is_null {
            None
        } else {
            Some(PgRelation::open(datum as pg_sys::Oid))
        }
    }
}

impl IntoDatum<PgRelation> for PgRelation {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.oid() as pg_sys::Datum)
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
        if self.need_close {
            match self.lockmode {
                None => unsafe { pg_sys::RelationClose(self.boxed.as_ptr()) },
                Some(lockmode) => unsafe { pg_sys::relation_close(self.boxed.as_ptr(), lockmode) },
            }
        }
    }
}
