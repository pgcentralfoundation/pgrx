use crate::pg_sys;

/// Return a Postgres-allocated pointer to a struct.  The struct could be a Postgres struct or
/// even a Rust struct.  In either case, the memory is heap-allocated by Postgres
#[inline]
pub fn palloc_struct<T>() -> *mut T {
    unsafe { pg_sys::palloc(std::mem::size_of::<T>()) as *mut T }
}

/// Return a Postgres-allocated pointer to a struct.  The struct could be a Postgres struct or
/// even a Rust struct.  In either case, the memory is heap-allocated by Postgres
///
/// Also zeros out the allocation block
#[inline]
pub fn palloc0_struct<T>() -> *mut T {
    unsafe { pg_sys::palloc0(std::mem::size_of::<T>()) as *mut T }
}
