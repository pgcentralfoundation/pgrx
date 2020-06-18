// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use crate::{pg_sys, FromDatum, IntoDatum};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Date(time::Date);
impl FromDatum for Date {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _typoid: u32) -> Option<Date> {
        if is_null {
            None
        } else {
            Some(Date(time::Date::from_julian_day(
                (datum as i32 + pg_sys::POSTGRES_EPOCH_JDATE as i32) as i64,
            )))
        }
    }
}
impl IntoDatum for Date {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some((self.julian_day() as i32 - pg_sys::POSTGRES_EPOCH_JDATE as i32) as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::DATEOID
    }
}

impl Date {
    pub fn new(date: time::Date) -> Self {
        Date(date)
    }
}

impl Deref for Date {
    type Target = time::Date;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Date {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl serde::Serialize for Date {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.format("%Y-%m-%d"))
    }
}
