use pgrx::prelude::*;
use pgrx::{pg_shmem_init, PgHashMap, PgSharedMemoryInitialization};

static HASH_MAP: PgHashMap<i64, i64> = PgHashMap::new(250);

#[pg_guard]
pub extern "C" fn _PG_init() {
    // This ensures that this functionality works across PostgreSQL versions
    pg_shmem_init!(HASH_MAP);
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use super::*;

    #[pg_test]
    pub fn test_insert() {
        for i in 0..250 {
            assert_eq!(HASH_MAP.insert(i, i), Ok(None));
        }
    }
}
