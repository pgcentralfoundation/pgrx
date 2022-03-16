// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{command::status::status_postgres, CommandExecute};
use owo_colors::OwoColorize;
use pgx_utils::pg_config::{PgConfig, PgConfigSelector, Pgx};
use cargo_toml::Manifest;
use eyre::{eyre, WrapErr};
use std::{
    path::PathBuf,
    process::Stdio
};

/// Stop a pgx-managed Postgres instance
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Stop {
    /// The Postgres version to stop (`pg10`, `pg11`, `pg12`, `pg13`, `pg14`, or `all`)
    #[clap(env = "PG_VERSION")]
    pg_version: Option<String>,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
    /// Package to determine default `pg_version` with (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, parse(from_os_str))]
    manifest_path: Option<PathBuf>,
}

impl CommandExecute for Stop {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let pgx = Pgx::from_config()?;

        let pg_version = match self.pg_version {
            Some(s) => s,
            None => {
                let metadata = crate::metadata::metadata(&Default::default(), self.manifest_path.as_ref())
                    .wrap_err("couldn't get cargo metadata")?;
                crate::metadata::validate(&metadata)?;
                let package_manifest_path = crate::manifest::manifest_path(&metadata, self.package.as_ref())
                    .wrap_err("Couldn't get manifest path")?;
                let package_manifest = Manifest::from_path(&package_manifest_path)
                    .wrap_err("Couldn't parse manifest")?;

                crate::manifest::default_pg_version(&package_manifest)
                    .ok_or(eyre!("no provided `pg$VERSION` flag."))?
            }
        };

        for pg_config in pgx.iter(PgConfigSelector::new(&pg_version)) {
            let pg_config = pg_config?;
            stop_postgres(pg_config)?
        }

        Ok(())
    }
}

#[tracing::instrument(level = "error", skip_all, fields(pg_version = %pg_config.version()?))]
pub(crate) fn stop_postgres(pg_config: &PgConfig) -> eyre::Result<()> {
    Pgx::home()?;
    let datadir = pg_config.data_dir()?;
    let bindir = pg_config.bin_dir()?;

    if status_postgres(pg_config)? == false {
        // it's not running, no need to stop it
        tracing::debug!("Already stopped");
        return Ok(());
    }

    println!(
        "{} Postgres v{}",
        "    Stopping".bold().green(),
        pg_config.major_version()?
    );

    let mut command = std::process::Command::new(format!("{}/pg_ctl", bindir.display()));
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("stop")
        .arg("-m")
        .arg("fast")
        .arg("-D")
        .arg(&datadir);

    let output = command.output()?;

    if !output.status.success() {
        Err(eyre!("{}", String::from_utf8(output.stderr)?,))
    } else {
        Ok(())
    }
}
