// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx_utils::{get_target_dir, handle_result};
use std::process::{Command, Stdio};

pub fn test_extension(major_version: u16) {
    let target_dir = get_target_dir();

    handle_result!(
        "failed to run cargo test",
        Command::new("cargo")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("test")
            .arg("--all")
            .arg("--features")
            .arg(format!("pg{}", major_version))
            .arg("--no-default-features")
            .env("CARGO_TARGET_DIR", target_dir.display().to_string())
            .status()
    );
}
