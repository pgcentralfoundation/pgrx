//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Safe RAII wrapper around Postgres' BufferManager (`ReadBuffer`, `LockBuffer`,
//! page access).
//!
//! # SAFETY invariants
//!
//! 1. **Pin lifetime.** [`PgBuffer`] is the sole pin owner. `Drop` calls
//!    `ReleaseBuffer` exactly once iff [`PgBuffer::is_valid`]. The type is
//!    `!Clone` and `!Copy`, ruling out double-release.
//! 2. **No leak across xact / longjmp.** pgrx uses `pg_guard` longjmp
//!    semantics, so Rust `Drop` is **not** guaranteed across `elog(ERROR)`.
//!    Same constraint as [`crate::rel::PgRelation`]. Keep `PgBuffer` short-
//!    lived inside a single Postgres callback or `#[pg_extern]` body.
//! 3. **Page lifetime ⊆ guard lifetime.** [`Page`] / [`PageMut`] are bound
//!    to the guard's lifetime via `PhantomData`; after the guard drops, no
//!    page reference can survive.
//! 4. **Guard exclusivity.** [`PgBuffer::read_lock`] / [`PgBuffer::write_lock`]
//!    take `&mut self`, so the borrow checker forbids two live guards on the
//!    same `PgBuffer`.
//! 5. **No aliased mutation.** [`PageMut::as_bytes_mut`] requires `&mut self`
//!    on the guard, while [`BufferWriteGuard::page`] / [`BufferWriteGuard::header`]
//!    borrow `&self`; the borrow checker prevents simultaneous shared and
//!    exclusive page views.
//! 6. **Invalid buffers.** Under `RBM_NORMAL`, PG raises ERROR rather than
//!    returning an invalid buffer, so [`PgBuffer::read`] never observes one.
//!    [`PgBuffer::read_extended`] with [`ReadBufferMode::ZeroOnError`] can.
//!    Lock-family methods panic with a clear message if `!is_valid()`; `Drop`
//!    skips `ReleaseBuffer` on invalid buffers.
//! 7. **Item bounds.** [`Page::item`] resolves an `ItemId`'s `(lp_off, lp_len)`
//!    and bounds-checks against `BLCKSZ` before returning `Some(&[u8])`. Corrupt
//!    pages produce `None`, never out-of-bounds reads.

use crate::pg_sys;
use crate::rel::PgRelation;
use core::marker::PhantomData;

/// Postgres fork numbers, version-stable subset.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum ForkNumber {
    Main = 0,
    Fsm = 1,
    VisibilityMap = 2,
    Init = 3,
}

impl ForkNumber {
    #[inline]
    fn to_pg(self) -> pg_sys::ForkNumber::Type {
        self as pg_sys::ForkNumber::Type
    }
}

/// Subset of `pg_sys::ReadBufferMode` exposed across pg13–pg18.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReadBufferMode {
    Normal,
    ZeroAndLock,
    ZeroAndCleanupLock,
    ZeroOnError,
    NormalNoLog,
}

impl ReadBufferMode {
    #[inline]
    fn to_pg(self) -> pg_sys::ReadBufferMode::Type {
        match self {
            ReadBufferMode::Normal => pg_sys::ReadBufferMode::RBM_NORMAL,
            ReadBufferMode::ZeroAndLock => pg_sys::ReadBufferMode::RBM_ZERO_AND_LOCK,
            ReadBufferMode::ZeroAndCleanupLock => pg_sys::ReadBufferMode::RBM_ZERO_AND_CLEANUP_LOCK,
            ReadBufferMode::ZeroOnError => pg_sys::ReadBufferMode::RBM_ZERO_ON_ERROR,
            ReadBufferMode::NormalNoLog => pg_sys::ReadBufferMode::RBM_NORMAL_NO_LOG,
        }
    }
}

/// Owns a buffer pin. `Drop` calls `ReleaseBuffer` exactly once for valid buffers.
///
/// SAFETY INVARIANTS (see module docs):
/// - Sole pin owner (no `Copy`, no `Clone`).
/// - Not safe across `elog(ERROR)` / panic; keep short-lived inside a single Postgres callback or `#[pg_extern]` body.
pub struct PgBuffer {
    raw: pg_sys::Buffer,
}

impl PgBuffer {
    /// `ReadBuffer(rel, blockNum)`. Defaults to MAIN fork, RBM_NORMAL.
    ///
    /// # Safety
    /// `rel` must remain open for the lifetime of the returned `PgBuffer`.
    #[inline]
    pub unsafe fn read(rel: &PgRelation, block: pg_sys::BlockNumber) -> Self {
        let raw = unsafe { pg_sys::ReadBuffer(rel.as_ptr(), block) };
        PgBuffer { raw }
    }

