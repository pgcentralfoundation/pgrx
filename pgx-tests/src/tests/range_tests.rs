use pgx::*;

#[pg_extern]
fn accept_range_i32(range: Range<i32>) -> Range<i32> {
    range
}

#[pg_extern]
fn accept_range_i64(range: Range<i64>) -> Range<i64> {
    range
}

#[pg_extern]
fn accept_range_numeric(range: Range<Numeric>) -> Range<Numeric> {
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
    let range_data_in: RangeData<T> = range.try_into().unwrap();
    if range_data_in.is_empty {
        RangeData::<T>::empty_range_data().try_into().unwrap()
    } else {
        let range_data_out = RangeData::<T>::from_range_values(
            range_data_in.lower_val(),
            range_data_in.upper_val(),
            range_data_in.lower.inclusive,
            range_data_in.upper.inclusive,
        );
        range_data_out.try_into().unwrap()
    }
}

fn range_round_trip_bounds<T>(range: Range<T>) -> Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    let range_data_in: RangeData<T> = range.try_into().unwrap();
    if range_data_in.is_empty {
        RangeData::<T>::empty_range_data().try_into().unwrap()
    } else {
        let mut lower_out = pg_sys::RangeBound::default();
        lower_out.lower = true;
        lower_out.inclusive = range_data_in.lower.inclusive;
        lower_out.infinite = range_data_in.lower.infinite;
        lower_out.val = range_data_in.lower.val.clone();

        let mut upper_out = pg_sys::RangeBound::default();
        upper_out.lower = false;
        upper_out.inclusive = range_data_in.upper.inclusive;
        upper_out.infinite = range_data_in.upper.infinite;
        upper_out.val = range_data_in.upper.val.clone();

        let range_data_out = RangeData::<T>::from_range_bounds(lower_out, upper_out);
        range_data_out.try_into().unwrap()
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
fn range_num_rt_values(range: Range<Numeric>) -> Range<Numeric> {
    range_round_trip_values(range)
}

#[pg_extern]
fn range_num_rt_bounds(range: Range<Numeric>) -> Range<Numeric> {
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

    use pgx::*;

    #[pg_test]
    fn test_accept_range_i32() {
        let matched =
            Spi::get_one::<bool>("SELECT accept_range_i32(int4range'[1,10)') = int4range'[1,10)'");
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn test_accept_range_i64() {
        let matched =
            Spi::get_one::<bool>("SELECT accept_range_i64(int8range'[1,10)') = int8range'[1,10)'");
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn test_accept_range_numeric() {
        let matched = Spi::get_one::<bool>(
            "SELECT accept_range_numeric(numrange'[1.0,10.0)') = numrange'[1.0,10.0)'",
        );
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn test_accept_range_date() {
        let matched =
            Spi::get_one::<bool>("SELECT accept_range_date(daterange'[2000-01-01,2022-01-01)') = daterange'[2000-01-01,2022-01-01)'");
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn test_accept_range_ts() {
        let matched =
            Spi::get_one::<bool>("SELECT accept_range_ts(tsrange'[2000-01-01T:12:34:56,2022-01-01T:12:34:56)') = tsrange'[2000-01-01T:12:34:56,2022-01-01T:12:34:56)'");
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn test_accept_range_tstz() {
        let matched =
            Spi::get_one::<bool>("SELECT accept_range_tstz(tstzrange'[2000-01-01T:12:34:56+00,2022-01-01T:12:34:56+00)') = tstzrange'[2000-01-01T:12:34:56+00,2022-01-01T:12:34:56+00)'");
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn tets_range_i32_rt_values() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_i32_rt_values(int4range'[1,10)') = int4range'[1,10)'",
        );
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn tets_range_i32_rt_bounds() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_i32_rt_bounds(int4range'[1,10)') = int4range'[1,10)'",
        );
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn tets_range_i64_rt_values() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_i64_rt_values(int8range'[1,10)') = int8range'[1,10)'",
        );
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn tets_range_i64_rt_bounds() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_i64_rt_bounds(int8range'[1,10)') = int8range'[1,10)'",
        );
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn tets_range_num_rt_values() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_num_rt_values(numrange'[1.0,10.0)') = numrange'[1.0,10.0)'",
        );
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn tets_range_num_rt_bounds() {
        let matched = Spi::get_one::<bool>(
            "SELECT range_num_rt_bounds(numrange'[1.0,10.0)') = numrange'[1.0,10.0)'",
        );
        assert!(matched.unwrap());
    }

    #[pg_test]
    fn tets_range_date_rt_values() {
        let matched = Spi::get_one::<&str>(
            "SELECT range_date_rt_values(daterange'[2000-01-01,2022-01-01)')::text",
        );
        assert_eq!("derp", matched.unwrap());
        // assert!(matched.unwrap());
    }

    #[pg_test]
    fn tets_range_date_rt_bounds() {
        let matched =
            Spi::get_one::<bool>("SELECT range_date_rt_bounds(daterange'[2000-01-01,2022-01-01)') = daterange'[2000-01-01,2022-01-01)'");
        assert!(matched.unwrap());
    }
}
