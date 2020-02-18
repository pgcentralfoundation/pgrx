use pgx::*;

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

#[cfg(test)]
mod serialization_tests {
    use pgx::Date;
    use serde_json::*;

    #[test]
    fn test_date_serialization() {
        let date = Date::new(time::date!(2020 - 09 - 09));
        let json = json!({ "date": date });

        assert_eq!(json!({"date":"2020-09-09"}), json);
    }
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

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
            "SELECT accept_time_with_time_zone(now()::time with time zone) = now()::time with time zone;",
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
}