    /// `ReadBufferExtended(rel, fork, block, mode, NULL)`.
    ///
    /// # Safety
    /// `rel` must remain open for the lifetime of the returned `PgBuffer`, with `ReadBufferMode::ZeroOnError` the result may be an invalid buffer; callers should consult [`PgBuffer::is_valid`] before locking.
    #[inline]
    pub unsafe fn read_extended(
        rel: &PgRelation,
        fork: ForkNumber,
        block: pg_sys::BlockNumber,
        mode: ReadBufferMode,
    ) -> Self {
        let raw = unsafe {
            pg_sys::ReadBufferExtended(
                rel.as_ptr(),
                fork.to_pg(),
                block,
                mode.to_pg(),
                core::ptr::null_mut(),
            )
        };
        PgBuffer { raw }
    }

    /// Raw `pg_sys::Buffer` handle. Provided for FFI escape; do not release manually.
    #[inline]
    pub fn as_raw(&self) -> pg_sys::Buffer {
        self.raw
    }

    /// `BufferIsValid`. False only for the zero / invalid sentinel.
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.raw != pg_sys::InvalidBuffer as pg_sys::Buffer
    }

    /// `BufferGetBlockNumber`. Panics on invalid buffers.
    #[inline]
    pub fn block_number(&self) -> pg_sys::BlockNumber {
        assert!(self.is_valid(), "PgBuffer::block_number on invalid buffer");
        unsafe { pg_sys::BufferGetBlockNumber(self.raw) }
    }
}

impl Drop for PgBuffer {
    fn drop(&mut self) {
        if self.is_valid() {
            unsafe { pg_sys::ReleaseBuffer(self.raw) };
        }
    }
}

/// Immutable view of a buffer page. Lifetime bound to a lock guard.
pub struct Page<'g> {
    ptr: *const u8,
    _g: PhantomData<&'g ()>,
}

/// Mutable view of a buffer page. Lifetime bound to a write guard.
pub struct PageMut<'g> {
    ptr: *mut u8,
    _g: PhantomData<&'g mut ()>,
}

/// Read-only view of `PageHeaderData`.
pub struct PageHeaderRef<'g> {
    ptr: *const pg_sys::PageHeaderData,
    _g: PhantomData<&'g ()>,
}

impl<'g> Page<'g> {
    /// Postgres page size (`BLCKSZ`).
    pub const SIZE: usize = pg_sys::BLCKSZ as usize;

    /// Whole page as a fixed-size byte array.
    ///
    /// ```compile_fail
    /// # use pgrx::buffer::PgBuffer;
    /// fn page_outlives_guard(mut buf: PgBuffer) {
    ///     let page_bytes = {
    ///         let g = buf.read_lock();
    ///         g.page().as_bytes()
    ///     }; // guard dropped here
    ///     let _ = page_bytes[0]; // ERROR: borrowed value does not live long enough
    /// }
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &'g [u8; Page::SIZE] {
        // SAFETY: PG guarantees BLCKSZ-sized backing storage; lifetime tied to guard.
        unsafe { &*(self.ptr as *const [u8; Page::SIZE]) }
    }

    /// `PageGetMaxOffsetNumber`: the highest valid line pointer (0 if empty).
    pub fn max_offset(&self) -> pg_sys::OffsetNumber {
        max_offset_number(self.ptr)
    }

    /// `PageGetItem(PageGetItemId(page, off))` with bounds checks. Returns `None` for out-of-range offsets, unused line pointers, or items whose `(lp_off, lp_len)` would exceed `BLCKSZ`.
    pub fn item(&self, off: pg_sys::OffsetNumber) -> Option<&'g [u8]> {
        unsafe {
            page_get_item_safe(self.ptr, off).map(|(p, len)| core::slice::from_raw_parts(p, len))
        }
    }
}

impl<'g> PageMut<'g> {
    #[inline]
    pub fn as_bytes(&self) -> &[u8; Page::SIZE] {
        unsafe { &*(self.ptr as *const [u8; Page::SIZE]) }
    }
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8; Page::SIZE] {
        unsafe { &mut *(self.ptr as *mut [u8; Page::SIZE]) }
    }
    pub fn max_offset(&self) -> pg_sys::OffsetNumber {
        max_offset_number(self.ptr as *const u8)
    }
    pub fn item(&self, off: pg_sys::OffsetNumber) -> Option<&[u8]> {
        unsafe {
            page_get_item_safe(self.ptr as *const u8, off)
                .map(|(p, len)| core::slice::from_raw_parts(p, len))
        }
    }
}

