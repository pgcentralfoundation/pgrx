use heapless::consts::*;
use pgx::lwlock::*;
use pgx::shmem::*;
use pgx::*;
use serde::*;
use std::iter::Iterator;
use std::sync::atomic::Ordering;

pub static mut PREV_SHMEM_STARTUP_HOOK: Option<unsafe extern "C" fn()> = None;

pg_module_magic!();

pgx_sharedmem_locked!(VEC, heapless::Vec<Pgtest, U400>);
pgx_sharedmem_locked!(HASH, heapless::FnvIndexMap<i32, i32, U4>);
pgx_sharedmem_locked!(STRUCT, Pgtest);
pgx_sharedmem_locked!(PRIMATIVE, i32);
pgx_sharedmem_atomic!(ATOMIC, std::sync::atomic::AtomicBool = true);

#[derive(PostgresType, Serialize, Deserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Pgtest {
    value1: i32,
    value2: i32,
}
impl Default for Pgtest {
    fn default() -> Self {
        Pgtest {
            value1: 0,
            value2: 0,
        }
    }
}

unsafe impl PGXSharedMemory for Pgtest {}

#[allow(non_snake_case)]
#[pg_guard]
pub extern "C" fn _PG_init() {
    PgSharedMem::pg_init_locked(&VEC);
    PgSharedMem::pg_init_locked(&HASH);
    PgSharedMem::pg_init_locked(&STRUCT);
    PgSharedMem::pg_init_locked(&PRIMATIVE);
    PgSharedMem::pg_init_atomic(&ATOMIC);
    unsafe {
        PREV_SHMEM_STARTUP_HOOK = pg_sys::shmem_startup_hook;
        pg_sys::shmem_startup_hook = Some(pg_shmem_init);
    }
}

//#[no_mangle]
extern "C" fn pg_shmem_init() {
    unsafe {
        if let Some(i) = PREV_SHMEM_STARTUP_HOOK {
            i();
        }
    }
    PgSharedMem::shmem_init_locked(&VEC);
    PgSharedMem::shmem_init_locked(&HASH);
    PgSharedMem::shmem_init_locked(&STRUCT);
    PgSharedMem::shmem_init_locked(&PRIMATIVE);
    PgSharedMem::shmem_init_atomic(&ATOMIC);
}

#[pg_extern]
fn vec_select() -> impl Iterator<Item = Pgtest> {
    VEC.with(|s| s.share().iter().map(|i| *i).collect::<Vec<Pgtest>>())
        .into_iter()
}

#[pg_extern]
fn vec_count() -> i32 {
    VEC.with(|s| s.share().len() as i32)
}

#[pg_extern]
fn vec_drain() -> impl Iterator<Item = Pgtest> {
    VEC.with(|s| {
        let mut vec = s.exclusive();
        let r = vec.iter().map(|i| *i).collect::<Vec<Pgtest>>();
        vec.clear();
        r
    })
    .into_iter()
}

#[pg_extern]
fn vec_push(value: Pgtest) {
    VEC.with(|s| {
        s.exclusive()
            .push(value)
            .unwrap_or_else(|_| warning!("Vector is full, discarding update"));
    });
}

#[pg_extern]
fn vec_pop() -> Option<Pgtest> {
    VEC.with(|s| s.exclusive().pop())
}

#[pg_extern]
fn hash_insert(key: i32, value: i32) {
    HASH.with(|s| s.exclusive().insert(key, value).unwrap());
}

#[pg_extern]
fn hash_get(key: i32) -> Option<i32> {
    HASH.with(|s| s.share().get(&key).cloned())
}

#[pg_extern]
fn struct_get() -> Pgtest {
    STRUCT.with(|s| s.share().clone())
}

#[pg_extern]
fn struct_set(value1: i32, value2: i32) {
    STRUCT.with(|s| **s.exclusive() = Pgtest { value1, value2 });
}

#[pg_extern]
fn primative_get() -> i32 {
    PRIMATIVE.with(|s| s.share().clone())
}

#[pg_extern]
fn primative_set(value: i32) {
    PRIMATIVE.with(|s| **s.exclusive() = value);
}
#[pg_extern]
fn atomic_get() -> bool {
    ATOMIC.load(Ordering::Relaxed).clone()
}

#[pg_extern]
fn atomic_set(value: bool) {
    ATOMIC.store(value, Ordering::Relaxed);
}
