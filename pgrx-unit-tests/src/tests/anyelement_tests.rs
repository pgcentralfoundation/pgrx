use pgrx::datum::TryFromDatumError;
use pgrx::{AllocatedByRust, AnyElement, prelude::*};
use std::fmt::Debug;

#[pg_extern]
fn anyelement_arg(element: AnyElement) -> AnyElement {
    element
}

/// Formats a checked read, or `incompatible` if the element's type does not match.
fn describe<T: Debug>(result: Result<Option<T>, TryFromDatumError>) -> String {
    match result {
        Ok(value) => format!("{value:?}"),
        Err(TryFromDatumError::IncompatibleTypes { .. }) => "incompatible".to_owned(),
        Err(error) => panic!("unexpected error: {error}"),
    }
}

#[pg_extern]
fn anyelement_value_as_i32(element: AnyElement) -> String {
    describe(element.value::<i32>())
}

#[pg_extern]
fn anyelement_value_as_string(element: AnyElement) -> String {
    describe(element.value::<String>())
}

#[pg_extern]
fn anyelement_value_as_vec_of_i32(element: AnyElement) -> String {
    describe(element.value::<Vec<i32>>())
}

/// Reads the record's attribute count.
#[pg_extern]
fn anyelement_value_as_record(element: AnyElement) -> String {
    describe(element.value::<PgHeapTuple<AllocatedByRust>>().map(|r| r.map(|r| r.len())))
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use pgrx::{AllocatedByRust, AnyElement, datum::DatumWithOid, prelude::*};
    use std::num::NonZeroUsize;

    #[pg_test]
    fn test_anyelement_arg() -> Result<(), pgrx::spi::Error> {
        let oid = unsafe { DatumWithOid::new(123, AnyElement::type_oid()) };
        let element = Spi::get_one_with_args::<AnyElement>("SELECT anyelement_arg($1);", &[oid])?
            .map(|e| e.datum());

        assert_eq!(element, 123.into_datum());

        Ok(())
    }

    fn select_text(query: &str) -> Result<String, pgrx::spi::Error> {
        Ok(Spi::get_one::<String>(query)?.expect("query returned NULL"))
    }

    #[pg_test]
    fn test_anyelement_value_reads_matching_types() -> Result<(), pgrx::spi::Error> {
        assert_eq!(select_text("SELECT anyelement_value_as_i32(7)")?, "Some(7)");
        assert_eq!(select_text("SELECT anyelement_value_as_string('x'::text)")?, r#"Some("x")"#);
        assert_eq!(select_text("SELECT anyelement_value_as_string('x'::varchar)")?, r#"Some("x")"#);
        assert_eq!(select_text("SELECT anyelement_value_as_record(ROW(1, 'x'))")?, "Some(2)");

        // Unlike anyarray, an anyelement parameter keeps the domain type of its argument.
        Spi::run("CREATE DOMAIN tests.anyelement_int_list AS int[]")?;
        assert_eq!(
            select_text(
                "SELECT anyelement_value_as_vec_of_i32(ARRAY[4, 5]::tests.anyelement_int_list)"
            )?,
            "Some([4, 5])"
        );
        Ok(())
    }

    /// SPI frees a query's results when its connection closes, before `get_one` returns, so it
    /// must copy a pass-by-reference value out of them.
    #[pg_test]
    fn test_anyelement_from_spi_outlives_connection() -> Result<(), pgrx::spi::Error> {
        let text = Spi::get_one::<AnyElement>("SELECT repeat('x', 40)")?.expect("text is NULL");
        assert_eq!(text.value::<String>(), Ok(Some("x".repeat(40))));

        let point = Spi::get_one::<AnyElement>("SELECT point(1, 2)")?.expect("point is NULL");
        let point = point.value::<pg_sys::Point>()?.expect("point is NULL");
        assert_eq!((point.x, point.y), (1.0, 2.0));

        let record = Spi::get_one::<AnyElement>("SELECT ROW(3, 'x')")?.expect("record is NULL");
        let record = record.value::<PgHeapTuple<AllocatedByRust>>()?.expect("record is NULL");
        assert_eq!(record.get_by_index::<i32>(NonZeroUsize::MIN)?, Some(3));

        let int = Spi::get_one::<AnyElement>("SELECT 7")?.expect("int is NULL");
        assert_eq!(int.value::<i32>(), Ok(Some(7)));
        Ok(())
    }

    #[pg_test]
    fn test_anyelement_value_rejects_other_types() -> Result<(), pgrx::spi::Error> {
        // Each of these reads would treat the integer as a pointer.
        assert_eq!(select_text("SELECT anyelement_value_as_string(7)")?, "incompatible");
        assert_eq!(select_text("SELECT anyelement_value_as_vec_of_i32(7)")?, "incompatible");
        assert_eq!(select_text("SELECT anyelement_value_as_record(7)")?, "incompatible");
        assert_eq!(select_text("SELECT anyelement_value_as_i32(ARRAY[7])")?, "incompatible");
        Ok(())
    }
}
