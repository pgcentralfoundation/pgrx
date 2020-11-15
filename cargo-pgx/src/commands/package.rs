// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::get::get_property;
use crate::commands::install::install_extension;
use pgx_utils::get_target_dir;
use pgx_utils::pg_config::PgConfig;
use std::path::PathBuf;

pub(crate) fn package_extension(
    pg_config: &PgConfig,
    is_debug: bool,
) -> Result<(), std::io::Error> {
    let base_path = build_base_path(pg_config, is_debug)?;

    if base_path.exists() {
        std::fs::remove_dir_all(&base_path)?;
    }

    if !base_path.exists() {
        std::fs::create_dir_all(&base_path)?;
    }
    install_extension(pg_config, !is_debug, Some(base_path))
}

fn build_base_path(pg_config: &PgConfig, is_debug: bool) -> Result<PathBuf, std::io::Error> {
    let mut target_dir = get_target_dir();
    let pgver = pg_config.major_version()?;
    let extname = get_property("extname").expect("could not determine extension name");
    target_dir.push(if is_debug { "debug" } else { "release" });
    target_dir.push(format!("{}-pg{}", extname, pgver));
    Ok(target_dir)
}
