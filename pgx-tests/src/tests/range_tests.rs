/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgx::prelude::*;

#[pg_extern]
fn accept_range_i32(range: Range<i32>) -> Range<i32> {
    range
}

#[pg_extern]
fn accept_range_i64(range: Range<i64>) -> Range<i64> {
    range
}

#[pg_extern]
fn accept_range_numeric(range: Range<AnyNumeric>) -> Range<AnyNumeric> {
    range
}

#[pg_extern]
fn accept_range_date(range: Range<Date>) -> Range<Date> {
    range
}

#[pg_extern]
fn accept_range_ts(range: Range<Timestamp>) -> Range<Timestamp> {
    range
}

#[pg_extern]
fn accept_range_tstz(range: Range<TimestampWithTimeZone>) -> Range<TimestampWithTimeZone> {
    range
}

fn range_round_trip_values<T>(range: Range<T>) -> Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    let range_data: RangeData<T> = range.into();
    if range_data.is_empty {
        RangeData::<T>::empty_range_data().into()
    } else {
        let range_data = RangeData::<T>::from_range_values(
            range_data.lower_val(),
            range_data.upper_val(),
            range_data.lower.inclusive,
            range_data.upper.inclusive,
        );
        range_data.into()
    }
}

fn range_round_trip_bounds<T>(range: Range<T>) -> Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    let range_data: RangeData<T> = range.into();
    if range_data.is_empty {
        RangeData::<T>::empty_range_data().into()
    } else {
        let mut lower_bound = pg_sys::RangeBound::default();
        lower_bound.lower = true;
        lower_bound.inclusive = range_data.lower.inclusive;
        lower_bound.infinite = range_data.lower.infinite;
        lower_bound.val = range_data.lower.val.clone();

        let mut upper_bound = pg_sys::RangeBound::default();
        upper_bound.lower = false;
        upper_bound.inclusive = range_data.upper.inclusive;
        upper_bound.infinite = range_data.upper.infinite;
        upper_bound.val = range_data.upper.val.clone();

        let range_data = RangeData::<T>::from_range_bounds(lower_bound, upper_bound);

        range_data.into()
    }
}

#[pg_extern]
fn range_i32_rt_values(range: Range<i32>) -> Range<i32> {
    range_round_trip_values(range)
}

#[pg_extern]
fn range_i32_rt_bounds(range: Range<i32>) -> Range<i32> {
    range_round_trip_bounds(range)
}

#[pg_extern]
fn range_i64_rt_values(range: Range<i64>) -> Range<i64> {
    range_round_trip_values(range)
}

#[pg_extern]
fn range_i64_rt_bounds(range: Range<i64>) -> Range<i64> {
    range_round_trip_bounds(range)
}

#[pg_extern]
fn range_num_rt_values(range: Range<AnyNumeric>) -> Range<AnyNumeric> {
    range_round_trip_values(range)
}

#[pg_extern]
fn range_num_rt_bounds(range: Range<AnyNumeric>) -> Range<AnyNumeric> {
    range_round_trip_bounds(range)
}

#[pg_extern]
fn range_date_rt_values(range: Range<Date>) -> Range<Date> {
    range_round_trip_values(range)
}

#[pg_extern]
fn range_date_rt_bounds(range: Range<Date>) -> Range<Date> {
    range_round_trip_bounds(range)
}

#[pg_extern]
fn range_ts_rt_values(range: Range<Timestamp>) -> Range<Timestamp> {
    range_round_trip_values(range)
}

