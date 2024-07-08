//! Shared memory hash map implemented with Postgres' internal `HTAB`,
//! which is used by other extensions like `pg_stat_statements`.
use crate::{pg_sys, shmem::PgSharedMemoryInitialization, spinlock::*, PGRXSharedMemory};
use once_cell::sync::OnceCell;
use std::ffi::c_void;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ShmemHashMapError {
    /// Hash table can't have more entries due to fixed allocation size.
    HashTableFull,
}

#[derive(Copy, Clone, Debug)]
struct ShmemHashMapInner {
    htab: *mut pg_sys::HTAB,
    elements: i64,
}

unsafe impl PGRXSharedMemory for ShmemHashMapInner {}
unsafe impl Send for ShmemHashMapInner {}
unsafe impl Sync for ShmemHashMapInner {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Key<K> {
    // We copy it with std::ptr::copy, but we don't actually use the field
    // in Rust, hence the warning.
    #[allow(dead_code)]
    key: K,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Value<K, V> {
    #[allow(dead_code)]
    key: Key<K>,
    value: V,
}

impl Default for ShmemHashMapInner {
    fn default() -> Self {
        Self { htab: std::ptr::null_mut(), elements: 0 }
    }
}

/// A shared memory HashMap using Postgres' `HTAB`.
/// This HashMap is used for `pg_stat_statements` and Postgres
/// internals to store key/value pairs in shared memory.
pub struct ShmemHashMap<K: Copy + Clone, V: Copy + Clone> {
    /// HTAB protected by a SpinLock.
    htab: OnceCell<PgSpinLock<ShmemHashMapInner>>,

    /// Max size, allocated at server start.
    size: i64,

    // Markers for key/value types.
    _phantom_key: std::marker::PhantomData<K>,
    _phantom_value: std::marker::PhantomData<V>,
}

/// Compute the hash for the key and its pointer
/// to pass to `pg_sys::hash_search_with_hash_value`.
/// Lock on HTAB should be taken, although not strictly required I think.
macro_rules! key {
    ($key:expr, $htab:expr) => {{
        let key = Key { key: $key };
        let key_ptr: *const c_void = std::ptr::addr_of!(key) as *const Key<K> as *const c_void;
        let hash_value = unsafe { pg_sys::get_hash_value($htab.htab, key_ptr) };

        (key_ptr, hash_value)
    }};
}

/// Get the value pointer. It's stored next to the key.
macro_rules! value_ptr {
    ($entry:expr) => {{
        let value_ptr: *mut Value<K, V> = $entry as *mut Value<K, V>;

        value_ptr
    }};
}

impl<K: Copy + Clone, V: Copy + Clone> ShmemHashMap<K, V> {
    /// Create new `ShmemHashMap`. This still needs to be allocated with
    /// `pg_shmem_init!` just like any other shared memory structure.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum number of elements in the HashMap. This is allocated
    ///            at server start and cannot be changed. `i64` is the expected type
    ///            for `pg_sys::ShmemInitHash`, so we don't attempt runtime conversions
    ///            unnecessarily.
    ///
    pub const fn new(size: i64) -> ShmemHashMap<K, V> {
        ShmemHashMap {
            htab: OnceCell::new(),
            size,
            _phantom_key: std::marker::PhantomData,
            _phantom_value: std::marker::PhantomData,
        }
    }

    /// Insert a key and value into the `ShmemHashMap`. If the key is already
    /// present, it will be replaced and returned. If the `ShmemHashMap` is full,
    /// an error is returned.
    pub fn insert(&self, key: K, value: V) -> Result<Option<V>, ShmemHashMapError> {
        let mut found = false;
        let mut htab = self.htab.get().unwrap().lock();
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

        let return_value = if entry.is_null() {
            None
        } else {
            let value_ptr = value_ptr!(entry);
            let value = unsafe { std::ptr::read(value_ptr) };
            Some(value.value)
        };

        // If we don't do this check, pg will overwrite
        // some random entry with our key/value pair...
        if entry.is_null() && htab.elements == self.size {
            return Err(ShmemHashMapError::HashTableFull);
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
            let value = Value { key: Key { key }, value };
            unsafe {
                std::ptr::copy(std::ptr::addr_of!(value), value_ptr, 1);
            }
            // We inserted a new element, increasing the size of the table.
            if return_value.is_none() {
                htab.elements += 1;
            }
            Ok(return_value)
        } else {
            // OOM. We pre-allocate at server start, so this should never be an issue.
            return Err(ShmemHashMapError::HashTableFull);
        }
    }

    /// Get a value from the HashMap using the key.
    /// If the key doesn't exist, return `None`.
    pub fn get(&self, key: K) -> Option<V> {
        let htab = self.htab.get().unwrap().lock();
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

    /// Remove the value from the `ShmemHashMap` and return it.
    /// If the key doesn't exist, return None.
    pub fn remove(&self, key: K) -> Option<V> {
        if let Some(value) = self.get(key) {
            let mut htab = self.htab.get().unwrap().lock();
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
    pub fn len(&self) -> i64 {
        let htab = self.htab.get().unwrap().lock();
        htab.elements
    }
}

impl<K: Copy + Clone, V: Copy + Clone> PgSharedMemoryInitialization for ShmemHashMap<K, V> {
    fn pg_init(&'static self) {
        self.htab
            .set(PgSpinLock::new(ShmemHashMapInner::default()))
            .expect("htab cell is not empty");
    }

    fn shmem_init(&'static self) {
        let mut htab = self.htab.get().unwrap().lock();

        let mut hash_ctl = pg_sys::HASHCTL::default();
        hash_ctl.keysize = std::mem::size_of::<Key<K>>();
        hash_ctl.entrysize = std::mem::size_of::<Value<K, V>>();

        let shm_name =
            alloc::ffi::CString::new(Uuid::new_v4().to_string()).expect("CString::new() failed");

        let htab_ptr = unsafe {
            pg_sys::ShmemInitHash(
                shm_name.into_raw(),
                self.size,
                self.size,
                &mut hash_ctl,
                (pg_sys::HASH_ELEM | pg_sys::HASH_BLOBS).try_into().unwrap(),
            )
        };

        htab.htab = htab_ptr;
    }
}
