// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::command::init::initdb;
use crate::command::status::status_postgres;
use crate::CommandExecute;
use eyre::{eyre, WrapErr};
use owo_colors::OwoColorize;
use pgx_utils::pg_config::{PgConfig, PgConfigSelector, Pgx};
use std::{
    os::unix::process::CommandExt,
    process::Stdio,
    path::PathBuf,
};
use cargo_toml::Manifest;

/// Start a pgx-managed Postgres instance
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Start {
    /// The Postgres version to start (`pg10`, `pg11`, `pg12`, `pg13`, `pg14`, or `all`)
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

impl CommandExecute for Start {
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
            start_postgres(pg_config)?
        }

        Ok(())
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
    // This is to work around a bug in PG10 + PG11 which don't call setsid in pg_ctl
    // This means that when cargo pgx run dumps a user into psql, pushing ctrl-c will abort
    // the postgres server started by pgx
    unsafe {
        command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("start")
            .arg(format!(
                "-o -i -p {} -c unix_socket_directories={}",
                port,
                Pgx::home()?.display()
            ))
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
