/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::CommandExecute;
// use pgx_pg_config::{PgConfig as pgCfg};
use std::path::PathBuf;

/// Interact with the pg_config that pgx is using for various postgres installations
#[derive(clap::Args, Debug, Clone)]
#[clap(author)]
pub(crate) struct PgConfig {
    /// The Postgres version to start (`pg11`, `pg12`, `pg13`, `pg14`, `pg15`, or `all`)
    #[clap(env = "PG_VERSION")]
    pg_version: Option<String>,
    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
    /// Package to determine default `pg_version` with (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
}

impl CommandExecute for PgConfig {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        // get PGX_HOME from env
        // read/parse {PGX_HOME}/config.toml
        // select config based on self.pg_version
        // print out the full path to the pg_config
        println!("pg_version: {:?}", self.pg_version);
        Ok(())
    }
}
