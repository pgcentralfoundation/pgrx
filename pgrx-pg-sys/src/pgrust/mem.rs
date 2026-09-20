//! Memory contexts and `palloc` for the pgrust target.
//!
//! pgrust threads `Mcx<'_>` explicitly and has no `CurrentMemoryContext`
//! global; pgrx assumes one. The compromise (ruling: extension code only) is a
//! per-thread current-context cell that [`crate::pgrust::fmgr::call_v1`] arms
//! with the call's result context and restores afterwards. Everything pgrx
//! allocates goes through [`palloc`], which prefixes a chunk header so that
//! [`pfree`], [`repalloc`] and [`GetMemoryChunkContext`] work without
//! pgrust's allocator needing a per-chunk header of its own.

use core::alloc::Layout;
use core::cell::{Cell, RefCell};
use core::ffi::{c_char, c_int, c_void};
use core::ptr::NonNull;
use std::collections::HashSet;

use allocator_api2::alloc::Allocator;

use super::error::raise;
use super::types::{MCXT_ALLOC_ZERO, MemoryContextCallback, Size};

/// An opaque handle: every `MemoryContext` points at a live pgrust
/// `mcx::MemoryContext`, but pgrx code only ever passes the pointer around.
/// (Opaque rather than an alias so the pointer stays `UnwindSafe`, which
/// pgrx's `PgTryBuilder`/aggregate closures require.)
#[repr(C)]
pub struct MemoryContextData {
    _opaque: [u8; 0],
}
pub type MemoryContext = *mut MemoryContextData;

/// The pgrust context behind a handle.
#[inline]
pub(crate) unsafe fn native_ctx<'a>(ctx: MemoryContext) -> &'a ::pgr_mcx::MemoryContext {
    &*(ctx as *const ::pgr_mcx::MemoryContext)
}

/// A handle for a pgrust context.
#[inline]
pub(crate) fn handle_of(ctx: &::pgr_mcx::MemoryContext) -> MemoryContext {
    ctx as *const ::pgr_mcx::MemoryContext as *mut MemoryContextData
}

thread_local! {
    static CURRENT: Cell<MemoryContext> = const { Cell::new(core::ptr::null_mut()) };
    static TOP: Cell<MemoryContext> = const { Cell::new(core::ptr::null_mut()) };
    // Contexts created through AllocSetContextCreateInternal (boxed by us),
    // with the parent they were created under.
    static OWNED: RefCell<Vec<(usize, MemoryContext)>> = const { RefCell::new(Vec::new()) };
    static OWNED_SET: RefCell<HashSet<usize>> = RefCell::new(HashSet::new());
    // Live pgrx-side chunks (user pointers). pgrx also pfrees pointers that
    // pgrust allocated (detoast results, builtin outputs); those are not ours
    // to free and stay in their context until it resets.
    static CHUNKS: RefCell<HashSet<usize>> = RefCell::new(HashSet::new());
}

#[inline]
fn is_chunk(ptr: *mut c_void) -> bool {
    CHUNKS.with(|c| c.borrow().contains(&(ptr as usize)))
}

/// The per-thread top context: a session-lifetime root pgrust reclaims at
/// session end. Created lazily on first use.
pub fn top_memory_context() -> MemoryContext {
    TOP.with(|t| {
        let p = t.get();
        if !p.is_null() {
            return p;
        }
        let root: &'static ::pgr_mcx::MemoryContext =
            ::pgr_mcx::session_root("pgrx TopMemoryContext");
        let p = handle_of(root);
        t.set(p);
        p
    })
}

/// C's `CurrentMemoryContext` read. Falls back to the top context when no
/// call wrapper has armed one (pgrx code running outside a function call).
#[inline]
pub fn current_memory_context() -> MemoryContext {
    let p = CURRENT.with(|c| c.get());
    if p.is_null() { top_memory_context() } else { p }
}

/// C's `CurrentMemoryContext = ctx`.
#[inline]
pub fn set_current_memory_context(ctx: MemoryContext) {
    CURRENT.with(|c| c.set(ctx));
}

/// C's `MemoryContextSwitchTo`.
#[inline(always)]
pub unsafe fn MemoryContextSwitchTo(context: MemoryContext) -> MemoryContext {
    let old = current_memory_context();
    set_current_memory_context(context);
    old
}

// The other well-known contexts pgrx names. pgrust has no process-lifetime
// analogues of these that extension code may allocate into, so they all
// resolve to the per-thread top context.
pub fn error_context() -> MemoryContext {
    top_memory_context()
}
pub fn portal_context() -> MemoryContext {
    current_memory_context()
}
pub fn cache_memory_context() -> MemoryContext {
    top_memory_context()
}
pub fn message_context() -> MemoryContext {
    current_memory_context()
}
pub fn top_transaction_context() -> MemoryContext {
    top_memory_context()
}
pub fn cur_transaction_context() -> MemoryContext {
    top_memory_context()
}
pub fn postmaster_context() -> MemoryContext {
    top_memory_context()
}

