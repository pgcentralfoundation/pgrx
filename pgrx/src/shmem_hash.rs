use crate::lwlock::*;
use crate::shmem::PgSharedMemoryInitialization;
use crate::PgSharedMem;
use crate::{pg_sys, PGRXSharedMemory};
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

    pub fn insert(&self, key: &i64, _value: &i64) {
        let htab = self.htab.exclusive();
        let void_ptr: *const core::ffi::c_void = key as *const _ as *const core::ffi::c_void;
        let mut found = false;

        let _entry = unsafe {
            pg_sys::hash_search(htab.htab, void_ptr, pg_sys::HASHACTION_HASH_ENTER_NULL, &mut found)
        };
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
        hash_ctl.keysize = std::mem::size_of::<i64>();
        hash_ctl.entrysize = std::mem::size_of::<i64>();

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
