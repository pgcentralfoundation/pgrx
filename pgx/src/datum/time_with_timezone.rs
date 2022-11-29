/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::datum::time::{Time, USECS_PER_DAY};
use crate::{pg_sys, FromDatum, IntoDatum, PgBox};
use pgx_utils::sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TimeWithTimeZone {
    t: Time,
    /// America/Denver time in ISO:      -06:00
    /// America/Denver time in Postgres: +21600
    /// Yes, the sign is flipped, following POSIX instead of ISO. Don't overthink it.
    tz_secs: i32,
}

impl FromDatum for TimeWithTimeZone {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: u32,
    ) -> Option<TimeWithTimeZone> {
        if is_null {
            None
        } else {
            let timetz = PgBox::from_pg(datum.cast_mut_ptr::<pg_sys::TimeTzADT>());

            let t = Time::from_polymorphic_datum(timetz.time.into(), false, typoid)
                .expect("failed to convert TimeWithTimeZone");
            let tz_secs = timetz.zone;

            Some(TimeWithTimeZone { t, tz_secs })
        }
    }
}

impl IntoDatum for TimeWithTimeZone {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let mut timetz = PgBox::<pg_sys::TimeTzADT>::alloc();
        timetz.zone = self.tz_secs;
        timetz.time = self.t.0 as i64;

        Some(timetz.into_pg().into())
    }

    fn type_oid() -> u32 {
        pg_sys::TIMETZOID
    }
}

impl TimeWithTimeZone {
    /// Constructs a TimeWithTimeZone from `time` crate components.
    #[deprecated(
        since = "0.5.0",
        note = "the repr of pgx::TimeWithTimeZone is no longer time::Time \
    and this fn will be removed in a future version"
    )]
    #[cfg(feature = "time-crate")]
    pub fn new(time: time::Time, at_tz_offset: time::UtcOffset) -> Self {
        let (h, m, s, micro) = time.as_hms_micro();
        let t = Time::from_hms_micro(h, m, s, micro).unwrap();
        // Flip the sign, because time::Time uses the ISO sign convention
        let tz_secs = -at_tz_offset.whole_seconds();
        TimeWithTimeZone { t, tz_secs }
    }

    pub fn to_utc(self) -> Time {
        let TimeWithTimeZone { t, tz_secs } = self;
        let tz_micros = tz_secs as i64 * 1_000_000;
        // tz_secs uses a flipped sign from the ISO tz string, so just add to get UTC
        let t_unwrapped = t.0 as i64 + tz_micros;
        Time(t_unwrapped.rem_euclid(USECS_PER_DAY as i64) as u64)
    }
}

impl From<Time> for TimeWithTimeZone {
    fn from(t: Time) -> TimeWithTimeZone {
        TimeWithTimeZone { t, tz_secs: 0 }
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
        let cstr: Option<&cstr_core::CStr> = unsafe {
            crate::direct_function_call(
                pg_sys::timetz_out,
                vec![Some(pg_sys::Datum::from(self as *const Self))],
            )
        };
        serializer.serialize_str(cstr.and_then(|c| c.to_str().ok()).unwrap())
    }
}

unsafe impl SqlTranslatable for TimeWithTimeZone {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("time with time zone"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("time with time zone")))
    }
}
