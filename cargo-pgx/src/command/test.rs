// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use eyre::{eyre, WrapErr};
use pgx_utils::{
    get_target_dir,
    pg_config::{PgConfig, PgConfigSelector, Pgx},
};
use std::process::{Command, Stdio};

use crate::CommandExecute;

/// Run the test suite for this crate
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Test {
    /// Do you want to run against Postgres `pg10`, `pg11`, `pg12`, `pg13`, `pg14`, or `all`?
    #[clap(env = "PG_VERSION")]
    pg_version: Option<String>,
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
    #[clap(flatten)]
    features: clap_cargo::Features,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Test {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let pgx = Pgx::from_config()?;
        let metadata = crate::metadata::metadata(&self.features)?;
        crate::metadata::validate(&metadata)?;
        let manifest = crate::manifest::manifest(&metadata)?;

        let pg_version = match self.pg_version {
            Some(s) => s,
            None => crate::manifest::default_pg_version(&manifest)
                .ok_or(eyre!("No provided `pg$VERSION` flag."))?,
        };

        for pg_config in pgx.iter(PgConfigSelector::new(&pg_version)) {
            let mut testname = self.testname.clone();
            let pg_config = match pg_config {
                Err(error) => {
                    tracing::debug!(
                        invalid_pg_version = %pg_version,
                        error = %error,
                        "Got invalid `pg$VERSION` flag, assuming it is a testname"
                    );
                    testname = Some(pg_version.clone());
                    pgx.get(
                        &crate::manifest::default_pg_version(&manifest)
                            .ok_or(eyre!("No provided `pg$VERSION` flag."))?,
                    )?
                }
                Ok(config) => config,
            };
            let pg_version = format!("pg{}", pg_config.major_version()?);

            let features = crate::manifest::features_for_version(
                self.features.clone(),
                &manifest,
                &pg_version,
            );
            test_extension(
                pg_config,
                self.release,
                self.no_schema,
                self.workspace,
                &features,
                testname.clone(),
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
    features: &clap_cargo::Features,
    testname: Option<impl AsRef<str>>,
) -> eyre::Result<()> {
    if let Some(ref testname) = testname {
        tracing::Span::current().record("testname", &tracing::field::display(&testname.as_ref()));
    }
    let target_dir = get_target_dir()?;

    let mut command = Command::new("cargo");

    let no_default_features_arg = features.no_default_features;
    let mut features_arg = features.features.join(" ");
    if features.features.iter().all(|f| f != "pg_test") {
        features_arg += " pg_test";
    }

    command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("test")
        .env("CARGO_TARGET_DIR", &target_dir)
        .env("PGX_FEATURES", features_arg.clone())
        .env(
            "PGX_NO_DEFAULT_FEATURES",
            if no_default_features_arg {
                "true"
            } else {
                "false"
            },
        )
        .env(
            "PGX_ALL_FEATURES",
            if features.all_features {
                "true"
            } else {
                "false"
            },
        )
        .env(
            "PGX_BUILD_PROFILE",
            if is_release { "release" } else { "debug" },
        )
        .env("PGX_NO_SCHEMA", if no_schema { "true" } else { "false" });

    if let Ok(rust_log) = std::env::var("RUST_LOG") {
        command.env("RUST_LOG", rust_log);
    }

    if !features_arg.trim().is_empty() {
        command.arg("--features");
        command.arg(&features_arg);
    }

    if no_default_features_arg {
        command.arg("--no-default-features");
    }

    if features.all_features {
        command.arg("--all-features");
    }

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
