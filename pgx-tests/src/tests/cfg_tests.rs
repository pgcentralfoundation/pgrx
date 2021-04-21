// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.
use pgx::*;



#[cfg(any(test, feature = "pg_test"))]
#[pg_extern]
fn func_test_cfg() {}

#[cfg(feature="nonexistent")]
#[pg_extern]
fn func_non_existent_cfg(t: NonexistentType) {}


#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_cfg_exists() {
        Spi::run("SELECT func_test_cfg();");
    }
}
