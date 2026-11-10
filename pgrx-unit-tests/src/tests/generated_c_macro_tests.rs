//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise generated C macros in their PostgreSQL backend execution environment.
//!
//! A macro can reference guarded PostgreSQL functions even when a particular
//! invocation short-circuits before calling them. Its complete expansion still
//! needs the backend's symbols at link/load time, so these integration checks
//! belong in the extension test harness rather than a standalone test binary.
//! The availability classifier follows the selected build's actual emitted API.

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

    // Test this macro's generated API only when that API exists in the inspected profile.
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

    /// Checks that `BufferIsValid`, whose assert-enabled checks read `NBuffers` and
    /// `NLocBuffer`, needs no `unsafe` in either build, nor does `BufferGetPageSize`, whose
    /// check calls it.
    #[pg_test]
    fn generated_buffer_checks_read_globals_without_unsafe() {
        assert!(!pg::BufferIsValid!(0_i32).is_true());
        assert!(pg::BufferIsValid!(1_i32).is_true());
        assert_eq!(pg::BufferGetPageSize!(1_i32).get(), u64::from(pg::BLCKSZ));
    }

    /// `Assert` is a statement in C, so the macro yields `()` in either build.
    fn generated_assert_statement(value: i32) {
        pg::Assert!(value)
    }

    /// Checks that PostgreSQL's assertion functions are macros that safe code calls in either
    /// build. An assert-enabled PostgreSQL runs the functions, whose checks hold here; a
    /// release build has no such functions and evaluates nothing.
    #[pg_test]
    fn generated_assertion_functions_run_in_a_backend() {
        generated_assert_statement(1);
        // SAFETY: a `#[pg_test]` runs in a transaction on the backend thread, which is all
        // this function requires.
        let xid = unsafe { pg::GetTopTransactionId() };
        let () = pg::AssertTransactionIdInAllowableRange!(xid).get();
        let () = pg::AssertTransactionIdInAllowableRange!(pg::FrozenTransactionId).get();
        let () = pg::AssertCouldGetRelation!().get();
        let () = pg::AssertPendingSyncs_RelationCache!().get();
    }

    /// Checks that `intVal` reads an integer node in either build. An assert-enabled
    /// PostgreSQL first checks the node's tag in `castNodeImpl`, which only it defines; a
    /// release build compiles that function without the check.
    #[pg_test]
    fn generated_node_value_checks_its_tag() {
        // SAFETY: makeInteger returns a valid Integer node allocated in the current memory
        // context, which outlives this read of its value.
        let value = unsafe { pg::intVal!(pg::makeInteger(42)).get() };
        assert_eq!(value, 42);
    }

    /// Checks that `makeNode` allocates a zeroed node of its type operand with the tag C
    /// names for that type, which `nodeTag`, `IsA` and `castNode` then find.
    #[pg_test]
    fn generated_make_node_tags_the_node_its_type_names() {
        // SAFETY: a `#[pg_test]` runs on the backend thread with a current memory context,
        // in which `makeNode` allocates a zeroed `Query` and stores its tag. Every pointer
        // below addresses that node, which is freed once at the end.
        unsafe {
            let query: *mut pg::Query = pg::makeNode!(pg::Query).get();
            let node = query.cast::<pg::Node>();
            assert_eq!(pg::nodeTag!(node).get(), pg::NodeTag::T_Query as u32);
            assert!(pg::IsA!(node, pg::Query).is_true());
            assert!(!pg::IsA!(node, pg::List).is_true());
            let cast: *mut pg::Query = pg::castNode!(pg::Query, node).get();
            assert_eq!(cast, query);
            assert!(!(*query).canSetTag && (*query).rtable.is_null());
            pg::pfree(query.cast());
        }
    }

    /// Checks that the list macros cast a cell's node to their type operand. An
    /// assert-enabled PostgreSQL checks each node's tag in `castNodeImpl` first, and every
    /// tag here is the one its macro names, so both builds return the nodes.
    #[pg_test]
    fn generated_list_node_macros_cast_cells_to_their_type_operand() {
        // SAFETY: a `#[pg_test]` runs on the backend thread with a current memory context.
        // `makeInteger` and `makeString` return valid nodes there, the list holds exactly
        // those five pointers, and `list_free_deep` frees the nodes and the list once.
        unsafe {
            let integers = [1, 2, 3, 4].map(|value| pg::makeInteger(value));
            let string = pg::makeString(c"last".as_ptr().cast_mut());
            let mut list = core::ptr::null_mut();
            for integer in integers {
                list = pg::lappend(list, integer.cast());
            }
            list = pg::lappend(list, string.cast());
            let first: *mut pg::Integer = pg::linitial_node!(pg::Integer, list).get();
            assert_eq!(first, integers[0]);
            assert_eq!(pg::lsecond_node!(pg::Integer, list).get(), integers[1]);
            assert_eq!(pg::lthird_node!(pg::Integer, list).get(), integers[2]);
            assert_eq!(pg::lfourth_node!(pg::Integer, list).get(), integers[3]);
            assert_eq!(pg::list_nth_node!(pg::Integer, list, 2_i32).get(), integers[2]);
            let last: *mut pg::String = pg::llast_node!(pg::String, list).get();
            assert_eq!(last, string);
            let cell = pg::list_nth_cell(list, 1);
            assert_eq!(pg::lfirst_node!(pg::Integer, cell).get(), integers[1]);
            // A null pointer has no tag to check.
            let null = core::ptr::null_mut::<pg::Node>();
            assert!(pg::castNode!(pg::Integer, null).get().is_null());
            assert!(pg::IsA!(last, pg::String).is_true());
            assert!(!pg::IsA!(last, pg::Integer).is_true());
            pg::list_free_deep(list);
        }
    }

    pg::__pgrx_c_classify! { @if_available INTERRUPTS_PENDING_CONDITION {
    /// Checks that the interrupt predicate reads `InterruptPending` from safe code on the
    /// backend thread, and panics before reading it on any other thread.
    #[pg_test]
    fn generated_interrupt_condition_checks_the_thread() {
        let pending = || {
            pg::__pgrx_c_macros::expression::truth(pg::INTERRUPTS_PENDING_CONDITION!().into_value())
        };
        // The first checked call claims its thread as the one allowed to read PostgreSQL's
        // globals, so this one must come before the read on another thread.
        pending();
        let off_thread = std::thread::scope(|scope| scope.spawn(pending).join());
        assert!(off_thread.is_err(), "INTERRUPTS_PENDING_CONDITION read a global off the backend thread");
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
