/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::CommandExecute;

/// Interact with the pg_config that pgx is using for various postgres installations
#[derive(clap::Args, Debug, Clone)]
#[clap(author)]
pub(crate) struct PgConfig {
    /// The Postgres version to start (`pg11`, `pg12`, `pg13`, `pg14`, `pg15`, or `all`)
    #[clap(env = "PG_VERSION")]
    pg_version: Option<String>,
    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
}

impl CommandExecute for PgConfig {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        // 1. get PGX_HOME from env
        // 2. read/parse {PGX_HOME}/config.toml
        // 3. select config based on self.pg_version
        // 4. print out the full path to the pg_config
        Ok(())
    }
}
