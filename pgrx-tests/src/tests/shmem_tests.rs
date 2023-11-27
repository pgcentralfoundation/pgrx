//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::prelude::*;
use pgrx::{pg_shmem_init, PgAtomic, PgLwLock, PgSharedMemoryInitialization};
use std::sync::atomic::AtomicBool;

#[cfg(feature = "cshim")]
use pgrx::PgHashMap;

static ATOMIC: PgAtomic<AtomicBool> = PgAtomic::new();
static LWLOCK: PgLwLock<bool> = PgLwLock::new();

#[cfg(feature = "cshim")]
static HASH_MAP: PgHashMap<i64, i64> = PgHashMap::new(500);

#[pg_guard]
pub extern "C" fn _PG_init() {
    // This ensures that this functionality works across PostgreSQL versions
    pg_shmem_init!(ATOMIC);
    pg_shmem_init!(LWLOCK);

    #[cfg(feature = "cshim")]
    pg_shmem_init!(HASH_MAP);
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    #[cfg(feature = "cshim")]
    use crate::tests::shmem_tests::HASH_MAP;
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

    #[cfg(feature = "cshim")]
    #[pg_test]
    pub fn test_pg_hash_map() {
        use rand::prelude::IteratorRandom;

        for i in 1..250 {
            assert_eq!(HASH_MAP.insert(i, i), Ok(None));
        }

        assert_eq!(HASH_MAP.len(), 249);

        for i in 1..250 {
            assert_eq!(HASH_MAP.get(i), Some(i));
        }

        assert_eq!(HASH_MAP.len(), 249);

        for i in 251..500 {
            assert_eq!(HASH_MAP.get(i), None);
        }

        assert_eq!(HASH_MAP.len(), 249);

        for i in 1..250 {
            assert_eq!(HASH_MAP.insert(i, i), Ok(Some(i)));
        }

        assert_eq!(HASH_MAP.len(), 249);

        for i in 1..250 {
            assert_eq!(HASH_MAP.remove(i), Some(i));
        }

        assert_eq!(HASH_MAP.len(), 0);

        for i in 1..250 {
            assert_eq!(HASH_MAP.get(i), None);
        }

        assert_eq!(HASH_MAP.len(), 0);

        for _ in 0..25_000 {
            for key in 0..250 {
                let value = (0..1000).choose(&mut rand::thread_rng()).unwrap();
                assert!(HASH_MAP.insert(key, value).is_ok());
            }
        }

        assert_eq!(HASH_MAP.len(), 250);
    }
}
