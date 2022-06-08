#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    use crate as pgx_tests;
    use pgx::*;

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
        assert!(PgHeapTuple::new_composite_type("nuthin").is_err());
    }
}
