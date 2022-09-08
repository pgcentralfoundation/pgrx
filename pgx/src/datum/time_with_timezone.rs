/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::datum::time::{Time, hms_micro_to_pgtime, pgtime_to_hms_micro};
use crate::{pg_sys, FromDatum, IntoDatum, PgBox};
use time::format_description::FormatItem;

#[derive(Debug)]
#[repr(C)]
pub struct TimeWithTimeZone{
    t: Time,
    tz: i32,
}

impl FromDatum for TimeWithTimeZone {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<TimeWithTimeZone> {
        if is_null {
            None
        } else {
            let timetz = PgBox::from_pg(datum.ptr_cast::<pg_sys::TimeTzADT>());

            let t = Time::from_datum(pg_sys::Datum::from(timetz.time), false)
                .expect("failed to convert TimeWithTimeZone");
            let tz = timetz.zone;

            Some(TimeWithTimeZone{ t, tz })
        }
    }
}

impl IntoDatum for TimeWithTimeZone {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let mut timetz = PgBox::<pg_sys::TimeTzADT>::alloc();
        timetz.zone = self.tz;
        timetz.time = self.t.0 as i64;

        Some(timetz.into_pg().into())
    }

    fn type_oid() -> u32 {
        pg_sys::TIMETZOID
    }
}

impl TimeWithTimeZone {
    /// Constructs a TimeWithTimeZone from `time` crate components.
    #[deprecated(since = "0.5.0",
    note = "the repr of pgx::TimeWithTimeZone is no longer time::Time \
    and this fn will be removed in a future version")]
    pub fn new(time: time::Time, at_tz_offset: time::UtcOffset) -> Self {
        let (h, m, s, micro) = time.as_hms_micro();
        let t = hms_micro_to_pgtime(h, m, s, micro).unwrap();
        let tz = at_tz_offset.whole_seconds();
        TimeWithTimeZone{ t, tz }
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
        let (h, m, s, micro) = pgtime_to_hms_micro(self.t.clone());
        let mut time = time::Time::from_hms_micro(h, m, s, micro).unwrap();
        time -= time::Duration::seconds(self.tz as _);
        if time.millisecond() > 0 {
            serializer.serialize_str(
                &time
                    .format(
                        &time::format_description::parse(&format!(
                            "[hour]:[minute]:[second].{}-00",
                            time.millisecond()
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
                &time
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
