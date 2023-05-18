/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{
    direct_function_call, pg_sys, Date, DateTimeConversionError, DateTimeParts, FromDatum,
    HasExtractableParts, Interval, IntoDatum, Timestamp, ToIsoString,
};
use pgrx_pg_sys::errcodes::PgSqlErrorCode;
use pgrx_pg_sys::PgTryBuilder;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use std::convert::TryFrom;
use std::panic::{RefUnwindSafe, UnwindSafe};

// taken from /include/datatype/timestamp.h
const MIN_TIMESTAMP_USEC: i64 = -211_813_488_000_000_000;
const END_TIMESTAMP_USEC: i64 = 9_223_371_331_200_000_000 - 1; // dec by 1 to accommodate exclusive range match pattern

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct TimestampWithTimeZone(pub pg_sys::TimestampTz);

impl TryFrom<pg_sys::TimestampTz> for TimestampWithTimeZone {
    type Error = FromTimeError;

    fn try_from(value: pg_sys::TimestampTz) -> Result<Self, Self::Error> {
        match value {
            i64::MIN | i64::MAX | MIN_TIMESTAMP_USEC..=END_TIMESTAMP_USEC => {
                Ok(TimestampWithTimeZone(value))
            }
            _ => Err(FromTimeError::MicrosOutOfBounds),
        }
    }
}

impl TryFrom<pg_sys::Datum> for TimestampWithTimeZone {
    type Error = FromTimeError;
    fn try_from(datum: pg_sys::Datum) -> Result<Self, Self::Error> {
        (datum.value() as pg_sys::TimestampTz).try_into()
    }
}

impl From<Date> for TimestampWithTimeZone {
    fn from(value: Date) -> Self {
        unsafe { direct_function_call(pg_sys::date_timestamptz, &[value.into_datum()]).unwrap() }
    }
}

impl From<Timestamp> for TimestampWithTimeZone {
    fn from(value: Timestamp) -> Self {
        unsafe {
            direct_function_call(pg_sys::timestamp_timestamptz, &[value.into_datum()]).unwrap()
        }
    }
}

impl IntoDatum for TimestampWithTimeZone {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self.0))
    }
    fn type_oid() -> pg_sys::Oid {
        pg_sys::TIMESTAMPTZOID
    }
}

impl FromDatum for TimestampWithTimeZone {
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            Some(datum.try_into().expect("Error converting timestamp with time zone datum"))
        }
    }
}

