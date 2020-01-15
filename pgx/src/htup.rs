use crate::*;

#[inline]
pub unsafe fn heap_tuple_header_get_datum_length(htup_header: pg_sys::HeapTupleHeader) -> usize {
    crate::varlena::varsize(htup_header as *const pg_sys::varlena)
}

extern "C" {
    fn pgx_heap_getattr(
        tuple: *const pg_sys::HeapTupleData,
        attnum: u32,
        tupdesc: pg_sys::TupleDesc,
        isnull: *mut bool,
    ) -> pg_sys::Datum;

}

/// [attno] is 1-based
#[inline]
pub unsafe fn heap_getattr<T: FromDatum<T>>(
    tuple: *const pg_sys::HeapTupleData,
    attno: u32,
    tupdesc: pg_sys::TupleDesc,
) -> Option<T> {
    let mut is_null = false;
    let datum = pgx_heap_getattr(tuple, attno as u32, tupdesc, &mut is_null);
    let typoid = tupdesc_get_typoid(tupdesc, attno);

    if is_null {
        None
    } else {
        T::from_datum(datum, false, typoid)
    }
}

#[derive(Debug, Clone)]
pub struct DatumWithTypeInfo {
    datum: pg_sys::Datum,
    is_null: bool,
    typoid: pg_sys::Oid,
    typlen: i16,
    typbyval: bool,
}

/// [attno] is 1-based
#[inline]
pub fn heap_getattr_datum_ex(
    tuple: *const pg_sys::HeapTupleData,
    attno: u32,
    tupdesc: pg_sys::TupleDesc,
) -> DatumWithTypeInfo {
    let mut is_null = false;
    let datum = unsafe { pgx_heap_getattr(tuple, attno as u32, tupdesc, &mut is_null) };
    let typoid = unsafe { tupdesc_get_typoid(tupdesc, attno) };

    let mut typlen = 0;
    let mut typbyval = false;
    let mut typalign = 0 as std::os::raw::c_char; // unused

    unsafe {
        pg_sys::get_typlenbyvalalign(typoid, &mut typlen, &mut typbyval, &mut typalign);
    }

    DatumWithTypeInfo {
        datum,
        is_null,
        typoid,
        typlen,
        typbyval,
    }
}
