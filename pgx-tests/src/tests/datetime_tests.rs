/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgx::prelude::*;
use std::convert::TryFrom;
use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

#[pg_extern]
fn accept_date(d: Date) -> Date {
    d
}

#[pg_extern]
fn accept_date_round_trip(d: Date) -> Date {
    match TryInto::<time::Date>::try_into(d) {
        Ok(date) => date.into(),
        Err(pg_epoch_days) => Date::from_pg_epoch_days(pg_epoch_days),
    }
}

#[pg_extern]
fn accept_time(t: Time) -> Time {
    t
}

#[pg_extern]
fn accept_time_with_time_zone(t: TimeWithTimeZone) -> TimeWithTimeZone {
    t
}

#[pg_extern]
fn convert_timetz_to_time(t: TimeWithTimeZone) -> Time {
    t.to_utc().into()
}

#[pg_extern]
fn accept_timestamp(t: Timestamp) -> Timestamp {
    t
}

#[pg_extern]
fn accept_timestamp_with_time_zone(t: TimestampWithTimeZone) -> TimestampWithTimeZone {
    t
}

#[pg_extern]
fn accept_timestamp_with_time_zone_offset_round_trip(
    t: TimestampWithTimeZone,
) -> Option<TimestampWithTimeZone> {
    match TryInto::<OffsetDateTime>::try_into(t) {
        Ok(offset) => Some(offset.try_into().unwrap()),
        Err(_) => None,
    }
}

#[pg_extern]
fn accept_timestamp_with_time_zone_datetime_round_trip(
    t: TimestampWithTimeZone,
) -> Option<TimestampWithTimeZone> {
    match TryInto::<PrimitiveDateTime>::try_into(t) {
        Ok(datetime) => Some(datetime.try_into().unwrap()),
        Err(_) => None,
    }
}

#[pg_extern]
fn return_3pm_mountain_time() -> TimestampWithTimeZone {
    let datetime = PrimitiveDateTime::new(
        time::Date::from_calendar_date(2020, time::Month::try_from(2).unwrap(), 19).unwrap(),
        time::Time::from_hms(15, 0, 0).unwrap(),
    )
    .assume_offset(UtcOffset::from_hms(-7, 0, 0).unwrap());

    let three_pm: TimestampWithTimeZone = datetime.try_into().unwrap();

    // this conversion will revert to UTC
    let offset: time::OffsetDateTime = three_pm.try_into().unwrap();

    // 3PM mountain time is 10PM UTC
    assert_eq!(22, offset.hour());

    datetime.try_into().unwrap()
}

#[pg_extern(sql = r#"
CREATE FUNCTION "timestamptz_to_i64"(
	"tstz" timestamptz
) RETURNS bigint
STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', '@FUNCTION_NAME@';
"#)]
fn timestamptz_to_i64(tstz: pg_sys::TimestampTz) -> i64 {
    tstz
}

#[cfg(test)]
#[pgx::pg_schema]
mod date_epoch_tests {
    use pg_sys;
    use pgx::prelude::*;

    #[test]
    fn test_to_pg_epoch_days() {
        let d = time::Date::from_calendar_date(2000, time::Month::January, 2).unwrap();
        let date: Date = d.into();

        assert_eq!(date.to_pg_epoch_days(), 1);
    }

    #[test]
    fn test_to_posix_time() {
        let d = time::Date::from_calendar_date(1970, time::Month::January, 2).unwrap();
        let date: Date = d.into();

        assert_eq!(date.to_posix_time(), 86400);
    }

    #[test]
    fn test_to_julian_days() {
        let d = time::Date::from_calendar_date(2000, time::Month::January, 1).unwrap();
        let date: Date = d.into();

        assert_eq!(date.to_julian_days(), pg_sys::POSTGRES_EPOCH_JDATE as i32);
    }
}

#[cfg(test)]
#[pgx::pg_schema]
mod serialization_tests {
    use pgx::prelude::*;
    use serde_json::*;

