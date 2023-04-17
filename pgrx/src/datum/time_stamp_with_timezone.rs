/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, IntoDatum};
use core::ffi::CStr;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use serde::Deserialize;
use std::convert::TryFrom;

#[allow(dead_code)] // such is cfg life
pub(crate) const USECS_PER_SEC: i64 = 1_000_000;

#[cfg(feature = "time-crate")]
mod with_time_crate {
    use super::*;
    use std::ops::Sub;
    use time::macros::date;

    const PG_EPOCH_OFFSET: time::OffsetDateTime = date!(2000 - 01 - 01).midnight().assume_utc();
    const PG_EPOCH_DATETIME: time::PrimitiveDateTime = date!(2000 - 01 - 01).midnight();

    impl TryFrom<time::OffsetDateTime> for TimestampWithTimeZone {
        type Error = FromTimeError;
        fn try_from(offset: time::OffsetDateTime) -> Result<Self, Self::Error> {
            let usecs = offset.sub(PG_EPOCH_OFFSET).whole_microseconds() as i64;
            usecs.try_into()
        }
    }

    impl TryFrom<TimestampWithTimeZone> for time::PrimitiveDateTime {
        type Error = FromTimeError;
        fn try_from(tstz: TimestampWithTimeZone) -> Result<Self, Self::Error> {
            match tstz {
                TimestampWithTimeZone::NEG_INFINITY => Err(FromTimeError::NegInfinity),
                TimestampWithTimeZone::INFINITY => Err(FromTimeError::Infinity),
                _ => {
                    let sec = tstz.0 / USECS_PER_SEC;
                    let usec = tstz.0 - (sec * USECS_PER_SEC);
                    let duration = time::Duration::new(sec, (usec as i32) * 1000);
                    match PG_EPOCH_DATETIME.checked_add(duration) {
                        Some(datetime) => Ok(datetime),
                        None => Err(FromTimeError::TimeCrate),
                    }
                }
            }
        }
    }

    impl TryFrom<time::PrimitiveDateTime> for TimestampWithTimeZone {
        type Error = FromTimeError;

        fn try_from(datetime: time::PrimitiveDateTime) -> Result<Self, Self::Error> {
            let offset = datetime.assume_utc();
            offset.try_into()
        }
    }

    impl TryFrom<TimestampWithTimeZone> for time::OffsetDateTime {
        type Error = FromTimeError;
        fn try_from(tstz: TimestampWithTimeZone) -> Result<Self, Self::Error> {
            let datetime: time::PrimitiveDateTime = tstz.try_into()?;
            Ok(datetime.assume_utc())
        }
    }
}

impl TryFrom<pg_sys::Datum> for TimestampWithTimeZone {
    type Error = FromTimeError;
    fn try_from(datum: pg_sys::Datum) -> Result<Self, Self::Error> {
        (datum.value() as pg_sys::TimestampTz).try_into()
    }
}

// taken from /include/datatype/timestamp.h
const MIN_TIMESTAMP_USEC: i64 = -211_813_488_000_000_000;
const END_TIMESTAMP_USEC: i64 = 9_223_371_331_200_000_000 - 1; // dec by 1 to accommodate exclusive range match pattern

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[repr(transparent)]
pub struct TimestampWithTimeZone(pg_sys::TimestampTz);

impl TimestampWithTimeZone {
    pub const NEG_INFINITY: Self = TimestampWithTimeZone(i64::MIN);
    pub const INFINITY: Self = TimestampWithTimeZone(i64::MAX);

    #[inline]
    pub fn is_infinity(&self) -> bool {
        self == &Self::INFINITY
    }

    #[inline]
    pub fn is_neg_infinity(&self) -> bool {
        self == &Self::NEG_INFINITY
    }
}

impl From<TimestampWithTimeZone> for i64 {
    fn from(tstz: TimestampWithTimeZone) -> Self {
        tstz.0
    }
}

impl TryFrom<pg_sys::TimestampTz> for TimestampWithTimeZone {
    type Error = FromTimeError;

    fn try_from(value: pg_sys::TimestampTz) -> Result<Self, Self::Error> {
        let usec = value as i64;
        match usec {
            i64::MIN => Ok(Self::NEG_INFINITY),
            i64::MAX => Ok(Self::INFINITY),
            MIN_TIMESTAMP_USEC..=END_TIMESTAMP_USEC => Ok(TimestampWithTimeZone(usec)),
            _ => Err(FromTimeError::MicrosOutOfBounds),
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
                        true,
                        tz,
                        tzn,
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

unsafe impl SqlTranslatable for TimestampWithTimeZone {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("timestamp with time zone"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("timestamp with time zone")))
    }
}
