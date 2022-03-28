/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, IntoDatum};
use std::ops::{Deref, DerefMut};
use time::format_description::FormatItem;

#[derive(Debug)]
pub struct Date(time::Date);
impl FromDatum for Date {
    const NEEDS_TYPID: bool = false;
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _typoid: u32) -> Option<Date> {
        if is_null {
            None
        } else {
            Some(Date(
                time::Date::from_julian_day(datum as i32 + pg_sys::POSTGRES_EPOCH_JDATE as i32)
                    .expect("Unexpected error getting the Julian day in Date::from_datum"),
            ))
        }
    }
}
impl IntoDatum for Date {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some((self.to_julian_day() as i32 - pg_sys::POSTGRES_EPOCH_JDATE as i32) as pg_sys::Datum)
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
        serializer.serialize_str(
            &self.format(&DATE_FORMAT).map_err(|e| {
                serde::ser::Error::custom(format!("Date formatting problem: {:?}", e))
            })?,
        )
    }
}

static DATE_FORMAT: &[FormatItem<'static>] =
    time::macros::format_description!("[year]-[month]-[day]");
