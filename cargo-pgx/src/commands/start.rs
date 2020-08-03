// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::status::status_postgres;
use colored::Colorize;
use pgx_utils::{
    exit_with_error, get_pgbin_dir, get_pgdata_dir, get_pglog_file, get_pgx_home, handle_result,
    BASE_POSTGRES_PORT_NO,
};
use std::path::PathBuf;
use std::process::Stdio;

pub(crate) fn start_postgres(major_version: u16) {
    let datadir = get_pgdata_dir(major_version);
    let logfile = get_pglog_file(major_version);
    let bindir = get_pgbin_dir(major_version);
    let port = BASE_POSTGRES_PORT_NO + major_version;

    if !datadir.exists() {
        initdb(&bindir, &datadir);
    }

    if status_postgres(major_version) {
        return;
    }

    println!(
        "{} Postgres v{} on port {}",
        "    Starting".bold().green(),
        major_version,
        port.to_string().bold().cyan()
    );
    let mut command = std::process::Command::new(format!("{}/pg_ctl", bindir.display()));
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("start")
        .arg("--options")
        .arg(format!(
            "-o -i -p {} -c unix_socket_directories={}",
            port,
            get_pgx_home().display()
        ))
        .arg("-D")
        .arg(datadir.display().to_string())
        .arg("-l")
        .arg(logfile.display().to_string());
    let command_str = format!("{:?}", command);

    let output = handle_result!(
        format!("failed to start postgres: {}", command_str),
        command.output()
    );

    if !output.status.success() {
        exit_with_error!(
            "problem running pg_ctl: {}\n\n{}",
            command_str,
            String::from_utf8(output.stderr).unwrap()
        )
    }
}

fn initdb(bindir: &PathBuf, datadir: &PathBuf) {
    println!(
        " {} data directory at {}",
        "Initializing".bold().green(),
        datadir.display()
    );
    let mut command = std::process::Command::new(format!("{}/initdb", bindir.display()));
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("-D")
        .arg(datadir.display().to_string());
    let command_str = format!("{:?}", command);

    let output = handle_result!(
        format!("failed to run initdb: {}", command_str),
        command.output()
    );

    if !output.status.success() {
        exit_with_error!(
            "problem running initdb: {}\n{}",
            command_str,
            String::from_utf8(output.stderr).unwrap()
        )
    }
}
