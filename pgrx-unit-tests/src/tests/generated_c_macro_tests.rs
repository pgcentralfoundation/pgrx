//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise generated C macros in their PostgreSQL backend execution environment.
//!
//! A macro can reference guarded PostgreSQL functions even when a particular
//! invocation short-circuits before calling them. Its complete expansion still
//! needs the backend's symbols at link/load time, so these integration checks
//! belong in the extension test harness rather than a standalone test binary.
//! The availability classifier follows the selected build's actual emitted API;
//! Windows profiles currently do not generate C macro definitions.

#![cfg(not(target_os = "windows"))]
#![deny(unsafe_op_in_unsafe_fn)]

/// Register generated backend-dependent macro checks in the harness’s tests schema.
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use core::cell::Cell;
    use core::mem::MaybeUninit;
    use core::ptr::{addr_of, addr_of_mut, read, write};
    use pgrx::pg_sys as pg;
    use pgrx::prelude::*;

    // Assertion-enabled C profiles skip this macro's stringification dependency.
    // Test its generated API only when that API exists in the inspected profile.
    pg::__pgrx_c_classify! { @if_available PageSetPrunable {
    /// Check that the generated page macro short-circuits before a backend comparison.
    #[pg_test]
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

    /// Keep the general form typechecked without executing its guarded backend call.
    ///
    /// # Safety
    /// page points to live aligned page storage with initialized pd_prune_xid and
    /// exclusive write access. xid is normal. The caller runs in the permitted
    /// PostgreSQL backend thread and obeys the page's locking and pinning contract.
    #[allow(dead_code)]
    unsafe fn set_prunable_in_backend(page: pg::Page, xid: pg::TransactionId) {
        // SAFETY: the caller establishes page access and backend-call preconditions.
        unsafe { pg::PageSetPrunable!(page, xid) }
    }
    } }

    /// Obtain raw addresses for the version-specific statistics counters without reading or
    /// borrowing the containing record.
    ///
    /// # Safety
    /// `status` must point into a live, aligned `PgStat_TableStatus` allocation.
    /// Fields need not be initialized: this helper only obtains their raw addresses.
    unsafe fn counter_fields(status: *mut pg::PgStat_TableStatus) -> [*mut pg::PgStat_Counter; 5] {
        // SAFETY: the caller provides allocated aligned record storage. Raw field
        // addresses do not create references, load fields, or read the whole record.
        #[cfg(feature = "pg15")]
        unsafe {
            [
                addr_of_mut!((*status).t_counts.t_numscans),
                addr_of_mut!((*status).t_counts.t_tuples_returned),
                addr_of_mut!((*status).t_counts.t_tuples_fetched),
                addr_of_mut!((*status).t_counts.t_blocks_fetched),
                addr_of_mut!((*status).t_counts.t_blocks_hit),
            ]
        }
        // SAFETY: the same allocation contract applies to the renamed PG16+ fields.
        #[cfg(not(feature = "pg15"))]
        unsafe {
            [
                addr_of_mut!((*status).counts.numscans),
                addr_of_mut!((*status).counts.tuples_returned),
                addr_of_mut!((*status).counts.tuples_fetched),
                addr_of_mut!((*status).counts.blocks_fetched),
                addr_of_mut!((*status).counts.blocks_hit),
            ]
        }
    }

    /// Observe the enabled and disabled statistics paths using owned storage initialized only for
    /// the selected fields, keeping backend association calls unreachable.
    fn exercise_counter(mut update: impl FnMut(pg::Relation), expected: [pg::PgStat_Counter; 5]) {
        let mut status = MaybeUninit::<pg::PgStat_TableStatus>::uninit();
        let mut relation = MaybeUninit::<pg::RelationData>::uninit();
        let status = status.as_mut_ptr();
        let relation = relation.as_mut_ptr();
        // SAFETY: both records have live owned aligned storage. Only the five count
        // fields and pgstat_info are initialized or read in the enabled path. Its
        // non-null pgstat_info short-circuits the pgstat_enabled load and guarded
        // pgstat_assoc_relation call. In the disabled path, pgstat_info is null and
        // pgstat_enabled is initialized false, so no counts or backend functions are
        // accessed. No whole-record values or references are created, no raw pointer
        // escapes, and all accesses are exclusive to this test's allocations.
        unsafe {
            let fields = counter_fields(status);
            for field in fields {
                write(field, 0);
            }
            write(addr_of_mut!((*relation).pgstat_info), status);
            update(relation);
            let observed = fields.map(|field| read(field));
            assert_eq!(observed, expected);
            write(addr_of_mut!((*relation).pgstat_info), core::ptr::null_mut());
            write(addr_of_mut!((*relation).pgstat_enabled), false);
            update(relation);
            let disabled = fields.map(|field| read(field));
            assert_eq!(disabled, expected, "disabled statistics leave every counter unchanged");
        }
    }

    // These expected changes come from the installed pgstat.h definitions, which
    // increment one field or add n. The macros and layouts used here are generated
    // for this build's profile; no checked-in bindings or pgrx hand ports are used.
    pg::__pgrx_c_classify! { @if_available pgstat_count_heap_scan {
    /// Checks that generated heap scan counts only the initialized scan field.
    #[pg_test]
    fn generated_heap_scan_counts_only_the_initialized_scan_field() {
        exercise_counter(|relation| {
            // SAFETY: exercise_counter supplies exclusive initialized accessed fields
            // and makes the guarded PostgreSQL association branch unreachable.
            unsafe { pg::pgstat_count_heap_scan!(relation); }
        },[1,0,0,0,0]);
    }
    } }
    pg::__pgrx_c_classify! { @if_available pgstat_count_heap_getnext {
    /// Checks that generated heap getnext counts only the initialized returned field.
    #[pg_test]
    fn generated_heap_getnext_counts_only_the_initialized_returned_field() {
        exercise_counter(|relation| {
            // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
            unsafe { pg::pgstat_count_heap_getnext!(relation); }
        },[0,1,0,0,0]);
    }
    } }
    pg::__pgrx_c_classify! { @if_available pgstat_count_heap_fetch {
    /// Checks that generated heap fetch counts only the initialized fetched field.
    #[pg_test]
    fn generated_heap_fetch_counts_only_the_initialized_fetched_field() {
        exercise_counter(|relation| {
            // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
            unsafe { pg::pgstat_count_heap_fetch!(relation); }
        },[0,0,1,0,0]);
    }
    } }
    pg::__pgrx_c_classify! { @if_available pgstat_count_index_scan {
    /// Checks that generated index scan counts only the initialized scan field.
    #[pg_test]
    fn generated_index_scan_counts_only_the_initialized_scan_field() {
        exercise_counter(|relation| {
            // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
            unsafe { pg::pgstat_count_index_scan!(relation); }
        },[1,0,0,0,0]);
    }
    } }
    pg::__pgrx_c_classify! { @if_available pgstat_count_index_tuples {
    /// Checks that generated index tuples evaluates the increment only in the enabled branch.
    #[pg_test]
    fn generated_index_tuples_evaluates_the_increment_only_in_the_enabled_branch() {
        let calls=Cell::new(0_u32);
        exercise_counter(|relation| {
            // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
            unsafe { pg::pgstat_count_index_tuples!(relation,{calls.set(calls.get()+1);-7_i32}); }
        },[0,-7,0,0,0]);
        assert_eq!(calls.get(),1,"the disabled branch must suppress the increment operand");
    }
    } }
    pg::__pgrx_c_classify! { @if_available pgstat_count_buffer_read {
    /// Checks that generated buffer read counts only the initialized read field.
    #[pg_test]
    fn generated_buffer_read_counts_only_the_initialized_read_field() {
        exercise_counter(|relation| {
            // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
            unsafe { pg::pgstat_count_buffer_read!(relation); }
        },[0,0,0,1,0]);
    }
    } }
    pg::__pgrx_c_classify! { @if_available pgstat_count_buffer_hit {
    /// Checks that generated buffer hit counts only the initialized hit field.
    #[pg_test]
    fn generated_buffer_hit_counts_only_the_initialized_hit_field() {
        exercise_counter(|relation| {
            // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
            unsafe { pg::pgstat_count_buffer_hit!(relation); }
        },[0,0,0,0,1]);
    }
    } }

    pg::__pgrx_c_classify! { @if_available INTERRUPTS_PENDING_CONDITION {
    #[allow(dead_code)] // Compile the real backend-global expansion; never execute it here.
    /// Keep the generated interrupt predicate typechecked where its backend globals exist.
    ///
    /// # Safety
    /// Requires PostgreSQL's initialized backend thread and signal/interrupt state.
    unsafe fn generated_interrupt_condition_requires_backend() -> bool {
        // SAFETY: the caller establishes the backend and signal-global access contract.
        unsafe {
            pg::__pgrx_c_macros::expression::truth(
                pg::INTERRUPTS_PENDING_CONDITION!().into_value()
            )
        }
    }
    } }
    pg::__pgrx_c_classify! { @if_available CHECK_FOR_INTERRUPTS {
    #[allow(dead_code)] // Compile the guarded backend call without executing it.
    /// Keep the generated interrupt call typechecked under its production guard contract.
    ///
    /// # Safety
    /// Requires the permitted backend thread with PostgreSQL's interrupt, error,
    /// memory-context, and native-call guard state initialized.
    unsafe fn generated_interrupt_check_requires_backend() {
        // SAFETY: the caller establishes global access and guarded native-call state.
        unsafe { pg::CHECK_FOR_INTERRUPTS!(); }
    }
    } }
}
