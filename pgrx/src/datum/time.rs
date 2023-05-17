/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::datum::datetime_support::*;
use crate::{
    direct_function_call, pg_sys, FromDatum, Interval, IntoDatum, TimeWithTimeZone, Timestamp,
    TimestampWithTimeZone,
};
use pgrx_pg_sys::errcodes::PgSqlErrorCode;
use pgrx_pg_sys::PgTryBuilder;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use std::num::TryFromIntError;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Time(pub pg_sys::TimeADT);

impl From<pg_sys::TimeADT> for Time {
    #[inline]
    fn from(value: pg_sys::TimeADT) -> Self {
        Time(value)
    }
}

impl From<Time> for pg_sys::TimeADT {
    #[inline]
    fn from(value: Time) -> Self {
        value.0
    }
}

impl From<Timestamp> for Time {
    fn from(value: Timestamp) -> Self {
        unsafe { direct_function_call(pg_sys::timestamp_time, &[value.into_datum()]).unwrap() }
    }
}

impl From<TimestampWithTimeZone> for Time {
    #[inline]
    fn from(value: TimestampWithTimeZone) -> Self {
        unsafe { direct_function_call(pg_sys::timestamptz_time, &[value.into_datum()]).unwrap() }
    }
}

impl From<Interval> for Time {
    fn from(value: Interval) -> Self {
        unsafe { direct_function_call(pg_sys::interval_time, &[value.into_datum()]).unwrap() }
    }
}

impl From<TimeWithTimeZone> for Time {
    fn from(value: TimeWithTimeZone) -> Self {
        unsafe { direct_function_call(pg_sys::timetz_time, &[value.into_datum()]).unwrap() }
    }
}

impl TryFrom<pg_sys::Datum> for Time {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(d: pg_sys::Datum) -> Result<Self, Self::Error> {
        let t: pg_sys::TimeADT = d.value().try_into()?;
        Ok(Time(t))
    }
}

impl FromDatum for Time {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<Time> {
        if is_null {
            None
        } else {
            Some(datum.try_into().expect("Error converting time datum"))
        }
    }
}

impl IntoDatum for Time {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self.0))
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::TIMEOID
    }
}

impl Time {
    pub const ALLBALLS: Self = Time(0);

    pub fn new(hour: u8, minute: u8, second: f64) -> Result<Time, PgSqlErrorCode> {
        let hour: i32 = hour as _;
        let minute: i32 = minute as _;

        PgTryBuilder::new(|| unsafe {
            Ok(direct_function_call(
                pg_sys::make_time,
                &[hour.into_datum(), minute.into_datum(), second.into_datum()],
            )
            .unwrap())
        })
        .catch_when(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW, |_| {
            Err(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT, |_| {
            Err(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT)
        })
        .execute()
    }

    pub fn new_unchecked(hour: u8, minute: u8, second: f64) -> Time {
        let hour: i32 = hour.try_into().expect("invalid hour");
        let minute: i32 = minute.try_into().expect("invalid minute");

        unsafe {
            direct_function_call(
                pg_sys::make_time,
                &[hour.into_datum(), minute.into_datum(), second.into_datum()],
            )
            .unwrap()
        }
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
}

impl serde::Serialize for Time {
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

impl<'de> serde::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(crate::DateTimeTypeVisitor::<Self>::new())
    }
}

unsafe impl SqlTranslatable for Time {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("time"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("time")))
    }
}
