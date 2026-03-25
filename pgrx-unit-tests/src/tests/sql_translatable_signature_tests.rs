use super::complex::Complex;
use super::enum_type_tests::Foo;
use super::fcinfo_tests::NullStrict;
use pgrx::nullable::Nullable;
use pgrx::prelude::*;
use pgrx::{AnyNumeric, Inet, Internal, Json, JsonB, Uuid};

macro_rules! identity_signature {
    ($name:ident, $ty:ty) => {
        #[pg_extern]
        fn $name(value: $ty) -> $ty {
            value
        }
    };
}

identity_signature!(schema_key_signature_bool, bool);
identity_signature!(schema_key_signature_i8, i8);
identity_signature!(schema_key_signature_i16, i16);
identity_signature!(schema_key_signature_i32, i32);
identity_signature!(schema_key_signature_i64, i64);
identity_signature!(schema_key_signature_f32, f32);
identity_signature!(schema_key_signature_f64, f64);
identity_signature!(schema_key_signature_text, String);
identity_signature!(schema_key_signature_bytea, Vec<u8>);
identity_signature!(schema_key_signature_vec_i32, Vec<i32>);
identity_signature!(schema_key_signature_option_i32, Option<i32>);
identity_signature!(schema_key_signature_nullable_i32, Nullable<i32>);
identity_signature!(schema_key_signature_date, Date);
identity_signature!(schema_key_signature_time, Time);
identity_signature!(schema_key_signature_timestamp, Timestamp);
identity_signature!(schema_key_signature_timestamptz, TimestampWithTimeZone);
identity_signature!(schema_key_signature_timetz, TimeWithTimeZone);
identity_signature!(schema_key_signature_interval, Interval);
identity_signature!(schema_key_signature_anynumeric, AnyNumeric);
identity_signature!(schema_key_signature_numeric_10_2, Numeric<10, 2>);
identity_signature!(schema_key_signature_json, Json);
identity_signature!(schema_key_signature_jsonb, JsonB);
identity_signature!(schema_key_signature_uuid, Uuid);
identity_signature!(schema_key_signature_inet, Inet);
identity_signature!(schema_key_signature_internal, Internal);
identity_signature!(schema_key_signature_range_i32, Range<i32>);
identity_signature!(schema_key_signature_range_i64, Range<i64>);
identity_signature!(schema_key_signature_range_numeric, Range<AnyNumeric>);
identity_signature!(schema_key_signature_range_date, Range<Date>);
identity_signature!(schema_key_signature_range_timestamp, Range<Timestamp>);
identity_signature!(schema_key_signature_range_timestamptz, Range<TimestampWithTimeZone>);
identity_signature!(schema_key_signature_enum, Foo);
identity_signature!(schema_key_signature_null_strict, NullStrict);
identity_signature!(schema_key_signature_complex, PgBox<Complex>);

#[pg_extern]
fn schema_key_signature_setof_return() -> SetOfIterator<'static, i32> {
    SetOfIterator::once(1)
}

#[pg_extern]
fn schema_key_signature_table_return() -> TableIterator<'static, (name!(id, i32),)> {
    TableIterator::once((1,))
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use pgrx::prelude::*;
    use std::collections::BTreeMap;

    const EXPECTED_SIGNATURES: &[(&str, &str, &str)] = &[
        ("schema_key_signature_anynumeric", "numeric", "numeric"),
        ("schema_key_signature_bool", "boolean", "boolean"),
        ("schema_key_signature_bytea", "bytea", "bytea"),
        ("schema_key_signature_complex", "complex", "complex"),
        ("schema_key_signature_date", "date", "date"),
        ("schema_key_signature_enum", "foo", "foo"),
        ("schema_key_signature_f32", "real", "real"),
        ("schema_key_signature_f64", "double precision", "double precision"),
        ("schema_key_signature_i16", "smallint", "smallint"),
        ("schema_key_signature_i32", "integer", "integer"),
        ("schema_key_signature_i64", "bigint", "bigint"),
        ("schema_key_signature_i8", "\"char\"", "\"char\""),
        ("schema_key_signature_inet", "inet", "inet"),
        ("schema_key_signature_internal", "internal", "internal"),
        ("schema_key_signature_interval", "interval", "interval"),
        ("schema_key_signature_json", "json", "json"),
        ("schema_key_signature_jsonb", "jsonb", "jsonb"),
        ("schema_key_signature_null_strict", "nullstrict", "nullstrict"),
        ("schema_key_signature_nullable_i32", "integer", "integer"),
        ("schema_key_signature_numeric_10_2", "numeric", "numeric"),
        ("schema_key_signature_option_i32", "integer", "integer"),
        ("schema_key_signature_range_date", "daterange", "daterange"),
        ("schema_key_signature_range_i32", "int4range", "int4range"),
        ("schema_key_signature_range_i64", "int8range", "int8range"),
        ("schema_key_signature_range_numeric", "numrange", "numrange"),
        ("schema_key_signature_range_timestamp", "tsrange", "tsrange"),
        ("schema_key_signature_range_timestamptz", "tstzrange", "tstzrange"),
        ("schema_key_signature_setof_return", "", "SETOF integer"),
        ("schema_key_signature_table_return", "", "TABLE(id integer)"),
        ("schema_key_signature_text", "text", "text"),
        ("schema_key_signature_time", "time without time zone", "time without time zone"),
        (
            "schema_key_signature_timestamp",
            "timestamp without time zone",
            "timestamp without time zone",
        ),
        (
            "schema_key_signature_timestamptz",
            "timestamp with time zone",
            "timestamp with time zone",
        ),
        (
            "schema_key_signature_timetz",
            "time with time zone",
            "time with time zone",
        ),
        ("schema_key_signature_uuid", "uuid", "uuid"),
        ("schema_key_signature_vec_i32", "integer[]", "integer[]"),
    ];

    #[pg_test]
    fn signature_matrix_functions_are_installed() -> spi::Result<()> {
        let actual = Spi::connect(|client| {
            let mut signatures = BTreeMap::new();
            let table = client.select(
                "SELECT proname::text AS proname, \
                        pg_get_function_identity_arguments(oid) AS identity_args, \
                        pg_get_function_result(oid) AS result \
                   FROM pg_proc \
                  WHERE proname LIKE 'schema_key_signature_%' \
                  ORDER BY proname",
                None,
                &[],
            )?;

            for row in table {
                let proname = row
                    .get_by_name::<String, _>("proname")?
                    .expect("signature query returned a null proname");
                let identity_args = row
                    .get_by_name::<String, _>("identity_args")?
                    .expect("signature query returned null identity args");
                let result = row
                    .get_by_name::<String, _>("result")?
                    .expect("signature query returned a null result");

                signatures.insert(proname, (identity_args, result));
            }

            Ok::<_, spi::Error>(signatures)
        })?;

        let expected = EXPECTED_SIGNATURES
            .iter()
            .map(|(proname, identity_args, result)| {
                let identity_args = if identity_args.is_empty() {
                    String::new()
                } else {
                    format!("value {identity_args}")
                };
                (
                    (*proname).to_string(),
                    (identity_args, (*result).to_string()),
                )
            })
            .collect::<BTreeMap<_, _>>();

        assert_eq!(actual, expected);

        let setof_rows =
            Spi::get_one::<i64>("SELECT count(*) FROM schema_key_signature_setof_return();")?;
        assert_eq!(setof_rows, Some(1));

        let table_rows =
            Spi::get_one::<i64>("SELECT count(*) FROM schema_key_signature_table_return();")?;
        assert_eq!(table_rows, Some(1));

        Ok(())
    }
}
