// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::install::install_extension;
use crate::commands::start::start_postgres;
use crate::commands::stop::stop_postgres;
use crate::CommandExecute;
use colored::Colorize;
use pgx_utils::createdb;
use pgx_utils::pg_config::{PgConfig, Pgx};
use std::os::unix::process::CommandExt;
use std::process::Command;

use super::get::get_property;

/// Compile/install extension to a pgx-managed Postgres instance and start psql
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Run {
    /// Do you want to run against Postgres `pg10`, `pg11`, `pg12`, `pg13`, `pg14`?
    #[clap(env = "PG_VERSION")]
    pg_version: String,
    /// The database to connect to (and create if the first time).  Defaults to a database with the same name as the current extension name
    dbname: Option<String>,
    /// Compile for release mode (default is debug)
    #[clap(env = "PROFILE", long, short)]
    release: bool,
    /// Don't regenerate the schema
    #[clap(long, short)]
    no_schema: bool,
    /// Additional cargo features to activate (default is '--no-default-features')
    #[clap(long)]
    features: Vec<String>,
}

impl CommandExecute for Run {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
        let dbname = self.dbname.map_or_else(
            || get_property("extname").expect("could not determine extension name"),
            |v| v.to_string(),
        );

        run_psql(
            Pgx::from_config()?.get(&self.pg_version)?,
            &dbname,
            self.release,
            self.no_schema,
            &self.features,
        )
    }
}

pub(crate) fn run_psql(
    pg_config: &PgConfig,
    dbname: &str,
    is_release: bool,
    no_schema: bool,
    additional_features: &Vec<impl AsRef<str>>,
) -> Result<(), std::io::Error> {
    // stop postgres
    stop_postgres(pg_config)?;

    // install the extension
    install_extension(pg_config, is_release, no_schema, None, additional_features)?;

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
