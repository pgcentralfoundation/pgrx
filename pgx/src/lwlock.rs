/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use crate::pg_sys;
use core::ops::{Deref, DerefMut};
use once_cell::sync::OnceCell;
use std::fmt;
use uuid::Uuid;

/// A Rust locking mechanism which uses a PostgreSQL LWLock to lock the data
///
/// This type of lock allows a number of readers or at most one writer at any
/// point in time. The write portion of this lock typically allows modification
/// of the underlying data (exclusive access) and the read portion of this lock
/// typically allows for read-only access (shared access).
///
/// The lock is valid across processes as the LWLock is managed by Postgres. Data
/// mutability once a lock is obtained is handled by Rust giving out `&` or `&mut`
/// pointers.
///
/// When a lock is given out it is wrapped in a PgLwLockShareGuard or
/// PgLwLockExclusiveGuard, which releases the lock on drop
///
/// # Poisoning
/// This lock can not be poisoned from Rust. Panic and Abort are handled by
/// PostgreSQL cleanly.
pub struct PgLwLock<T> {
    inner: OnceCell<PgLwLockInner<T>>,
    name: OnceCell<&'static str>,
}

unsafe impl<T> Send for PgLwLock<T> {}
unsafe impl<T> Sync for PgLwLock<T> {}

impl<T> PgLwLock<T> {
    /// Create an empty lock which can be created as a global with None as a
    /// sentinel value
    pub const fn new() -> Self {
        PgLwLock { inner: OnceCell::new(), name: OnceCell::new() }
    }

    /// Create a new lock for T by attaching a LWLock, which is looked up by name
    pub fn from_named(input_name: &'static str, value: *mut T) -> Self {
        let inner = OnceCell::new();
        let name = OnceCell::new();
        inner.set(PgLwLockInner::<T>::new(input_name, value)).unwrap();
        name.set(input_name).unwrap();
        PgLwLock { inner, name }
    }

    /// Get the name of the PgLwLock
    pub fn get_name(&self) -> &'static str {
        match self.name.get() {
            None => {
                let name = Box::leak(Uuid::new_v4().to_string().into_boxed_str());
                self.name.set(name).unwrap();
                name
            }
            Some(name) => name,
        }
    }

    /// Obtain a shared lock (which comes with `&T` access)
    pub fn share(&self) -> PgLwLockShareGuard<T> {
        self.inner.get().expect("Can't give out share, lock is in an empty state").share()
    }

    /// Obtain an exclusive lock (which comes with `&mut T` access)
    pub fn exclusive(&self) -> PgLwLockExclusiveGuard<T> {
        self.inner.get().expect("Can't give out exclusive, lock is in an empty state").exclusive()
    }

    /// Attach an empty PgLwLock lock to a LWLock, and wrap T
    pub fn attach(&self, value: *mut T) {
        self.inner
            .set(PgLwLockInner::<T>::new(self.get_name(), value))
            .expect("Can't attach, lock is not in an empty state");
    }
}

pub struct PgLwLockInner<T> {
    lock_ptr: *mut pg_sys::LWLock,
    data: *mut T,
}

impl<T> fmt::Debug for PgLwLockInner<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PgLwLockInner").finish()
    }
}

impl<'a, T> PgLwLockInner<T> {
    fn new(name: &'static str, data: *mut T) -> Self {
        unsafe {
            let lock = std::ffi::CString::new(name).expect("CString::new failed");
            PgLwLockInner {
                lock_ptr: &mut (*pg_sys::GetNamedLWLockTranche(lock.as_ptr())).lock,
                data,
            }
        }
    }

    fn share(&self) -> PgLwLockShareGuard<T> {
        unsafe {
            pg_sys::LWLockAcquire(self.lock_ptr, pg_sys::LWLockMode_LW_SHARED);

            PgLwLockShareGuard { data: self.data.as_ref().unwrap(), lock: self.lock_ptr }
        }
    }

    fn exclusive(&self) -> PgLwLockExclusiveGuard<T> {
        unsafe {
            pg_sys::LWLockAcquire(self.lock_ptr, pg_sys::LWLockMode_LW_EXCLUSIVE);

            PgLwLockExclusiveGuard { data: self.data.as_mut().unwrap(), lock: self.lock_ptr }
        }
    }
}

pub struct PgLwLockShareGuard<'a, T> {
    data: &'a T,
    lock: *mut pg_sys::LWLock,
}

impl<T> Drop for PgLwLockShareGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            pg_sys::LWLockRelease(self.lock);
        }
    }
}

impl<T> Deref for PgLwLockShareGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

pub struct PgLwLockExclusiveGuard<'a, T> {
    data: &'a mut T,
    lock: *mut pg_sys::LWLock,
}

impl<T> Deref for PgLwLockExclusiveGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for PgLwLockExclusiveGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T> Drop for PgLwLockExclusiveGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            pg_sys::LWLockRelease(self.lock);
        }
    }
}
