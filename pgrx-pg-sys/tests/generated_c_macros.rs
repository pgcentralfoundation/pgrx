//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![cfg(any(pgrx_c_macros, docsrs))]

// Exercise the public exports from a downstream crate, including a renamed import.
use pg::__pgrx_c_macros::{CInt, CUnsignedInt, CUnsignedLong, CValue};
use pgrx_pg_sys as pg;

fn c_int(value: CValue<CInt>) -> i32 {
    value.get()
}

#[test]
fn generated_handports_preserve_c_results_through_the_public_bindings() {
    let size = CValue::<CUnsignedLong>::new(u64::MAX);
    assert_eq!(pg::TYPEALIGN!(8_i32, size).get(), 0_u64);
    assert_eq!(pg::MAXALIGN!(size).get(), 0_u64);
    assert_eq!(c_int(pg::BufferIsLocal!(-1)), 1);
    assert_eq!(c_int(pg::BufferIsLocal!(0)), 0);
    assert_eq!(c_int(pg::BufferIsLocal!(i32::MIN)), 1);
    for (xid, normal) in [(0, 0), (1, 0), (2, 0), (3, 1), (u32::MAX, 1)] {
        assert_eq!(c_int(pg::TransactionIdIsNormal!(pg::TransactionId::from_inner(xid))), normal);
    }
}

#[test]
fn checked_newtype_inputs_keep_their_c_integer_identity() {
    let oid = pg::Oid::from_u32(u32::MAX);
    let value: CValue<CUnsignedInt> = pg::Max!(oid, 0_u32);
    assert_eq!(value.get(), u32::MAX);
    let xid = pg::TransactionId::from_inner(3);
    let value: CValue<CUnsignedInt> = pg::Min!(xid, 42_u32);
    assert_eq!(value.get(), 3_u32);
}

#[test]
fn expansion_hygiene_preserves_repeated_and_lazy_arguments() {
    let mut calls = 0;
    let value = pg::Max!(
        {
            calls += 1;
            7
        },
        2
    );
    assert_eq!(value.get(), 7);
    assert_eq!(calls, 2);
}

#[cfg(not(docsrs))]
#[test]
fn binding_references_and_macro_calls_use_the_selected_build() {
    // These macros use the selected build's ALIGNOF_BUFFER binding and the
    // independently generated TYPEALIGN macros.
    assert_eq!(pg::BUFFERALIGN!(33_i32).get(), 64);
    assert_eq!(pg::BUFFERALIGN_DOWN!(63_i32).get(), 32);
    assert_eq!(pg::BUFFERALIGN!(CValue::<CUnsignedLong>::new(u64::MAX)).get(), 0);
}
