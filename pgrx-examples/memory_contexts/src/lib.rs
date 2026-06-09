//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

mod basics;
mod bgworker_state;
mod srf_per_call;

pgrx::pg_module_magic!(name, version);

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    // The bgworker example registers itself only when loaded via shared_preload_libraries. Outside that mode this is a no-op.
    if unsafe { pgrx::pg_sys::process_shared_preload_libraries_in_progress } {
        crate::bgworker_state::register_bgworker();
    }
}

use pgrx::pg_guard;

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec!["shared_preload_libraries='memory_contexts'"]
    }
}