#[pg_extern]
fn range_ts_rt_bounds(range: Range<Timestamp>) -> Range<Timestamp> {
    range_round_trip_bounds(range)
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::prelude::*;

    #[pg_test]
    fn test_accept_range_i32() {
        let matched =
            Spi::get_one::<bool>("SELECT accept_range_i32(int4range'[1,10)') = int4range'[1,10)'")
                .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_accept_range_i64() {
        let matched =
            Spi::get_one::<bool>("SELECT accept_range_i64(int8range'[1,10)') = int8range'[1,10)'")
                .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_accept_range_numeric() {
        let matched = Spi::get_one::<bool>(
            "SELECT accept_range_numeric(numrange'[1.0,10.0)') = numrange'[1.0,10.0)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_accept_range_date() {
        let matched =
            Spi::get_one::<bool>("SELECT accept_range_date(daterange'[2000-01-01,2022-01-01)') = daterange'[2000-01-01,2022-01-01)'")
            .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_accept_range_ts() {
        let matched =
            Spi::get_one::<bool>("SELECT accept_range_ts(tsrange'[2000-01-01T:12:34:56,2022-01-01T:12:34:56)') = tsrange'[2000-01-01T:12:34:56,2022-01-01T:12:34:56)'")
            .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_accept_range_tstz() {
        let matched =
            Spi::get_one::<bool>("SELECT accept_range_tstz(tstzrange'[2000-01-01T:12:34:56+00,2022-01-01T:12:34:56+00)') = tstzrange'[2000-01-01T:12:34:56+00,2022-01-01T:12:34:56+00)'")
            .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_i32_rt_values() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_i32_rt_values(int4range'[1,10)') = int4range'[1,10)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_i32_rt_bounds() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_i32_rt_bounds(int4range'[1,10)') = int4range'[1,10)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_i64_rt_values() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_i64_rt_values(int8range'[1,10)') = int8range'[1,10)'",
        )
        .unwrap();
        assert!(matched);
    }

    #[pg_test]
    fn test_range_i64_rt_bounds() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_i64_rt_bounds(int8range'[1,10)') = int8range'[1,10)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_num_rt_values() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_num_rt_values(numrange'[1.0,10.0)') = numrange'[1.0,10.0)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_num_rt_bounds() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_num_rt_bounds(numrange'[1.0,10.0)') = numrange'[1.0,10.0)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_values() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_values(daterange'[2000-01-01,2022-01-01)') = daterange'[2000-01-01,2022-01-01)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_bounds() {
        let matched =
            Spi::get_one::<bool>("SELECT range_date_rt_bounds(daterange'[2000-01-01,2022-01-01)') = daterange'[2000-01-01,2022-01-01)'")
            .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_ts_rt_values() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_ts_rt_values(tsrange'[2000-01-01T12:34:56,2022-01-01T12:34:56)') = tsrange'[2000-01-01T12:34:56,2022-01-01T12:34:56)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_ts_rt_bounds() {
        let matched =
            Spi::get_one::<bool>("SELECT range_ts_rt_bounds(tsrange'[2000-01-01T12:34:56,2022-01-01T12:34:56)') = tsrange'[2000-01-01T12:34:56,2022-01-01T12:34:56)'")
            .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_values_empty() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_values(daterange'[2000-01-01,2000-01-01)') = daterange'empty'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_bounds_empty() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_bounds(daterange'[2000-01-01,2000-01-01)') = daterange'empty'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_values_neg_inf() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_values(daterange'(-infinity,2000-01-01)') = daterange'(-infinity,2000-01-01)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_bounds_neg_inf() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_bounds(daterange'(-infinity,2000-01-01)') = daterange'(-infinity,2000-01-01)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_values_inf() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_values(daterange'(2000-01-01,infinity)') = daterange'(2000-01-01,infinity)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_bounds_inf() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_bounds(daterange'(2000-01-01,infinity)') = daterange'(2000-01-01,infinity)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_values_neg_inf_inf() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_values(daterange'(-infinity,infinity)') = daterange'(-infinity,infinity)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_bounds_neg_inf_inf() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_bounds(daterange'(-infinity,infinity)') = daterange'(-infinity,infinity)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_values_neg_inf_val() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_values(daterange'(,2000-01-01)') = daterange'(,2000-01-01)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_bounds_neg_inf_val() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_bounds(daterange'(,2000-01-01)') = daterange'(,2000-01-01)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_values_val_inf() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_values(daterange'(2000-01-01,)') = daterange'(2000-01-01,)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_bounds_val_inf() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_date_rt_bounds(daterange'(2000-01-01,)') = daterange'(2000-01-01,)'",
        )
        .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_values_full() {
        let matched =
            Spi::get_one::<bool>("SELECT range_date_rt_values(daterange'(,)') = daterange'(,)'")
                .expect("failed to get SPI result");
        assert!(matched);
    }

    #[pg_test]
    fn test_range_date_rt_bounds_full() {
        let matched =
            Spi::get_one::<bool>("SELECT range_date_rt_bounds(daterange'(,)') = daterange'(,)'")
                .expect("failed to get SPI result");
        assert!(matched);
    }
}
