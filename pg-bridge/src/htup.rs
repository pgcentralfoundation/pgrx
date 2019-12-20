use crate::pg_sys;

#[inline]
pub fn heap_tuple_header_get_datum_length(htup_header: pg_sys::HeapTupleHeader) -> usize {
    crate::varlena::varsize(htup_header as *const pg_sys::varlena)
}
