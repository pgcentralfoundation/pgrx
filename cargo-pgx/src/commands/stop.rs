// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::status::status_postgres;
use colored::Colorize;
use pgx_utils::{
    exit_with_error, get_pgbin_dir, get_pgdata_dir, get_pgx_config_path, handle_result,
};
use std::process::Stdio;

pub(crate) fn stop_postgres(major_version: u16, fail_on_error: bool) {
    if !get_pgx_config_path().exists() && !fail_on_error {
        // we don't have a pgx config path and we're not asked to fail on errors
        // so just silently return
        return;
    }

    let datadir = get_pgdata_dir(major_version);
    let bindir = get_pgbin_dir(major_version);

    if !status_postgres(major_version, fail_on_error) {
        return;
    }

    println!(
        "{} Postgres v{}",
        "    Stopping".bold().green(),
        major_version
    );
    let mut command = std::process::Command::new(format!("{}/pg_ctl", bindir.display()));
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("stop")
        .arg("-m")
        .arg("fast")
        .arg("-D")
        .arg(datadir.display().to_string());
    let command_str = format!("{:?}", command);

    if fail_on_error {
        let output = handle_result!(
            format!("failed to stop postgres: {}", command_str),
            command.output()
        );

        if !output.status.success() {
            exit_with_error!(
                "problem running pg_ctl: {}\n\n{}",
                command_str,
                String::from_utf8(output.stderr).unwrap()
            )
        }
    } else {
        command.status().ok();
    }
}
