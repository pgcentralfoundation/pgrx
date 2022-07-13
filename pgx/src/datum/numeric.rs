/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{direct_function_call, pg_sys, FromDatum, IntoDatum};
use pgx_pg_sys::{pg_try, AsPgCStr, InvalidOid};
use serde::de::{Error, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Number;
use std::ffi::CStr;
use std::fmt;

pub const NUMERIC_MAX_PRECISION: i32 = 1000;
pub const NUMERIC_MAX_RESULT_SCALE: i32 = NUMERIC_MAX_PRECISION * 2;

#[derive(Debug)]
pub struct Numeric<const PRECISION: i32, const SCALE: i32>(pg_sys::Numeric);

#[inline(always)]
const fn make_typmod(precision: i32, scale: i32) -> i32 {
    ((precision << 16) | scale) + pg_sys::VARHDRSZ as i32
}

impl<const PRECISION: i32, const SCALE: i32> std::fmt::Display for Numeric<PRECISION, SCALE> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let numeric_out = unsafe {
            direct_function_call::<&CStr>(
                pg_sys::numeric_out,
                vec![Some(pg_sys::Datum::from(self.0))],
            )
            .unwrap()
        };
        let s = numeric_out
            .to_str()
            .expect("numeric_out is not a valid UTF8 string");
        fmt.write_str(s)
    }
}

impl<const PRECISION: i32, const SCALE: i32> Serialize for Numeric<PRECISION, SCALE> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de, const PRECISION: i32, const SCALE: i32> Deserialize<'de> for Numeric<PRECISION, SCALE> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumericVisitor<const PRECISION: i32, const SCALE: i32>;

        impl<'de, const PRECISION: i32, const SCALE: i32> Visitor<'de>
            for NumericVisitor<PRECISION, SCALE>
        {
            type Value = Numeric<PRECISION, SCALE>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a JSON number or a \"quoted JSON number\"")
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Numeric<PRECISION, SCALE>, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Numeric<PRECISION, SCALE>, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Numeric<PRECISION, SCALE>, E>
            where
                E: de::Error,
            {
                let result =
                    Number::from_f64(value).ok_or_else(|| de::Error::custom("not a JSON number"));
                match result {
                    Ok(num) => Ok(num.as_f64().unwrap().into()),
                    Err(e) => Err(e),
                }
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_string(v.to_owned())
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                // try to convert the provided String value into a Postgres Numeric Datum
                // if it doesn't raise an ERROR, then we're good
                unsafe {
                    pg_try(|| {
                        // this might throw, but that's okay
                        let numeric = v.clone().into();

                        // we have it as a valid String
                        Ok(numeric)
                    })
                    .unwrap_or(Err(Error::custom(format!("invalid Numeric value: {}", v))))
                }
            }
        }

        deserializer.deserialize_any(NumericVisitor)
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for i8 {
    fn into(self) -> Numeric<PRECISION, SCALE> {
        format!("{}", self).into()
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for i16 {
    #[inline]
    fn into(self) -> Numeric<PRECISION, SCALE> {
        unsafe {
            direct_function_call::<Numeric<PRECISION, SCALE>>(
                pg_sys::int2_numeric,
                vec![self.into_datum()],
            )
        }
        .unwrap()
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for i32 {
    #[inline]
    fn into(self) -> Numeric<PRECISION, SCALE> {
        unsafe {
            direct_function_call::<Numeric<PRECISION, SCALE>>(
                pg_sys::int4_numeric,
                vec![self.into_datum()],
            )
        }
        .unwrap()
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for i64 {
    #[inline]
    fn into(self) -> Numeric<PRECISION, SCALE> {
        unsafe {
            direct_function_call::<Numeric<PRECISION, SCALE>>(
                pg_sys::int8_numeric,
                vec![self.into_datum()],
            )
        }
        .unwrap()
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for u8 {
    #[inline]
    fn into(self) -> Numeric<PRECISION, SCALE> {
        format!("{}", self).into()
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for u16 {
    #[inline]
    fn into(self) -> Numeric<PRECISION, SCALE> {
        format!("{}", self).into()
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for u32 {
    #[inline]
    fn into(self) -> Numeric<PRECISION, SCALE> {
        format!("{}", self).into()
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for u64 {
    #[inline]
    fn into(self) -> Numeric<PRECISION, SCALE> {
        format!("{}", self).into()
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for f32 {
    #[inline]
    fn into(self) -> Numeric<PRECISION, SCALE> {
        unsafe {
            direct_function_call::<Numeric<PRECISION, SCALE>>(
                pg_sys::float4_numeric,
                vec![self.into_datum()],
            )
        }
        .unwrap()
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for f64 {
    #[inline]
    fn into(self) -> Numeric<PRECISION, SCALE> {
        unsafe {
            direct_function_call::<Numeric<PRECISION, SCALE>>(
                pg_sys::float8_numeric,
                vec![self.into_datum()],
            )
        }
        .unwrap()
    }
}

impl<const PRECISION: i32, const SCALE: i32> Into<Numeric<PRECISION, SCALE>> for String {
    #[inline]
    fn into(self) -> Numeric<PRECISION, SCALE> {
        unsafe {
            let s = self.as_pg_cstr();
            let numeric = direct_function_call::<Numeric<PRECISION, SCALE>>(
                pg_sys::numeric_in,
                vec![
                    Some(pg_sys::Datum::from(s)),
                    InvalidOid.into_datum(),
                    make_typmod(PRECISION, SCALE).into_datum(),
                ],
            )
            .unwrap();

            pg_sys::pfree(s as _);
            numeric
        }
    }
}

impl<const PRECISION: i32, const SCALE: i32> FromDatum for Numeric<PRECISION, SCALE> {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            let datum = pg_sys::pg_detoast_datum(datum.ptr_cast()) as pg_sys::Numeric;
            Some(Numeric(datum))
        }
    }
}

impl<const PRECISION: i32, const SCALE: i32> IntoDatum for Numeric<PRECISION, SCALE> {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self.0))
    }

    fn type_oid() -> u32 {
        pg_sys::NUMERICOID
    }
}
