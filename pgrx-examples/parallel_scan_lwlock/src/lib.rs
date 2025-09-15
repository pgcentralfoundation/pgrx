//! Supports both parallel and non-parallel scans, avoiding code duplication.
//!
//! Initialization:
//! * [_PG_init] for startup initialization
//! * [counter_handler] for startup registration of FDW handler callbacks
//!
//! Scan start:
//! * [pgrx_get_foreign_rel_size] for table-level estimates
//! * [pgrx_get_foreign_paths] provides the scan strategies available (just scan and parallel_scan)
//! * [pgrx_get_foreign_plan] processes clauses and constructs an executable plan
//!
//! Sequential scan execution:
//! * [pgrx_begin_foreign_scan] prepares the execution, initializing executable expressions
//! * [pgrx_iterate_foreign_scan] fetches the next row from the block, advancing to the next if necessary
//! * [pgrx_end_foreign_scan] cleans up on scan complete
//!
//! Parallel scan execution:
//! * [pgrx_begin_foreign_scan] prepares the execution, initializing executable expressions
//! * [pgrx_estimate_dsm_foreign_scan] calculates the amount of shared memory needed for the block list (prefiltered by temporal index)
//! * [pgrx_initialize_dsm_foreign_scan] creates the shared memory locks in the allocated DSM
//! * [pgrx_initialize_worker_foreign_scan] saves the pointer to the mapped shared memory in the worker process space
//! * See `Sequential scan execution` above
//!
//! Other support functions:
//! * [counter_validator] called for option validation
//!
//! # Extension setup example
//!
//! First add the extension to the `shared_preload_libraries` in `postgresql.conf`
//!
//! shared_preload_libraries = 'parallel_scan_lwlock.so'
//!
//! Then run the project with `cargo pgrx run` and execute:
//!
//! CREATE EXTENSION parallel_scan_lwlock;
//! CREATE FOREIGN DATA WRAPPER counter HANDLER counter_handler VALIDATOR counter_validator;
//! CREATE SERVER counter_srv FOREIGN DATA WRAPPER counter;
//! IMPORT FOREIGN SCHEMA my_counter FROM SERVER counter_srv INTO public;
//!
//! ## Query examples
//!
//! > The counter table never ends, so always use `LIMIT`.
//!
//! select * from my_counter limit 10;
//!
//! select count(distinct(c.counter)) as count from (select counter from my_counter limit 10000000) as c;

mod routine;

use crate::routine::*;
use pgrx::lwlock::scan::*;
use pgrx::pg_sys::*;
use pgrx::*;
use std::ffi::{CStr, c_void};
use std::time::Duration;

pg_module_magic!(name, version);

static SCAN_COUNTER_LOCKS: ParallelScanLwLockTranche =
    ParallelScanLwLockTranche::new(c"scan_counter_lock");

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    if unsafe { !pgrx::pg_sys::process_shared_preload_libraries_in_progress } {
        pgrx::error!("this extension must be loaded via shared_preload_libraries.");
    }
    pg_shmem_init!(SCAN_COUNTER_LOCKS);
}

// CREATE FOREIGN DATA WRAPPER counter_fdw HANDLER counter_handler VALIDATOR counter_validator;

/// ```pgrxsql
/// CREATE OR REPLACE FUNCTION "counter_handler"() RETURNS fdw_handler
/// STRICT LANGUAGE c /* Rust */
/// AS 'MODULE_PATHNAME', '@FUNCTION_NAME@';
/// ```
#[pg_extern]
fn counter_handler() -> CounterFdwRoutine {
    CounterFdwRoutine(FdwRoutine {
        GetForeignRelSize: Some(pgrx_get_foreign_rel_size),
        GetForeignPaths: Some(pgrx_get_foreign_paths),
        GetForeignPlan: Some(pgrx_get_foreign_plan),
        BeginForeignScan: Some(pgrx_begin_foreign_scan),
        IterateForeignScan: Some(pgrx_iterate_foreign_scan),
        EndForeignScan: Some(pgrx_end_foreign_scan),
        ReScanForeignScan: Some(pgrx_re_scan_foreign_scan),
        IsForeignScanParallelSafe: Some(pgrx_is_foreign_scan_parallel_safe),
        EstimateDSMForeignScan: Some(pgrx_estimate_dsm_foreign_scan),
        InitializeDSMForeignScan: Some(pgrx_initialize_dsm_foreign_scan),
        ReInitializeDSMForeignScan: Some(pgrx_reinitialize_dsm_foreign_scan),
        InitializeWorkerForeignScan: Some(pgrx_initialize_worker_foreign_scan),
        ShutdownForeignScan: Some(pgrx_shutdown_foreign_scan),
        ImportForeignSchema: Some(pgrx_import_foreign_schema),
        ..EMPTY_FDW
    })
}

