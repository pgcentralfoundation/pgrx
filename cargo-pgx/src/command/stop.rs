// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{command::status::status_postgres, CommandExecute};
use colored::Colorize;
use pgx_utils::pg_config::{PgConfig, PgConfigSelector, Pgx};

use std::process::Stdio;
use eyre::eyre as eyre_err;

/// Stop a pgx-managed Postgres instance
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Stop {
    /// The Postgres version to stop (`pg10`, `pg11`, `pg12`, `pg13`, `pg14`, or `all`)
    #[clap(env = "PG_VERSION")]
    pg_version: String,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Stop {
    #[tracing::instrument(level = "info", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let pgver = self.pg_version;
        let pgx = Pgx::from_config()?;

        for pg_config in pgx.iter(PgConfigSelector::new(&pgver)) {
            let pg_config = pg_config?;
            stop_postgres(pg_config)?
        }

        Ok(())
    }
}

#[tracing::instrument(level = "info", skip_all, fields(pg_version = %pg_config.version()?))]
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
        Err(eyre_err!("{}",
            String::from_utf8(output.stderr)?,
        ))
    } else {
        Ok(())
    }
}
