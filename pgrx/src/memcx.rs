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
use std::slice;

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

    /// Stores the current global memory context, switches to *this* memory context,
    /// and executes the closure `f`.
    /// Once `f` completes, the previous global memory context is restored.
    pub unsafe fn exec_in<T>(&self, f: impl FnOnce() -> T) -> T {
        let remembered = pg_sys::MemoryContextSwitchTo(self.ptr.as_ptr());
        let res = f();
        pg_sys::MemoryContextSwitchTo(remembered);
        res
    }
}

/// Acquire the current global context and operate inside it.
pub fn current_context<'curr, F, T>(f: F) -> T
where
    F: for<'clos> FnOnce(&'clos MemCx<'curr>) -> T,
{
    let memcx = unsafe { MemCx::from_ptr(pg_sys::CurrentMemoryContext) };

    f(&memcx)
}

#[cfg(all(
    feature = "nightly",
    any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15")
))]
use std::alloc::Layout;

/// Does alignment / padding logic for pre-Postgres16 8-byte alignment.
#[cfg(all(
    feature = "nightly",
    any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15")
))]
fn layout_for_pre16(layout: Layout) -> Result<Layout, std::alloc::AllocError> {
    Ok(layout.align_to(8).map_err(|_e| std::alloc::AllocError)?.pad_to_align())
}

#[cfg(feature = "nightly")]
unsafe impl<'mcx> std::alloc::Allocator for &MemCx<'mcx> {
    // Future-proofing - currently only pg16 supports arbitrary alignment, but that is likely
    // to change, whereas old versions are unlikely to lose 8-byte alignment.
    #[cfg(any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15"))]
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        // On versions before Postgres 16, alignment is always 8 byte / 64 bit.
        let layout = layout_for_pre16(layout)?;
        unsafe {
            let ptr = pgrx_pg_sys::MemoryContextAlloc(self.ptr.as_ptr(), layout.size());
            let slice: &mut [u8] = slice::from_raw_parts_mut(ptr.cast(), layout.size());
            Ok(NonNull::new_unchecked(slice))
        }
    }

    #[cfg(not(any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15")))]
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        // Postgres 16 and newer permit any arbitrary power-of-2 alignment
        unsafe {
            // Bitflags for MemoryContextAllocAligned:
            // #define MCXT_ALLOC_HUGE    0x01 /* allow huge allocation (> 1 GB) */
            // #define MCXT_ALLOC_NO_OOM  0x02 /* no failure if out-of-memory */
            // #define MCXT_ALLOC_ZERO    0x04 /* zero allocated memory */
            let ptr = pgrx_pg_sys::MemoryContextAllocAligned(
                self.ptr.as_ptr(),
                layout.size(),
                layout.align(),
                0,
            );
            let slice: &mut [u8] = slice::from_raw_parts_mut(ptr.cast(), layout.size());
            Ok(NonNull::new_unchecked(slice))
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: std::alloc::Layout) {
        // TODO: Find faster free for use when MemoryContext is known.
        // This is the global function that looks up the relevant Memory Context by address range.
        pgrx_pg_sys::pfree(ptr.as_ptr().cast())
    }

    #[cfg(any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15"))]
    fn allocate_zeroed(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        // Overriding default function here to use Postgres' zeroing implementation.
        // On versions before Postgres 16, alignment is always 8 byte / 64 bit.
        let layout = layout_for_pre16(layout)?;
        unsafe {
            let ptr = pgrx_pg_sys::MemoryContextAllocZero(self.ptr.as_ptr(), layout.size());
            let slice: &mut [u8] = slice::from_raw_parts_mut(ptr.cast(), layout.size());
            Ok(NonNull::new_unchecked(slice))
        }
    }
    #[cfg(not(any(feature = "pg12", feature = "pg13", feature = "pg14", feature = "pg15")))]
    fn allocate_zeroed(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        // Overriding default function here to use Postgres' zeroing implementation.
        // Postgres 16 and newer permit any arbitrary power-of-2 alignment
        unsafe {
            // Bitflags for MemoryContextAllocAligned:
            // #define MCXT_ALLOC_HUGE    0x01 /* allow huge allocation (> 1 GB) */
            // #define MCXT_ALLOC_NO_OOM  0x02 /* no failure if out-of-memory */
            // #define MCXT_ALLOC_ZERO    0x04 /* zero allocated memory */
            let ptr = pgrx_pg_sys::MemoryContextAllocAligned(
                self.ptr.as_ptr(),
                layout.size(),
                layout.align(),
                4,
            );
            let slice: &mut [u8] = slice::from_raw_parts_mut(ptr.cast(), layout.size());
            Ok(NonNull::new_unchecked(slice))
        }
    }
}
