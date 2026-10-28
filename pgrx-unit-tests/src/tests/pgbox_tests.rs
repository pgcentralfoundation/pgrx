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
    use pgrx::prelude::*;
    use pgrx::{AllocatedByRust, AnyArray, Array};

    /// Reads each point after the array it came from has dropped.
    #[pg_extern]
    fn pgbox_points_from_vec(points: Vec<PgBox<pg_sys::Point>>) -> Vec<pg_sys::Point> {
        points.iter().map(|point| **point).collect()
    }

    #[pg_extern]
    fn pgbox_points_from_array_into_iter(
        points: Array<PgBox<pg_sys::Point>>,
    ) -> Vec<pg_sys::Point> {
        let points: Vec<_> = points.into_iter().map(|point| point.expect("NULL point")).collect();
        points.iter().map(|point| **point).collect()
    }

    #[pg_extern]
    fn pgbox_points_from_anyarray(points: AnyArray) -> Vec<pg_sys::Point> {
        let points = points
            .value::<Vec<PgBox<pg_sys::Point>>>()
            .expect("argument is a point[]")
            .expect("NULL argument");
        points.iter().map(|point| **point).collect()
    }

    /// A point[] read from a table is detoasted into a copy that its `Array` frees when it drops:
    /// a short array has a 1-byte header, and the long one here is stored out of line. Each
    /// `PgBox` unboxed from the array must still hold its point after that.
    #[pg_test]
    fn test_pgbox_array_elements_outlive_array() -> spi::Result<()> {
        Spi::run("CREATE TABLE tests.pgbox_points (id int, points point[])")?;
        // EXTERNAL moves a long value out of line without compressing it.
        Spi::run("ALTER TABLE tests.pgbox_points ALTER COLUMN points SET STORAGE EXTERNAL")?;
        Spi::run(
            "INSERT INTO tests.pgbox_points
                SELECT 1, ARRAY[point(1, -1), point(2, -2)]
                UNION ALL
                SELECT 2, array_agg(point(i, -i) ORDER BY i) FROM generate_series(1, 200) i",
        )?;
        let toast = Spi::get_one::<String>(
            "SELECT reltoastrelid::regclass::text FROM pg_class
                WHERE oid = 'tests.pgbox_points'::regclass",
        )?
        .expect("no TOAST relation");
        assert_eq!(Spi::get_one::<bool>(&format!("SELECT count(*) > 0 FROM {toast}"))?, Some(true));

        for (id, len) in [(1, 2), (2, 200)] {
            let expected: Vec<pg_sys::Point> =
                (1..=len).map(|i| pg_sys::Point { x: i as f64, y: -i as f64 }).collect();
            for function in [
                "pgbox_points_from_vec",
                "pgbox_points_from_array_into_iter",
                "pgbox_points_from_anyarray",
            ] {
                let points = Spi::get_one::<Vec<pg_sys::Point>>(&format!(
                    "SELECT tests.{function}(points) FROM tests.pgbox_points WHERE id = {id}"
                ))?;
                assert_eq!(points.as_ref(), Some(&expected), "{function} on row {id}");
            }
            // SPI frees its own memory when the query ends, so these must be in the caller's
            // memory context.
            let points = Spi::get_one::<Vec<PgBox<pg_sys::Point>>>(&format!(
                "SELECT points FROM tests.pgbox_points WHERE id = {id}"
            ))?
            .expect("NULL points");
            assert_eq!(points.iter().map(|point| **point).collect::<Vec<_>>(), expected);
        }
        Ok(())
    }

    #[pg_test]
    fn pgbox_alloc() {
        let mut ptr: PgBox<i32, AllocatedByRust> = unsafe { PgBox::<i32>::alloc() };
        // ptr is uninitialized data!!! This is dangerous to read from!!!
        *ptr = 5;

        assert_eq!(*ptr, 5);
    }

    #[pg_test]
    fn pgbox_alloc0() {
        let mut ptr: PgBox<i32, AllocatedByRust> = unsafe { PgBox::<i32>::alloc0() };

        assert_eq!(*ptr, 0);

        *ptr = 5;

        assert_eq!(*ptr, 5);
    }
}
