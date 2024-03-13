use crate::proptest::PgTestRunner;
use core::ffi;
use paste::paste;
use pgrx::prelude::*;
use proptest::prelude::*;

macro_rules! pg_proptest_datetime_roundtrip_tests {
    ($datetime_ty:ty, $nop_fn:ident, $prop_strat:expr) => {

paste! {

#[pg_test]
pub fn [<$datetime_ty:lower _spi_roundtrip>] () {
    // 2. Constructing the Postgres-adapted test runner
    let mut proptest = PgTestRunner::default();
    // 3. A strategy to create and refining values, which is a somewhat aggrandized function.
    //    In some cases it actually can be replaced directly by a closure, or, in this case,
    //    it involves using a closure to `prop_map` an existing Strategy for producing
    //    "any kind of i32" into "any kind of in-range value for a Date".
    let strat = $prop_strat;
    proptest
        .run(&strat, |datetime| {
            let query = concat!("SELECT ", stringify!($nop_fn), "($1)");
            let builtin_oid = PgOid::BuiltIn(pg_sys::BuiltinOid::from_u32(<$datetime_ty as IntoDatum>::type_oid().as_u32()).unwrap());
            let args = vec![(builtin_oid, datetime.into_datum())];
            let spi_ret: $datetime_ty = Spi::get_one_with_args(query, args).unwrap().unwrap();
            // 5. A condition on which the test is accepted or rejected:
            //    this is easily done via `prop_assert!` and its friends,
            //    which just early-returns a TestCaseError on failure
            prop_assert_eq!(datetime, spi_ret);
            Ok(())
        })
        .unwrap();
}

#[pg_test]
pub fn [<$datetime_ty:lower _literal_spi_roundtrip>] () {
    let mut proptest = PgTestRunner::default();
    let strat = $prop_strat;
    proptest
        .run(&strat, |datetime| {
            let datum = datetime.into_datum();
            let datetime_cstr: &ffi::CStr =
                unsafe { pgrx::direct_function_call(pg_sys::date_out, &[datum]).unwrap() };
            let datetime_text = datetime_cstr.to_str().unwrap().to_owned();
            let spi_select_command = format!(concat!("SELECT ", stringify!($nop_fn), "('{}')"), datetime_text);
            let spi_ret: Option<$datetime_ty> = Spi::get_one(&spi_select_command).unwrap();
            prop_assert_eq!(datetime, spi_ret.unwrap());
            Ok(())
        })
        .unwrap();
}

}

    }
}

macro_rules! pg_proptest_datetime_types {
    ($($datetime_ty:ty = $prop_strat:expr;)*) => {

        paste! {
    $(

#[pg_extern]
pub fn [<nop_ $datetime_ty:lower>](datetime: $datetime_ty) -> $datetime_ty {
    datetime
}

    )*

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use super::*;
    #[allow(unused)] // I can never tell when this is actually needed.
    use crate as pgrx_tests;

    $(
    pg_proptest_datetime_roundtrip_tests! {
            $datetime_ty, [<nop_ $datetime_ty:lower>], $prop_strat
    }
    )*
}

        }

    }
}

pg_proptest_datetime_types! {
    Date = prop::num::i32::ANY.prop_map(Date::saturating_from_raw);
}

// #[cfg(any(test, feature = "pg_test"))]
// #[pgrx::pg_schema]
// mod tests {
//     use super::*;
//     #[allow(unused)] // I can never tell when this is actually needed.
//     use crate as pgrx_tests;

