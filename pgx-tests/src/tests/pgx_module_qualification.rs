#![forbid(unsafe_op_in_unsafe_fn)]
//! If this test compiles, and assuming it includes a usage of all the things pgx generates code for
//! then we know that pgx has ::fully::qualified::all::its::pgx::symbols

#[cfg(feature = "pg_test")]
mod pgx_modqual_tests {
    // this is the trick (thanks @thomcc for the idea!).  We redefine the pgx module as (essentially)
    // empty.  This way, if pgx emits any code that uses `pgx::path::to::Thing` instead of `::pgx::path::to::Thing`
    // we'll fail to compile
    mod pgx {
        // the code #[pg_guard] emits isn't qualified with `::pgx`
        // This is intentional by pgx, so need to account for it
        pub mod pg_sys {
            pub mod submodules {
                pub mod panic {
                    pub use ::pgx::pg_sys::panic::pgx_extern_c_guard;
                }
            }
        }
    }

    use pgx_macros::{
        opname, pg_aggregate, pg_extern, pg_guard, pg_operator, pg_schema, pg_trigger, pgx,
        PostgresEq, PostgresHash, PostgresOrd, PostgresType,
    };

    ::pgx::extension_sql!("SELECT 1;", name = "pgx_module_qualification_test");

    #[derive(
        Eq,
        Ord,
        PartialOrd,
        PartialEq,
        Hash,
        PostgresType,
        PostgresOrd,
        PostgresEq,
        PostgresHash,
        serde::Serialize,
        serde::Deserialize,
        Copy,
        Clone,
        Debug
    )]
    pub struct PgxModuleQualificationTest {
        v: i32,
    }

    #[pg_extern]
    fn foo() {}

    #[pg_extern]
    fn foo_i32() -> i32 {
        42
    }

    #[pg_extern]
    fn foo_two_args_return_void(_a: i32, _b: String) {}

    #[pg_extern]
    fn foo_return_null() -> Option<i32> {
        None
    }

    #[pg_extern]
    fn foo_composite() -> ::pgx::composite_type!("Foo") {
        todo!()
    }

    #[pg_extern]
    fn foo_default_arg(_a: ::pgx::default!(i64, 1)) {
        todo!()
    }

    #[pg_extern]
    fn foo_table(
    ) -> ::pgx::iter::TableIterator<'static, (::pgx::name!(index, i32), ::pgx::name!(value, f64))>
    {
        todo!()
    }

    #[pg_extern]
    fn foo_set() -> ::pgx::iter::SetOfIterator<'static, i64> {
        todo!()
    }

    #[pg_extern]
    fn foo_result_set(
    ) -> std::result::Result<::pgx::iter::SetOfIterator<'static, i64>, Box<dyn std::error::Error>>
    {
        todo!()
    }

    #[pg_extern]
    fn foo_result_option_set(
    ) -> std::result::Result<::pgx::iter::SetOfIterator<'static, i64>, Box<dyn std::error::Error>>
    {
        todo!()
    }

    #[pg_operator]
    #[opname(=<>=)]
    fn fake_foo_operator(_a: PgxModuleQualificationTest, _b: PgxModuleQualificationTest) -> bool {
        todo!()
    }

    #[allow(dead_code)]
    #[pg_guard]
    extern "C" fn extern_foo_func() {}

    #[pg_schema]
    mod foo_schema {}

    #[pg_trigger]
    fn foo_trigger(
        _trigger: &::pgx::PgTrigger,
    ) -> Result<
        ::pgx::heap_tuple::PgHeapTuple<'_, ::pgx::pgbox::AllocatedByPostgres>,
        Box<dyn std::any::Any>,
    > {
        todo!()
    }

    #[pg_extern]
    fn err() {
        ::pgx::error!("HERE")
    }

    #[pg_aggregate]
    impl ::pgx::Aggregate for PgxModuleQualificationTest {
        type State = ::pgx::PgVarlena<Self>;
        type Args = ::pgx::name!(value, Option<i32>);
        const NAME: &'static str = "PgxModuleQualificationTestAgg";

        const INITIAL_CONDITION: Option<&'static str> = Some(r#"{"v": 0}"#);

        #[pgx(parallel_safe, immutable)]
        fn state(
            _current: Self::State,
            _arg: Self::Args,
            _fcinfo: ::pgx::pg_sys::FunctionCallInfo,
        ) -> Self::State {
            todo!()
        }

        // You can skip all these:
        type Finalize = i32;

        fn finalize(
            _current: Self::State,
            _direct_args: Self::OrderedSetArgs,
            _fcinfo: ::pgx::pg_sys::FunctionCallInfo,
        ) -> Self::Finalize {
            todo!()
        }
    }
}
