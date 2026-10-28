//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::prelude::*;
use pgrx::rel::PgRelation;
#[pg_extern]
fn accept_relation(rel: PgRelation) {
    _ = rel;
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use pgrx::prelude::*;
    use pgrx::rel::PgRelation;

    #[pg_test]
    fn test_accept_relation() {
        Spi::run("CREATE TABLE relation_test(name TEXT);").unwrap();
        Spi::run("CREATE INDEX relation_test_index ON relation_test(name);").unwrap();
        Spi::run("SELECT accept_relation('relation_test_index');").unwrap();
    }

    #[pg_test]
    fn test_name_survives_rename_of_open_relation() {
        Spi::run("CREATE TABLE rel_rename_abcd(id int);").unwrap();
        let rel = PgRelation::open_with_name_and_share_lock("rel_rename_abcd").unwrap();
        let name = rel.name();

        // Our own AccessShareLock does not block the rename, and the relcache rebuild after it
        // copies the new pg_class row over the open relation's `rd_rel`. The new name cuts a
        // multibyte character at the old name's length.
        Spi::run("ALTER TABLE rel_rename_abcd RENAME TO \"rel_rename_ab€\";").unwrap();

        assert_eq!(name, "rel_rename_abcd");
        assert_eq!(rel.name(), "rel_rename_ab€");
    }

    #[pg_test]
    fn test_namespace_outlives_spi_context() {
        Spi::run("CREATE SCHEMA rel_namespace_probe;").unwrap();
        Spi::run("CREATE TABLE rel_namespace_probe.t(id int);").unwrap();
        let rel = PgRelation::open_with_name_and_share_lock("rel_namespace_probe.t").unwrap();

        // SPI_finish deletes the memory context that is current inside the closure.
        let namespace = Spi::connect(|_| rel.namespace());

        assert_eq!(namespace, "rel_namespace_probe");
    }
}
