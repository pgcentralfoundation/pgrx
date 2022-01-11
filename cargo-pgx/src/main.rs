// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

mod commands;

use clap::Parser;
use pgx_utils::exit_with_error;
use pgx_utils::handle_result;
const SUPPORTED_MAJOR_VERSIONS: &[u16] = &[10, 11, 12, 13, 14];

trait CommandExecute {
    fn execute(self) -> std::result::Result<(), std::io::Error>;
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
}

impl CommandExecute for CargoCommand {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
        self.subcommand.execute()
    }
}

#[derive(clap::Subcommand, Debug)]
enum CargoSubcommands {
    Pgx(CargoPgxCommand),
}

impl CommandExecute for CargoSubcommands {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
        use CargoSubcommands::*;
        match self {
            Pgx(c) => c.execute(),
        }
    }
}

#[derive(clap::Args, Debug)]
#[clap(about, author)]
struct CargoPgxCommand {
    #[clap(subcommand)]
    subcommand: CargoPgxSubCommands,
}

impl CommandExecute for CargoPgxCommand {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
        self.subcommand.execute()
    }
}

#[derive(clap::Subcommand, Debug)]
enum CargoPgxSubCommands {
    Init(commands::init::Init),
    Start(commands::start::Start),
    Stop(commands::stop::Stop),
    Status(commands::status::Status),
    New(commands::new::New),
    Install(commands::install::Install),
    Package(commands::package::Package),
    Schema(commands::schema::Schema),
    Run(commands::run::Run),
    Connect(commands::connect::Connect),
    Test(commands::test::Test),
    Get(commands::get::Get),
}

impl CommandExecute for CargoPgxSubCommands {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
        use CargoPgxSubCommands::*;
        match self {
            Init(c) => c.execute(),
            Start(c) => c.execute(),
            Stop(c) => c.execute(),
            Status(c) => c.execute(),
            New(c) => c.execute(),
            Install(c) => c.execute(),
            Package(c) => c.execute(),
            Schema(c) => c.execute(),
            Run(c) => c.execute(),
            Connect(c) => c.execute(),
            Test(c) => c.execute(),
            Get(c) => c.execute(),
        }
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    handle_result!(do_it(), "");
    Ok(())
}

fn do_it() -> std::result::Result<(), std::io::Error> {
    let cargo_cli = CargoCommand::parse();
    cargo_cli.execute()
}
