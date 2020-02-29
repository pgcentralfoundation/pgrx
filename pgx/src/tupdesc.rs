//! Provides a safe wrapper around Postgres' `pg_sys::TupleDescData` struct
use crate::{pg_sys, void_mut_ptr, PgBox, PgRelation};

use std::ops::Deref;

pub struct PgTupleDesc<'a> {
    tupdesc: PgBox<pg_sys::TupleDescData>,
    parent: Option<&'a PgRelation>,
    need_release: bool,
    need_pfree: bool,
}

impl<'a> PgTupleDesc<'a> {
    /// Wrap a Postgres-provided `pg_sys::TupleDescData`.  It is assumed the provided TupleDesc
    /// is reference counted by Postgres.
    ///
    /// The wrapped TupleDesc will have its reference count decremented  when this `PgTupleDesc`
    /// instance is dropped.
    ///
    /// ## Safety
    ///
    /// This method is unsafe as we cannot validate that the provided `pg_sys::TupleDesc` is valid
    /// or requires reference counting.
    pub unsafe fn from_pg<'b>(ptr: pg_sys::TupleDesc) -> PgTupleDesc<'b> {
        PgTupleDesc {
            tupdesc: PgBox::from_pg(ptr),
            parent: None,
            need_release: true,
            need_pfree: false,
        }
    }

    /// Wrap a copy of a `pg_sys::TupleDesc`.  This form is not reference counted and the copy is
    /// allocated in the `CurrentMemoryContext`
    ///
    /// When this instance is dropped, the copied TupleDesc is `pfree()`'d
    pub fn from_pg_copy<'b>(ptr: pg_sys::TupleDesc) -> PgTupleDesc<'b> {
        PgTupleDesc {
            tupdesc: PgBox::from_pg(unsafe { pg_sys::CreateTupleDescCopyConstr(ptr) }),
            parent: None,
            need_release: false,
            need_pfree: true,
        }
    }

    /// Similar to `::from_pg_copy()`, but assumes the provided `TupleDesc` is already a copy.
    ///
    /// When this instance is dropped, the TupleDesc is `pfree()`'d
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use pgx::{pg_sys, PgTupleDesc};
    /// let typid = 42 as pg_sys::Oid;  // a valid pg_type "oid" value
    /// let typmod = 0; // it's corresponding typemod value
    /// let tupdesc = unsafe { PgTupleDesc::from_pg_is_copy(pg_sys::lookup_rowtype_tupdesc_copy(typid, typmod)) };
    ///
    /// // assert the tuple descriptor has 12 attributes
    /// assert_eq!(tupdesc.len(), 12);
    ///
    /// // the wrapped tupdesc pointer is pfree'd
    /// drop(tupdesc)
    /// ```
    ///
    /// ## Safety
    ///
    /// This method is unsafe as we cannot validate that the provided `pg_sys::TupleDesc` is valid
    /// or is actually a copy that requires a `pfree()` on Drop.
    pub unsafe fn from_pg_is_copy<'b>(ptr: pg_sys::TupleDesc) -> PgTupleDesc<'b> {
        PgTupleDesc {
            tupdesc: PgBox::from_pg(ptr),
            parent: None,
            need_release: false,
            need_pfree: true,
        }
    }

    /// wrap the `pg_sys::TupleDesc` contained by the specified `PgRelation`
    pub(crate) fn from_relation(parent: &PgRelation) -> PgTupleDesc {
        PgTupleDesc {
            tupdesc: PgBox::from_pg(parent.rd_att),
            parent: Some(parent),
            need_release: false,
            need_pfree: false,
        }
    }

    /// From which relation was this TupleDesc created, if any?
    pub fn parent(&self) -> Option<&PgRelation> {
        self.parent
    }

    /// What is the pg_type oid of this TupleDesc?
    pub fn oid(&self) -> pg_sys::Oid {
        self.tupdesc.tdtypeid
    }

    /// What is the typemod of this TupleDesc?
    pub fn typmod(&self) -> i32 {
        self.tupdesc.tdtypmod
    }

    /// How many attributes do we have?
    pub fn len(&self) -> usize {
        self.tupdesc.natts as usize
    }

    /// Get a numbered attribute.  Attribute numbers are zero-based
    pub fn get(&self, i: usize) -> Option<&pg_sys::FormData_pg_attribute> {
        if i >= self.len() {
            None
        } else {
            Some(tupdesc_get_attr(&self.tupdesc, i))
        }
    }

    /// Iterate over our attributes
    pub fn iter(&self) -> TupleDescIterator {
        TupleDescIterator {
            tupdesc: self,
            curr: 0,
        }
    }
}

impl<'a> Deref for PgTupleDesc<'a> {
    type Target = PgBox<pg_sys::TupleDescData>;

    fn deref(&self) -> &Self::Target {
        &self.tupdesc
    }
}

impl<'a> Drop for PgTupleDesc<'a> {
    fn drop(&mut self) {
        if self.need_release {
            unsafe { release_tupdesc(self.tupdesc.as_ptr()) }
        } else if self.need_pfree {
            unsafe { pg_sys::pfree(self.tupdesc.as_ptr() as void_mut_ptr) }
        }
    }
}

pub unsafe fn release_tupdesc(ptr: pg_sys::TupleDesc) {
    if (*ptr).tdrefcount >= 0 {
        pg_sys::DecrTupleDescRefCount(ptr)
    }
}

/// `attno` is 0-based
#[cfg(feature = "pg10")]
#[inline]
fn tupdesc_get_attr(
    tupdesc: &PgBox<pg_sys::TupleDescData>,
    attno: usize,
) -> &pg_sys::FormData_pg_attribute {
    let atts = unsafe { std::slice::from_raw_parts(tupdesc.attrs, tupdesc.natts as usize) };
    unsafe { atts[attno].as_ref().unwrap() }
}

/// `attno` is 0-based
#[cfg(any(feature = "pg11", feature = "pg12"))]
#[inline]
fn tupdesc_get_attr(
    tupdesc: &PgBox<pg_sys::TupleDescData>,
    attno: usize,
) -> &pg_sys::FormData_pg_attribute {
    let atts = unsafe { tupdesc.attrs.as_slice(tupdesc.natts as usize) };
    &atts[attno]
}

pub struct TupleDescIterator<'a> {
    tupdesc: &'a PgTupleDesc<'a>,
    curr: usize,
}

impl<'a> Iterator for TupleDescIterator<'a> {
    type Item = &'a pg_sys::FormData_pg_attribute;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.tupdesc.get(self.curr);
        self.curr += 1;
        result
    }
}

pub struct TupleDescDataIntoIterator<'a> {
    tupdesc: PgTupleDesc<'a>,
    curr: usize,
}

impl<'a> IntoIterator for PgTupleDesc<'a> {
    type Item = pg_sys::FormData_pg_attribute;
    type IntoIter = TupleDescDataIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TupleDescDataIntoIterator {
            tupdesc: self,
            curr: 0,
        }
    }
}

impl<'a> Iterator for TupleDescDataIntoIterator<'a> {
    type Item = pg_sys::FormData_pg_attribute;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.tupdesc.get(self.curr) {
            Some(result) => *result,
            None => {
                return None;
            }
        };
        self.curr += 1;
        Some(result)
    }
}
