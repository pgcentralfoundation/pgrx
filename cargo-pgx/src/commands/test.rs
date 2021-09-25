// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx_utils::pg_config::PgConfig;
use pgx_utils::{exit_with_error, get_target_dir, handle_result};
use std::fmt::Write;
use std::process::{Command, Stdio};

pub fn test_extension(
    pg_config: &PgConfig,
    is_release: bool,
    test_workspace: bool,
    additional_features: Vec<&str>,
    testname: Option<&str>,
) -> Result<(), std::io::Error> {
    let major_version = pg_config.major_version()?;
    let target_dir = get_target_dir();

    let mut command = Command::new("cargo");

    let mut features = additional_features.join(" ");
    let _ = write!(&mut features, " pg{} pg_test", major_version);

    command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("test")
        .arg("--features")
        .arg(features)
        .arg("--no-default-features")
        .env("CARGO_TARGET_DIR", &target_dir)
        .env(
            "PGX_BUILD_PROFILE",
            if is_release { "release" } else { "debug" },
        );

    if is_release {
        command.arg("--release");
    }

    if test_workspace {
        command.arg("--all");
    }

    if let Some(testname) = testname {
        command.arg(testname);
    }

    eprintln!("{:?}", command);
    let status = handle_result!(command.status(), "failed to run cargo test");
    if !status.success() {
        exit_with_error!("cargo pgx test failed with status = {:?}", status.code())
    }

    Ok(())
}
