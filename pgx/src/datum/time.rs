/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, FromTimeError, IntoDatum};
use pgx_utils::sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
#[cfg(feature = "time-crate")]
use time::format_description::FormatItem;

const MINS_PER_HOUR: u64 = 60;
const SEC_PER_MIN: u64 = 60;
pub(crate) const USECS_PER_SEC: u64 = 1_000_000;
pub(crate) const USECS_PER_MINUTE: u64 = USECS_PER_SEC * SEC_PER_MIN;
pub(crate) const USECS_PER_HOUR: u64 = USECS_PER_MINUTE * MINS_PER_HOUR;
pub(crate) const USECS_PER_DAY: u64 = USECS_PER_HOUR * 24;

#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct Time(pub u64 /* Microseconds since midnight */);
impl FromDatum for Time {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: u32,
    ) -> Option<Time> {
        if is_null {
            None
        } else {
            Some(Time(datum.value() as _))
        }
    }
}

impl IntoDatum for Time {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let datum = pg_sys::Datum::try_from(self.0).unwrap();

        Some(datum)
    }

    fn type_oid() -> u32 {
        pg_sys::TIMEOID
    }
}

impl Time {
    pub const ALLBALLS: Self = Time(0);

    pub fn from_hms_micro(h: u8, m: u8, s: u8, micro: u32) -> Result<Time, FromTimeError> {
        match (h, m, s, micro) {
            (24, 0, 0, 0) => Ok(Time(u64::from(h) * USECS_PER_HOUR)),
            (24.., _, _, _) => Err(FromTimeError::HoursOutOfBounds),
            (_, 60.., _, _) => Err(FromTimeError::MinutesOutOfBounds),
            (_, _, 60.., _) => Err(FromTimeError::SecondsOutOfBounds),
            (0..=23, 0..=59, 0..=59, _) => {
                let t = u64::from(h) * USECS_PER_HOUR
                    + u64::from(m) * USECS_PER_MINUTE
                    + u64::from(s) * USECS_PER_SEC
                    + u64::from(micro);
                if t > USECS_PER_DAY {
                    Err(FromTimeError::MicrosOutOfBounds)
                } else {
                    Ok(Time(t))
                }
            }
        }
    }

    /// To hours, minutes, seconds, and microseconds.
    pub fn to_hms_micro(self) -> (u8, u8, u8, u32) {
        let mut time = self.0;
        let hour = time / USECS_PER_HOUR;
        time -= hour * USECS_PER_HOUR;

        let min = time / USECS_PER_MINUTE;
        time -= min * USECS_PER_MINUTE;

        let sec = time / USECS_PER_SEC;
        time -= sec * USECS_PER_SEC;

        let hour = u8::try_from(hour).unwrap();
        let min = u8::try_from(min).unwrap();
        let sec = u8::try_from(sec).unwrap();
        let micro = u32::try_from(time).unwrap();
        (hour, min, sec, micro)
    }
}

#[cfg(feature = "time-crate")]
mod with_time_crate {
    impl TryFrom<time::Time> for crate::Time {
        type Error = crate::FromTimeError;
        fn try_from(t: time::Time) -> Result<crate::Time, Self::Error> {
            let (h, m, s, micro) = t.as_hms_micro();
            Self::from_hms_micro(h, m, s, micro)
        }
    }
}
#[cfg(feature = "time-crate")]
impl serde::Serialize for Time {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        let (h, m, s, micro) = self.clone().to_hms_micro();
        let t = time::Time::from_hms_micro(h, m, s, micro).unwrap();
        if t.millisecond() > 0 {
            serializer.serialize_str(
                &t.format(
                    &time::format_description::parse(&format!(
                        "[hour]:[minute]:[second].{}",
                        t.millisecond()
                    ))
                    .map_err(|e| {
                        serde::ser::Error::custom(format!("Time invalid format problem: {:?}", e))
                    })?,
                )
                .map_err(|e| {
                    serde::ser::Error::custom(format!("Time formatting problem: {:?}", e))
                })?,
            )
        } else {
            serializer.serialize_str(&t.format(&DEFAULT_TIME_FORMAT).map_err(|e| {
                serde::ser::Error::custom(format!("Time formatting problem: {:?}", e))
            })?)
        }
    }
}
#[cfg(feature = "time-crate")]
static DEFAULT_TIME_FORMAT: &[FormatItem<'static>] =
    time::macros::format_description!("[hour]:[minute]:[second]");

unsafe impl SqlTranslatable for Time {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("time"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("time")))
    }
}
