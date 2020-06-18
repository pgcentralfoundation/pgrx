// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

#[macro_use]
extern crate clap;

mod crate_template;
mod extension_installer;
mod property_inspector;
mod schema_generator;
mod test_runner;

use clap::App;
use crate_template::*;
use extension_installer::*;
use property_inspector::*;
use schema_generator::*;
use std::path::PathBuf;
use std::str::FromStr;
use test_runner::*;

fn main() -> std::result::Result<(), std::io::Error> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from(yaml);

    let matches = app.get_matches();

    if let Some(extension) = matches.subcommand_matches("pgx") {
        if let Some(create) = extension.subcommand_matches("new") {
            let name = create
                .value_of("name")
                .expect("<NAME> argument to create is required");
            // TODO:  validate name to make sure it's all ascii [a-z0-9_]
            let path = PathBuf::from_str(&format!("{}/", name)).unwrap();
            create_crate_template(path, name)?;
        } else if let Some(install) = extension.subcommand_matches("install") {
            let target = install.is_present("release");
            install_extension(target)?;
        } else if let Some(_schema) = extension.subcommand_matches("schema") {
            generate_schema()?;
        } else if let Some(test) = extension.subcommand_matches("test") {
            let version = test.value_of("pg_version").unwrap_or("all");
            match version {
                "pg10" | "pg11" | "pg12" | "all" => test_extension(version)?,
                _ => panic!("Unrecognized version: {}", version),
            }
        } else if let Some(get) = extension.subcommand_matches("get") {
            let name = get.value_of("name").expect("no property name specified");
            if let Some(value) = get_property(name) {
                println!("{}", value);
            }
        } else {
            eprintln!("{}", extension.usage());
        }
    } else {
        eprintln!("{}", matches.usage());
    }

    Ok(())
}
