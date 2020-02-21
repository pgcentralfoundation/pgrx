#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    unsafe fn test_callbacks() {
        pgx::hooks::init();
        struct TestHook {
            events: u32,
        }
        impl PgHooks for TestHook {
            fn executor_before_start(
                &mut self,
                _query_desc: &PgBox<pg_sys::QueryDesc>,
                _eflags: i32,
            ) {
                self.events += 1;
            }

            fn executor_after_start(
                &mut self,
                _query_desc: &PgBox<pg_sys::QueryDesc>,
                _eflags: i32,
            ) {
                self.events += 1;
            }

            fn executor_before_run(
                &mut self,
                _query_desc: &PgBox<pg_sys::QueryDesc>,
                _direction: i32,
                _count: u64,
                _execute_once: bool,
            ) {
                self.events += 1;
            }

            fn executor_after_run(
                &mut self,
                _query_desc: &PgBox<pg_sys::QueryDesc>,
                _direction: i32,
                _count: u64,
                _execute_once: bool,
            ) {
                self.events += 1;
            }

            fn executor_before_finish(&mut self, _query_desc: &PgBox<pg_sys::QueryDesc>) {
                self.events += 1;
            }

            fn executor_after_finish(&mut self, _query_desc: &PgBox<pg_sys::QueryDesc>) {
                self.events += 1;
            }

            fn executor_before_end(&mut self, _query_desc: &PgBox<pg_sys::QueryDesc>) {
                self.events += 1;
            }

            fn executor_after_end(&mut self, _query_desc: &PgBox<pg_sys::QueryDesc>) {
                self.events += 1;
            }

            fn executor_check_perms(
                &mut self,
                _range_table: &PgList<*mut pg_sys::RangeTblEntry>,
                _ereport_on_violation: bool,
            ) -> bool {
                self.events += 1;
                true
            }
        }

        static mut HOOK: TestHook = TestHook { events: 0 };
        pgx::hooks::register_hook(&mut HOOK);
        Spi::run("SELECT 1");
        assert_eq!(9, HOOK.events);

        // TODO:  it'd be nice to also test that .commit() and .abort() also get called
        //    but I don't see how to do that since we're running *inside* a transaction here
    }
}
