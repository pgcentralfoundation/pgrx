//! Memory Contexts in PostgreSQL, now with lifetimes.
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, ReturnsError, ReturnsRef, SqlMappingRef, SqlTranslatable,
};

// "Why isn't this pgrx::mem or pgrx::memcxt?"
// Postgres actually uses all of:
// - mcxt
// - memcxt
// - mctx
// Search engines will see "memc[tx]{2}" and assume you mean memcpy!
// And it's nice-ish to have shorter lifetime names and have 'mcx consistently mean the lifetime.
use crate::callconv::{Arg, ArgAbi};
use crate::nullable::Nullable;
use crate::pg_sys;
use core::alloc::Layout;
use core::{marker::PhantomData, ptr::NonNull};

/// Postgres's `MaxAllocSize` (1 GiB - 1). `bindgen` does not expose it
/// (see `pgrx/src/array.rs:219`), so we mirror the definition here. Any
/// `palloc` call exceeding this size requires `MCXT_ALLOC_HUGE`; without
/// that flag, Postgres `ereport`s — which longjmps out of Rust frames.
const MAX_ALLOC_SIZE: usize = 0x3fff_ffff;

/// Postgres's `MaxAllocHugeSize` (`SIZE_MAX / 2`). `bindgen` records this as`0` because the C macro depends on `SIZE_MAX`; we mirror the real value so the huge-allocation entrypoints can reject sizes that would otherwise trigger Postgres's `MemoryContextCheckSize` ereport and longjmp out of Rust frames.
const MAX_ALLOC_HUGE_SIZE: usize = usize::MAX / 2;

/// A borrowed memory context.
#[repr(transparent)]
pub struct MemCx<'mcx> {
    ptr: NonNull<pg_sys::MemoryContextData>,
    _marker: PhantomData<&'mcx pg_sys::MemoryContextData>,
}

