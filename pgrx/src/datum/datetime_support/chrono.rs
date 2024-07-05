//! This module contains implementations and functionality that enables [`pgrx`] types (ex. [`pgrx::datum::Date`])
//! to be converted to [`chrono`] data types (ex. [`chrono::Date`])
//!
//! Note that `chrono` has no reasonable analog for the `time with timezone` (i.e. [`pgrx::TimeWithTimeZone`]), so there are no added conversions for that type outside of the ones already implemented.
#![cfg(feature = "chrono")]

use core::convert::Infallible;
use core::num::TryFromIntError;
use std::convert::TryFrom;

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};

use crate::datum::datetime_support::DateTimeConversionError;
use crate::datum::{Date, Time, Timestamp, TimestampWithTimeZone};

/// Convenience type for [`Result`]s that fail with a [`DateTimeConversionError`]
type DtcResult<T> = Result<T, DateTimeConversionError>;

impl From<TryFromIntError> for DateTimeConversionError {
    fn from(_tfie: TryFromIntError) -> Self {
        DateTimeConversionError::FieldOverflow
    }
}

impl From<Infallible> for DateTimeConversionError {
    fn from(_i: Infallible) -> Self {
        DateTimeConversionError::FieldOverflow
    }
}

impl TryFrom<Date> for NaiveDate {
    type Error = DateTimeConversionError;

    fn try_from(d: Date) -> DtcResult<NaiveDate> {
        NaiveDate::from_ymd_opt(d.year(), d.month().into(), d.day().into())
            .ok_or_else(|| DateTimeConversionError::InvalidFormat)
    }
}

impl TryFrom<NaiveDate> for Date {
    type Error = DateTimeConversionError;

    fn try_from(d: NaiveDate) -> DtcResult<Date> {
        let month = u8::try_from(d.month())?;
        let day = u8::try_from(d.day())?;
        Date::new(d.year(), month, day)
    }
}

/// Note: conversions from Postgres' `time` type [`pgrx::Time`] to [`chrono::NaiveTime`]
/// incur a loss of precision as Postgres only exposes microseconds.
impl TryFrom<Time> for NaiveTime {
    type Error = DateTimeConversionError;

    fn try_from(t: Time) -> DtcResult<NaiveTime> {
        let (hour, minute, second, microseconds) = t.to_hms_micro();
        let seconds_micro: u32 = Into::<u32>::into(second)
            .checked_mul(1_000_000)
            .ok_or(DateTimeConversionError::FieldOverflow)?;
        NaiveTime::from_hms_micro_opt(
            hour.into(),
            minute.into(),
            second.into(),
            // Since pgrx counts the fractional seconds (between 1_000_000 and 2_000_000),
            // the microseconds value will be 1_000_000 * seconds + fractional.
            //
            // - at 12:01:01 => hour=12, minute=1, second=2, microseconds=2_000_000
            // - at 12:01:59 => hour=12, minute=1, second=59, microseconds=59_000_000
            // - at 12:01:59 => hour=12, minute=1, second=59, microseconds=59_000_000
            //
            // Since chrono *does* support leap seconds (representing them as 59 seconds & >1_000_000 microseconds)
            // we can strip the microseconds in that case to zero and pretend they're not there,
            // since Postgres does not support leap seconds
            if second == 59 && microseconds > 1_000_000 { 0 } else { microseconds - seconds_micro },
        )
        .ok_or(DateTimeConversionError::FieldOverflow)
    }
}

impl TryFrom<NaiveTime> for Time {
    type Error = DateTimeConversionError;

    fn try_from(t: NaiveTime) -> DtcResult<Time> {
        let hour = u8::try_from(t.hour())?;
        let minute = u8::try_from(t.minute())?;
        Time::new(hour, minute, convert_chrono_seconds_to_pgrx(t.second(), t.nanosecond())?)
    }
}

/// Normally as seconds are represented by `f64` in pgrx, we must convert
fn convert_chrono_seconds_to_pgrx(seconds: u32, nanos: u32) -> DtcResult<f64> {
    let second_whole =
        f64::try_from(seconds).map_err(|_| DateTimeConversionError::FieldOverflow)?;
    let second_nanos = f64::try_from(nanos).map_err(|_| DateTimeConversionError::FieldOverflow)?;
    Ok(second_whole + (second_nanos / 1_000_000_000.0))
}

