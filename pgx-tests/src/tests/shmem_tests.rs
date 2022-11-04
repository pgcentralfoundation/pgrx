/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::prelude::*;
use pgx::{pg_shmem_init, PgAtomic, PgSharedMemoryInitialization};
use std::sync::atomic::AtomicBool;

static ATOMIC: PgAtomic<AtomicBool> = PgAtomic::new();

#[pg_guard]
pub extern "C" fn _PG_init() {
    // This ensures that this functionality works across PostgreSQL versions
    pg_shmem_init!(ATOMIC);
}
