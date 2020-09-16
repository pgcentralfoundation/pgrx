// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx_utils::{exit_with_error, get_target_dir, handle_result};
use std::process::{Command, Stdio};

pub fn test_extension(major_version: u16, is_release: bool) {
    let target_dir = get_target_dir();

    let mut command = Command::new("cargo");

    command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("test")
        .arg("--all")
        .arg("--features")
        .arg(format!("pg{} pg_test", major_version))
        .arg("--no-default-features")
        .env("CARGO_TARGET_DIR", target_dir.display().to_string())
        .env(
            "PGX_BUILD_PROFILE",
            if is_release { "release" } else { "debug" },
        );

    if is_release {
        command.arg("--release");
    }

    eprintln!("{:?}", command);
    let status = handle_result!("failed to run cargo test", command.status());
    if !status.success() {
        exit_with_error!("cargo pgx test failed with status = {:?}", status.code())
    }
}
