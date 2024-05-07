//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#![allow(clippy::needless_borrow)]
use crate::pg_sys;
use core::ops::{Deref, DerefMut};
use once_cell::sync::OnceCell;
use std::cell::UnsafeCell;
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
    inner: UnsafeCell<PgLwLockInner<T>>,
    name: OnceCell<&'static str>,
}

unsafe impl<T: Send> Send for PgLwLock<T> {}
unsafe impl<T: Send + Sync> Sync for PgLwLock<T> {}

impl<T> PgLwLock<T> {
    /// Create an empty lock which can be created as a global with None as a
    /// sentinel value
    pub const fn new() -> Self {
        PgLwLock { inner: UnsafeCell::new(PgLwLockInner::empty()), name: OnceCell::new() }
    }

    /// Create a new lock for T by attaching a LWLock, which is looked up by name
    pub fn from_named(input_name: &'static str, value: *mut T) -> Self {
        let name = OnceCell::new();
        let inner = UnsafeCell::new(PgLwLockInner::<T>::new(input_name, value));
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
        unsafe { self.inner.get().as_ref().unwrap().share() }
    }

    /// Obtain an exclusive lock (which comes with `&mut T` access)
    pub fn exclusive(&self) -> PgLwLockExclusiveGuard<T> {
        unsafe { self.inner.get().as_ref().unwrap().exclusive() }
    }

    /// Attach an empty PgLwLock lock to a LWLock, and wrap T
    /// SAFETY: Must only be called from inside the Postgres shared memory init hook
    pub unsafe fn attach(&self, value: *mut T) {
        *self.inner.get() = PgLwLockInner::<T>::new(self.get_name(), value);
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
            let lock = alloc::ffi::CString::new(name).expect("CString::new failed");
            PgLwLockInner {
                lock_ptr: &mut (*pg_sys::GetNamedLWLockTranche(lock.as_ptr())).lock,
                data,
            }
        }
    }

    const fn empty() -> Self {
        PgLwLockInner { lock_ptr: std::ptr::null_mut(), data: std::ptr::null_mut() }
    }

    fn share(&self) -> PgLwLockShareGuard<T> {
        if self.lock_ptr.is_null() {
            panic!("PgLwLock is not initialized");
        }
        unsafe {
            pg_sys::LWLockAcquire(self.lock_ptr, pg_sys::LWLockMode_LW_SHARED);

            PgLwLockShareGuard { data: self.data.as_ref().unwrap(), lock: self.lock_ptr }
        }
    }

    fn exclusive(&self) -> PgLwLockExclusiveGuard<T> {
        if self.lock_ptr.is_null() {
            panic!("PgLwLock is not initialized");
        }
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
        // SAFETY: self.lock is always valid
        unsafe { release_unless_elog_unwinding(self.lock) }
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
        // SAFETY: self.lock is always valid
        unsafe { release_unless_elog_unwinding(self.lock) }
    }
}

/// Releases the given lock, unless we are unwinding due to an `error` in postgres code
///
/// `elog(ERROR)` from postgres code resets `pg_sys::InterruptHoldoffCount` to zero, and
/// `LWLockRelease` fails an assertion if called in this case.
/// If we detect this condition, we skip releasing the lock; all lwlocks will be released
/// on (sub)transaction abort anyway.
///
/// SAFETY: the given lock must be valid
unsafe fn release_unless_elog_unwinding(lock: *mut pg_sys::LWLock) {
    // SAFETY: mut static access is ok from a single (main) thread.
    if pg_sys::InterruptHoldoffCount > 0 {
        pg_sys::LWLockRelease(lock);
    }
}