impl<'g> PageHeaderRef<'g> {
    #[inline]
    fn header(&self) -> &pg_sys::PageHeaderData {
        unsafe { &*self.ptr }
    }
    pub fn lsn(&self) -> pg_sys::XLogRecPtr {
        // pd_lsn is a struct of two u32 halves; reassemble.
        let h = self.header();
        ((h.pd_lsn.xlogid as u64) << 32) | (h.pd_lsn.xrecoff as u64)
    }
    pub fn checksum(&self) -> u16 {
        self.header().pd_checksum
    }
    pub fn flags(&self) -> u16 {
        self.header().pd_flags
    }
    pub fn lower(&self) -> u16 {
        self.header().pd_lower
    }
    pub fn upper(&self) -> u16 {
        self.header().pd_upper
    }
    pub fn special(&self) -> u16 {
        self.header().pd_special
    }
    pub fn page_version(&self) -> u16 {
        // pd_pagesize_version stores size|version; mask 8 low bits.
        self.header().pd_pagesize_version & 0x00FF
    }
}

// ---- internal page item helpers (pure Rust, no cshim) ---------------------
//
// PostgreSQL page layout (per src/include/storage/bufpage.h):
//   [ PageHeaderData (24 bytes) | ItemIdData[N] | ... free ... | items ]
// pd_lower = end-of-line-pointer-array offset
// pd_upper = start-of-items offset
//
// max_offset = (pd_lower - SizeOfPageHeaderData) / sizeof(ItemIdData)

const SIZE_OF_PAGE_HEADER_DATA: usize = core::mem::size_of::<pg_sys::PageHeaderData>();
const SIZE_OF_ITEM_ID_DATA: usize = core::mem::size_of::<pg_sys::ItemIdData>();

#[inline]
fn max_offset_number(page: *const u8) -> pg_sys::OffsetNumber {
    // SAFETY: caller guarantees `page` points to a BLCKSZ page.
    let h = unsafe { &*(page as *const pg_sys::PageHeaderData) };
    let lower = h.pd_lower as usize;
    if lower <= SIZE_OF_PAGE_HEADER_DATA || lower > Page::SIZE {
        0
    } else {
        ((lower - SIZE_OF_PAGE_HEADER_DATA) / SIZE_OF_ITEM_ID_DATA) as pg_sys::OffsetNumber
    }
}

/// Returns `(item_ptr, item_len)` for a valid in-bounds item, else `None`.
unsafe fn page_get_item_safe(
    page: *const u8,
    off: pg_sys::OffsetNumber,
) -> Option<(*const u8, usize)> {
    if off == 0 {
        return None;
    }
    let max = max_offset_number(page);
    if off > max {
        return None;
    }

    // ItemIdData lives at pd_linp[off-1]; pd_linp begins right after the header.
    let lp_array = unsafe { page.add(SIZE_OF_PAGE_HEADER_DATA) } as *const pg_sys::ItemIdData;
    let lp = unsafe { &*lp_array.add((off - 1) as usize) };

    // pgrx-pg-sys exposes lp_off/lp_len/lp_flags as packed-bitfield accessors.
    let lp_flags = lp.lp_flags();
    // Only LP_NORMAL line pointers have a valid (lp_off, lp_len) payload.
    if lp_flags != pg_sys::LP_NORMAL {
        return None;
    }
    let lp_off = lp.lp_off() as usize;
    let lp_len = lp.lp_len() as usize;
    if lp_off == 0 || lp_off + lp_len > Page::SIZE {
        return None;
    }
    Some((unsafe { page.add(lp_off) }, lp_len))
}

// pg_sys exposes these as u32 constants (BUFFER_LOCK_UNLOCK = 0, BUFFER_LOCK_SHARE = 1, BUFFER_LOCK_EXCLUSIVE = 2). LockBuffer expects c_int.
const BUFFER_LOCK_UNLOCK: core::ffi::c_int = pg_sys::BUFFER_LOCK_UNLOCK as core::ffi::c_int;
const BUFFER_LOCK_SHARE: core::ffi::c_int = pg_sys::BUFFER_LOCK_SHARE as core::ffi::c_int;
const BUFFER_LOCK_EXCLUSIVE: core::ffi::c_int = pg_sys::BUFFER_LOCK_EXCLUSIVE as core::ffi::c_int;

