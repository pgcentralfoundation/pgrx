/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::command::init::initdb;
use crate::command::status::status_postgres;
use crate::manifest::{get_package_manifest, pg_config_and_version};
use crate::CommandExecute;
use eyre::eyre;
use owo_colors::OwoColorize;
use pgx_pg_config::{PgConfig, PgConfigSelector, Pgx};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Stdio;

/// Start a pgx-managed Postgres instance
#[derive(clap::Args, Debug, Clone)]
#[clap(author)]
pub(crate) struct Start {
    /// The Postgres version to start (`pg11`, `pg12`, `pg13`, `pg14`, `pg15`, or `all`)
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

impl CommandExecute for Start {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        fn perform(me: Start, pgx: &Pgx) -> eyre::Result<()> {
            let (package_manifest, _) = get_package_manifest(
                &clap_cargo::Features::default(),
                me.package.as_ref(),
                me.manifest_path,
            )?;
            let (pg_config, _) =
                pg_config_and_version(&pgx, &package_manifest, me.pg_version, None, false)?;

            start_postgres(pg_config)
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
pub(crate) fn start_postgres(pg_config: &PgConfig) -> eyre::Result<()> {
    let datadir = pg_config.data_dir()?;
    let logfile = pg_config.log_file()?;
    let bindir = pg_config.bin_dir()?;
    let port = pg_config.port()?;

    if !datadir.exists() {
        initdb(&bindir, &datadir)?;
    }

    if status_postgres(pg_config)? {
        tracing::debug!("Already started");
        return Ok(());
    }

    println!(
        "{} Postgres v{} on port {}",
        "    Starting".bold().green(),
        pg_config.major_version()?,
        port.to_string().bold().cyan()
    );
    let mut command = std::process::Command::new(format!("{}/pg_ctl", bindir.display()));
    // Unsafe block is for the pre_exec setsid call below
    //
    // This is to work around a bug in PG11 which does not call setsid in pg_ctl
    // This means that when cargo pgx run dumps a user into psql, pushing ctrl-c will abort
    // the postgres server started by pgx
    unsafe {
        command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("start")
            .arg(format!("-o -i -p {} -c unix_socket_directories={}", port, Pgx::home()?.display()))
            .arg("-D")
            .arg(&datadir)
            .arg("-l")
            .arg(&logfile)
            .pre_exec(|| {
                fork::setsid().expect("setsid call failed for pg_ctl");
                Ok(())
            });
    }

    let command_str = format!("{:?}", command);
    let output = command.output()?;

    if !output.status.success() {
        return Err(eyre!(
            "problem running pg_ctl: {}\n\n{}",
            command_str,
            String::from_utf8(output.stderr).unwrap()
        ));
    }

    Ok(())
}