impl TimestampWithTimeZone {
    const NEG_INFINITY: pg_sys::TimestampTz = pg_sys::TimestampTz::MIN;
    const INFINITY: pg_sys::TimestampTz = pg_sys::TimestampTz::MAX;

    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f64,
    ) -> Result<Self, DateTimeConversionError> {
        let month: i32 = month as _;
        let day: i32 = day as _;
        let hour: i32 = hour as _;
        let minute: i32 = minute as _;

        PgTryBuilder::new(|| unsafe {
            Ok(direct_function_call(
                pg_sys::make_timestamptz,
                &[
                    year.into_datum(),
                    month.into_datum(),
                    day.into_datum(),
                    hour.into_datum(),
                    minute.into_datum(),
                    second.into_datum(),
                ],
            )
            .unwrap())
        })
        .catch_when(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW, |_| {
            Err(DateTimeConversionError::FieldOverflow)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT, |_| {
            Err(DateTimeConversionError::InvalidFormat)
        })
        .execute()
    }

    pub fn new_unchecked(
        year: isize,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f64,
    ) -> Self {
        let year: i32 = year as _;
        let month: i32 = month as _;
        let day: i32 = day as _;
        let hour: i32 = hour as _;
        let minute: i32 = minute as _;

        unsafe {
            direct_function_call(
                pg_sys::make_timestamptz,
                &[
                    year.into_datum(),
                    month.into_datum(),
                    day.into_datum(),
                    hour.into_datum(),
                    minute.into_datum(),
                    second.into_datum(),
                ],
            )
            .unwrap()
        }
    }

    pub fn with_timezone<Tz: AsRef<str> + UnwindSafe + RefUnwindSafe>(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f64,
        timezone: Tz,
    ) -> Result<Self, DateTimeConversionError> {
        let month: i32 = month as _;
        let day: i32 = day as _;
        let hour: i32 = hour as _;
        let minute: i32 = minute as _;
        let timezone_datum = timezone.as_ref().into_datum();

        PgTryBuilder::new(|| unsafe {
            Ok(direct_function_call(
                pg_sys::make_timestamptz_at_timezone,
                &[
                    year.into_datum(),
                    month.into_datum(),
                    day.into_datum(),
                    hour.into_datum(),
                    minute.into_datum(),
                    second.into_datum(),
                    timezone_datum,
                ],
            )
            .unwrap())
        })
        .catch_when(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW, |_| {
            Err(DateTimeConversionError::FieldOverflow)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT, |_| {
            Err(DateTimeConversionError::InvalidFormat)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE, |_| {
            Err(DateTimeConversionError::UnknownTimezone(timezone.as_ref().to_string()))
        })
        .execute()
    }

    #[inline]
    pub fn is_infinity(&self) -> bool {
        self.0 == Self::INFINITY
    }

    #[inline]
    pub fn is_neg_infinity(&self) -> bool {
        self.0 == Self::NEG_INFINITY
    }

    pub fn month(&self) -> u8 {
        self.extract_part(DateTimeParts::Month).unwrap().try_into().unwrap()
    }

    pub fn day(&self) -> u8 {
        self.extract_part(DateTimeParts::Day).unwrap().try_into().unwrap()
    }

    pub fn year(&self) -> i32 {
        self.extract_part(DateTimeParts::Year).unwrap().try_into().unwrap()
    }

    pub fn hour(&self) -> u8 {
        self.extract_part(DateTimeParts::Hour).unwrap().try_into().unwrap()
    }

    pub fn minute(&self) -> u8 {
        self.extract_part(DateTimeParts::Minute).unwrap().try_into().unwrap()
    }

    pub fn second(&self) -> f64 {
        self.extract_part(DateTimeParts::Second).unwrap().try_into().unwrap()
    }

    pub fn microseconds(&self) -> u32 {
        self.extract_part(DateTimeParts::Microseconds).unwrap().try_into().unwrap()
    }

    pub fn to_hms_micro(&self) -> (u8, u8, u8, u32) {
        (self.hour(), self.minute(), self.second() as u8, self.microseconds())
    }

    pub fn to_utc(&self) -> Timestamp {
        self.at_timezone("UTC").unwrap()
    }

    pub fn at_timezone<Tz: AsRef<str> + UnwindSafe + RefUnwindSafe>(
        &self,
        timezone: Tz,
    ) -> Result<Timestamp, DateTimeConversionError> {
        let timezone_datum = timezone.as_ref().into_datum();
        PgTryBuilder::new(|| unsafe {
            Ok(direct_function_call(
                pg_sys::timestamptz_zone,
                &[timezone_datum, self.clone().into_datum()],
            )
            .unwrap())
        })
        .catch_when(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW, |_| {
            Err(DateTimeConversionError::FieldOverflow)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT, |_| {
            Err(DateTimeConversionError::InvalidFormat)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE, |_| {
            Err(DateTimeConversionError::UnknownTimezone(timezone.as_ref().to_string()))
        })
        .execute()
    }

    pub fn is_finite(&self) -> bool {
        // yes, this is the correct function, despite the seemingly mismatched type name
        unsafe { direct_function_call(pg_sys::timestamp_finite, &[self.into_datum()]).unwrap() }
    }

    /// Truncate [`TimestampWithTimeZone`] to specified units
    pub fn truncate(self, units: DateTimeParts) -> Self {
        unsafe {
            direct_function_call(
                pg_sys::timestamptz_trunc,
                &[units.into_datum(), self.into_datum()],
            )
            .unwrap()
        }
    }

    /// Truncate [`TimestampWithTimeZone`] to specified units in specified time zone
    ///
    /// Not available under Postgres v11
    #[cfg(not(feature = "pg11"))]
    pub fn truncate_with_time_zone<Tz: AsRef<str>>(self, units: DateTimeParts, zone: Tz) -> Self {
        unsafe {
            direct_function_call(
                pg_sys::timestamptz_trunc_zone,
                &[units.into_datum(), self.into_datum(), zone.as_ref().into_datum()],
            )
            .unwrap()
        }
    }

    /// Subtract `other` from `self`, producing a “symbolic” result that uses years and months, rather than just days
    pub fn age(&self, other: &TimestampWithTimeZone) -> Interval {
        let ts_self: Timestamp = (*self).into();
        let ts_other: Timestamp = (*other).into();
        ts_self.age(&ts_other)
    }
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum FromTimeError {
    #[error("timestamp value is negative infinity and shouldn't map to time::PrimitiveDateTime")]
    NegInfinity,
    #[error("timestamp value is negative infinity and shouldn't map to time::PrimitiveDateTime")]
    Infinity,
    #[error("time::PrimitiveDateTime was unable to convert this timestamp")]
    TimeCrate,
    #[error("microseconds outside of target microsecond range")]
    MicrosOutOfBounds,
    #[error("hours outside of target range")]
    HoursOutOfBounds,
    #[error("minutes outside of target range")]
    MinutesOutOfBounds,
    #[error("seconds outside of target range")]
    SecondsOutOfBounds,
}

impl serde::Serialize for TimestampWithTimeZone {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer
            .serialize_str(&self.to_iso_string())
            .map_err(|e| serde::ser::Error::custom(format!("formatting problem: {:?}", e)))
    }
}

impl<'de> serde::Deserialize<'de> for TimestampWithTimeZone {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(crate::DateTimeTypeVisitor::<Self>::new())
    }
}

unsafe impl SqlTranslatable for TimestampWithTimeZone {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("timestamp with time zone"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("timestamp with time zone")))
    }
}
