use crate::lwlock::*;
use crate::{pg_sys, PgAtomic};
use crate::shmem::PgSharedMemoryInitialization;
use std::hash::Hash;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    HashTableFull,
}

pub struct PgHTab<K, V> {
	htab: *mut pg_sys::HTAB,
	size: i64,
}

impl<K, V> PgHTab<K, V> {
	pub fn new(size: i64) -> PgHTab<K, V> {
		PgHTab::<K, V> {
			htab: std::ptr::null(),
			size,
		}
	}

	pub fn insert(&mut self, key: &K, value: &V) -> Result<Option<K>, Error> {
		let void_ptr: *const core::ffi::c_void = &key as *const _ as *const core::ffi::c_void;
		let mut found = false;

		// Delete entry if exists
		let entry = unsafe {
			pg_sys::hash_search(
				&mut self.htab,
				void_ptr,
				pg_sys::HASH_REMOVE,
				&mut found,
			)
		};
		drop(key);
		drop(value);
	}
}

impl<K, V> PgSharedMemoryInitialization for PgHTab<K, V> {
	fn pg_init(&'static self) {}
	fn shmem_init(&'static self) {
		let key_size = std::mem::size_of::<K>();
		let value_size = std::mem::size_of::<V>();
		
		let mut hash_ctl = pg_sys::HASHCTL::default();
		hash_ctl.keysize  = key_size;
		hash_ctl.entrysize = value_size;

		let shm_name = alloc::ffi::CString::new(Uuid::new_v4().to_string())
                .expect("CString::new() failed");

		self.htab = unsafe {
			pg_sys::ShmemInitHash(
				shm_name.into_raw(),
				self.size,
				self.size,
				&mut hash_ctl,
				pg_sys::HASH_ELEM | pg_sys::HASH_BLOBS,
			)
		};
	}
}
