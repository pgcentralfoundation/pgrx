//! Heap tuples and tuple descriptors for the pgrust target.
//!
//! pgrx reads `TupleDescData` and `HeapTupleData` as C structs (attribute
//! arrays by pointer arithmetic, `t_data` pointers). pgrust's are Rust
//! structs. The shims here convert at every call: a C-layout image is built
//! from a pgrust descriptor (allocated with `palloc`, so pgrx may `pfree`
//! it), and a pgrust descriptor is rebuilt from a C image when a pgrust
//! function needs one. Tuple images themselves are the on-disk format on
//! both sides, so only the `HeapTupleData` header struct is converted.

use core::ffi::{c_char, c_int};

use ::pgr_datum::Datum as NDatum;
use ::pgr_types_tuple::{
    CompactAttribute as NCompact, FormData_pg_attribute as NAttr, HeapTupleData as NHeapTuple,
    NameData as NName, TupleDescData as NTupleDesc,
};

use super::error::unwrap_pg;
use super::fmgr::result_mcx;
use super::mem::{palloc, palloc0};
use super::types::{
    CompactAttribute as CCompact, FormData_pg_attribute as CAttr, HeapTuple,
    HeapTupleData as CHeapTuple, HeapTupleHeader, ItemPointerData as CItemPointer,
    NameData as CName, TupleDesc, TupleDescData as CTupleDesc, int32,
};
use crate::{Datum, Oid};

// ---- attribute conversion ------------------------------------------------------

fn name_to_c(n: &NName) -> CName {
    let mut out = CName { data: [0; 64] };
    for (i, b) in n.data.iter().enumerate() {
        out.data[i] = *b as c_char;
    }
    out
}

fn name_from_c(n: &CName) -> NName {
    let mut out = NName { data: [0; 64] };
    for (i, b) in n.data.iter().enumerate() {
        out.data[i] = *b as u8;
    }
    out
}

fn attr_to_c(a: &NAttr) -> CAttr {
    CAttr {
        attrelid: Oid::from_u32(a.attrelid),
        attname: name_to_c(&a.attname),
        atttypid: Oid::from_u32(a.atttypid),
        attlen: a.attlen,
        attnum: a.attnum,
        atttypmod: a.atttypmod,
        attndims: a.attndims,
        attbyval: a.attbyval,
        attalign: a.attalign as c_char,
        attstorage: a.attstorage as c_char,
        attcompression: a.attcompression as c_char,
        attnotnull: a.attnotnull,
        atthasdef: a.atthasdef,
        atthasmissing: a.atthasmissing,
        attidentity: a.attidentity as c_char,
        attgenerated: a.attgenerated as c_char,
        attisdropped: a.attisdropped,
        attislocal: a.attislocal,
        attinhcount: a.attinhcount,
        attcollation: Oid::from_u32(a.attcollation),
    }
}

fn attr_from_c(a: &CAttr) -> NAttr {
    NAttr {
        attrelid: a.attrelid.to_u32(),
        attname: name_from_c(&a.attname),
        atttypid: a.atttypid.to_u32(),
        attlen: a.attlen,
        attnum: a.attnum,
        atttypmod: a.atttypmod,
        attndims: a.attndims,
        attbyval: a.attbyval,
        attalign: a.attalign as i8,
        attstorage: a.attstorage as i8,
        attcompression: a.attcompression as i8,
        attnotnull: a.attnotnull,
        atthasdef: a.atthasdef,
        atthasmissing: a.atthasmissing,
        attidentity: a.attidentity as i8,
        attgenerated: a.attgenerated as i8,
        attisdropped: a.attisdropped,
        attislocal: a.attislocal,
        attinhcount: a.attinhcount,
        attcollation: a.attcollation.to_u32(),
    }
}

fn compact_to_c(c: &NCompact) -> CCompact {
    CCompact {
        attcacheoff: c.attcacheoff.get(),
        attlen: c.attlen,
        attbyval: c.attbyval,
        attispackable: c.attispackable,
        atthasmissing: c.atthasmissing,
        attisdropped: c.attisdropped,
        attgenerated: c.attgenerated,
        attnullability: c.attnullability as c_char,
        attalignby: c.attalignby,
    }
}

// ---- tuple descriptor images ------------------------------------------------------

const TD_HDR: usize = core::mem::offset_of!(CTupleDesc, compact_attrs);

/// Byte size of a C tuple descriptor image with `natts` attributes.
fn c_tupdesc_size(natts: usize) -> usize {
    TD_HDR + natts * core::mem::size_of::<CCompact>() + natts * core::mem::size_of::<CAttr>()
}

/// The `Form_pg_attribute` array of a C descriptor image (after the compact
/// attributes, as PG18's `TupleDescAttrAddress`).
unsafe fn c_attrs_ptr(td: TupleDesc) -> *mut CAttr {
    let natts = (*td).natts as usize;
    (*td).compact_attrs.as_mut_ptr().add(natts).cast::<CAttr>()
}

