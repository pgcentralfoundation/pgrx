/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{direct_function_call, pg_sys, FromDatum, IntoDatum};
use pgrx_pg_sys::warning;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

pub const USECS_PER_SEC: i64 = 1_000_000;
pub const USECS_PER_DAY: i64 = pg_sys::SECS_PER_DAY as i64 * USECS_PER_SEC;

/// From the PG docs  https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-INTERVAL-INPUT
/// Internally interval values are stored as months, days, and microseconds. This is done because the number of days in a month varies,
/// and a day can have 23 or 25hours if a daylight savings time adjustment is involved. The months and days fields are integers while
/// the microseconds field can store fractional seconds. Because intervals are usually created from constant strings or timestamp
/// subtraction, this storage method works well in most cases...
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Interval(pub pg_sys::Interval);

impl Interval {
    /// This function takes `months`/`days`/`microseconds` as input to convert directly to the internal PG storage struct `pg_sys::Interval`
    /// - the sign of all units must be all matching in the positive or all matching in the negative direction
    pub fn new(months: i32, days: i32, micros: i64) -> Result<Self, IntervalConversionError> {
        if months < 0 {
            if days > 0 || micros > 0 {
                return Err(IntervalConversionError::MismatchedSigns);
            }
        } else if months > 0 {
            if days < 0 || micros < 0 {
                return Err(IntervalConversionError::MismatchedSigns);
            }
        }

        Ok(Interval(pg_sys::Interval { time: micros, day: days, month: months }))
    }

    /// Total number of months before/after 2000-01-01
    #[inline]
    pub fn months(&self) -> i32 {
        self.0.month
    }

    /// Total number of days before/after the `months()` offset (sign must match `months`)
    #[inline]
    pub fn days(&self) -> i32 {
        self.0.day
    }

    /// Total number of microseconds before/after the `days()` offset (sign must match `months`/`days`)
    #[inline]
    pub fn micros(&self) -> i64 {
        self.0.time
    }

    #[inline]
    pub fn total_micros(&self) -> i128 {
        self.micros() as i128
            + self.months() as i128 * pg_sys::DAYS_PER_MONTH as i128 * USECS_PER_DAY as i128
            + self.days() as i128 * USECS_PER_DAY as i128
    }

    #[inline]
    pub fn abs(self) -> Self {
        Interval(pg_sys::Interval {
            time: self.0.time.abs(),
            day: self.0.day.abs(),
            month: self.0.month.abs(),
        })
    }

    #[inline]
    pub fn signum(self) -> Self {
        if self.0.month == 0 && self.0.day == 0 && self.0.time == 0 {
            Interval(pg_sys::Interval { time: 0, day: 0, month: 0 })
        } else if self.is_positive() {
            Interval(pg_sys::Interval { time: 1, day: 0, month: 0 })
        } else {
            Interval(pg_sys::Interval { time: -1, day: 0, month: 0 })
        }
    }

    #[inline]
    pub fn is_positive(self) -> bool {
        !self.is_negative()
    }

    #[inline]
    pub fn is_negative(self) -> bool {
        self.0.month < 0 || self.0.day < 0 || self.0.time < 0
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
            // SAFETY:  Caller asserted the datum points to a pg_sys::Interval
            Some(Interval(ptr.read()))
        }
    }
}

impl IntoDatum for Interval {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        unsafe {
            let ptr =
                pg_sys::palloc(std::mem::size_of::<pg_sys::Interval>()).cast::<pg_sys::Interval>();
            ptr.write(self.0);
            Some(pg_sys::Datum::from(ptr))
        }
    }
    fn type_oid() -> pg_sys::Oid {
        pg_sys::INTERVALOID
    }
}

impl TryFrom<std::time::Duration> for Interval {
    type Error = IntervalConversionError;
    fn try_from(duration: std::time::Duration) -> Result<Interval, Self::Error> {
        let microseconds = duration.as_micros();
        let seconds = microseconds / USECS_PER_SEC as u128;
        let days = seconds / pg_sys::SECS_PER_DAY as u128;
        let months = days / pg_sys::DAYS_PER_MONTH as u128;
        let leftover_days = days - months * pg_sys::DAYS_PER_MONTH as u128;
        let leftover_microseconds = microseconds
            - (leftover_days * USECS_PER_DAY as u128
                + (months * pg_sys::DAYS_PER_MONTH as u128 * USECS_PER_DAY as u128));

        warning!("{}: {months} {leftover_days} {leftover_microseconds}", microseconds);
        Interval::new(
            months.try_into().map_err(|_| IntervalConversionError::DurationMonthsOutOfBounds)?,
            leftover_days.try_into().expect("bad math during Duration to Interval days"),
            leftover_microseconds.try_into().expect("bad math during Duration to Interval micros"),
        )
    }
}

impl TryFrom<Interval> for std::time::Duration {
    type Error = IntervalConversionError;

    fn try_from(interval: Interval) -> Result<Self, Self::Error> {
        if interval.0.time < 0 || interval.0.month < 0 || interval.0.day < 0 {
            return Err(IntervalConversionError::NegativeInterval);
        }

        let micros = interval.0.time as u128
            + interval.0.day as u128 * pg_sys::SECS_PER_DAY as u128 * USECS_PER_SEC as u128
            + interval.0.month as u128 * pg_sys::DAYS_PER_MONTH as u128 * USECS_PER_DAY as u128;

        Ok(std::time::Duration::from_micros(
            micros.try_into().map_err(|_| IntervalConversionError::IntervalTooLarge)?,
        ))
    }
}

impl serde::Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
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

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntervalConversionError {
    #[error("duration's total month count outside of valid i32::MIN..=i32::MAX range")]
    DurationMonthsOutOfBounds,
    #[error("Interval parts must all have the same sign")]
    MismatchedSigns,
    #[error("Negative Intervals cannot be converted into Durations")]
    NegativeInterval,
    #[error("Interval overflows Duration's u64 micros constructor")]
    IntervalTooLarge,
}
