//! pgrust: the handful of `htup` helpers the heap-tuple module uses, over the
//! shims in `pg_sys::pgrust::tuples` (the C-layout `htup` submodule needs
//! bindgen constants the pgrust target does not carry).

use std::num::NonZeroUsize;

use crate::pg_sys;

/// `heap_getattr` on a tuple, `None` for SQL NULL.
///
/// # Safety
/// `tuple` and `tupdesc` must be live C-layout tuple and descriptor images.
pub unsafe fn heap_getattr_raw(
    tuple: *mut pg_sys::HeapTupleData,
    attno: NonZeroUsize,
    tupdesc: pg_sys::TupleDesc,
) -> Option<pg_sys::Datum> {
    let mut is_null = false;
    let datum = unsafe { pg_sys::heap_getattr(tuple, attno.get() as _, tupdesc, &mut is_null) };
    if is_null { None } else { Some(datum) }
}

/// # Safety
/// `htup_header` must point at a composite tuple image.
pub unsafe fn heap_tuple_header_get_type_id(htup_header: pg_sys::HeapTupleHeader) -> pg_sys::Oid {
    unsafe { pg_sys::heap_tuple_header_get_type_id(htup_header) }
}

/// # Safety
/// `htup_header` must point at a composite tuple image.
pub unsafe fn heap_tuple_header_get_typmod(htup_header: pg_sys::HeapTupleHeader) -> i32 {
    unsafe { pg_sys::heap_tuple_header_get_typmod(htup_header) }
}

/// # Safety
/// `htup_header` must point at a composite tuple image.
pub fn heap_tuple_header_get_datum_length(htup_header: pg_sys::HeapTupleHeader) -> usize {
    unsafe { pg_sys::heap_tuple_header_get_datum_length(htup_header) as usize }
}

/// Relations are not exposed under pgrust; this stands in for the type so
/// `PgTupleDesc::parent` keeps its signature.
pub struct PgRelation {
    _private: (),
}

impl PgRelation {
    /// Relations cannot be opened under pgrust.
    pub fn oid(&self) -> pg_sys::Oid {
        pg_sys::InvalidOid
    }
}
