/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, FromTimeError, IntoDatum, TimestampWithTimeZone};
use core::ffi::CStr;
use pgx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[repr(transparent)]
pub struct Timestamp(pg_sys::Timestamp);

impl Timestamp {
    pub const NEG_INFINITY: Self = Timestamp(i64::MIN);
    pub const INFINITY: Self = Timestamp(i64::MAX);

    #[inline]
    pub fn is_infinity(&self) -> bool {
        self == &Self::INFINITY
    }

    #[inline]
    pub fn is_neg_infinity(&self) -> bool {
        self == &Self::NEG_INFINITY
    }
}

impl From<TimestampWithTimeZone> for Timestamp {
    fn from(tstz: TimestampWithTimeZone) -> Self {
        Timestamp(tstz.into())
    }
}

impl From<Timestamp> for TimestampWithTimeZone {
    fn from(ts: Timestamp) -> Self {
        ts.0.try_into().expect("error converting Timestamp to TimestampWithTimeZone")
    }
}

impl From<Timestamp> for i64 {
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

impl TryFrom<pg_sys::Timestamp> for Timestamp {
    type Error = FromTimeError;

    fn try_from(value: pg_sys::Timestamp) -> Result<Self, Self::Error> {
        TryInto::<TimestampWithTimeZone>::try_into(value).map(|tstz| tstz.into())
    }
}

impl TryFrom<pg_sys::Datum> for Timestamp {
    type Error = FromTimeError;

    fn try_from(datum: pg_sys::Datum) -> Result<Self, Self::Error> {
        (datum.value() as pg_sys::Timestamp).try_into()
    }
}

#[cfg(feature = "time-crate")]
mod with_time_crate {
    use super::*;

    impl TryFrom<time::OffsetDateTime> for Timestamp {
        type Error = FromTimeError;

        fn try_from(offset: time::OffsetDateTime) -> Result<Self, Self::Error> {
            TryInto::<TimestampWithTimeZone>::try_into(offset).map(|tstz| tstz.into())
        }
    }

    impl TryFrom<Timestamp> for time::PrimitiveDateTime {
        type Error = FromTimeError;

        fn try_from(ts: Timestamp) -> Result<Self, Self::Error> {
            let tstz: TimestampWithTimeZone = ts.into();
            TryInto::<time::PrimitiveDateTime>::try_into(tstz)
        }
    }

    impl TryFrom<time::PrimitiveDateTime> for Timestamp {
        type Error = FromTimeError;

        fn try_from(datetime: time::PrimitiveDateTime) -> Result<Self, Self::Error> {
            TryInto::<TimestampWithTimeZone>::try_into(datetime).map(|tstz| tstz.into())
        }
    }

    impl TryFrom<Timestamp> for time::OffsetDateTime {
        type Error = FromTimeError;
        fn try_from(ts: Timestamp) -> Result<Self, Self::Error> {
            let tstz: TimestampWithTimeZone = ts.into();
            tstz.try_into()
        }
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
