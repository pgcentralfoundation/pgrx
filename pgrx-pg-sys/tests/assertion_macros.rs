//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check that PostgreSQL's assertion macros are the same macros in every build flavor.
//!
//! An assert-enabled PostgreSQL checks these conditions and a release build does not. Either
//! way each macro exists, takes the same operands, yields the same type and needs `unsafe`
//! for the same calls. This file compiles against both, and `unused_unsafe` rejects an
//! `unsafe` block that one of them would not need.
//!
//! Macros that call PostgreSQL in an assert-enabled build are used in generic functions that
//! nothing instantiates. Those are type-checked and linted like any other, and compile to no
//! code that would need a backend's symbols. `pgrx-unit-tests` runs the macros in a backend,
//! and the fixtures of `pgrx-c-macros` check that a failed assertion stops the process.

#![cfg(not(docsrs))]
#![deny(unsafe_op_in_unsafe_fn, unused_unsafe)]

use pgrx_pg_sys as pg;

/// `Assert` is a statement in C, so the macro yields `()` and can end a function.
fn assert_statement(value: i32) {
    pg::Assert!(value)
}

/// Assertions whose conditions hold return in either build, from safe code.
#[test]
fn assertions_that_hold_need_no_unsafe() {
    assert_statement(1);
    let () = pg::Assert!(1_i32);
    let aligned = 0_u32;
    let () = pg::AssertPointerAlignment!(core::ptr::from_ref(&aligned), 4_i32);
    let () = pg::AssertMacro!(1_i32).get();
}

/// Checks whose conditions are pure run in an assert-enabled build without needing `unsafe`.
#[test]
fn pure_checks_need_no_unsafe() {
    let (older, newer) = (pg::TransactionId::from(3_u32), pg::TransactionId::from(4_u32));
    assert!(pg::NormalTransactionIdPrecedes!(older, newer).is_true());
    assert_eq!(pg::AttrNumberGetAttrOffset!(1_i16).get(), 0);
}

/// Assertion helpers that an assert-enabled build declares as functions are macros in both
/// builds. Those that take no pointer need no `unsafe`.
#[allow(dead_code)]
fn assertion_functions_are_safe_macros_in_every_build<Uninstantiated>() {
    let xid = pg::TransactionId::from(3_u32);
    let () = pg::AssertTransactionIdInAllowableRange!(xid).get();
    let () = pg::AssertPendingSyncs_RelationCache!().get();
    let () = pg::AssertCouldGetRelation!().get();
}

/// A check that reads through a pointer needs `unsafe` in both builds, although only an
/// assert-enabled PostgreSQL evaluates it.
///
/// # Safety
/// `page` must point to an initialized page and `control` to an initialized SLRU control
/// structure, as the assert-enabled checks require.
#[allow(dead_code)]
unsafe fn checks_through_pointers_need_unsafe_in_every_build<Uninstantiated>(
    page: pg::Page,
    #[cfg(not(feature = "pg19"))] control: pg::SlruCtl,
    // PostgreSQL 19 names the control structure `SlruDesc`.
    #[cfg(feature = "pg19")] control: *mut pg::SlruDesc,
) {
    use pg::cmacros::c::Pointer;
    // SAFETY: the caller provides the page and control structure these checks read.
    unsafe {
        // PostgreSQL 15's definition yields `bool` and later versions' yields nothing; the
        // type is the same for both build flavors of one version.
        let _ = pg::PageValidateSpecialPointer!(Pointer::from(page)).get();
        let () = pg::SlruPagePrecedesUnitTests!(control, 1_i32).get();
    }
}

/// `pg_unreachable` exists in both builds; reaching it would abort the process.
#[test]
fn unreachable_code_needs_no_unsafe() {
    if core::hint::black_box(false) {
        let () = pg::pg_unreachable!().get();
    }
}
