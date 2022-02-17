// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{
    command::{get::get_property, install::install_extension},
    CommandExecute,
};
use eyre::eyre;
use pgx_utils::{get_target_dir, pg_config::PgConfig};
use std::path::PathBuf;

/// Create an installation package directory (in `./target/[debug|release]/extname-pgXX/`).
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Package {
    /// Compile for debug mode (default is release)
    #[clap(env = "PROFILE", long, short)]
    debug: bool,
    /// Build in test mode (for `cargo pgx test`)
    #[clap(long)]
    test: bool,
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c', parse(from_os_str))]
    pg_config: Option<PathBuf>,
    #[clap(flatten)]
    features: clap_cargo::Features,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Package {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let metadata = crate::metadata::metadata(&self.features)?;
        crate::metadata::validate(&metadata)?;
        let manifest = crate::manifest::manifest(&metadata)?;

        let pg_config = match self.pg_config {
            None => PgConfig::from_path(),
            Some(config) => PgConfig::new(PathBuf::from(config)),
        };
        let pg_version = format!("pg{}", pg_config.major_version()?);
        let features = crate::manifest::features_for_version(self.features, &manifest, &pg_version);

        package_extension(&manifest, &pg_config, self.debug, self.test, &features)
    }
}

#[tracing::instrument(level = "error", skip_all, fields(
    pg_version = %pg_config.version()?,
    release = !is_debug,
    test = is_test,
))]
pub(crate) fn package_extension(
    manifest: &cargo_toml::Manifest,
    pg_config: &PgConfig,
    is_debug: bool,
    is_test: bool,
    features: &clap_cargo::Features,
) -> eyre::Result<()> {
    let base_path = build_base_path(pg_config, is_debug)?;

    if base_path.exists() {
        std::fs::remove_dir_all(&base_path)?;
    }

    if !base_path.exists() {
        std::fs::create_dir_all(&base_path)?;
    }
    install_extension(manifest, pg_config, !is_debug, is_test, false, Some(base_path), features)
}

fn build_base_path(pg_config: &PgConfig, is_debug: bool) -> eyre::Result<PathBuf> {
    let mut target_dir = get_target_dir()?;
    let pgver = pg_config.major_version()?;
    let extname = get_property("extname")?.ok_or(eyre!("could not determine extension name"))?;
    target_dir.push(if is_debug { "debug" } else { "release" });
    target_dir.push(format!("{}-pg{}", extname, pgver));
    Ok(target_dir)
}
