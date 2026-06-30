//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use cargo_toml::Manifest;
use eyre::Context;
use pgrx_pg_config::{PgConfig, Pgrx, get_target_dir};
use std::path::{Path, PathBuf};
use std::process::Stdio;

use crate::CommandExecute;
use crate::cargo::CargoProfile;
use crate::manifest::{get_package_manifest, pg_config_and_version};

/// Run the test suite for this crate
#[derive(clap::Args, Debug, Clone)]
#[clap(author)]
pub(crate) struct Test {
    /// Do you want to run against pg13, pg14, pg15, pg16, pg17, pg18, pg19, or all?
    #[clap(env = "PG_VERSION")]
    pg_version: Option<String>,
    /// If specified, only run tests containing any of these strings in their names
    #[clap(value_name = "TESTNAME")]
    testnames: Vec<String>,
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
    /// Use `sudo` to initialize and run the Postgres test instance as this system user
    #[clap(long, value_name = "USER")]
    runas: Option<String>,
    /// Initialize the test database cluster here, instead of the default location.  If used with `--runas`, then it must be writable by the user
    #[clap(long, value_name = "DIR")]
    pgdata: Option<PathBuf>,
    #[clap(flatten)]
    features: clap_cargo::Features,
    #[clap(from_global, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl CommandExecute for Test {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(mut self) -> eyre::Result<()> {
        #[tracing::instrument(level = "error", skip(me, package_manifest))]
        fn perform(
            me: Test,
            pgrx: &Pgrx,
            package_manifest: &Manifest,
            package_manifest_path: &Path,
        ) -> eyre::Result<()> {
            let mut features = me.features.clone();
            let (pg_config, _pg_version) = pg_config_and_version(
                pgrx,
                package_manifest,
                me.pg_version.clone(),
                Some(&mut features),
                true,
            )?;

            let profile = CargoProfile::from_flags(
                me.profile.as_deref(),
                if me.release { CargoProfile::Release } else { CargoProfile::Dev },
            )?;

            test_extension(
                &pg_config,
                package_manifest_path,
                &profile,
                me.no_schema,
                &features,
                &me.testnames,
                me.runas,
                me.pgdata,
            )?;

            Ok(())
        }

        let (package_manifest, package_manifest_path) = get_package_manifest(
            &self.features,
            self.package.as_deref(),
            self.manifest_path.as_deref(),
        )?;
        let pgrx = Pgrx::from_config()?;

        (self.pg_version, self.testnames) =
            resolve_test_args(self.pg_version.take(), self.testnames, |arg| {
                arg == "all" || pgrx.is_feature_flag(arg)
            });

        if self.pg_version.as_deref() == Some("all") {
            // run the tests for **all** the Postgres versions we know about
            for v in crate::manifest::all_pg_in_both_tomls(&package_manifest, &pgrx) {
                let mut versioned_test = self.clone();
                versioned_test.pg_version = Some(v?.label()?);
                perform(versioned_test, &pgrx, &package_manifest, &package_manifest_path)?;
            }

            Ok(())
        } else {
            // attempt to run the test for the Postgres version `run_test()` will figure out
            perform(self, &pgrx, &package_manifest, &package_manifest_path)
        }
    }
}

#[tracing::instrument(skip_all, fields(
    pg_version = %pg_config.version()?,
    testnames = tracing::field::Empty,
    ?profile,
))]
pub fn test_extension(
    pg_config: &PgConfig,
    package_manifest_path: &Path,
    profile: &CargoProfile,
    no_schema: bool,
    features: &clap_cargo::Features,
    testnames: &[String],
    runas: Option<String>,
    pgdata: Option<PathBuf>,
) -> eyre::Result<()> {
    #[cfg(target_os = "windows")]
    if runas.is_some() {
        eyre::bail!("`--runas` is not supported on Windows");
    }

    if !testnames.is_empty() {
        tracing::Span::current().record("testnames", tracing::field::display(&testnames.join(",")));
    }
    let target_dir = get_target_dir()?;

    let mut command = crate::cargo::cargo();

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
        .env("PGRX_FEATURES", features_arg.clone())
        .env("PGRX_NO_DEFAULT_FEATURES", if no_default_features_arg { "true" } else { "false" })
        .env("PGRX_ALL_FEATURES", if features.all_features { "true" } else { "false" })
        .env("PGRX_BUILD_PROFILE", profile.name())
        .env("PGRX_NO_SCHEMA", if no_schema { "true" } else { "false" });
    apply_resolved_manifest_to_test_command(&mut command, package_manifest_path);

    if let Some(runas) = runas {
        command.env("CARGO_PGRX_TEST_RUNAS", runas);
    }

    if let Some(pgdata) = pgdata {
        command.env("CARGO_PGRX_TEST_PGDATA", pgdata);
    }

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

    apply_test_filters_to_command(&mut command, testnames);

    eprintln!("{command:?}");

    tracing::debug!(command = ?command, "Running");
    let status = command.status().wrap_err("failed to run cargo test")?;
    tracing::trace!(status_code = %status, command = ?command, "Finished");
    if !status.success() && !status.success() {
        // We explicitly do not want to return a spantraced error here.
        std::process::exit(1)
    }

    Ok(())
}

