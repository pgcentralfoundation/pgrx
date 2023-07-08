#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;
    use pgrx::xid_to_64bit;

    #[test]
    fn make_idea_happy() {}

    #[pg_test]
    fn test_convert_xid_to_u64() {
        let xid = xid_to_64bit(32768);
        assert_eq!(xid, 32768)
    }
}
