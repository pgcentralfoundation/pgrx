// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use std::process::{Command, Stdio};

pub fn test_extension(version: &str) -> Result<(), std::io::Error> {
    let versions = if version == "all" {
        vec!["pg10", "pg11", "pg12"]
    } else {
        vec![version]
    };

    for version in versions {
        let cargo_target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| {
            format!(
                "{}/target",
                std::env::current_dir()
                    .expect("couldn't detect current directory")
                    .display()
            )
        });
        let pgx_target_dir = format!("{}/pgx-test-{}", cargo_target_dir, version);

        let result = Command::new("cargo")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("test")
            .arg("--all")
            .arg("--features")
            .arg(version)
            .arg("--no-default-features")
            .env("RUST_BACKTRACE", "1")
            .env("CARGO_TARGET_DIR", pgx_target_dir)
            .env("PG_DOWNLOAD_TARGET_DIR", cargo_target_dir)
            .status();

        if result.is_err() {
            return Err(result.err().unwrap());
        }
    }

    Ok(())
}
