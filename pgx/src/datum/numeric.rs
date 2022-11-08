/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{direct_function_call, direct_function_call_as_datum, pg_sys, FromDatum, IntoDatum};
use pg_sys::errcodes::PgSqlErrorCode;
use pg_sys::{AsPgCStr, Datum, InvalidOid, PgTryBuilder};
use pgx_utils::sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
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
        let s = numeric_out.to_str().expect("numeric_out is not a valid UTF8 string");
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
                // if it doesn't raise an conversion error, then we're good
                PgTryBuilder::new(|| {
                    // this might throw, but that's okay
                    let datum = Numeric(v.clone()).into_datum().unwrap();

                    unsafe {
                        // and don't leak the 'inet' datum Postgres created
                        pg_sys::pfree(datum.cast_mut_ptr());
                    }

                    // we have it as a valid String
                    Ok(Numeric(v.clone()))
                })
                .catch_when(PgSqlErrorCode::ERRCODE_INVALID_TEXT_REPRESENTATION, |_| {
                    Err(Error::custom(format!("invalid Numeric value: {}", v)))
                })
                .execute()
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
    unsafe fn from_polymorphic_datum(
        datum: Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<Self>
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

unsafe impl<const PRECISION: i32, const SCALE: i32> SqlTranslatable for Numeric<PRECISION, SCALE> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        match (PRECISION, SCALE) {
            (0, 0) => Ok(SqlMapping::literal("NUMERIC")),
            (p, 0) => Ok(SqlMapping::As(format!("NUMERIC({p})"))),
            (p, s) => Ok(SqlMapping::As(format!("NUMERIC({p}, {s})"))),
        }
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        match (PRECISION, SCALE) {
            (0, 0) => Ok(Returns::One(SqlMapping::literal("NUMERIC"))),
            (p, 0) => Ok(Returns::One(SqlMapping::As(format!("NUMERIC({p})")))),
            (p, s) => Ok(Returns::One(SqlMapping::As(format!("NUMERIC({p}, {s})")))),
        }
    }
}
