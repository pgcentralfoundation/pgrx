// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use crate::datum::time::Time;
use crate::{pg_sys, FromDatum, IntoDatum, PgBox};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct TimeWithTimeZone(Time);
impl FromDatum for TimeWithTimeZone {
    #[inline]
    unsafe fn from_datum(datum: usize, is_null: bool, typoid: u32) -> Option<TimeWithTimeZone> {
        if is_null {
            None
        } else {
            let timetz = PgBox::from_pg(datum as *mut pg_sys::TimeTzADT);

            let mut time = Time::from_datum(timetz.time as pg_sys::Datum, false, typoid)
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
            .expect("failed to convert timetz into datum") as i64;

        Some(timetz.into_pg() as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::TIMETZOID
    }
}

impl TimeWithTimeZone {
    /// This shifts the provided `time` back to UTC using the specified `utc_offset`
    pub fn new(mut time: time::Time, at_tz_offset: time::UtcOffset) -> Self {
        time -= time::Duration::seconds(at_tz_offset.as_seconds() as i64);
        TimeWithTimeZone(Time(time))
    }
}

impl Deref for TimeWithTimeZone {
    type Target = time::Time;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TimeWithTimeZone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
            serializer.serialize_str(&self.format(&format!("%H:%M:%S.{}-00", self.millisecond())))
        } else {
            serializer.serialize_str(&self.format("%H:%M:%S-00"))
        }
    }
}
