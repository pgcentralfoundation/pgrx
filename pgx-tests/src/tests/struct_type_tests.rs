/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgx::pgx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use pgx::prelude::*;
use pgx::stringinfo::StringInfo;
use pgx::AllocatedByRust;

use crate::get_named_capture;

#[derive(Debug)]
#[repr(C)]
struct Complex {
    x: f64,
    y: f64,
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
fn complex_in(input: &std::ffi::CStr) -> PgBox<Complex, AllocatedByRust> {
    let input_as_str = input.to_str().unwrap();
    let re = regex::Regex::new(
        r#"(?P<x>[-+]?([0-9]*\.[0-9]+|[0-9]+)),\s*(?P<y>[-+]?([0-9]*\.[0-9]+|[0-9]+))"#,
    )
    .unwrap();
    let x = get_named_capture(&re, "x", input_as_str).unwrap();
    let y = get_named_capture(&re, "y", input_as_str).unwrap();
    let mut complex = PgBox::<Complex>::alloc();

    complex.x = str::parse::<f64>(&x).unwrap_or_else(|_| panic!("{} isn't a f64", x));
    complex.y = str::parse::<f64>(&y).unwrap_or_else(|_| panic!("{} isn't a f64", y));

    complex
}

#[pg_extern(immutable)]
fn complex_out(complex: PgBox<Complex>) -> &'static std::ffi::CStr {
    let mut sb = StringInfo::new();
    sb.push_str(&format!("{}, {}", &complex.x, &complex.y));
    sb.into()
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
    use crate as pgx_tests;

    use crate::tests::struct_type_tests::Complex;
    use pgx::prelude::*;

    #[pg_test]
    fn test_complex_in() {
        Spi::execute(|client| {
            let complex = client
                .select("SELECT '1.1,2.2'::complex;", None, None)
                .first()
                .get_one::<PgBox<Complex>>()
                .unwrap();

            assert_eq!(&complex.x, &1.1);
            assert_eq!(&complex.y, &2.2);
        })
    }

    #[pg_test]
    fn test_complex_out() {
        let string_val = Spi::get_one::<&str>("SELECT complex_out('1.1,2.2')::text").unwrap();

        assert_eq!(string_val, "1.1, 2.2");
    }

    #[pg_test]
    fn test_complex_from_text() {
        Spi::execute(|client| {
            let complex = client
                .select("SELECT '1.1, 2.2'::complex;", None, None)
                .first()
                .get_one::<PgBox<Complex>>()
                .unwrap();

            assert_eq!(&complex.x, &1.1);
            assert_eq!(&complex.y, &2.2);
        });
    }

    #[pg_test]
    fn test_complex_storage_and_retrieval() {
        let complex = Spi::connect(|client| {
            client.update(
                "CREATE TABLE complex_test AS SELECT s as id, (s || '.0, 2.0' || s)::complex as value FROM generate_series(1, 1000) s;\
                SELECT value FROM complex_test ORDER BY id;", None, None).first().get_one::<PgBox<Complex>>()
        }).unwrap();

        assert_eq!(&complex.x, &1.0);
        assert_eq!(&complex.y, &2.01);
    }
}
