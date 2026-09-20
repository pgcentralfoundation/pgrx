//! Hand-written shims for the non-fmgr backend functions the pgrx runtime
//! calls, each with its bindgen signature, implemented over pgrust crates.
//! Results that pgrx may `pfree` are copied into `palloc` chunks.

use core::ffi::{c_char, c_int, c_void};

use ::pgr_datum::Datum as NDatum;
use ::pgr_types_core::Oid as NOid;
use ::pgr_types_error::PgError;

use super::error::{raise, unwrap_pg};
use super::fmgr::{FmgrInfo, FunctionCallInfoBaseData, result_mcx};
use super::mem::{MemoryContext, current_memory_context, palloc, palloc_copy, pfree, repalloc};
use super::types::{
    ArrayType, HeapTupleHeader, Numeric, RangeBound, RangeType, StringInfo, StringInfoData,
    TupleDesc, TypeFuncClass, int16, int32, text, varlena,
};
use crate::{Datum, Oid};

#[inline]
fn noid(o: Oid) -> NOid {
    o.to_u32()
}

#[inline]
fn oid(o: NOid) -> Oid {
    Oid::from_u32(o)
}

#[cold]
fn unsupported(what: &str) -> ! {
    raise(Box::new(
        PgError::error(format!("pgrx: {what} is not supported under pgrust yet"))
            .with_sqlstate(::pgr_types_error::ERRCODE_FEATURE_NOT_SUPPORTED),
    ))
}

// ---- varlena / detoast ----------------------------------------------------

/// The full image behind a varlena pointer, whatever its header form.
unsafe fn varlena_image<'a>(p: *const u8) -> &'a [u8] {
    let len = ::pgr_types_tuple::varatt::varsize_any(p);
    core::slice::from_raw_parts(p, len)
}

unsafe fn detoast_to_chunk(p: *const u8) -> *mut varlena {
    let image = varlena_image(p);
    let flat = unwrap_pg(::pgr_detoast::detoast_attr(result_mcx(), image));
    palloc_copy(&flat) as *mut varlena
}

pub unsafe fn pg_detoast_datum(datum: *mut varlena) -> *mut varlena {
    let p = datum as *const u8;
    if ::pgr_types_tuple::varatt::varatt_is_4b_u(p) {
        return datum;
    }
    detoast_to_chunk(p)
}

pub unsafe fn pg_detoast_datum_copy(datum: *mut varlena) -> *mut varlena {
    let p = datum as *const u8;
    if ::pgr_types_tuple::varatt::varatt_is_4b_u(p) {
        return palloc_copy(varlena_image(p)) as *mut varlena;
    }
    detoast_to_chunk(p)
}

pub unsafe fn pg_detoast_datum_packed(datum: *mut varlena) -> *mut varlena {
    let p = datum as *const u8;
    // C: only external and compressed images are expanded; short-header
    // images are returned as they are.
    // VARATT_IS_4B_C: 4-byte header with the compressed bit (little-endian).
    let is_4b_c = (*p & 0x03) == 0x02;
    if ::pgr_types_tuple::varatt::varatt_is_1b_e(p) || is_4b_c {
        detoast_to_chunk(p)
    } else {
        datum
    }
}

pub unsafe fn cstring_to_text_with_len(s: *const c_char, len: c_int) -> *mut text {
    let len = len as usize;
    let p = palloc(4 + len) as *mut u8;
    let header = ((4 + len) as u32) << 2; // VARSIZE_4B, little-endian
    core::ptr::copy_nonoverlapping(header.to_le_bytes().as_ptr(), p, 4);
    core::ptr::copy_nonoverlapping(s as *const u8, p.add(4), len);
    p as *mut text
}

// ---- arrays -----------------------------------------------------------------