/// ```pgrxsql
/// CREATE OR REPLACE FUNCTION "counter_validator"(text[], oid) RETURNS void
/// STRICT LANGUAGE c /* Rust */
/// AS 'MODULE_PATHNAME', '@FUNCTION_NAME@';
/// ```
#[pg_extern]
fn counter_validator(_fcinfo: pg_sys::FunctionCallInfo) {
    // No options
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_get_foreign_rel_size(
    _root: *mut PlannerInfo,
    _baserel: *mut RelOptInfo,
    _foreigntableid: Oid,
) {
    let expected_query_rows = 1_000_000f64; // Arbitrary
    unsafe {
        (*_baserel).tuples = expected_query_rows;
        (*_baserel).rows = expected_query_rows;
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_get_foreign_paths(
    root: *mut PlannerInfo,
    baserel: *mut RelOptInfo,
    _foreigntableid: Oid,
) {
    // Just some arbitrary cost estimates
    let startup_cost = 100f64;
    let total_cost = 1_000_000_000f64;

    unsafe {
        let sequential_path = create_foreignscan_path(
            root,
            baserel,
            std::ptr::null_mut(), // Use default target
            (*baserel).rows,
            #[cfg(not(any(
                feature = "pg13",
                feature = "pg14",
                feature = "pg15",
                feature = "pg16",
                feature = "pg17"
            )))]
            0, // disabled_nodes
            startup_cost,
            total_cost,
            std::ptr::null_mut(), // pathkeys
            std::ptr::null_mut(), // no outer rel
            std::ptr::null_mut(), // no extra plan
            #[cfg(not(any(
                feature = "pg13",
                feature = "pg14",
                feature = "pg15",
                feature = "pg16"
            )))]
            std::ptr::null_mut(), // fdw_restrictinfo
            std::ptr::null_mut(), // no fdw_private data while planning
        );
        add_path(baserel, sequential_path as *mut Path);

        let work_factor = max_parallel_workers_per_gather as f64;
        let parallel_path = create_foreignscan_path(
            root,
            baserel,
            std::ptr::null_mut(), // Use default target
            (*baserel).rows,
            #[cfg(not(any(
                feature = "pg13",
                feature = "pg14",
                feature = "pg15",
                feature = "pg16",
                feature = "pg17"
            )))]
            0, // disabled_nodes
            startup_cost,
            total_cost / work_factor,
            std::ptr::null_mut(), // pathkeys
            std::ptr::null_mut(), // no outer rel
            std::ptr::null_mut(), // no extra plan
            #[cfg(not(any(
                feature = "pg13",
                feature = "pg14",
                feature = "pg15",
                feature = "pg16"
            )))]
            std::ptr::null_mut(), // fdw_restrictinfo
            std::ptr::null_mut(), // no fdw_private data while planning
        );
        // Path might not be parallel_safe if parallel execution is disabled via max_parallel_workers_per_gather=0,
        // failing an assertion in add_partial_path
        if (*parallel_path).path.parallel_safe {
            (*parallel_path).path.parallel_aware = true;
            (*parallel_path).path.parallel_workers = max_parallel_workers_per_gather;
            add_partial_path(baserel, parallel_path as *mut Path);
        }
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_get_foreign_plan(
    _root: *mut PlannerInfo,
    baserel: *mut RelOptInfo,
    _foreigntableid: Oid,
    _best_path: *mut ForeignPath,
    tlist: *mut List,
    scan_clauses: *mut List,
    outer_plan: *mut Plan,
) -> *mut ForeignScan {
    let where_clauses = unsafe { extract_actual_clauses(scan_clauses, false) };
    unsafe {
        make_foreignscan(
            tlist,
            where_clauses,
            (*baserel).relid,
            std::ptr::null_mut(), // fdw_exprs: no expressions to be evaluated by Postgres
            std::ptr::null_mut(), // fdw_private: no fdw data while planning
            std::ptr::null_mut(), // fdw_scan_tlist
            std::ptr::null_mut(), // fdw_recheck_quals
            outer_plan,
        )
    }
}

