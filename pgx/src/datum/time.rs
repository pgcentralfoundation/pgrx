// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use crate::{pg_sys, FromDatum, IntoDatum};
use std::ops::{Deref, DerefMut};

pub(crate) const USECS_PER_HOUR: i64 = 3_600_000_000;
pub(crate) const USECS_PER_MINUTE: i64 = 60_000_000;
pub(crate) const USECS_PER_SEC: i64 = 1_000_000;
pub(crate) const MINS_PER_HOUR: i64 = 60;
pub(crate) const SEC_PER_MIN: i64 = 60;

#[derive(Debug)]
pub struct Time(pub(crate) time::Time);
impl FromDatum for Time {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _typoid: u32) -> Option<Time> {
        if is_null {
            None
        } else {
            let mut time = datum as i64;

            let hour = time / USECS_PER_HOUR;
            time -= hour * USECS_PER_HOUR;

            let min = time / USECS_PER_MINUTE;
            time -= min * USECS_PER_MINUTE;

            let second = time / USECS_PER_SEC;
            time -= second * USECS_PER_SEC;

            let microsecond = time;

            Some(Time(
                time::Time::try_from_hms_micro(
                    hour as u8,
                    min as u8,
                    second as u8,
                    microsecond as u32,
                )
                .expect("failed to convert time"),
            ))
        }
    }
}

impl IntoDatum for Time {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let datum = ((((self.hour() as i64 * MINS_PER_HOUR + self.minute() as i64) * SEC_PER_MIN)
            + self.second() as i64)
            * USECS_PER_SEC)
            + self.microsecond() as i64;

        Some(datum as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::TIMEOID
    }
}

impl Time {
    pub fn new(time: time::Time) -> Self {
        Time(time)
    }
}

impl Deref for Time {
    type Target = time::Time;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Time {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl serde::Serialize for Time {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        if self.millisecond() > 0 {
            serializer.serialize_str(&self.format(&format!("%H:%M:%S.{}", self.millisecond())))
        } else {
            serializer.serialize_str(&self.format("%H:%M:%S"))
        }
    }
}