pub unsafe fn deconstruct_array(
    array: *mut ArrayType,
    _elmtype: Oid,
    elmlen: c_int,
    elmbyval: bool,
    elmalign: c_char,
    elemsp: *mut *mut Datum,
    nullsp: *mut *mut bool,
    nelemsp: *mut c_int,
) {
    let image = varlena_image(array as *const u8);
    let (elems, nulls) = unwrap_pg(::pgr_arrayfuncs::deconstruct_array(
        result_mcx(),
        image,
        elmlen,
        elmbyval,
        elmalign as u8,
        true,
    ));
    let n = elems.len();
    let ep = palloc(n.max(1) * core::mem::size_of::<Datum>()) as *mut Datum;
    let np = palloc(n.max(1)) as *mut bool;
    for i in 0..n {
        ep.add(i).write(Datum::from(elems[i].as_usize()));
        np.add(i).write(nulls[i]);
    }
    *elemsp = ep;
    if !nullsp.is_null() {
        *nullsp = np;
    }
    *nelemsp = n as c_int;
}

pub unsafe fn construct_md_array(
    elems: *mut Datum,
    nulls: *mut bool,
    ndims: c_int,
    dims: *mut c_int,
    lbs: *mut c_int,
    elmtype: Oid,
    elmlen: c_int,
    elmbyval: bool,
    elmalign: c_char,
) -> *mut ArrayType {
    let nd = ndims.max(0) as usize;
    let dims_s = core::slice::from_raw_parts(dims, nd);
    let lbs_s = core::slice::from_raw_parts(lbs, nd);
    let n: usize = dims_s.iter().map(|&d| d.max(0) as usize).product::<usize>() * (nd > 0) as usize;
    let elems_v: Vec<NDatum> = core::slice::from_raw_parts(elems, n)
        .iter()
        .map(|d| NDatum::from_usize(d.value()))
        .collect();
    let nulls_v: Option<Vec<bool>> =
        (!nulls.is_null()).then(|| core::slice::from_raw_parts(nulls, n).to_vec());
    let image = unwrap_pg(::pgr_arrayfuncs::construct_md_array(
        result_mcx(),
        &elems_v,
        nulls_v.as_deref(),
        ndims,
        dims_s,
        lbs_s,
        noid(elmtype),
        elmlen,
        elmbyval,
        elmalign as u8,
    ));
    palloc_copy(&image) as *mut ArrayType
}

pub unsafe fn construct_array(
    elems: *mut Datum,
    nelems: c_int,
    elmtype: Oid,
    elmlen: c_int,
    elmbyval: bool,
    elmalign: c_char,
) -> *mut ArrayType {
    let mut dims = [nelems];
    let mut lbs = [1];
    construct_md_array(
        elems,
        core::ptr::null_mut(),
        1,
        dims.as_mut_ptr(),
        lbs.as_mut_ptr(),
        elmtype,
        elmlen,
        elmbyval,
        elmalign,
    )
}

pub unsafe fn array_contains_nulls(array: *mut ArrayType) -> bool {
    ::pgr_arrayfuncs::array_contains_nulls(varlena_image(array as *const u8))
}

// ---- type lookups -------------------------------------------------------------

pub unsafe fn get_array_type(typid: Oid) -> Oid {
    oid(unwrap_pg(::pgr_lsyscache::get_array_type(noid(typid))))
}

pub unsafe fn get_element_type(typid: Oid) -> Oid {
    oid(unwrap_pg(::pgr_lsyscache::get_element_type(noid(typid))))
}

pub unsafe fn get_typlenbyvalalign(
    typid: Oid,
    typlen: *mut int16,
    typbyval: *mut bool,
    typalign: *mut c_char,
) {
    let (len, byval, align) = unwrap_pg(::pgr_lsyscache::get_typlenbyvalalign(noid(typid)));
    *typlen = len;
    *typbyval = byval;
    *typalign = align as c_char;
}

pub unsafe fn get_typlenbyval(typid: Oid, typlen: *mut int16, typbyval: *mut bool) {
    let (len, byval) = unwrap_pg(::pgr_lsyscache::get_typlenbyval(noid(typid)));
    *typlen = len;
    *typbyval = byval;
}

pub unsafe fn format_type_extended(type_oid: Oid, typemod: int32, flags: u16) -> *mut c_char {
    match unwrap_pg(::pgr_format_type::format_type_extended(
        noid(type_oid),
        typemod,
        flags,
    )) {
        Some(s) => {
            let mut bytes = s.into_bytes();
            bytes.push(0);
            palloc_copy(&bytes) as *mut c_char
        }
        None => core::ptr::null_mut(),
    }
}