//     // Property tests consist of 1:
//     /// Hypothesis: We can pass random dates directly into Postgres functions and get them back.
//     #[pg_test]
//     pub fn date_spi_roundtrip() {
//         // 2. Constructing the Postgres-adapted test runner
//         let mut proptest = PgTestRunner::default();
//         // 3. A strategy to create and refining values, which is a somewhat aggrandized function.
//         //    In some cases it actually can be replaced directly by a closure, or, in this case,
//         //    it involves using a closure to `prop_map` an existing Strategy for producing
//         //    "any kind of i32" into "any kind of in-range value for a Date".
//         let strat = prop::num::i32::ANY.prop_map(Date::saturating_from_raw);
//         // 4. The runner invocation
//         proptest
//             .run(&strat, |date| {
//                 let spi_ret: Date = Spi::get_one_with_args(
//                     "SELECT nop_date($1)",
//                     vec![(PgBuiltInOids::DATEOID.into(), date.into_datum())],
//                 )
//                 .unwrap()
//                 .unwrap();

//                 // 5. A condition on which the test is accepted or rejected:
//                 //    this is easily done via `prop_assert!` and its friends,
//                 //    which just early-returns a TestCaseError on failure
//                 prop_assert_eq!(date, spi_ret);
//                 Ok(())
//             })
//             .unwrap();
//     }

//     // Proptest's "trophy case" for pgrx includes:
//     // Demonstrating that existing infallible functions can have fallible results when
//     // their code is actually put in contact with the database, as this test, when
//     // initially written, used a simpler `prop_map_into` strategy.
//     // This revealed that random i32s cause errors when Postgres uses `date_in` on the
//     // date literal string derived from using `date_out`.
//     /// Hypothesis: We can ask Postgres to accept a Date from an in-range i32, print its value,
//     /// then get the same Date back after passing it through SPI as a date literal
//     #[pg_test]
//     pub fn date_literal_spi_roundtrip() {
//         let mut proptest = PgTestRunner::default();
//         let strat = prop::num::i32::ANY.prop_map(Date::saturating_from_raw);
//         proptest
//             .run(&strat, |date| {
//                 let datum = date.into_datum();
//                 let date_cstr: &ffi::CStr =
//                     unsafe { pgrx::direct_function_call(pg_sys::date_out, &[datum]).unwrap() };
//                 let date_text = date_cstr.to_str().unwrap().to_owned();
//                 let spi_select_command = format!("SELECT nop_date('{}')", date_text);
//                 let spi_ret: Option<Date> = Spi::get_one(&spi_select_command).unwrap();
//                 prop_assert_eq!(date, spi_ret.unwrap());
//                 Ok(())
//             })
//             .unwrap();
//     }

//     /// Hypothesis: We can pass random times directly into Postgres functions and get them back.
//     // absolutely not, apparently
//     #[pg_test]
//     pub fn time_spi_roundtrip() {
//         let mut proptest = PgTestRunner::default();
//         let strat = prop::num::i64::ANY.prop_map(Time::from);
//         proptest
//             .run(&strat, |time| {
//                 let spi_ret: Time = Spi::get_one_with_args(
//                     "SELECT nop_time($1)",
//                     vec![(PgBuiltInOids::TIMEOID.into(), time.into_datum())],
//                 )
//                 .unwrap()
//                 .unwrap();

//                 prop_assert_eq!(time, spi_ret);
//                 Ok(())
//             })
//             .unwrap();
//     }

//     /// Hypothesis: We can ask Postgres to accept a Time from an in-range i32, print its value,
//     /// then get the same Time back after passing it through SPI as a time literal
//     // absolutely not
//     #[pg_test]
//     pub fn time_literal_spi_roundtrip() {
//         let mut proptest = PgTestRunner::default();
//         let strat = prop::num::i64::ANY.prop_map(Time::from);
//         proptest
//             .run(&strat, |time| {
//                 let datum = time.into_datum();
//                 let time_cstr: &ffi::CStr =
//                     unsafe { pgrx::direct_function_call(pg_sys::time_out, &[datum]).unwrap() };
//                 let time_text = time_cstr.to_str().unwrap().to_owned();
//                 let spi_select_command = format!("SELECT nop_time('{}')", time_text);
//                 let spi_ret: Option<Time> = Spi::get_one(&spi_select_command).unwrap();
//                 prop_assert_eq!(time, spi_ret.unwrap());
//                 Ok(())
//             })
//             .unwrap();
//     }
// }
