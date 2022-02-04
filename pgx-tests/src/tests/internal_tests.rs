#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;
    use pgx::*;

    #[pg_test]
    fn insert() {
        let mut val = Internal::default();
        assert_eq!(val.initialized(), false);

        let inner = unsafe { val.insert::<i32>(5) };

        assert_eq!(*inner, 5);
        assert_eq!(val.initialized(), true);

        let inner = unsafe { val.insert::<i32>(6) };

        assert_eq!(*inner, 6);
        assert_eq!(val.initialized(), true);
    }

    #[pg_test]
    fn get_or_insert_default() {
        let mut val = Internal::default();
        assert_eq!(val.initialized(), false);

        let inner = unsafe { val.get_or_insert_default::<i32>() };

        assert_eq!(*inner, 0);
        assert_eq!(val.initialized(), true);
    }

    #[pg_test]
    fn get_or_insert() {
        let mut val = Internal::default();
        assert_eq!(val.initialized(), false);

        let inner = unsafe { val.get_or_insert::<i32>(5) };

        assert_eq!(*inner, 5);
        assert_eq!(val.initialized(), true);
    }

    #[pg_test]
    fn get_or_insert_with() {
        let mut val = Internal::default();
        assert_eq!(val.initialized(), false);

        let inner = unsafe { val.get_or_insert_with(|| 5) };

        assert_eq!(*inner, 5);
        assert_eq!(val.initialized(), true);
    }
}
