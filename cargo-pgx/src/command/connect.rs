// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{
    command::{get::get_property, run::exec_psql, start::start_postgres},
    CommandExecute,
};
use eyre::{eyre, WrapErr};
use owo_colors::OwoColorize;
use pgx_utils::createdb;
use pgx_utils::pg_config::{PgConfig, Pgx};

/// Connect, via psql, to a Postgres instance
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Connect {
    /// Do you want to run against Postgres `pg10`, `pg11`, `pg12`, `pg13`, `pg14`?
    #[clap(env = "PG_VERSION")]
    pg_version: Option<String>,
    /// The database to connect to (and create if the first time).  Defaults to a database with the same name as the current extension name
    #[clap(env = "DBNAME")]
    dbname: Option<String>,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Connect {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(mut self) -> eyre::Result<()> {
        let pgx = Pgx::from_config()?;

        let pg_version = match self.pg_version {
            Some(pg_version) => match pgx.get(&pg_version) {
                Ok(_) => pg_version,
                Err(err) => {
                    if self.dbname.is_some() {
                        return Err(err);
                    }
                    // It's actually the dbname! We should infer from the manifest.
                    self.dbname = Some(pg_version);

                    let metadata = crate::metadata::metadata(&Default::default())?;
                    crate::metadata::validate(&metadata)?;
                    let manifest = crate::manifest::manifest(&metadata)?;

                    let default_pg_version = crate::manifest::default_pg_version(&manifest)
                        .ok_or(eyre!("No provided `pg$VERSION` flag."))?;
                    default_pg_version
                }
            },
            None => {
                // We should infer from the manifest.
                let metadata = crate::metadata::metadata(&Default::default())?;
                crate::metadata::validate(&metadata)?;
                let manifest = crate::manifest::manifest(&metadata)?;

                let default_pg_version = crate::manifest::default_pg_version(&manifest)
                    .ok_or(eyre!("No provided `pg$VERSION` flag."))?;
                default_pg_version
            }
        };

        let dbname = match self.dbname {
            Some(dbname) => dbname,
            None => get_property("extname")
                .wrap_err("could not determine extension name")?
                .ok_or(eyre!("extname not found in control file"))?,
        };

        connect_psql(Pgx::from_config()?.get(&pg_version)?, &dbname)
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
