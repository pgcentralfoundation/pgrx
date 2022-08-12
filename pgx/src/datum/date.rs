/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, IntoDatum};
use std::ffi::CStr;

#[derive(Debug)]
pub struct Date(i32);

impl TryFrom<pg_sys::Datum> for Date {
    type Error = i32;
    fn try_from(d: pg_sys::Datum) -> Result<Self, Self::Error> {
        Ok(Date(d.value() as i32))
    }
}

impl IntoDatum for Date {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self.0))
    }
    fn type_oid() -> u32 {
        pg_sys::DATEOID
    }
}

impl FromDatum for Date {
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            Some(datum.try_into().expect("Error converting date datum"))
        }
    }
}

impl Date {
    pub const NEG_INFINITY: i32 = i32::MIN;
    pub const INFINITY: i32 = i32::MAX;

    #[inline]
    pub fn from_date(date: time::Date) -> Date {
        Self::from_pg_epoch_days(date.to_julian_day() - pg_sys::POSTGRES_EPOCH_JDATE as i32)
    }

    #[inline]
    pub fn from_pg_epoch_days(pg_epoch_days: i32) -> Date {
        Date(pg_epoch_days)
    }

    #[inline]
    pub fn is_infinity(&self) -> bool {
        self.0 == Self::INFINITY
    }

    #[inline]
    pub fn is_neg_infinity(&self) -> bool {
        self.0 == Self::NEG_INFINITY
    }

    #[inline]
    pub fn to_julian_day(&self) -> i32 {
        self.0
    }

    pub fn try_get_date(&self) -> Result<time::Date, i32> {
        const INNER_RANGE_BEGIN: i32 = time::Date::MIN.to_julian_day();
        const INNER_RANGE_END: i32 = time::Date::MAX.to_julian_day();
        match self.0 {
            INNER_RANGE_BEGIN..=INNER_RANGE_END => {
                time::Date::from_julian_day(self.0 + pg_sys::POSTGRES_EPOCH_JDATE as i32)
                    .or_else(|_e| Err(self.0))
            }
            v => Err(v),
        }
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
        let cstr;
        unsafe {
            let buf = [0u8; pg_sys::MAXDATELEN as usize + 1].as_mut_ptr() as *mut i8;
            match &self.0 {
                &Self::NEG_INFINITY | &Self::INFINITY => {
                    pg_sys::EncodeSpecialDate(self.0, buf);
                }
                _ => {
                    let mut pg_tm: pg_sys::pg_tm = Default::default();
                    pg_sys::j2date(
                        &self.0 + pg_sys::POSTGRES_EPOCH_JDATE as i32,
                        &mut pg_tm.tm_year,
                        &mut pg_tm.tm_mon,
                        &mut pg_tm.tm_mday,
                    );
                    pg_sys::EncodeDateOnly(&mut pg_tm, pg_sys::USE_XSD_DATES as i32, buf)
                }
            }
            cstr = CStr::from_ptr(pg_sys::pstrdup(buf));
        }

        serializer
            .serialize_str(cstr.to_str().unwrap())
            .map_err(|e| serde::ser::Error::custom(format!("Date formatting problem: {:?}", e)))
    }
}
