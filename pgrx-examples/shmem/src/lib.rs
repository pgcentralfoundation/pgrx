/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgrx::atomics::*;
use pgrx::lwlock::PgLwLock;
use pgrx::prelude::*;
use pgrx::shmem::*;
use pgrx::{pg_shmem_init, warning};
use serde::*;
use std::iter::Iterator;
use std::sync::atomic::Ordering;

pgrx::pg_module_magic!();

// types behind a `LwLock` must derive/implement `Copy` and `Clone`
#[derive(Copy, Clone)]
// This is for general Postgres type support -- not strictly necessary if the type is not exposed via SQL
#[derive(PostgresType, Serialize, Deserialize)]
pub struct Pgtest {
    value1: i32,
    value2: i32,
}
impl Default for Pgtest {
    fn default() -> Self {
        Pgtest { value1: 0, value2: 0 }
    }
}
unsafe impl PGRXSharedMemory for Pgtest {}

static VEC: PgLwLock<heapless::Vec<Pgtest, 400>> = PgLwLock::new();
static HASH: PgLwLock<heapless::FnvIndexMap<i32, i32, 4>> = PgLwLock::new();
static STRUCT: PgLwLock<Pgtest> = PgLwLock::new();
static PRIMITIVE: PgLwLock<i32> = PgLwLock::new();
static ATOMIC: PgAtomic<std::sync::atomic::AtomicBool> = PgAtomic::new();

#[pg_guard]
pub extern "C" fn _PG_init() {
    pg_shmem_init!(VEC);
    pg_shmem_init!(HASH);
    pg_shmem_init!(STRUCT);
    pg_shmem_init!(PRIMITIVE);
    pg_shmem_init!(ATOMIC);
}

#[pg_extern]
fn vec_select() -> SetOfIterator<'static, Pgtest> {
    SetOfIterator::new(VEC.share().iter().map(|i| *i).collect::<Vec<Pgtest>>().into_iter())
}

#[pg_extern]
fn vec_count() -> i32 {
    VEC.share().len() as i32
}

#[pg_extern]
fn vec_drain() -> SetOfIterator<'static, Pgtest> {
    let mut vec = VEC.exclusive();
    let r = vec.iter().map(|i| *i).collect::<Vec<Pgtest>>();
    vec.clear();
    SetOfIterator::new(r.into_iter())
}

#[pg_extern]
fn vec_push(value: Pgtest) {
    VEC.exclusive().push(value).unwrap_or_else(|_| warning!("Vector is full, discarding update"));
}

#[pg_extern]
fn vec_pop() -> Option<Pgtest> {
    VEC.exclusive().pop()
}

#[pg_extern]
fn hash_insert(key: i32, value: i32) {
    HASH.exclusive().insert(key, value).unwrap();
}

#[pg_extern]
fn hash_get(key: i32) -> Option<i32> {
    HASH.share().get(&key).cloned()
}

#[pg_extern]
fn struct_get() -> Pgtest {
    STRUCT.share().clone()
}

#[pg_extern]
fn struct_set(value1: i32, value2: i32) {
    *STRUCT.exclusive() = Pgtest { value1, value2 };
}

#[pg_extern]
fn primitive_get() -> i32 {
    PRIMITIVE.share().clone()
}

#[pg_extern]
fn primitive_set(value: i32) {
    *PRIMITIVE.exclusive() = value;
}

#[pg_extern]
fn atomic_get() -> bool {
    ATOMIC.get().load(Ordering::Relaxed)
}

#[pg_extern]
fn atomic_set(value: bool) -> bool {
    ATOMIC.get().swap(value, Ordering::Relaxed)
}
