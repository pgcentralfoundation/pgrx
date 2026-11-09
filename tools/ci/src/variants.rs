//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Test the PostgreSQL build variants that only Linux CI covers, for one PostgreSQL version:
//!
//! - pgrx-unit-tests without the C shim
//! - custom_types without schema generation, which compiles its tests out
//! - pgrx-unit-tests checked for aarch64 against target headers fetched for the cross build
//!
//! Each builds in its own target directory, so the three builds overlap.

use std::process::Command;

use eyre::{WrapErr, eyre};

use crate::pg::{self, Postgres};
use crate::process::{self, log};
use crate::testbin;
use crate::workspace::Workspace;

pub(crate) fn run(version: u16) -> eyre::Result<bool> {
    let workspace = Workspace::load()?;
    let mut postgres = Postgres::from_env(version)?;
    postgres.extra_features.insert("pgrx-unit-tests", &["proptest"]);
    let feature = postgres.feature();
    let unit_tests =
        workspace.package("pgrx-unit-tests").ok_or_else(|| eyre!("no pgrx-unit-tests package"))?;
    let custom_types =
        workspace.package("custom_types").ok_or_else(|| eyre!("no custom_types package"))?;
    let test_target = workspace.target_dir("target");
    let ext_target = workspace.target_dir("target-ext");
    let cross_target = workspace.target_dir("target-cross");
    let cargo_pgrx = pg::cargo_pgrx_in(&ext_target);

    std::thread::scope(|scope| -> eyre::Result<bool> {
        let cross = scope.spawn(|| {
            process::run(
                Command::new("cargo")
                    .args([
                        "check",
                        "--locked",
                        "--tests",
                        "--features",
                        &feature,
                        "--package",
                        "pgrx-unit-tests",
                    ])
                    .args(["--target", "aarch64-unknown-linux-gnu"])
                    .env("CARGO_TARGET_DIR", &cross_target),
            )
        });
        // custom_types without schema generation has no tests left, so building is its test.
        let tools = scope.spawn(|| -> eyre::Result<bool> {
            if !pg::build_cargo_pgrx(&ext_target)? {
                return Ok(false);
            }
            process::run(
                Command::new("cargo")
                    .args(["test", "--locked", "--no-run", "--manifest-path"])
                    .arg(&custom_types.manifest_path)
                    .args([
                        "--features",
                        &format!("{feature} no-schema-generation"),
                        "--no-default-features",
                    ])
                    .env("CARGO_TARGET_DIR", &ext_target),
            )
        });

        // pgrx-unit-tests without the C shim: its tests and its extension build in `target`.
        let features = postgres.features("pgrx-unit-tests");
        let binaries = testbin::build(
            &workspace,
            &[
                "--locked",
                "--features",
                &features,
                "--no-default-features",
                "-p",
                "pgrx-unit-tests",
            ]
            .map(String::from),
        )?;
        let tools_passed =
            tools.join().map_err(|_| eyre!("the cargo-pgrx build thread panicked"))??;
        let mut passed = tools_passed;
        if tools_passed {
            postgres.install(&cargo_pgrx, &test_target, unit_tests);
            let outcomes = testbin::run_all(binaries, pg::parallelism(), |binary, command| {
                postgres.configure(
                    command,
                    &binary.package,
                    cargo_pgrx.as_std_path(),
                    test_target.as_std_path(),
                );
            });
            passed &= testbin::passed(&outcomes);
        }
        let cross_passed = cross
            .join()
            .map_err(|_| eyre!("the aarch64 check thread panicked"))?
            .wrap_err("aarch64 check")?;
        if !cross_passed {
            log("FAILED: the aarch64 check of pgrx-unit-tests");
        }
        Ok(passed && cross_passed)
    })
}
