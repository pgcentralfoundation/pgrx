#![cfg(feature = "cshim")]
#![allow(deprecated)]

#[cfg(not(feature = "pg19"))]
use crate as pg_sys;

// On Postgres 19+ the SpinLock functions are `static inline` in `storage/spin.h`, so
// bindgen wraps them automatically and they come in through the generated bindings.
// (`SpinLockFree` was removed from Postgres entirely in v19.)
#[cfg(not(feature = "pg19"))]
#[pgrx_macros::pg_guard]
unsafe extern "C-unwind" {
    #[link_name = "SpinLockInit__pgrx_cshim"]
    pub fn SpinLockInit(lock: *mut pg_sys::slock_t);
    #[link_name = "SpinLockAcquire__pgrx_cshim"]
    pub fn SpinLockAcquire(lock: *mut pg_sys::slock_t);
    #[link_name = "SpinLockRelease__pgrx_cshim"]
    pub fn SpinLockRelease(lock: *mut pg_sys::slock_t);
    #[link_name = "SpinLockFree__pgrx_cshim"]
    pub fn SpinLockFree(lock: *mut pg_sys::slock_t) -> bool;
}
