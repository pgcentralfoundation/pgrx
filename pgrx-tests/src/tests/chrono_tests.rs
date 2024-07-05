//! Tests for the `chrono` features of `cargo-pgrx`
//!
#![cfg(feature = "chrono")]

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use std::result::Result;

    use chrono::{Datelike as _, Timelike as _, Utc};

    use pgrx::pg_test;
    use pgrx::DateTimeConversionError;

    // Utility class for errors
    type DtcResult<T> = Result<T, DateTimeConversionError>;

    /// Ensure simple conversion ([`pgrx::Date`] -> [`chrono::NaiveDate`]) works
    #[pg_test]
    fn chrono_simple_date_conversion() -> DtcResult<()> {
        let original = pgrx::Date::new(1970, 1, 1)?;
        let d = chrono::NaiveDate::try_from(original)?;
        assert_eq!(d.year(), original.year(), "year matches");
        assert_eq!(d.month(), 1, "month matches");
        assert_eq!(d.day(), 1, "day matches");
        let backwards = pgrx::Date::try_from(d)?;
        assert_eq!(backwards, original);
        Ok(())
    }

    /// Ensure simple conversion ([`pgrx::Time`] -> [`chrono::NaiveTime`]) works
    #[pg_test]
    fn chrono_simple_time_conversion() -> DtcResult<()> {
        let original = pgrx::Time::new(12, 1, 59.0000001)?;
        let d = chrono::NaiveTime::try_from(original)?;
        assert_eq!(d.hour(), 12, "hours match");
        assert_eq!(d.minute(), 1, "minutes match");
        assert_eq!(d.second(), 59, "seconds match");
        assert_eq!(d.nanosecond(), 0, "nanoseconds are zero (pg only supports microseconds)");
        let backwards = pgrx::Time::try_from(d)?;
        assert_eq!(backwards, original);
        Ok(())
    }

    /// Ensure simple conversion ([`pgrx::Timestamp`] -> [`chrono::NaiveDateTime`]) works
    #[pg_test]
    fn chrono_simple_timestamp_conversion() -> DtcResult<()> {
        let original = pgrx::Timestamp::new(1970, 1, 1, 1, 1, 1.0)?;
        let d = chrono::NaiveDateTime::try_from(original)?;
        assert_eq!(d.hour(), 1, "hours match");
        assert_eq!(d.minute(), 1, "minutes match");
        assert_eq!(d.second(), 1, "seconds match");
        assert_eq!(d.nanosecond(), 0, "nanoseconds are zero (pg only supports microseconds)");
        let backwards = pgrx::Timestamp::try_from(d)?;
        assert_eq!(backwards, original, "NaiveDateTime -> Timestamp return conversion failed");
        Ok(())
    }

    /// Ensure simple conversion ([`pgrx::TimestampWithTimeZone`] -> [`chrono::DateTime<Utc>`]) works
    #[pg_test]
    fn chrono_simple_datetime_with_time_zone_conversion() -> DtcResult<()> {
        let original = pgrx::TimestampWithTimeZone::with_timezone(1970, 1, 1, 1, 1, 1.0, "utc")?;
        let d = chrono::DateTime::<Utc>::try_from(original)?;
        assert_eq!(d.hour(), 1, "hours match");
        assert_eq!(d.minute(), 1, "minutes match");
        assert_eq!(d.second(), 1, "seconds match");
        assert_eq!(d.nanosecond(), 0, "nanoseconds are zero (pg only supports microseconds)");
        let backwards = pgrx::TimestampWithTimeZone::try_from(d)?;
        assert_eq!(backwards, original);
        Ok(())
    }
}
