// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::install::install_extension;
use crate::commands::start::start_postgres;
use crate::commands::stop::stop_postgres;
use colored::Colorize;
use pgx_utils::createdb;
use pgx_utils::pg_config::PgConfig;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub(crate) fn run_psql(
    pg_config: &PgConfig,
    dbname: &str,
    is_release: bool,
) -> Result<(), std::io::Error> {
    // stop postgres
    stop_postgres(pg_config)?;

    // install the extension
    install_extension(pg_config, is_release, None)?;

    // restart postgres
    start_postgres(pg_config)?;

    // create the named database
    if !createdb(pg_config, dbname, false, true)? {
        println!(
            "{} existing database {}",
            "    Re-using".bold().cyan(),
            dbname
        );
    }

    // run psql
    exec_psql(pg_config, dbname)
}

pub(crate) fn exec_psql(pg_config: &PgConfig, dbname: &str) -> Result<(), std::io::Error> {
    let mut command = Command::new(pg_config.psql_path()?);
    command
        .env_remove("PGDATABASE")
        .env_remove("PGHOST")
        .env_remove("PGPORT")
        .env_remove("PGUSER")
        .arg("-h")
        .arg(pg_config.host())
        .arg("-p")
        .arg(pg_config.port()?.to_string())
        .arg(dbname);

    // we'll never return from here as we've now become psql
    panic!("{}", command.exec());
}
