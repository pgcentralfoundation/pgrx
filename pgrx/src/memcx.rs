//! Memory Contexts in PostgreSQL, now with lifetimes.
// "Why isn't this pgrx::mem or pgrx::memcxt?"
// Postgres actually uses all of:
// - mcxt
// - memcxt
// - mctx
// Search engines will see "memc[tx]{2}" and assume you mean memcpy!
// And it's nice-ish to have shorter lifetime names and have 'mcx consistently mean the lifetime.
use crate::pg_sys;
use core::{marker::PhantomData, ptr::NonNull};

/// A borrowed memory context.
pub struct MemCx<'mcx> {
    ptr: NonNull<pg_sys::MemoryContextData>,
    _marker: PhantomData<&'mcx pg_sys::MemoryContextData>,
}

impl<'mcx> MemCx<'mcx> {
    /// You probably shouldn't be using this.
    pub(crate) unsafe fn from_ptr(raw: pg_sys::MemoryContext) -> MemCx<'mcx> {
        let ptr = NonNull::new(raw).expect("memory context must be non-null");
        MemCx { ptr, _marker: PhantomData }
    }

    /// You probably shouldn't be using this.
    pub(crate) unsafe fn alloc_bytes(&self, size: usize) -> *mut u8 {
        pg_sys::MemoryContextAlloc(self.ptr.as_ptr(), size).cast()
    }

    /// You probably shouldn't be using this.
    pub(crate) unsafe fn exec_in<T>(&self, f: impl FnOnce() -> T) -> T {
        let remembered = pg_sys::MemoryContextSwitchTo(self.ptr.as_ptr());
        let res = f();
        pg_sys::MemoryContextSwitchTo(remembered);
        res
    }
}

/// Acquire the current context and operate inside it.
pub fn current_context<'curr, F, T>(f: F) -> T
where
    F: for<'clos> FnOnce(&'clos MemCx<'curr>) -> T,
{
    let memcx = unsafe { MemCx::from_ptr(pg_sys::CurrentMemoryContext) };
    let ret = { f(&memcx) };
    ret
}
