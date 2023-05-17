/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::datetime_support::{DateTimeParts, HasExtractableParts};
use crate::datum::time::Time;
use crate::{
    direct_function_call, direct_function_call_as_datum, pg_sys, FromDatum, Interval, IntoDatum,
    PgMemoryContexts, TimestampWithTimeZone, ToIsoString,
};
use pgrx_pg_sys::errcodes::PgSqlErrorCode;
use pgrx_pg_sys::PgTryBuilder;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use std::panic::RefUnwindSafe;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct TimeWithTimeZone(pub pg_sys::TimeTzADT);

impl From<pg_sys::TimeTzADT> for TimeWithTimeZone {
    #[inline]
    fn from(value: pg_sys::TimeTzADT) -> Self {
        TimeWithTimeZone(value)
    }
}

impl From<TimeWithTimeZone> for pg_sys::TimeTzADT {
    #[inline]
    fn from(value: TimeWithTimeZone) -> Self {
        value.0
    }
}

impl From<TimeWithTimeZone> for (pg_sys::TimeADT, i32) {
    #[inline]
    fn from(value: TimeWithTimeZone) -> Self {
        (value.0.time, value.0.zone)
    }
}

impl From<(pg_sys::TimeADT, i32)> for TimeWithTimeZone {
    #[inline]
    fn from(value: (pg_sys::TimeADT, i32)) -> Self {
        TimeWithTimeZone { 0: pg_sys::TimeTzADT { time: value.0, zone: value.1 } }
    }
}

impl From<TimestampWithTimeZone> for TimeWithTimeZone {
    fn from(value: TimestampWithTimeZone) -> Self {
        unsafe { direct_function_call(pg_sys::timestamptz_timetz, &[value.into_datum()]).unwrap() }
    }
}

impl FromDatum for TimeWithTimeZone {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<TimeWithTimeZone> {
        if is_null {
            None
        } else {
            unsafe { Some(TimeWithTimeZone(datum.cast_mut_ptr::<pg_sys::TimeTzADT>().read())) }
        }
    }
}

impl IntoDatum for TimeWithTimeZone {
    #[inline]
    fn into_datum(mut self) -> Option<pg_sys::Datum> {
        let timetzadt = unsafe {
            PgMemoryContexts::CurrentMemoryContext
                .copy_ptr_into(&mut self.0 as *mut _, core::mem::size_of::<pg_sys::TimeTzADT>())
        };

        Some(pg_sys::Datum::from(timetzadt))
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::TIMETZOID
    }
}

impl TimeWithTimeZone {
    pub fn new(hour: u8, minute: u8, second: f64) -> Result<Self, PgSqlErrorCode> {
        PgTryBuilder::new(|| unsafe {
            let hour = hour as i32;
            let minute = minute as i32;
            let time = direct_function_call_as_datum(
                pg_sys::make_time,
                &[hour.into_datum(), minute.into_datum(), second.into_datum()],
            );
            Ok(direct_function_call(pg_sys::time_timetz, &[time]).unwrap())
        })
        .catch_when(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW, |_| {
            Err(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT, |_| {
            Err(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT)
        })
        .execute()
    }

    pub fn with_timezone<Tz: AsRef<str> + RefUnwindSafe>(
        hour: u8,
        minute: u8,
        second: f64,
        timezone: Tz,
    ) -> Result<Self, PgSqlErrorCode> {
        PgTryBuilder::new(|| {
            let mut time = Self::new(hour, minute, second)?;
            let tzoff = crate::get_timezone_offset(timezone.as_ref())?;

            time.0.zone = -tzoff;
            Ok(time)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW, |_| {
            Err(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT, |_| {
            Err(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE, |_| {
            Err(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE)
        })
        .execute()
    }

    pub fn with_timezone_offset(
        hour: u8,
        minute: u8,
        second: f64,
        timezone_offset: Interval,
    ) -> Result<Self, PgSqlErrorCode> {
        PgTryBuilder::new(|| unsafe {
            let time = Self::new(hour, minute, second)?;

            Ok(direct_function_call(
                pg_sys::timetz_izone,
                &[timezone_offset.into_datum(), time.into_datum()],
            )
            .unwrap())
        })
        .catch_when(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW, |_| {
            Err(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT, |_| {
            Err(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE, |_| {
            Err(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE)
        })
        .execute()
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

    pub fn timezone_offset(&self) -> i32 {
        self.extract_part(DateTimeParts::Timezone).unwrap().try_into().unwrap()
    }

    pub fn timezone_hour(&self) -> i32 {
        self.extract_part(DateTimeParts::TimezoneHour).unwrap().try_into().unwrap()
    }

    pub fn timezone_minute(&self) -> i32 {
        self.extract_part(DateTimeParts::TimezoneMinute).unwrap().try_into().unwrap()
    }

    pub fn to_hms_micro(&self) -> (u8, u8, u8, u32) {
        (self.hour(), self.minute(), self.second() as u8, self.microseconds())
    }

    pub fn to_utc(&self) -> Time {
        self.at_timezone("UTC").unwrap().into()
    }

    pub fn at_timezone<Tz: AsRef<str>>(&self, timezone: Tz) -> Result<Self, PgSqlErrorCode> {
        let timezone = timezone.as_ref().into_datum();
        PgTryBuilder::new(|| unsafe {
            Ok(direct_function_call(pg_sys::timetz_zone, &[timezone, self.clone().into_datum()])
                .unwrap())
        })
        .catch_when(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW, |_| {
            Err(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT, |_| {
            Err(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE, |_| {
            Err(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE)
        })
        .execute()
    }
}

impl From<Time> for TimeWithTimeZone {
    fn from(t: Time) -> TimeWithTimeZone {
        TimeWithTimeZone(pg_sys::TimeTzADT { time: t.0, zone: 0 })
    }
}

impl serde::Serialize for TimeWithTimeZone {
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

impl<'de> serde::Deserialize<'de> for TimeWithTimeZone {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(crate::DateTimeTypeVisitor::<Self>::new())
    }
}

unsafe impl SqlTranslatable for TimeWithTimeZone {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("time with time zone"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("time with time zone")))
    }
}
