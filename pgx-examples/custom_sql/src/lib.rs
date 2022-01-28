// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx::*;
use serde::{Deserialize, Serialize};

pg_module_magic!();

#[pg_schema]
mod home {
    use super::*;

    #[pg_schema]
    pub mod dogs {
        use super::*;

        #[derive(PostgresEnum, Serialize, Deserialize)]
        pub enum Dog {
            Brandy,
            Nami,
        }
    }

    #[derive(PostgresType, Serialize, Deserialize)]
    pub struct Ball {
        last_chomp: Dog,
    }
}
pub use home::dogs::Dog;

// `extension_sql` allows you to define your own custom SQL.
//
// Since PostgreSQL is is order dependent, you may need to specify a positioning.
//
// Valid options are:
//  * `bootstrap` positions the block before any other generated SQL. It should be unique.
//    Errors if `before`/`after` are also present.
//  * `before = [$ident]` & `after = [$ident]` positions the block before/after `$ident`
//    where `$ident` is a string identifier or a path to a SQL entity (such as a type which derives
//    `PostgresType`)
//  * `creates = [Enum($ident), Type($ident), Function($ident)]` tells the dependency graph that
//    this block creates a given entity.
//  * `name` is a string identifier for the item, in case you need to refer to it in other
//    positioning.
extension_sql!(
    "\n\
        CREATE TABLE extension_sql (message TEXT);\n\
        INSERT INTO extension_sql VALUES ('bootstrap');\n\
    ",
    name = "bootstrap_raw",
    bootstrap,
);
extension_sql!(
    format!("INSERT INTO extension_sql VALUES ('{}');", "single_raw"),
    name = "single_raw",
    requires = [home::dogs]
);
extension_sql!(
    "\n\
    INSERT INTO extension_sql VALUES ('multiple_raw');\n\
",
    name = "multiple_raw",
    requires = [Dog, home::Ball, "single_raw", "single"],
);

// `extension_sql_file` does the same as `extension_sql`
extension_sql_file!("../sql/single.sql", name = "single", requires = ["single_raw"]);
extension_sql_file!(
    concat!("../sql/", "multiple.sql"),
    name = "multiple",
    requires = [Dog, home::Ball, "single_raw", "single", "multiple_raw"],
);

extension_sql_file!("../sql/finalizer.sql", name = "finalizer", finalize);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_ordering() {
        let buf = Spi::connect(|client| {
            let buf = client
                .select("SELECT * FROM extension_sql", None, None)
                .flat_map(|tup| tup.by_ordinal(1).ok().and_then(|ord| ord.value::<String>()))
                .collect::<Vec<String>>();

            Ok(Some(buf))
        });

        assert_eq!(
            buf.unwrap(),
            vec![
                String::from("bootstrap"),
                String::from("single_raw"),
                String::from("single"),
                String::from("multiple_raw"),
                String::from("multiple"),
                String::from("finalizer")
            ]
        )
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
