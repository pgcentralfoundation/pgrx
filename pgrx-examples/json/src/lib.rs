//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::Json;
use pgrx::prelude::*;
use serde_json::json;

pgrx::pg_module_magic!(name, version);

/// Serialize a Postgres `text[]` into a JSON document without allocating an
/// intermediate `Vec<String>`.
///
/// Taking the parameter as `Array<'a, &'a str>` lets pgrx borrow each element straight out of the caller's array buffer; the borrowed `&str` is then handed to `serde_json` which copies it into the JSON output. Compared with a `Vec<String>` signature this saves one `String` allocation per element, which adds up quickly on large tag lists or label arrays.
#[pg_extern]
fn text_array_to_json_doc<'dat>(values: Array<'dat, &'dat str>) -> Json {
    Json(json! { { "values": values } })
}

/// Same idea for `bytea[]`: borrow the byte slices rather than owning them.
#[pg_extern]
fn bytea_array_to_json_doc<'dat>(values: Array<'dat, &'dat [u8]>) -> Json {
    Json(json! { { "values": values } })
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
