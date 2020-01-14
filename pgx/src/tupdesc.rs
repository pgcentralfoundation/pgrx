#[cfg(feature = "pg10")]
pub use v10::*;
#[cfg(feature = "pg11")]
pub use v11::*;
#[cfg(feature = "pg12")]
pub use v12::*;

#[cfg(feature = "pg10")]
mod v10 {
    use crate::pg_sys;

    /// [attno] is 1-based
    #[inline]
    pub unsafe fn tupdesc_get_typoid(
        tupdesc: *const pg_sys::pg10_specific::tupleDesc,
        attno: u32,
    ) -> pg_sys::Oid {
        let tupdesc_ref = tupdesc.as_ref().unwrap();
        let atts = std::slice::from_raw_parts(tupdesc_ref.attrs, tupdesc_ref.natts as usize);

        atts[(attno - 1) as usize].as_ref().unwrap().atttypid
    }
}

#[cfg(feature = "pg11")]
mod v11 {
    use crate::pg_sys;

    /// [attno] is 1-based
    #[inline]
    pub unsafe fn tupdesc_get_typoid(
        tupdesc: *const pg_sys::pg11_specific::tupleDesc,
        attno: u32,
    ) -> pg_sys::Oid {
        let tupdesc_ref = tupdesc.as_ref().unwrap();
        let atts = tupdesc_ref.attrs.as_slice(tupdesc_ref.natts as usize);

        atts[(attno - 1) as usize].atttypid
    }
}

#[cfg(feature = "pg12")]
mod v12 {
    use crate::pg_sys;

    /// [attno] is 1-based
    #[inline]
    pub unsafe fn tupdesc_get_typoid(
        tupdesc: *const pg_sys::pg12_specific::TupleDescData,
        attno: u32,
    ) -> pg_sys::Oid {
        let tupdesc_ref = tupdesc.as_ref().unwrap();
        let atts = tupdesc_ref.attrs.as_slice(tupdesc_ref.natts as usize);

        atts[(attno - 1) as usize].atttypid
    }
}
