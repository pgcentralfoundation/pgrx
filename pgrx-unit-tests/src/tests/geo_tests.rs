//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use pgrx::fn_call::{Arg, fn_call};
    use pgrx::geo::*;
    use pgrx::prelude::*;

    #[pg_extern]
    fn geo_path_identity(path: Path) -> Path {
        path
    }

    #[pg_extern]
    fn geo_polygon_identity(polygon: Polygon) -> Polygon {
        polygon
    }

    #[pg_test]
    fn test_point_datum() -> spi::Result<()> {
        let p = Spi::get_one::<Point>("SELECT '42, 99'::point")?.expect("SPI result was null");
        assert_eq!(p, Point { x: 42.0, y: 99.0 });
        let p2 = Spi::get_one_with_args::<Point>("SELECT $1", &[p.into()])?
            .expect("SPI result was null");
        assert_eq!(p, p2);
        Ok(())
    }

    #[pg_test]
    fn test_box_datum() -> spi::Result<()> {
        let b = Spi::get_one::<Box>("SELECT '1,2,3,4'::box")?.expect("SPI result was null");
        assert_eq!(b, Box { high: Point { x: 3.0, y: 4.0 }, low: Point { x: 1.0, y: 2.0 } });
        let b2 =
            Spi::get_one_with_args::<Box>("SELECT $1", &[b.into()])?.expect("SPI result was null");
        assert_eq!(b, b2);
        Ok(())
    }

    #[pg_test]
    fn test_circle_datum() -> spi::Result<()> {
        let c = Spi::get_one::<Circle>("SELECT '1,2,3'::circle")?.expect("SPI result was null");
        assert_eq!(c, Circle { center: Point { x: 1.0, y: 2.0 }, radius: 3.0 });
        let c2 = Spi::get_one_with_args::<Circle>("SELECT $1", &[c.into()])?
            .expect("SPI result was null");
        assert_eq!(c, c2);
        Ok(())
    }

    #[pg_test]
    fn test_line_datum() -> spi::Result<()> {
        let l = Spi::get_one::<Line>("SELECT '{1,2,3}'::line")?.expect("SPI result was null");
        assert_eq!(l.A, 1.0);
        assert_eq!(l.B, 2.0);
        assert_eq!(l.C, 3.0);
        let l2 =
            Spi::get_one_with_args::<Line>("SELECT $1", &[l.into()])?.expect("SPI result was null");
        assert_eq!(l.A, l2.A);
        assert_eq!(l.B, l2.B);
        assert_eq!(l.C, l2.C);
        Ok(())
    }

    #[pg_test]
    fn test_lseg_datum() -> spi::Result<()> {
        let l = Spi::get_one::<LineSegment>("SELECT '(1,2),(3,4)'::lseg")?
            .expect("SPI result was null");
        assert_eq!(l.p, [Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }]);
        let l2 = Spi::get_one_with_args::<LineSegment>("SELECT $1", &[l.into()])?
            .expect("SPI result was null");
        assert_eq!(l.p, l2.p);
        Ok(())
    }

    #[pg_test]
    fn test_path_datum() -> spi::Result<()> {
        // Closed path
        let p = Spi::get_one::<Path>("SELECT '((1,2),(3,4))'::path")?.expect("SPI result was null");
        assert_eq!(p.points(), [Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }]);
        assert_eq!(p.closed(), true);
        let p2 = Spi::get_one_with_args::<Path>("SELECT $1", &[p.clone().into()])?
            .expect("SPI result was null");
        assert_eq!(p.points(), p2.points());
        assert_eq!(p.closed(), p2.closed());

        // Open path
        let p = Spi::get_one::<Path>("SELECT '[(1,2),(3,4)]'::path")?.expect("SPI result was null");
        assert_eq!(p.points(), [Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }]);
        assert_eq!(p.closed(), false);
        let p2 = Spi::get_one_with_args::<Path>("SELECT $1", &[p.clone().into()])?
            .expect("SPI result was null");
        assert_eq!(p.points(), p2.points());
        assert_eq!(p.closed(), p2.closed());

        Ok(())
    }

    #[pg_test]
    fn test_polygon_datum() -> spi::Result<()> {
        let p = Spi::get_one::<Polygon>("SELECT '((1,2),(3,4),(0,5))'::polygon")?
            .expect("SPI result was null");
        assert_eq!(
            p.points(),
            [Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }, Point { x: 0.0, y: 5.0 }]
        );
        assert_eq!(
            p.boundbox(),
            Box { high: Point { x: 3.0, y: 5.0 }, low: Point { x: 0.0, y: 2.0 } }
        );
        let p2 = Spi::get_one_with_args::<Polygon>("SELECT $1", &[p.clone().into()])?
            .expect("SPI result was null");
        assert_eq!(p.points(), p2.points());
        assert_eq!(p.boundbox(), p2.boundbox());
        Ok(())
    }

    #[pg_test]
    fn test_fn_call_path_datum() {
        let path = Path::new(vec![Point { x: 1.0, y: 2.0 }, Point { x: 3.0, y: 4.0 }], true);
        let result = fn_call::<Path>("tests.geo_path_identity", &[&Arg::Value(path.clone())])
            .expect("fn_call failed")
            .expect("fn_call result was null");

        assert_eq!(path.points(), result.points());
        assert_eq!(path.closed(), result.closed());
    }

    #[pg_test]
    fn test_fn_call_polygon_datum() {
        let polygon = Polygon::new(vec![
            Point { x: 1.0, y: 2.0 },
            Point { x: 3.0, y: 4.0 },
            Point { x: 0.0, y: 5.0 },
        ]);
        let result =
            fn_call::<Polygon>("tests.geo_polygon_identity", &[&Arg::Value(polygon.clone())])
                .expect("fn_call failed")
                .expect("fn_call result was null");

        assert_eq!(polygon.points(), result.points());
        assert_eq!(polygon.boundbox(), result.boundbox());
    }
}
