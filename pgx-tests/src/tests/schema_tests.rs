// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.
use pgx::*;

mod test_schema {
    use pgx::*;
    use serde::{Deserialize, Serialize};

    #[pg_extern]
    fn func_in_diff_schema() {}

    #[derive(Debug, PostgresType, Serialize, Deserialize)]
    pub struct TestType(pub u64);
}

#[pg_extern(schema = "test_schema")]
fn func_in_diff_schema2() {}

#[pg_extern]
fn type_in_diff_schema() -> test_schema::TestType {
    test_schema::TestType(1)
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_in_different_schema() {
        Spi::run("SELECT test_schema.func_in_diff_schema();");
    }

    #[pg_test]
    fn test_in_different_schema2() {
        Spi::run("SELECT test_schema.func_in_diff_schema2();");
    }

    #[pg_test]
    fn test_type_in_different_schema() {
        Spi::run("SELECT type_in_diff_schema();");
    }
}
