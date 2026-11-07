pub mod support;
pub(crate) use support as datetime_support;
pub use support::*;

mod date;
mod interval;
mod time;
mod time_stamp;
mod time_stamp_with_timezone;
mod time_with_timezone;

pub use date::*;
pub use interval::*;
pub use time::*;
pub use time_stamp::*;
pub use time_stamp_with_timezone::*;
pub use time_with_timezone::*;

/// The finite date and timestamp ranges come from the generated `IS_VALID_DATE!` and
/// `IS_VALID_TIMESTAMP!`. These tests pin their edges to the values in PostgreSQL's
/// `datatype/timestamp.h`, which are the same for PG15-19.
#[cfg(test)]
mod range_tests {
    use super::*;

    /// `DATETIME_MIN_JULIAN - POSTGRES_EPOCH_JDATE`, the first valid date.
    const FIRST_DATE: i32 = -2_451_545;
    /// `DATE_END_JULIAN - POSTGRES_EPOCH_JDATE`, the first date past the valid range.
    const END_DATE: i32 = 2_145_031_949;
    /// `MIN_TIMESTAMP`, the first valid timestamp.
    const MIN_TIMESTAMP: i64 = -211_813_488_000_000_000;
    /// `END_TIMESTAMP`, the first timestamp past the valid range.
    const END_TIMESTAMP: i64 = 9_223_371_331_200_000_000;

    #[test]
    fn date_range_edges() {
        let date = |raw| Date::try_from(raw).map(Date::into_inner);
        assert_eq!(date(FIRST_DATE - 1), Err(DateTimeConversionError::OutOfRange));
        assert_eq!(date(FIRST_DATE), Ok(FIRST_DATE));
        assert_eq!(date(END_DATE - 1), Ok(END_DATE - 1));
        assert_eq!(date(END_DATE), Err(DateTimeConversionError::OutOfRange));

        let neg_infinity = Date::try_from(i32::MIN).unwrap();
        let infinity = Date::try_from(i32::MAX).unwrap();
        assert!(neg_infinity.is_neg_infinity() && !neg_infinity.is_finite());
        assert!(infinity.is_infinity() && !infinity.is_finite());
        assert!(Date::try_from(FIRST_DATE).unwrap().is_finite());
        assert!(Date::try_from(END_DATE - 1).unwrap().is_finite());

        assert!(Date::saturating_from_raw(FIRST_DATE - 1).is_neg_infinity());
        assert!(Date::saturating_from_raw(END_DATE).is_infinity());
        assert_eq!(Date::saturating_from_raw(FIRST_DATE).into_inner(), FIRST_DATE);
        assert_eq!(Date::saturating_from_raw(END_DATE - 1).into_inner(), END_DATE - 1);
    }

    #[test]
    fn timestamp_range_edges() {
        let ts = |raw| Timestamp::try_from(raw).map(Timestamp::into_inner);
        assert_eq!(ts(MIN_TIMESTAMP - 1), Err(MIN_TIMESTAMP - 1));
        assert_eq!(ts(MIN_TIMESTAMP), Ok(MIN_TIMESTAMP));
        assert_eq!(ts(END_TIMESTAMP - 1), Ok(END_TIMESTAMP - 1));
        assert_eq!(ts(END_TIMESTAMP), Err(END_TIMESTAMP));

        let neg_infinity = Timestamp::try_from(i64::MIN).unwrap();
        let infinity = Timestamp::try_from(i64::MAX).unwrap();
        assert!(neg_infinity.is_neg_infinity() && !neg_infinity.is_finite());
        assert!(infinity.is_infinity() && !infinity.is_finite());
        assert!(Timestamp::try_from(MIN_TIMESTAMP).unwrap().is_finite());
        assert!(Timestamp::try_from(END_TIMESTAMP - 1).unwrap().is_finite());

        assert!(Timestamp::saturating_from_raw(MIN_TIMESTAMP - 1).is_neg_infinity());
        assert!(Timestamp::saturating_from_raw(END_TIMESTAMP).is_infinity());
        assert_eq!(Timestamp::saturating_from_raw(MIN_TIMESTAMP).into_inner(), MIN_TIMESTAMP);
        assert_eq!(
            Timestamp::saturating_from_raw(END_TIMESTAMP - 1).into_inner(),
            END_TIMESTAMP - 1
        );
    }

    #[test]
    fn timestamp_with_time_zone_range_edges() {
        let tstz =
            |raw| TimestampWithTimeZone::try_from(raw).map(TimestampWithTimeZone::into_inner);
        assert!(matches!(tstz(MIN_TIMESTAMP - 1), Err(FromTimeError::MicrosOutOfBounds)));
        assert!(matches!(tstz(MIN_TIMESTAMP), Ok(MIN_TIMESTAMP)));
        assert!(matches!(tstz(END_TIMESTAMP - 1), Ok(v) if v == END_TIMESTAMP - 1));
        assert!(matches!(tstz(END_TIMESTAMP), Err(FromTimeError::MicrosOutOfBounds)));

        let neg_infinity = TimestampWithTimeZone::try_from(i64::MIN).unwrap();
        let infinity = TimestampWithTimeZone::try_from(i64::MAX).unwrap();
        assert!(neg_infinity.is_neg_infinity() && !neg_infinity.is_finite());
        assert!(infinity.is_infinity() && !infinity.is_finite());
        assert!(TimestampWithTimeZone::try_from(MIN_TIMESTAMP).unwrap().is_finite());
    }
}
