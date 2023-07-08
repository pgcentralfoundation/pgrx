/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgrx::prelude::*;
use pgrx::{opname, pg_operator};
use serde::{Deserialize, Serialize};
mod derived;
mod pgvarlena;

pgrx::pg_module_magic!();

#[derive(PostgresType, Serialize, Deserialize, Eq, PartialEq)]
pub struct MyType {
    value: i32,
}

#[pg_operator]
#[opname(=)]
fn my_eq(left: MyType, right: MyType) -> bool {
    left == right
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
