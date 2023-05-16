/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{
    direct_function_call, pg_sys, Date, DateTimeParts, FromDatum, HasExtractableParts, IntoDatum,
    Time, TimestampWithTimeZone,
};
use core::ffi::CStr;
use pgrx_pg_sys::errcodes::PgSqlErrorCode;
use pgrx_pg_sys::PgTryBuilder;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use std::num::TryFromIntError;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Timestamp(pub pg_sys::Timestamp);

impl From<(Date, Time)> for Timestamp {
    fn from(value: (Date, Time)) -> Self {
        let (date, time) = value;
        Timestamp::new(
            date.year(),
            date.month(),
            date.day(),
            time.hour(),
            time.minute(),
            time.second(),
        )
        .unwrap()
    }
}

impl From<Timestamp> for i64 {
    #[inline]
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

impl TryFrom<TimestampWithTimeZone> for Timestamp {
    type Error = PgSqlErrorCode;

    fn try_from(value: TimestampWithTimeZone) -> Result<Self, Self::Error> {
        PgTryBuilder::new(|| unsafe {
            Ok(direct_function_call(pg_sys::timestamptz_timestamp, &[value.into_datum()]).unwrap())
        })
        .catch_when(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW, |_| {
            Err(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)
        })
        .execute()
    }
}

impl From<pg_sys::Timestamp> for Timestamp {
    fn from(value: pgrx_pg_sys::Timestamp) -> Self {
        Timestamp(value)
    }
}

impl TryFrom<pg_sys::Datum> for Timestamp {
    type Error = TryFromIntError;

    fn try_from(datum: pg_sys::Datum) -> Result<Self, Self::Error> {
        pg_sys::Timestamp::try_from(datum.value() as isize).map(|d| Timestamp(d))
    }
}

impl IntoDatum for Timestamp {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self.0))
    }
    fn type_oid() -> pg_sys::Oid {
        pg_sys::TIMESTAMPOID
    }
}

impl FromDatum for Timestamp {
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

impl Timestamp {
    const NEG_INFINITY: pg_sys::Timestamp = pg_sys::Timestamp::MIN;
    const INFINITY: pg_sys::Timestamp = pg_sys::Timestamp::MAX;

    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f64,
    ) -> Result<Self, PgSqlErrorCode> {
        let month: i32 = month as _;
        let day: i32 = day as _;
        let hour: i32 = hour as _;
        let minute: i32 = minute as _;

        PgTryBuilder::new(|| unsafe {
            Ok(direct_function_call(
                pg_sys::make_timestamp,
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
            Err(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT, |_| {
            Err(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT)
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
                pg_sys::make_timestamp,
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
}

impl serde::Serialize for Timestamp {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        let cstr;
        assert!(pg_sys::MAXDATELEN > 0); // free at runtime
        const BUF_LEN: usize = pg_sys::MAXDATELEN as usize * 2;
        let mut buffer = [0u8; BUF_LEN];
        let buf = buffer.as_mut_slice().as_mut_ptr().cast::<libc::c_char>();
        // SAFETY: This provides a quite-generous writing pad to Postgres
        // and Postgres has promised to use far less than this.
        unsafe {
            match self.0 {
                Self::NEG_INFINITY | Self::INFINITY => {
                    pg_sys::EncodeSpecialTimestamp(self.0, buf);
                }
                _ => {
                    let mut pg_tm: pg_sys::pg_tm =
                        pg_sys::pg_tm { tm_zone: std::ptr::null_mut(), ..Default::default() };
                    let mut tz = 0i32;
                    let mut fsec = 0 as pg_sys::fsec_t;
                    let mut tzn = std::ptr::null::<std::os::raw::c_char>();
                    pg_sys::timestamp2tm(
                        self.0,
                        &mut tz,
                        &mut pg_tm,
                        &mut fsec,
                        &mut tzn,
                        std::ptr::null_mut(),
                    );
                    pg_sys::EncodeDateTime(
                        &mut pg_tm,
                        fsec,
                        false,
                        0,
                        std::ptr::null::<std::os::raw::c_char>(),
                        pg_sys::USE_XSD_DATES as i32,
                        buf,
                    );
                }
            }
            assert!(buffer[BUF_LEN - 1] == 0);
            cstr = CStr::from_ptr(buf);
        }

        /* This unwrap is fine as Postgres won't ever write invalid UTF-8,
           because Postgres only writes ASCII
        */
        serializer
            .serialize_str(cstr.to_str().unwrap())
            .map_err(|e| serde::ser::Error::custom(format!("Date formatting problem: {:?}", e)))
    }
}

unsafe impl SqlTranslatable for crate::datum::Timestamp {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("timestamp"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("timestamp")))
    }
}