#[inline]
fn mcx<'a>(ctx: MemoryContext) -> ::pgr_mcx::Mcx<'a> {
    // SAFETY: every MemoryContext handed to pgrx code points at a live pgrust
    // context (the call's result context, the top context, or one we boxed).
    unsafe { native_ctx(ctx).mcx() }
}

const CHUNK_MAGIC: usize = 0x7067_7278_6368_6e6b; // "pgrxchnk"
const CHUNK_ALIGN: usize = 16;

/// Prefix of every pgrx-side allocation. 48 bytes with the alignment padding,
/// so the user pointer keeps 16-byte alignment.
#[repr(C, align(16))]
struct ChunkHeader {
    ctx: MemoryContext,
    base: *mut u8,
    size: usize,
    total: usize,
    magic: usize,
}

const HDR: usize = core::mem::size_of::<ChunkHeader>();

#[inline]
unsafe fn header_of(ptr: *mut c_void) -> *mut ChunkHeader {
    let h = (ptr as *mut u8).sub(HDR) as *mut ChunkHeader;
    debug_assert_eq!(
        (*h).magic,
        CHUNK_MAGIC,
        "pfree of a pointer not allocated by pgrx palloc"
    );
    h
}

fn alloc_in(
    ctx: MemoryContext,
    size: usize,
    align: usize,
    zero: bool,
    no_oom: bool,
) -> *mut c_void {
    let align = align.max(CHUNK_ALIGN);
    let total = size + HDR + (align - CHUNK_ALIGN);
    let layout = Layout::from_size_align(total.max(1), CHUNK_ALIGN).expect("palloc layout");
    let m = mcx(ctx);
    let base = match m.allocate(layout) {
        Ok(p) => p.cast::<u8>().as_ptr(),
        Err(_) if no_oom => return core::ptr::null_mut(),
        Err(_) => raise(Box::new(m.oom(size))),
    };
    // user pointer: first `align`-aligned address at or after base + HDR
    let user = {
        let start = base as usize + HDR;
        ((start + align - 1) & !(align - 1)) as *mut u8
    };
    // SAFETY: header lies inside [base, base+total) by construction.
    unsafe {
        let h = user.sub(HDR) as *mut ChunkHeader;
        h.write(ChunkHeader {
            ctx,
            base,
            size,
            total,
            magic: CHUNK_MAGIC,
        });
        if zero {
            core::ptr::write_bytes(user, 0, size);
        }
    }
    CHUNKS.with(|c| c.borrow_mut().insert(user as usize));
    user as *mut c_void
}

pub unsafe fn MemoryContextAlloc(context: MemoryContext, size: Size) -> *mut c_void {
    alloc_in(context, size, CHUNK_ALIGN, false, false)
}

pub unsafe fn MemoryContextAllocZero(context: MemoryContext, size: Size) -> *mut c_void {
    alloc_in(context, size, CHUNK_ALIGN, true, false)
}

pub unsafe fn MemoryContextAllocExtended(
    context: MemoryContext,
    size: Size,
    flags: c_int,
) -> *mut c_void {
    let zero = flags & MCXT_ALLOC_ZERO as c_int != 0;
    let no_oom = flags & super::types::MCXT_ALLOC_NO_OOM as c_int != 0;
    alloc_in(context, size, CHUNK_ALIGN, zero, no_oom)
}

pub unsafe fn MemoryContextAllocAligned(
    context: MemoryContext,
    size: Size,
    alignto: Size,
    flags: c_int,
) -> *mut c_void {
    let zero = flags & MCXT_ALLOC_ZERO as c_int != 0;
    let no_oom = flags & super::types::MCXT_ALLOC_NO_OOM as c_int != 0;
    alloc_in(context, size, alignto, zero, no_oom)
}

pub unsafe fn palloc(size: Size) -> *mut c_void {
    alloc_in(current_memory_context(), size, CHUNK_ALIGN, false, false)
}

pub unsafe fn palloc0(size: Size) -> *mut c_void {
    alloc_in(current_memory_context(), size, CHUNK_ALIGN, true, false)
}

/// Allocate a copy of `bytes` on the current context (the shape most shims
/// use to hand pgrust-produced images to pgrx code that may `pfree` them).
pub fn palloc_copy(bytes: &[u8]) -> *mut u8 {
    // SAFETY: fresh allocation of exactly bytes.len() bytes.
    unsafe {
        let p = palloc(bytes.len()) as *mut u8;
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
        p
    }
}

pub unsafe fn pfree(pointer: *mut c_void) {
    if pointer.is_null() || !CHUNKS.with(|c| c.borrow_mut().remove(&(pointer as usize))) {
        // not a pgrx chunk: pgrust owns it, its context frees it
        return;
    }
    let h = header_of(pointer);
    let ChunkHeader {
        ctx, base, total, ..
    } = h.read();
    let layout = Layout::from_size_align(total.max(1), CHUNK_ALIGN).expect("pfree layout");
    mcx(ctx).deallocate(NonNull::new_unchecked(base), layout);
}

