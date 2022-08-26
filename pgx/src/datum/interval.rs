/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use std::ops::{Mul, Sub};

use crate::{direct_function_call, pg_sys, FromDatum, IntoDatum, USECS_PER_DAY, USECS_PER_SEC};
use pg_sys::{DAYS_PER_MONTH, SECS_PER_DAY};
use time::Duration;

const MONTH_DURATION: Duration = Duration::days(DAYS_PER_MONTH as i64);

#[derive(Debug)]
#[repr(transparent)]
pub struct Interval(pg_sys::Interval);

impl Interval {
    pub fn try_from_months_days_usecs(
        months: i32,
        days: i8,
        usecs: i64,
    ) -> Result<Self, IntervalConversionError> {
        if days.abs() >= pg_sys::DAYS_PER_MONTH as i8 {
            return Err(IntervalConversionError::FromDaysOutOfBounds);
        }
        if usecs.abs() >= USECS_PER_DAY {
            return Err(IntervalConversionError::FromUSecOutOfBounds);
        }
        Ok(Interval(pg_sys::Interval {
            day: days as i32,
            month: months,
            time: usecs,
        }))
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

impl TryFrom<pg_sys::Datum> for Interval {
    type Error = &'static str;
    fn try_from(d: pg_sys::Datum) -> Result<Self, Self::Error> {
        Ok(Interval(unsafe { *d.ptr_cast() }))
    }
}

impl IntoDatum for Interval {
    fn into_datum(mut self) -> Option<pg_sys::Datum> {
        let interval = &mut self.0;
        Some(pg_sys::Datum::from(interval as *mut _))
    }
    fn type_oid() -> pg_sys::Oid {
        pg_sys::INTERVALOID
    }
}

impl FromDatum for Interval {
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

impl TryFrom<Duration> for Interval {
    type Error = IntervalConversionError;
    fn try_from(duration: Duration) -> Result<Interval, Self::Error> {
        let total_months = duration.whole_days() / (pg_sys::DAYS_PER_MONTH as i64);

        if total_months >= (i32::MIN as i64) && total_months <= (i32::MAX as i64) {
            let mut month = 0;
            let mut day = 0;
            let mut d = duration;

            if Duration::abs(d) >= MONTH_DURATION {
                month = d.whole_days() as i32 / (DAYS_PER_MONTH as i32);
                d = d.sub(MONTH_DURATION.mul(month));
            }

            if Duration::abs(d) >= Duration::DAY {
                day = d.whole_days() as i32;
                d = d.sub(Duration::DAY.mul(day));
            }

            let time = d.whole_microseconds() as i64;

            Ok(Interval(pg_sys::Interval { day, month, time }))
        } else {
            Err(IntervalConversionError::DurationMonthsOutOfBounds)
        }
    }
}

impl From<Interval> for Duration {
    fn from(interval: Interval) -> Duration {
        let interval = interval.0; // internal interval
        let sec = interval.time / USECS_PER_SEC;
        let fsec = ((interval.time - (sec * USECS_PER_SEC)) * 1000) as i32; // convert usec to nsec

        let mut duration = Duration::new(sec, fsec);

        if interval.month != 0 {
            duration = duration.saturating_add(MONTH_DURATION.mul(interval.month));
        }

        if interval.day != 0 {
            duration = duration.saturating_add(Duration::new(
                (interval.day * SECS_PER_DAY as i32) as i64,
                0,
            ));
        }

        duration
    }
}

impl serde::Serialize for Interval {
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

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum IntervalConversionError {
    #[error("duration's total month count outside of valid i32::MIN..=i32::MAX range")]
    DurationMonthsOutOfBounds,
    #[error("try_from_months_days_usecs's days abs count must be < DAYS_PER_MONTH (30)")]
    FromDaysOutOfBounds,
    #[error("try_from_months_days_usecs's usec abs count must be < USECS_PER_DAY")]
    FromUSecOutOfBounds,
}
