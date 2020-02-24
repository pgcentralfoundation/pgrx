use crate::{name_data_to_str, pg_sys, FromDatum, IntoDatum, PgBox};
use std::ops::Deref;

pub struct PgRelation {
    boxed: PgBox<pg_sys::RelationData>,
    owned_by_pg: bool,
    lockmode: Option<pg_sys::LOCKMODE>,
}

impl PgRelation {
    pub fn from_pg(ptr: pg_sys::Relation) -> PgRelation {
        PgRelation {
            boxed: PgBox::from_pg(ptr),
            owned_by_pg: true,
            lockmode: None,
        }
    }

    pub fn open(oid: pg_sys::Oid) -> PgRelation {
        unsafe {
            PgRelation {
                boxed: PgBox::from_pg(pg_sys::RelationIdGetRelation(oid)),
                owned_by_pg: false,
                lockmode: None,
            }
        }
    }

    pub fn with_lock(oid: pg_sys::Oid, lockmode: pg_sys::LOCKMODE) -> PgRelation {
        unsafe {
            PgRelation {
                boxed: PgBox::from_pg(pg_sys::relation_open(oid, lockmode)),
                owned_by_pg: false,
                lockmode: Some(lockmode),
            }
        }
    }

    /// RelationGetRelationName
    ///		Returns the rel's name.
    ///
    /// Note that the name is only unique within the containing namespace.
    pub fn name(&self) -> &str {
        let rd_rel: PgBox<pg_sys::FormData_pg_class> = PgBox::from_pg(self.boxed.rd_rel);
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
    pub fn get_heap_relation(&self) -> Option<PgRelation> {
        let rd_index: PgBox<pg_sys::FormData_pg_index> = PgBox::from_pg(self.boxed.rd_index);
        if rd_index.is_null() {
            None
        } else {
            Some(PgRelation::open(rd_index.indrelid))
        }
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
        if !self.owned_by_pg {
            match self.lockmode {
                None => unsafe { pg_sys::RelationClose(self.boxed.as_ptr()) },
                Some(lockmode) => unsafe { pg_sys::relation_close(self.boxed.as_ptr(), lockmode) },
            }
        }
    }
}
