/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use std::ops::{Mul, Sub};

use crate::{direct_function_call, pg_sys, FromDatum, IntoDatum, USECS_PER_SEC};
use pg_sys::{DAYS_PER_MONTH, SECS_PER_DAY};
use time::Duration;

#[derive(Debug)]
#[repr(transparent)]
pub struct PgInterval(pg_sys::Interval);

impl PgInterval {
    const MONTH_DURATION: Duration = Duration::days(DAYS_PER_MONTH as i64);

    pub fn get_duration(&self) -> Duration {
        let sec: i64;
        let interval = self.0;
        sec = interval.time / USECS_PER_SEC;
        let fsec = ((interval.time - (sec * USECS_PER_SEC)) * 1000) as i32; // convert usec to nsec
        let mut duration = Duration::new(sec, fsec);
        if interval.month != 0 {
            duration = duration.saturating_add(Self::MONTH_DURATION.mul(interval.month));
        }
        if interval.day != 0 {
            duration = duration.saturating_add(Duration::new(
                (interval.day * SECS_PER_DAY as i32) as i64,
                0,
            ));
        }

        duration
    }

    pub fn from_months_days_usecs(months: i32, days: i32, usecs: i64) -> Self {
        PgInterval(pg_sys::Interval {
            day: days,
            month: months,
            time: usecs,
        })
    }

    pub fn try_from_duration(duration: Duration) -> Result<Self, &'static str> {
        const INNER_RANGE_BEGIN: Duration =
            Duration::new(-178_000_000i64 * pg_sys::SECS_PER_YEAR as i64, 0);
        const INNER_RANGE_END: Duration =
            Duration::new(178_000_000i64 * pg_sys::SECS_PER_YEAR as i64, 0);
        if duration >= INNER_RANGE_BEGIN && duration <= INNER_RANGE_END {
            let mut month = 0;
            let mut day = 0;
            let mut d = duration;
            if Duration::abs(d) >= Self::MONTH_DURATION {
                month = d.whole_days() as i32 / (DAYS_PER_MONTH as i32);
                d = d.sub(Self::MONTH_DURATION.mul(month));
            }
            if Duration::abs(d) >= Duration::DAY {
                day = d.whole_days() as i32;
                d = d.sub(Duration::DAY.mul(day));
            }
            let time = d.whole_microseconds() as i64;

            Ok(PgInterval(pg_sys::Interval { day, month, time }))
        } else {
            Err("duration outside of -178,000,000 year to 178,000,000 year bound")
        }
    }

    pub fn months(&self) -> i32 {
        self.0.month
    }

    pub fn days(&self) -> i32 {
        self.0.day
    }

    pub fn usecs(&self) -> i64 {
        self.0.time
    }
}

impl TryFrom<pg_sys::Datum> for PgInterval {
    type Error = &'static str;

    fn try_from(d: pg_sys::Datum) -> Result<Self, Self::Error> {
        Ok(PgInterval(unsafe { *d.ptr_cast() }))
    }
}

impl IntoDatum for PgInterval {
    fn into_datum(mut self) -> Option<pg_sys::Datum> {
        let interval = &mut self.0;
        Some(pg_sys::Datum::from(interval as *mut _))
    }
    fn type_oid() -> pg_sys::Oid {
        pg_sys::INTERVALOID
    }
}

impl FromDatum for PgInterval {
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            Some(datum.try_into().expect("Error converting interval datum"))
        }
    }
}

impl serde::Serialize for PgInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let interval = &self.0;
        unsafe {
            let cstr = direct_function_call::<&std::ffi::CStr>(
                pg_sys::interval_out,
                vec![Some(pg_sys::Datum::from(interval as *const _))],
            )
            .expect("failed to convert interval to a cstring");

            serializer.serialize_str(cstr.to_str().unwrap())
        }
    }
}