pub unsafe fn repalloc(pointer: *mut c_void, size: Size) -> *mut c_void {
    if !is_chunk(pointer) {
        raise(Box::new(::pgr_types_error::PgError::error(
            "pgrx: repalloc of memory not allocated by palloc is not supported under pgrust",
        )));
    }
    let h = header_of(pointer);
    let old = (*h).size;
    let ctx = (*h).ctx;
    let new = alloc_in(ctx, size, CHUNK_ALIGN, false, false);
    core::ptr::copy_nonoverlapping(pointer as *const u8, new as *mut u8, old.min(size));
    pfree(pointer);
    new
}

pub unsafe fn GetMemoryChunkContext(pointer: *mut c_void) -> MemoryContext {
    if !is_chunk(pointer) {
        return current_memory_context();
    }
    (*header_of(pointer)).ctx
}

pub unsafe fn MemoryContextIsValid(context: MemoryContext) -> bool {
    !context.is_null()
}

pub unsafe fn pstrdup(in_: *const c_char) -> *mut c_char {
    let len = core::ffi::CStr::from_ptr(in_).to_bytes().len();
    let p = palloc(len + 1) as *mut c_char;
    core::ptr::copy_nonoverlapping(in_, p, len + 1);
    p
}

pub unsafe fn pnstrdup(in_: *const c_char, len: Size) -> *mut c_char {
    // C: copies at most len bytes, stopping at the first NUL.
    let bytes = core::slice::from_raw_parts(in_ as *const u8, len);
    let n = bytes.iter().position(|&b| b == 0).unwrap_or(len);
    let p = palloc(n + 1) as *mut c_char;
    core::ptr::copy_nonoverlapping(in_, p, n);
    *p.add(n) = 0;
    p
}

pub unsafe fn MemoryContextStrdup(context: MemoryContext, string: *const c_char) -> *mut c_char {
    let len = core::ffi::CStr::from_ptr(string).to_bytes().len();
    let p = MemoryContextAlloc(context, len + 1) as *mut c_char;
    core::ptr::copy_nonoverlapping(string, p, len + 1);
    p
}

/// C's AllocSetContextCreate: a child context pgrx owns and will delete.
pub unsafe fn AllocSetContextCreateInternal(
    parent: MemoryContext,
    name: *const c_char,
    _min_context_size: Size,
    _init_block_size: Size,
    _max_block_size: Size,
) -> MemoryContext {
    let parent = if parent.is_null() {
        top_memory_context()
    } else {
        parent
    };
    let name: &'static str = if name.is_null() {
        "pgrx context"
    } else {
        Box::leak(
            core::ffi::CStr::from_ptr(name)
                .to_string_lossy()
                .into_owned()
                .into_boxed_str(),
        )
    };
    let child = native_ctx(parent).new_child(name);
    let p = Box::into_raw(Box::new(child)) as MemoryContext;
    OWNED.with(|o| o.borrow_mut().push((p as usize, parent)));
    OWNED_SET.with(|s| s.borrow_mut().insert(p as usize));
    p
}

/// pgrx spells the C macro's name; same thing.
pub unsafe fn AllocSetContextCreateExtended(
    parent: MemoryContext,
    name: *const c_char,
    min_context_size: Size,
    init_block_size: Size,
    max_block_size: Size,
) -> MemoryContext {
    AllocSetContextCreateInternal(
        parent,
        name,
        min_context_size,
        init_block_size,
        max_block_size,
    )
}

fn is_owned(ctx: MemoryContext) -> bool {
    OWNED_SET.with(|s| s.borrow().contains(&(ctx as usize)))
}

pub unsafe fn MemoryContextDelete(context: MemoryContext) {
    if context.is_null() || !is_owned(context) {
        // Contexts owned by pgrust (the executor's) are never deleted from here.
        return;
    }
    if current_memory_context() == context {
        set_current_memory_context(top_memory_context());
    }
    OWNED.with(|o| o.borrow_mut().retain(|(p, _)| *p != context as usize));
    OWNED_SET.with(|s| s.borrow_mut().remove(&(context as usize)));
    drop(Box::from_raw(context as *mut ::pgr_mcx::MemoryContext));
}

pub unsafe fn MemoryContextReset(context: MemoryContext) {
    (*(context as *mut ::pgr_mcx::MemoryContext)).reset();
}

pub unsafe fn MemoryContextGetParent(context: MemoryContext) -> MemoryContext {
    OWNED.with(|o| {
        o.borrow()
            .iter()
            .find(|(p, _)| *p == context as usize)
            .map(|(_, parent)| *parent)
            .unwrap_or(core::ptr::null_mut())
    })
}

pub unsafe fn MemoryContextRegisterResetCallback(
    context: MemoryContext,
    cb: *mut MemoryContextCallback,
) {
    let cb = cb as usize;
    native_ctx(context).register_reset_callback(move || {
        let cb = cb as *mut MemoryContextCallback;
        // SAFETY: the callback struct was allocated in this context by pgrx
        // and is still live when the context's reset callbacks fire.
        unsafe {
            if let Some(f) = (*cb).func {
                f((*cb).arg);
            }
        }
    });
}
