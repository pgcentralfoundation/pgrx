use crate::{pg_guard, pg_sys, void_mut_ptr, PgBox, PgList};

pub trait PgHooks {
    /// Called when a transaction commits
    fn commit(&mut self) {}

    /// Called when a transacton aborts
    fn abort(&mut self) {}

    /// Hook for plugins to get control in ExecutorStart()
    fn executor_before_start(&mut self, _query_desc: &PgBox<pg_sys::QueryDesc>, _eflags: i32) {}
    fn executor_after_start(&mut self, _query_desc: &PgBox<pg_sys::QueryDesc>, _eflags: i32) {}

    /// Hook for plugins to get control in ExecutorRun()
    fn executor_before_run(
        &mut self,
        _query_desc: &PgBox<pg_sys::QueryDesc>,
        _direction: pg_sys::ScanDirection,
        _count: u64,
        _execute_once: bool,
    ) {
    }
    fn executor_after_run(
        &mut self,
        _query_desc: &PgBox<pg_sys::QueryDesc>,
        _direction: pg_sys::ScanDirection,
        _count: u64,
        _execute_once: bool,
    ) {
    }

    /// Hook for plugins to get control in ExecutorFinish()
    fn executor_before_finish(&mut self, _query_desc: &PgBox<pg_sys::QueryDesc>) {}
    fn executor_after_finish(&mut self, _query_desc: &PgBox<pg_sys::QueryDesc>) {}

    /// Hook for plugins to get control in ExecutorEnd()
    fn executor_before_end(&mut self, _query_desc: &PgBox<pg_sys::QueryDesc>) {}
    fn executor_after_end(&mut self, _query_desc: &PgBox<pg_sys::QueryDesc>) {}

    /// Hook for plugins to get control in ExecCheckRTPerms()
    fn executor_check_perms(
        &mut self,
        _range_table: &PgList<*mut pg_sys::RangeTblEntry>,
        _ereport_on_violation: bool,
    ) -> bool {
        true
    }
}

static mut REGISTRATIONS: Vec<
    Box<&'static mut (dyn PgHooks + std::panic::UnwindSafe + std::panic::RefUnwindSafe)>,
> = Vec::new();

static mut EXECUTOR_START_HOOK: pg_sys::ExecutorStart_hook_type = None;
static mut EXECUTOR_RUN_HOOK: pg_sys::ExecutorRun_hook_type = None;
static mut EXECUTOR_FINISH_HOOK: pg_sys::ExecutorFinish_hook_type = None;
static mut EXECUTOR_END_HOOK: pg_sys::ExecutorEnd_hook_type = None;
static mut EXECUTOR_CHECK_PERMS_HOOK: pg_sys::ExecutorCheckPerms_hook_type = None;

/// Register a `PgHook` instance to respond to the various hook points
pub fn register_hook(
    hook: &'static mut (dyn PgHooks + std::panic::UnwindSafe + std::panic::RefUnwindSafe),
) {
    unsafe extern "C" fn xact_callback(event: pg_sys::XactEvent, _: void_mut_ptr) {
        match event {
            pg_sys::XactEvent_XACT_EVENT_ABORT => {
                for hook in REGISTRATIONS.iter_mut() {
                    hook.abort();
                }
            }
            pg_sys::XactEvent_XACT_EVENT_PRE_COMMIT => {
                for hook in REGISTRATIONS.iter_mut() {
                    hook.commit();
                }
            }
            _ => { /* noop */ }
        }
    }

    unsafe {
        if REGISTRATIONS.is_empty() {
            pg_sys::RegisterXactCallback(Some(xact_callback), std::ptr::null_mut());
        }
        REGISTRATIONS.push(Box::new(hook));
    }
}

/// Initialize the system for hooking Postgres' various function hook types
pub unsafe fn init() {
    EXECUTOR_START_HOOK = pg_sys::ExecutorStart_hook
        .replace(pgx_executor_start)
        .or(Some(pgx_standard_executor_start_wrapper));
    EXECUTOR_RUN_HOOK = pg_sys::ExecutorRun_hook
        .replace(pgx_executor_run)
        .or(Some(pgx_standard_executor_run_wrapper));
    EXECUTOR_FINISH_HOOK = pg_sys::ExecutorFinish_hook
        .replace(pgx_executor_finish)
        .or(Some(pgx_standard_executor_finish_wrapper));
    EXECUTOR_END_HOOK = pg_sys::ExecutorEnd_hook
        .replace(pgx_executor_end)
        .or(Some(pgx_standard_executor_end_wrapper));
    EXECUTOR_CHECK_PERMS_HOOK = pg_sys::ExecutorCheckPerms_hook.replace(pgx_executor_check_perms);
}

