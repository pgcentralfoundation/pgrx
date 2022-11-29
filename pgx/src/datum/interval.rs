/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use std::ops::{Mul, Sub};

use crate::datum::time::USECS_PER_SEC;
use crate::{direct_function_call, pg_sys, FromDatum, IntoDatum, PgBox};
use pg_sys::{DAYS_PER_MONTH, SECS_PER_DAY};
use pgx_utils::sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

#[cfg(feature = "time-crate")]
const MONTH_DURATION: time::Duration = time::Duration::days(DAYS_PER_MONTH as i64);

/// From the PG docs  https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-INTERVAL-INPUT
/// Internally interval values are stored as months, days, and microseconds. This is done because the number of days in a month varies,
/// and a day can have 23 or 25hours if a daylight savings time adjustment is involved. The months and days fields are integers while
/// the microseconds field can store fractional seconds. Because intervals are usually created from constant strings or timestamp
/// subtraction, this storage method works well in most cases...
#[derive(Debug)]
#[repr(transparent)]
pub struct Interval(pg_sys::Interval);

impl Interval {
    /// This function takes `months`/`days`/`usecs` as input to convert directly to the internal PG storage struct `pg_sys::Interval`
    /// - the sign of all units must be all matching in the positive or all matching in the negative direction
    pub fn try_from_months_days_usecs(
        months: i32,
        days: i32,
        usecs: i64,
    ) -> Result<Self, IntervalConversionError> {
        let mut interval = PgBox::<pg_sys::Interval>::alloc();
        interval.day = days;
        interval.month = months;
        interval.time = usecs;
        Ok(Interval(*interval))
    }

    /// Total number of months before/after 2000-01-01
    pub fn months(&self) -> i32 {
        self.0.month
    }

    /// Total number of days before/after the `months()` offset (sign must match `months`)
    pub fn days(&self) -> i32 {
        self.0.day
    }

    /// Total number of usecs before/after the `days()` offset (sign must match `months`/`days`)
    pub fn usecs(&self) -> i64 {
        self.0.time
    }
}

impl TryFrom<pg_sys::Datum> for Interval {
    type Error = &'static str;
    fn try_from(datum: pg_sys::Datum) -> Result<Self, Self::Error> {
        if datum.is_null() {
            return Err("NULL datum");
        }
        Ok(Interval(unsafe { *datum.cast_mut_ptr::<pg_sys::Interval>() }))
    }
}

impl IntoDatum for Interval {
    fn into_datum(mut self) -> Option<pg_sys::Datum> {
        let interval = &mut self.0;
        // assume interval was allocated by PgBox
        Some(pg_sys::Datum::from(interval as *mut _))
    }
    fn type_oid() -> pg_sys::Oid {
        pg_sys::INTERVALOID
    }
}

impl FromDatum for Interval {
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            Some(datum.try_into().ok()?)
        }
    }
}

#[cfg(feature = "time-crate")]
impl TryFrom<time::Duration> for Interval {
    type Error = IntervalConversionError;
    fn try_from(duration: time::Duration) -> Result<Interval, Self::Error> {
        let total_months = duration.whole_days() / (pg_sys::DAYS_PER_MONTH as i64);

        if total_months >= (i32::MIN as i64) && total_months <= (i32::MAX as i64) {
            let mut month = 0;
            let mut d = duration;

            if time::Duration::abs(d) >= MONTH_DURATION {
                month = total_months as i32;
                d = d.sub(MONTH_DURATION.mul(month));
            }

            let time = d.whole_microseconds() as i64;

            Interval::try_from_months_days_usecs(month, 0, time)
        } else {
            Err(IntervalConversionError::DurationMonthsOutOfBounds)
        }
    }
}

#[cfg(feature = "time-crate")]
impl From<Interval> for time::Duration {
    fn from(interval: Interval) -> time::Duration {
        let interval = interval.0; // internal interval
        let sec = interval.time / USECS_PER_SEC as i64;
        let fsec = ((interval.time - (sec * USECS_PER_SEC as i64)) * 1000) as i32; // convert usec to nsec

        let mut duration = time::Duration::new(sec, fsec);

        if interval.month != 0 {
            duration = duration.saturating_add(MONTH_DURATION.mul(interval.month));
        }

        if interval.day != 0 {
            duration = duration.saturating_add(time::Duration::new(
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

unsafe impl SqlTranslatable for Interval {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("interval"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("interval")))
    }
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum IntervalConversionError {
    #[error("duration's total month count outside of valid i32::MIN..=i32::MAX range")]
    DurationMonthsOutOfBounds,
}
