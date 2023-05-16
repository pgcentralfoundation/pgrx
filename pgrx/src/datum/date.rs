/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::datetime_support::{DateTimeParts, HasExtractableParts};
use crate::{direct_function_call, pg_sys, FromDatum, IntoDatum};
use core::num::TryFromIntError;
use pgrx_pg_sys::errcodes::PgSqlErrorCode;
use pgrx_pg_sys::PgTryBuilder;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

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

impl TryFrom<pg_sys::Datum> for Date {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(d: pg_sys::Datum) -> Result<Self, Self::Error> {
        i32::try_from(d.value() as isize).map(|d| Date(d))
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

const NEG_INFINITY: pg_sys::DateADT = pg_sys::DateADT::MIN;
const INFINITY: pg_sys::DateADT = pg_sys::DateADT::MAX;

impl Date {
    pub fn new(year: isize, month: u8, day: u8) -> Result<Self, PgSqlErrorCode> {
        let year: i32 =
            year.try_into().map_err(|_| PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)?;
        let month: i32 =
            month.try_into().map_err(|_| PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)?;
        let day: i32 =
            day.try_into().map_err(|_| PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)?;

        let result = PgTryBuilder::new(|| unsafe {
            let result = direct_function_call(
                pg_sys::make_date,
                &[year.into_datum(), month.into_datum(), day.into_datum()],
            )
            .unwrap();
            Ok(result)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW, |_| {
            Err(PgSqlErrorCode::ERRCODE_DATETIME_FIELD_OVERFLOW)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT, |_| {
            Err(PgSqlErrorCode::ERRCODE_INVALID_DATETIME_FORMAT)
        })
        .execute();
        result
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
    pub fn from_pg_epoch_days(pg_epoch_days: i32) -> Date {
        Date(pg_epoch_days)
    }

    #[inline]
    pub fn is_infinity(&self) -> bool {
        self.0 == INFINITY
    }

    #[inline]
    pub fn is_neg_infinity(&self) -> bool {
        self.0 == NEG_INFINITY
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

#[cfg(feature = "time-crate")]
pub use with_time_crate::TryFromDateError;

#[cfg(feature = "time-crate")]
mod with_time_crate {
    use crate::{Date, POSTGRES_EPOCH_JDATE};
    use core::fmt::{Display, Formatter};
    use std::error::Error;

    #[derive(Debug, PartialEq, Clone)]
    #[non_exhaustive]
    pub struct TryFromDateError(pub Date);

    impl TryFromDateError {
        #[inline]
        pub fn into_inner(self) -> Date {
            self.0
        }

        #[inline]
        pub fn as_i32(&self) -> i32 {
            self.0 .0
        }
    }

    impl Display for TryFromDateError {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            write!(f, "`{}` is not compatible with `time::Date`", self.0 .0)
        }
    }

    impl Error for TryFromDateError {}

    impl From<time::Date> for Date {
        #[inline]
        fn from(date: time::Date) -> Self {
            Date::from_pg_epoch_days(date.to_julian_day() - POSTGRES_EPOCH_JDATE)
        }
    }

    impl TryFrom<Date> for time::Date {
        type Error = TryFromDateError;
        fn try_from(date: Date) -> Result<time::Date, Self::Error> {
            const INNER_RANGE_BEGIN: i32 = time::Date::MIN.to_julian_day();
            const INNER_RANGE_END: i32 = time::Date::MAX.to_julian_day();
            match date.0 {
                INNER_RANGE_BEGIN..=INNER_RANGE_END => {
                    time::Date::from_julian_day(date.0 + POSTGRES_EPOCH_JDATE)
                        .or_else(|_e| Err(TryFromDateError(date)))
                }
                _ => Err(TryFromDateError(date)),
            }
        }
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
                NEG_INFINITY | INFINITY => {
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
