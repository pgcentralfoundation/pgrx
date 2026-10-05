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

use crate::PgMemoryContexts;
use crate::datum::{FromDatum, IntoDatum};
use crate::{AllocatedByRust, PgBox, pg_sys};

/// Decode a Rust-owned item pointer into its block and offset numbers, without validity assertions.
#[inline]
pub fn item_pointer_get_both(
    ctid: pg_sys::ItemPointerData,
) -> (pg_sys::BlockNumber, pg_sys::OffsetNumber) {
    // SAFETY: ctid is initialized native storage and stays live for both non-mutating reads.
    // On PG16+ these are guarded native calls: they must run on the backend thread, and
    // the guard panics on any other thread.
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
    let blockno = (value >> 32) as pg_sys::BlockNumber;
    let offno = value as pg_sys::OffsetNumber;
    // SAFETY: the exclusive reference provides aligned writable ItemPointerData storage.
    unsafe {
        pg_sys::ItemPointerSet!(&raw mut *tid, blockno, offno).get();
    }
}

#[inline]
pub fn u64_to_item_pointer_parts(value: u64) -> (pg_sys::BlockNumber, pg_sys::OffsetNumber) {
    let blockno = (value >> 32) as pg_sys::BlockNumber;
    let offno = value as pg_sys::OffsetNumber;
    (blockno, offno)
}

/// Allocate an initialized item pointer in the current PostgreSQL memory context.
#[inline]
pub fn new_item_pointer(
    blockno: pg_sys::BlockNumber,
    offno: pg_sys::OffsetNumber,
) -> PgBox<pg_sys::ItemPointerData, AllocatedByRust> {
    // SAFETY: PgBox allocates aligned storage for ItemPointerData; ItemPointerSet initializes all fields.
    unsafe {
        let tid = PgBox::<pg_sys::ItemPointerData>::alloc();
        pg_sys::ItemPointerSet!(tid.as_ptr(), blockno, offno).get();
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
            // SAFETY: FromDatum's caller provides an initialized ItemPointerData datum.
            unsafe {
                let tid = datum.cast_mut_ptr::<pg_sys::ItemPointerData>();
                let mut tid_copy = pg_sys::ItemPointerData::default();
                pg_sys::ItemPointerCopy!(tid, &raw mut tid_copy).get();
                Some(tid_copy)
            }
        }
    }
}

impl IntoDatum for pg_sys::ItemPointerData {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let tid = self;
        let tid_ptr = unsafe {
            // SAFETY:  CurrentMemoryContext is always valid
            PgMemoryContexts::CurrentMemoryContext.palloc_struct::<pg_sys::ItemPointerData>()
        };
        // SAFETY: tid is initialized; tid_ptr has aligned writable PostgreSQL allocation bounds.
        unsafe {
            pg_sys::ItemPointerCopy!(&raw const tid, tid_ptr).get();
        }

        Some(tid_ptr.into())
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::TIDOID
    }
}
