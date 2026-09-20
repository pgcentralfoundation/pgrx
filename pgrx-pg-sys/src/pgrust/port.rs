//! The subset of pgrx's `port.rs` (inline C macros) that applies to the
//! pgrust target: no buffers, pages or tree walkers.
#![allow(non_snake_case, non_upper_case_globals)]

use crate as pg_sys;
use core::mem::offset_of;

/// this comes from `postgres_ext.h`
pub const InvalidOid: crate::Oid = crate::Oid::INVALID;
pub const InvalidOffsetNumber: crate::OffsetNumber = 0;
pub const FirstOffsetNumber: crate::OffsetNumber = 1;
pub const InvalidBlockNumber: u32 = 0xFFFF_FFFF as crate::BlockNumber;
pub const VARHDRSZ: usize = std::mem::size_of::<crate::int32>();
pub const VARHDRSZ_EXTERNAL: usize = offset_of!(crate::varattrib_1b_e, va_data);
pub const VARHDRSZ_SHORT: usize = offset_of!(crate::varattrib_1b, va_data);
pub const InvalidCommandId: crate::CommandId = (!(0 as crate::CommandId)) as crate::CommandId;
pub const FirstCommandId: crate::CommandId = 0 as crate::CommandId;
pub const InvalidTransactionId: crate::TransactionId = crate::TransactionId::INVALID;
pub const BootstrapTransactionId: crate::TransactionId = crate::TransactionId::BOOTSTRAP;
pub const FrozenTransactionId: crate::TransactionId = crate::TransactionId::FROZEN;
pub const FirstNormalTransactionId: crate::TransactionId = crate::TransactionId::FIRST_NORMAL;
pub const MaxTransactionId: crate::TransactionId = crate::TransactionId::MAX;

// Extension SQL constants pgx-pg-sys defines outside bindgen.
pub const USECS_PER_SEC: i64 = 1_000_000;
pub const USECS_PER_MINUTE: i64 = 60 * USECS_PER_SEC;
pub const USECS_PER_HOUR: i64 = 60 * USECS_PER_MINUTE;
pub const USECS_PER_DAY: i64 = 24 * USECS_PER_HOUR;
pub const DATEVAL_NOBEGIN: i32 = i32::MIN;
pub const DATEVAL_NOEND: i32 = i32::MAX;
pub const DT_NOBEGIN: i64 = i64::MIN;
pub const DT_NOEND: i64 = i64::MAX;

/// Given a valid HeapTuple pointer, return address of the user data
///
/// # Safety
///
/// This function cannot determine if the `tuple` argument is really a non-null pointer to a [`pg_sys::HeapTuple`].
#[inline(always)]
pub unsafe fn GETSTRUCT(tuple: crate::HeapTuple) -> *mut std::os::raw::c_char {
    (*tuple)
        .t_data
        .cast::<std::os::raw::c_char>()
        .add((*(*tuple).t_data).t_hoff as _)
}

#[inline(always)]
pub const unsafe fn TYPEALIGN(alignval: usize, len: usize) -> usize {
    (len + (alignval - 1)) & !(alignval - 1)
}

#[inline(always)]
pub const unsafe fn MAXALIGN(len: usize) -> usize {
    TYPEALIGN(8, len)
}

#[inline]
pub fn get_pg_major_version_string() -> &'static str {
    "18"
}

#[inline]
pub fn get_pg_major_version_num() -> u16 {
    18
}

#[inline]
pub fn get_pg_version_string() -> &'static str {
    "PostgreSQL 18 (pgrust)"
}

pub fn get_pg_major_minor_version_string() -> &'static str {
    "18.6"
}

#[inline]
pub fn TransactionIdIsNormal(xid: crate::TransactionId) -> bool {
    xid >= FirstNormalTransactionId
}

/// `TransactionIdPrecedes` --- is id1 logically < id2?
#[inline]
pub unsafe fn TransactionIdPrecedes(id1: crate::TransactionId, id2: crate::TransactionId) -> bool {
    if !TransactionIdIsNormal(id1) || !TransactionIdIsNormal(id2) {
        return id1 < id2;
    }
    let diff = u32::from(id1).wrapping_sub(u32::from(id2)) as i32;
    diff < 0
}

#[inline]
pub unsafe fn TransactionIdPrecedesOrEquals(
    id1: crate::TransactionId,
    id2: crate::TransactionId,
) -> bool {
    if !TransactionIdIsNormal(id1) || !TransactionIdIsNormal(id2) {
        return id1 <= id2;
    }
    let diff = u32::from(id1).wrapping_sub(u32::from(id2)) as i32;
    diff <= 0
}

#[inline]
pub unsafe fn TransactionIdFollows(id1: crate::TransactionId, id2: crate::TransactionId) -> bool {
    if !TransactionIdIsNormal(id1) || !TransactionIdIsNormal(id2) {
        return id1 > id2;
    }
    let diff = u32::from(id1).wrapping_sub(u32::from(id2)) as i32;
    diff > 0
}

#[inline]
pub unsafe fn TransactionIdFollowsOrEquals(
    id1: crate::TransactionId,
    id2: crate::TransactionId,
) -> bool {
    if !TransactionIdIsNormal(id1) || !TransactionIdIsNormal(id2) {
        return id1 >= id2;
    }
    let diff = u32::from(id1).wrapping_sub(u32::from(id2)) as i32;
    diff >= 0
}

/// ```c
///     #define type_is_array(typid)  (get_element_type(typid) != InvalidOid)
/// ```
#[inline]
pub unsafe fn type_is_array(typoid: crate::Oid) -> bool {
    crate::get_element_type(typoid) != InvalidOid
}

/// that if it does, that its bytes are bitwise compatible with `T`.
///
/// [`FormData_pg_class`]: crate::FormData_pg_class
#[inline]
pub unsafe fn heap_tuple_get_struct<T>(htup: crate::HeapTuple) -> *mut T {
    if htup.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe {
            // SAFETY:  The caller has told us `htop` is a valid HeapTuple
            GETSTRUCT(htup).cast()
        }
    }
}
