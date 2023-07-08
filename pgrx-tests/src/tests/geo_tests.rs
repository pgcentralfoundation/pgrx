#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;

    #[pg_test]
    fn test_point_into_datum() -> spi::Result<()> {
        let p =
            Spi::get_one::<pg_sys::Point>("SELECT '42, 99'::point")?.expect("SPI result was null");
        assert_eq!(p.x, 42.0);
        assert_eq!(p.y, 99.0);
        Ok(())
    }

    #[pg_test]
    fn test_box_into_datum() -> spi::Result<()> {
        let b = Spi::get_one::<pg_sys::BOX>("SELECT '1,2,3,4'::box")?.expect("SPI result was null");
        assert_eq!(b.high.x, 3.0);
        assert_eq!(b.high.y, 4.0);
        assert_eq!(b.low.x, 1.0);
        assert_eq!(b.low.y, 2.0);
        Ok(())
    }
}
