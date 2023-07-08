/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use maplit::*;
use pgrx::prelude::*;
use serde::*;
use std::collections::HashMap;

#[derive(PostgresType, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RustStore(HashMap<String, String>);

impl Default for RustStore {
    fn default() -> Self {
        RustStore(HashMap::default())
    }
}

#[pg_extern]
fn rstore(key: String, value: String) -> RustStore {
    RustStore(hashmap!(key => value))
}

#[pg_extern]
fn rstore_put(rstore: Option<RustStore>, key: String, value: String) -> RustStore {
    let mut rstore = rstore.unwrap_or_default();
    rstore.0.insert(key, value);
    rstore
}

#[pg_extern]
fn rstore_get(rstore: Option<RustStore>, key: String) -> Option<String> {
    rstore.map_or(None, |rstore| rstore.0.get(&key).cloned())
}

#[pg_extern]
fn rstore_remove(rstore: Option<RustStore>, key: String) -> Option<RustStore> {
    match rstore {
        Some(mut rstore) => {
            rstore.0.remove(&key);

            if rstore.0.is_empty() {
                None
            } else {
                Some(rstore)
            }
        }
        None => None,
    }
}

#[pg_extern]
fn rstore_size(rstore: Option<RustStore>) -> i64 {
    rstore.map_or(0, |rstore| rstore.0.len()) as i64
}

#[pg_extern]
fn rstore_table(
    rstore: Option<RustStore>,
) -> TableIterator<'static, (name!(key, String), name!(value, String))> {
    match rstore {
        Some(rstore) => TableIterator::new(rstore.0.into_iter()),
        None => TableIterator::once((String::new(), String::new())),
    }
}
