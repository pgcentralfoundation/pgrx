// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::run::exec_psql;
use crate::commands::start::start_postgres;
use colored::Colorize;
use pgx_utils::createdb;
use pgx_utils::pg_config::PgConfig;

pub(crate) fn connect_psql(pg_config: &PgConfig, dbname: &str) -> Result<(), std::io::Error> {
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