fn resolve_test_args<F>(
    pg_version: Option<String>,
    mut testnames: Vec<String>,
    is_pg_selector: F,
) -> (Option<String>, Vec<String>)
where
    F: FnOnce(&str) -> bool,
{
    match pg_version {
        Some(first) if is_pg_selector(&first) => (Some(first), testnames),
        Some(first) => {
            testnames.insert(0, first);
            (None, testnames)
        }
        None => (None, testnames),
    }
}

fn apply_test_filters_to_command(command: &mut std::process::Command, testnames: &[String]) {
    if !testnames.is_empty() {
        command.arg("--");
        command.args(testnames);
    }
}

fn apply_resolved_manifest_to_test_command(
    command: &mut std::process::Command,
    package_manifest_path: &Path,
) {
    command
        .env("PGRX_MANIFEST_PATH", package_manifest_path)
        .arg("--manifest-path")
        .arg(package_manifest_path);
}

#[cfg(test)]
mod tests {
    use super::{Test, apply_resolved_manifest_to_test_command, apply_test_filters_to_command};
    use clap::{Args, Parser, Subcommand};
    use std::path::Path;
    use std::process::Command;

    fn strings(args: &[&str]) -> Vec<String> {
        args.iter().map(|arg| (*arg).to_string()).collect()
    }

    #[derive(Parser)]
    struct CargoCli {
        #[clap(subcommand)]
        subcommand: CargoSubcommand,
        #[clap(short = 'v', long, action = clap::ArgAction::Count, global = true)]
        verbose: u8,
    }

    #[derive(Subcommand)]
    enum CargoSubcommand {
        Pgrx(PgrxCli),
    }

    #[derive(Args)]
    struct PgrxCli {
        #[clap(subcommand)]
        subcommand: PgrxSubcommand,
    }

    #[derive(Subcommand)]
    enum PgrxSubcommand {
        Test(Test),
    }

    #[test]
    fn test_cli_accepts_multiple_testnames() {
        let parsed = CargoCli::try_parse_from([
            "cargo",
            "pgrx",
            "test",
            "--package",
            "extension",
            "pg18",
            "test_a",
            "test_b",
            "test_n",
        ])
        .expect("multiple test filters should parse");

        let CargoSubcommand::Pgrx(PgrxCli { subcommand: PgrxSubcommand::Test(test) }) =
            parsed.subcommand;
        assert_eq!(test.pg_version.as_deref(), Some("pg18"));
        assert_eq!(test.testnames, strings(&["test_a", "test_b", "test_n"]));
    }

    #[test]
    fn resolve_test_args_treats_non_version_first_arg_as_testname() {
        let (pg_version, testnames) = super::resolve_test_args(
            Some("test_a".to_string()),
            strings(&["test_b", "test_n"]),
            |arg| arg == "all" || arg == "pg18",
        );

        assert_eq!(pg_version, None);
        assert_eq!(testnames, strings(&["test_a", "test_b", "test_n"]));
    }

    #[test]
    fn resolve_test_args_keeps_pg_selector() {
        let (pg_version, testnames) = super::resolve_test_args(
            Some("pg18".to_string()),
            strings(&["test_a", "test_b"]),
            |arg| arg == "all" || arg == "pg18",
        );

        assert_eq!(pg_version.as_deref(), Some("pg18"));
        assert_eq!(testnames, strings(&["test_a", "test_b"]));
    }

    #[test]
    fn test_filters_are_passed_to_libtest_after_separator() {
        let mut command = Command::new("cargo");
        command.arg("test");
        apply_test_filters_to_command(&mut command, &strings(&["test_a", "test_b", "test_n"]));

        let args = command.get_args().map(|arg| arg.to_string_lossy()).collect::<Vec<_>>();
        assert_eq!(args, ["test", "--", "test_a", "test_b", "test_n"]);
    }

    #[test]
    fn test_command_targets_resolved_manifest_for_outer_and_inner_builds() {
        let package_manifest_path = Path::new("/workspace/postgres/Cargo.toml");
        let mut command = Command::new("cargo");
        command.arg("test");

        apply_resolved_manifest_to_test_command(&mut command, package_manifest_path);

        let args = command.get_args().map(|arg| arg.to_string_lossy()).collect::<Vec<_>>();
        assert!(
            args.windows(2).any(|window| {
                window[0].as_ref() == "--manifest-path"
                    && window[1].as_ref() == package_manifest_path.to_string_lossy()
            }),
            "outer cargo test should target the resolved package manifest: {args:?}"
        );

        let manifest_env = command
            .get_envs()
            .find_map(|(key, value)| (key == "PGRX_MANIFEST_PATH").then_some(value))
            .flatten();
        assert_eq!(
            manifest_env,
            Some(package_manifest_path.as_os_str()),
            "inner pgrx-test install should receive the resolved package manifest"
        );
    }
}
