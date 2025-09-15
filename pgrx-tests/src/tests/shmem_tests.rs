//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::lwlock::dsm::{DsmLwLock, DsmLwLockTranche};
use pgrx::lwlock::scan::{ParallelScanLwLock, ParallelScanLwLockTranche};
use pgrx::prelude::*;
#[cfg(feature = "cshim")]
use pgrx::spinlock::PgSpinLock;
use pgrx::{PgAtomic, PgLwLock, pg_shmem_init};
use std::sync::atomic::AtomicBool;

static ATOMIC: PgAtomic<AtomicBool> = unsafe { PgAtomic::new(c"pgrx_tests_atomic") };
static LWLOCK: PgLwLock<bool> = unsafe { PgLwLock::new(c"pgrx_tests_lwlock") };

#[cfg(feature = "cshim")]
static SPINLOCK: PgAtomic<PgSpinLock<usize>> = unsafe { PgAtomic::new(c"pgrx_tests_spinlock") };

static DSMLWLOCK: DsmLwLockTranche = DsmLwLockTranche::new(c"pgrx_tests_dsm_lwlock");
static DSMLWLOCKMEM: TestDSM =
    unsafe { TestDSM::new(DsmLwLock::<bool>::mem_size(), c"pgrx_tests_dsm_lwlock_mem") };
static SCANLWLOCK: ParallelScanLwLockTranche =
    ParallelScanLwLockTranche::new(c"pgrx_tests_scan_lwlock");
static SCANLWLOCKMEM: TestDSM =
    unsafe { TestDSM::new(ParallelScanLwLock::<bool>::mem_size(), c"pgrx_tests_scan_lwlock_mem") };

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    // This ensures that this functionality works across PostgreSQL versions
    pg_shmem_init!(ATOMIC);
    pg_shmem_init!(LWLOCK);

    #[cfg(feature = "cshim")]
    pg_shmem_init!(SPINLOCK = PgSpinLock::new(0));

    pg_shmem_init!(DSMLWLOCK);
    pg_shmem_init!(DSMLWLOCKMEM);
    pg_shmem_init!(SCANLWLOCK);
    pg_shmem_init!(SCANLWLOCKMEM);
}

// Allocates just plain shared memory.
// TODO: Should be easier by using GetNamedDSMSegment when its bindings are included.
struct TestDSM {
    size: usize,
    name: &'static std::ffi::CStr,
    inner: std::cell::UnsafeCell<*mut std::ffi::c_void>,
}

impl TestDSM {
    pub const unsafe fn new(size: usize, name: &'static std::ffi::CStr) -> Self {
        Self { size, name, inner: std::cell::UnsafeCell::new(std::ptr::null_mut()) }
    }

    pub unsafe fn mem(&self) -> *mut std::ffi::c_void {
        *(self.inner.get())
    }
}

unsafe impl Sync for TestDSM {}

impl pgrx::PgSharedMemoryInitialization for TestDSM {
    type Value = ();

    unsafe fn on_shmem_request(&'static self) {
        unsafe {
            pgrx::pg_sys::RequestAddinShmemSpace(self.size);
        }
    }

    unsafe fn on_shmem_startup(&'static self, _value: ()) {
        unsafe {
            use pgrx::pg_sys;

            let shm_name = self.name;
            let addin_shmem_init_lock = &raw mut (*pg_sys::MainLWLockArray.add(21)).lock;
            pg_sys::LWLockAcquire(addin_shmem_init_lock, pg_sys::LWLockMode::LW_EXCLUSIVE);

            let mut found = false;
            let fv_shmem = pg_sys::ShmemInitStruct(shm_name.as_ptr(), self.size, &mut found);
            assert!(fv_shmem.is_aligned(), "shared memory is not aligned");

            *self.inner.get() = fv_shmem;

            pg_sys::LWLockRelease(addin_shmem_init_lock);
        }
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

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

    fn init_dsm_lwlock() -> DsmLwLockHandle<bool> {
        use super::{DSMLWLOCK, DSMLWLOCKMEM};
        let data: bool = false;
        unsafe {
            let shmem = DSMLWLOCKMEM.mem();
            DSMLWLOCK.init(shmem, &data as *const bool);
            DSMLWLOCK.register(shmem)
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
        use super::{SCANLWLOCK, SCANLWLOCKMEM};
        let data: bool = false;
        let mut lock = SCANLWLOCK.lock_for(data);
        unsafe {
            lock.initialize_dsm_and_register_leader(SCANLWLOCKMEM.mem());
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
        let handle_ref = &handle;
        let _res = std::panic::catch_unwind(|| {
            let _lock = handle_ref.shared();
            panic!("get out")
        });
        let _lock = handle.exclusive();
    }
}
