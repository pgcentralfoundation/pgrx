//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Utility functions for working with [`pg_sys::HeapTuple`] and [`pg_sys::HeapTupleHeader`] structs
use crate::*;
use core::ptr;
use std::num::NonZeroUsize;

/// Given a `pg_sys::Datum` representing a composite row type, return a boxed `HeapTupleData`,
/// which can be used by the various `heap_getattr` methods
///
/// Like PostgreSQL's `DatumGetHeapTupleHeader`, this fully detoasts `row`, so a
/// packed, compressed, or external composite is first copied into an ordinary
/// value allocated in `CurrentMemoryContext`.
///
/// # Safety
///
/// `row` must be a composite (row type) Datum: a pointer to a live varlena whose
/// detoasted payload is a `HeapTupleHeader` datum. The call must run on the
/// PostgreSQL backend thread. The returned tuple points into `row`, or into its
/// detoasted copy, so it is valid only while that memory is.
///
/// # Panics
///
/// Panics if `row` is a NULL pointer.
#[inline]
pub unsafe fn composite_row_type_make_tuple(
    row: pg_sys::Datum,
) -> PgBox<pg_sys::HeapTupleData, AllocatedByRust> {
    // Detoasting reads the varlena header, so reject NULL before it can be dereferenced.
    if row.is_null() {
        panic!("Attempt to dereference a null HeapTupleHeader");
    }
    // SAFETY: the caller supplies a live composite varlena. pg_detoast_datum,
    // unlike pg_detoast_datum_packed, also converts a one-byte header to the
    // four-byte form that HeapTupleHeaderGetDatumLength reads.
    let htup_header =
        unsafe { pg_sys::pg_detoast_datum(row.cast_mut_ptr()) } as pg_sys::HeapTupleHeader;
    // SAFETY: alloc0 returns zeroed, aligned storage for the plain C struct.
    let mut tuple = unsafe { PgBox::<pg_sys::HeapTupleData>::alloc0() };

    // SAFETY: the detoasted value is a live composite datum with a four-byte
    // varlena header, which is the header HeapTupleHeaderGetDatumLength reads.
    tuple.t_len = unsafe { pg_sys::HeapTupleHeaderGetDatumLength!(htup_header).get() };
    tuple.t_data = htup_header;

    tuple
}

/// Extract an attribute of a heap tuple and return it as Rust type.
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
pub fn heap_getattr<T: FromDatum, AllocatedBy: WhoAllocated>(
    tuple: &PgBox<pg_sys::HeapTupleData, AllocatedBy>,
    attno: NonZeroUsize,
    tupdesc: &PgTupleDesc,
) -> Option<T> {
    // PostgreSQL only asserts that `attno` is within the descriptor, so check it first.
    let typoid = tupdesc.get(attno.get() - 1).expect("no attribute").type_oid();
    // SAFETY: not established. The lookup above bounds `attno`, and its reference ends with that
    // statement. This safe function cannot check the rest of heap_getattr_raw's contract or the
    // result's lifetime: `tuple` can be null or store attributes that `tupdesc` does not
    // describe, a caller can hold a reference to `tupdesc`'s attribute entries across the call,
    // and a by-reference `T` can outlive the tuple.
    let datum = unsafe { heap_getattr_raw(tuple.as_ptr(), attno, tupdesc.as_ptr()) }?;
    unsafe { T::from_polymorphic_datum(datum, false, typoid.value()) }
}