impl<'mcx> MemCx<'mcx> {
    /// Wrap the provided [`pg_sys::MemoryContext`]
    ///
    /// # Safety
    /// Assumes the provided [`pg_sys::MemoryContext`] is valid and properly initialized.
    /// This method does check to ensure the pointer is non-null, but that is the only sanity
    /// check that is performed.
    pub(crate) unsafe fn from_ptr(raw: pg_sys::MemoryContext) -> MemCx<'mcx> {
        let ptr = NonNull::new(raw).expect("memory context must be non-null");
        MemCx { ptr, _marker: PhantomData }
    }

    /// Allocate a raw byte buffer `size` bytes in length
    /// and returns a pointer to the new allocation.
    pub fn alloc_bytes(&self, size: usize) -> Result<NonNull<u8>, OutOfMemory> {
        let flags = (pg_sys::MCXT_ALLOC_NO_OOM) as i32;
        let ptr = unsafe { pg_sys::MemoryContextAllocExtended(self.ptr.as_ptr(), size, flags) };
        NonNull::new(ptr.cast()).ok_or(OutOfMemory::new())
    }

    /// Allocate a raw byte buffer `size` bytes in length
    /// and returns a pointer to the new allocation.
    pub fn alloc_zeroed_bytes(&self, size: usize) -> Result<NonNull<u8>, OutOfMemory> {
        let flags = (pg_sys::MCXT_ALLOC_NO_OOM | pg_sys::MCXT_ALLOC_ZERO) as i32;
        let ptr = unsafe { pg_sys::MemoryContextAllocExtended(self.ptr.as_ptr(), size, flags) };
        NonNull::new(ptr.cast()).ok_or(OutOfMemory::new())
    }

    /// Allocate a buffer matching `layout`.
    ///
    /// Sizes above Postgres's `MaxAllocSize` (1 GiB - 1) are rejected up
    /// front with `OutOfMemory`. Without this guard, the underlying palloc
    /// call would `ereport` and longjmp out of the Rust frame. For genuine
    /// huge allocations, use [`MemCx::alloc_huge_layout`] instead.
    ///
    /// On PG >= 16 this uses `MemoryContextAllocAligned` for arbitrary power-of-two alignment. On earlier versions this falls back to size-only `MemoryContextAllocExtended`, which only guarantees `MAXIMUM_ALIGNOF` (8 on 64-bit, 4 on 32-bit); requests for stronger alignment are rejected with `OutOfMemory` rather than silently returning an under-aligned pointer.
    pub fn alloc_layout(&self, layout: Layout) -> Result<NonNull<u8>, OutOfMemory> {
        Self::check_layout(layout, MAX_ALLOC_SIZE)?;
        self.alloc_layout_with_flags(layout, pg_sys::MCXT_ALLOC_NO_OOM as i32)
    }

    /// Like [`MemCx::alloc_layout`] but zeroes the allocation.
    pub fn alloc_layout_zeroed(&self, layout: Layout) -> Result<NonNull<u8>, OutOfMemory> {
        Self::check_layout(layout, MAX_ALLOC_SIZE)?;
        self.alloc_layout_with_flags(
            layout,
            (pg_sys::MCXT_ALLOC_NO_OOM | pg_sys::MCXT_ALLOC_ZERO) as i32,
        )
    }

    /// Like [`MemCx::alloc_layout`] but permits sizes above `MaxAllocSize`
    /// (1 GiB - 1), up to `MaxAllocHugeSize` (`SIZE_MAX / 2`). Sizes beyond
    /// that bound are rejected up front with `OutOfMemory`; without that
    /// guard Postgres's `MemoryContextCheckSize` would `ereport` (which
    /// `MCXT_ALLOC_NO_OOM` does **not** suppress) and longjmp out of the
    /// Rust frame.
    ///
    /// Use only when the allocation is genuinely known to be huge. The 1 GiB
    /// cap on [`MemCx::alloc_layout`] is a deliberate sanity check that
    /// catches sizing bugs early; opting into `_huge_` should be a
    /// considered choice, not a routine workaround.
    ///
    /// Note: varlena values cannot exceed 1 GiB by design (the 4-byte
    /// header encodes 30-bit size). Huge allocations are only useful for
    /// Rust-side scratch buffers, not for values returned to Postgres.
    pub fn alloc_huge_layout(&self, layout: Layout) -> Result<NonNull<u8>, OutOfMemory> {
        Self::check_layout(layout, MAX_ALLOC_HUGE_SIZE)?;
        self.alloc_layout_with_flags(
            layout,
            (pg_sys::MCXT_ALLOC_NO_OOM | pg_sys::MCXT_ALLOC_HUGE) as i32,
        )
    }

    /// Like [`MemCx::alloc_huge_layout`] but zeroes the allocation.
    pub fn alloc_huge_layout_zeroed(&self, layout: Layout) -> Result<NonNull<u8>, OutOfMemory> {
        Self::check_layout(layout, MAX_ALLOC_HUGE_SIZE)?;
        self.alloc_layout_with_flags(
            layout,
            (pg_sys::MCXT_ALLOC_NO_OOM | pg_sys::MCXT_ALLOC_HUGE | pg_sys::MCXT_ALLOC_ZERO) as i32,
        )
    }

    /// Validate a `Layout` against the per-context size cap and the alignment guarantees actually delivered by the underlying palloc call.
    ///
    /// On PG >= 16 `MemoryContextAllocAligned` honors arbitrary power-of-two alignments. On older versions the implementation falls back to size-only `MemoryContextAllocExtended`, which only guarantees`MAXIMUM_ALIGNOF`. Rejecting over-aligned requests there prevents safe callers from receiving an under-aligned `NonNull<u8>` that silently violates the requested `Layout`.
    #[inline]
    fn check_layout(layout: Layout, max_size: usize) -> Result<(), OutOfMemory> {
        if layout.size() > max_size {
            return Err(OutOfMemory::new());
        }
        #[cfg(not(any(feature = "pg16", feature = "pg17", feature = "pg18")))]
        {
            if layout.align() > pg_sys::MAXIMUM_ALIGNOF as usize {
                return Err(OutOfMemory::new());
            }
        }
        Ok(())
    }

    #[inline]
    fn alloc_layout_with_flags(
        &self,
        layout: Layout,
        flags: i32,
    ) -> Result<NonNull<u8>, OutOfMemory> {
        let ptr = unsafe {
            #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
            {
                pg_sys::MemoryContextAllocAligned(
                    self.ptr.as_ptr(),
                    layout.size(),
                    layout.align(),
                    flags,
                )
            }
            #[cfg(not(any(feature = "pg16", feature = "pg17", feature = "pg18")))]
            {
                let _ = layout.align();
                pg_sys::MemoryContextAllocExtended(self.ptr.as_ptr(), layout.size(), flags)
            }
        };
        NonNull::new(ptr.cast()).ok_or_else(OutOfMemory::new)
    }

    /// Stores the current memory context, switches to *this* memory context,
    /// and executes the closure `f`.
    /// Once `f` completes, the previous current memory context is restored.
    ///
    /// # Safety
    /// If `f` panics, the current memory context will remain set to this MemCx,
    /// and the previous current memory context will not be restored, leaving the entire
    /// Postgres environment in an invalid state.
    /// Please do not use this method with closures that can panic (of course, this is
    /// less of a concern for unit tests).
    pub unsafe fn exec_in<T>(&self, f: impl FnOnce() -> T) -> T {
        let remembered = pg_sys::MemoryContextSwitchTo(self.ptr.as_ptr());
        let res = f();
        pg_sys::MemoryContextSwitchTo(remembered);
        res
    }

    /// Allocate a varlena with a valid 4-byte header and `payload_len` zeroed payload bytes. The returned [`VarlenaBuf`] maintains the header as a type invariant.
    ///
    /// [`VarlenaBuf`]: crate::datum::varlena_buf::VarlenaBuf
    pub fn alloc_varlena(
        &self,
        payload_len: usize,
    ) -> Result<crate::datum::varlena_buf::VarlenaBuf<'mcx>, OutOfMemory> {
        let total = (pg_sys::VARHDRSZ).checked_add(payload_len).ok_or_else(OutOfMemory::new)?;
        let layout = core::alloc::Layout::from_size_align(total, core::mem::align_of::<u32>())
            .map_err(|_| OutOfMemory::new())?;
        let raw = self.alloc_layout_zeroed(layout)?;
        // Set the 4-byte header. The total_size encoded in the header is `total` (header bytes + payload bytes), per Postgres convention for 4-byte headers.
        // SAFETY: `raw` points at `total` zeroed bytes, large enough for a 4-byte header.
        unsafe {
            crate::varlena::set_varsize_4b(raw.as_ptr() as *mut pg_sys::varlena, total as i32);
        }
        // SAFETY: `raw` is non-null, sized at `total`, header is now valid.
        Ok(unsafe { crate::datum::varlena_buf::VarlenaBuf::from_raw(raw, total, self) })
    }

    /// Inspect an existing varlena pointer as a `&RawVarlena`.
    ///
    /// The returned reference is bound to `&self`, not to the full `'mcx`
    /// lifetime. This narrows the borrow so that other handles in the same
    /// `MemCx` (e.g. a `PBox<RawVarlena>` returned by
    /// `VarlenaBuf::into_pbox`) cannot be re-borrowed as `&mut` while the
    /// `&RawVarlena` is alive.
    ///
    /// Lifetime caveat: the `&'a self` tie does **not** prove the pointee
    /// outlives `'a`. The pointer is raw and originates outside the type
    /// system. Use-after-free protection relies entirely on the unsafe
    /// preconditions below — specifically that `*ptr` lives for at least
    /// `'mcx`. The lifetime is purely an aliasing knob, not a UAF guard.
    ///
    /// # Panics
    /// Panics if `ptr` references an external/TOAST (`VARATT_IS_1B_E`) or
    /// 4-byte compressed (`VARATT_IS_4B_C`) varlena. Both classes carry
    /// out-of-line / compressed payloads that `RawVarlena::payload`
    /// cannot soundly expose; detoast first (e.g. `pg_detoast_datum`).
    /// Inside a `#[pg_extern]` the panic is converted to `ereport ERROR`
    /// by pgrx's panic handler.
    ///
    /// # Safety
    /// - `ptr` must point to a valid varlena whose header size is correctly set
    /// - `ptr` must be 4-byte aligned when the header is a 4-byte header (uncompressed or compressed); `varsize_4b` reads a `u32` directly from `ptr`. 1-byte (short) headers carry no alignment requirement.
    /// - `ptr` must point to an in-line (non-TOAST, non-compressed) varlena;
    ///   `RawVarlena::payload` can only expose bytes that live in the same
    ///   allocation. TOAST/compressed varlenas must be detoasted first
    ///   (e.g. via `pg_detoast_datum`).
    /// - the pointee must live for at least `'mcx`
    /// - no `&mut` reference to the same allocation may exist while the
    ///   returned `&RawVarlena` is alive
    pub unsafe fn inspect_varlena<'a>(
        &'a self,
        ptr: *const pg_sys::varlena,
    ) -> &'a crate::datum::varlena_buf::RawVarlena {
        assert!(
            !crate::varlena::varatt_is_1b_e(ptr),
            "inspect_varlena: external/TOASTed varlena must be detoasted first",
        );
        assert!(
            !crate::varlena::varatt_is_4b_c(ptr),
            "inspect_varlena: compressed varlena must be detoasted first",
        );
        let total = crate::varlena::varsize_any(ptr);
        let slice_ptr: *const [u8] = core::ptr::slice_from_raw_parts(ptr as *const u8, total);
        let raw_ptr: *const crate::datum::varlena_buf::RawVarlena = slice_ptr as *const _;
        &*raw_ptr
    }
}

