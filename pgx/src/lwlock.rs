use crate::pg_sys;
use core::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;

/// A Rust locking mechanism which uses a PostgreSQL LWLock to lock the data
///
/// This type of lock allows a number of readers or at most one writer at any
/// point in time. The write portion of this lock typically allows modification
/// of the underlying data (exclusive access) and the read portion of this lock
/// typically allows for read-only access (shared access).
///
/// The lock is valid across processes as the LWLock is managed by Postgres. Data
/// mutability once a lock is obtained is handled by Rust giving out & or &mut
/// pointers.
///
/// When a lock is given out it is wrapped in a PgLwLockShareGuard or
/// PgLwLockExclusiveGuard, which releases the lock on drop
///
/// # Poisoning
/// This lock can not be poisoned from Rust. Panic and Abort are handled by
/// PostgreSQL cleanly.

pub struct PgLwLock<T> {
    inner: UnsafeCell<Option<PgLwLockInner<T>>>,
    name: &'static str,
}

impl<T> PgLwLock<T> {
    /// Create a new lock for T by attaching a LWLock, which is looked up by name
    pub fn new(name: &'static str, value: *mut T) -> Self {
        PgLwLock {
            inner: UnsafeCell::new(Some(PgLwLockInner::<T>::new(name, value))),
            name,
        }
    }

    /// Create an empty lock wich can be created as a global with None as a
    /// sentiel value
    pub fn empty(name: &'static str) -> Self {
        PgLwLock {
            inner: UnsafeCell::new(None),
            name,
        }
    }

    /// Get the name of the PgLwLock
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Obtain a shared lock (which comes with &T access)
    pub fn share(&self) -> PgLwLockShareGuard<T> {
        unsafe {
            (*self.inner.get())
                .as_ref()
                .expect("Lock is in an empty state")
                .share()
        }
    }

    /// Obtain an exclusive lock (which comes with &mut T access)
    pub fn exclusive(&self) -> PgLwLockExclusiveGuard<T> {
        unsafe {
            (*self.inner.get())
                .as_ref()
                .expect("Lock is in an empty state")
                .exclusive()
        }
    }

    /// Attach an empty PgLwLock lock to a LWLock, and wrap T
    pub fn attach(&self, value: *mut T) {
        let slot = unsafe { &*self.inner.get() };
        if slot.is_some() {
            panic!("Lock is not in an empty state");
        }
        let slot = unsafe { &mut *self.inner.get() };
        *slot = Some(PgLwLockInner::<T>::new(self.name, value));
    }
}

pub struct PgLwLockInner<T> {
    lock_ptr: *mut pg_sys::LWLock,
    data: *mut T,
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

            PgLwLockShareGuard {
                data: self.data.as_ref().unwrap(),
                lock: self.lock_ptr,
            }
        }
    }

    fn exclusive(&self) -> PgLwLockExclusiveGuard<T> {
        unsafe {
            pg_sys::LWLockAcquire(self.lock_ptr, pg_sys::LWLockMode_LW_EXCLUSIVE);

            PgLwLockExclusiveGuard {
                data: self.data.as_mut().unwrap(),
                lock: self.lock_ptr,
            }
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