/// Extract user attribute `attno` (1-based) of a heap tuple and return it as a Datum.
///
/// If the field in question has a NULL value, we return `None`.
/// Otherwise, a `Some(pg_sys::Datum)`
///
/// System attributes, such as `ctid`, have non-positive attribute numbers and cannot be
/// requested. Unlike [`heap_getattr()`], this does not check `attno` against the descriptor.
///
/// ## Safety
///
/// `tuple` must point to a live heap tuple, and each attribute it stores must have the same
/// length, alignment, and pass-by-value flag as the attribute in that position of the live
/// descriptor `tupdesc`, which can describe more attributes than the tuple stores. A tuple
/// formed with `tupdesc` qualifies. So does a stored row of the same row type after attributes
/// are added or dropped, because PostgreSQL keeps a dropped attribute's length, alignment, and
/// pass-by-value flag, though it clears its type. `attno` must not exceed the descriptor's
/// attribute count. The call must run on the PostgreSQL backend thread.
///
/// The call can cache attribute offsets by writing `attcacheoff` in `tupdesc`'s attribute
/// entries, so no Rust reference to those entries may be live across it. On PostgreSQL 15
/// through 17, those entries are the attributes that [`PgTupleDesc::get`] borrows.
///
/// A pass-by-reference result can point into the tuple or, on PostgreSQL 15.0 through 15.4,
/// into `tupdesc`, and is valid only while both are.
#[inline]
pub unsafe fn heap_getattr_raw(
    tuple: *mut pg_sys::HeapTupleData,
    attno: NonZeroUsize,
    tupdesc: pg_sys::TupleDesc,
) -> Option<pg_sys::Datum> {
    // PostgreSQL 15 through 19 define `heap_getattr` and `fastgetattr` as static inline
    // functions with this logic for a positive attribute number (PG18 and later read the
    // `CompactAttribute` instead of the `FormData_pg_attribute`):
    //
    //     if (attnum > (int) HeapTupleHeaderGetNatts(tup->t_data))
    //         return getmissingattr(tupleDesc, attnum, isnull);
    //     *isnull = false;
    //     if (HeapTupleNoNulls(tup))
    //     {
    //         att = TupleDescAttr(tupleDesc, attnum - 1);
    //         if (att->attcacheoff >= 0)
    //             return fetchatt(att, (char *) tup->t_data + tup->t_data->t_hoff +
    //                             att->attcacheoff);
    //         else
    //             return nocachegetattr(tup, attnum, tupleDesc);
    //     }
    //     else
    //     {
    //         if (att_isnull(attnum - 1, tup->t_data->t_bits))
    //         {
    //             *isnull = true;
    //             return (Datum) NULL;
    //         }
    //         else
    //             return nocachegetattr(tup, attnum, tupleDesc);
    //     }
    //
    // The generated `heap_getattr!` calls a guarded native `fastgetattr` or `fetch_att` for
    // every attribute, so this keeps the cached-offset and NULL-bitmap paths in Rust and calls
    // the exported `getmissingattr` and `nocachegetattr` only where the C does.
    let index = attno.get() - 1;
    // tupdesc->natts is a C int, which bounds attno.
    let attnum = attno.get() as core::ffi::c_int;
    // SAFETY: the caller supplies a live tuple, a live descriptor whose attributes match the
    // stored ones in length, alignment, and pass-by-value flag, and an in-range attno.
    unsafe {
        let t_data = (*tuple).t_data;
        if attno.get() > pg_sys::HeapTupleHeaderGetNatts!(t_data).get() as usize {
            let mut isnull = false;
            let datum = pg_sys::getmissingattr(tupdesc, attnum, &mut isnull);
            return (!isnull).then_some(datum);
        }

        if pg_sys::HeapTupleNoNulls!(tuple).is_true() {
            let (cacheoff, attbyval, attlen) = attribute_fetch_info(tupdesc, index);
            if cacheoff >= 0 {
                let value = t_data.cast::<u8>().add((*t_data).t_hoff as usize + cacheoff as usize);
                return Some(fetch_att(value, attbyval, attlen));
            }
        } else {
            let bits = ptr::addr_of!((*t_data).t_bits).cast::<pg_sys::bits8>();
            if pg_sys::att_isnull!(attnum - 1, bits).is_true() {
                return None;
            }
        }
        Some(pg_sys::nocachegetattr(tuple, attnum, tupdesc))
    }
}

/// Return `(attcacheoff, attbyval, attlen)` for attribute `index`, from the attribute that
/// `fastgetattr` reads: `TupleDescAttr(tupleDesc, index)` on PG15-17.
///
/// # Safety
///
/// `tupdesc` must be live and `index` less than its attribute count.
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[inline]
unsafe fn attribute_fetch_info(tupdesc: pg_sys::TupleDesc, index: usize) -> (i32, bool, i16) {
    // SAFETY: the caller supplies a live descriptor and an in-range index.
    unsafe {
        let att = crate::tupdesc::tupdesc_attr(tupdesc, index);
        ((*att).attcacheoff, (*att).attbyval, (*att).attlen)
    }
}

