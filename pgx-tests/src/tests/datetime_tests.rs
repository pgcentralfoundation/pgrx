// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx::*;
use std::convert::TryFrom;
use time::UtcOffset;

#[pg_extern]
fn accept_date(d: Date) -> Date {
    d
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
fn accept_timestamp(t: Timestamp) -> Timestamp {
    t
}

#[pg_extern]
fn accept_timestamp_with_time_zone(t: TimestampWithTimeZone) -> TimestampWithTimeZone {
    t
}

#[pg_extern]
fn return_3pm_mountain_time() -> TimestampWithTimeZone {
    let three_pm = TimestampWithTimeZone::new(
        time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(2020, time::Month::try_from(2).unwrap(), 19).unwrap(),
            time::Time::from_hms(15, 0, 0).unwrap(),
        ),
        UtcOffset::from_hms(-7, 0, 0).unwrap(),
    );

    assert_eq!(7, three_pm.offset().whole_hours());

    three_pm
}

#[pg_extern]
fn timestamptz_to_i64(tstz: pg_sys::TimestampTz) -> i64 {
    tstz
}

#[cfg(test)]
#[pgx::pg_schema]
mod serialization_tests {
    use pgx::*;
    use serde_json::*;
    use std::convert::TryFrom;
    use time::{PrimitiveDateTime, UtcOffset};

    #[test]
    fn test_date_serialization() {
        let date = Date::new(
            time::Date::from_calendar_date(2020, time::Month::try_from(4).unwrap(), 07).unwrap(),
        );
        let json = json!({ "date test": date });

        assert_eq!(json!({"date test":"2020-04-07"}), json);
    }

    #[test]
    fn test_time_serialization() {
        let time = Time::new(time::Time::from_hms(0, 0, 0).unwrap());
        let json = json!({ "time test": time });

        assert_eq!(json!({"time test":"00:00:00"}), json);
    }
    #[test]
    fn test_time_with_timezone_serialization() {
        let time_with_timezone = TimeWithTimeZone::new(
            time::Time::from_hms(12, 23, 34).unwrap(),
            time::UtcOffset::from_hms(2, 0, 0).unwrap(),
        );
        let json = json!({ "time W/ Zone test": time_with_timezone });

        // we automatically converted to UTC upon construction in ::new()
        assert_eq!(10, time_with_timezone.hour());

        // b/c we always want our times output in UTC
        assert_eq!(json!({"time W/ Zone test":"10:23:34-00"}), json);
    }

    #[test]
    fn test_timestamp_serialization() {
        let time_stamp = Timestamp::new(PrimitiveDateTime::new(
            time::Date::from_calendar_date(2020, time::Month::try_from(1).unwrap(), 1).unwrap(),
            time::Time::from_hms(12, 34, 54).unwrap(),
        ));
        let json = json!({ "time stamp test": time_stamp });

        assert_eq!(json!({"time stamp test":"2020-01-01T12:34:54-00"}), json);
    }
    #[test]
    fn test_timestamp_with_timezone_serialization() {
        let time_stamp_with_timezone = TimestampWithTimeZone::new(
            PrimitiveDateTime::new(
                time::Date::from_calendar_date(2022, time::Month::try_from(2).unwrap(), 2).unwrap(),
                time::Time::from_hms(16, 57, 11).unwrap(),
            ),
            UtcOffset::parse(
                "+0200",
                &time::format_description::parse("[offset_hour][offset_minute]").unwrap(),
            )
            .unwrap(),
        );

        let json = json!({ "time stamp with timezone test": time_stamp_with_timezone });

        // b/c we shift back to UTC during construction in ::new()
        assert_eq!(14, time_stamp_with_timezone.hour());

        // but we serialize timestamps at UTC
        assert_eq!(
            json!({"time stamp with timezone test":"2022-02-02T14:57:11-00"}),
            json
        );
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    use std::time::Duration;

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
    fn test_accept_date_random() {
        let result =
            Spi::get_one::<bool>("SELECT accept_date('1823-03-28'::date) = '1823-03-28'::date;")
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
    fn test_accept_time_with_time_zone_now() {
        let result = Spi::get_one::<bool>(
            "SELECT accept_time_with_time_zone(now()::time with time zone at time zone 'America/Denver') = now()::time with time zone at time zone 'utc';",
        )
        .expect("failed to get SPI result");
        assert!(result)
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

        assert_eq!(22, result.hour());
    }

    #[pg_test]
    fn test_is_timestamp_with_time_zone_utc() {
        let ts = Spi::get_one::<TimestampWithTimeZone>(
            "SELECT '2020-02-18 14:08 -07'::timestamp with time zone",
        )
        .expect("failed to get SPI result");

        assert_eq!(ts.hour(), 21);
    }

    #[pg_test]
    fn test_is_timestamp_utc() {
        let ts = Spi::get_one::<Timestamp>("SELECT '2020-02-18 14:08'::timestamp")
            .expect("failed to get SPI result");

        assert_eq!(ts.hour(), 14);
    }

    #[pg_test]
    fn test_timestamptz() {
        let result = Spi::get_one::<i64>(
            "SELECT timestamptz_to_i64('2000-01-01 00:01:00.0000000+00'::timestamptz)",
        )
        .expect("failed to get SPI result");

        assert_eq!(result, Duration::from_secs(60).as_micros() as i64);
    }
}