#[derive(Debug)]
pub struct OutOfMemory {
    _reserve: (),
}
impl OutOfMemory {
    pub fn new() -> OutOfMemory {
        OutOfMemory { _reserve: () }
    }
}

/// Acquire the current context and operate inside it.
pub fn current_context<'curr, F, T>(f: F) -> T
where
    F: for<'clos> FnOnce(&'clos MemCx<'curr>) -> T,
{
    let memcx = unsafe { MemCx::from_ptr(pg_sys::CurrentMemoryContext) };

    f(&memcx)
}

#[cfg(all(feature = "nightly", feature = "pg16", feature = "pg17", feature = "pg18"))]
mod nightly {
    use super::*;
    use std::slice;

    unsafe impl<'mcx> std::alloc::Allocator for &MemCx<'mcx> {
        fn allocate(
            &self,
            layout: std::alloc::Layout,
        ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
            unsafe {
                // Bitflags for MemoryContextAllocAligned:
                // #define MCXT_ALLOC_HUGE    0x01 /* allow huge allocation (> 1 GB) */
                // #define MCXT_ALLOC_NO_OOM  0x02 /* no failure if out-of-memory */
                // #define MCXT_ALLOC_ZERO    0x04 /* zero allocated memory */
                let ptr = pg_sys::MemoryContextAllocAligned(
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
            pg_sys::pfree(ptr.as_ptr().cast())
        }

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
                let ptr = pg_sys::MemoryContextAllocAligned(
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
}

unsafe impl<'fcx> ArgAbi<'fcx> for &MemCx<'fcx> {
    unsafe fn unbox_arg_unchecked(_arg: Arg<'_, 'fcx>) -> Self {
        // SAFETY: We are called to unbox an argument, which means the backend was initialized.
        // We use this horrific expression to allow the lifetime to be extended arbitrarily
        // and achieve an "in-place" transformation of CurrentMemoryContext's pointer.
        // The soundness of this is riding on the lifetimes used for `unbox_arg_unchecked` in our macros,
        // as the expanded code is designed so that `fcinfo` and each `arg` are truly "borrowed" in rustc's eyes.
        unsafe { &*((&raw mut pg_sys::CurrentMemoryContext).cast()) }
    }

    unsafe fn unbox_nullable_arg(arg: Arg<'_, 'fcx>) -> Nullable<Self> {
        // SAFETY: Should never happen in actuality, but as long as we're here...
        if unsafe { pg_sys::CurrentMemoryContext.is_null() } {
            Nullable::Null
        } else {
            Nullable::Valid(Self::unbox_arg_unchecked(arg))
        }
    }

    fn is_virtual_arg() -> bool {
        true
    }
}

/// SAFETY: virtual argument
unsafe impl<'mcx> SqlTranslatable for &MemCx<'mcx> {
    const TYPE_IDENT: &'static str = crate::pgrx_resolved_type!(MemCx<'mcx>);
    const TYPE_ORIGIN: pgrx_sql_entity_graph::metadata::TypeOrigin =
        pgrx_sql_entity_graph::metadata::TypeOrigin::External;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Ok(SqlMappingRef::Skip);
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> = Ok(ReturnsRef::One(SqlMappingRef::Skip));
}
