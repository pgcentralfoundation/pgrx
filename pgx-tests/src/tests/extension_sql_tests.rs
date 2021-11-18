// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx::*;

extension_sql!(
    "CREATE TABLE extension_sql_table();\n",
    name = "extension_sql_table"
);

extension_sql!(
    format!("CREATE TABLE extension_sql_{}();\n", "dynamic"),
    name = "extension_sql_dynamic"
);

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_extension_sql_literal() {
        Spi::run("SELECT FROM extension_sql_table");
    }

    #[pg_test]
    fn test_extension_sql_expression() {
        Spi::run("SELECT FROM extension_sql_dynamic");
    }
}