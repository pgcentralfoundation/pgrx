// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::status::status_postgres;
use colored::Colorize;
use pgx_utils::pg_config::{PgConfig, Pgx};

use std::process::Stdio;

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
