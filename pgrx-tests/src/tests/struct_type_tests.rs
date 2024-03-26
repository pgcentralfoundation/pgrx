//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use core::ffi::CStr;
use pgrx::pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use pgrx::prelude::*;
use pgrx::stringinfo::StringInfo;

use crate::get_named_capture;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Complex {
    x: f64,
    y: f64,
}

impl Eq for Complex {}
impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Complex {
    #[allow(dead_code)]
    pub fn random() -> PgBox<Complex> {
        unsafe {
            let mut c = PgBox::<Complex>::alloc0();
            c.x = rand::random();
            c.y = rand::random();
            c.into_pg_boxed()
        }
    }
}

extension_sql!(
    r#"CREATE TYPE complex;"#,
    name = "create_complex_shell_type",
    creates = [Type(Complex)]
);

unsafe impl SqlTranslatable for Complex {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("Complex"))
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("Complex")))
    }
}

#[pg_extern(immutable)]
fn complex_in(input: &CStr) -> PgBox<Complex, AllocatedByRust> {
    let input_as_str = input.to_str().unwrap();
    let re = regex::Regex::new(
        r#"(?P<x>[-+]?([0-9]*\.[0-9]+|[0-9]+)),\s*(?P<y>[-+]?([0-9]*\.[0-9]+|[0-9]+))"#,
    )
    .unwrap();
    let x = get_named_capture(&re, "x", input_as_str).unwrap();
    let y = get_named_capture(&re, "y", input_as_str).unwrap();
    let mut complex = unsafe { PgBox::<Complex>::alloc() };

    complex.x = str::parse::<f64>(&x).unwrap_or_else(|_| panic!("{x} isn't a f64"));
    complex.y = str::parse::<f64>(&y).unwrap_or_else(|_| panic!("{y} isn't a f64"));

    complex
}

#[pg_extern(immutable)]
fn complex_out(complex: PgBox<Complex>) -> &'static CStr {
    let mut sb = StringInfo::new();
    sb.push_str(&format!("{}, {}", complex.x, complex.y));
    unsafe { sb.leak_cstr() }
}

extension_sql!(
    r#"
CREATE TYPE complex (
   internallength = 16,
   input = complex_in,
   output = complex_out,
   alignment = double
);
"#,
    name = "create_complex_type",
    requires = ["create_complex_shell_type", complex_in, complex_out]
);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use crate::tests::struct_type_tests::Complex;
    use pgrx::prelude::*;

    #[pg_test]
    fn test_complex_in() -> Result<(), pgrx::spi::Error> {
        Spi::connect(|client| {
            let complex = client
                .select("SELECT '1.1,2.2'::complex;", None, None)?
                .first()
                .get_one::<PgBox<Complex>>()?
                .expect("datum was null");

            assert_eq!(&complex.x, &1.1);
            assert_eq!(&complex.y, &2.2);
            Ok(())
        })
    }

    #[pg_test]
    fn test_complex_out() {
        let string_val = Spi::get_one::<&str>("SELECT complex_out('1.1,2.2')::text");

        assert_eq!(string_val, Ok(Some("1.1, 2.2")));
    }

    #[pg_test]
    fn test_complex_from_text() -> Result<(), pgrx::spi::Error> {
        Spi::connect(|client| {
            let complex = client
                .select("SELECT '1.1, 2.2'::complex;", None, None)?
                .first()
                .get_one::<PgBox<Complex>>()?
                .expect("datum was null");

            assert_eq!(&complex.x, &1.1);
            assert_eq!(&complex.y, &2.2);
            Ok(())
        })
    }

    #[pg_test]
    fn test_complex_storage_and_retrieval() -> Result<(), pgrx::spi::Error> {
        let complex = Spi::connect(|mut client| {
            client.update(
                "CREATE TABLE complex_test AS SELECT s as id, (s || '.0, 2.0' || s)::complex as value FROM generate_series(1, 1000) s;\
                SELECT value FROM complex_test ORDER BY id;", None, None)?.first().get_one::<PgBox<Complex>>()
        })?.expect("datum was null");

        assert_eq!(&complex.x, &1.0);
        assert_eq!(&complex.y, &2.01);
        Ok(())
    }
}