pub unsafe fn format_type_be(type_oid: Oid) -> *mut c_char {
    let mut bytes = unwrap_pg(::pgr_format_type::format_type_be(noid(type_oid))).into_bytes();
    bytes.push(0);
    palloc_copy(&bytes) as *mut c_char
}

pub unsafe fn IsBinaryCoercible(srctype: Oid, targettype: Oid) -> bool {
    unwrap_pg(::pgr_coerce::IsBinaryCoercible(
        noid(srctype),
        noid(targettype),
    ))
}

// ---- fmgr helpers -----------------------------------------------------------------

pub unsafe fn get_fn_expr_argtype(flinfo: *mut FmgrInfo, argnum: c_int) -> Oid {
    if flinfo.is_null() || (*flinfo).native.is_null() {
        return Oid::INVALID;
    }
    let native: &::pgr_fmgr::FmgrInfo = &*((*flinfo).native as *const ::pgr_fmgr::FmgrInfo);
    oid(::pgr_funcapi::get_fn_expr_argtype(
        Some(native),
        argnum as usize,
    ))
}

pub unsafe fn get_fn_expr_arg_stable(flinfo: *mut FmgrInfo, argnum: c_int) -> bool {
    if flinfo.is_null() || (*flinfo).native.is_null() {
        return false;
    }
    let native: &::pgr_fmgr::FmgrInfo = &*((*flinfo).native as *const ::pgr_fmgr::FmgrInfo);
    ::pgr_funcapi::get_fn_expr_arg_stable(Some(native), argnum as usize)
}

pub unsafe fn get_fn_expr_rettype(flinfo: *mut FmgrInfo) -> Oid {
    if flinfo.is_null() || (*flinfo).native.is_null() {
        return Oid::INVALID;
    }
    let native: &::pgr_fmgr::FmgrInfo = &*((*flinfo).native as *const ::pgr_fmgr::FmgrInfo);
    oid(::pgr_funcapi::get_fn_expr_rettype(native))
}

pub unsafe fn fmgr_info(_function_id: Oid, _finfo: *mut FmgrInfo) {
    unsupported("fmgr_info on a C-shaped FmgrInfo")
}

pub unsafe fn get_call_result_type(
    _fcinfo: *mut FunctionCallInfoBaseData,
    _result_type_id: *mut Oid,
    _result_tuple_desc: *mut TupleDesc,
) -> TypeFuncClass::Type {
    unsupported("get_call_result_type")
}

pub unsafe fn BlessTupleDesc(tupdesc: TupleDesc) -> TupleDesc {
    tupdesc
}

pub unsafe fn HeapTupleHeaderGetDatum(_tuple: HeapTupleHeader) -> Datum {
    unsupported("HeapTupleHeaderGetDatum")
}

// ---- StringInfo -------------------------------------------------------------------

pub unsafe fn makeStringInfo() -> StringInfo {
    let si = palloc(core::mem::size_of::<StringInfoData>()) as StringInfo;
    initStringInfo(si);
    si
}

pub unsafe fn initStringInfo(str_: StringInfo) {
    let cap = 1024;
    (*str_).data = palloc(cap) as *mut c_char;
    (*str_).maxlen = cap as c_int;
    (*str_).len = 0;
    (*str_).cursor = 0;
    *(*str_).data = 0;
}

pub unsafe fn resetStringInfo(str_: StringInfo) {
    (*str_).len = 0;
    (*str_).cursor = 0;
    *(*str_).data = 0;
}

pub unsafe fn enlargeStringInfo(str_: StringInfo, needed: c_int) {
    let need = (*str_).len as usize + needed as usize + 1;
    if need <= (*str_).maxlen as usize {
        return;
    }
    let mut cap = (*str_).maxlen as usize;
    while cap < need {
        cap *= 2;
    }
    (*str_).data = repalloc((*str_).data as *mut c_void, cap) as *mut c_char;
    (*str_).maxlen = cap as c_int;
}

