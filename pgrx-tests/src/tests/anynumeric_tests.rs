use pgrx::{prelude::*, AnyNumeric};

#[pg_extern]
fn anynumeric_arg(numeric: AnyNumeric) -> AnyNumeric {
    numeric
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::{prelude::*, AnyNumeric};

    #[pg_test]
    fn test_anynumeric_arg() -> Result<(), pgrx::spi::Error> {
        let numeric = Spi::get_one_with_args::<AnyNumeric>(
            "SELECT anynumeric_arg($1);",
            vec![(PgBuiltInOids::INT4OID.oid(), 123.into_datum())],
        )?
        .map(|n| n.normalize().to_string());

        assert_eq!(numeric, Some("123".to_string()));

        Ok(())
    }
}
