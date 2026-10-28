//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::datum::TryFromDatumError;
use pgrx::fn_call::{Arg, fn_call};
use pgrx::prelude::*;
use pgrx::{AllocatedByRust, AnyArray, AnyElement, Array, Json, direct_function_call};
use std::fmt::Debug;
use std::num::NonZeroUsize;

#[pg_extern]
fn anyarray_arg(array: AnyArray) -> Json {
    unsafe { direct_function_call::<Json>(pg_sys::array_to_json, &[array.into_datum()]) }
        .expect("conversion to json returned null")
}

#[pg_extern]
fn anyarray_iter_arg(array: AnyArray) -> Json {
    let mut vec = Vec::<_>::new();
    for el in &array {
        vec.push(el.expect("element is null").datum().value() as i64)
    }
    unsafe { direct_function_call::<Json>(pg_sys::array_to_json, &[vec.into_datum()]) }
        .expect("conversion to json returned null")
}

/// Reads each element after the iteration that yielded it has ended.
#[pg_extern]
fn anyarray_read_elements_after_iteration(array: AnyArray) -> Vec<String> {
    let elements: Vec<Option<AnyElement>> = (&array).into_iter().collect();
    elements
        .iter()
        .map(|element| {
            let element = element.expect("element is NULL");
            element.value::<String>().expect("element is text").expect("element is NULL")
        })
        .collect()
}

/// Returns the first element after the iteration that yielded it has ended.
#[pg_extern]
fn anyarray_first_element(array: AnyArray) -> Option<AnyElement> {
    (&array).into_iter().next().flatten()
}

/// Formats a checked read, or `incompatible` if the array's type does not match.
fn describe<T: Debug>(result: Result<Option<T>, TryFromDatumError>) -> String {
    match result {
        Ok(value) => format!("{value:?}"),
        Err(TryFromDatumError::IncompatibleTypes { .. }) => "incompatible".to_owned(),
        Err(error) => panic!("unexpected error: {error}"),
    }
}

#[pg_extern]
fn anyarray_value_as_array_of_i32(array: AnyArray) -> String {
    describe(array.value::<Array<i32>>().map(|a| a.map(|a| a.iter().collect::<Vec<_>>())))
}

#[pg_extern]
fn anyarray_value_as_vec_of_i32(array: AnyArray) -> String {
    describe(array.value::<Vec<i32>>())
}

#[pg_extern]
fn anyarray_value_as_vec_of_string(array: AnyArray) -> String {
    describe(array.value::<Vec<String>>())
}

#[pg_extern]
fn anyarray_value_as_string(array: AnyArray) -> String {
    describe(array.value::<String>())
}

/// Reads the first attribute of each record as `i32`.
#[pg_extern]
fn anyarray_value_as_records(array: AnyArray) -> String {
    describe(array.value::<Array<PgHeapTuple<AllocatedByRust>>>().map(|records| {
        records.map(|records| {
            records
                .iter()
                .map(|record| record.map(|r| r.get_by_index::<i32>(NonZeroUsize::MIN).unwrap()))
                .collect::<Vec<_>>()
        })
    }))
}

/// `Array<Array<i32>>::type_oid()` is InvalidOid, because int4[] has no array type.
#[pg_extern]
fn anyarray_value_as_array_of_arrays(array: AnyArray) -> String {
    let result = array.value::<Array<Array<i32>>>().map(|a| a.map(|a| a.len()));
    format!("oid {}: {}", array.oid(), describe(result))
}

