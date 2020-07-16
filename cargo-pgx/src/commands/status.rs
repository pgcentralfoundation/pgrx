use pgx_utils::{exit_with_error, get_pg_config, get_pgdata_dir, handle_result, run_pg_config};
use std::path::PathBuf;
use std::process::Stdio;

pub(crate) fn status_postgres(major_version: u16) -> bool {
    let datadir = get_pgdata_dir(major_version);
    let pg_config = get_pg_config(major_version);
    let bindir: PathBuf = run_pg_config(&pg_config, "--bindir").into();

    let mut command = std::process::Command::new(format!("{}/pg_ctl", bindir.display()));
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("status")
        .arg("-D")
        .arg(datadir.display().to_string());
    let command_str = format!("{:?}", command);

    let output = handle_result!(
        format!("failed to get postgres' status: {}", command_str),
        command.output()
    );

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
    is_running
}
