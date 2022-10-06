/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use core::ffi::CStr;
use pgx::prelude::*;
use pgx::{InOutFuncs, PgVarlena, PgVarlenaInOutFuncs, StringInfo};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Copy, Clone, PostgresType)]
#[pgvarlena_inoutfuncs]
pub struct VarlenaType {
    a: f32,
    b: f32,
    c: i64,
}

impl PgVarlenaInOutFuncs for VarlenaType {
    fn input(input: &CStr) -> PgVarlena<Self> where {
        let mut iter = input.to_str().unwrap().split(',');
        let (a, b, c) = (iter.next(), iter.next(), iter.next());

        let mut result = PgVarlena::<VarlenaType>::new();
        result.a = f32::from_str(a.unwrap()).expect("a is not a valid f32");
        result.b = f32::from_str(b.unwrap()).expect("b is not a valid f32");
        result.c = i64::from_str(c.unwrap()).expect("c is not a valid i64");
        result
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&format!("{},{},{}", self.a, self.b, self.c))
    }
}

#[derive(Serialize, Deserialize, PostgresType)]
#[inoutfuncs]
pub struct CustomTextFormatSerializedType {
    a: f32,
    b: f32,
    c: i64,
}

impl InOutFuncs for CustomTextFormatSerializedType {
    fn input(input: &CStr) -> Self {
        let mut iter = input.to_str().unwrap().split(',');
        let (a, b, c) = (iter.next(), iter.next(), iter.next());

        CustomTextFormatSerializedType {
            a: f32::from_str(a.unwrap()).expect("a is not a valid f32"),
            b: f32::from_str(b.unwrap()).expect("b is not a valid f32"),
            c: i64::from_str(c.unwrap()).expect("c is not a valid i64"),
        }
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&format!("{},{},{}", self.a, self.b, self.c))
    }
}

#[pg_extern(immutable)]
fn fn_takes_option(input: Option<CustomTextFormatSerializedType>) -> String {
    input.map_or("nothing".to_string(), |c| {
        let mut b = StringInfo::new();
        c.output(&mut b);
        b.to_string()
    })
}

#[derive(Serialize, Deserialize, PostgresType)]
pub struct JsonType {
    a: f32,
    b: f32,
    c: i64,
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use crate::tests::postgres_type_tests::{
        CustomTextFormatSerializedType, JsonType, VarlenaType,
    };
    use pgx::prelude::*;
    use pgx::PgVarlena;

    #[pg_test]
    fn test_mytype() {
        let result = Spi::get_one::<PgVarlena<VarlenaType>>("SELECT '1.0,2.0,3'::VarlenaType")
            .expect("SPI returned NULL");
        assert_eq!(result.a, 1.0);
        assert_eq!(result.b, 2.0);
        assert_eq!(result.c, 3);
    }

    #[pg_test]
    fn test_call_with_value() {
        let result = Spi::get_one::<String>(
            "SELECT fn_takes_option('1.0,2.0,3'::CustomTextFormatSerializedType);",
        );
        assert_eq!("1,2,3", result.unwrap());
    }

    #[pg_test]
    fn test_call_with_null() {
        let result = Spi::get_one::<String>("SELECT fn_takes_option(NULL);");
        assert_eq!(Some(String::from("nothing")), result);
    }

    #[pg_test]
    fn test_serializedtype() {
        let result = Spi::get_one::<CustomTextFormatSerializedType>(
            "SELECT '1.0,2.0,3'::CustomTextFormatSerializedType",
        )
        .expect("SPI returned NULL");
        assert_eq!(result.a, 1.0);
        assert_eq!(result.b, 2.0);
        assert_eq!(result.c, 3);
    }

    #[pg_test]
    fn test_jsontype() {
        let result = Spi::get_one::<JsonType>(r#"SELECT '{"a": 1.0, "b": 2.0, "c": 3}'::JsonType"#)
            .expect("SPI returned NULL");
        assert_eq!(result.a, 1.0);
        assert_eq!(result.b, 2.0);
        assert_eq!(result.c, 3);
    }
}