/// A palloc'd C image of a pgrust descriptor (refcount -1: pgrx pfrees it).
pub fn c_tupdesc_from_native(td: &NTupleDesc<'_>) -> TupleDesc {
    let natts = td.natts as usize;
    // SAFETY: the image is sized for natts attributes and fully written below.
    unsafe {
        let img = palloc0(c_tupdesc_size(natts)) as TupleDesc;
        (*img).natts = td.natts;
        (*img).tdtypeid = Oid::from_u32(td.tdtypeid);
        (*img).tdtypmod = td.tdtypmod;
        (*img).tdrefcount = -1;
        (*img).constr = core::ptr::null_mut();
        let compact = (*img).compact_attrs.as_mut_ptr();
        let attrs = c_attrs_ptr(img);
        for i in 0..natts {
            compact.add(i).write(compact_to_c(&td.compact_attrs[i]));
            attrs.add(i).write(attr_to_c(&td.attrs[i]));
        }
        img
    }
}

/// A pgrust descriptor rebuilt from a C image, in `mcx`.
///
/// # Safety
/// `td` must be a live C descriptor image.
pub unsafe fn native_tupdesc_from_c<'m>(mcx: ::pgr_mcx::Mcx<'m>, td: TupleDesc) -> NTupleDesc<'m> {
    let natts = (*td).natts;
    let mut out = unwrap_pg(::pgr_tupdesc::CreateTemplateTupleDesc(mcx, natts));
    out.tdtypeid = (*td).tdtypeid.to_u32();
    out.tdtypmod = (*td).tdtypmod;
    let attrs = c_attrs_ptr(td);
    for i in 0..natts as usize {
        let a = attr_from_c(&*attrs.add(i));
        out.compact_attrs[i] = NCompact::populate_from(&a);
        out.attrs[i] = a;
    }
    out
}

pub unsafe fn CreateTupleDescCopyConstr(tupdesc: TupleDesc) -> TupleDesc {
    let natts = (*tupdesc).natts as usize;
    let size = c_tupdesc_size(natts);
    let copy = palloc(size) as TupleDesc;
    core::ptr::copy_nonoverlapping(tupdesc as *const u8, copy as *mut u8, size);
    (*copy).tdrefcount = -1;
    (*copy).constr = core::ptr::null_mut();
    copy
}

pub unsafe fn CreateTupleDescCopy(tupdesc: TupleDesc) -> TupleDesc {
    CreateTupleDescCopyConstr(tupdesc)
}

/// Our images are never refcounted (tdrefcount = -1); nothing to release.
pub unsafe fn DecrTupleDescRefCount(_tupdesc: TupleDesc) {}

pub unsafe fn lookup_rowtype_tupdesc_copy(type_id: Oid, typmod: int32) -> TupleDesc {
    let td = unwrap_pg(::pgr_typcache::lookup_rowtype_tupdesc_copy(
        result_mcx(),
        type_id.to_u32(),
        typmod,
    ));
    c_tupdesc_from_native(&td)
}

/// C's non-copy variant pins a cached descriptor; here every descriptor
/// handed to pgrx is its own image, so this is the copy too.
pub unsafe fn lookup_rowtype_tupdesc(type_id: Oid, typmod: int32) -> TupleDesc {
    lookup_rowtype_tupdesc_copy(type_id, typmod)
}

pub unsafe fn get_typtype(typid: Oid) -> c_char {
    unwrap_pg(::pgr_lsyscache::get_typtype(typid.to_u32())) as c_char
}

// ---- heap tuples ---------------------------------------------------------------------

fn itemptr_to_c(p: &::pgr_types_tuple::ItemPointerData) -> CItemPointer {
    // SAFETY: both are 6-byte repr(C) {BlockIdData, OffsetNumber}.
    unsafe { core::mem::transmute_copy(p) }
}

fn itemptr_from_c(p: &CItemPointer) -> ::pgr_types_tuple::ItemPointerData {
    // SAFETY: as above.
    unsafe { core::mem::transmute_copy(p) }
}

/// A palloc'd C `HeapTupleData` whose `t_data` points at `image`, which
/// must outlive the C struct's use.
unsafe fn c_heaptuple_over(
    t_len: u32,
    t_self: CItemPointer,
    t_table_oid: Oid,
    image: *mut u8,
) -> HeapTuple {
    let ht = palloc(core::mem::size_of::<CHeapTuple>()) as HeapTuple;
    ht.write(CHeapTuple {
        t_len,
        t_self,
        t_tableOid: t_table_oid,
        t_data: image as HeapTupleHeader,
    });
    ht
}

/// C's heap_copytuple: one palloc chunk holding the struct and the image.
pub unsafe fn heap_copytuple(tuple: HeapTuple) -> HeapTuple {
    if tuple.is_null() || (*tuple).t_data.is_null() {
        return core::ptr::null_mut();
    }
    let t_len = (*tuple).t_len as usize;
    let hdr = (core::mem::size_of::<CHeapTuple>() + 7) & !7;
    let chunk = palloc(hdr + t_len) as *mut u8;
    let ht = chunk as HeapTuple;
    let image = chunk.add(hdr);
    core::ptr::copy_nonoverlapping((*tuple).t_data as *const u8, image, t_len);
    ht.write(CHeapTuple {
        t_len: (*tuple).t_len,
        t_self: (*tuple).t_self,
        t_tableOid: (*tuple).t_tableOid,
        t_data: image as HeapTupleHeader,
    });
    ht
}

