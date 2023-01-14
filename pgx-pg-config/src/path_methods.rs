use eyre::{eyre, WrapErr};
use serde_json::value::Value as JsonValue;
use std::path::PathBuf;
use std::process::Command;

// Originally part of `pgx-utils`
pub fn prefix_path<P: Into<PathBuf>>(dir: P) -> String {
    let mut path = std::env::split_paths(&std::env::var_os("PATH").expect("failed to get $PATH"))
        .collect::<Vec<_>>();

    path.insert(0, dir.into());
    std::env::join_paths(path)
        .expect("failed to join paths")
        .into_string()
        .expect("failed to construct path")
}

// Originally part of `pgx-utils`
pub fn get_target_dir() -> eyre::Result<PathBuf> {
    let mut command = Command::new("cargo");
    command.arg("metadata").arg("--format-version=1").arg("--no-deps");
    let output =
        command.output().wrap_err("Unable to get target directory from `cargo metadata`")?;
    if !output.status.success() {
        return Err(eyre!("'cargo metadata' failed with exit code: {}", output.status));
    }

    let json: JsonValue =
        serde_json::from_slice(&output.stdout).wrap_err("Invalid `cargo metadata` response")?;
    let target_dir = json.get("target_directory");
    match target_dir {
        Some(JsonValue::String(target_dir)) => Ok(target_dir.into()),
        v => Err(eyre!("could not read target dir from `cargo metadata` got: {:?}", v,)),
    }
}
