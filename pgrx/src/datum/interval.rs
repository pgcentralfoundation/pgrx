/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use std::ptr::NonNull;

#[cfg(feature = "time-crate")]
use crate::{
    datum::time::USECS_PER_SEC,
    pg_sys::{DAYS_PER_MONTH, SECS_PER_DAY},
};
use crate::{direct_function_call, pg_sys, FromDatum, IntoDatum, PgBox};
use pgrx_sql_entity_graph::metadata::{
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
pub struct Interval(NonNull<pg_sys::Interval>);

impl Interval {
    /// This function takes `months`/`days`/`microseconds` as input to convert directly to the internal PG storage struct `pg_sys::Interval`
    /// - the sign of all units must be all matching in the positive or all matching in the negative direction
    pub fn try_from_months_days_micros(
        months: i32,
        days: i32,
        micros: i64,
    ) -> Result<Self, IntervalConversionError> {
        // SAFETY: `pg_sys::Interval` will be uninitialized, set all fields
        let mut interval = unsafe { PgBox::<pg_sys::Interval>::alloc() };
        interval.day = days;
        interval.month = months;
        interval.time = micros;
        let ptr = interval.into_pg();
        let non_null = NonNull::new(ptr).expect("pointer is null");
        Ok(Interval::from_ptr(non_null))
    }

    pub fn from_ptr(ptr: NonNull<pg_sys::Interval>) -> Self {
        Interval(ptr)
    }

    pub fn as_ptr(&self) -> NonNull<pg_sys::Interval> {
        self.0
    }

    /// Total number of months before/after 2000-01-01
    pub fn months(&self) -> i32 {
        // SAFETY: Validity asserted on construction
        unsafe { (*self.0.as_ptr()).month }
    }

    /// Total number of days before/after the `months()` offset (sign must match `months`)
    pub fn days(&self) -> i32 {
        // SAFETY: Validity asserted on construction
        unsafe { (*self.0.as_ptr()).day }
    }

    /// Total number of microseconds before/after the `days()` offset (sign must match `months`/`days`)
    pub fn micros(&self) -> i64 {
        // SAFETY: Validity asserted on construction
        unsafe { (*self.0.as_ptr()).time }
    }
}

impl FromDatum for Interval {
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            let ptr = datum.cast_mut_ptr::<pg_sys::Interval>();
            let non_null = NonNull::new(ptr).expect("ptr was null");
            Some(Interval(non_null))
        }
    }
}

impl IntoDatum for Interval {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let interval = self.0.as_ptr();
        Some(interval.into())
    }
    fn type_oid() -> pg_sys::Oid {
        pg_sys::INTERVALOID
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
                d = d - MONTH_DURATION * month;
            }

            let time = d.whole_microseconds() as i64;

            Interval::try_from_months_days_micros(month, 0, time)
        } else {
            Err(IntervalConversionError::DurationMonthsOutOfBounds)
        }
    }
}

#[cfg(feature = "time-crate")]
impl From<Interval> for time::Duration {
    fn from(interval: Interval) -> time::Duration {
        // SAFETY: Validity of interval's ptr asserted on construction
        unsafe {
            let interval = *interval.0.as_ptr(); // internal interval
            let sec = interval.time / USECS_PER_SEC as i64;
            let fsec = ((interval.time - (sec * USECS_PER_SEC as i64)) * 1000) as i32; // convert microseconds to nanonseconds

            let mut duration = time::Duration::new(sec, fsec);

            if interval.month != 0 {
                duration = duration.saturating_add(MONTH_DURATION * interval.month);
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
}

impl serde::Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unsafe {
            // SAFETY: Validity of ptr asserted on construction
            let interval = self.0.as_ptr();
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
