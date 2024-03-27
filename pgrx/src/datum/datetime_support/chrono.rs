//! This module contains implementations and functionality that enables [`pgrx`] types (ex. [`pgrx::datum::Date`])
//! to be converted to [`chrono`] data types (ex. [`chrono::Date`])
#![cfg(feature = "chrono")]

use std::convert::TryFrom;

use chrono;

use crate::datum::datetime_support::DateTimeConversionError;
use crate::datum::Date;

impl TryFrom<Date> for chrono::NaiveDate {
    type Error = DateTimeConversionError;

    fn try_from(d: Date) -> Result<chrono::NaiveDate, DateTimeConversionError> {
        chrono::NaiveDate::from_ymd_opt(d.year(), d.month().into(), d.day().into())
            .ok_or_else(|| DateTimeConversionError::InvalidFormat)
    }
}

impl TryFrom<chrono::NaiveDate> for Date {
    type Error = DateTimeConversionError;

    fn try_from(d: chrono::NaiveDate) -> Result<Date, DateTimeConversionError> {
        Date::new(d.year(), d.month().into(), d.day().into())
    }
}
