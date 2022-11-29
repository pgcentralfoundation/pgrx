/*
Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pg_sys::{
    Const, Node, NodeTag_T_Const, ParamListInfoData, PgNode, PlannedStmt, Query, QueryDesc,
    RangeTblEntry, FLOAT4OID, FLOAT8OID, NUMERICOID,
};
use pgx::{cstr_core::c_char, error, prelude::*, warning, FromDatum, HookResult, PgHooks, PgList};

pub struct ExampleHook {}
impl PgHooks for ExampleHook {
    fn planner(
        &mut self,
        parse: PgBox<Query>,
        query_string: *const c_char,
        cursor_options: i32,
        bound_params: PgBox<ParamListInfoData>,
        prev_hook: fn(
            PgBox<Query>,
            query_string: *const c_char,
            i32,
            PgBox<ParamListInfoData>,
        ) -> HookResult<*mut PlannedStmt>,
    ) -> HookResult<*mut PlannedStmt> {
        let planned_stmt = prev_hook(parse, query_string, cursor_options, bound_params);
        struct Args {
            has_pi_approximation: bool,
        }
        let mut arg = Args { has_pi_approximation: false };
        fn pi_finder(node: *mut Node, context: &mut Args) -> () {
            unsafe {
                if node.is_null() {
                    return;
                }

                if (*node).type_ == NodeTag_T_Const {
                    let constant = *(node as *mut Const);

                    let floatval: Option<f64> = match constant.consttype {
                        FLOAT4OID => f32::from_polymorphic_datum(
                            constant.constvalue,
                            constant.constisnull,
                            constant.consttype,
                        )
                        .map(|v| v as f64),
                        FLOAT8OID => f64::from_polymorphic_datum(
                            constant.constvalue,
                            constant.constisnull,
                            constant.consttype,
                        ),
                        NUMERICOID => AnyNumeric::from_polymorphic_datum(
                            constant.constvalue,
                            constant.constisnull,
                            constant.consttype,
                        )
                        .and_then(|s| format!("{}", s).parse().ok()),
                        _ => None,
                    };

                    if floatval.is_none() {
                        return;
                    }
                    let floatval = floatval.unwrap();
                    if 3.14 <= floatval && floatval < 3.142 && floatval != std::f64::consts::PI {
                        context.has_pi_approximation = true;
                        error!("Found a bad approximation for pi!");
                    }
                }
            }
        }
        warning!("Traversing planned statement {:?}", planned_stmt.inner);

        unsafe { (*planned_stmt.inner).traverse(pi_finder, &mut arg) };

        if arg.has_pi_approximation {
            error!("Found a bad approximation for pi!",);
        }
        planned_stmt
    }
    // Just use the upstream behavior for everything but the planner.
    fn executor_start(
        &mut self,
        query_desc: PgBox<QueryDesc>,
        eflags: i32,
        prev_hook: fn(PgBox<QueryDesc>, i32) -> HookResult<()>,
    ) -> HookResult<()> {
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
        prev_hook(query_desc, direction, count, execute_once)
    }

    fn executor_finish(
        &mut self,
        query_desc: PgBox<QueryDesc>,
        prev_hook: fn(PgBox<QueryDesc>) -> HookResult<()>,
    ) -> HookResult<()> {
        prev_hook(query_desc)
    }

    fn executor_end(
        &mut self,
        query_desc: PgBox<QueryDesc>,
        prev_hook: fn(PgBox<QueryDesc>) -> HookResult<()>,
    ) -> HookResult<()> {
        prev_hook(query_desc)
    }

    fn executor_check_perms(
        &mut self,
        range_table: PgList<*mut RangeTblEntry>,
        ereport_on_violation: bool,
        prev_hook: fn(PgList<*mut RangeTblEntry>, bool) -> HookResult<bool>,
    ) -> HookResult<bool> {
        prev_hook(range_table, ereport_on_violation)
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;
    use crate::tests::node_tests::ExampleHook;

    use pgx::{prelude::*, register_hook, warning};

    #[pg_test]
    fn with_no_pi() {
        static mut HOOK: ExampleHook = ExampleHook {};
        unsafe {
            register_hook(&mut HOOK);
        }
        warning!("Registered hook!");
        Spi::run("SELECT 1 where 3.141 < 4 group by 1 order by 1;");
    }
    #[pg_test(error = "Found a bad approximation for pi!")]
    fn in_targetlist() {
        static mut HOOK: ExampleHook = ExampleHook {};
        unsafe {
            register_hook(&mut HOOK);
        }
        Spi::run("SELECT 3.141;");
    }
    #[pg_test(error = "Found a bad approximation for pi!")]
    fn in_with_clause() {
        static mut HOOK: ExampleHook = ExampleHook {};
        unsafe {
            register_hook(&mut HOOK);
        }
        Spi::run("with surprise as (SELECT 3.141 as x) select 1 = 2, x from surprise;");
    }
    #[pg_test(error = "Found a bad approximation for pi!")]
    fn in_having_clause() {
        static mut HOOK: ExampleHook = ExampleHook {};
        unsafe {
            register_hook(&mut HOOK);
        }
        Spi::run("select count(num) as x FROM generate_series(1, 6) num group by num % 3 having count(*) = 3.141;");
    }
}
