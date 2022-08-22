/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use eyre::{eyre, WrapErr};
use serde_json::value::Value as JsonValue;
use std::{path::PathBuf, process::Command};

pub fn get_target_dir() -> eyre::Result<PathBuf> {
    let mut command = Command::new("cargo");
    command
        .arg("metadata")
        .arg("--format-version=1")
        .arg("--no-deps");
    let output = command
        .output()
        .wrap_err("Unable to get target directory from `cargo metadata`")?;
    if !output.status.success() {
        return Err(eyre!(
            "'cargo metadata' failed with exit code: {}",
            output.status
        ));
    }

    let json: JsonValue =
        serde_json::from_slice(&output.stdout).wrap_err("Invalid `cargo metada` response")?;
    let target_dir = json.get("target_directory");
    match target_dir {
        Some(JsonValue::String(target_dir)) => Ok(target_dir.into()),
        v => Err(eyre!(
            "could not read target dir from `cargo metadata` got: {:?}",
            v,
        )),
    }
}

pub fn prefix_path<P: Into<PathBuf>>(dir: P) -> String {
    let mut path = std::env::split_paths(&std::env::var_os("PATH").expect("failed to get $PATH"))
        .collect::<Vec<_>>();

    path.insert(0, dir.into());
    std::env::join_paths(path)
        .expect("failed to join paths")
        .into_string()
        .expect("failed to construct path")
}