/// Shared (read) lock guard. `Drop` releases the lock; pin is retained by PgBuffer.
pub struct BufferReadGuard<'b> {
    buf: &'b mut PgBuffer,
}

/// Exclusive (write) lock guard. `Drop` releases the lock; pin is retained.
pub struct BufferWriteGuard<'b> {
    buf: &'b mut PgBuffer,
}

impl PgBuffer {
    fn ensure_valid_for_lock(&self) {
        assert!(self.is_valid(), "cannot lock an invalid PgBuffer");
    }

    /// `LockBuffer(BUFFER_LOCK_SHARE)`. Panics on invalid buffers.
    ///
    /// Two simultaneous guards on the same buffer are forbidden by the borrow checker:
    ///
    /// ```compile_fail
    /// # use pgrx::buffer::PgBuffer;
    /// fn cannot_double_lock(mut buf: PgBuffer) {
    ///     let g1 = buf.read_lock();
    ///     let g2 = buf.read_lock(); // ERROR: second &mut borrow of `buf`
    ///     drop((g1, g2));
    /// }
    /// ```
    pub fn read_lock(&mut self) -> BufferReadGuard<'_> {
        self.ensure_valid_for_lock();
        unsafe { pg_sys::LockBuffer(self.raw, BUFFER_LOCK_SHARE) };
        BufferReadGuard { buf: self }
    }

    /// `LockBuffer(BUFFER_LOCK_EXCLUSIVE)`. Panics on invalid buffers.
    pub fn write_lock(&mut self) -> BufferWriteGuard<'_> {
        self.ensure_valid_for_lock();
        unsafe { pg_sys::LockBuffer(self.raw, BUFFER_LOCK_EXCLUSIVE) };
        BufferWriteGuard { buf: self }
    }

    /// `ConditionalLockBuffer`. Postgres only ships the exclusive variant.
    pub fn try_write_lock(&mut self) -> Option<BufferWriteGuard<'_>> {
        self.ensure_valid_for_lock();
        let got = unsafe { pg_sys::ConditionalLockBuffer(self.raw) };
        if got { Some(BufferWriteGuard { buf: self }) } else { None }
    }
}

impl<'b> BufferReadGuard<'b> {
    #[inline]
    pub fn page(&self) -> Page<'_> {
        Page { ptr: unsafe { pg_sys::BufferGetPage(self.buf.raw) } as *const u8, _g: PhantomData }
    }
    #[inline]
    pub fn header(&self) -> PageHeaderRef<'_> {
        PageHeaderRef {
            ptr: unsafe { pg_sys::BufferGetPage(self.buf.raw) } as *const pg_sys::PageHeaderData,
            _g: PhantomData,
        }
    }
    /// Owning buffer (for `block_number`, `is_valid` queries during the lock).
    #[inline]
    pub fn buffer(&self) -> &PgBuffer {
        self.buf
    }
}

impl<'b> Drop for BufferReadGuard<'b> {
    fn drop(&mut self) {
        unsafe { pg_sys::LockBuffer(self.buf.raw, BUFFER_LOCK_UNLOCK) };
    }
}

impl<'b> BufferWriteGuard<'b> {
    #[inline]
    pub fn page(&self) -> Page<'_> {
        Page { ptr: unsafe { pg_sys::BufferGetPage(self.buf.raw) } as *const u8, _g: PhantomData }
    }
    #[inline]
    pub fn page_mut(&mut self) -> PageMut<'_> {
        PageMut { ptr: unsafe { pg_sys::BufferGetPage(self.buf.raw) } as *mut u8, _g: PhantomData }
    }
    #[inline]
    pub fn header(&self) -> PageHeaderRef<'_> {
        PageHeaderRef {
            ptr: unsafe { pg_sys::BufferGetPage(self.buf.raw) } as *const pg_sys::PageHeaderData,
            _g: PhantomData,
        }
    }
    /// `MarkBufferDirty`. The caller must hold this exclusive guard.
    #[inline]
    pub fn mark_dirty(&mut self) {
        unsafe { pg_sys::MarkBufferDirty(self.buf.raw) };
    }
    #[inline]
    pub fn buffer(&self) -> &PgBuffer {
        self.buf
    }
}

impl<'b> Drop for BufferWriteGuard<'b> {
    fn drop(&mut self) {
        unsafe { pg_sys::LockBuffer(self.buf.raw, BUFFER_LOCK_UNLOCK) };
    }
}

#[doc(hidden)]
pub fn __test_only_invalid_pgbuffer() -> PgBuffer {
    PgBuffer { raw: pg_sys::InvalidBuffer as pg_sys::Buffer }
}
