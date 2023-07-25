#![cfg(feature = "cshim")]

use crate as pg_sys;
use core::ffi;

#[pgrx_macros::pg_guard]
extern "C" {
    pub fn pgrx_list_nth(list: *mut pg_sys::List, nth: i32) -> *mut ffi::c_void;
    pub fn pgrx_list_nth_int(list: *mut pg_sys::List, nth: i32) -> i32;
    pub fn pgrx_list_nth_oid(list: *mut pg_sys::List, nth: i32) -> pg_sys::Oid;
    pub fn pgrx_list_nth_cell(list: *mut pg_sys::List, nth: i32) -> *mut pg_sys::ListCell;

    #[link_name = "pgrx_planner_rt_fetch"]
    pub fn planner_rt_fetch(
        index: pg_sys::Index,
        root: *mut pg_sys::PlannerInfo,
    ) -> *mut pg_sys::RangeTblEntry;

    #[link_name = "pgrx_SpinLockInit"]
    pub fn SpinLockInit(lock: *mut pg_sys::slock_t);
    #[link_name = "pgrx_SpinLockAcquire"]
    pub fn SpinLockAcquire(lock: *mut pg_sys::slock_t);
    #[link_name = "pgrx_SpinLockRelease"]
    pub fn SpinLockRelease(lock: *mut pg_sys::slock_t);
    #[link_name = "pgrx_SpinLockFree"]
    pub fn SpinLockFree(lock: *mut pg_sys::slock_t) -> bool;
}
