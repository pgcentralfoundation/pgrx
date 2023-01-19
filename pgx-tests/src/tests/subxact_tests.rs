/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::prelude::*;
    use pgx::spi::SpiClient;
    use pgx::subxact::CommitOnDrop;

    #[pg_test]
    fn test_subxact_smoketest() {
        Spi::connect(|mut c| {
            c.update("CREATE TABLE a (v INTEGER)", None, None).unwrap();
            let c = c.sub_transaction(|mut xact| {
                xact.update("INSERT INTO a VALUES (0)", None, None).unwrap();
                assert_eq!(
                    0,
                    xact.select("SELECT v FROM a", Some(1), None)
                        .unwrap()
                        .first()
                        .get::<i64>(1)
                        .unwrap()
                        .unwrap()
                );
                let xact = xact.sub_transaction(|mut xact| {
                    xact.update("INSERT INTO a VALUES (1)", None, None).unwrap();
                    assert_eq!(
                        2,
                        xact.select("SELECT COUNT(*) FROM a", Some(1), None)
                            .unwrap()
                            .first()
                            .get::<i64>(1)
                            .unwrap()
                            .unwrap()
                    );
                    xact.rollback()
                });
                xact.rollback()
            });
            assert_eq!(
                0,
                c.select("SELECT COUNT(*) FROM a", Some(1), None)
                    .unwrap()
                    .first()
                    .get::<i64>(1)
                    .unwrap()
                    .unwrap()
            );
        })
    }

    #[pg_test]
    fn test_commit_on_drop() {
        Spi::connect(|mut c| {
            c.update("CREATE TABLE a (v INTEGER)", None, None).unwrap();
            // The type below is explicit to ensure it's commit on drop by default
            c.sub_transaction(|mut xact: SubTransaction<SpiClient, CommitOnDrop>| {
                xact.update("INSERT INTO a VALUES (0)", None, None).unwrap();
                // Dropped explicitly for illustration purposes
                drop(xact);
            });
            // Create a new client to check the state
            Spi::connect(|c| {
                // The above insert should have been committed
                assert_eq!(
                    1,
                    c.select("SELECT COUNT(*) FROM a", Some(1), None)
                        .unwrap()
                        .first()
                        .get::<i64>(1)
                        .unwrap()
                        .unwrap()
                );
            });
        })
    }

    #[pg_test]
    fn test_rollback_on_drop() {
        Spi::connect(|mut c| {
            c.update("CREATE TABLE a (v INTEGER)", None, None).unwrap();
            // The type below is explicit to ensure it's commit on drop by default
            c.sub_transaction(|mut xact: SubTransaction<SpiClient, CommitOnDrop>| {
                xact.update("INSERT INTO a VALUES (0)", None, None).unwrap();
                let xact = xact.rollback_on_drop();
                // Dropped explicitly for illustration purposes
                drop(xact);
            });
            // Create a new client to check the state
            Spi::connect(|c| {
                // The above insert should NOT have been committed
                assert_eq!(
                    0,
                    c.select("SELECT COUNT(*) FROM a", Some(1), None)
                        .unwrap()
                        .first()
                        .get::<i64>(1)
                        .unwrap()
                        .unwrap()
                );
            });
        })
    }
}
