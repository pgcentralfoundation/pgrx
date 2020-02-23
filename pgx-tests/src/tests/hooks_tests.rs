#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    unsafe fn test_callbacks() {
        use pgx::pg_sys::*;

        struct TestHook {
            events: u32,
        }
        impl PgHooks for TestHook {
            fn executor_start(
                &mut self,
                query_desc: PgBox<QueryDesc>,
                eflags: i32,
                prev_hook: fn(PgBox<QueryDesc>, i32) -> HookResult<()>,
            ) -> HookResult<()> {
                self.events += 1;
                prev_hook(query_desc, eflags)
            }

            fn executor_run(
                &mut self,
                query_desc: PgBox<QueryDesc>,
                direction: i32,
                count: u64,
                execute_once: bool,
                prev_hook: fn(PgBox<QueryDesc>, i32, u64, bool) -> HookResult<()>,
            ) -> HookResult<()> {
                self.events += 1;
                prev_hook(query_desc, direction, count, execute_once)
            }

            fn executor_finish(
                &mut self,
                query_desc: PgBox<QueryDesc>,
                prev_hook: fn(PgBox<QueryDesc>) -> HookResult<()>,
            ) -> HookResult<()> {
                self.events += 1;
                prev_hook(query_desc)
            }

            fn executor_end(
                &mut self,
                query_desc: PgBox<QueryDesc>,
                prev_hook: fn(PgBox<QueryDesc>) -> HookResult<()>,
            ) -> HookResult<()> {
                self.events += 1;
                prev_hook(query_desc)
            }

            fn executor_check_perms(
                &mut self,
                range_table: PgList<*mut RangeTblEntry>,
                ereport_on_violation: bool,
                prev_hook: fn(PgList<*mut RangeTblEntry>, bool) -> HookResult<bool>,
            ) -> HookResult<bool> {
                self.events += 1;
                prev_hook(range_table, ereport_on_violation)
            }

            fn planner(
                &mut self,
                parse: PgBox<Query>,
                cursor_options: i32,
                bound_params: PgBox<ParamListInfoData>,
                prev_hook: fn(
                    PgBox<Query>,
                    i32,
                    PgBox<ParamListInfoData>,
                ) -> HookResult<*mut PlannedStmt>,
            ) -> HookResult<*mut PlannedStmt> {
                self.events += 1;
                prev_hook(parse, cursor_options, bound_params)
            }
        }

        static mut HOOK: TestHook = TestHook { events: 0 };
        pgx::hooks::register_hook(&mut HOOK);
        Spi::run("SELECT 1");
        assert_eq!(6, HOOK.events);

        // TODO:  it'd be nice to also test that .commit() and .abort() also get called
        //    but I don't see how to do that since we're running *inside* a transaction here
    }
}
