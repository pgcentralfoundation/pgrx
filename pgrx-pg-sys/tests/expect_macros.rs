//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise generated prediction-hint macros through pgrx-pg-sys exports.
//!
//! Runtime checks ensure C result identity and operand effects survive the public
//! entry point. The standalone compiler witnesses separately establish branch
//! weights; these integration tests check usable exported behavior.

#![cfg(all(pgrx_c_macros, not(docsrs)))]
#![deny(unsafe_op_in_unsafe_fn)]

/// Record callback or operand observations without changing the generated C expression types.
use core::cell::Cell;
/// Provide owned raw record storage so tests can initialize only fields that a C macro actually
/// accesses.
use core::mem::MaybeUninit;
/// Address selected C fields without creating references to an incompletely initialized record.
use core::ptr::{addr_of_mut, read, write};
/// Exercise the selected build's public bindings and macro exports from a downstream consumer.
use pgrx_pg_sys as pg;

/// Expose the semantic C rank for assertions that branch-hint truth results remain C long.
fn rank<K: pg::__pgrx_c_macros::CInteger>(_: pg::__pgrx_c_macros::CValue<K>) -> u8 {
    K::RANK
}

pg::__pgrx_c_classify! { @if_available likely {
pg::__pgrx_c_classify! { @if_available unlikely {
/// Checks that generated expectation wrappers preserve C long truth and single evaluation.
#[test]
fn generated_expectation_wrappers_preserve_c_long_truth_and_single_evaluation() {
    // The installed c.h passes (x != 0) to __builtin_expect, whose compiler
    // prototype returns long. The independent C oracle verifies that rank and
    // width even though the payload has only the values zero and one.
    for value in [-2147483647_i32,-1,0,1,17,i32::MAX] {
        let calls=Cell::new(0_u32);
        let likely=pg::likely!({calls.set(calls.get()+1);value});
        let unlikely=pg::unlikely!({calls.set(calls.get()+1);value});
        assert_eq!(likely.get(),i64::from(value!=0));
        assert_eq!(unlikely.get(),i64::from(value!=0));
        assert_eq!(rank(likely.into_value()),4);
        assert_eq!(rank(unlikely.into_value()),4);
        assert_eq!(core::mem::size_of_val(&likely.get()),core::mem::size_of::<core::ffi::c_long>());
        assert_eq!(calls.get(),2);
    }
    for value in [false,true] {
        assert_eq!(pg::likely!(value).get(),i64::from(value));
        assert_eq!(pg::unlikely!(value).get(),i64::from(value));
    }
    for value in [-3.75_f64,-0.0,0.0,0.75,f64::NAN,f64::INFINITY] {
        assert_eq!(pg::likely!(value).get(),i64::from(value!=0.0));
        assert_eq!(pg::unlikely!(value).get(),i64::from(value!=0.0));
    }
    let mut object=31_i32;
    for pointer in [core::ptr::null_mut(),&raw mut object] {
        assert_eq!(pg::likely!(pointer).get(),i64::from(!pointer.is_null()));
        assert_eq!(pg::unlikely!(pointer).get(),i64::from(!pointer.is_null()));
    }
}
} }
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
#[test]
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
#[test]
fn generated_heap_getnext_counts_only_the_initialized_returned_field() {
    exercise_counter(|relation| {
        // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
        unsafe { pg::pgstat_count_heap_getnext!(relation); }
    },[0,1,0,0,0]);
}
} }
pg::__pgrx_c_classify! { @if_available pgstat_count_heap_fetch {
/// Checks that generated heap fetch counts only the initialized fetched field.
#[test]
fn generated_heap_fetch_counts_only_the_initialized_fetched_field() {
    exercise_counter(|relation| {
        // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
        unsafe { pg::pgstat_count_heap_fetch!(relation); }
    },[0,0,1,0,0]);
}
} }
pg::__pgrx_c_classify! { @if_available pgstat_count_index_scan {
/// Checks that generated index scan counts only the initialized scan field.
#[test]
fn generated_index_scan_counts_only_the_initialized_scan_field() {
    exercise_counter(|relation| {
        // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
        unsafe { pg::pgstat_count_index_scan!(relation); }
    },[1,0,0,0,0]);
}
} }
pg::__pgrx_c_classify! { @if_available pgstat_count_index_tuples {
/// Checks that generated index tuples evaluates the increment only in the enabled branch.
#[test]
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
#[test]
fn generated_buffer_read_counts_only_the_initialized_read_field() {
    exercise_counter(|relation| {
        // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
        unsafe { pg::pgstat_count_buffer_read!(relation); }
    },[0,0,0,1,0]);
}
} }
pg::__pgrx_c_classify! { @if_available pgstat_count_buffer_hit {
/// Checks that generated buffer hit counts only the initialized hit field.
#[test]
fn generated_buffer_hit_counts_only_the_initialized_hit_field() {
    exercise_counter(|relation| {
        // SAFETY: exercise_counter establishes owned field and lazy-call contracts.
        unsafe { pg::pgstat_count_buffer_hit!(relation); }
    },[0,0,0,0,1]);
}
} }

pg::__pgrx_c_classify! { @if_available INTERRUPTS_PENDING_CONDITION {
#[allow(dead_code)] // Compile the real backend-global expansion; never execute it here.
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
/// # Safety
/// Requires the permitted backend thread with PostgreSQL's interrupt, error,
/// memory-context, and native-call guard state initialized.
unsafe fn generated_interrupt_check_requires_backend() {
    // SAFETY: the caller establishes global access and guarded native-call state.
    unsafe { pg::CHECK_FOR_INTERRUPTS!(); }
}
} }