pub unsafe fn appendBinaryStringInfo(str_: StringInfo, data: *const c_void, datalen: c_int) {
    enlargeStringInfo(str_, datalen);
    let dst = (*str_).data.add((*str_).len as usize) as *mut u8;
    core::ptr::copy_nonoverlapping(data as *const u8, dst, datalen as usize);
    (*str_).len += datalen;
    *(*str_).data.add((*str_).len as usize) = 0;
}

pub unsafe fn appendStringInfoString(str_: StringInfo, s: *const c_char) {
    let len = core::ffi::CStr::from_ptr(s).to_bytes().len();
    appendBinaryStringInfo(str_, s as *const c_void, len as c_int);
}

pub unsafe fn appendStringInfoChar(str_: StringInfo, ch: c_char) {
    appendBinaryStringInfo(str_, &ch as *const c_char as *const c_void, 1);
}

// ---- numeric / ranges (phase 1) --------------------------------------------------

pub unsafe fn numeric_is_nan(num: Numeric) -> bool {
    // NUMERIC_NAN: sign field 0xC000 in the (short or long) header; both
    // header forms put n_sign_dscale / n_header at offset 4 after the
    // varlena header (image is 4B-header, uncompressed here).
    let p = num as *const u8;
    let image = varlena_image(p);
    if image.len() < 6 {
        return false;
    }
    let hdr = u16::from_le_bytes([image[4], image[5]]);
    (hdr & 0xC000) == 0xC000 && (hdr & 0xF000) != 0xF000 || hdr == 0xC000
}

pub unsafe fn numeric_normalize(_num: Numeric) -> *mut c_char {
    unsupported("numeric_normalize")
}

pub unsafe fn lookup_type_cache(_type_id: Oid, _flags: c_int) -> *mut c_void {
    unsupported("lookup_type_cache")
}

pub unsafe fn make_range(
    _typcache: *mut c_void,
    _lower: *mut RangeBound,
    _upper: *mut RangeBound,
    _empty: bool,
    _escontext: *mut c_void,
) -> *mut RangeType {
    unsupported("make_range")
}

pub unsafe fn range_deserialize(
    _typcache: *mut c_void,
    _range: *const RangeType,
    _lower: *mut RangeBound,
    _upper: *mut RangeBound,
    _empty: *mut bool,
) {
    unsupported("range_deserialize")
}

// ---- timestamps ------------------------------------------------------------------------

pub unsafe fn GetCurrentTransactionStartTimestamp() -> i64 {
    ::pgr_xact::GetCurrentTransactionStartTimestamp()
}

pub unsafe fn GetCurrentStatementStartTimestamp() -> i64 {
    ::pgr_xact::GetCurrentStatementStartTimestamp()
}

pub unsafe fn GetCurrentTimestamp() -> i64 {
    ::pgr_adt_timestamp::GetCurrentTimestamp()
}

pub unsafe fn GetSQLCurrentTimestamp(typmod: int32) -> i64 {
    ::pgr_adt_timestamp::GetSQLCurrentTimestamp(typmod)
}

pub unsafe fn GetSQLLocalTimestamp(typmod: int32) -> i64 {
    unwrap_pg(::pgr_adt_timestamp::GetSQLLocalTimestamp(typmod))
}

pub unsafe fn JsonEncodeDateTime(
    _buf: *mut c_void,
    _value: Datum,
    _typid: Oid,
    _tzp: *const c_int,
) -> *mut c_char {
    unsupported("JsonEncodeDateTime")
}

// Timezone-name helpers (pgrx datetime support): not bridged yet.
pub unsafe fn DecodeTimezoneName(
    _tzname: *const c_char,
    _offset: *mut c_int,
    _tz: *mut *mut super::pg_tz,
) -> c_int {
    unsupported("DecodeTimezoneName")
}

pub unsafe fn DetermineTimeZoneAbbrevOffsetTS(
    _ts: i64,
    _abbr: *const c_char,
    _tzp: *mut super::pg_tz,
    _isdst: *mut c_int,
) -> c_int {
    unsupported("DetermineTimeZoneAbbrevOffsetTS")
}

