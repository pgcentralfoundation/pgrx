/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use core::num::TryFromIntError;
use pgrx_pg_sys::errcodes::PgSqlErrorCode;
use pgrx_pg_sys::PgTryBuilder;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

use crate::datetime_support::{DateTimeParts, HasExtractableParts};
use crate::{direct_function_call, pg_sys, FromDatum, IntoDatum, Timestamp, TimestampWithTimeZone};

pub const POSTGRES_EPOCH_JDATE: i32 = pg_sys::POSTGRES_EPOCH_JDATE as i32;
pub const UNIX_EPOCH_JDATE: i32 = pg_sys::UNIX_EPOCH_JDATE as i32;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Date(pub pg_sys::DateADT);

impl From<pg_sys::DateADT> for Date {
    #[inline]
    fn from(value: pg_sys::DateADT) -> Self {
        Date(value)
    }
}

impl From<Date> for pg_sys::DateADT {
    #[inline]
    fn from(value: Date) -> Self {
        value.0
    }
}

impl From<Timestamp> for Date {
    fn from(value: Timestamp) -> Self {
        unsafe { direct_function_call(pg_sys::timestamp_date, &[value.into_datum()]).unwrap() }
    }
}

impl From<TimestampWithTimeZone> for Date {
    fn from(value: TimestampWithTimeZone) -> Self {
        unsafe { direct_function_call(pg_sys::timestamptz_date, &[value.into_datum()]).unwrap() }
    }
}

impl TryFrom<pg_sys::Datum> for Date {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(datum: pg_sys::Datum) -> Result<Self, Self::Error> {
        pg_sys::DateADT::try_from(datum.value() as isize).map(|d| Date(d))
    }
}

impl FromDatum for Date {
    #[inline]
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
            Some(datum.try_into().expect("Error converting date datum"))
        }
    }
}

impl IntoDatum for Date {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self.0))
    }
    fn type_oid() -> pg_sys::Oid {
        pg_sys::DATEOID
    }
}

impl Date {
    pub const NEG_INFINITY: pg_sys::DateADT = pg_sys::DateADT::MIN;
    pub const INFINITY: pg_sys::DateADT = pg_sys::DateADT::MAX;

    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, PgSqlErrorCode> {
        let month: i32 = month as _;
        let day: i32 = day as _;

        PgTryBuilder::new(|| unsafe {
            Ok(direct_function_call(
                pg_sys::make_date,
                &[year.into_datum(), month.into_datum(), day.into_datum()],
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

    pub fn new_unchecked(year: isize, month: u8, day: u8) -> Self {
        let year: i32 = year.try_into().expect("invalid year");
        let month: i32 = month.try_into().expect("invalid month");
        let day: i32 = day.try_into().expect("invalid day");

        unsafe {
            direct_function_call(
                pg_sys::make_date,
                &[year.into_datum(), month.into_datum(), day.into_datum()],
            )
            .unwrap()
        }
    }

    #[inline]
    pub fn from_pg_epoch_days(pg_epoch_days: i32) -> Date {
        Date(pg_epoch_days)
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

    #[inline]
    pub fn is_infinity(&self) -> bool {
        self.0 == Self::INFINITY
    }

    #[inline]
    pub fn is_neg_infinity(&self) -> bool {
        self.0 == Self::NEG_INFINITY
    }

    #[inline]
    pub fn to_julian_days(&self) -> i32 {
        self.0 + POSTGRES_EPOCH_JDATE
    }

    #[inline]
    pub fn to_pg_epoch_days(&self) -> i32 {
        self.0
    }

    /// Returns the date as an i32 representing the elapsed time since UNIX epoch in days
    #[inline]
    pub fn to_unix_epoch_days(&self) -> i32 {
        self.0 + POSTGRES_EPOCH_JDATE - UNIX_EPOCH_JDATE
    }

    #[inline]
    pub fn to_posix_time(&self) -> libc::time_t {
        let secs_per_day: libc::time_t =
            pg_sys::SECS_PER_DAY.try_into().expect("couldn't fit time into time_t");
        libc::time_t::from(self.to_unix_epoch_days()) * secs_per_day
    }
}

impl serde::Serialize for Date {
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
                    pg_sys::EncodeSpecialDate(self.0, buf);
                }
                _ => {
                    let mut pg_tm: pg_sys::pg_tm = Default::default();
                    pg_sys::j2date(
                        &self.0 + POSTGRES_EPOCH_JDATE,
                        &mut pg_tm.tm_year,
                        &mut pg_tm.tm_mon,
                        &mut pg_tm.tm_mday,
                    );
                    pg_sys::EncodeDateOnly(&mut pg_tm, pg_sys::USE_XSD_DATES as i32, buf)
                }
            }
            assert!(buffer[BUF_LEN - 1] == 0);
            cstr = core::ffi::CStr::from_ptr(buf);
        }

        /* This unwrap is fine as Postgres won't ever write invalid UTF-8,
           because Postgres only writes ASCII
        */
        serializer
            .serialize_str(cstr.to_str().unwrap())
            .map_err(|e| serde::ser::Error::custom(format!("Date formatting problem: {:?}", e)))
    }
}

unsafe impl SqlTranslatable for Date {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("date"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("date")))
    }
}
