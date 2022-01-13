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
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c', parse(from_os_str))]
    pg_config: Option<PathBuf>,
    /// Additional cargo features to activate (default is '--no-default-features')
    #[clap(long)]
    features: Vec<String>,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Package {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let pg_config = match self.pg_config {
            None => PgConfig::from_path(),
            Some(config) => PgConfig::new(PathBuf::from(config)),
        };
        package_extension(&pg_config, self.debug, &self.features)
    }
}

#[tracing::instrument(level = "error", skip_all, fields(
    pg_version = %pg_config.version()?,
    release = !is_debug,
))]
pub(crate) fn package_extension(
    pg_config: &PgConfig,
    is_debug: bool,
    additional_features: &Vec<impl AsRef<str>>,
) -> eyre::Result<()> {
    let base_path = build_base_path(pg_config, is_debug)?;

    if base_path.exists() {
        std::fs::remove_dir_all(&base_path)?;
    }

    if !base_path.exists() {
        std::fs::create_dir_all(&base_path)?;
    }
    install_extension(
        pg_config,
        !is_debug,
        false,
        Some(base_path),
        additional_features,
    )
}

fn build_base_path(pg_config: &PgConfig, is_debug: bool) -> eyre::Result<PathBuf> {
    let mut target_dir = get_target_dir()?;
    let pgver = pg_config.major_version()?;
    let extname = get_property("extname")?.ok_or(eyre!("could not determine extension name"))?;
    target_dir.push(if is_debug { "debug" } else { "release" });
    target_dir.push(format!("{}-pg{}", extname, pgver));
    Ok(target_dir)
}
