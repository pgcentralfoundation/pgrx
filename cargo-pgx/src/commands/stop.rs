use colored::Colorize;
use pgx_utils::{exit_with_error, get_pg_config, get_pgdata_dir, handle_result, run_pg_config};
use std::path::PathBuf;
use std::process::Stdio;

pub(crate) fn stop_postgres(major_version: u16) -> Result<(), std::io::Error> {
    let datadir = get_pgdata_dir(major_version);
    let pg_config = get_pg_config(major_version);
    let bindir: PathBuf = run_pg_config(&pg_config, "--bindir").into();

    println!(
        "  {} Postgres v{}",
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

    Ok(())
}
