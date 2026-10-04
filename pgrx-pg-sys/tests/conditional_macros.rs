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
