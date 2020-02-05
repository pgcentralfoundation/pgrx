use crate::{pg_sys, PgBox};

#[cfg(feature = "pg10")]
pub use v10::*;
#[cfg(any(feature = "pg11", feature = "pg12"))]
pub use v11_v12::*;

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
        atts[attno]
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

impl PgBox<pg_sys::TupleDescData> {
    #[inline]
    pub fn len(&self) -> usize {
        self.natts as usize
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<&pg_sys::FormData_pg_attribute> {
        if i >= self.len() {
            None
        } else {
            Some(tupdesc_get_attr(self, i))
        }
    }

    #[inline]
    pub fn iter(&self) -> TupleDescIterator {
        TupleDescIterator {
            pgbox: self,
            curr: 0,
        }
    }
}

pub struct TupleDescIterator<'a> {
    pgbox: &'a PgBox<pg_sys::TupleDescData>,
    curr: usize,
}

impl<'a> Iterator for TupleDescIterator<'a> {
    type Item = &'a pg_sys::FormData_pg_attribute;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.pgbox.get(self.curr);
        self.curr += 1;
        result
    }
}

pub struct TupleDescDataIntoIterator {
    pgbox: PgBox<pg_sys::TupleDescData>,
    curr: usize,
}

impl IntoIterator for PgBox<pg_sys::TupleDescData> {
    type Item = pg_sys::FormData_pg_attribute;
    type IntoIter = TupleDescDataIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        TupleDescDataIntoIterator {
            pgbox: self,
            curr: 0,
        }
    }
}

impl Iterator for TupleDescDataIntoIterator {
    type Item = pg_sys::FormData_pg_attribute;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.pgbox.get(self.curr) {
            Some(result) => *result,
            None => {
                return None;
            }
        };
        self.curr += 1;
        Some(result)
    }
}
