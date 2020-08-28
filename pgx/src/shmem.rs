use core::ops::{Deref, DerefMut};
use crate::pg_sys;
use std::cell::UnsafeCell;

// RwLock clone which uses a PostgreSQL LWLock to guard data (so it's safe cross process)
pub struct PgLwLock<T> {
    inner: UnsafeCell<Option<PgLwLockInner<T>>>,
}

impl<T> PgLwLock<T> {
    pub const fn new() -> Self {
        PgLwLock {
            inner: UnsafeCell::new(None),
        }
    }
    pub fn share(&self) -> PgLwLockShareGuard<T> {
        unsafe { (*self.inner.get()).as_ref().unwrap().share() }
    }

    pub fn exclusive(&self) -> PgLwLockExclusiveGuard<T> {
        unsafe { (*self.inner.get()).as_mut().unwrap().exclusive() }
    }

    // To use a lock it has to be attached to an allocated Postgres LWLock, which
    // we look up by name
    pub fn attach(&self, lock: String, value: T) -> Result<(), T> {
        let slot = unsafe { &*self.inner.get() };
        if slot.is_some() {
            return Err(value);
        }
        let slot = unsafe { &mut *self.inner.get() };
        *slot = Some(PgLwLockInner::<T>::new(lock, value));
        Ok(())
    }
}

pub struct PgLwLockInner<T> {
    lock_ptr: *mut pg_sys::LWLock,
    data: UnsafeCell<T>,
}

impl<'a, T> PgLwLockInner<T> {
    fn new(lock_name: String, data: T) -> Self {
        unsafe {
            let lock = std::ffi::CString::new(lock_name).expect("CString::new failed");
            PgLwLockInner {
                lock_ptr: &mut (*pg_sys::GetNamedLWLockTranche(lock.as_ptr())).lock,
                data: UnsafeCell::new(data),
            }
        }
    }

    fn share(&self) -> PgLwLockShareGuard<T> {
        unsafe {
            pg_sys::LWLockAcquire(self.lock_ptr, pg_sys::LWLockMode_LW_SHARED);
        }
        unsafe {
            PgLwLockShareGuard {
                data: &*self.data.get(),
                lock: self.lock_ptr,
            }
        }
    }

    fn exclusive(&mut self) -> PgLwLockExclusiveGuard<T> {
        unsafe {
            pg_sys::LWLockAcquire(self.lock_ptr, pg_sys::LWLockMode_LW_EXCLUSIVE);
        }
        unsafe {
            PgLwLockExclusiveGuard {
                data: &mut *self.data.get(),
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
