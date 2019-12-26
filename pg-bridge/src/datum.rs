use crate::{pg_sys, vardata_any, varsize_any_exhdr, PgBox};
use pg_guard::PostgresStruct;
use std::convert::{Infallible, TryInto};
use std::fmt::Debug;
use std::marker::PhantomData;

pub struct PgDatum<T>(Option<pg_sys::Datum>, PhantomData<T>)
where
    T: Sized;

impl<T> PgDatum<T>
where
    T: Sized,
{
    #[inline]
    pub fn new(datum: pg_sys::Datum, is_null: bool) -> Self {
        PgDatum(if is_null { None } else { Some(datum) }, PhantomData)
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_none()
    }
}

impl<T> Into<pg_sys::Datum> for PgDatum<T> {
    #[inline]
    fn into(self) -> pg_sys::Datum {
        match self.0 {
            Some(datum) => datum,
            None => 0 as pg_sys::Datum,
        }
    }
}

impl<T> From<*mut T> for PgDatum<T>
where
    T: Sized + PostgresStruct,
{
    #[inline]
    fn from(val: *mut T) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl<T> From<PgBox<T>> for PgDatum<T>
where
    T: Sized + PostgresStruct + Debug,
{
    #[inline]
    fn from(val: PgBox<T>) -> Self {
        PgDatum::<T>::from(val.into_pg())
    }
}

//
// From trait implementations for built-in Postgres types
//

impl<'a> From<&'a pg_sys::varlena> for PgDatum<&'a pg_sys::varlena> {
    #[inline]
    fn from(val: &'a pg_sys::varlena) -> Self {
        PgDatum(
            Some(val as *const pg_sys::varlena as pg_sys::Datum),
            PhantomData,
        )
    }
}

impl<'a> From<&'a std::ffi::CString> for PgDatum<&'a std::ffi::CStr> {
    #[inline]
    fn from(val: &'a std::ffi::CString) -> Self {
        PgDatum(
            Some(val as *const std::ffi::CString as pg_sys::Datum),
            PhantomData,
        )
    }
}

//
// From trait implementations for primitive types
//

impl From<bool> for PgDatum<bool> {
    #[inline]
    fn from(val: bool) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<char> for PgDatum<char> {
    #[inline]
    fn from(val: char) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<i8> for PgDatum<i8> {
    #[inline]
    fn from(val: i8) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<i16> for PgDatum<i16> {
    #[inline]
    fn from(val: i16) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<i32> for PgDatum<i32> {
    #[inline]
    fn from(val: i32) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<i64> for PgDatum<i64> {
    #[inline]
    fn from(val: i64) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<u8> for PgDatum<u8> {
    #[inline]
    fn from(val: u8) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<u16> for PgDatum<u16> {
    #[inline]
    fn from(val: u16) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<u32> for PgDatum<u32> {
    #[inline]
    fn from(val: u32) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<u64> for PgDatum<u64> {
    #[inline]
    fn from(val: u64) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<f32> for PgDatum<f32> {
    #[inline]
    fn from(val: f32) -> Self {
        PgDatum(Some(f32::to_bits(val) as pg_sys::Datum), PhantomData)
    }
}

impl From<f64> for PgDatum<f64> {
    #[inline]
    fn from(val: f64) -> Self {
        PgDatum(Some(f64::to_bits(val) as pg_sys::Datum), PhantomData)
    }
}

//
// TryInto trait implementations for Postgres types
//

impl<'a> TryInto<&'a pg_sys::varlena> for PgDatum<&'a pg_sys::varlena> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<&'a pg_sys::varlena, Self::Error> {
        match self.0 {
            Some(datum) => Ok(unsafe { &*(datum as *const pg_sys::varlena) }),
            None => Err("Datum is NULL"),
        }
    }
}

impl<'a> TryInto<&'a str> for PgDatum<&'a pg_sys::varlena> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<&'a str, Self::Error> {
        match self.0 {
            Some(datum) => unsafe {
                let t = datum as *const pg_sys::varlena;
                let len = varsize_any_exhdr(t);
                let data = vardata_any(t);

                Ok(std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                    data as *mut u8,
                    len,
                )))
            },
            None => Err("Datum is NULL"),
        }
    }
}

impl<'a> TryInto<&'a std::ffi::CStr> for PgDatum<&'a std::ffi::CStr> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<&'a std::ffi::CStr, Self::Error> {
        match self.0 {
            Some(datum) => {
                let s = datum as *const std::os::raw::c_char;
                Ok(unsafe { std::ffi::CStr::from_ptr(s) })
            }
            None => Err("Datum is NULL"),
        }
    }
}

//
// TryInto trait implementations for primitive types
//

impl TryInto<bool> for PgDatum<bool> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<bool, Self::Error> {
        match self.0 {
            Some(datum) => Ok(datum != 0),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<char> for PgDatum<char> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<char, Self::Error> {
        match self.0 {
            Some(datum) => Ok(datum as u8 as char),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<i8> for PgDatum<i8> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<i8, Self::Error> {
        match self.0 {
            Some(datum) => Ok(datum as i8),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<i16> for PgDatum<i16> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<i16, Self::Error> {
        match self.0 {
            Some(datum) => Ok(datum as i16),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<i32> for PgDatum<i32> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<i32, Self::Error> {
        match self.0 {
            Some(datum) => Ok(datum as i32),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<i64> for PgDatum<i64> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<i64, Self::Error> {
        match self.0 {
            Some(datum) => Ok(datum as i64),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<u8> for PgDatum<u8> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<u8, Self::Error> {
        match self.0 {
            Some(datum) => Ok(datum as u8),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<u16> for PgDatum<u16> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<u16, Self::Error> {
        match self.0 {
            Some(datum) => Ok(datum as u16),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<u32> for PgDatum<u32> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<u32, Self::Error> {
        match self.0 {
            Some(datum) => Ok(datum as u32),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<u64> for PgDatum<u64> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<u64, Self::Error> {
        match self.0 {
            Some(datum) => Ok(datum as u64),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<f32> for PgDatum<f32> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<f32, Self::Error> {
        match self.0 {
            Some(datum) => Ok(f32::from_bits(datum as u32)),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryInto<f64> for PgDatum<f64> {
    type Error = (&'static str);

    #[inline]
    fn try_into(self) -> Result<f64, Self::Error> {
        match self.0 {
            Some(datum) => Ok(f64::from_bits(datum as u64)),
            None => Err("Datum is NULL"),
        }
    }
}

impl<T> TryInto<PgBox<T>> for PgDatum<PgBox<T>>
where
    T: Sized + PostgresStruct + Debug,
{
    type Error = Infallible;

    #[inline]
    fn try_into(self) -> Result<PgBox<T>, Self::Error> {
        Ok(PgBox::from_pg(
            if self.0.is_some() { self.0.unwrap() } else { 0 } as *mut T,
        ))
    }
}
