/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
#![deny(clippy::perf)] // our compile times are awful
#![allow(clippy::or_fun_call)] // often false positives

mod command;
mod manifest;
mod metadata;
mod pgrx_pg_sys_stub;

pub(crate) mod env;
pub(crate) mod profile;

use atty::Stream;
use clap::Parser;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

trait CommandExecute {
    fn execute(self) -> eyre::Result<()>;
}

/// `cargo` stub for `cargo-pgrx` (you probably meant to run `cargo pgrx`)
#[derive(clap::Parser, Debug)]
#[clap(name = "cargo", bin_name = "cargo", version, propagate_version = true)]
struct CargoCommand {
    #[clap(subcommand)]
    subcommand: CargoSubcommands,
    /// Enable info logs, -vv for debug, -vvv for trace
    #[clap(short = 'v', long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,
}

impl CommandExecute for CargoCommand {
    fn execute(self) -> eyre::Result<()> {
        self.subcommand.execute()
    }
}

#[derive(clap::Subcommand, Debug)]
enum CargoSubcommands {
    Pgrx(command::pgrx::Pgrx),
}

impl CommandExecute for CargoSubcommands {
    fn execute(self) -> eyre::Result<()> {
        use CargoSubcommands::*;
        match self {
            Pgrx(c) => c.execute(),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    env::initialize();
    color_eyre::config::HookBuilder::default()
        .theme(if !atty::is(Stream::Stderr) {
            color_eyre::config::Theme::new()
        } else {
            color_eyre::config::Theme::default()
        })
        .install()?;

    let cargo_cli = CargoCommand::parse();

    // Initialize tracing with tracing-error, and eyre
    let fmt_layer = tracing_subscriber::fmt::Layer::new()
        .with_ansi(atty::is(Stream::Stderr))
        .with_writer(std::io::stderr)
        .pretty();

    let filter_layer = match EnvFilter::try_from_default_env() {
        Ok(filter_layer) => filter_layer,
        Err(_) => {
            let log_level = match cargo_cli.verbose {
                0 => "info",
                1 => "debug",
                _ => "trace",
            };
            let filter_layer = EnvFilter::new("warn");
            let filter_layer =
                filter_layer.add_directive(format!("cargo_pgrx={}", log_level).parse()?);
            let filter_layer = filter_layer.add_directive(format!("pgrx={}", log_level).parse()?);
            let filter_layer =
                filter_layer.add_directive(format!("pgrx_macros={}", log_level).parse()?);
            let filter_layer =
                filter_layer.add_directive(format!("pgrx_tests={}", log_level).parse()?);
            let filter_layer =
                filter_layer.add_directive(format!("pgrx_pg_sys={}", log_level).parse()?);
            let filter_layer =
                filter_layer.add_directive(format!("pgrx_utils={}", log_level).parse()?);
            filter_layer
        }
    };

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    cargo_cli.execute()
}
