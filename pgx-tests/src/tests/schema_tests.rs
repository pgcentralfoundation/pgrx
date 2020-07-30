// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


mod test_schema {
    use pgx::*;

    #[pg_extern]
    fn func_in_diff_schema() {}
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
}
