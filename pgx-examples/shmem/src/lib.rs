use heapless::consts::*;
use pgx::lwlock::*;
use pgx::shmem::*;
use pgx::*;
use serde::*;
use std::iter::Iterator;
use std::sync::atomic::Ordering;

pg_module_magic!();

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

thread_local! {
    static VEC: PgLwLock<heapless::Vec<Pgtest, U400>> = PgLwLock::new();
    static HASH: PgLwLock<heapless::FnvIndexMap<i32, i32, U4>> = PgLwLock::new();
    static STRUCT: PgLwLock<Pgtest> = PgLwLock::new();
    static PRIMITIVE: PgLwLock<i32> = PgLwLock::new();
    static ATOMIC: PgAtomic<std::sync::atomic::AtomicBool, bool> = PgAtomic::new(true);
    static ATOMIC_INT32: PgAtomic<std::sync::atomic::AtomicI32, i32> = PgAtomic::new(42);

}

static ATOMIC_FANCY: PgAtomicFancy<std::sync::atomic::AtomicBool> = PgAtomicFancy::new();

#[allow(non_snake_case)]
#[pg_guard]
pub extern "C" fn _PG_init() {
    pg_shmem_init!(VEC);
    pg_shmem_init!(HASH);
    pg_shmem_init!(STRUCT);
    pg_shmem_init!(PRIMITIVE);
    pg_shmem_init!(ATOMIC);
    pg_shmem_init!(ATOMIC_INT32);
    pg_shmem_init!(ATOMIC_FANCY);
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
    STRUCT.with(|s| *s.exclusive() = Pgtest { value1, value2 });
}

#[pg_extern]
fn primitive_get() -> i32 {
    PRIMITIVE.with(|s| s.share().clone())
}

#[pg_extern]
fn primitive_set(value: i32) {
    PRIMITIVE.with(|s| *s.exclusive() = value);
}

#[pg_extern]
fn atomic_get() -> bool {
    ATOMIC.with(|s| s.load(Ordering::Relaxed))
}

#[pg_extern]
fn atomic_set(value: bool) {
    ATOMIC.with(|s| s.store(value, Ordering::Relaxed))
}

#[pg_extern]
fn atomic_i32_get() -> i32 {
    ATOMIC_INT32.with(|s| s.load(Ordering::Relaxed))
}

#[pg_extern]
fn atomic_i32_set(value: i32) -> i32 {
    ATOMIC_INT32.with(|s| s.swap(value, Ordering::Relaxed))
}

#[pg_extern]
fn atomic_fancy_get() -> bool {
    ATOMIC_FANCY.get().load(Ordering::Relaxed)
}

#[pg_extern]
fn atomic_fancy_set(value: bool) -> bool {
    ATOMIC_FANCY.get().swap(value, Ordering::Relaxed)
}
