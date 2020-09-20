// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::install::install_extension;
use crate::commands::start::start_postgres;
use crate::commands::stop::stop_postgres;
use colored::Colorize;
use pgx_utils::{createdb, get_pg_config, get_psql_path, BASE_POSTGRES_PORT_NO};
use std::os::unix::process::CommandExt;
use std::process::Command;

pub(crate) fn run_psql(major_version: u16, dbname: &str, is_release: bool) {
    let pg_config = get_pg_config(major_version);

    // stop postgres
    stop_postgres(major_version, true);

    // install the extension
    install_extension(&pg_config, is_release, None);

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

fn exec_psql(major_version: u16, dbname: &str) {
    let mut command = Command::new(get_psql_path(major_version));
    command
        .env_remove("PGDATABASE")
        .env_remove("PGHOST")
        .env_remove("PGPORT")
        .env_remove("PGUSER")
        .arg("-h")
        .arg("localhost")
        .arg("-p")
        .arg((BASE_POSTGRES_PORT_NO + major_version).to_string())
        .arg(dbname);

    // we'll never return from here as we've now become psql
    panic!("{}", command.exec());
}