/// Calls `reader` through `fn_call`, which passes no expression tree, so the AnyArray that
/// `reader` receives has InvalidOid for its type.
#[pg_extern]
fn anyarray_value_without_expression(array: AnyArray, reader: &str) -> String {
    fn_call::<String>(reader, &[&Arg::Value(array)])
        .expect("fn_call failed")
        .expect("reader returned NULL")
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use pgrx::datum::TryFromDatumError;
    use pgrx::prelude::*;
    use pgrx::{AnyArray, Json};
    use serde_json::*;

    #[pg_test]
    fn test_anyarray_arg() -> std::result::Result<(), pgrx::spi::Error> {
        let json = Spi::get_one::<Json>("SELECT anyarray_arg(ARRAY[1::integer,2,3]::integer[]);")?
            .expect("datum was null");
        assert_eq!(json.0, json! {[1,2,3]});
        Ok(())
    }

    #[pg_test]
    fn test_anyarray_iter_arg() -> std::result::Result<(), pgrx::spi::Error> {
        let json =
            Spi::get_one::<Json>("SELECT anyarray_iter_arg(ARRAY[1::integer,2,3]::integer[]);")?
                .expect("datum was null");
        assert_eq!(json.0, json! {[1,2,3]});
        Ok(())
    }

    /// A short array read from a table has a 1-byte header, and a long one may be compressed, so
    /// iteration reads a detoasted copy. The elements point into that copy.
    #[pg_test]
    fn test_anyarray_elements_outlive_iterator() -> std::result::Result<(), pgrx::spi::Error> {
        Spi::run("CREATE TABLE tests.anyarray_stored (id int, a text[])")?;
        Spi::run(
            "INSERT INTO tests.anyarray_stored VALUES
                (1, ARRAY['hello', 'world']),
                (2, ARRAY[repeat('x', 10000), 'y'])",
        )?;
        let read = |id: i32| {
            Spi::get_one::<Vec<String>>(&format!(
                "SELECT anyarray_read_elements_after_iteration(a) FROM tests.anyarray_stored WHERE id = {id}"
            ))
        };
        assert_eq!(read(1)?, Some(vec!["hello".to_owned(), "world".to_owned()]));
        assert_eq!(read(2)?, Some(vec!["x".repeat(10000), "y".to_owned()]));
        assert_eq!(
            Spi::get_one::<String>(
                "SELECT anyarray_first_element(a) FROM tests.anyarray_stored WHERE id = 1"
            )?,
            Some("hello".to_owned())
        );
        Ok(())
    }

    fn select_text(query: &str) -> std::result::Result<String, pgrx::spi::Error> {
        Ok(Spi::get_one::<String>(query)?.expect("query returned NULL"))
    }

    #[pg_test]
    fn test_anyarray_value_reads_matching_types() -> std::result::Result<(), pgrx::spi::Error> {
        assert_eq!(
            select_text("SELECT anyarray_value_as_array_of_i32(ARRAY[1, NULL, 3])")?,
            "Some([Some(1), None, Some(3)])"
        );
        assert_eq!(
            select_text("SELECT anyarray_value_as_vec_of_i32(ARRAY[1, 2])")?,
            "Some([1, 2])"
        );
        assert_eq!(
            select_text("SELECT anyarray_value_as_vec_of_string(ARRAY['a', 'b'])")?,
            r#"Some(["a", "b"])"#
        );

        Spi::run("CREATE TYPE tests.anyarray_pair AS (a int, b text)")?;
        assert_eq!(
            select_text(
                "SELECT anyarray_value_as_records(ARRAY[(1, 'x'), NULL, (3, 'z')]::tests.anyarray_pair[])"
            )?,
            "Some([Some(Some(1)), None, Some(Some(3))])"
        );
        // anonymous records: record[]
        assert_eq!(
            select_text("SELECT anyarray_value_as_records(ARRAY[ROW(4, 'x'), ROW(5, 'y')])")?,
            "Some([Some(Some(4)), Some(Some(5))])"
        );
        Ok(())
    }

    /// SPI frees a query's results when its connection closes, before `get_one` returns, so it
    /// must copy the array out of them.
    #[pg_test]
    fn test_anyarray_from_spi_outlives_connection() -> std::result::Result<(), pgrx::spi::Error> {
        let array =
            Spi::get_one::<AnyArray>("SELECT ARRAY[repeat('x', 40), 'y']")?.expect("array is NULL");
        let expected = vec!["x".repeat(40), "y".to_owned()];
        assert_eq!(array.value::<Vec<String>>(), Ok(Some(expected.clone())));
        let elements = (&array)
            .into_iter()
            .map(|element| element.expect("element is NULL").value::<String>())
            .collect::<std::result::Result<Vec<_>, _>>()?;
        assert_eq!(elements, expected.into_iter().map(Some).collect::<Vec<_>>());
        Ok(())
    }

    /// A call coerces a domain over an array to its base type before an anyarray parameter, but an
    /// SPI result keeps the domain.
    #[pg_test]
    fn test_anyarray_value_reads_domain_over_array() -> std::result::Result<(), pgrx::spi::Error> {
        Spi::run("CREATE DOMAIN tests.anyarray_int_list AS int[] CHECK (cardinality(VALUE) > 0)")?;
        let domain = Spi::get_one::<pg_sys::Oid>("SELECT 'tests.anyarray_int_list'::regtype::oid")?
            .expect("domain oid");
        Spi::connect(|client| {
            let table =
                client.select("SELECT ARRAY[4, 5]::tests.anyarray_int_list", None, &[])?.first();
            // The handle refers to the SPI result, which stays allocated until this closure
            // returns.
            let array = table.get_one::<AnyArray>()?.expect("array is NULL");
            assert_eq!(array.oid(), domain);
            assert_eq!(array.value::<Vec<i32>>(), Ok(Some(vec![4, 5])));
            assert!(matches!(
                array.value::<Vec<String>>(),
                Err(TryFromDatumError::IncompatibleTypes { .. })
            ));
            // Each element has the domain's element type.
            let elements = (&array)
                .into_iter()
                .map(|element| element.expect("element is NULL").value::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            assert_eq!(elements, vec![Some(4), Some(5)]);
            Ok(())
        })
    }

    #[pg_test]
    fn test_anyarray_value_rejects_other_types() -> std::result::Result<(), pgrx::spi::Error> {
        // Reading these elements as records or strings would treat each integer as a pointer.
        assert_eq!(select_text("SELECT anyarray_value_as_records(ARRAY[7])")?, "incompatible");
        assert_eq!(
            select_text("SELECT anyarray_value_as_vec_of_string(ARRAY[7])")?,
            "incompatible"
        );
        // Reading the array itself as text would expose the array header and elements as a str.
        assert_eq!(select_text("SELECT anyarray_value_as_string(ARRAY[-1])")?, "incompatible");
        assert_eq!(
            select_text("SELECT anyarray_value_as_array_of_i32(ARRAY[1]::bigint[])")?,
            "incompatible"
        );
        Ok(())
    }

    #[pg_test]
    fn test_anyarray_value_rejects_unresolved_type() -> std::result::Result<(), pgrx::spi::Error> {
        assert_eq!(
            select_text("SELECT anyarray_value_as_array_of_arrays(ARRAY[7])")?,
            "oid 1007: incompatible"
        );
        assert_eq!(
            select_text(
                "SELECT anyarray_value_without_expression(ARRAY[7], 'anyarray_value_as_array_of_arrays')"
            )?,
            "oid 0: incompatible"
        );
        assert_eq!(
            select_text(
                "SELECT anyarray_value_without_expression(ARRAY[7], 'anyarray_value_as_vec_of_i32')"
            )?,
            "incompatible"
        );
        Ok(())
    }
}
