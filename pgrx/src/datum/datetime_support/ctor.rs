//! Exposes constructor methods for creating [`TimestampWithTimeZone`]s based on the various
//! ways Postgres likes to interpret the "current time".
use crate::{direct_function_call, pg_sys, Date, Timestamp, TimestampWithTimeZone};

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
