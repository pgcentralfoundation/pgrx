use crate::{pg_sys, PgBox};

use std::ops::Deref;
#[cfg(feature = "pg10")]
pub use v10::*;
#[cfg(any(feature = "pg11", feature = "pg12"))]
pub use v11_v12::*;

pub struct PgTupleDesc(PgBox<pg_sys::TupleDescData>);

impl PgTupleDesc {
    #[inline]
    pub fn from_pg(ptr: *mut pg_sys::TupleDescData) -> PgTupleDesc {
        PgTupleDesc(PgBox::from_pg(ptr))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.natts as usize
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<&pg_sys::FormData_pg_attribute> {
        if i >= self.len() {
            None
        } else {
            Some(tupdesc_get_attr(&self.0, i))
        }
    }

    #[inline]
    pub fn iter(&self) -> TupleDescIterator {
        TupleDescIterator {
            tupdesc: self,
            curr: 0,
        }
    }
}

impl Deref for PgTupleDesc {
    type Target = PgBox<pg_sys::TupleDescData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for PgTupleDesc {
    fn drop(&mut self) {
        if self.0.tdrefcount >= 0 {
            crate::info!("releasing tupdesc on drop");
            unsafe {
                pg_sys::DecrTupleDescRefCount(self.0.as_ptr());
            }
        }
    }
}

/// [attno] is 1-based
#[inline]
pub fn tupdesc_get_typoid(tupdesc: &PgBox<pg_sys::TupleDescData>, attno: usize) -> pg_sys::Oid {
    tupdesc_get_attr(tupdesc, attno - 1).atttypid
}

/// [attno] is 1-based
#[inline]
pub fn tupdesc_get_typmod(tupdesc: &PgBox<pg_sys::TupleDescData>, attno: usize) -> i32 {
    tupdesc_get_attr(tupdesc, attno - 1).atttypmod
}

#[cfg(feature = "pg10")]
mod v10 {
    use crate::{pg_sys, PgBox};

    /// [attno] is 0-based
    #[inline]
    pub fn tupdesc_get_attr(
        tupdesc: &PgBox<pg_sys::TupleDescData>,
        attno: usize,
    ) -> &pg_sys::FormData_pg_attribute {
        let atts = unsafe { std::slice::from_raw_parts(tupdesc.attrs, tupdesc.natts as usize) };
        unsafe { atts[attno].as_ref().unwrap() }
    }
}

#[cfg(any(feature = "pg11", feature = "pg12"))]
mod v11_v12 {
    use crate::{pg_sys, PgBox};

    /// [attno] is 0-based
    #[allow(mutable_transmutes)]
    #[inline]
    pub fn tupdesc_get_attr(
        tupdesc: &PgBox<pg_sys::TupleDescData>,
        attno: usize,
    ) -> &pg_sys::FormData_pg_attribute {
        let atts = unsafe { tupdesc.attrs.as_slice(tupdesc.natts as usize) };
        &atts[attno]
    }
}

pub struct TupleDescIterator<'a> {
    tupdesc: &'a PgTupleDesc,
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

pub struct TupleDescDataIntoIterator {
    tupdesc: PgTupleDesc,
    curr: usize,
}

impl IntoIterator for PgTupleDesc {
    type Item = pg_sys::FormData_pg_attribute;
    type IntoIter = TupleDescDataIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        TupleDescDataIntoIterator {
            tupdesc: self,
            curr: 0,
        }
    }
}

impl Iterator for TupleDescDataIntoIterator {
    type Item = pg_sys::FormData_pg_attribute;

    #[inline]
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
