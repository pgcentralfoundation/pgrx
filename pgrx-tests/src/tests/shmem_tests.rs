/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgrx::prelude::*;
use pgrx::{pg_shmem_init, PgAtomic, PgLwLock, PgSharedMemoryInitialization};
use std::sync::atomic::AtomicBool;

static ATOMIC: PgAtomic<AtomicBool> = PgAtomic::new();
static LWLOCK: PgLwLock<bool> = PgLwLock::new();

#[pg_guard]
pub extern "C" fn _PG_init() {
    // This ensures that this functionality works across PostgreSQL versions
    pg_shmem_init!(ATOMIC);
    pg_shmem_init!(LWLOCK);
}
#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use crate::tests::shmem_tests::LWLOCK;
    use pgrx::prelude::*;

    #[pg_test]
    #[should_panic(expected = "cache lookup failed for type 0")]
    pub fn test_behaves_normally_when_elog_while_holding_lock() {
        // Hold lock
        let _lock = LWLOCK.exclusive();
        // Call into pg_guarded postgres function which internally reports an error
        unsafe { pg_sys::format_type_extended(pg_sys::InvalidOid, -1, 0) };
    }

    #[pg_test]
    pub fn test_lock_is_released_on_drop() {
        let lock = LWLOCK.exclusive();
        drop(lock);
        let _lock = LWLOCK.exclusive();
    }

    #[pg_test]
    pub fn test_lock_is_released_on_unwind() {
        let _res = std::panic::catch_unwind(|| {
            let _lock = LWLOCK.exclusive();
            panic!("get out")
        });
        let _lock = LWLOCK.exclusive();
    }
}
