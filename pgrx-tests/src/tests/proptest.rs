use pg_sys::panic::CaughtError;
use pgrx::prelude::*;
use proptest::prelude::*;
use proptest::strategy::Strategy;
use proptest::test_runner::{TestCaseResult, TestError, TestRunner};
use std::panic::AssertUnwindSafe;

#[derive(Default)]
pub struct PgTestRunner(TestRunner);

impl PgTestRunner {
    pub fn run<S: Strategy>(
        &mut self,
        strategy: &S,
        test: impl Fn(S::Value) -> TestCaseResult,
    ) -> Result<(), TestError<<S as Strategy>::Value>> {
        self.0.run(strategy, |value| {
            PgTryBuilder::new(AssertUnwindSafe(|| test(value)))
                .catch_others(|err| match err {
                    CaughtError::PostgresError(err)
                    | CaughtError::ErrorReport(err)
                    | CaughtError::RustPanic { ereport: err, .. } => {
                        Err(TestCaseError::Fail(err.message().to_owned().into()))
                    }
                })
                .execute()
        })
    }
}

#[pg_extern]
pub fn nop_date(date: Date) -> Date {
    date
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use super::*;
    use crate as pgrx_tests;

    /// We can pass random dates directly into Postgres functions and get them back.
    #[pg_test]
    pub fn proptest_spi_passthrough() {
        let mut proptest = PgTestRunner::default();
        let strat = prop::num::i32::ANY.prop_map_into::<Date>();
        proptest
            .run(&strat, |date| {
                let spi_ret: Date = Spi::get_one_with_args(
                    "SELECT nop_date($1)",
                    vec![(PgBuiltInOids::DATEOID.into(), date.into_datum())],
                )
                .unwrap()
                .unwrap();
                prop_assert_eq!(date, spi_ret);
                Ok(())
            })
            .unwrap();
    }

    /// We can ask Postgres to accept any i32 as a Date, print it out, pass it in via SPI, and get back the same number
    /// Fails on:
    /// - date values between i32::MIN and -2451545
    /// - date values between i32::MAX and (2147483494 - 2451545) - 1
    #[pg_test]
    pub fn proptest_spi_text_passthrough() {
        let mut proptest = PgTestRunner::default();
        let strat = prop::num::i32::ANY.prop_map_into::<Date>();
        proptest
            .run(&strat, |date| {
                let datum = date.into_datum();
                let date_cstr: &std::ffi::CStr =
                    unsafe { pgrx::direct_function_call(pg_sys::date_out, &[datum]).unwrap() };
                let date_text = date_cstr.to_str().unwrap().to_owned();
                let spi_select_command = format!("SELECT nop_date('{}')", date_text);
                let spi_ret: Option<Date> = Spi::get_one(&spi_select_command).unwrap();
                prop_assert_eq!(date, spi_ret.unwrap());
                Ok(())
            })
            .unwrap();
    }
}

// struct TimeValueTree {}
// struct TimestampValueTree {}
// struct TimestampWithTimezoneValueTree {}

// fn create_array_sql_repr() -> ! {

// }
