use crate::datum::time::Time;
use crate::{pg_sys, FromDatum, IntoDatum, PgBox};
use std::ops::{Deref, DerefMut};
use time::ComponentRangeError;

pub struct TimeWithTimeZone(Time);
impl FromDatum<TimeWithTimeZone> for TimeWithTimeZone {
    #[inline]
    unsafe fn from_datum(datum: usize, is_null: bool, typoid: u32) -> Option<TimeWithTimeZone> {
        if is_null {
            None
        } else {
            let timetz = PgBox::from_pg(datum as *mut pg_sys::TimeTzADT);

            let mut time = Time::from_datum(timetz.time as pg_sys::Datum, false, typoid)
                .expect("failed to convert TimeWithTimeZone");
            time.0 += time::Duration::seconds(timetz.zone as i64);

            Some(TimeWithTimeZone(time))
        }
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

impl TimeWithTimeZone {
    pub fn new(time: time::Time) -> Self {
        TimeWithTimeZone(Time(time))
    }

    pub fn from_hmso(
        hour: u8,
        minute: u8,
        second: u8,
        offset: time::Duration,
    ) -> std::result::Result<Time, ComponentRangeError> {
        match time::Time::try_from_hms(hour, minute, second) {
            Ok(mut time) => {
                time = time - offset;
                Ok(Time(time)) //not sure this is right, it is a mystery
            }
            Err(e) => Err(e),
        }
    }
}

impl Deref for TimeWithTimeZone {
    type Target = time::Time;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TimeWithTimeZone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl serde::Serialize for TimeWithTimeZone {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.format("%h-%m-%s-%z"))
    }
}
