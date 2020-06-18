// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use crate::datum::time::USECS_PER_SEC;
use crate::{direct_function_call_as_datum, pg_sys, FromDatum, IntoDatum, TimestampWithTimeZone};
use std::ops::{Deref, DerefMut};
use time::PrimitiveDateTime;

#[derive(Debug)]
pub struct Timestamp(time::PrimitiveDateTime);
impl FromDatum for Timestamp {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, typoid: u32) -> Option<Timestamp> {
        let ts: Option<TimestampWithTimeZone> =
            TimestampWithTimeZone::from_datum(datum, is_null, typoid);
        match ts {
            None => None,
            Some(ts) => {
                let date = ts.date();
                let time = ts.time();

                Some(Timestamp(PrimitiveDateTime::new(date, time)))
            }
        }
    }
}
impl IntoDatum for Timestamp {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let year = self.year();
        let month = self.month() as i32;
        let mday = self.day() as i32;
        let hour = self.hour() as i32;
        let minute = self.minute() as i32;
        let second = self.second() as f64 + (self.microsecond() as f64 / USECS_PER_SEC as f64);

        direct_function_call_as_datum(
            pg_sys::make_timestamp,
            vec![
                year.into_datum(),
                month.into_datum(),
                mday.into_datum(),
                hour.into_datum(),
                minute.into_datum(),
                second.into_datum(),
            ],
        )
    }

    fn type_oid() -> u32 {
        pg_sys::TIMESTAMPOID
    }
}
impl Timestamp {
    pub fn new(timestamp: time::PrimitiveDateTime) -> Self {
        Timestamp(timestamp)
    }
}

impl Deref for Timestamp {
    type Target = time::PrimitiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Timestamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        if self.millisecond() > 0 {
            serializer.serialize_str(
                &self.format(&format!("%Y-%m-%dT%H:%M:%S.{}-00", self.millisecond())),
            )
        } else {
            serializer.serialize_str(&self.format("%Y-%m-%dT%H:%M:%S-00"))
        }
    }
}
