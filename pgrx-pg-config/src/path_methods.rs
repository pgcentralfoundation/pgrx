//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use eyre::{WrapErr, eyre};
use serde_json::value::Value as JsonValue;
use std::path::{Path, PathBuf};
use std::process::Command;

// Originally part of `pgrx-utils`
pub fn prefix_path<P: Into<PathBuf>>(dir: P) -> String {
    let mut path = std::env::split_paths(&std::env::var_os("PATH").expect("failed to get $PATH"))
        .collect::<Vec<_>>();

    path.insert(0, dir.into());
    std::env::join_paths(path)
        .expect("failed to join paths")
        .into_string()
        .expect("failed to construct path")
}

/// The target dir, or where we think it will be
pub fn get_target_dir() -> eyre::Result<PathBuf> {
    if let Some(path) = std::env::var_os("CARGO_TARGET_DIR") {
        return Ok(path.into());
    }
    // if we don't actually have CARGO_TARGET_DIR set, we try to infer it
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut command = Command::new(cargo);
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
        v => Err(eyre!("could not read target dir from `cargo metadata` got: {v:?}")),
    }
}

/// The first executable named `name` in a directory on `PATH`, as a shell finds it. Windows
/// also tries the name with each extension in `PATHEXT`.
pub(crate) fn find_executable_in_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path).find_map(|dir| executable_in(&dir, name))
}

#[cfg(unix)]
fn executable_in(dir: &Path, name: &str) -> Option<PathBuf> {
    use std::os::unix::fs::PermissionsExt;
    let candidate = dir.join(name);
    let metadata = std::fs::metadata(&candidate).ok()?;
    (metadata.is_file() && metadata.permissions().mode() & 0o111 != 0).then_some(candidate)
}

#[cfg(not(unix))]
fn executable_in(dir: &Path, name: &str) -> Option<PathBuf> {
    let candidate = dir.join(name);
    if candidate.is_file() {
        return Some(candidate);
    }
    let extensions = std::env::var_os("PATHEXT").unwrap_or_else(|| ".EXE".into());
    std::env::split_paths(&extensions)
        .map(|extension| {
            let mut file = candidate.clone().into_os_string();
            file.push(extension);
            PathBuf::from(file)
        })
        .find(|file| file.is_file())
}

#[cfg(all(test, unix))]
mod tests {
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn finds_only_executable_files() {
        let dir = std::env::temp_dir().join(format!("pgrx-path-search-{}", std::process::id()));
        let (plain, script) = (dir.join("plain"), dir.join("script"));
        std::fs::create_dir_all(dir.join("dir")).unwrap();
        std::fs::write(&plain, "").unwrap();
        std::fs::write(&script, "").unwrap();
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();

        assert_eq!(super::executable_in(&dir, "script"), Some(script));
        assert_eq!(super::executable_in(&dir, "plain"), None);
        assert_eq!(super::executable_in(&dir, "dir"), None);
        assert_eq!(super::executable_in(&dir, "missing"), None);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
