//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Test the workspace packages that don't need PostgreSQL.
//!
//! nextest runs each test in its own process, so tests that serialize libclang within one
//! process run in parallel, and so do tests from different binaries. Doctests run afterwards
//! with the same package selection, which reuses the libraries nextest built.

use std::process::Command;

use eyre::eyre;

use crate::process;
use crate::workspace::{self, Workspace};

pub(crate) fn run(packages: &[String]) -> eyre::Result<bool> {
    if packages.is_empty() {
        return Ok(true);
    }
    let workspace = Workspace::load()?;
    let mut selection = Vec::new();
    let mut libraries = false;
    for name in packages {
        let package =
            workspace.package(name).ok_or_else(|| eyre!("no workspace package named {name}"))?;
        libraries |= workspace::has_rust_library(package);
        selection.extend(["-p", name.as_str()]);
    }

    // pgrx-bindgen's fixture builds run Cargo offline, so every locked crate must be present.
    let mut passed = process::run(Command::new("cargo").args(["fetch", "--locked"]))?;
    passed &= process::run(
        Command::new("cargo")
            .args(["nextest", "run", "--locked", "--no-fail-fast", "--no-tests=pass"])
            .args(&selection),
    )?;
    // `cargo test --doc` refuses a selection without a library, such as cargo-pgrx alone.
    if libraries {
        passed &= process::run(
            Command::new("cargo").args(["test", "--locked", "--doc"]).args(&selection),
        )?;
    }
    Ok(passed)
}
