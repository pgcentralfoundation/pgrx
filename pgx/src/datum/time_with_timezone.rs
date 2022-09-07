/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::datum::time::Time;
use crate::{pg_sys, FromDatum, IntoDatum, PgBox};
use time::format_description::FormatItem;

#[derive(Debug)]
pub struct TimeWithTimeZone(Time);
impl FromDatum for TimeWithTimeZone {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<TimeWithTimeZone> {
        if is_null {
            None
        } else {
            let timetz = PgBox::from_pg(datum.ptr_cast::<pg_sys::TimeTzADT>());

            let mut time = Time::from_datum(pg_sys::Datum::from(timetz.time), false)
                .expect("failed to convert TimeWithTimeZone");
            time.0 += time::Duration::seconds(timetz.zone as i64);

            Some(TimeWithTimeZone(time))
        }
    }
}

impl IntoDatum for TimeWithTimeZone {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let mut timetz = PgBox::<pg_sys::TimeTzADT>::alloc();
        timetz.zone = 0;
        timetz.time = self
            .0
            .into_datum()
            .expect("failed to convert timetz into datum")
            .value() as i64;

        Some(timetz.into_pg().into())
    }

    fn type_oid() -> u32 {
        pg_sys::TIMETZOID
    }
}

impl TimeWithTimeZone {
    /// This shifts the provided `time` back to UTC using the specified `utc_offset`
    pub fn new(mut time: time::Time, at_tz_offset: time::UtcOffset) -> Self {
        time -= time::Duration::seconds(at_tz_offset.whole_seconds() as i64);
        TimeWithTimeZone(Time(time))
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
        if self.millisecond() > 0 {
            serializer.serialize_str(
                &self
                    .format(
                        &time::format_description::parse(&format!(
                            "[hour]:[minute]:[second].{}-00",
                            self.millisecond()
                        ))
                        .map_err(|e| {
                            serde::ser::Error::custom(format!(
                                "TimeWithTimeZone invalid format problem: {:?}",
                                e
                            ))
                        })?,
                    )
                    .map_err(|e| {
                        serde::ser::Error::custom(format!(
                            "TimeWithTimeZone formatting problem: {:?}",
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
                            "TimeWithTimeZone formatting problem: {:?}",
                            e
                        ))
                    })?,
            )
        }
    }
}

static DEFAULT_TIMESTAMP_WITH_TIMEZONE_FORMAT: &[FormatItem<'static>] =
    time::macros::format_description!("[hour]:[minute]:[second]-00");
