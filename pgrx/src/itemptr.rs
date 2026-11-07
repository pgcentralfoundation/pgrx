//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Helper functions for working with Postgres `ItemPointerData` (`tid`) type
#![allow(
    unused_unsafe,
    reason = "docs.rs builds replace generated C macros with shells, some of which need no unsafe"
)]

use crate::PgMemoryContexts;
use crate::datum::{FromDatum, IntoDatum};
use crate::{AllocatedByRust, PgBox, pg_sys};

/// Decode a Rust-owned item pointer into its block and offset numbers, without validity assertions.
#[inline]
pub fn item_pointer_get_both(
    ctid: pg_sys::ItemPointerData,
) -> (pg_sys::BlockNumber, pg_sys::OffsetNumber) {
    // SAFETY: ctid is initialized native storage and stays live for both non-mutating reads.
    unsafe {
        let ptr = &raw const ctid;
        (
            pg_sys::ItemPointerGetBlockNumberNoCheck!(ptr).get(),
            pg_sys::ItemPointerGetOffsetNumberNoCheck!(ptr).get(),
        )
    }
}

/// Convert an `ItemPointerData` struct into a `u64`
#[inline]
pub fn item_pointer_to_u64(ctid: pg_sys::ItemPointerData) -> u64 {
    let (blockno, offno) = item_pointer_get_both(ctid);
    let blockno = blockno as u64;
    let offno = offno as u64;

    (blockno << 32) | offno
}

/// Deconstruct a `u64` into an otherwise uninitialized `ItemPointerData` struct
#[inline]
pub fn u64_to_item_pointer(value: u64, tid: &mut pg_sys::ItemPointerData) {
    let (blockno, offno) = u64_to_item_pointer_parts(value);
    *tid = item_pointer(blockno, offno);
}

#[inline]
pub fn u64_to_item_pointer_parts(value: u64) -> (pg_sys::BlockNumber, pg_sys::OffsetNumber) {
    let blockno = (value >> 32) as pg_sys::BlockNumber;
    let offno = value as pg_sys::OffsetNumber;
    (blockno, offno)
}

/// Build an item pointer with the stores PostgreSQL's `ItemPointerSet` performs.
///
/// PG15 defines `ItemPointerSet` and `BlockIdSet` as macros and PG16-19 as static
/// inline functions, with the same stores in every version:
/// `bi_hi = blockNumber >> 16`, `bi_lo = blockNumber & 0xffff`, and `ip_posid = offNum`.
/// Whether the generated `ItemPointerSet!` translates that body or calls the native
/// function depends on the PostgreSQL version and build and on libclang. The native call
/// would make these safe helpers call into PostgreSQL and panic off the backend thread,
/// so they build the fields in Rust.
#[inline]
fn item_pointer(
    blockno: pg_sys::BlockNumber,
    offno: pg_sys::OffsetNumber,
) -> pg_sys::ItemPointerData {
    pg_sys::ItemPointerData {
        ip_blkid: pg_sys::BlockIdData { bi_hi: (blockno >> 16) as u16, bi_lo: blockno as u16 },
        ip_posid: offno,
    }
}

/// Allocate an initialized item pointer in the current PostgreSQL memory context.
#[inline]
pub fn new_item_pointer(
    blockno: pg_sys::BlockNumber,
    offno: pg_sys::OffsetNumber,
) -> PgBox<pg_sys::ItemPointerData, AllocatedByRust> {
    // SAFETY: PgBox::alloc returns live storage aligned for ItemPointerData, and the
    // raw write initializes all of it without reading the uninitialized bytes.
    unsafe {
        let tid = PgBox::<pg_sys::ItemPointerData>::alloc();
        tid.as_ptr().write(item_pointer(blockno, offno));
        tid
    }
}

impl FromDatum for pg_sys::ItemPointerData {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<pg_sys::ItemPointerData> {
        if is_null {
            None
        } else {
            // SAFETY: FromDatum's caller provides a datum pointing to an initialized
            // ItemPointerData, which PostgreSQL aligns as `tid` (typalign 's'). The copy
            // is the struct assignment that PostgreSQL's ItemPointerCopy performs.
            Some(unsafe { datum.cast_mut_ptr::<pg_sys::ItemPointerData>().read() })
        }
    }
}

impl IntoDatum for pg_sys::ItemPointerData {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let tid_ptr = unsafe {
            // SAFETY:  CurrentMemoryContext is always valid
            PgMemoryContexts::CurrentMemoryContext.palloc_struct::<pg_sys::ItemPointerData>()
        };
        // SAFETY: palloc_struct returns live storage aligned for ItemPointerData; writing the
        // initialized value is the struct assignment that PostgreSQL's ItemPointerCopy performs.
        unsafe { tid_ptr.write(self) };

        Some(tid_ptr.into())
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::TIDOID
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The helpers must not call into PostgreSQL, which this test binary cannot do: a
    /// PostgreSQL symbol left in it makes the binary fail to load or link before any test
    /// runs. That is how a native `ItemPointerSet!` call would show up here. Where the
    /// generated macro translates the body, as it can with non-assert headers, this test
    /// cannot tell the helpers from that macro.
    #[test]
    fn u64_item_pointer_round_trip_needs_no_backend_thread() {
        let convert = |value: u64| {
            let mut tid = pg_sys::ItemPointerData::default();
            u64_to_item_pointer(value, &mut tid);
            (item_pointer_get_both(tid), item_pointer_to_u64(tid))
        };
        let value = (0x1234_5678_u64 << 32) | 0x9abc;
        assert_eq!(convert(value), ((0x1234_5678, 0x9abc), value));
        let other = std::thread::spawn(move || convert(value)).join().unwrap();
        assert_eq!(other, ((0x1234_5678, 0x9abc), value));
    }
}
