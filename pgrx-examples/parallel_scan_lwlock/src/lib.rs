//! A foreign data wrapper whose tables return the values of a counter. A parallel scan shares
//! one counter among all its participants, guarded by a [ParallelScanLwLock] in dynamic shared
//! memory (DSM).
//!
//! Initialization:
//! * [_PG_init] for startup initialization
//! * `counter_handler` for startup registration of FDW handler callbacks
//!
//! Scan start:
//! * `pgrx_get_foreign_rel_size` for table-level estimates
//! * `pgrx_get_foreign_paths` provides the scan strategies available (just scan and parallel_scan)
//! * `pgrx_get_foreign_plan` processes clauses and constructs an executable plan
//!
//! Sequential scan execution:
//! * `pgrx_begin_foreign_scan` creates the scan state, with the counter in local memory
//! * `pgrx_iterate_foreign_scan` returns the next counter value
//! * `pgrx_end_foreign_scan` cleans up on scan complete
//!
//! Parallel scan execution:
//! * `pgrx_begin_foreign_scan` creates the scan state, with the counter in local memory
//! * `pgrx_estimate_dsm_foreign_scan` calculates the amount of shared memory needed for the counter
//! * `pgrx_initialize_dsm_foreign_scan` moves the counter and its lock into the allocated DSM
//! * `pgrx_initialize_worker_foreign_scan` points each worker's lock at the counter in the DSM
//! * `pgrx_shutdown_foreign_scan` is the last chance to use the counter before the DSM goes away
//! * See `Sequential scan execution` above
//!
//! Other support functions:
//! * `counter_validator` called for option validation
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
use pgrx::guc::{GucContext, GucFlags, GucRegistry, GucSetting};
use pgrx::lwlock::scan::*;
use pgrx::pg_sys::*;
use pgrx::*;
use std::ffi::{CStr, c_void};
use std::time::{Duration, Instant};

pg_module_magic!(name, version);

static SCAN_COUNTER_LOCKS: ParallelScanLwLockTranche =
    ParallelScanLwLockTranche::new(c"scan_counter_lock");

