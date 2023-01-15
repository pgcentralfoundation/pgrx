/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2023 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::CommandExecute;
pub(crate) mod pgx_target;

/// Commands having to do with cross-compilation. (Experimental)
#[derive(clap::Args, Debug)]
#[clap(about, author)]
pub(crate) struct Cross {
    #[command(subcommand)]
    pub(crate) subcommand: CargoPgxCrossSubCommands,
}

impl CommandExecute for Cross {
    fn execute(self) -> eyre::Result<()> {
        self.subcommand.execute()
    }
}

/// Subcommands relevant to cross-compilation.
#[derive(clap::Subcommand, Debug)]
pub(crate) enum CargoPgxCrossSubCommands {
    PgxTarget(pgx_target::PgxTarget),
}

impl CommandExecute for CargoPgxCrossSubCommands {
    fn execute(self) -> eyre::Result<()> {
        use CargoPgxCrossSubCommands::*;
        match self {
            PgxTarget(target_info) => target_info.execute(),
        }
    }
}
