/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::command::status::status_postgres;
use crate::manifest::{get_package_manifest, pg_config_and_version};
use crate::CommandExecute;
use eyre::eyre;
use owo_colors::OwoColorize;
use pgx_pg_config::{PgConfig, PgConfigSelector, Pgx};
use std::path::PathBuf;
use std::process::Stdio;

/// Stop a pgx-managed Postgres instance
#[derive(clap::Args, Debug, Clone)]
#[clap(author)]
pub(crate) struct Stop {
    /// The Postgres version to stop (`pg11`, `pg12`, `pg13`, `pg14`, `pg15`, or `all`)
    #[clap(env = "PG_VERSION")]
    pg_version: Option<String>,
    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
    /// Package to determine default `pg_version` with (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
}

impl CommandExecute for Stop {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        fn perform(me: Stop, pgx: &Pgx) -> eyre::Result<()> {
            let (package_manifest, _) = get_package_manifest(
                &clap_cargo::Features::default(),
                me.package.as_ref(),
                me.manifest_path,
            )?;
            let (pg_config, _) =
                pg_config_and_version(&pgx, &package_manifest, me.pg_version, None, false)?;

            stop_postgres(pg_config)
        }

        let pgx = Pgx::from_config()?;
        if self.pg_version == Some("all".into()) {
            for v in pgx.iter(PgConfigSelector::All) {
                let mut versioned_start = self.clone();
                versioned_start.pg_version = Some(v?.label()?);
                perform(versioned_start, &pgx)?;
            }
            Ok(())
        } else {
            perform(self, &pgx)
        }
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

    println!("{} Postgres v{}", "    Stopping".bold().green(), pg_config.major_version()?);

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
