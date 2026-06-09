//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

// Each module demonstrates one PostgresType in/out path or a related derive
// pattern. See README.md for a comparison table and selection criteria.
mod composite_and_array;
mod handrolled_datum;
mod inoutfuncs_custom;
mod json_default;
mod varlena_zerocopy;

pgrx::pg_module_magic!(name, version);

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
