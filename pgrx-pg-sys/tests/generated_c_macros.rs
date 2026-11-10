//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise generated C macros as a downstream pgrx-pg-sys consumer.
//!
//! Renamed imports, checked integer wrappers, repeated operands, symbolic binding
//! references, and raw fields verify the public export path. All version-specific
//! facts come from this build. Docs builds only type-check these tests, against the
//! documentation snapshot's typed shells.

// Exercise the public exports from a downstream crate, including a renamed import.
use pg::__pgrx_c_macros::{CInt, CUnsignedInt, CValue};
use pgrx_pg_sys as pg;

/// Extract the C int storage of a predicate used as an operand of another generated macro.
fn c_int(value: CValue<CInt>) -> i32 {
    value.get()
}

/// Checks C wrapping alignment and integer predicates through downstream public macro exports.
/// A predicate is a comparison, so its macro returns a Rust `bool`. As an operand of another
/// generated macro it is still the C `int` zero or one.
#[test]
fn generated_handports_preserve_c_results_through_the_public_bindings() {
    let size = CValue::<pg::__pgrx_c_types::uintptr_t>::new(usize::MAX as _);
    assert_eq!(pg::TYPEALIGN!(8_i32, size).get(), 0);
    assert_eq!(pg::MAXALIGN!(size).get(), 0);
    let local: [bool; 3] =
        [pg::BufferIsLocal!(-1), pg::BufferIsLocal!(0), pg::BufferIsLocal!(i32::MIN)];
    assert_eq!(local, [true, false, true]);
    assert_eq!(c_int(pg::Max!(pg::BufferIsLocal!(-1), 0_i32).into_value()), 1);
    assert_eq!(c_int(pg::Max!(pg::BufferIsLocal!(0), -5_i32).into_value()), 0);
    for (xid, normal) in [(0, false), (1, false), (2, false), (3, true), (u32::MAX, true)] {
        assert_eq!(pg::TransactionIdIsNormal!(pg::TransactionId::from_inner(xid)), normal);
    }
}

/// Checks Oid and TransactionId inputs retain their verified unsigned C rank in min/max
/// expressions.
#[test]
fn checked_newtype_inputs_keep_their_c_integer_identity() {
    let oid = pg::Oid::from_u32(u32::MAX);
    let value: CValue<CUnsignedInt> = pg::Max!(oid, 0_u32).into_value();
    assert_eq!(value.get(), u32::MAX);
    let xid = pg::TransactionId::from_inner(3);
    let value: CValue<CUnsignedInt> = pg::Min!(xid, 42_u32).into_value();
    assert_eq!(value.get(), 3_u32);
}

/// Checks that constants PostgreSQL casts to `Oid`, `RelFileNumber`, `TransactionId` or
/// `MultiXactId` keep pgrx's wrapper types, which generated macros accept as those C types.
#[test]
fn wrapper_typed_constants_keep_their_types() {
    let invalid: pg::Oid = pg::InvalidOid;
    assert_eq!(invalid, pg::Oid::INVALID);
    assert!(!pg::OidIsValid!(pg::InvalidOid));
    assert!(pg::OidIsValid!(pg::BOOLOID));
    #[cfg(not(feature = "pg15"))]
    {
        let invalid: pg::RelFileNumber = pg::InvalidRelFileNumber;
        assert_eq!(invalid, pg::Oid::INVALID);
    }
    let xids: [pg::TransactionId; 5] = [
        pg::InvalidTransactionId,
        pg::BootstrapTransactionId,
        pg::FrozenTransactionId,
        pg::FirstNormalTransactionId,
        pg::MaxTransactionId,
    ];
    assert_eq!(xids.map(pg::TransactionId::into_inner), [0, 1, 2, 3, u32::MAX]);
    assert!(!pg::TransactionIdIsNormal!(pg::FrozenTransactionId));
    assert!(pg::TransactionIdIsNormal!(pg::FirstNormalTransactionId));
    let multis: [pg::MultiXactId; 3] =
        [pg::InvalidMultiXactId, pg::FirstMultiXactId, pg::MaxMultiXactId];
    assert_eq!(multis.map(pg::MultiXactId::into_inner), [0, 1, u32::MAX]);
}

/// Checks that public macro expansion repeats the selected Max operand exactly as the C
/// definition does.
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

/// Checks symbolic ALIGNOF_BUFFER and nested TYPEALIGN calls against the current generated
/// bindings, including unsigned wraparound.
#[test]
fn binding_references_and_macro_calls_use_the_selected_build() {
    // These macros use the selected build's ALIGNOF_BUFFER binding and the
    // independently generated TYPEALIGN macros.
    assert_eq!(pg::BUFFERALIGN!(33_i32).get(), 64);
    assert_eq!(pg::BUFFERALIGN_DOWN!(63_i32).get(), 32);
    assert_eq!(
        pg::BUFFERALIGN!(CValue::<pg::__pgrx_c_types::uintptr_t>::new(usize::MAX as _)).get(),
        0
    );
}

/// Checks that alignment results, whose C type is `uintptr_t`, extract as `usize`.
#[test]
fn size_results_extract_as_usize() {
    assert_eq!(pg::BUFFERALIGN!(33_usize).get_usize(), 64);
    assert_eq!(usize::from(pg::MAXALIGN!(usize::MAX)), 0);
}

/// Checks raw enum bytes and union fields without materializing invalid Rust record or enum
/// values.
#[test]
fn generated_field_access_preserves_raw_c_storage() {
    let mut node = core::mem::MaybeUninit::<pg::Node>::uninit();
    let node = node.as_mut_ptr();
    let mut cell = core::mem::MaybeUninit::<pg::ListCell>::uninit();
    let cell = cell.as_mut_ptr();
    // SAFETY: Both pointers refer to live, aligned allocations. Only selected
    // fields are accessed, with their C integer representations initialized
    // before reading. The NodeTag bytes deliberately have no Rust enum variant;
    // neither allocation is materialized as a Rust Node or ListCell value.
    unsafe {
        // The compatible C enum integer is signed on MSVC and unsigned on other
        // targets. Convert the expected bits to that exact inferred storage type.
        assert_eq!(pg::NodeSetTag!(node, u32::MAX).get(), u32::MAX as _);
        assert_eq!(pg::nodeTag!(node).get(), u32::MAX as _);
        assert_eq!(pg::NodeSetTag!(node, pg::NodeTag::T_Invalid).get(), 0);
        assert_eq!(pg::nodeTag!(node).get(), 0);
        core::ptr::addr_of_mut!((*cell).int_value).write(-17);
        assert_eq!(pg::lfirst_int!(cell).get(), -17);
    }
}
