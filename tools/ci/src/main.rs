//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Chooses and runs the jobs of pgrx's CI workflow, `.github/workflows/tests.yml`.
//!
//! `plan` decides which jobs a change needs. The `test-*` commands run one job's tests and
//! exit unsuccessfully when any test fails. `macro-summaries` and `macro-compare` check that
//! every Clang version generates the same C macros.

mod args;
mod crates;
mod macros;
mod pg;
mod plan;
mod process;
mod testbin;
mod variants;
mod workspace;

use std::process::ExitCode;

use eyre::WrapErr;

const USAGE: &str = "\
usage:
    pgrx-ci plan --all
    pgrx-ci plan --base REVISION --head REVISION
    pgrx-ci test-pg --pg VERSION [--run 'PACKAGE ...'] [--cli-checks]
    pgrx-ci test-variants --pg VERSION
    pgrx-ci test-crates PACKAGE...
    pgrx-ci macro-summaries --clang MAJOR --out DIRECTORY PG_VERSION...
    pgrx-ci macro-compare DIRECTORY

test-pg and test-variants read the PostgreSQL installation from PG_CONFIG.
macro-summaries builds pgrx-pg-sys for each PostgreSQL version, which PGRX_HOME must
configure, and writes what the build generated for each C macro to DIRECTORY/pgNN.json.
macro-compare fails when the summaries under DIRECTORY differ between Clang versions.";

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("Error: {error:?}");
            ExitCode::FAILURE
        }
    }
}

/// Run the requested command, returning whether its tests passed.
fn run() -> eyre::Result<bool> {
    let mut args = args::Args::from_env();
    let Some(command) = args.command() else {
        eyre::bail!("no command\n\n{USAGE}");
    };
    match command.as_str() {
        "plan" => {
            let source = if args.flag("--all") {
                plan::Source::Everything
            } else {
                let base = args.required("--base")?;
                let head = args.required("--head")?;
                plan::Source::Diff { base, head }
            };
            args.finish()?;
            plan::run(&source)?;
            Ok(true)
        }
        "test-pg" => {
            let version = args.pg_version()?;
            let run = args.value("--run")?;
            let cli_checks = args.flag("--cli-checks");
            args.finish()?;
            pg::run(&pg::Options { version, run: run.as_deref(), cli_checks })
        }
        "test-variants" => {
            let version = args.pg_version()?;
            args.finish()?;
            variants::run(version)
        }
        "test-crates" => {
            let packages = args.rest();
            crates::run(&packages)
        }
        "macro-summaries" => {
            let clang = args.required("--clang")?;
            let clang =
                clang.parse().wrap_err_with(|| format!("--clang {clang} is not a Clang major"))?;
            let out = args.required("--out")?;
            let versions = args
                .rest()
                .iter()
                .map(|version| {
                    version
                        .parse()
                        .wrap_err_with(|| format!("{version} is not a PostgreSQL major version"))
                })
                .collect::<eyre::Result<Vec<u16>>>()?;
            macros::summarize(clang, out.as_ref(), &versions)
        }
        "macro-compare" => {
            let dir = args.required_positional("DIRECTORY")?;
            args.finish()?;
            macros::run_compare(dir.as_ref())
        }
        "help" | "--help" | "-h" => {
            println!("{USAGE}");
            Ok(true)
        }
        other => eyre::bail!("unknown command `{other}`\n\n{USAGE}"),
    }
}
