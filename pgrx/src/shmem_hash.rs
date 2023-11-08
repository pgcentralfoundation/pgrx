use crate::lwlock::*;
use crate::shmem::PgSharedMemoryInitialization;
use crate::PgSharedMem;
use crate::{pg_sys, PGRXSharedMemory};
use std::ffi::c_void;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    HashTableFull,
}

#[derive(Copy, Clone)]
pub struct PgHashMapInner {
    htab: *mut pg_sys::HTAB,
}

unsafe impl PGRXSharedMemory for PgHashMapInner {}
unsafe impl Send for PgHashMapInner {}
unsafe impl Sync for PgHashMapInner {}

#[repr(align(8))]
#[derive(Copy, Clone, Debug)]
struct Key {
    key: i64,
}

#[repr(align(8))]
#[derive(Copy, Clone, Debug)]
struct Value {
    value: i64,
}

impl Default for PgHashMapInner {
    fn default() -> Self {
        Self { htab: std::ptr::null_mut() }
    }
}

pub struct PgHashMap {
    htab: PgLwLock<PgHashMapInner>,
    size: u64,
}

impl PgHashMap {
    pub const fn new(size: u64) -> PgHashMap {
        PgHashMap { htab: PgLwLock::new(), size }
    }

    pub fn insert(&self, key: i64, value: i64) {
        let htab = self.htab.exclusive();
        let mut found = false;

        let key_value = Key { key };
        let key_ptr: *const c_void = std::ptr::addr_of!(key_value) as *const Key as *const c_void;

        let entry = unsafe {
            pg_sys::hash_search(htab.htab, key_ptr, pg_sys::HASHACTION_HASH_ENTER, &mut found)
        };

        if !entry.is_null() {
            let value_ptr: *mut Value = entry as *mut Value;
            unsafe {
                std::ptr::write(value_ptr, Value { value });
            }
        }
    }
}

impl PgSharedMemoryInitialization for PgHashMap {
    fn pg_init(&'static self) {
        PgSharedMem::pg_init_locked(&self.htab);
    }

    fn shmem_init(&'static self) {
        PgSharedMem::shmem_init_locked(&self.htab);
        let mut htab = self.htab.exclusive();

        let mut hash_ctl = pg_sys::HASHCTL::default();
        hash_ctl.keysize = std::mem::size_of::<Key>();
        hash_ctl.entrysize = std::mem::size_of::<Value>();

        let shm_name =
            alloc::ffi::CString::new(Uuid::new_v4().to_string()).expect("CString::new() failed");

        let htab_ptr = unsafe {
            pg_sys::ShmemInitHash(
                shm_name.into_raw(),
                self.size.try_into().unwrap(),
                self.size.try_into().unwrap(),
                &mut hash_ctl,
                (pg_sys::HASH_ELEM | pg_sys::HASH_BLOBS).try_into().unwrap(),
            )
        };

        htab.htab = htab_ptr;
    }
}
