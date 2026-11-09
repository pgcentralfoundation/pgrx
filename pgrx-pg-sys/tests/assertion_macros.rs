//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check that PostgreSQL's assertion macros have the same form and safety in every build.
//!
//! An assert-enabled PostgreSQL checks these conditions and a release build does not. Either
//! way each macro exists, takes the same operands, and needs `unsafe` only where its release
//! form does, so every call here is safe code, which `unused_unsafe` would also flag.

#![cfg(not(docsrs))]
#![deny(unsafe_op_in_unsafe_fn, unused_unsafe)]

use pgrx_pg_sys as pg;

/// Assertions whose conditions hold return in either build.
#[test]
fn assertions_that_hold_need_no_unsafe() {
    let _ = pg::Assert!(1_i32);
    let _ = pg::AssertMacro!(1_i32);
    let aligned = 0_u32;
    let _ = pg::AssertPointerAlignment!(core::ptr::from_ref(&aligned), 4_i32);
    let xid = pg::TransactionId::from(3_u32);
    let _ = pg::AssertTransactionIdInAllowableRange!(xid);
}

/// Assertion helpers that an assert-enabled build declares as functions are macros in
/// both builds.
#[test]
fn assertion_helpers_are_macros_in_every_build() {
    let _ = pg::AssertPendingSyncs_RelationCache!();
    let _ = pg::AssertCouldGetRelation!();
    let _ = pg::SlruPagePrecedesUnitTests!(core::ptr::null::<u8>(), 1_i32);
}

/// A check that reads a global keeps no `unsafe` requirement when nothing else needs one.
#[test]
fn checks_that_read_globals_do_not_need_unsafe() {
    assert!(!pg::BufferIsValid!(0_i32).is_true());
}

/// Checks whose conditions are pure stay in an assert-enabled build without needing `unsafe`.
#[test]
fn pure_checks_need_no_unsafe() {
    let (older, newer) = (pg::TransactionId::from(3_u32), pg::TransactionId::from(4_u32));
    assert!(pg::NormalTransactionIdPrecedes!(older, newer).is_true());
    assert_eq!(pg::AttrNumberGetAttrOffset!(1_i16).get(), 0);
    // Its check calls BufferIsValid, whose own checks are omitted.
    assert!(pg::BufferGetPageSize!(1_i32).get() > 0);
}

/// `castNode` casts without its assert-enabled `castNodeImpl` check.
#[test]
fn cast_node_is_a_safe_pointer_cast() {
    let node = core::ptr::null_mut::<pg::Node>();
    let query: *mut pg::Query = pg::castNode!(pg::Query, node).get();
    assert!(query.is_null());
}

/// `pg_unreachable` exists in both builds; reaching it would abort the process.
#[test]
fn unreachable_code_needs_no_unsafe() {
    if core::hint::black_box(false) {
        let _ = pg::pg_unreachable!();
    }
}
