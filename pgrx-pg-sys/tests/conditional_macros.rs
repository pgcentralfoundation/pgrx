//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Test generated conditional macros through real PostgreSQL binding types.
//!
//! The suite exercises branch selection and effects using the selected build's
//! exports. These checks cover the public integration layer above the isolated
//! C/Rust control-flow oracles.

#![cfg(all(pgrx_c_macros, not(docsrs)))]
#![deny(unsafe_op_in_unsafe_fn)]

/// Exercise the selected build's public bindings and macro exports from a downstream consumer.
use pgrx_pg_sys as pg;

/// Checks that generated transaction advance matches original C wraparound cases.
#[test]
fn generated_transaction_advance_matches_original_c_wraparound_cases() {
    // C results from TransactionIdAdvance in access/transam.h, including every
    // special xid and the unsigned wraparound boundary.
    let cases = [
        (0_u32, 3_u32),
        (1, 3),
        (2, 3),
        (3, 4),
        (u32::MAX - 1, u32::MAX),
        (u32::MAX, 3),
        (0x80000000, 0x80000001),
    ];
    for (input, expected) in cases {
        let mut xid = pg::TransactionId::from_inner(input);
        // SAFETY: xid is an initialized scalar in an owned aligned exclusively
        // writable slot. The macro performs only unsigned integer operations
        // and never calls into a backend or accesses shared transaction state.
        unsafe { pg::TransactionIdAdvance!(xid) }
        assert_eq!(xid.into_inner(), expected, "xid={input:#010X}");
    }
}

/// Checks that generated jsonb offset selects absolute or relative and evaluates entry once.
#[test]
fn generated_jsonb_offset_selects_absolute_or_relative_and_evaluates_entry_once() {
    // C results from JBE_ADVANCE_OFFSET in utils/jsonb.h. Both branches mask
    // the type bits; only the HAS_OFF bit chooses assignment over addition.
    let cases = [
        (0_u32, 0_u32, 0_u32),
        (17, 0, 17),
        (17, 7, 24),
        (u32::MAX, 1, 0),
        (17, 0x80000007, 7),
        (u32::MAX, u32::MAX, 0x0FFFFFFF),
        (12, 0x70000005, 17),
        (0xFFFFFFF0, 0x20, 0x10),
    ];
    for (initial, entry, expected) in cases {
        let mut offset = initial;
        let calls = core::cell::Cell::new(0_u32);
        // SAFETY: offset is an initialized, aligned exclusively writable local.
        // The scalar argument initializes the generated JEntry local, whose
        // address stays within its invocation. Neither branch calls a backend.
        unsafe {
            pg::JBE_ADVANCE_OFFSET!(offset, {
                calls.set(calls.get() + 1);
                entry
            });
        }
        assert_eq!(offset, expected, "offset={initial:#010X}, entry={entry:#010X}");
        assert_eq!(calls.get(), 1, "the C declaration evaluates its initializer once");
    }
}

// Assertion-enabled C profiles skip this macro's stringification dependency.
// Test its generated API only when that API exists in the inspected profile.
pg::__pgrx_c_classify! { @if_available PageSetPrunable {
mod prunable {
/// Bring the actual consumer capability into scope for this fixture's generated-macro
/// integration checks.
use super::pg;
/// Provide owned raw record storage so tests can initialize only fields that a C macro actually
/// accesses.
use core::mem::MaybeUninit;
/// Address selected C fields without creating references to an incompletely initialized record.
use core::ptr::{addr_of, addr_of_mut};

/// Checks that generated prunable page short circuits before the backend comparison.
#[test]
fn generated_prunable_page_short_circuits_before_the_backend_comparison() {
    for input in [3_u32, u32::MAX] {
        let mut storage = MaybeUninit::<pg::PageHeaderData>::uninit();
        let header = addr_of_mut!(storage).cast::<pg::PageHeaderData>();
        let page = header as pg::Page;
        let xid = pg::TransactionId::from_inner(input);
        // SAFETY: header points into owned aligned live PageHeaderData storage
        // with exclusive access. Only pd_prune_xid is initialized and accessed;
        // no reference or value of the incomplete whole page is formed. Its
        // invalid initial xid makes the OR's first operand true, so the guarded
        // TransactionIdPrecedes backend function is never called. The input xid
        // is normal, so assertion-enabled builds need no failure callback either.
        unsafe {
            addr_of_mut!((*header).pd_prune_xid).write(pg::TransactionId::INVALID);
            pg::PageSetPrunable!(page, xid);
            assert_eq!(addr_of!((*header).pd_prune_xid).read().into_inner(), input);
        }
    }
}

// Keep the general form typechecked without executing its guarded backend call.
/// # Safety
/// page points to live aligned page storage with initialized pd_prune_xid and
/// exclusive write access. xid is normal. The caller runs in the permitted
/// PostgreSQL backend thread and obeys the page's locking and pinning contract.
#[allow(dead_code)]
unsafe fn set_prunable_in_backend(page: pg::Page, xid: pg::TransactionId) {
    // SAFETY: the caller establishes page access and backend-call preconditions.
    unsafe { pg::PageSetPrunable!(page, xid) }
}
}
} }
