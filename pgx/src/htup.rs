// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! Utility functions for working with `pg_sys::HeapTuple` and `pg_sys::HeapTupleHeader` structs
use crate::*;

/// Given a `pg_sys::Datum` representing a composite row type, return a boxed `HeapTupleData`,
/// which can be used by the various `heap_getattr` methods
///
/// ## Safety
///
/// This function is safe, but if the provided `HeapTupleHeader` is null, it will `panic!()`
#[inline]
pub fn composite_row_type_make_tuple(row: pg_sys::Datum) -> PgBox<pg_sys::HeapTupleData> {
    let htup_header = unsafe { pg_sys::pg_detoast_datum_packed(row as *mut pg_sys::varlena) }
        as pg_sys::HeapTupleHeader;
    let mut tuple = PgBox::<pg_sys::HeapTupleData>::alloc0();

    tuple.t_len = heap_tuple_header_get_datum_length(htup_header) as u32;
    tuple.t_data = htup_header;

    tuple
}

pub fn deconstruct_row_type<'a>(
    tupdesc: &'a PgTupleDesc,
    row: pg_sys::Datum,
) -> Array<'a, pg_sys::Datum> {
    extern "C" {
        fn pgx_deconstruct_row_type(
            tupdesc: pg_sys::TupleDesc,
            row: pg_sys::Datum,
            columns: *mut *mut pg_sys::Datum,
            nulls: *mut *mut bool,
        );
    }
    let mut columns = std::ptr::null_mut();
    let mut nulls = std::ptr::null_mut();
    unsafe {
        pgx_deconstruct_row_type(tupdesc.as_ptr(), row, &mut columns, &mut nulls);

        Array::over(columns, nulls, tupdesc.len())
    }
}

/// ## Safety
///
/// This function is safe, but if the provided `HeapTupleHeader` is null, it will `panic!()`
#[inline]
pub fn heap_tuple_header_get_datum_length(htup_header: pg_sys::HeapTupleHeader) -> usize {
    if htup_header.is_null() {
        panic!("Attempt to dereference a null HeapTupleHeader");
    }

    unsafe { crate::varlena::varsize(htup_header as *const pg_sys::varlena) }
}

/// convert a HeapTupleHeader to a Datum.
#[inline]
pub fn heap_tuple_get_datum(heap_tuple: pg_sys::HeapTuple) -> pg_sys::Datum {
    unsafe { pg_sys::HeapTupleHeaderGetDatum((*heap_tuple).t_data) }
}

/// ```c
/// #define HeapTupleHeaderGetTypeId(tup) \
/// ( \
/// (tup)->t_choice.t_datum.datum_typeid \
/// )
/// ```
#[inline]
pub unsafe fn heap_tuple_header_get_type_id(htup_header: pg_sys::HeapTupleHeader) -> pg_sys::Oid {
    htup_header.as_ref().unwrap().t_choice.t_datum.datum_typeid
}

/// ```c
/// #define HeapTupleHeaderGetTypMod(tup) \
/// ( \
/// (tup)->t_choice.t_datum.datum_typmod \
/// )
/// ```
#[inline]
pub unsafe fn heap_tuple_header_get_typmod(htup_header: pg_sys::HeapTupleHeader) -> i32 {
    htup_header.as_ref().unwrap().t_choice.t_datum.datum_typmod
}

extern "C" {
    fn pgx_heap_getattr(
        tuple: *const pg_sys::HeapTupleData,
        attnum: u32,
        tupdesc: pg_sys::TupleDesc,
        isnull: *mut bool,
    ) -> pg_sys::Datum;

}

/// Extract an attribute of a heap tuple and return it as a Datum.
/// This works for either system or user attributes.  The given `attnum`
/// is properly range-checked.
///
/// If the field in question has a NULL value, we return `None`.
/// Otherwise, a `Some(T)`
///
/// 'tup' is the pointer to the heap tuple.  'attnum' is the attribute
/// number of the column (field) caller wants.  'tupleDesc' is a
/// pointer to the structure describing the row and all its fields.
///
/// `attno` is 1-based
#[inline]
pub fn heap_getattr<T: FromDatum>(
    tuple: &PgBox<pg_sys::HeapTupleData>,
    attno: usize,
    tupdesc: &PgTupleDesc,
) -> Option<T> {
    let mut is_null = false;
    let datum =
        unsafe { pgx_heap_getattr(tuple.as_ptr(), attno as u32, tupdesc.as_ptr(), &mut is_null) };
    let typoid = tupdesc.get(attno - 1).expect("no attribute").type_oid();

    if is_null {
        None
    } else {
        unsafe { T::from_datum(datum, false, typoid.value()) }
    }
}

#[derive(Debug, Clone)]
pub struct DatumWithTypeInfo {
    pub datum: pg_sys::Datum,
    pub is_null: bool,
    pub typoid: PgOid,
    pub typlen: i16,
    pub typbyval: bool,
}

impl DatumWithTypeInfo {
    #[inline]
    pub fn into_value<T: FromDatum>(self) -> T {
        unsafe { T::from_datum(self.datum, self.is_null, self.typoid.value()).unwrap() }
    }
}

/// Similar to `heap_getattr()`, but returns extended information about the requested attribute
/// `attno` is 1-based
#[inline]
pub fn heap_getattr_datum_ex(
    tuple: &PgBox<pg_sys::HeapTupleData>,
    attno: usize,
    tupdesc: &PgTupleDesc,
) -> DatumWithTypeInfo {
    let mut is_null = false;
    let datum =
        unsafe { pgx_heap_getattr(tuple.as_ptr(), attno as u32, tupdesc.as_ptr(), &mut is_null) };
    let typoid = tupdesc.get(attno - 1).expect("no attribute").type_oid();

    let mut typlen = 0;
    let mut typbyval = false;
    let mut typalign = 0 as std::os::raw::c_char; // unused

    unsafe {
        pg_sys::get_typlenbyvalalign(typoid.value(), &mut typlen, &mut typbyval, &mut typalign);
    }

    DatumWithTypeInfo {
        datum,
        is_null,
        typoid,
        typlen,
        typbyval,
    }
}