/// Return `(attcacheoff, attbyval, attlen)` for attribute `index`, from the attribute that
/// `fastgetattr` reads: `TupleDescCompactAttr(tupleDesc, index)` on PG18 and PG19.
///
/// That static inline function returns `&tupdesc->compact_attrs[i]`; assert-enabled builds
/// also call `verify_compact_attribute`, which the generated form makes a native call.
///
/// # Safety
///
/// `tupdesc` must be live and `index` less than its attribute count.
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[allow(clippy::useless_conversion, reason = "attcacheoff is int32 on PG18 and int16 on PG19")]
#[inline]
unsafe fn attribute_fetch_info(tupdesc: pg_sys::TupleDesc, index: usize) -> (i32, bool, i16) {
    // SAFETY: the caller supplies a live descriptor and an in-range index. The raw place
    // projection keeps the descriptor's provenance for its trailing compact_attrs array.
    unsafe {
        let att =
            ptr::addr_of!((*tupdesc).compact_attrs).cast::<pg_sys::CompactAttribute>().add(index);
        ((*att).attcacheoff.into(), (*att).attbyval, (*att).attlen)
    }
}

/// Convert the attribute stored at `value` to a Datum, as PostgreSQL's `fetch_att` does.
///
/// PG15 defines `fetch_att` as a macro and PG16-19 as a static inline function. Both return
/// `PointerGetDatum(T)` for a by-reference attribute. For a by-value one they apply
/// `CharGetDatum`, `Int16GetDatum`, or `Int32GetDatum` to a 1-, 2-, or 4-byte value, and load
/// an 8-byte value as a whole Datum (PG19 spells that `Int64GetDatum`). PG16-19 raise
/// "unsupported byval length" for any other length, and PG15 asserts.
///
/// The 8-byte value is loaded as a `Datum`, not rebuilt from an integer, so a pointer stored
/// by value, such as an `internal` attribute, keeps its provenance.
///
/// # Safety
///
/// `value` must point to an initialized attribute of `attlen` bytes inside a live tuple.
#[inline]
unsafe fn fetch_att(value: *mut u8, attbyval: bool, attlen: i16) -> pg_sys::Datum {
    if !attbyval {
        return pg_sys::Datum::from(value);
    }
    // SAFETY: the caller supplies attlen initialized bytes at value. PostgreSQL aligns them
    // for their type; the unaligned reads simply do not rely on that.
    unsafe {
        match attlen {
            // CharGetDatum widens plain C char with the inspected signedness.
            1 => pg_sys::Datum::from(value.cast::<pg_sys::__pgrx_c_macros::CCharRepr>().read()),
            2 => pg_sys::Datum::from(value.cast::<i16>().read_unaligned()),
            4 => pg_sys::Datum::from(value.cast::<i32>().read_unaligned()),
            8 if size_of::<pg_sys::Datum>() == 8 => value.cast::<pg_sys::Datum>().read_unaligned(),
            _ => panic!("unsupported byval length: {attlen}"),
        }
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
        unsafe { T::from_polymorphic_datum(self.datum, self.is_null, self.typoid.value()).unwrap() }
    }
}

/// Similar to `heap_getattr()`, but returns extended information about the requested attribute
/// `attno` is 1-based
#[inline]
pub fn heap_getattr_datum_ex(
    tuple: &PgBox<pg_sys::HeapTupleData>,
    attno: NonZeroUsize,
    tupdesc: &PgTupleDesc,
) -> DatumWithTypeInfo {
    // PostgreSQL only asserts that `attno` is within the descriptor, so check it first.
    let typoid = tupdesc.get(attno.get() - 1).expect("no attribute").type_oid();
    // SAFETY: not established. The lookup above bounds `attno`, and its reference ends with that
    // statement. This safe function cannot check the rest of heap_getattr_raw's contract:
    // `tuple` can be null or store attributes that `tupdesc` does not describe, and a caller can
    // hold a reference to `tupdesc`'s attribute entries across the call.
    let datum = unsafe { heap_getattr_raw(tuple.as_ptr(), attno, tupdesc.as_ptr()) };
    let is_null = datum.is_none();
    let datum = datum.unwrap_or_else(pg_sys::Datum::null);

    let mut typlen = 0;
    let mut typbyval = false;
    let mut typalign = 0 as std::os::raw::c_char; // unused

    unsafe {
        pg_sys::get_typlenbyvalalign(typoid.value(), &mut typlen, &mut typbyval, &mut typalign);
    }

    DatumWithTypeInfo { datum, is_null, typoid, typlen, typbyval }
}

/// Implemented for Rust tuples that can be represented as a Postgres [`pg_sys::HeapTupleData`].
pub trait IntoHeapTuple {
    /// Convert `Self` into a `pg_sys::HeapTupleData`, returning a pointer to it.
    ///
    /// # Safety
    ///
    /// This function is unsafe as it cannot guarantee the specified `tupdesc` is valid.
    unsafe fn into_heap_tuple(
        self,
        tupdesc: *mut pg_sys::TupleDescData,
    ) -> *mut pg_sys::HeapTupleData;
}
