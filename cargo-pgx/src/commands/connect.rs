// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::run::exec_psql;
use crate::commands::start::start_postgres;
use colored::Colorize;
use pgx_utils::{createdb, BASE_POSTGRES_PORT_NO};

pub(crate) fn connect_psql(major_version: u16, dbname: &str) {
    // restart postgres
    start_postgres(major_version);

    // create the named database
    if !createdb(
        major_version,
        "localhost",
        BASE_POSTGRES_PORT_NO + major_version,
        dbname,
        true,
    ) {
        println!(
            "{} existing database {}",
            "    Re-using".bold().cyan(),
            dbname
        );
    }

    // run psql
    exec_psql(major_version, dbname);
}