struct CounterScanState {
    /// The shared counter where values are taken from
    counter_lock: ParallelScanLwLock<i64>,
    /// The number of participants (including the leader when `parallel_leader_participation=true`)
    concurrency: u64,
    /// Rows produced by this process since the scan (re)started.
    produced_rows: u64,
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_begin_foreign_scan(
    foreign_scan_state: *mut ForeignScanState,
    _eflags: ::std::os::raw::c_int,
) {
    let scan_state = CounterScanState {
        counter_lock: SCAN_COUNTER_LOCKS.lock_for(0i64),
        concurrency: 1, // Yet unknown
        produced_rows: 0u64,
    };
    unsafe {
        (*foreign_scan_state).fdw_state = PgMemoryContexts::CurrentMemoryContext
            .leak_and_drop_on_delete(scan_state)
            as *mut ::std::os::raw::c_void
    };
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_iterate_foreign_scan(
    node: *mut ForeignScanState,
) -> *mut TupleTableSlot {
    unsafe {
        let scan_state = &mut *((*node).fdw_state as *mut CounterScanState);

        let slot = (*node).ss.ss_ScanTupleSlot;
        assert!(!slot.is_null());
        (*(*slot).tts_ops).clear.unwrap()(slot);

        // Hack for testing the DSM LWLocks: let other participant workers do some of the work by
        // limiting how much a single worker can produce ahead of others (up to 150% of its share).
        // This was made to avoid that the participant leader finished the query before the
        // parallel workers started up.
        let mut too_much_ahead_compared_to_other_participants = true;
        while too_much_ahead_compared_to_other_participants {
            too_much_ahead_compared_to_other_participants = {
                let count = (*scan_state.counter_lock.shared()).cast_unsigned();
                let share = count * 3 / 2 / scan_state.concurrency.max(1);
                let self_produced = scan_state.produced_rows;
                self_produced > share
            };
            if too_much_ahead_compared_to_other_participants {
                std::thread::sleep(Duration::from_millis(10));
            }
        }

        let column_count = (*(*slot).tts_tupleDescriptor).natts as usize;
        assert_eq!(
            column_count, 1,
            "Foreign table column count should be always = 1 in this example"
        );
        let nulls = std::slice::from_raw_parts_mut((*slot).tts_isnull, column_count);
        let values: &mut [Datum] = std::slice::from_raw_parts_mut((*slot).tts_values, column_count);

        let current_value = {
            let mut counter = scan_state.counter_lock.exclusive();
            let current_value = *counter;
            *counter += 1;
            current_value
        };
        scan_state.produced_rows += 1;

        (*nulls)[0] = false;
        (*values)[0] =
            current_value.into_datum().expect("Unexpected conversion error from i64 to Datum");

        let slot = &mut *slot;
        assert!(!slot.tts_tupleDescriptor.is_null());
        assert!(slot.tts_flags & TTS_FLAG_EMPTY as u16 != 0);
        slot.tts_flags &= !(TTS_FLAG_EMPTY as u16);
        slot.tts_nvalid = (*slot.tts_tupleDescriptor).natts as i16;
        slot
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_end_foreign_scan(foreign_scan_state: *mut ForeignScanState) {
    unsafe {
        let _scan_state = &mut *((*foreign_scan_state).fdw_state as *mut CounterScanState);
        // Nothing to clean-up, as our fdw state is quite naive
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_re_scan_foreign_scan(foreign_scan_state: *mut ForeignScanState) {
    unsafe {
        let scan_state = &mut *((*foreign_scan_state).fdw_state as *mut CounterScanState);
        *scan_state.counter_lock.exclusive() = 0i64;
        scan_state.produced_rows = 0;
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_is_foreign_scan_parallel_safe(
    _root: *mut PlannerInfo,
    _rel: *mut RelOptInfo,
    _rte: *mut RangeTblEntry,
) -> bool {
    true
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_estimate_dsm_foreign_scan(
    _foreign_scan_state: *mut ForeignScanState,
    _pcxt: *mut ParallelContext,
) -> Size {
    ParallelScanLwLock::<i64>::mem_size()
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_initialize_dsm_foreign_scan(
    foreign_scan_state: *mut ForeignScanState,
    pcxt: *mut ParallelContext,
    shared_mem: *mut ::std::os::raw::c_void,
) {
    debug1!("Initializing parallel foreign scan leader (PID: {}) DSM", std::process::id());
    unsafe {
        let scan_state = &mut *((*foreign_scan_state).fdw_state as *mut CounterScanState);

        // Inform all participants about concurrency (check the example test for the reason behind this)
        let workers =
            u64::try_from((*pcxt).nworkers_launched).expect("negative number of parallel workers");
        scan_state.concurrency =
            (workers + u64::from(pg_sys::parallel_leader_participation)).max(1);

        // IMPORTANT! The lock is still local here: move it to the DSM and let parallel workers
        // receive a copy of the handle instead of the state itself
        scan_state.counter_lock.initialize_dsm_and_register_leader(shared_mem);
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_reinitialize_dsm_foreign_scan(
    node: *mut ForeignScanState,
    _pcxt: *mut ParallelContext,
    _coordinate: *mut ::std::os::raw::c_void,
) {
    unsafe {
        let scan_state = &mut *((*node).fdw_state as *mut CounterScanState);
        *(scan_state.counter_lock.exclusive()) = 0i64;
        scan_state.produced_rows = 0;
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_initialize_worker_foreign_scan(
    foreign_scan_state: *mut ForeignScanState,
    _toc: *mut shm_toc,
    coordinate: *mut ::std::os::raw::c_void,
) {
    debug1!("Initializing parallel foreign scan worker (PID: {}) DSM", std::process::id());
    unsafe {
        let scan_state = &mut *((*foreign_scan_state).fdw_state as *mut CounterScanState);

        // IMPORTANT! Register the handle to the state in the DSM wor the parallel worker
        scan_state.counter_lock.register_parallel_worker(coordinate);

        // Not necessary, should have received a copy from the leader initial state
        scan_state.produced_rows = 0u64;
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_shutdown_foreign_scan(_node: *mut ForeignScanState) {
    // Nothing to do here
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_import_foreign_schema(
    stmt: *mut ImportForeignSchemaStmt,
    _server_oid: Oid,
) -> *mut List {
    let stmt = unsafe { &(*stmt) };
    let table_name = unsafe {
        CStr::from_ptr(quote_qualified_identifier(std::ptr::null_mut(), stmt.remote_schema))
            .to_str()
            .unwrap()
    };
    let server_name = unsafe {
        CStr::from_ptr(quote_qualified_identifier(std::ptr::null_mut(), stmt.server_name))
            .to_str()
            .unwrap()
    };
    let create_table_statement =
        format!(r#"CREATE FOREIGN TABLE {} ("counter" bigint) SERVER {}"#, table_name, server_name);

    pgrx::memcx::current_context(|memcx| {
        let mut statements = pgrx::list::List::default();
        statements.unstable_push_in_context(
            pgrx::StringInfo::from(create_table_statement).into_char_ptr() as *const c_void
                as *mut c_void,
            memcx,
        );
        statements.into_ptr()
    })
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    const ROWS_LIMIT: u64 = 1_000_000;
    const PARALLEL_WORKERS: u64 = 2; // Add +1 when counting also the leader

    enum ParallelLeaderParticipation {
        On,
        Off,
    }

    #[pg_test]
    fn parallel_workers_share_the_counter() {
        assert_parallel_scan_yields_distinct_counters(ParallelLeaderParticipation::Off);
    }

    #[pg_test]
    fn parallel_workers_and_leader_share_the_counter() {
        assert_parallel_scan_yields_distinct_counters(ParallelLeaderParticipation::On);
    }

    fn assert_parallel_scan_yields_distinct_counters(
        leader_participation: ParallelLeaderParticipation,
    ) {
        let leader_participation = match leader_participation {
            ParallelLeaderParticipation::On => true,
            ParallelLeaderParticipation::Off => false,
        };
        Spi::run(&format!(
            "CREATE FOREIGN DATA WRAPPER counter HANDLER counter_handler VALIDATOR counter_validator;
            CREATE SERVER counter_srv FOREIGN DATA WRAPPER counter;
            IMPORT FOREIGN SCHEMA my_counter FROM SERVER counter_srv INTO tests;
            SET LOCAL parallel_setup_cost = 0;
            SET LOCAL parallel_tuple_cost = 0;
            SET LOCAL max_parallel_workers_per_gather = {PARALLEL_WORKERS};
            SET LOCAL parallel_leader_participation = {leader_participation};"
        ))
            .unwrap();

        // EXPLAIN ANALYZE executes a CREATE TABLE AS, so the plan describes the same parallel
        // execution that produced the rows checked below. We read the EXPLAIN (FORMAT JSON) to
        // find out how many rows produced each parallel worker.
        let plan = Spi::connect_mut(|client| {
            client
                .update(
                    &format!(
                        "EXPLAIN (ANALYZE, VERBOSE, FORMAT JSON)
                        CREATE TABLE tests.scanned AS SELECT counter FROM tests.my_counter LIMIT {ROWS_LIMIT}"
                    ),
                    None,
                    &[],
                )?
                .first()
                .get_one::<pgrx::Json>()
        })
            .unwrap()
            .unwrap()
            .0;

        let gather = find_plan_node(&plan[0]["Plan"], "Gather")
            .unwrap_or_else(|| panic!("missing Gather node: {plan}"));
        assert_eq!(gather["Workers Launched"], PARALLEL_WORKERS, "{plan}");
        let scan = find_plan_node(gather, "Foreign Scan")
            .unwrap_or_else(|| panic!("missing Foreign Scan node: {plan}"));

        // Check that all workers participated in producing at least one row
        let worker_rows: Vec<f64> = scan["Workers"]
            .as_array()
            .unwrap_or_else(|| panic!("missing per-worker statistics: {plan}"))
            .iter()
            .filter_map(|worker| worker["Actual Rows"].as_f64())
            .collect();
        assert_eq!(worker_rows.len() as u64, PARALLEL_WORKERS, "{plan}");
        assert!(
            worker_rows.iter().all(|&rows| rows > 0.0),
            "each worker should produce at least one row"
        );

        let counts =
            Spi::get_two::<i64, i64>("SELECT count(*), count(DISTINCT counter) FROM tests.scanned");
        assert_eq!(counts, Ok((Some(ROWS_LIMIT as i64), Some(ROWS_LIMIT as i64))));
    }

    fn find_plan_node<'a>(
        node: &'a serde_json::Value,
        node_type: &str,
    ) -> Option<&'a serde_json::Value> {
        if node["Node Type"] == node_type {
            return Some(node);
        }
        node["Plans"].as_array()?.iter().find_map(|child| find_plan_node(child, node_type))
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec!["shared_preload_libraries='parallel_scan_lwlock'"]
    }
}

pub static EMPTY_FDW: FdwRoutine = FdwRoutine {
    type_: NodeTag::T_FdwRoutine,
    GetForeignRelSize: None,
    GetForeignPaths: None,
    GetForeignPlan: None,
    BeginForeignScan: None,
    IterateForeignScan: None,
    ReScanForeignScan: None,
    EndForeignScan: None,
    GetForeignJoinPaths: None,
    GetForeignUpperPaths: None,
    AddForeignUpdateTargets: None,
    PlanForeignModify: None,
    BeginForeignModify: None,
    ExecForeignInsert: None,
    ExecForeignUpdate: None,
    ExecForeignDelete: None,
    EndForeignModify: None,
    BeginForeignInsert: None,
    EndForeignInsert: None,
    IsForeignRelUpdatable: None,
    PlanDirectModify: None,
    BeginDirectModify: None,
    IterateDirectModify: None,
    EndDirectModify: None,
    GetForeignRowMarkType: None,
    RefetchForeignRow: None,
    RecheckForeignScan: None,
    ExplainForeignScan: None,
    ExplainForeignModify: None,
    ExplainDirectModify: None,
    AnalyzeForeignTable: None,
    #[cfg(not(any(
        feature = "pg13",
        feature = "pg14",
        feature = "pg15",
        feature = "pg16",
        feature = "pg17",
        feature = "pg18",
    )))]
    ImportForeignStatistics: None,
    ImportForeignSchema: None,
    IsForeignScanParallelSafe: None,
    EstimateDSMForeignScan: None,
    InitializeDSMForeignScan: None,
    ReInitializeDSMForeignScan: None,
    InitializeWorkerForeignScan: None,
    ShutdownForeignScan: None,
    ReparameterizeForeignPathByChild: None,
    #[cfg(not(feature = "pg13"))]
    ExecForeignBatchInsert: None,
    #[cfg(not(feature = "pg13"))]
    ExecForeignTruncate: None,
    #[cfg(not(feature = "pg13"))]
    ForeignAsyncConfigureWait: None,
    #[cfg(not(feature = "pg13"))]
    ForeignAsyncNotify: None,
    #[cfg(not(feature = "pg13"))]
    ForeignAsyncRequest: None,
    #[cfg(not(feature = "pg13"))]
    GetForeignModifyBatchSize: None,
    #[cfg(not(feature = "pg13"))]
    IsForeignPathAsyncCapable: None,
};
