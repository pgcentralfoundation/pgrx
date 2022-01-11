// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use colored::Colorize;
use pgx_utils::exit_with_error;
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
}

impl CommandExecute for Status {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
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

pub(crate) fn status_postgres(pg_config: &PgConfig) -> Result<bool, std::io::Error> {
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
    let output = command.output()?;
    let code = output.status.code().unwrap();
    let is_running = code == 0; // running
    let is_stopped = code == 3; // not running

    if !is_running && !is_stopped {
        exit_with_error!(
            "problem running pg_ctl: {}\n\n{}",
            command_str,
            String::from_utf8(output.stderr).unwrap()
        )
    }

    // a status code of zero means it's running
    Ok(is_running)
}
