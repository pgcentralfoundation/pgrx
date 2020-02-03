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
        unsafe {
            atts[attno]
                .as_ref()
                .expect("found null FormData_pg_attribute")
        }
    }
}

#[cfg(any(feature = "pg11", feature = "pg12"))]
mod v11_v12 {
    use crate::{pg_sys, PgBox};
    use std::borrow::Borrow;

    /// [attno] is 0-based
    #[inline]
    pub fn tupdesc_get_attr(
        tupdesc: &PgBox<pg_sys::TupleDescData>,
        attno: usize,
    ) -> &pg_sys::FormData_pg_attribute {
        let atts = unsafe { tupdesc.attrs.as_slice(tupdesc.natts as usize) };

        atts[attno].borrow()
    }
}
