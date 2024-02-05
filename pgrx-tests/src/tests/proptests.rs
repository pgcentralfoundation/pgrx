use crate::proptest::PgTestRunner;
use pgrx::prelude::*;
use proptest::prelude::*;

#[pg_extern]
pub fn nop_date(date: Date) -> Date {
    date
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use super::*;
    #[allow(unused)] // I can never tell when this is actually needed.
    use crate as pgrx_tests;

    // Property tests consist of 1:
    /// Hypothesis: We can pass random dates directly into Postgres functions and get them back.
    #[pg_test]
    pub fn date_spi_roundtrip() {
        // 2. Constructing the Postgres-adapted test runner
        let mut proptest = PgTestRunner::default();
        // 3. A strategy to create and refining values, which is a somewhat aggrandized function.
        //    In some cases it actually can be replaced directly by a closure, or, in this case,
        //    it involves using a closure to `prop_map` an existing Strategy for producing
        //    "any kind of i32" into "any kind of in-range value for a Date".
        let strat = prop::num::i32::ANY.prop_map(Date::saturating_from_raw);
        // 4. The runner invocation
        proptest
            .run(&strat, |date| {
                let spi_ret: Date = Spi::get_one_with_args(
                    "SELECT nop_date($1)",
                    vec![(PgBuiltInOids::DATEOID.into(), date.into_datum())],
                )
                .unwrap()
                .unwrap();

                // 5. A condition on which the test is accepted or rejected:
                //    this is easily done via `prop_assert!` and its friends,
                //    which just early-returns a TestCaseError on failure
                prop_assert_eq!(date, spi_ret);
                Ok(())
            })
            .unwrap();
    }

    // Proptest's "trophy case" for pgrx includes:
    // Demonstrating that existing infallible functions can have fallible results when
    // their code is actually put in contact with the database, as this test, when
    // initially written, used a simpler `prop_map_into` strategy.
    // This revealed that random i32s cause errors when Postgres uses `date_in` on the
    // date literal string derived from using `date_out`.
    /// Hypothesis: We can ask Postgres to accept a Date from an in-range i32, print its value,
    /// then get the same Date back after passing it through SPI as a date literal
    #[pg_test]
    pub fn date_literal_spi_roundtrip() {
        let mut proptest = PgTestRunner::default();
        let strat = prop::num::i32::ANY.prop_map(Date::saturating_from_raw);
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
