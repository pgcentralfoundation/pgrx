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
    elements: u64,
}

unsafe impl PGRXSharedMemory for PgHashMapInner {}
unsafe impl Send for PgHashMapInner {}
unsafe impl Sync for PgHashMapInner {}

#[repr(align(8))]
#[derive(Copy, Clone, Debug)]
struct Key<K> {
    // We copy it with std::ptr::copy, but we don't actually use the field
    // in Rust, hence the warning.
    #[allow(dead_code)]
    key: K,
}

#[repr(align(8))]
#[derive(Copy, Clone, Debug)]
struct Value<V> {
    value: V,
}

impl Default for PgHashMapInner {
    fn default() -> Self {
        Self { htab: std::ptr::null_mut(), elements: 0 }
    }
}

pub struct PgHashMap<K: Copy + Clone, V: Copy + Clone> {
    htab: PgLwLock<PgHashMapInner>,
    size: u64,
    phantom_key: std::marker::PhantomData<K>,
    phantom_value: std::marker::PhantomData<V>,
}

/// Compute the hash for the key and it's pointer
/// to pass to the hash_search. Lock on HTAB should be taken,
/// although not strictly required I think.
macro_rules! key {
    ($key:expr, $htab:expr) => {{
        let key = Key { key: $key };
        let key_ptr: *const c_void = std::ptr::addr_of!(key) as *const Key<K> as *const c_void;
        let hash_value = unsafe { pg_sys::get_hash_value($htab.htab, key_ptr) };

        (key_ptr, hash_value)
    }};
}

/// Get the value pointer. It's stored next to the key.
/// https://github.com/postgres/postgres/blob/1f998863b0bc6fc8ef3d971d9c6d2c29b52d8ba2/src/backend/utils/hash/dynahash.c#L246-L250
macro_rules! value_ptr {
    ($entry:expr) => {{
        let value_ptr: *mut Value<V> =
            unsafe { $entry.offset(std::mem::size_of::<Key<K>>().try_into().unwrap()) }
                as *mut Value<V>;

        value_ptr
    }};
}

impl<K: Copy + Clone, V: Copy + Clone> PgHashMap<K, V> {
    /// Create new PgHashMap. This still needs to be allocated with
    /// `pg_shmem_init!` just like any other shared memory structure.
    pub const fn new(size: u64) -> PgHashMap<K, V> {
        PgHashMap {
            htab: PgLwLock::new(),
            size,
            phantom_key: std::marker::PhantomData,
            phantom_value: std::marker::PhantomData,
        }
    }

    /// Insert a key and value into the HashMap. If the key is already
    /// present, it will be replaced. If the HashMap is full, return an error.
    pub fn insert(&self, key: K, value: V) -> Result<(), Error> {
        let mut found = false;
        let mut htab = self.htab.exclusive();
        let (key_ptr, hash_value) = key!(key, htab);

        let entry = unsafe {
            pg_sys::hash_search_with_hash_value(
                htab.htab,
                key_ptr,
                hash_value,
                pg_sys::HASHACTION_HASH_FIND,
                &mut found,
            )
        };

        // If we don't do this check, pg will overwrite
        // some random entry with our key/value...
        if entry.is_null() && htab.elements == self.size {
            return Err(Error::HashTableFull);
        }

        let entry = unsafe {
            pg_sys::hash_search_with_hash_value(
                htab.htab,
                key_ptr,
                hash_value,
                pg_sys::HASHACTION_HASH_ENTER_NULL,
                &mut found,
            )
        };

        if !entry.is_null() {
            let value_ptr = value_ptr!(entry);
            let value = Value { value };
            unsafe {
                std::ptr::copy(std::ptr::addr_of!(value), value_ptr, 1);
            }
            htab.elements += 1;
            Ok(())
        } else {
            // OOM.
            return Err(Error::HashTableFull);
        }
    }

    /// Get a value from the HashMap using the key.
    /// If the key doesn't exist, return None.
    pub fn get(&self, key: K) -> Option<V> {
        let htab = self.htab.exclusive();
        let (key_ptr, hash_value) = key!(key, htab);

        let entry = unsafe {
            pg_sys::hash_search_with_hash_value(
                htab.htab,
                key_ptr,
                hash_value,
                pg_sys::HASHACTION_HASH_FIND,
                std::ptr::null_mut(),
            )
        };

        if entry.is_null() {
            return None;
        } else {
            let value_ptr = value_ptr!(entry);
            let value = unsafe { std::ptr::read(value_ptr) };
            return Some(value.value);
        }
    }

    /// Remove the value from the HashMap and return it.
    pub fn remove(&self, key: K) -> Option<V> {
        if let Some(value) = self.get(key) {
            let mut htab = self.htab.exclusive();
            let (key_ptr, hash_value) = key!(key, htab);

            // Dangling pointer, don't touch it.
            let _ = unsafe {
                pg_sys::hash_search_with_hash_value(
                    htab.htab,
                    key_ptr,
                    hash_value,
                    pg_sys::HASHACTION_HASH_REMOVE,
                    std::ptr::null_mut(),
                );
            };

            htab.elements -= 1;
            return Some(value);
        } else {
            return None;
        }
    }

    /// Get the number of elements in the HashMap.
    pub fn len(&self) -> usize {
        let htab = self.htab.exclusive();
        htab.elements.try_into().unwrap()
    }
}

impl<K: Copy + Clone, V: Copy + Clone> PgSharedMemoryInitialization for PgHashMap<K, V> {
    fn pg_init(&'static self) {
        PgSharedMem::pg_init_locked(&self.htab);
    }

    fn shmem_init(&'static self) {
        PgSharedMem::shmem_init_locked(&self.htab);
        let mut htab = self.htab.exclusive();

        let mut hash_ctl = pg_sys::HASHCTL::default();
        hash_ctl.keysize = std::mem::size_of::<Key<K>>();
        hash_ctl.entrysize = std::mem::size_of::<Value<V>>();

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
