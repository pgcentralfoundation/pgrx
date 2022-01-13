// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{
    command::{get::get_property, run::exec_psql, start::start_postgres},
    CommandExecute,
};
use colored::Colorize;
use eyre::{eyre, WrapErr};
use pgx_utils::createdb;
use pgx_utils::pg_config::{PgConfig, Pgx};

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
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Connect {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let dbname = match self.dbname {
            Some(dbname) => dbname,
            None => get_property("extname")
                .wrap_err("could not determine extension name")?
                .ok_or(eyre!("extname not found in control file"))?,
        };
        connect_psql(Pgx::from_config()?.get(&self.pg_version)?, &dbname)
    }
}

#[tracing::instrument(level = "error", skip_all, fields(
    pg_version = %pg_config.version()?,
    dbname,
))]
pub(crate) fn connect_psql(pg_config: &PgConfig, dbname: &str) -> eyre::Result<()> {
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
