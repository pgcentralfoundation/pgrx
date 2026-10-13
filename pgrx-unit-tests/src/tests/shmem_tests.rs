//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::lwlock::dsm::DsmLwLockTranche;
use pgrx::lwlock::scan::ParallelScanLwLockTranche;
use pgrx::prelude::*;
#[cfg(feature = "cshim")]
use pgrx::spinlock::PgSpinLock;
use pgrx::{PgAtomic, PgLwLock, pg_shmem_init};
use std::sync::atomic::AtomicBool;

static ATOMIC: PgAtomic<AtomicBool> = unsafe { PgAtomic::new(c"pgrx_unit_tests_atomic") };
static LWLOCK: PgLwLock<bool> = unsafe { PgLwLock::new(c"pgrx_unit_tests_lwlock") };

#[cfg(feature = "cshim")]
static SPINLOCK: PgAtomic<PgSpinLock<usize>> =
    unsafe { PgAtomic::new(c"pgrx_unit_tests_spinlock") };

static DSMLWLOCK: DsmLwLockTranche = DsmLwLockTranche::new(c"pgrx_tests_dsm_lwlock");
static SCANLWLOCK: ParallelScanLwLockTranche =
    ParallelScanLwLockTranche::new(c"pgrx_tests_scan_lwlock");

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    // This ensures that this functionality works across PostgreSQL versions
    pg_shmem_init!(ATOMIC);
    pg_shmem_init!(LWLOCK);

    #[cfg(feature = "cshim")]
    pg_shmem_init!(SPINLOCK = PgSpinLock::new(0));

    pg_shmem_init!(DSMLWLOCK);
    pg_shmem_init!(SCANLWLOCK);
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use pgrx::lwlock::dsm::DsmLwLock;
    use pgrx::lwlock::dsm::DsmLwLockHandle;
    use pgrx::lwlock::scan::ParallelScanLwLock;
    use pgrx::prelude::*;

    #[pg_test]
    #[should_panic(expected = "cache lookup failed for type 0")]
    pub fn test_behaves_normally_when_elog_while_holding_lock() {
        use super::LWLOCK;
        // Hold lock
        let _lock = LWLOCK.exclusive();
        // Call into pg_guarded postgres function which internally reports an error
        unsafe { pg_sys::format_type_extended(pg_sys::InvalidOid, -1, 0) };
    }

    #[pg_test]
    pub fn test_lock_is_released_on_drop() {
        use super::LWLOCK;
        let lock = LWLOCK.exclusive();
        drop(lock);
        let _lock = LWLOCK.exclusive();
    }

    #[pg_test]
    pub fn test_lock_is_released_on_unwind() {
        use super::LWLOCK;
        let _res = std::panic::catch_unwind(|| {
            let _lock = LWLOCK.exclusive();
            panic!("get out")
        });
        let _lock = LWLOCK.exclusive();
    }

    #[cfg(feature = "cshim")]
    #[pg_test]
    pub fn test_spinlock() {
        use super::SPINLOCK;
        for i in 0..10 {
            let mut lock = SPINLOCK.get().lock();
            assert!(*lock == i);
            *lock = i + 1;
            drop(lock);
        }
    }

    /// The segment is owned by the transaction resource owner and gets detached on transaction end.
    /// In an ordinary use case, we would expect a single, statically allocated shared memory area.
    /// In the case of concurrent tests, a statically allocated shared memory area would be
    /// re-initialized by `LWLockInitialize` while another backend is holding it.
    /// Therefore, here we create a private DSM segment for each test to guarantee that it is
    /// initialized exactly once.
    unsafe fn provide_dsm_for_test(size: usize) -> *mut std::ffi::c_void {
        let segment = pg_sys::dsm_create(size, 0);
        pg_sys::dsm_segment_address(segment)
    }

    fn init_dsm_lwlock() -> DsmLwLockHandle<bool> {
        use super::DSMLWLOCK;
        let data: bool = false;
        unsafe {
            let shmem = provide_dsm_for_test(DsmLwLock::<bool>::mem_size());
            DSMLWLOCK.init(shmem, &data)
        }
    }

    #[pg_test]
    #[should_panic(expected = "dynamic shared memory is not aligned")]
    pub fn dsm_test_init_rejects_misaligned_memory() {
        use super::DSMLWLOCK;
        let data: bool = false;
        unsafe {
            let shmem = provide_dsm_for_test(DsmLwLock::<bool>::mem_size() + 1);
            DSMLWLOCK.init(shmem.byte_add(1), &data);
        }
    }

    #[pg_test]
    #[should_panic(expected = "cache lookup failed for type 0")]
    pub fn dsm_test_behaves_normally_when_elog_while_holding_lock() {
        let handle = init_dsm_lwlock();
        let _lock = handle.exclusive();
        // Call into pg_guarded postgres function which internally reports an error
        unsafe { pg_sys::format_type_extended(pg_sys::InvalidOid, -1, 0) };
    }

    #[pg_test]
    pub fn dsm_test_lock_is_released_on_drop() {
        let handle = init_dsm_lwlock();
        let lock = handle.exclusive();
        drop(lock);
        let _lock = handle.exclusive();
    }

    #[pg_test]
    pub fn dsm_test_lock_is_released_on_unwind() {
        let handle = init_dsm_lwlock();
        let _res = std::panic::catch_unwind(|| {
            let _lock = handle.exclusive();
            panic!("get out")
        });
        let _lock = handle.exclusive();
    }

    fn init_scan_lwlock() -> ParallelScanLwLock<bool> {
        use super::SCANLWLOCK;
        let data: bool = false;
        let mut lock = SCANLWLOCK.lock_for(data);
        unsafe {
            let shmem = provide_dsm_for_test(ParallelScanLwLock::<bool>::mem_size());
            lock.initialize_dsm_and_register_leader(shmem);
        }
        lock
    }

    #[pg_test]
    #[should_panic(expected = "cache lookup failed for type 0")]
    pub fn scan_test_behaves_normally_when_elog_while_holding_lock() {
        let mut handle = init_scan_lwlock();
        let _lock = handle.exclusive();
        // Call into pg_guarded postgres function which internally reports an error
        unsafe { pg_sys::format_type_extended(pg_sys::InvalidOid, -1, 0) };
    }

    #[pg_test]
    pub fn scan_test_lock_is_released_on_drop() {
        let mut handle = init_scan_lwlock();
        let lock = handle.exclusive();
        drop(lock);
        let _lock = handle.exclusive();
    }

    #[pg_test]
    pub fn scan_test_lock_is_released_on_unwind() {
        let mut handle = init_scan_lwlock();
        let handle_ref = std::panic::AssertUnwindSafe(&handle);
        let _res = std::panic::catch_unwind(|| {
            let _lock = handle_ref.shared();
            panic!("get out")
        });
        let _lock = handle.exclusive();
    }
}
