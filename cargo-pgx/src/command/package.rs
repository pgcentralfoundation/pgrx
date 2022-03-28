/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{
    command::{get::get_property, install::install_extension},
    CommandExecute,
};
use cargo_toml::Manifest;
use eyre::{eyre, WrapErr};
use pgx_utils::{get_target_dir, pg_config::PgConfig};
use std::path::{Path, PathBuf};

/// Create an installation package directory.
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Package {
    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, parse(from_os_str))]
    manifest_path: Option<PathBuf>,
    /// Compile for debug mode (default is release)
    #[clap(env = "PROFILE", long, short)]
    debug: bool,
    /// Build in test mode (for `cargo pgx test`)
    #[clap(long)]
    test: bool,
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c', parse(from_os_str))]
    pg_config: Option<PathBuf>,
    /// The directory to output the package (default is `./target/[debug|release]/extname-pgXX/`)
    #[clap(long, parse(from_os_str))]
    out_dir: Option<PathBuf>,
    #[clap(flatten)]
    features: clap_cargo::Features,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Package {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let metadata = crate::metadata::metadata(&self.features, self.manifest_path.as_ref())
            .wrap_err("couldn't get cargo metadata")?;
        crate::metadata::validate(&metadata)?;
        let package_manifest_path =
            crate::manifest::manifest_path(&metadata, self.package.as_ref())
                .wrap_err("Couldn't get manifest path")?;
        let package_manifest =
            Manifest::from_path(&package_manifest_path).wrap_err("Couldn't parse manifest")?;

        let pg_config = match self.pg_config {
            None => PgConfig::from_path(),
            Some(config) => PgConfig::new(PathBuf::from(config)),
        };
        let pg_version = format!("pg{}", pg_config.major_version()?);

        let features =
            crate::manifest::features_for_version(self.features, &package_manifest, &pg_version);

        let out_dir = if let Some(out_dir) = self.out_dir {
            out_dir
        } else {
            build_base_path(&pg_config, &package_manifest_path, self.debug)?
        };
        package_extension(
            self.manifest_path.as_ref(),
            self.package.as_ref(),
            &package_manifest_path,
            &pg_config,
            out_dir,
            self.debug,
            self.test,
            &features,
        )
    }
}

#[tracing::instrument(level = "error", skip_all, fields(
    pg_version = %pg_config.version()?,
    release = !is_debug,
    test = is_test,
))]
pub(crate) fn package_extension(
    user_manifest_path: Option<impl AsRef<Path>>,
    user_package: Option<&String>,
    package_manifest_path: impl AsRef<Path>,
    pg_config: &PgConfig,
    out_dir: PathBuf,
    is_debug: bool,
    is_test: bool,
    features: &clap_cargo::Features,
) -> eyre::Result<()> {
    if !out_dir.exists() {
        std::fs::create_dir_all(&out_dir)?;
    }

    install_extension(
        user_manifest_path,
        user_package,
        &package_manifest_path,
        pg_config,
        !is_debug,
        is_test,
        Some(out_dir),
        features,
    )
}

fn build_base_path(
    pg_config: &PgConfig,
    manifest_path: impl AsRef<Path>,
    is_debug: bool,
) -> eyre::Result<PathBuf> {
    let mut target_dir = get_target_dir()?;
    let pgver = pg_config.major_version()?;
    let extname = get_property(manifest_path, "extname")?
        .ok_or(eyre!("could not determine extension name"))?;
    target_dir.push(if is_debug { "debug" } else { "release" });
    target_dir.push(format!("{}-pg{}", extname, pgver));
    Ok(target_dir)
}
