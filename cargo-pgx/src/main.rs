// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

#[macro_use]
extern crate clap;

mod commands;

use crate::commands::get::get_property;
use crate::commands::init::init_pgx;
use crate::commands::install::install_extension;
use crate::commands::new::create_crate_template;
use crate::commands::schema::generate_schema;
use crate::commands::test::test_extension;
use clap::App;
use pgx_utils::{exit, exit_with_error};
use std::path::PathBuf;
use std::str::FromStr;

fn main() -> std::result::Result<(), std::io::Error> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from(yaml);

    let matches = app.get_matches();

    if let Some(extension) = matches.subcommand_matches("pgx") {
        let result = match extension.subcommand() {
            ("init", Some(init)) => {
                let pg10_path = init.value_of("pg10");
                let pg11_path = init.value_of("pg11");
                let pg12_path = init.value_of("pg12");

                init_pgx(pg10_path, pg11_path, pg12_path)
            }
            ("new", Some(new)) => {
                let extname = new
                    .value_of("name")
                    .expect("<NAME> argument to create is required");
                validate_extension_name(extname);
                let path = PathBuf::from_str(&format!("{}/", extname)).unwrap();
                create_crate_template(path, extname)
            }
            ("install", Some(install)) => {
                let target = install.is_present("release");
                install_extension(target)
            }
            ("test", Some(test)) => {
                let version = test.value_of("pg_version").unwrap_or("all");
                match version {
                    "pg10" | "pg11" | "pg12" | "all" => test_extension(version),
                    _ => panic!("Unrecognized version: {}", version),
                }
            }
            ("schema", Some(_schema)) => generate_schema(),
            ("get", Some(get)) => {
                let name = get.value_of("name").expect("no property name specified");
                if let Some(value) = get_property(name) {
                    println!("{}", value);
                }
                Ok(())
            }
            _ => exit!(extension.usage()),
        };

        if let Err(e) = result {
            exit!("{}", e)
        }
    } else {
        exit!(matches.usage())
    }

    Ok(())
}

fn validate_extension_name(extname: &str) {
    for c in extname.chars() {
        if !c.is_alphanumeric() && c != '_' && !c.is_lowercase() {
            exit_with_error!("Extension name must be in the set of [a-z0-9_]")
        }
    }
}
