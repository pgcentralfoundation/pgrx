/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use cargo_toml::Manifest;
use eyre::{eyre, WrapErr};
use pgx_pg_config::{get_target_dir, PgConfig, PgConfigSelector, Pgx};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::profile::CargoProfile;
use crate::CommandExecute;

/// Run the test suite for this crate
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Test {
    /// Do you want to run against Postgres `pg11`, `pg12`, `pg13`, `pg14`, `pg15`, or `all`?
    #[clap(env = "PG_VERSION")]
    pg_version: Option<String>,
    /// If specified, only run tests containing this string in their names
    testname: Option<String>,
    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// compile for release mode (default is debug)
    #[clap(long, short)]
    release: bool,
    /// Specific profile to use (conflicts with `--release`)
    #[clap(long)]
    profile: Option<String>,
    /// Don't regenerate the schema
    #[clap(long, short)]
    no_schema: bool,
    #[clap(flatten)]
    features: clap_cargo::Features,
    #[clap(from_global, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl CommandExecute for Test {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let pgx = Pgx::from_config()?;

        let metadata = crate::metadata::metadata(&self.features, self.manifest_path.as_ref())
            .wrap_err("couldn't get cargo metadata")?;
        crate::metadata::validate(&metadata)?;
        let package_manifest_path =
            crate::manifest::manifest_path(&metadata, self.package.as_ref())
                .wrap_err("Couldn't get manifest path")?;
        let package_manifest =
            Manifest::from_path(&package_manifest_path).wrap_err("Couldn't parse manifest")?;

        let pg_version = match self.pg_version {
            Some(ref s) => s.clone(),
            None => crate::manifest::default_pg_version(&package_manifest)
                .ok_or(eyre!("No provided `pg$VERSION` flag."))?,
        };
        let profile = CargoProfile::from_flags(
            self.profile.as_deref(),
            self.release.then_some(CargoProfile::Release).unwrap_or(CargoProfile::Dev),
        )?;

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
                        &crate::manifest::default_pg_version(&package_manifest)
                            .ok_or(eyre!("No provided `pg$VERSION` flag."))?,
                    )?
                }
                Ok(config) => config,
            };
            let pg_version = format!("pg{}", pg_config.major_version()?);

            let features = crate::manifest::features_for_version(
                self.features.clone(),
                &package_manifest,
                &pg_version,
            );

            test_extension(
                pg_config,
                self.manifest_path.as_ref(),
                self.package.as_ref(),
                &profile,
                self.no_schema,
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
    ?profile,
))]
pub fn test_extension(
    pg_config: &PgConfig,
    user_manifest_path: Option<impl AsRef<Path>>,
    user_package: Option<&String>,
    profile: &CargoProfile,
    no_schema: bool,
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
        .env("PGX_NO_DEFAULT_FEATURES", if no_default_features_arg { "true" } else { "false" })
        .env("PGX_ALL_FEATURES", if features.all_features { "true" } else { "false" })
        .env("PGX_BUILD_PROFILE", profile.name())
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

    command.args(profile.cargo_args());

    if let Some(user_manifest_path) = user_manifest_path {
        command.arg("--manifest-path");
        command.arg(user_manifest_path.as_ref());
    }

    if let Some(user_package) = user_package {
        command.arg("--package");
        command.arg(user_package);
    }

    if let Some(testname) = testname {
        command.arg(testname.as_ref());
    }

    eprintln!("{:?}", command);

    tracing::debug!(command = ?command, "Running");
    let status = command.status().wrap_err("failed to run cargo test")?;
    tracing::trace!(status_code = %status, command = ?command, "Finished");
    if !status.success() {
        if !status.success() {
            // We explicitly do not want to return a spantraced error here.
            std::process::exit(1)
        }
    }

    Ok(())
}
