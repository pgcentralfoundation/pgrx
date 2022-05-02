/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{
    direct_function_call, direct_function_call_as_datum, pg_sys, void_mut_ptr, FromDatum, IntoDatum,
};
use pgx_pg_sys::pg_try;
use serde::de::{Error, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Number;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct Numeric(pub String);

impl std::fmt::Display for Numeric {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{}", self.0))
    }
}

impl<'de> Deserialize<'de> for Numeric {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumericVisitor;

        impl<'de> Visitor<'de> for NumericVisitor {
            type Value = Numeric;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a JSON number or a \"quoted JSON number\"")
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Numeric, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Numeric, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Numeric, E>
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
                        let datum = Numeric(v.clone()).into_datum().unwrap();

                        // and don't leak the NumericData datum Postgres created
                        pg_sys::pfree(datum as void_mut_ptr);

                        // we have it as a valid String
                        Ok(Numeric(v.clone()))
                    })
                    .unwrap_or(Err(Error::custom(format!("invalid Numeric value: {}", v))))
                }
            }
        }

        deserializer.deserialize_any(NumericVisitor)
    }
}

impl Into<Numeric> for i8 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for i16 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for i32 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for i64 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for u8 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for u16 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for u32 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for u64 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for f32 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for f64 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl FromDatum for Numeric {
    unsafe fn from_datum(datum: usize, is_null: bool, _typoid: u32) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            let cstr =
                direct_function_call::<&std::ffi::CStr>(pg_sys::numeric_out, vec![Some(datum)])
                    .expect("numeric_out returned null");
            Some(Numeric(cstr.to_str().unwrap().into()))
        }
    }
}

impl IntoDatum for Numeric {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let cstring =
            std::ffi::CString::new(self.0).expect("failed to convert numeric string into CString");
        let cstr = cstring.as_c_str();

        unsafe {
            direct_function_call_as_datum(
                pg_sys::numeric_in,
                vec![
                    cstr.into_datum(),
                    pg_sys::InvalidOid.into_datum(),
                    0i32.into_datum(),
                ],
            )
        }
    }

    fn type_oid() -> u32 {
        pg_sys::NUMERICOID
    }
}
