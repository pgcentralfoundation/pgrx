// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

#[macro_use]
extern crate clap;

mod commands;

use clap::{Parser, SubCommand, AppSettings};
use colored::Colorize;
use pgx_utils::handle_result;
use pgx_utils::pg_config::{PgConfig, PgConfigSelector, Pgx};
use pgx_utils::{exit, exit_with_error};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

const SUPPORTED_MAJOR_VERSIONS: &[u16] = &[10, 11, 12, 13, 14];

fn main() -> std::result::Result<(), std::io::Error> {
    handle_result!(do_it(), "");
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct CargoPgx {
    #[clap(subcommand)]
    command: CargoPgxCommand,
}

impl PgxCommand for CargoPgx {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
        use CargoPgxCommand::*;
        match self.command {
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

#[derive(Subcommand, Debug)]
enum CargoPgxCommand {
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

trait PgxCommand {
    fn execute(self) -> std::result::Result<(), std::io::Error>;
}

fn do_it() -> std::result::Result<(), std::io::Error> {
    let cargo_pgx = CargoPgx::parse();
    cargo_pgx.execute()
}
