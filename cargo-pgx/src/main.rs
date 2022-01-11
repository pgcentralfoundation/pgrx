// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

mod command;

use clap::Parser;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
const SUPPORTED_MAJOR_VERSIONS: &[u16] = &[10, 11, 12, 13, 14];

trait CommandExecute {
    fn execute(self) -> eyre::Result<()>;
}

/// `cargo` stub for `cargo-pgx` (you probably meant to run `cargo pgx`)
#[derive(clap::Parser, Debug)]
#[clap(
    name = "cargo",
    bin_name = "cargo",
    version,
    global_setting(clap::AppSettings::PropagateVersion)
)]
struct CargoCommand {
    #[clap(subcommand)]
    subcommand: CargoSubcommands,
    /// Enable info logs, -vv for debug, -vvv for trace
    #[clap(short = 'v', long, parse(from_occurrences), global = true)]
    verbose: usize,
}

impl CommandExecute for CargoCommand {
    fn execute(self) -> eyre::Result<()> {
        self.subcommand.execute()
    }
}

#[derive(clap::Subcommand, Debug)]
enum CargoSubcommands {
    Pgx(command::pgx::Pgx),
}

impl CommandExecute for CargoSubcommands {
    fn execute(self) -> eyre::Result<()> {
        use CargoSubcommands::*;
        match self {
            Pgx(c) => c.execute(),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    let cargo_cli = CargoCommand::parse();

    // Initialize tracing with tracing-error, and eyre
    let fmt_layer = tracing_subscriber::fmt::Layer::new()
        .pretty();

    // Unless the user opts in specifically we don't want to impact `cargo-pgx schema` output.
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| match cargo_cli.verbose {
            0 => EnvFilter::try_new("warn"),
            1 => EnvFilter::try_new("info"),
            2 => EnvFilter::try_new("debug"),
            _ => EnvFilter::try_new("trace"),
        })
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    color_eyre::install()?;

    cargo_cli.execute()
}
