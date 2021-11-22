// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::datum::time::USECS_PER_SEC;
use crate::{direct_function_call_as_datum, pg_sys, FromDatum, IntoDatum};
use std::{
    convert::TryFrom,
    ops::{Deref, DerefMut},
};
use time::{format_description::FormatItem, UtcOffset};

#[derive(Debug)]
pub struct TimestampWithTimeZone(time::OffsetDateTime);

impl FromDatum for TimestampWithTimeZone {
    #[inline]
    unsafe fn from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: u32,
    ) -> Option<TimestampWithTimeZone> {
        if is_null {
            None
        } else {
            let mut tm = pg_sys::pg_tm {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: 0,
                tm_gmtoff: 0,
                tm_zone: std::ptr::null_mut(),
            };
            let mut tz = 0i32;
            let mut fsec = 0 as pg_sys::fsec_t;
            let mut tzn = std::ptr::null::<std::os::raw::c_char>();
            pg_sys::timestamp2tm(
                datum as i64,
                &mut tz,
                &mut tm,
                &mut fsec,
                &mut tzn,
                std::ptr::null_mut(),
            );
            let date = time::Date::from_calendar_date(
                tm.tm_year,
                time::Month::try_from(tm.tm_mon as u8)
                    .expect("Got month outside of range in TimestampWithTimeZone::from_datum"),
                tm.tm_mday as u8,
            )
            .expect("failed to create date from TimestampWithTimeZone");

            let time = time::Time::from_hms_micro(
                tm.tm_hour as u8,
                tm.tm_min as u8,
                tm.tm_sec as u8,
                fsec as u32,
            )
            .expect("failed to create time from TimestampWithTimeZonez");

            Some(TimestampWithTimeZone(
                time::PrimitiveDateTime::new(date, time)
                    .assume_utc()
                    .to_offset(
                        UtcOffset::from_whole_seconds(tz)
                            .expect("Unexpected error in `UtcOffset::from_whole_seconds` during `TimestampWithTimeZone::from_datum`")
                    ),
            ))
        }
    }
}

impl IntoDatum for TimestampWithTimeZone {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let year = self.year();
        let month = self.month() as i32;
        let mday = self.day() as i32;
        let hour = self.hour() as i32;
        let minute = self.minute() as i32;
        let second = self.second() as f64 + (self.microsecond() as f64 / USECS_PER_SEC as f64);

        unsafe {
            direct_function_call_as_datum(
                pg_sys::make_timestamptz_at_timezone,
                vec![
                    year.into_datum(),
                    month.into_datum(),
                    mday.into_datum(),
                    hour.into_datum(),
                    minute.into_datum(),
                    second.into_datum(),
                    "UTC".into_datum(),
                ],
            )
        }
    }

    fn type_oid() -> u32 {
        pg_sys::TIMESTAMPTZOID
    }
}

impl TimestampWithTimeZone {
    /// This shifts the provided `time` back to UTC
    pub fn new(time: time::PrimitiveDateTime, at_tz_offset: time::UtcOffset) -> Self {
        TimestampWithTimeZone(
            time.assume_utc()
                .to_offset(
                    UtcOffset::from_whole_seconds(-at_tz_offset.whole_seconds())
                        .expect("Unexpected error in `UtcOffset::from_whole_seconds` during `TimestampWithTimeZone::new`")
                ),
        )
    }
}

impl Deref for TimestampWithTimeZone {
    type Target = time::OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TimestampWithTimeZone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl serde::Serialize for TimestampWithTimeZone {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        if self.millisecond() > 0 {
            serializer.serialize_str(
                &self
                    .format(
                        &time::format_description::parse(&format!(
                            "[year]-[month]-[day]T[hour]:[minute]:[second].{}-00",
                            self.millisecond()
                        ))
                        .map_err(|e| {
                            serde::ser::Error::custom(format!(
                                "TimeStampWithTimeZone invalid format problem: {:?}",
                                e
                            ))
                        })?,
                    )
                    .map_err(|e| {
                        serde::ser::Error::custom(format!(
                            "TimeStampWithTimeZone formatting problem: {:?}",
                            e
                        ))
                    })?,
            )
        } else {
            serializer.serialize_str(
                &self
                    .format(&DEFAULT_TIMESTAMP_WITH_TIMEZONE_FORMAT)
                    .map_err(|e| {
                        serde::ser::Error::custom(format!(
                            "TimeStampWithTimeZone formatting problem: {:?}",
                            e
                        ))
                    })?,
            )
        }
    }
}

static DEFAULT_TIMESTAMP_WITH_TIMEZONE_FORMAT: &[FormatItem<'static>] =
    time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]-00");