static WAIT_FOR_PARTICIPANTS: GucSetting<bool> = GucSetting::<bool>::new(false);

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    if unsafe { !pgrx::pg_sys::process_shared_preload_libraries_in_progress } {
        pgrx::error!("this extension must be loaded via shared_preload_libraries.");
    }
    pg_shmem_init!(SCAN_COUNTER_LOCKS);
    GucRegistry::define_bool_guc(
        c"parallel_scan_lwlock.wait_for_participants",
        c"Hold each participant's first row until all planned participants have one.",
        c"Makes every launched parallel worker return rows, which the tests rely on.",
        &WAIT_FOR_PARTICIPANTS,
        GucContext::Userset,
        GucFlags::default(),
    );
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
        type_: NodeTag::T_FdwRoutine,
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
        ..Default::default()
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
    baserel: *mut RelOptInfo,
    _foreigntableid: Oid,
) {
    let expected_query_rows = 1_000_000f64; // Arbitrary
    unsafe {
        (*baserel).tuples = expected_query_rows;
        (*baserel).rows = expected_query_rows;
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
    let foreign_path = |total_cost| unsafe {
        create_foreignscan_path(
            root,
            baserel,
            std::ptr::null_mut(), // Use default target
            (*baserel).rows,
            #[cfg(any(feature = "pg18", feature = "pg19"))]
            0, // disabled_nodes
            startup_cost,
            total_cost,
            std::ptr::null_mut(), // pathkeys
            std::ptr::null_mut(), // no outer rel
            std::ptr::null_mut(), // no extra plan
            #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
            std::ptr::null_mut(), // fdw_restrictinfo
            std::ptr::null_mut(), // no fdw_private data while planning
        )
    };

    unsafe {
        add_path(baserel, foreign_path(total_cost).cast());

        let workers = max_parallel_workers_per_gather;
        let parallel_path = foreign_path(total_cost / f64::from(workers));
        // Path might not be parallel_safe if parallel execution is disabled via max_parallel_workers_per_gather=0,
        // failing an assertion in add_partial_path
        if (*parallel_path).path.parallel_safe {
            (*parallel_path).path.parallel_aware = true;
            (*parallel_path).path.parallel_workers = workers;
            add_partial_path(baserel, parallel_path.cast());
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

/// The state that a scan's participants share through the lock.
#[derive(Clone, Copy, Default)]
struct Counter {
    /// The next value to return.
    next: i64,
    /// Participants the leader planned for, and how many have taken a value. Used only when
    /// `parallel_scan_lwlock.wait_for_participants` is on.
    planned: u32,
    started: u32,
}

// SAFETY: `Counter` holds only integers, which mean the same in every process.
unsafe impl PGRXSharedMemory for Counter {}

struct CounterScanState {
    counter: ParallelScanLwLock<Counter>,
    /// Whether this process has taken a value since the scan (re)started.
    started: bool,
}

/// Returns the scan state that [pgrx_begin_foreign_scan] stored in `node`.
///
/// # Safety
///
/// `node` must be a scan of this FDW that began, and the returned reference must be the only one.
unsafe fn scan_state<'a>(node: *mut ForeignScanState) -> &'a mut CounterScanState {
    // SAFETY: the caller guarantees that `fdw_state` holds a live `CounterScanState`, which
    // `pgrx_begin_foreign_scan` leaked into the executor's memory context.
    unsafe { &mut *(*node).fdw_state.cast() }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_begin_foreign_scan(
    foreign_scan_state: *mut ForeignScanState,
    _eflags: ::std::os::raw::c_int,
) {
    let scan_state = CounterScanState {
        counter: SCAN_COUNTER_LOCKS.lock_for(Counter::default()),
        started: false,
    };
    unsafe {
        (*foreign_scan_state).fdw_state =
            PgMemoryContexts::CurrentMemoryContext.leak_and_drop_on_delete(scan_state).cast()
    };
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_iterate_foreign_scan(
    node: *mut ForeignScanState,
) -> *mut TupleTableSlot {
    unsafe {
        let scan_state = scan_state(node);

        let slot = (*node).ss.ss_ScanTupleSlot;
        assert!(!slot.is_null());
        (*(*slot).tts_ops).clear.unwrap()(slot);

        let value = {
            let mut counter = scan_state.counter.exclusive();
            let value = counter.next;
            counter.next += 1;
            value
        };
        if !std::mem::replace(&mut scan_state.started, true) && WAIT_FOR_PARTICIPANTS.get() {
            wait_for_participants(&mut scan_state.counter);
        }

        assert_eq!(
            (*(*slot).tts_tupleDescriptor).natts,
            1,
            "Foreign table column count should be always = 1 in this example"
        );
        *(*slot).tts_isnull = false;
        *(*slot).tts_values =
            value.into_datum().expect("Unexpected conversion error from i64 to Datum");
        ExecStoreVirtualTuple(slot)
    }
}

/// Holds this participant's first row until every participant the leader planned for has taken
/// a value, so that no participant reaches the query's `LIMIT` before the others start.
///
/// Gives up after 30 seconds, in case PostgreSQL launched fewer workers than planned.
fn wait_for_participants(counter: &mut ParallelScanLwLock<Counter>) {
    counter.exclusive().started += 1;
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        let Counter { planned, started, .. } = *counter.share();
        if started >= planned {
            return;
        }
        // SAFETY: this runs on the backend thread, from an FDW callback guarded by #[pg_guard],
        // and holds no lock guard that an error from the interrupt check could skip.
        unsafe { pg_sys::CHECK_FOR_INTERRUPTS!() };
        std::thread::sleep(Duration::from_millis(1));
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_end_foreign_scan(_foreign_scan_state: *mut ForeignScanState) {
    // Nothing to clean up. A parallel scan's DSM may be gone by now, so this must not use the
    // counter.
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_re_scan_foreign_scan(foreign_scan_state: *mut ForeignScanState) {
    let scan_state = unsafe { scan_state(foreign_scan_state) };
    // A parallel scan resets the shared counter in pgrx_reinitialize_dsm_foreign_scan instead:
    // PostgreSQL can call this function in the leader after the new workers took values.
    if let ParallelScanLwLockExclusiveGuard::Local(counter) = scan_state.counter.exclusive() {
        *counter = Counter::default();
    }
    scan_state.started = false;
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
    ParallelScanLwLock::<Counter>::mem_size()
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_initialize_dsm_foreign_scan(
    foreign_scan_state: *mut ForeignScanState,
    pcxt: *mut ParallelContext,
    shared_mem: *mut ::std::os::raw::c_void,
) {
    debug1!("Initializing parallel foreign scan leader (PID: {}) DSM", std::process::id());
    unsafe {
        let scan_state = scan_state(foreign_scan_state);

        // Workers launch after this function returns, so count the planned ones.
        let workers = u32::try_from((*pcxt).nworkers).expect("negative number of parallel workers");
        scan_state.counter.exclusive().planned =
            workers + u32::from(pg_sys::parallel_leader_participation);

        // IMPORTANT! The lock is still local here: move it to the DSM and let parallel workers
        // receive a copy of the handle instead of the state itself.
        // SAFETY: PostgreSQL sized `shared_mem` with pgrx_estimate_dsm_foreign_scan, and nothing
        // uses the counter after pgrx_shutdown_foreign_scan, when the DSM may go away.
        scan_state.counter.initialize_dsm_and_register_leader(shared_mem);
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_reinitialize_dsm_foreign_scan(
    node: *mut ForeignScanState,
    _pcxt: *mut ParallelContext,
    _coordinate: *mut ::std::os::raw::c_void,
) {
    let scan_state = unsafe { scan_state(node) };
    let mut counter = scan_state.counter.exclusive();
    counter.next = 0;
    counter.started = 0;
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_initialize_worker_foreign_scan(
    foreign_scan_state: *mut ForeignScanState,
    _toc: *mut shm_toc,
    coordinate: *mut ::std::os::raw::c_void,
) {
    debug1!("Initializing parallel foreign scan worker (PID: {}) DSM", std::process::id());
    unsafe {
        let scan_state = scan_state(foreign_scan_state);

        // IMPORTANT! Point this worker's lock at the counter the leader moved into the DSM.
        // SAFETY: the leader initialized `coordinate` in pgrx_initialize_dsm_foreign_scan, and
        // nothing uses the counter after pgrx_shutdown_foreign_scan.
        scan_state.counter.register_parallel_worker(coordinate);
    }
}

#[pg_guard]
unsafe extern "C-unwind" fn pgrx_shutdown_foreign_scan(_node: *mut ForeignScanState) {
    // PostgreSQL may destroy the DSM right after this returns. Copy anything that EXPLAIN or
    // pgrx_end_foreign_scan needs out of the counter here; this example needs nothing.
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

    /// Creates the foreign table and forces parallel plans that return rows from every worker.
    fn set_up_parallel_scan(leader_participation: ParallelLeaderParticipation) {
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
            SET LOCAL parallel_leader_participation = {leader_participation};
            SET LOCAL parallel_scan_lwlock.wait_for_participants = on;"
        ))
        .unwrap();
    }

    fn explain_analyze(statement: &str) -> serde_json::Value {
        Spi::connect_mut(|client| {
            client
                .update(&format!("EXPLAIN (ANALYZE, VERBOSE, FORMAT JSON) {statement}"), None, &[])?
                .first()
                .get_one::<pgrx::Json>()
        })
        .unwrap()
        .unwrap()
        .0
    }

    fn assert_parallel_scan_yields_distinct_counters(
        leader_participation: ParallelLeaderParticipation,
    ) {
        set_up_parallel_scan(leader_participation);

        // EXPLAIN ANALYZE executes a CREATE TABLE AS, so the plan describes the same parallel
        // execution that produced the rows checked below. We read the EXPLAIN (FORMAT JSON) to
        // find out how many rows produced each parallel worker.
        let plan = explain_analyze(&format!(
            "CREATE TABLE tests.scanned AS SELECT counter FROM tests.my_counter LIMIT {ROWS_LIMIT}"
        ));

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
            "each worker should produce at least one row: {plan}"
        );

        let counts =
            Spi::get_two::<i64, i64>("SELECT count(*), count(DISTINCT counter) FROM tests.scanned");
        assert_eq!(counts, Ok((Some(ROWS_LIMIT as i64), Some(ROWS_LIMIT as i64))));
    }

    /// A parallel scan on the inner side of a nested loop runs once per outer row. Each rescan
    /// reinitializes the DSM and launches new workers, which must find the counter reset.
    #[pg_test]
    fn parallel_rescans_restart_the_counter() {
        set_up_parallel_scan(ParallelLeaderParticipation::On);
        const LOOPS: i64 = 2;
        const ROWS: i64 = 1000;
        let plan = explain_analyze(&format!(
            "CREATE TABLE tests.rescanned AS
             SELECT g, counter FROM generate_series(1, {LOOPS}) g,
             LATERAL (SELECT counter FROM tests.my_counter LIMIT {ROWS} + 0 * g) s"
        ));
        let gather = find_plan_node(&plan[0]["Plan"], "Gather")
            .unwrap_or_else(|| panic!("missing Gather node: {plan}"));
        assert_eq!(gather["Actual Loops"], LOOPS, "{plan}");
        assert_eq!(gather["Workers Launched"], PARALLEL_WORKERS, "{plan}");

        // Each participant's first row holds one of the first values taken after the reset, so
        // every loop starts below the number of participants. Without the reset, the second loop
        // would start at ROWS or above.
        let participants = PARALLEL_WORKERS as i64 + 1;
        let loops = Spi::connect(|client| {
            client
                .select(
                    "SELECT count(*), count(DISTINCT counter), min(counter)
                     FROM tests.rescanned GROUP BY g ORDER BY g",
                    None,
                    &[],
                )?
                .map(|row| Ok((row.get::<i64>(1)?, row.get::<i64>(2)?, row.get::<i64>(3)?)))
                .collect::<Result<Vec<_>, pgrx::spi::SpiError>>()
        })
        .unwrap();
        assert_eq!(loops.len() as i64, LOOPS, "{plan}");
        for (rows, distinct, min) in loops {
            assert_eq!((rows, distinct), (Some(ROWS), Some(ROWS)), "{plan}");
            assert!(min.is_some_and(|min| min < participants), "loop started at {min:?}: {plan}");
        }
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
