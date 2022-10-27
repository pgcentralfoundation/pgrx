/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::command::get::get_property;
use crate::command::run::exec_psql;
use crate::command::start::start_postgres;
use crate::CommandExecute;
use cargo_toml::Manifest;
use eyre::{eyre, WrapErr};
use owo_colors::OwoColorize;
use pgx_pg_config::{createdb, PgConfig, Pgx};
use std::path::PathBuf;

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
    #[clap(from_global, action = ArgAction::Count)]
    verbose: usize,
    /// Package to determine default `pg_version` with (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// Use an existing `pgcli` on the $PATH.
    #[clap(env = "PGX_PGCLI", long)]
    pgcli: bool,
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

                    let metadata =
                        crate::metadata::metadata(&Default::default(), self.manifest_path.as_ref())
                            .wrap_err("couldn't get cargo metadata")?;
                    crate::metadata::validate(&metadata)?;
                    let package_manifest_path =
                        crate::manifest::manifest_path(&metadata, self.package.as_ref())
                            .wrap_err("Couldn't get manifest path")?;
                    let package_manifest = Manifest::from_path(&package_manifest_path)
                        .wrap_err("Couldn't parse manifest")?;

                    let default_pg_version = crate::manifest::default_pg_version(&package_manifest)
                        .ok_or(eyre!("no provided `pg$VERSION` flag."))?;
                    default_pg_version
                }
            },
            None => {
                // We should infer from the manifest.
                let metadata =
                    crate::metadata::metadata(&Default::default(), self.manifest_path.as_ref())
                        .wrap_err("couldn't get cargo metadata")?;
                crate::metadata::validate(&metadata)?;
                let package_manifest_path =
                    crate::manifest::manifest_path(&metadata, self.package.as_ref())
                        .wrap_err("Couldn't get manifest path")?;
                let package_manifest = Manifest::from_path(&package_manifest_path)
                    .wrap_err("Couldn't parse manifest")?;

                let default_pg_version = crate::manifest::default_pg_version(&package_manifest)
                    .ok_or(eyre!("no provided `pg$VERSION` flag."))?;
                default_pg_version
            }
        };

        let dbname = match self.dbname {
            Some(dbname) => dbname,
            None => {
                // We should infer from package
                let metadata =
                    crate::metadata::metadata(&Default::default(), self.manifest_path.as_ref())
                        .wrap_err("couldn't get cargo metadata")?;
                crate::metadata::validate(&metadata)?;
                let package_manifest_path =
                    crate::manifest::manifest_path(&metadata, self.package.as_ref())
                        .wrap_err("Couldn't get manifest path")?;

                get_property(&package_manifest_path, "extname")
                    .wrap_err("could not determine extension name")?
                    .ok_or(eyre!("extname not found in control file"))?
            }
        };

        connect_psql(Pgx::from_config()?.get(&pg_version)?, &dbname, self.pgcli)
    }
}

#[tracing::instrument(level = "error", skip_all, fields(
    pg_version = %pg_config.version()?,
    dbname,
))]
pub(crate) fn connect_psql(pg_config: &PgConfig, dbname: &str, pgcli: bool) -> eyre::Result<()> {
    // restart postgres
    start_postgres(pg_config)?;

    // create the named database
    if !createdb(pg_config, dbname, false, true)? {
        println!("{} existing database {}", "    Re-using".bold().cyan(), dbname);
    }

    // run psql
    exec_psql(&pg_config, dbname, pgcli)
}
