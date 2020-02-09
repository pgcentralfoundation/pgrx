use crate::{direct_function_call_as_datum, pg_sys, FromDatum, IntoDatum, PgBox};
use serde::Serializer;
use std::ops::Deref;
use time::UtcOffset;

pub type Date = time::Date;
impl FromDatum<Date> for Date {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _typoid: u32) -> Option<Date> {
        if is_null {
            None
        } else {
            Some(time::Date::from_julian_day(
                (datum as i32 + pg_sys::POSTGRES_EPOCH_JDATE as i32) as i64,
            ))
        }
    }
}

impl IntoDatum<time::Date> for time::Date {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some((self.julian_day() as i32 - pg_sys::POSTGRES_EPOCH_JDATE as i32) as pg_sys::Datum)
    }
}

const USECS_PER_HOUR: i64 = 3600000000;
const USECS_PER_MINUTE: i64 = 60000000;
const USECS_PER_SEC: i64 = 1000000;
const MINS_PER_HOUR: i64 = 60;
const SEC_PER_MIN: i64 = 60;

pub type Time = time::Time;

impl FromDatum<Time> for Time {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _typoid: u32) -> Option<Time> {
        if is_null {
            None
        } else {
            let mut time = datum as i64;

            let hour = time / USECS_PER_HOUR;
            time -= hour * USECS_PER_HOUR;

            let min = time / USECS_PER_MINUTE;
            time -= min * USECS_PER_MINUTE;

            let second = time / USECS_PER_SEC;
            time -= second * USECS_PER_SEC;

            let microsecond = time;

            Some(
                time::Time::try_from_hms_micro(
                    hour as u8,
                    min as u8,
                    second as u8,
                    microsecond as u32,
                )
                .expect("failed to convert time"),
            )
        }
    }
}

impl IntoDatum<time::Time> for time::Time {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let datum = ((((self.hour() as i64 * MINS_PER_HOUR + self.minute() as i64) * SEC_PER_MIN)
            + self.second() as i64)
            * USECS_PER_SEC)
            + self.microsecond() as i64;

        Some(datum as pg_sys::Datum)
    }
}

pub struct TimeWithTimeZone(time::Time);

impl FromDatum<TimeWithTimeZone> for TimeWithTimeZone {
    #[inline]
    unsafe fn from_datum(datum: usize, is_null: bool, typoid: u32) -> Option<TimeWithTimeZone> {
        if is_null {
            None
        } else {
            let timetz = PgBox::from_pg(datum as *mut pg_sys::TimeTzADT);

            let mut time = Time::from_datum(timetz.time as pg_sys::Datum, false, typoid)
                .expect("failed to convert TimeWithTimeZone");
            time += time::Duration::seconds(timetz.zone as i64);

            Some(TimeWithTimeZone(time))
        }
    }
}

impl Deref for TimeWithTimeZone {
    type Target = time::Time;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl serde::Serialize for TimeWithTimeZone {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl IntoDatum<TimeWithTimeZone> for TimeWithTimeZone {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let mut timetz = PgBox::<pg_sys::TimeTzADT>::alloc();
        timetz.zone = 0;
        timetz.time = self
            .0
            .into_datum()
            .expect("failed to convert timetz into datum") as i64;

        Some(timetz.into_pg() as pg_sys::Datum)
    }
}

pub type Timestamp = time::PrimitiveDateTime;

impl FromDatum<Timestamp> for Timestamp {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, typoid: u32) -> Option<Timestamp> {
        let ts: Option<TimestampWithTimeZone> =
            TimestampWithTimeZone::from_datum(datum, is_null, typoid);
        match ts {
            None => None,
            Some(ts) => {
                let date = ts.date();
                let time = ts.time();

                Some(Timestamp::new(date, time))
            }
        }
    }
}

impl IntoDatum<Timestamp> for Timestamp {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let year = self.year();
        let month = self.month() as i32;
        let mday = self.day() as i32;
        let hour = self.hour() as i32;
        let minute = self.minute() as i32;
        let second = self.second() as f64 + (self.microsecond() as f64 / USECS_PER_SEC as f64);

        direct_function_call_as_datum(
            pg_sys::make_timestamp,
            vec![
                year.into_datum(),
                month.into_datum(),
                mday.into_datum(),
                hour.into_datum(),
                minute.into_datum(),
                second.into_datum(),
            ],
        )
    }
}

pub type TimestampWithTimeZone = time::OffsetDateTime;

impl FromDatum<TimestampWithTimeZone> for TimestampWithTimeZone {
    #[inline]
    unsafe fn from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: u32,
    ) -> Option<TimestampWithTimeZone> {
        if is_null {
            None
        } else {
            let mut tm = pg_sys::pg_tm {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: 0,
                tm_gmtoff: 0,
                tm_zone: std::ptr::null_mut(),
            };
            let mut tz = 0i32;
            let mut fsec = 0 as pg_sys::fsec_t;
            let mut tzn = 0 as *const std::os::raw::c_char;
            pg_sys::timestamp2tm(
                datum as i64,
                &mut tz,
                &mut tm,
                &mut fsec,
                &mut tzn,
                std::ptr::null_mut(),
            );
            let date = time::Date::try_from_ymd(tm.tm_year, tm.tm_mon as u8, tm.tm_mday as u8)
                .expect("failed to create date from TimestampWithTimeZonez");

            let time = time::Time::try_from_hms_micro(
                tm.tm_hour as u8,
                tm.tm_min as u8,
                tm.tm_sec as u8,
                fsec as u32,
            )
            .expect("failed to create time from TimestampWithTimeZonez");

            Some(Timestamp::new(date, time).using_offset(UtcOffset::seconds(tz)))
        }
    }
}

impl IntoDatum<TimestampWithTimeZone> for TimestampWithTimeZone {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let year = self.year();
        let month = self.month() as i32;
        let mday = self.day() as i32;
        let hour = self.hour() as i32;
        let minute = self.minute() as i32;
        let second = self.second() as f64 + (self.microsecond() as f64 / USECS_PER_SEC as f64);

        direct_function_call_as_datum(
            pg_sys::make_timestamptz_at_timezone,
            vec![
                year.into_datum(),
                month.into_datum(),
                mday.into_datum(),
                hour.into_datum(),
                minute.into_datum(),
                second.into_datum(),
                "UTC".into_datum(),
            ],
        )
    }
}
