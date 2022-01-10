// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{commands::status::status_postgres, CommandExecute};
use colored::Colorize;
use pgx_utils::pg_config::{PgConfig, PgConfigSelector, Pgx};

use std::process::Stdio;

/// Stop a pgx-managed Postgres instance
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Stop {
    /// The Postgres version to stop (`pg10`, `pg11`, `pg12`, `pg13`, `pg14`, or `all`)
    #[clap(env = "PG_VERSION")]
    pg_version: String,
}

impl CommandExecute for Stop {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
        let pgver = self.pg_version;
        let pgx = Pgx::from_config()?;

        for pg_config in pgx.iter(PgConfigSelector::new(&pgver)) {
            stop_postgres(pg_config?)?
        }

        Ok(())
    }
}

pub(crate) fn stop_postgres(pg_config: &PgConfig) -> Result<(), std::io::Error> {
    Pgx::home()?;
    let datadir = pg_config.data_dir()?;
    let bindir = pg_config.bin_dir()?;

    if status_postgres(pg_config)? == false {
        // it's not running, no need to stop it
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
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            String::from_utf8(output.stderr).unwrap(),
        ))
    } else {
        Ok(())
    }
}
