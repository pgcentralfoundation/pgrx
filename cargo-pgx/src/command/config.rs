/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use crate::CommandExecute;
use pgx_pg_config::Pgx;

/// Interact with the pg_config that pgx is using for various postgres installations
#[derive(clap::Args, Debug, Clone)]
#[clap(author)]
pub(crate) struct PgConfig {
    /// The Postgres version to start (`pg11`, `pg12`, `pg13`, `pg14`, `pg15`, or `all`)
    #[clap(env = "PG_VERSION")]
    pg_version: Option<String>,
    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
    /// The PgxConfig attribute to get
    attribute: Option<String>,
}

impl CommandExecute for PgConfig {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let t = Pgx::from_config()?;

        if self.pg_version.is_none() {
            println!("{:#?}", t);
            return Ok(());
        }

        let result = t.get(&self.pg_version.unwrap())?;
        if self.attribute.is_none() {
            println!("{:#?}", result);
            return Ok(());
        }
        match self.attribute.unwrap().as_str() {
            "version" => {
                println!("{:?}", result.version);
            }
            "pg_config" => {
                println!("{}", result.pg_config.unwrap().to_string_lossy());
            }
            "known_props" => {
                println!("{:?}", result.known_props);
            }
            "base_port" => {
                println!("{:?}", result.base_port);
            }
            "base_testing_port" => {
                println!("{:?}", result.base_testing_port);
            }
            _ => {
                println!("Unknown attribute");
            }
        }
        Ok(())
    }
}
