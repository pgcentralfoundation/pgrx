#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[cfg(test)]
    use crate as pgx_tests;
    use pgx::*;
    use std::num::NonZeroUsize;

    #[pg_test]
    fn test_new_composite_type() {
        Spi::run("CREATE TYPE dog AS (name text, age int);");
        let mut heap_tuple = PgHeapTuple::new_composite_type("dog").unwrap();

        assert_eq!(heap_tuple.get_by_name::<String>("name").unwrap(), None);
        assert_eq!(heap_tuple.get_by_name::<i32>("age").unwrap(), None);

        heap_tuple
            .set_by_name("name", "Brandy".to_string())
            .unwrap();
        heap_tuple.set_by_name("age", 42).unwrap();

        assert_eq!(
            heap_tuple.get_by_name("name").unwrap(),
            Some("Brandy".to_string())
        );
        assert_eq!(heap_tuple.get_by_name("age").unwrap(), Some(42i32));
    }

    #[pg_test]
    fn test_missing_type() {
        const NON_EXISTING_ATTRIBUTE: &str = "DEFINITELY_NOT_EXISTING";
        let attr_string = NON_EXISTING_ATTRIBUTE.to_string();

        match PgHeapTuple::new_composite_type(NON_EXISTING_ATTRIBUTE) {
            Err(PgHeapTupleError::NoSuchType(not_found)) if not_found == NON_EXISTING_ATTRIBUTE.to_string() => (),
            Err(err) => panic!("{}", err),
            Ok(_) => panic!("Able to find what should be a not existing composite type"),
        }
    }

    #[pg_test]
    fn test_missing_field() {
        Spi::run("CREATE TYPE dog AS (name text, age int);");
        let mut heap_tuple = PgHeapTuple::new_composite_type("dog").unwrap();

        const NON_EXISTING_ATTRIBUTE: &str = "DEFINITELY_NOT_EXISTING";
        assert_eq!(
            heap_tuple.get_by_name::<String>(NON_EXISTING_ATTRIBUTE),
            Err(TryFromDatumError::NoSuchAttributeName(NON_EXISTING_ATTRIBUTE.into())),
        );

        assert_eq!(
            heap_tuple
                .set_by_name(NON_EXISTING_ATTRIBUTE, "Brandy".to_string()),
            Err(TryFromDatumError::NoSuchAttributeName(NON_EXISTING_ATTRIBUTE.into())),
        );
    }

    #[pg_test]
    fn test_missing_number() {
        Spi::run("CREATE TYPE dog AS (name text, age int);");
        let mut heap_tuple = PgHeapTuple::new_composite_type("dog").unwrap();

        const NON_EXISTING_ATTRIBUTE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(9001) };
        assert_eq!(
            heap_tuple.get_by_index::<String>(NON_EXISTING_ATTRIBUTE),
            Err(TryFromDatumError::NoSuchAttributeNumber(NON_EXISTING_ATTRIBUTE)),
        );

        assert_eq!(
            heap_tuple
                .set_by_index(NON_EXISTING_ATTRIBUTE, "Brandy".to_string()),
            Err(TryFromDatumError::NoSuchAttributeNumber(NON_EXISTING_ATTRIBUTE)),
        );
    }

    #[pg_test]
    fn test_wrong_type_assumed() {
        Spi::run("CREATE TYPE dog AS (name text, age int);");
        let mut heap_tuple = PgHeapTuple::new_composite_type("dog").unwrap();

        // These are **deliberately** the wrong types.
        assert_eq!(
            heap_tuple.get_by_name::<i32>("name"),
            Ok(None), // We don't get an error here, yet...
        );
        assert_eq!(
            heap_tuple.get_by_name::<String>("age"),
            Ok(None), // We don't get an error here, yet...
        );

        // These are **deliberately** the wrong types.
        assert_eq!(
            heap_tuple
                .set_by_name("name", 1_i32),
            Err(TryFromDatumError::IncompatibleTypes),
        );
        assert_eq!(
            heap_tuple
                .set_by_name("age", "Brandy"),
            Err(TryFromDatumError::IncompatibleTypes),
        );


        // Now set them properly, to test that we get errors when they're set...
        heap_tuple
            .set_by_name("name", "Brandy".to_string())
            .unwrap();
        heap_tuple.set_by_name("age", 42).unwrap();

        // These are **deliberately** the wrong types.
        assert_eq!(
            heap_tuple.get_by_name::<i32>("name"),
            Err(TryFromDatumError::IncompatibleTypes),
        );
        assert_eq!(
            heap_tuple.get_by_name::<String>("age"),
            Err(TryFromDatumError::IncompatibleTypes),
        );
    }
}
