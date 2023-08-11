use pgrx::prelude::*;
use proptest::prelude::*;
use proptest::strategy::{ValueTree, Strategy, NewTree};
use proptest::test_runner::TestRunner;

// Hypothesis: We can ask Postgres to accept any i32 as a Date, print it out, pass it in via SPI, and get back the same number
struct DateBinarySearch(prop::num::i32::BinarySearch);

#[derive(Debug)]
struct AnyDate();

impl ValueTree for DateBinarySearch {
    type Value = Date;
    fn current(&self) -> Self::Value {
        Date::from(self.0.current())
    }

    fn simplify(&mut self) -> bool {
        self.0.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.0.complicate()
    }
}

impl Strategy for AnyDate {
    type Tree = DateBinarySearch;
    type Value = Date;

    fn new_tree(&self, _runner: &mut TestRunner) -> NewTree<Self> {
        Ok(DateBinarySearch(prop::num::i32::BinarySearch::new(i32::MAX)))
    }
}


#[pg_extern]
pub fn nop_date(date: Date) -> Date {
    eprintln!("input date: {date}");
    date
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use proptest::prelude::*;
    use pgrx::prelude::*;
    use proptest::test_runner::TestRunner;
    use crate as pgrx_tests;
    use crate::tests::proptest::AnyDate;

    #[pg_test]
    pub fn proptest_spi_passthrough() {
        let mut proptest = TestRunner::default();
        let strat = AnyDate().prop_map(|date| Date::from(date.to_pg_epoch_days().clamp(-2451545, 2147483494 - 2451545)));
        proptest.run(&strat, |date| {
            let spi_ret: Date = Spi::get_one_with_args("SELECT nop_date($1)", vec![(PgBuiltInOids::DATEOID.into(), date.into_datum())]).unwrap().unwrap();
            prop_assert_eq!(date, spi_ret);
            Ok(())
        }).unwrap();
    }

    #[pg_test]
    pub fn proptest_spi_text_passthrough() {
        let mut proptest = TestRunner::default();
        let strat = AnyDate().prop_map(|date| Date::from(date.to_pg_epoch_days().clamp(-2451545, 2147483494 - 2451545)));
        proptest.run(&strat, |date| {
            let datum = date.into_datum();
            let date_cstr: &std::ffi::CStr = unsafe { pgrx::direct_function_call(pg_sys::date_out, &[datum]).unwrap() };
            let date_text = date_cstr.to_str().unwrap().to_owned();
            let spi_select_command = format!("SELECT nop_date({});", date_text);
            pgrx::info!("date: {date_text}");
            let spi_ret: Date = Spi::get_one(&spi_select_command).unwrap().unwrap();
            prop_assert_eq!(date, spi_ret);
            Ok(())
        }).unwrap();
    }
}



// struct TimeValueTree {}
// struct TimestampValueTree {}
// struct TimestampWithTimezoneValueTree {}

// fn create_array_sql_repr() -> ! {

// }
