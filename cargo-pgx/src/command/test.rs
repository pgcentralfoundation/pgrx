// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use eyre::{eyre, WrapErr};
use pgx_utils::{
    get_target_dir,
    pg_config::{PgConfig, PgConfigSelector, Pgx},
};
use std::fmt::Write;
use std::process::{Command, Stdio};

use crate::CommandExecute;

/// Run the test suite for this crate
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Test {
    /// Do you want to run against Postgres `pg10`, `pg11`, `pg12`, `pg13`, `pg14`, or `all`?
    #[clap(env = "PG_VERSION", default_value = "all")]
    pg_version: String,
    /// If specified, only run tests containing this string in their names
    testname: Option<String>,
    /// compile for release mode (default is debug)
    #[clap(env = "PROFILE", long, short)]
    release: bool,
    /// Don't regenerate the schema
    #[clap(long, short)]
    no_schema: bool,
    /// Test all packages in the workspace
    #[clap(long)]
    workspace: bool,
    /// Additional cargo features to activate (default is `--no-default-features`)
    #[clap(long)]
    features: Vec<String>,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Test {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let pgx = Pgx::from_config()?;
        for pg_config in pgx.iter(PgConfigSelector::new(&self.pg_version)) {
            let pg_config = pg_config?;
            test_extension(
                pg_config,
                self.release,
                self.no_schema,
                self.workspace,
                &self.features,
                self.testname.clone(),
            )?
        }
        Ok(())
    }
}

#[tracing::instrument(skip_all, fields(
    pg_version = %pg_config.version()?,
    testname =  tracing::field::Empty,
    release = is_release,
))]
pub fn test_extension(
    pg_config: &PgConfig,
    is_release: bool,
    no_schema: bool,
    test_workspace: bool,
    additional_features: &Vec<impl AsRef<str>>,
    testname: Option<impl AsRef<str>>,
) -> eyre::Result<()> {
    if let Some(ref testname) = testname {
        tracing::Span::current().record("testname", &tracing::field::display(&testname.as_ref()));
    }

    let additional_features = additional_features
        .iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>();
    let major_version = pg_config.major_version()?;
    let target_dir = get_target_dir()?;

    let mut command = Command::new("cargo");

    let mut features = additional_features.join(" ");
    let _ = write!(&mut features, " pg{} pg_test", major_version);

    command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("test")
        .arg("--lib")
        .arg("--features")
        .arg(features)
        .arg("--no-default-features")
        .env("CARGO_TARGET_DIR", &target_dir)
        .env(
            "PGX_BUILD_PROFILE",
            if is_release { "release" } else { "debug" },
        )
        .env("PGX_NO_SCHEMA", if no_schema { "true" } else { "false" });

    if is_release {
        command.arg("--release");
    }

    if test_workspace {
        command.arg("--all");
    }

    if let Some(testname) = testname {
        command.arg(testname.as_ref());
    }

    eprintln!("{:?}", command);

    tracing::debug!(command = ?command, "Running");
    let status = command.status().wrap_err("failed to run cargo test")?;
    tracing::trace!(status_code = %status, command = ?command, "Finished");
    if !status.success() {
        return Err(eyre!(
            "cargo pgx test failed with status = {:?}",
            status.code()
        ));
    }

    Ok(())
}