#[pg_guard]
unsafe extern "C" fn pgx_executor_start(query_desc: *mut pg_sys::QueryDesc, eflags: i32) {
    let query_desc_boxed = PgBox::from_pg(query_desc);
    for hook in REGISTRATIONS.iter_mut() {
        hook.executor_before_start(&query_desc_boxed, eflags);
    }

    (EXECUTOR_START_HOOK.as_ref().unwrap())(query_desc, eflags);

    for hook in REGISTRATIONS.iter_mut() {
        hook.executor_after_start(&query_desc_boxed, eflags);
    }
}

#[pg_guard]
unsafe extern "C" fn pgx_executor_run(
    query_desc: *mut pg_sys::QueryDesc,
    direction: pg_sys::ScanDirection,
    count: u64,
    execute_once: bool,
) {
    let query_desc_boxed = PgBox::from_pg(query_desc);
    for hook in REGISTRATIONS.iter_mut() {
        hook.executor_before_run(&query_desc_boxed, direction, count, execute_once);
    }

    (EXECUTOR_RUN_HOOK.as_ref().unwrap())(query_desc, direction, count, execute_once);

    for hook in REGISTRATIONS.iter_mut() {
        hook.executor_after_run(&query_desc_boxed, direction, count, execute_once);
    }
}

#[pg_guard]
unsafe extern "C" fn pgx_executor_finish(query_desc: *mut pg_sys::QueryDesc) {
    let query_desc_boxed = PgBox::from_pg(query_desc);
    for hook in REGISTRATIONS.iter_mut() {
        hook.executor_before_finish(&query_desc_boxed);
    }

    (EXECUTOR_FINISH_HOOK.as_ref().unwrap())(query_desc);

    for hook in REGISTRATIONS.iter_mut() {
        hook.executor_after_finish(&query_desc_boxed);
    }
}

#[pg_guard]
unsafe extern "C" fn pgx_executor_end(query_desc: *mut pg_sys::QueryDesc) {
    let query_desc_boxed = PgBox::from_pg(query_desc);
    for hook in REGISTRATIONS.iter_mut() {
        hook.executor_before_end(&query_desc_boxed);
    }

    (EXECUTOR_END_HOOK.as_ref().unwrap())(query_desc);

    for hook in REGISTRATIONS.iter_mut() {
        hook.executor_after_end(&query_desc_boxed);
    }
}

#[pg_guard]
unsafe extern "C" fn pgx_executor_check_perms(
    range_table: *mut pg_sys::List,
    ereport_on_violation: bool,
) -> bool {
    let range_table = PgList::from_pg(range_table);
    for hook in REGISTRATIONS.iter_mut() {
        if !hook.executor_check_perms(&range_table, ereport_on_violation) {
            return false;
        }
    }

    true
}

unsafe extern "C" fn pgx_standard_executor_start_wrapper(
    query_desc: *mut pg_sys::QueryDesc,
    eflags: i32,
) {
    pg_sys::standard_ExecutorStart(query_desc, eflags);
}

unsafe extern "C" fn pgx_standard_executor_run_wrapper(
    query_desc: *mut pg_sys::QueryDesc,
    direction: pg_sys::ScanDirection,
    count: u64,
    execute_once: bool,
) {
    pg_sys::standard_ExecutorRun(query_desc, direction, count, execute_once);
}

unsafe extern "C" fn pgx_standard_executor_finish_wrapper(query_desc: *mut pg_sys::QueryDesc) {
    pg_sys::standard_ExecutorFinish(query_desc);
}

unsafe extern "C" fn pgx_standard_executor_end_wrapper(query_desc: *mut pg_sys::QueryDesc) {
    pg_sys::standard_ExecutorEnd(query_desc);
}
