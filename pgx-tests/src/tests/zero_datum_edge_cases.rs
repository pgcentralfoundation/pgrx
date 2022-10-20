#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;
    use pgx::*;

    fn from_helper<T: FromDatum + IntoDatum>(d: Datum) -> Option<T> {
        unsafe { T::from_polymorphic_datum(d, false, pg_sys::InvalidOid) }
    }

    #[pg_test]
    fn test_two_tuple_bool() {
        let d = (Some(false), Some(true)).into_datum().unwrap();
        let (a, b) = from_helper::<(Option<bool>, Option<bool>)>(d).unwrap();
        assert_eq!((a, b), (Some(false), Some(true)));
    }

    #[pg_test]
    fn test_vec_bool() {
        let d = vec![Some(false).into_datum(), Some(true).into_datum()].into_datum().unwrap();
        let a = from_helper::<Vec<Option<pg_sys::Datum>>>(d).unwrap();
        assert_eq!(a.first().unwrap().is_some(), true);
    }

    #[pg_test]
    fn test_vec_ints() {
        let d = vec![Some(0).into_datum(), Some(1).into_datum()].into_datum().unwrap();
        let a = from_helper::<Vec<Option<pg_sys::Datum>>>(d).unwrap();
        assert_eq!(a.first().unwrap().is_some(), true);
    }

    #[pg_test]
    fn test_zero_i32() {
        let d = 0.into_datum();
        assert!(d.is_some());

        let i = unsafe { i32::from_datum(d.unwrap(), false) };
        assert_eq!(i, Some(0));
    }

    #[pg_test]
    fn test_false_bool() {
        let d = false.into_datum();
        assert!(d.is_some());

        let i = unsafe { bool::from_datum(d.unwrap(), false) };
        assert_eq!(i, Some(false));
    }

    #[pg_test]
    fn test_zero_i32_is_some_zero() {
        let d = pg_sys::Datum::from(0i32);

        let d = unsafe { pg_sys::Datum::from_datum(d, false) };
        assert!(d.is_some());

        let i = unsafe { i32::from_datum(d.unwrap(), false) };
        assert_eq!(i, Some(0));
    }

    #[pg_test]
    fn test_false_bool_is_some_false() {
        let d = pg_sys::Datum::from(false);

        let d = unsafe { pg_sys::Datum::from_datum(d, false) };
        assert!(d.is_some());

        let b = unsafe { bool::from_datum(d.unwrap(), false) };
        assert_eq!(b, Some(false));
    }
}
