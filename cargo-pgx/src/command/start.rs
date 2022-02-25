// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::command::init::initdb;
use crate::command::status::status_postgres;
use crate::CommandExecute;
use owo_colors::OwoColorize;
use eyre::eyre;
use pgx_utils::pg_config::{PgConfig, PgConfigSelector, Pgx};
use std::os::unix::process::CommandExt;
use std::process::Stdio;

/// Start a pgx-managed Postgres instance
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Start {
    /// The Postgres version to start (`pg10`, `pg11`, `pg12`, `pg13`, `pg14`, or `all`)
    #[clap(env = "PG_VERSION")]
    pg_version: Option<String>,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Start {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let pgx = Pgx::from_config()?;

        let pg_version = match self.pg_version {
            Some(s) => s,
            None => {
                let metadata = crate::metadata::metadata(&Default::default())?;
                crate::metadata::validate(&metadata)?;
                let manifest = crate::manifest::manifest(&metadata)?;
                crate::manifest::default_pg_version(&manifest)
                    .ok_or(eyre!("No provided `pg$VERSION` flag."))?
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
