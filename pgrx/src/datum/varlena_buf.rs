//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Safe, lifetime-bound varlena allocation.
//!
//! [`MemCx::alloc_varlena`] returns a [`VarlenaBuf<'mcx>`] whose 4-byte header
//! is set to a valid total size before the wrapper exists. Writing into the
//! payload via [`VarlenaBuf::payload_mut`] cannot resize, so the header
//! invariant is preserved.
//!
//! # Examples
//!
//! Building a bytea and handing it to Postgres:
//!
//! ```no_run
//! use pgrx::memcx;
//! use pgrx::pg_sys;
//!
//! memcx::current_context(|cx| {
//!     let mut buf = cx.alloc_varlena(11).expect("OOM");
//!     buf.payload_mut().copy_from_slice(b"hello world");
//!     let ptr: *mut pg_sys::varlena = buf.into_raw();
//!     // `ptr` can be returned from a #[pg_extern] as a varlena Datum.
//!     // The allocation lives in `cx` until the memory context resets.
//!     let _ = ptr;
//! });
//! ```
//!
//! See: `docs/superpowers/specs/2026-06-15-memcx-slice-varlena-parity-design.md`

use crate::memcx::MemCx;
use crate::palloc::PBox;
use crate::pg_sys;
use core::ptr::NonNull;

/// A `varlena` allocation with a valid 4-byte header, maintained as a type invariant.
///
/// `RawVarlena` is a DST whose in-memory representation is the full varlena byte
/// sequence (header + payload). Safely obtainable only through
/// [`MemCx::alloc_varlena`] (which sets the header) or [`MemCx::inspect_varlena`]
/// (caller-asserted).
///
/// # Layout
///
/// `#[repr(transparent)]` over `[u8]`. The header is at byte offset 0; payload
/// starts at offset `VARHDRSZ` (= 4). The total length matches the header's
/// stored size.
#[repr(transparent)]
pub struct RawVarlena {
    bytes: [u8],
}

impl RawVarlena {
    /// Total size in bytes (header + payload), as recorded in the header.
    pub fn total_size(&self) -> usize {
        // SAFETY: type invariant — header at offset 0 is a valid 4-byte varlena header.
        unsafe { crate::varlena::varsize_any(self.as_ptr()) }
    }

    /// Read-only access to the payload bytes (excludes the header).
    pub fn payload(&self) -> &[u8] {
        let total = self.total_size();
        let hdr = pg_sys::VARHDRSZ;
        let payload_len = total.checked_sub(hdr).expect("corrupt varlena: total < VARHDRSZ");
        // SAFETY: payload starts at offset VARHDRSZ; `checked_sub` above rules out
        // an underflowed length on a corrupt header (release-mode UB guard).
        unsafe {
            core::slice::from_raw_parts((self as *const Self as *const u8).add(hdr), payload_len)
        }
    }

    /// Mutable access to the payload bytes. Cannot be used to grow or shrink:
    /// the header (and therefore `total_size`) is unchanged.
    pub fn payload_mut(&mut self) -> &mut [u8] {
        let total = self.total_size();
        let hdr = pg_sys::VARHDRSZ;
        let payload_len = total.checked_sub(hdr).expect("corrupt varlena: total < VARHDRSZ");
        // SAFETY: payload starts at offset VARHDRSZ; `checked_sub` above rules out
        // an underflowed length on a corrupt header. We hold `&mut self`, so the
        // borrow is exclusive.
        unsafe {
            core::slice::from_raw_parts_mut((self as *mut Self as *mut u8).add(hdr), payload_len)
        }
    }

    /// Raw `*const pg_sys::varlena` pointer. The pointee remains valid for
    /// the lifetime of `&self`.
    pub fn as_ptr(&self) -> *const pg_sys::varlena {
        self as *const Self as *const pg_sys::varlena
    }

    /// Raw `*mut pg_sys::varlena` pointer.
    pub fn as_mut_ptr(&mut self) -> *mut pg_sys::varlena {
        self as *mut Self as *mut pg_sys::varlena
    }
}

/// A varlena buffer: a `PBox`-owned `RawVarlena`, scoped to a memory context.
pub struct VarlenaBuf<'mcx> {
    inner: PBox<'mcx, RawVarlena>,
}

