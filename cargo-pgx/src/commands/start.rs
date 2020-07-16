use colored::Colorize;
use pgx_utils::{
    exit_with_error, get_pg_config, get_pgdata_dir, get_pglog_file, handle_result, run_pg_config,
    BASE_POSTGRES_PORT_NO,
};
use std::path::PathBuf;
use std::process::Stdio;

pub(crate) fn start_postgres(major_version: u16) -> Result<(), std::io::Error> {
    let datadir = get_pgdata_dir(major_version);
    let logfile = get_pglog_file(major_version);
    let pg_config = get_pg_config(major_version);
    let bindir: PathBuf = run_pg_config(&pg_config, "--bindir").into();
    let port = BASE_POSTGRES_PORT_NO + major_version;

    if !datadir.exists() {
        initdb(&bindir, &datadir);
    }

    println!(
        "  {} Postgres v{} on port {}",
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
        .arg(format!("-o -i -p {}", port))
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

    Ok(())
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
