// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use colored::Colorize;
use eyre::eyre;
use pgx_utils::pg_config::{PgConfig, PgConfigSelector, Pgx};
use std::process::Stdio;

use crate::CommandExecute;

/// Is a pgx-managed Postgres instance running?
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Status {
    /// The Postgres version
    #[clap(env = "PG_VERSION")]
    pg_version: String,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Status {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let pgver = self.pg_version;
        let pgx = Pgx::from_config()?;

        for pg_config in pgx.iter(PgConfigSelector::new(&pgver)) {
            let pg_config = pg_config?;
            if status_postgres(pg_config)? {
                println!(
                    "Postgres v{} is {}",
                    pg_config.major_version()?,
                    "running".bold().green()
                )
            } else {
                println!(
                    "Postgres v{} is {}",
                    pg_config.major_version()?,
                    "stopped".bold().red()
                )
            }
        }

        Ok(())
    }
}

#[tracing::instrument(level = "error", skip_all, fields(pg_version = %pg_config.version()?))]
pub(crate) fn status_postgres(pg_config: &PgConfig) -> eyre::Result<bool> {
    let datadir = pg_config.data_dir()?;
    let bindir = pg_config.bin_dir()?;

    if !datadir.exists() {
        // Postgres couldn't possibly be running if there's no data directory
        // and even if it were, we'd have no way of knowing
        return Ok(false);
    }

    let mut command = std::process::Command::new(format!("{}/pg_ctl", bindir.display()));
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("status")
        .arg("-D")
        .arg(&datadir);
    let command_str = format!("{:?}", command);
    tracing::debug!(command = %command_str, "Running");

    let output = command.output()?;
    let code = output.status.code().unwrap();
    tracing::trace!(status_code = %code, command = %command_str, "Finished");

    let is_running = code == 0; // running
    let is_stopped = code == 3; // not running

    if !is_running && !is_stopped {
        return Err(eyre!(
            "problem running pg_ctl: {}\n\n{}",
            command_str,
            String::from_utf8(output.stderr).unwrap()
        ));
    }

    // a status code of zero means it's running
    Ok(is_running)
}
