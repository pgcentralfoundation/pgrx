/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, IntoDatum};
use core::num::TryFromIntError;
use pgx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use std::ffi::CStr;

pub const POSTGRES_EPOCH_JDATE: i32 = pg_sys::POSTGRES_EPOCH_JDATE as i32;
pub const UNIX_EPOCH_JDATE: i32 = pg_sys::UNIX_EPOCH_JDATE as i32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[repr(transparent)]
pub struct Date(i32);

impl TryFrom<pg_sys::Datum> for Date {
    type Error = TryFromIntError;
    fn try_from(d: pg_sys::Datum) -> Result<Self, Self::Error> {
        i32::try_from(d.value() as isize).map(|d| Date(d))
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

impl FromDatum for Date {
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

impl Date {
    pub const NEG_INFINITY: Self = Date(i32::MIN);
    pub const INFINITY: Self = Date(i32::MAX);

    #[inline]
    pub fn from_pg_epoch_days(pg_epoch_days: i32) -> Date {
        Date(pg_epoch_days)
    }

    #[inline]
    pub fn is_infinity(&self) -> bool {
        self == &Self::INFINITY
    }

    #[inline]
    pub fn is_neg_infinity(&self) -> bool {
        self == &Self::NEG_INFINITY
    }

    #[inline]
    pub fn to_julian_days(&self) -> i32 {
        self.0 + POSTGRES_EPOCH_JDATE
    }

    #[inline]
    pub fn to_pg_epoch_days(&self) -> i32 {
        self.0
    }

    #[inline]
    fn to_unix_epoch_days(&self) -> i32 {
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
impl From<time::Date> for Date {
    #[inline]
    fn from(date: time::Date) -> Self {
        Date::from_pg_epoch_days(date.to_julian_day() - POSTGRES_EPOCH_JDATE)
    }
}

#[cfg(feature = "time-crate")]
impl TryFrom<Date> for time::Date {
    type Error = i32;
    fn try_from(date: Date) -> Result<time::Date, Self::Error> {
        const INNER_RANGE_BEGIN: i32 = time::Date::MIN.to_julian_day();
        const INNER_RANGE_END: i32 = time::Date::MAX.to_julian_day();
        match date.0 {
            INNER_RANGE_BEGIN..=INNER_RANGE_END => {
                time::Date::from_julian_day(date.0 + POSTGRES_EPOCH_JDATE).or_else(|_e| Err(date.0))
            }
            v => Err(v),
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
            match self {
                &Self::NEG_INFINITY | &Self::INFINITY => {
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

unsafe impl SqlTranslatable for Date {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("date"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("date")))
    }
}