pub unsafe fn timestamp2tm(
    _dt: i64,
    _tzp: *mut c_int,
    _tm: *mut super::types::pg_tm,
    _fsec: *mut i32,
    _tzn: *mut *const c_char,
    _attimezone: *mut super::pg_tz,
) -> c_int {
    unsupported("timestamp2tm")
}

// ---- quoting / transactions (Spi) ------------------------------------------------

pub unsafe fn quote_identifier(ident: *const c_char) -> *const c_char {
    let bytes = core::ffi::CStr::from_ptr(ident).to_bytes();
    let q = unwrap_pg(::pgr_adt_quote::quote_identifier(result_mcx(), bytes));
    let mut out = q.as_bytes().to_vec();
    out.push(0);
    palloc_copy(&out) as *const c_char
}

pub unsafe fn quote_qualified_identifier(
    qualifier: *const c_char,
    ident: *const c_char,
) -> *mut c_char {
    let q = if qualifier.is_null() {
        None
    } else {
        Some(
            core::ffi::CStr::from_ptr(qualifier)
                .to_string_lossy()
                .into_owned(),
        )
    };
    let i = core::ffi::CStr::from_ptr(ident)
        .to_string_lossy()
        .into_owned();
    let mut out = ::pgr_ruleutils::quote_qualified_identifier(q.as_deref(), &i).into_bytes();
    out.push(0);
    palloc_copy(&out) as *mut c_char
}

pub unsafe fn quote_literal_cstr(rawstr: *const c_char) -> *mut c_char {
    let bytes = core::ffi::CStr::from_ptr(rawstr).to_bytes();
    let v = unwrap_pg(::pgr_adt_quote::quote_literal(result_mcx(), bytes));
    // the text payload after the 4-byte varlena header
    let image = v.into_image();
    let mut out = image[4..].to_vec();
    out.push(0);
    palloc_copy(&out) as *mut c_char
}

pub unsafe fn GetCurrentTransactionIdIfAny() -> crate::TransactionId {
    crate::TransactionId::from(::pgr_xact::GetCurrentTransactionIdIfAny())
}

pub unsafe fn GetCurrentTransactionId() -> crate::TransactionId {
    crate::TransactionId::from(unwrap_pg(::pgr_xact::GetCurrentTransactionId()))
}

// ---- aggregates ----------------------------------------------------------------------

/// C's AggCheckCallContext: 1 (and the aggregate's memory context) when the
/// call is an aggregate transition/final call, else 0.
pub unsafe fn AggCheckCallContext(
    fcinfo: *mut FunctionCallInfoBaseData,
    aggcontext: *mut MemoryContext,
) -> c_int {
    let Some(native) = super::fmgr::native_fcinfo(fcinfo) else {
        return 0;
    };
    match native.agg_context() {
        Some(mcx) => {
            if !aggcontext.is_null() {
                *aggcontext = super::mem::handle_of(mcx.context());
            }
            1
        }
        None => 0,
    }
}

// ---- misc -----------------------------------------------------------------------------

pub unsafe fn message_level_is_interesting(elevel: c_int) -> bool {
    ::pgr_elog::message_level_is_interesting(::pgr_types_error::ErrorLevel(elevel))
}

/// C's CHECK_FOR_INTERRUPTS(): a pending cancel/die raises.
pub fn check_for_interrupts() {
    if ::pgr_init_small::globals::InterruptPending() {
        unwrap_pg(::pgr_postgres_seams::check_for_interrupts::call());
    }
}

/// pgrx's `PgTryBuilder` flushes the C error state after a handler ran;
/// pgrust errors are values, so there is nothing to flush.
pub unsafe fn FlushErrorState() {}

pub unsafe fn GetDatabaseEncoding() -> c_int {
    ::pgr_mbutils::GetDatabaseEncoding() as c_int
}

/// Delete pgrx-owned memory contexts is handled in `mem`; this keeps the
/// C name available for the few `PgBox` drop paths that call it directly.
pub use super::mem::MemoryContextDelete as MemoryContextDeleteShim;

#[allow(dead_code)]
fn _unused(_: MemoryContext, _: *mut c_void) {
    // SAFETY: never called; keeps the imports honest without warnings.
    let _ = pfree;
}