/// A pgrust view of a C heap tuple (no copy).
///
/// # Safety
/// `tuple` must be a live C heap tuple whose image outlives `'a`.
pub unsafe fn native_view_from_c<'a>(tuple: HeapTuple) -> NHeapTuple<'a> {
    NHeapTuple::from_raw_parts(
        (*tuple).t_data as *const u8,
        (*tuple).t_len,
        itemptr_from_c(&(*tuple).t_self),
        (*tuple).t_tableOid.to_u32(),
    )
}

/// A C heap tuple over an owned pgrust tuple; the image is leaked into the
/// result context (pgrust frees it with the context).
fn c_heaptuple_from_owned(t: ::pgr_heaptuple::HeapTuple<'_>) -> HeapTuple {
    let td = t.as_tuple();
    let t_len = td.t_len;
    let t_self = itemptr_to_c(&td.t_self);
    let t_table_oid = Oid::from_u32(td.t_tableOid);
    let image = td.header_ptr() as *mut u8;
    core::mem::forget(t);
    // SAFETY: the image lives in the result context for the rest of the call.
    unsafe { c_heaptuple_over(t_len, t_self, t_table_oid, image) }
}

pub unsafe fn heap_form_tuple(
    tupdesc: TupleDesc,
    values: *const Datum,
    isnull: *const bool,
) -> HeapTuple {
    let mcx = result_mcx();
    let ntd = native_tupdesc_from_c(mcx, tupdesc);
    let natts = ntd.natts as usize;
    let vals: Vec<NDatum> = if values.is_null() {
        vec![NDatum::null(); natts]
    } else {
        core::slice::from_raw_parts(values, natts)
            .iter()
            .map(|d| NDatum::from_usize(d.value()))
            .collect()
    };
    let nulls: Vec<bool> = core::slice::from_raw_parts(isnull, natts).to_vec();
    let t = unwrap_pg(::pgr_heaptuple::heap_form_tuple(mcx, &ntd, &vals, &nulls));
    c_heaptuple_from_owned(t)
}

pub unsafe fn heap_modify_tuple(
    tuple: HeapTuple,
    tupdesc: TupleDesc,
    repl_values: *const Datum,
    repl_isnull: *const bool,
    do_replace: *const bool,
) -> HeapTuple {
    let mcx = result_mcx();
    let ntd = native_tupdesc_from_c(mcx, tupdesc);
    let natts = ntd.natts as usize;
    let view = native_view_from_c(tuple);
    let vals: Vec<NDatum> = core::slice::from_raw_parts(repl_values, natts)
        .iter()
        .map(|d| NDatum::from_usize(d.value()))
        .collect();
    let nulls: Vec<bool> = core::slice::from_raw_parts(repl_isnull, natts).to_vec();
    let repl: Vec<bool> = core::slice::from_raw_parts(do_replace, natts).to_vec();
    let t = unwrap_pg(::pgr_heaptuple::heap_modify_tuple(
        mcx, &view, &ntd, &vals, &nulls, &repl,
    ));
    c_heaptuple_from_owned(t)
}

pub unsafe fn heap_copy_tuple_as_datum(tuple: HeapTuple, tupdesc: TupleDesc) -> Datum {
    let mcx = result_mcx();
    let ntd = native_tupdesc_from_c(mcx, tupdesc);
    let view = native_view_from_c(tuple);
    let d = unwrap_pg(::pgr_heaptuple::heap_copy_tuple_as_datum(mcx, &view, &ntd));
    Datum::from(d.as_usize())
}

pub unsafe fn heap_getattr(
    tup: HeapTuple,
    attnum: c_int,
    tupdesc: TupleDesc,
    isnull: *mut bool,
) -> Datum {
    let mcx = result_mcx();
    let ntd = native_tupdesc_from_c(mcx, tupdesc);
    let view = native_view_from_c(tup);
    let mut null = false;
    let d = ::pgr_types_tuple::heap_getattr(&view, attnum, &ntd, &mut null);
    *isnull = null;
    Datum::from(d.as_usize())
}

/// `HeapTupleHeaderGetTypeId` on a composite image.
pub unsafe fn heap_tuple_header_get_type_id(hdr: HeapTupleHeader) -> Oid {
    (*hdr).t_choice.t_datum.datum_typeid
}

/// `HeapTupleHeaderGetTypMod` on a composite image.
pub unsafe fn heap_tuple_header_get_typmod(hdr: HeapTupleHeader) -> i32 {
    (*hdr).t_choice.t_datum.datum_typmod
}

/// `HeapTupleHeaderGetDatumLength`.
pub unsafe fn heap_tuple_header_get_datum_length(hdr: HeapTupleHeader) -> u32 {
    ((*hdr).t_choice.t_datum.datum_len_ as u32) >> 2
}
