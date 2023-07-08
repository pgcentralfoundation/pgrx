//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Exposes constructor methods for creating [`TimestampWithTimeZone`]s based on the various
//! ways Postgres likes to interpret the "current time".
use crate::{direct_function_call, pg_sys, Date, IntoDatum, Timestamp, TimestampWithTimeZone};

/// Current date and time (start of current transaction)
pub fn now() -> TimestampWithTimeZone {
    unsafe { pg_sys::GetCurrentTransactionStartTimestamp().try_into().unwrap() }
}

/// Current date and time (start of current transaction)
///
/// This is the same as [`now()`].
pub fn transaction_timestamp() -> TimestampWithTimeZone {
    now()
}

/// Current date and time (start of current statement)
pub fn statement_timestamp() -> TimestampWithTimeZone {
    unsafe { pg_sys::GetCurrentStatementStartTimestamp().try_into().unwrap() }
}

/// Get the current operating system time (changes during statement execution)
///
/// Result is in the form of a [`TimestampWithTimeZone`] value, and is expressed to the
/// full precision of the `gettimeofday()` syscall
pub fn clock_timestamp() -> TimestampWithTimeZone {
    unsafe { pg_sys::GetCurrentTimestamp().try_into().unwrap() }
}

pub enum TimestampPrecision {
    /// Resulting timestamp is given to the full available precision
    Full,

    /// Resulting timestamp to be rounded to that many fractional digits in the seconds field
    Rounded(i32),
}

/// Helper to convert a [`TimestampPrecision`] into a Postgres "typemod" integer
impl From<TimestampPrecision> for i32 {
    fn from(value: TimestampPrecision) -> Self {
        match value {
            TimestampPrecision::Full => -1,
            TimestampPrecision::Rounded(p) => p,
        }
    }
}

/// Current date (changes during statement execution)
pub fn current_date() -> Date {
    current_timestamp(TimestampPrecision::Full).into()
}

/// Current time (changes during statement execution)
pub fn current_time() -> Date {
    current_timestamp(TimestampPrecision::Full).into()
}

/// implements CURRENT_TIMESTAMP, CURRENT_TIMESTAMP(n)  (changes during statement execution)
pub fn current_timestamp(precision: TimestampPrecision) -> TimestampWithTimeZone {
    unsafe { pg_sys::GetSQLCurrentTimestamp(precision.into()).try_into().unwrap() }
}

/// implements LOCALTIMESTAMP, LOCALTIMESTAMP(n)
pub fn local_timestamp(precision: TimestampPrecision) -> Timestamp {
    unsafe { pg_sys::GetSQLLocalTimestamp(precision.into()).try_into().unwrap() }
}

/// Returns the current time as String (changes during statement execution)
pub fn time_of_day() -> String {
    unsafe { direct_function_call(pg_sys::timeofday, &[]).unwrap() }
}

/// Convert Unix epoch (seconds since 1970-01-01 00:00:00+00) to [`TimestampWithTimeZone`]
pub fn to_timestamp(epoch_seconds: f64) -> TimestampWithTimeZone {
    unsafe {
        direct_function_call(pg_sys::float8_timestamptz, &[epoch_seconds.into_datum()]).unwrap()
    }
}

/// “bins” the input timestamp into the specified interval (the stride) aligned with a specified origin.
///
/// `source` is a value expression of type [`Timestamp`].
/// `stride` is a value expression of type [`Interval`].
///
/// The return value is likewise of type [`Timestamp`], and it marks the beginning of the bin into
/// which the source is placed.
///
/// # Notes
///
/// Only available on Postgres v14 and greater.
///
/// In the case of full units (1 minute, 1 hour, etc.), it gives the same result as the analogous
/// `date_trunc()` function, but the difference is that [`date_bin()`] can truncate to an arbitrary
/// interval.
///
/// The stride interval must be greater than zero and cannot contain units of month or larger.
///
/// # Examples
///
/// ```sql
/// SELECT date_bin('15 minutes', TIMESTAMP '2020-02-11 15:44:17', TIMESTAMP '2001-01-01');
/// Result: 2020-02-11 15:30:00
///
/// SELECT date_bin('15 minutes', TIMESTAMP '2020-02-11 15:44:17', TIMESTAMP '2001-01-01 00:02:30');
/// Result: 2020-02-11 15:32:30
/// ```
#[cfg(any(features = "pg14", features = "pg15"))]
pub fn date_bin(
    stride: crate::datum::interval::Interval,
    source: Timestamp,
    origin: Timestamp,
) -> Timestamp {
    unsafe {
        direct_function_call(
            pg_sys::date_bin,
            &[stride.into_datum(), source.into_datum(), origin.into_datum()],
        )
        .unwrap()
    }
}