/// Utility function for easy `f64` to `u32` conversion
fn f64_to_u32(n: f64) -> DtcResult<u32> {
    let truncated = n.trunc();
    if truncated.is_nan()
        || truncated.is_infinite()
        || truncated < 0.0
        || truncated > u32::MAX.into()
    {
        return Err(DateTimeConversionError::FieldOverflow);
    }

    Ok(truncated as u32)
}

/// Seconds are represented by `f64` in pgrx, with a maximum of microsecond precision
fn convert_pgrx_seconds_to_chrono(orig: f64) -> DtcResult<(u32, u32, u32)> {
    let seconds = f64_to_u32(orig)?;
    let microseconds = f64_to_u32((orig * 1_000_000.0) % 1_000_000.0)?;
    let nanoseconds = f64_to_u32((orig * 1_000_000_000.0) % 1_000_000_000.0)?;
    Ok((seconds, microseconds, nanoseconds))
}

///////////////
// Timestamp //
///////////////

/// Since [`pgrx::Timestamp`]s are tied to the Postgres instance's timezone,
/// to figure out *which* timezone it's actually in, we convert to a [`pgrx::TimestampWithTimeZone`].
///
/// Once the offset is known, we can create and return a [`chrono::NaiveDateTime`]
/// with the appropriate offset
impl TryFrom<Timestamp> for NaiveDateTime {
    type Error = DateTimeConversionError;

    fn try_from(t: Timestamp) -> DtcResult<Self> {
        let twtz: TimestampWithTimeZone = t.into();
        let (seconds, _micros, _nanos) = convert_pgrx_seconds_to_chrono(twtz.second())?;
        NaiveDate::from_ymd_opt(twtz.year(), twtz.month().into(), twtz.day().into())
            .ok_or(DateTimeConversionError::FieldOverflow)?
            .and_hms_opt(twtz.hour().into(), twtz.minute().into(), seconds)
            .ok_or(DateTimeConversionError::FieldOverflow)
    }
}

impl TryFrom<NaiveDateTime> for Timestamp {
    type Error = DateTimeConversionError;

    fn try_from(ndt: NaiveDateTime) -> DtcResult<Self> {
        let utc = ndt.and_utc();
        let seconds = convert_chrono_seconds_to_pgrx(utc.second(), utc.nanosecond())?;
        let twtz = TimestampWithTimeZone::with_timezone(
            utc.year(),
            utc.month().try_into()?,
            utc.day().try_into()?,
            utc.hour().try_into()?,
            utc.minute().try_into()?,
            seconds,
            "utc",
        )?;
        Ok(twtz.to_utc())
    }
}

///////////////////////////
// TimestampWithTimeZone //
///////////////////////////

impl TryFrom<TimestampWithTimeZone> for DateTime<Utc> {
    type Error = DateTimeConversionError;

    fn try_from(twtz: TimestampWithTimeZone) -> DtcResult<Self> {
        let twtz = twtz.to_utc();
        let (seconds, _micros, _nanos) = convert_pgrx_seconds_to_chrono(twtz.second())?;
        let datetime = NaiveDate::from_ymd_opt(twtz.year(), twtz.month().into(), twtz.day().into())
            .ok_or(DateTimeConversionError::FieldOverflow)?
            .and_hms_opt(twtz.hour().into(), twtz.minute().into(), seconds)
            .ok_or(DateTimeConversionError::FieldOverflow)?;
        Ok(Self::from_naive_utc_and_offset(datetime, chrono::offset::Utc))
    }
}

impl TryFrom<DateTime<Utc>> for TimestampWithTimeZone {
    type Error = DateTimeConversionError;

    fn try_from(ndt: DateTime<Utc>) -> DtcResult<Self> {
        let seconds = convert_chrono_seconds_to_pgrx(ndt.second(), ndt.nanosecond())?;
        Ok(Self::with_timezone(
            ndt.year(),
            ndt.month().try_into()?,
            ndt.day().try_into()?,
            ndt.hour().try_into()?,
            ndt.minute().try_into()?,
            seconds,
            "utc",
        )?)
    }
}
