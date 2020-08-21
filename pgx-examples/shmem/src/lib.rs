use heapless::consts::*;
use pgx::lwlock::*;
use pgx::shmem::*;
use pgx::*;
use serde::*;
use std::iter::Iterator;
use std::sync::atomic::Ordering;

pub static mut PREV_SHMEM_STARTUP_HOOK: Option<unsafe extern "C" fn()> = None;

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

// derive macro would generate all this code {
// #[derive(PostgresSharedMemoryContext(MY_CONTEXT))]
struct SharedMemoryContext {
    vec: PgLwLock<heapless::Vec<Pgtest, U400>>,
    hash: PgLwLock<heapless::FnvIndexMap<i32, i32, U4>>,
    struct_: PgLwLock<Pgtest>,
    primitive: PgLwLock<i32>,
    atomic: std::sync::atomic::AtomicBool,
}

impl Default for SharedMemoryContext {
    fn default() -> Self {
        SharedMemoryContext {
            vec: PgLwLock::empty("vec"),
            hash: PgLwLock::empty("hash"),
            struct_: PgLwLock::empty("struct_"),
            primitive: PgLwLock::empty("primitive"),
            atomic: Default::default(),
        }
    }
}

thread_local! {
    static MY_CONTEXT: SharedMemoryContext = SharedMemoryContext::default();
}

fn _pg_init_shmem() {
    MY_CONTEXT.with(|value| {
        PgSharedMem::pg_init_locked(&value.vec);
        PgSharedMem::pg_init_locked(&value.hash);
        PgSharedMem::pg_init_locked(&value.struct_);
        PgSharedMem::pg_init_locked(&value.primitive);
        PgSharedMem::pg_init_atomic(&value.atomic);
    });
}

fn _pg_shmem_hook_init() {
    MY_CONTEXT.with(|value| {
        PgSharedMem::shmem_init_locked(&value.vec);
        PgSharedMem::shmem_init_locked(&value.hash);
        PgSharedMem::shmem_init_locked(&value.struct_);
        PgSharedMem::shmem_init_locked(&value.primitive);
        PgSharedMem::shmem_init_atomic(&value.atomic);
    });
}

// }

#[allow(non_snake_case)]
#[pg_guard]
pub extern "C" fn _PG_init() {
    _pg_init_shmem();

    // and we'd make a regular macro to generate this
    //   pg_shared_memory_init()
    unsafe {
        PREV_SHMEM_STARTUP_HOOK = pg_sys::shmem_startup_hook;
        pg_sys::shmem_startup_hook = Some(shmem_hook);

        #[pg_guard]
        extern "C" fn shmem_hook() {
            unsafe {
                if let Some(i) = PREV_SHMEM_STARTUP_HOOK {
                    i();
                }
            }
            _pg_shmem_hook_init();
        }
    }
}

#[pg_extern]
fn vec_select() -> impl Iterator<Item = Pgtest> {
    MY_CONTEXT
        .with(|s| s.vec.share().iter().map(|i| *i).collect::<Vec<Pgtest>>())
        .into_iter()
}

#[pg_extern]
fn vec_count() -> i32 {
    MY_CONTEXT.with(|s| s.vec.share().len() as i32)
}

#[pg_extern]
fn vec_drain() -> impl Iterator<Item = Pgtest> {
    MY_CONTEXT
        .with(|s| {
            let mut vec = s.vec.exclusive();
            let r = vec.iter().map(|i| *i).collect::<Vec<Pgtest>>();
            vec.clear();
            r
        })
        .into_iter()
}

#[pg_extern]
fn vec_push(value: Pgtest) {
    MY_CONTEXT.with(|s| {
        s.vec
            .exclusive()
            .push(value)
            .unwrap_or_else(|_| warning!("Vector is full, discarding update"));
    });
}

#[pg_extern]
fn vec_pop() -> Option<Pgtest> {
    MY_CONTEXT.with(|s| s.vec.exclusive().pop())
}

// #[pg_extern]
// fn hash_insert(key: i32, value: i32) {
//     HASH.with(|s| s.exclusive().insert(key, value).unwrap());
// }
//
// #[pg_extern]
// fn hash_get(key: i32) -> Option<i32> {
//     HASH.with(|s| s.share().get(&key).cloned())
// }
//
// #[pg_extern]
// fn struct_get() -> Pgtest {
//     STRUCT.with(|s| s.share().clone())
// }
//
// #[pg_extern]
// fn struct_set(value1: i32, value2: i32) {
//     STRUCT.with(|s| **s.exclusive() = Pgtest { value1, value2 });
// }
//
// #[pg_extern]
// fn primative_get() -> i32 {
//     PRIMATIVE.with(|s| s.share().clone())
// }
//
// #[pg_extern]
// fn primative_set(value: i32) {
//     PRIMATIVE.with(|s| **s.exclusive() = value);
// }
// #[pg_extern]
// fn atomic_get() -> bool {
//     ATOMIC.load(Ordering::Relaxed).clone()
// }
//
// #[pg_extern]
// fn atomic_set(value: bool) {
//     ATOMIC.store(value, Ordering::Relaxed);
// }
