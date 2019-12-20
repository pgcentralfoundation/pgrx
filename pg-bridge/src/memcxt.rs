use crate::pg_sys;
use std::ops::{Deref, DerefMut};

/// Return a Postgres-allocated pointer to a struct.  The struct could be a Postgres struct or
/// even a Rust struct.  In either case, the memory is heap-allocated by Postgres
#[inline]
pub fn palloc_struct<T: Sized>() -> *mut T {
    unsafe { pg_sys::palloc(std::mem::size_of::<T>()) as *mut T }
}

/// Return a Postgres-allocated pointer to a struct.  The struct could be a Postgres struct or
/// even a Rust struct.  In either case, the memory is heap-allocated by Postgres
///
/// Also zeros out the allocation block
#[inline]
pub fn palloc0_struct<T: Sized>() -> *mut T {
    unsafe { pg_sys::palloc0(std::mem::size_of::<T>()) as *mut T }
}

#[derive(Debug)]
enum WhoAllocated {
    Postgres,
    Rust,
}

#[derive(Debug)]
pub struct PgBox<T>
where
    T: Sized,
{
    ptr: Option<*mut T>,
    owner: WhoAllocated,
}

impl<T> PgBox<T>
where
    T: Sized,
{
    /// Allocate enough memory for the type'd struct, using Postgres.  The
    /// allocated memory is uninitialized.
    ///
    /// When this object is dropped the backing memory will be pfree'd,
    /// unless it is instead turned `into_pg()`, at which point it will be freeded
    /// when its owning MemoryContext is deleted by Postgres (likely transaction end).
    ///
    /// ## Examples
    /// ```rust
    /// use pg_bridge::{PgBox, pg_sys};
    /// let ctid = PgBox::<pg_sys::ItemPointerData>::alloc();
    /// ```
    pub fn alloc() -> PgBox<T> {
        PgBox::<T> {
            ptr: Some(palloc_struct::<T>()),
            owner: WhoAllocated::Rust,
        }
    }

    /// Allocate enough memory for the type'd struct, using Postgres.  The
    /// allocated memory is zero-filled.
    ///
    /// When this object is dropped the backing memory will be pfree'd,
    /// unless it is instead turned `into_pg()`, at which point it will be freeded
    /// when its owning MemoryContext is deleted by Postgres (likely transaction end).
    ///
    /// ## Examples
    /// ```rust
    /// use pg_bridge::{PgBox, pg_sys};
    /// let ctid = PgBox::<pg_sys::ItemPointerData>::alloc0();
    /// ```
    pub fn alloc0() -> PgBox<T> {
        PgBox::<T> {
            ptr: Some(palloc0_struct::<T>()),
            owner: WhoAllocated::Rust,
        }
    }

    pub fn from_pg(ptr: *mut T) -> PgBox<T> {
        PgBox::<T> {
            ptr: if ptr.is_null() { None } else { Some(ptr) },
            owner: WhoAllocated::Postgres,
        }
    }

    pub fn into_pg(self) -> *mut T {
        let ptr = self.ptr;
        std::mem::forget(self);

        match ptr {
            Some(ptr) => ptr,
            None => 0 as *mut T,
        }
    }

    pub fn into_inner(self) -> T {
        let ptr = self.ptr;
        match ptr {
            Some(ptr) => {
                std::mem::forget(self);
                unsafe { ptr.read() }
            }
            None => panic!("attempt to dereference a null pointer"),
        }
    }
}

impl<T> Deref for PgBox<T>
where
    T: Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.ptr {
            Some(ptr) => unsafe { &*ptr },
            None => panic!("Attempt to dereference null pointer"),
        }
    }
}

impl<T> DerefMut for PgBox<T>
where
    T: Sized,
{
    fn deref_mut(&mut self) -> &mut T {
        match self.ptr {
            Some(ptr) => unsafe { &mut *ptr },
            None => panic!("Attempt to dereference null pointer"),
        }
    }
}

impl<T> From<*mut T> for PgBox<T>
where
    T: Sized,
{
    fn from(ptr: *mut T) -> Self {
        PgBox::from_pg(ptr)
    }
}

impl<T> Drop for PgBox<T>
where
    T: Sized,
{
    fn drop(&mut self) {
        if self.ptr.is_some() {
            match self.owner {
                WhoAllocated::Postgres => { /* do nothing, we'll let Postgres free the pointer */ }
                WhoAllocated::Rust => {
                    super::info!("Freeing pointer");
                    // we own it here in rust, so we need to free it too
                    let ptr = self.ptr.unwrap();
                    unsafe {
                        pg_sys::pfree(ptr as *mut std::ffi::c_void);
                    }
                }
            }
        }
    }
}
