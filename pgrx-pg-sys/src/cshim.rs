#![cfg(feature = "cshim")]
#![allow(deprecated)]

use crate as pg_sys;
use core::ffi;

#[pgrx_macros::pg_guard]
extern "C" {
    pub fn pgrx_list_nth(list: *mut pg_sys::List, nth: i32) -> *mut ffi::c_void;
    pub fn pgrx_list_nth_int(list: *mut pg_sys::List, nth: i32) -> i32;
    pub fn pgrx_list_nth_oid(list: *mut pg_sys::List, nth: i32) -> pg_sys::Oid;
    pub fn pgrx_list_nth_cell(list: *mut pg_sys::List, nth: i32) -> *mut pg_sys::ListCell;

    #[link_name = "pgrx_SpinLockInit"]
    pub fn SpinLockInit(lock: *mut pg_sys::slock_t);
    #[link_name = "pgrx_SpinLockAcquire"]
    pub fn SpinLockAcquire(lock: *mut pg_sys::slock_t);
    #[link_name = "pgrx_SpinLockRelease"]
    pub fn SpinLockRelease(lock: *mut pg_sys::slock_t);
    #[link_name = "pgrx_SpinLockFree"]
    pub fn SpinLockFree(lock: *mut pg_sys::slock_t) -> bool;
    #[link_name = "pgrx_PageGetSpecialPointer"]
    pub fn PageGetSpecialPointer(page: pg_sys::Page) -> *mut i8;
    #[link_name = "pgrx_PageGetItem"]
    pub fn PageGetItem(page: pg_sys::Page, itemId: pg_sys::ItemId) -> pg_sys::Item;
    #[link_name = "pgrx_PageGetItemId"]
    pub fn PageGetItemId(page: pg_sys::Page, offsetNumber: pg_sys::OffsetNumber) -> pg_sys::ItemId;
    #[link_name = "pgrx_table_beginscan_strat"]
    pub fn table_beginscan_strat(
        relation: pg_sys::Relation,
        snapshot: pg_sys::Snapshot,
        nkeys: i32,
        keys: *mut pg_sys::ScanKeyData,
        allow_strat: bool,
        allow_sync: bool,
    ) -> pg_sys::TableScanDesc;
    #[link_name = "pgrx_table_endscan"]
    pub fn table_endscan(scan: pg_sys::TableScanDesc);
    #[link_name = "pgrx_ExecQual"]
    pub fn ExecQual(state: *mut pg_sys::ExprState, econtext: *mut pg_sys::ExprContext) -> bool;
    #[link_name = "pgrx_ExecCopySlotHeapTuple"]
    pub fn ExecCopySlotHeapTuple(slot: pg_sys::TupleTableSlot) -> pg_sys::HeapTuple;
}
