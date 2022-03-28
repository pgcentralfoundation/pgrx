/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::*;

#[cfg(any(test, feature = "pg_test"))]
#[pg_extern]
fn func_test_cfg() {}

#[cfg(feature = "nonexistent")]
#[pg_extern]
fn func_non_existent_cfg(t: NonexistentType) {}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_cfg_exists() {
        Spi::run("SELECT func_test_cfg();");
    }
}
