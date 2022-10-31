/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Provides a safe wrapper around Postgres' `pg_sys::TupleDescData` struct
use crate::{pg_sys, void_mut_ptr, PgBox, PgRelation};

use pgx_pg_sys::AsPgCStr;
use std::ops::Deref;

/// This struct is passed around within the backend to describe the structure
/// of tuples.  For tuples coming from on-disk relations, the information is
/// collected from the pg_attribute, pg_attrdef, and pg_constraint catalogs.
/// Transient row types (such as the result of a join query) have anonymous
/// TupleDesc structs that generally omit any constraint info; therefore the
/// structure is designed to let the constraints be omitted efficiently.
///
/// Note that only user attributes, not system attributes, are mentioned in
/// TupleDesc; with the exception that tdhasoid indicates if OID is present.
///
/// If the tupdesc is known to correspond to a named rowtype (such as a table's
/// rowtype) then tdtypeid identifies that type and tdtypmod is -1.  Otherwise
/// tdtypeid is RECORDOID, and tdtypmod can be either -1 for a fully anonymous
/// row type, or a value >= 0 to allow the rowtype to be looked up in the
/// typcache.c type cache.
///
/// Note that tdtypeid is never the OID of a domain over composite, even if
/// we are dealing with values that are known (at some higher level) to be of
/// a domain-over-composite type.  This is because tdtypeid/tdtypmod need to
/// match up with the type labeling of composite Datums, and those are never
/// explicitly marked as being of a domain type, either.
///
/// Tuple descriptors that live in caches (relcache or typcache, at present)
/// are reference-counted: they can be deleted when their reference count goes
/// to zero.  Tuple descriptors created by the executor need no reference
/// counting, however: they are simply created in the appropriate memory
/// context and go away when the context is freed.  We set the tdrefcount
/// field of such a descriptor to -1, while reference-counted descriptors
/// always have tdrefcount >= 0.
///
/// PGX's safe wrapper takes care of properly freeing or decrementing reference counts
pub struct PgTupleDesc<'a> {
    tupdesc: Option<PgBox<pg_sys::TupleDescData>>,
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
            tupdesc: Some(PgBox::from_pg(ptr)),
            parent: None,
            need_release: true,
            need_pfree: false,
        }
    }

    /// Wrap a Postgres-provided `pg_sys::TupleDescData`.  
    ///
    /// The wrapped TupleDesc will **not** have its reference count decremented  when this `PgTupleDesc`
    /// instance is dropped.
    ///
    /// ## Safety
    ///
    /// This method is unsafe as we cannot validate that the provided `pg_sys::TupleDesc` is valid
    pub unsafe fn from_pg_unchecked<'b>(ptr: pg_sys::TupleDesc) -> PgTupleDesc<'b> {
        PgTupleDesc {
            tupdesc: Some(PgBox::from_pg(ptr)),
            parent: None,
            need_release: false,
            need_pfree: false,
        }
    }

    /// Wrap a copy of a `pg_sys::TupleDesc`.  This form is not reference counted and the copy is
    /// allocated in the `CurrentMemoryContext`
    ///
    /// When this instance is dropped, the copied TupleDesc is `pfree()`'d
    ///
    /// ## Safety
    ///
    /// This method is unsafe as we cannot validate that the provided `pg_sys::TupleDesc` is valid
    /// or requires reference counting.
    pub unsafe fn from_pg_copy<'b>(ptr: pg_sys::TupleDesc) -> PgTupleDesc<'b> {
        PgTupleDesc {
            // SAFETY:  pg_sys::CreateTupleDescCopyConstr will be returning a valid pointer
            tupdesc: Some(PgBox::from_pg(pg_sys::CreateTupleDescCopyConstr(ptr))),
            parent: None,
            need_release: false,
            need_pfree: true,
        }
    }

    /// Similar to `::from_rust_copy()`, but assumes the provided `TupleDesc` is already a copy.
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
            tupdesc: Some(PgBox::from_pg(ptr)),
            parent: None,
            need_release: false,
            need_pfree: true,
        }
    }

    /// wrap the `pg_sys::TupleDesc` contained by the specified `PgRelation`
    pub fn from_relation(parent: &PgRelation) -> PgTupleDesc {
        PgTupleDesc {
            // SAFETY:  `parent` is a Rust reference, and as such its rd_att attribute will be property initialized
            tupdesc: Some(unsafe { PgBox::from_pg(parent.rd_att) }),
            parent: Some(parent),
            need_release: false,
            need_pfree: false,
        }
    }

    /** Retrieve the tuple description of the shape of a defined composite type

    ```rust,no_run
    use pgx::{prelude::*, PgTupleDesc};

    Spi::run("CREATE TYPE Dog AS (name text, age int);");
    let tuple_desc = PgTupleDesc::for_composite_type("Dog").unwrap();
    let natts = tuple_desc.len();

    unsafe {
        let mut is_null = (0..natts).map(|_| true).collect::<Vec<_>>();

        let heap_tuple_data =
            pg_sys::heap_form_tuple(tuple_desc.as_ptr(), std::ptr::null_mut(), is_null.as_mut_ptr());

        let heap_tuple = PgHeapTuple::from_heap_tuple(
            tuple_desc,
            heap_tuple_data,
        );
    }
    ```
    */
    pub fn for_composite_type(name: &str) -> Option<PgTupleDesc<'a>> {
        unsafe {
            let mut typoid = 0;
            let mut typmod = 0;
            pg_sys::parseTypeString(name.as_pg_cstr(), &mut typoid, &mut typmod, true);

            if typoid == pg_sys::InvalidOid {
                return None;
            }

            // It's important to make a copy of the tupledesc: https://www.postgresql.org/message-id/flat/24471.1136768659%40sss.pgh.pa.us
            let tuple_desc = pg_sys::lookup_rowtype_tupdesc_copy(typoid, typmod);

            Some(PgTupleDesc::from_pg_copy(tuple_desc))
        }
    }

    /// From which relation was this TupleDesc created, if any?
    pub fn parent(&self) -> Option<&PgRelation> {
        self.parent
    }

    /// What is the pg_type oid of this TupleDesc?
    pub fn oid(&self) -> pg_sys::Oid {
        self.tupdesc.as_ref().unwrap().tdtypeid
    }

    /// What is the typemod of this TupleDesc?
    pub fn typmod(&self) -> i32 {
        self.tupdesc.as_ref().unwrap().tdtypmod
    }

    /// How many attributes do we have?
    pub fn len(&self) -> usize {
        self.tupdesc.as_ref().unwrap().natts as usize
    }

    /// Do we have attributes?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a numbered attribute.  Attribute numbers are zero-based
    pub fn get(&self, i: usize) -> Option<&pg_sys::FormData_pg_attribute> {
        if i >= self.len() {
            None
        } else {
            Some(tupdesc_get_attr(self.tupdesc.as_ref().unwrap(), i))
        }
    }

    /// Iterate over our attributes
    pub fn iter(&self) -> TupleDescIterator {
        TupleDescIterator { tupdesc: self, curr: 0 }
    }

    /// Convert this [PgTupleDesc] into a pointer for passing into Postgres.  You are responsible
    /// for releasing or freeing the returned [pg_sys::TupleDescData] pointer.
    pub fn into_pg(mut self) -> *mut pg_sys::TupleDescData {
        self.tupdesc.take().unwrap().into_pg()
    }
}

impl<'a> Deref for PgTupleDesc<'a> {
    type Target = PgBox<pg_sys::TupleDescData>;

    fn deref(&self) -> &Self::Target {
        self.tupdesc.as_ref().unwrap()
    }
}

impl<'a> Drop for PgTupleDesc<'a> {
    fn drop(&mut self) {
        if self.tupdesc.is_some() {
            let tupdesc = self.tupdesc.take().unwrap();
            if self.need_release {
                unsafe { release_tupdesc(tupdesc.as_ptr()) }
            } else if self.need_pfree {
                unsafe { pg_sys::pfree(tupdesc.as_ptr() as void_mut_ptr) }
            }
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
#[cfg(any(
    feature = "pg11",
    feature = "pg12",
    feature = "pg13",
    feature = "pg14",
    feature = "pg15"
))]
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
        TupleDescDataIntoIterator { tupdesc: self, curr: 0 }
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
