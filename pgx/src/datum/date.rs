/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, IntoDatum};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};
use time::format_description::FormatItem;

#[derive(Debug)]
pub struct Date(time::Date);
impl FromDatum for Date {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<Date> {
        if is_null {
            None
        } else {
            let pg_epoch_days = datum.value() as i32;
            let date = match pg_epoch_days {
                i32::MIN => time::Date::MIN,
                i32::MAX => time::Date::MAX,
                _ => {
                    time::Date::from_julian_day(pg_epoch_days + pg_sys::POSTGRES_EPOCH_JDATE as i32)
                        .expect("Unexpected error getting the Julian day in Date::from_datum")
                }
            };
            Some(Date(date))
        }
    }
}
impl IntoDatum for Date {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let pg_epoch_days = match self.0 {
            time::Date::MIN => i32::MIN,
            time::Date::MAX => i32::MAX,
            _ => self.to_julian_day() as i32 - pg_sys::POSTGRES_EPOCH_JDATE as i32,
        };
        Some(pg_epoch_days.into())
    }

    fn type_oid() -> u32 {
        pg_sys::DATEOID
    }
}

impl Date {
    pub fn new(date: time::Date) -> Self {
        Date(date)
    }

    pub fn infinity() -> Self {
        Date(time::Date::MAX)
    }

    pub fn neg_infinity() -> Self {
        Date(time::Date::MIN)
    }

    #[inline]
    pub fn is_infinity(self) -> bool {
        self.0 == time::Date::MAX
    }

    #[inline]
    pub fn is_neg_infinity(self) -> bool {
        self.0 == time::Date::MIN
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

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &self.format(&DATE_FORMAT).expect("Date formatting problem")
        )
    }
}