impl<'mcx> VarlenaBuf<'mcx> {
    /// # Safety
    /// `data_ptr` must point to a valid varlena allocation in `cx` whose header has
    /// already been set, and the total byte length must equal `total`.
    pub(crate) unsafe fn from_raw(data_ptr: NonNull<u8>, total: usize, cx: &MemCx<'mcx>) -> Self {
        // Build a *mut [u8] fat pointer of the right length, then cast to
        // *mut RawVarlena (sound: RawVarlena is #[repr(transparent)] over [u8]).
        let slice_ptr: *mut [u8] = core::ptr::slice_from_raw_parts_mut(data_ptr.as_ptr(), total);
        let raw_ptr: *mut RawVarlena = slice_ptr as *mut RawVarlena;
        // SAFETY: data_ptr was non-null, so the fat pointer's data pointer is non-null.
        let nn = NonNull::new_unchecked(raw_ptr);
        let inner = PBox::from_raw_in(nn, cx);
        VarlenaBuf { inner }
    }

    /// Read-only payload access.
    pub fn payload(&self) -> &[u8] {
        // SAFETY: `inner.ptr` is a valid fat pointer to a RawVarlena whose header
        // is valid by construction.
        unsafe { (&*self.inner.as_ptr()).payload() }
    }

    /// Mutable payload access.
    pub fn payload_mut(&mut self) -> &mut [u8] {
        // SAFETY: `inner` exclusively owns the allocation; we hold `&mut self`.
        unsafe { (&mut *self.inner.as_mut_ptr()).payload_mut() }
    }

    /// Total size (header + payload) as recorded in the header.
    pub fn total_size(&self) -> usize {
        // SAFETY: same as `payload`.
        unsafe { (&*self.inner.as_ptr()).total_size() }
    }

    /// Convert into the underlying `PBox<RawVarlena>`. Ownership of the
    /// memory-context-bound allocation is preserved.
    pub fn into_pbox(self) -> PBox<'mcx, RawVarlena> {
        self.inner
    }

    /// Leak the `PBox` and return a raw `*mut pg_sys::varlena` for handoff
    /// to Postgres (e.g., as a function return). The allocation remains in
    /// `cx` and is reclaimed at context reset.
    pub fn into_raw(self) -> *mut pg_sys::varlena {
        let mut me = core::mem::ManuallyDrop::new(self);
        me.inner.as_mut_ptr() as *mut pg_sys::varlena
    }
}

// --- Returning `PBox<RawVarlena>` directly from a `#[pg_extern]` -----------
//
// `RawVarlena` deliberately does NOT implement `BorrowDatum` (it has no
// fixed in-Datum representation: a varlena is always pass-by-reference and
// its size is header-driven). The blanket `BoxRet`/`SqlTranslatable` impls
// for `PBox<T: BorrowDatum>` in `palloc/pbox.rs` therefore do not cover it,
// and we add specific impls here.

use crate::callconv::{BoxRet, FcInfo};
use crate::datum::Datum;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, ReturnsError, ReturnsRef, SqlMappingRef, SqlTranslatable,
};

/// Returning a `PBox<'mcx, RawVarlena>` from a `#[pg_extern]` hands the
/// underlying varlena pointer to Postgres unchanged. No copy: the
/// allocation stays in the caller's `MemCx` and is reclaimed on context
/// reset.
unsafe impl<'mcx> BoxRet for PBox<'mcx, RawVarlena> {
    unsafe fn box_into<'fcx>(self, fcinfo: &mut FcInfo<'fcx>) -> Datum<'fcx> {
        // Leak the PBox; ownership of the allocation transfers to Postgres.
        let mut me = core::mem::ManuallyDrop::new(self);
        let ptr = me.as_mut_ptr() as *mut pg_sys::varlena;
        // SAFETY: `ptr` points at a valid varlena allocation in the caller's
        // memory context; Postgres owns it from here on.
        unsafe { fcinfo.return_raw_datum(pg_sys::Datum::from(ptr)) }
    }
}

/// `PBox<'mcx, RawVarlena>` maps to SQL `bytea`.
unsafe impl<'mcx> SqlTranslatable for PBox<'mcx, RawVarlena> {
    const TYPE_IDENT: &'static str = "bytea";
    const TYPE_ORIGIN: pgrx_sql_entity_graph::metadata::TypeOrigin =
        pgrx_sql_entity_graph::metadata::TypeOrigin::External;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Ok(SqlMappingRef::literal("bytea"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("bytea")));
}
