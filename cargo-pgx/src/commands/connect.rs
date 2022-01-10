// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::run::exec_psql;
use crate::commands::start::start_postgres;
use crate::CommandExecute;
use colored::Colorize;
use pgx_utils::createdb;
use pgx_utils::pg_config::{PgConfig, Pgx};

use super::get::get_property;

/// Connect, via psql, to a Postgres instance
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Connect {
    /// Do you want to run against Postgres `pg10`, `pg11`, `pg12`, `pg13`, `pg14`?
    #[clap(env = "PG_VERSION")]
    pg_version: String,
    /// The database to connect to (and create if the first time).  Defaults to a database with the same name as the current extension name
    #[clap(env = "DBNAME")]
    dbname: Option<String>,
}

impl CommandExecute for Connect {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
        let dbname = self.dbname.map_or_else(
            || get_property("extname").expect("could not determine extension name"),
            |v| v.to_string(),
        );
        connect_psql(Pgx::from_config()?.get(&self.pg_version)?, &dbname)
    }
}

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