    #[test]
    #[allow(deprecated)]
    fn test_time_with_timezone_serialization() {
        let time_with_timezone = TimeWithTimeZone::new(
            time::Time::from_hms(12, 23, 34).unwrap(),
            time::UtcOffset::from_hms(2, 0, 0).unwrap(),
        );
        let json = json!({ "time W/ Zone test": time_with_timezone });

        let (h, ..) = time_with_timezone.to_utc().to_hms_micro();
        assert_eq!(10, h);

        // b/c we always want our times output in UTC
        assert_eq!(json!({"time W/ Zone test":"10:23:34-00"}), json);
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::prelude::*;
    use serde_json::*;
    use std::time::Duration;
    use time;
    use time::PrimitiveDateTime;

    #[pg_test]
    fn test_date_serialization() {
        let date: Date =
            time::Date::from_calendar_date(2020, time::Month::try_from(4).unwrap(), 07)
                .unwrap()
                .into();

        let json = json!({ "date test": date });

        assert_eq!(json!({"date test":"2020-04-07"}), json);
    }

    #[pg_test]
    #[allow(deprecated)]
    fn test_time_serialization() {
        let time = Time::new(time::Time::from_hms(0, 0, 0).unwrap());
        let json = json!({ "time test": time });

        assert_eq!(json!({"time test":"00:00:00"}), json);
    }

    #[pg_test]
    fn test_accept_date_now() {
        let result = Spi::get_one::<bool>("SELECT accept_date(now()::date) = now()::date;")
            .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_date_yesterday() {
        let result =
            Spi::get_one::<bool>("SELECT accept_date('yesterday'::date) = 'yesterday'::date;")
                .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_date_tomorrow() {
        let result =
            Spi::get_one::<bool>("SELECT accept_date('tomorrow'::date) = 'tomorrow'::date;")
                .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_date_neg_infinity() {
        let result =
            Spi::get_one::<bool>("SELECT accept_date('-infinity'::date) = '-infinity'::date;")
                .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_date_infinity() {
        let result =
            Spi::get_one::<bool>("SELECT accept_date('infinity'::date) = 'infinity'::date;")
                .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_date_large_date() {
        let result =
            Spi::get_one::<bool>("SELECT accept_date('10001-01-01'::date) = '10001-01-01'::date;")
                .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_date_random() {
        let result =
            Spi::get_one::<bool>("SELECT accept_date('1823-03-28'::date) = '1823-03-28'::date;")
                .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_date_round_trip_large_date() {
        let result = Spi::get_one::<bool>(
            "SELECT accept_date_round_trip('10001-01-01'::date) = '10001-01-01'::date;",
        )
        .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_date_round_trip_random() {
        let result = Spi::get_one::<bool>(
            "SELECT accept_date_round_trip('1823-03-28'::date) = '1823-03-28'::date;",
        )
        .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_time_now() {
        let result = Spi::get_one::<bool>("SELECT accept_time(now()::time) = now()::time;")
            .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_convert_time_with_time_zone_now() {
        // This test used to simply compare for equality in Postgres, assert on the bool
        // however, failed `=` in Postgres doesn't say much if it fails.
        // Thus this esoteric formulation: it derives a delta if there is one.
        let result = Spi::get_one::<Time>(
            "SELECT (
                convert_timetz_to_time(now()::time with time zone at time zone 'America/Denver')
                - convert_timetz_to_time(now()::time with time zone at time zone 'utc')
                + 'allballs'::time
            );",
        )
        .expect("failed to get SPI result");

        assert_eq!(result, Time::ALLBALLS)
    }

    #[pg_test]
    fn test_accept_time_yesterday() {
        let result = Spi::get_one::<bool>(
            "SELECT accept_time('yesterday'::timestamp::time) = 'yesterday'::timestamp::time;",
        )
        .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_time_tomorrow() {
        let result = Spi::get_one::<bool>(
            "SELECT accept_time('tomorrow'::timestamp::time) = 'tomorrow'::timestamp::time;",
        )
        .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_time_random() {
        let result = Spi::get_one::<bool>(
            "SELECT accept_time('1823-03-28 7:54:03 am'::time) = '1823-03-28 7:54:03 am'::time;",
        )
        .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_timestamp() {
        let result =
            Spi::get_one::<bool>("SELECT accept_timestamp(now()::timestamp) = now()::timestamp;")
                .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_timestamp_with_time_zone() {
        let result = Spi::get_one::<bool>("SELECT accept_timestamp_with_time_zone(now()) = now();")
            .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_accept_timestamp_with_time_zone_not_utc() {
        let result = Spi::get_one::<bool>("SELECT accept_timestamp_with_time_zone('1990-01-23 03:45:00-07') = '1990-01-23 03:45:00-07'::timestamp with time zone;")
            .expect("failed to get SPI result");
        assert!(result)
    }

    #[pg_test]
    fn test_return_3pm_mountain_time() {
        let result = Spi::get_one::<TimestampWithTimeZone>("SELECT return_3pm_mountain_time();")
            .expect("failed to get SPI result");

        let offset: time::OffsetDateTime = result.try_into().unwrap();

        assert_eq!(22, offset.hour());
    }

    #[pg_test]
    fn test_is_timestamp_with_time_zone_utc() {
        let ts = Spi::get_one::<TimestampWithTimeZone>(
            "SELECT '2020-02-18 14:08 -07'::timestamp with time zone",
        )
        .expect("failed to get SPI result");

        let datetime: time::PrimitiveDateTime = ts.try_into().unwrap();

        assert_eq!(datetime.hour(), 21);
    }

    #[pg_test]
    fn test_is_timestamp_utc() {
        let ts = Spi::get_one::<Timestamp>("SELECT '2020-02-18 14:08'::timestamp")
            .expect("failed to get SPI result");
        let datetime: time::PrimitiveDateTime = ts.try_into().unwrap();
        assert_eq!(datetime.hour(), 14);
    }

    #[pg_test]
    fn test_timestamptz() {
        let result = Spi::get_one::<i64>(
            "SELECT timestamptz_to_i64('2000-01-01 00:01:00.0000000+00'::timestamptz)",
        )
        .expect("failed to get SPI result");

        assert_eq!(result, Duration::from_secs(60).as_micros() as i64);
    }

    #[pg_test]
    fn test_accept_timestamp_with_time_zone_offset_round_trip() {
        let result = Spi::get_one::<bool>(
            "SELECT accept_timestamp_with_time_zone_offset_round_trip(now()) = now()",
        )
        .expect("failed to get SPI result");

        assert!(result);
    }

    #[pg_test]
    fn test_accept_timestamp_with_time_zone_datetime_round_trip() {
        let result = Spi::get_one::<bool>(
            "SELECT accept_timestamp_with_time_zone_datetime_round_trip(now()) = now()",
        )
        .expect("failed to get SPI result");

        assert!(result);
    }

    #[pg_test]
    fn test_timestamp_with_timezone_serialization() {
        let time_stamp_with_timezone: TimestampWithTimeZone = PrimitiveDateTime::new(
            time::Date::from_calendar_date(2022, time::Month::try_from(2).unwrap(), 2).unwrap(),
            time::Time::from_hms(16, 57, 11).unwrap(),
        )
        .assume_offset(
            time::UtcOffset::parse(
                "+0200",
                &time::format_description::parse("[offset_hour][offset_minute]").unwrap(),
            )
            .unwrap(),
        )
        .try_into()
        .unwrap();

        // prevents PG's timestamp serialization from imposing the local servers time zone
        Spi::run("SET TIME ZONE 'UTC'");
        let json = json!({ "time stamp with timezone test": time_stamp_with_timezone });

        // but we serialize timestamps at UTC
        assert_eq!(json!({"time stamp with timezone test":"2022-02-02T14:57:11+00:00"}), json);
    }

    #[pg_test]
    fn test_timestamp_serialization() {
        // prevents PG's timestamp serialization from imposing the local servers time zone
        Spi::run("SET TIME ZONE 'UTC'");

        let datetime = PrimitiveDateTime::new(
            time::Date::from_calendar_date(2020, time::Month::try_from(1).unwrap(), 1).unwrap(),
            time::Time::from_hms(12, 34, 54).unwrap(),
        );
        let ts: Timestamp = datetime.try_into().unwrap();
        let json = json!({ "time stamp test": ts });

        assert_eq!(json!({"time stamp test":"2020-01-01T12:34:54"}), json);
    }

    #[pg_test]
    fn test_timestamp_with_timezone_infinity() {
        let result =
            Spi::get_one::<bool>("SELECT accept_timestamp_with_time_zone('-infinity') = TIMESTAMP WITH TIME ZONE '-infinity';")
                .expect("failed to get SPI result");
        assert!(result);

        let result =
            Spi::get_one::<bool>("SELECT accept_timestamp_with_time_zone('infinity') = TIMESTAMP WITH TIME ZONE 'infinity';")
                .expect("failed to get SPI result");
        assert!(result);

        let tstz =
            Spi::get_one::<TimestampWithTimeZone>("SELECT TIMESTAMP WITH TIME ZONE'infinity'")
                .expect("failed to get SPI result");
        assert!(tstz.is_infinity());

        let tstz =
            Spi::get_one::<TimestampWithTimeZone>("SELECT TIMESTAMP WITH TIME ZONE'-infinity'")
                .expect("failed to get SPI result");
        assert!(tstz.is_neg_infinity());
    }

    #[pg_test]
    fn test_timestamp_infinity() {
        let result =
            Spi::get_one::<bool>("SELECT accept_timestamp('-infinity') = '-infinity'::timestamp;")
                .expect("failed to get SPI result");
        assert!(result);

        let result =
            Spi::get_one::<bool>("SELECT accept_timestamp('infinity') = 'infinity'::timestamp;")
                .expect("failed to get SPI result");
        assert!(result);

        let ts = Spi::get_one::<Timestamp>("SELECT 'infinity'::timestamp")
            .expect("failed to get SPI result");
        assert!(ts.is_infinity());

        let ts = Spi::get_one::<Timestamp>("SELECT '-infinity'::timestamp")
            .expect("failed to get SPI result");
        assert!(ts.is_neg_infinity());
    }
}
