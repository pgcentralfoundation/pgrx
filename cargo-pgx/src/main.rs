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
use crate::commands::start::start_postgres;
use crate::commands::status::status_postgres;
use crate::commands::stop::stop_postgres;
use crate::commands::test::test_extension;
use clap::App;
use colored::Colorize;
use pgx_utils::{exit, exit_with_error, get_pg_config};
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
            ("start", Some(start)) => {
                let pgver = start
                    .value_of("pg_version")
                    .expect("<PG_VERSION> argument is required");
                start_postgres(make_pg_major_version(pgver))
            }
            ("stop", Some(start)) => {
                let pgver = start
                    .value_of("pg_version")
                    .expect("<PG_VERSION> argument is required");
                stop_postgres(make_pg_major_version(pgver))
            }
            ("status", Some(start)) => {
                let pgver = start.value_of("pg_version").unwrap_or("all");

                let versions = if pgver == "all" {
                    vec![10, 11, 12]
                } else {
                    vec![make_pg_major_version(pgver)]
                };

                for major_version in versions {
                    if status_postgres(major_version) {
                        println!(
                            "Postgres v{} is {}",
                            major_version,
                            "running".bold().green()
                        )
                    } else {
                        println!("Postgres v{} is {}", major_version, "stopped".bold().red())
                    }
                }
                Ok(())
            }
            ("install", Some(install)) => {
                let target = install.is_present("release");
                let pg_config = match std::env::var("PGX_TEST_MODE_VERSION") {
                    // for test mode, we want the pg_config specified in PGX_TEST_MODE_VERSION
                    Ok(pgver) => get_pg_config(u16::from_str(&pgver).expect(
                        "PGX_TEST_MODE_VERSION does not contain a valid postgres version number",
                    )),

                    // otherwise, the user just ran "cargo pgx install", and we use whatever "pg_config" is on the path
                    Err(_) => Some("pg_config".to_string()),
                };
                install_extension(&pg_config, target)
            }
            ("test", Some(test)) => {
                let pgver = test.value_of("pg_version").unwrap_or("all");
                let versions = if pgver == "all" {
                    vec![10, 11, 12]
                } else {
                    vec![make_pg_major_version(pgver)]
                };

                for major_version in versions {
                    test_extension(major_version);
                }
                Ok(())
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

fn make_pg_major_version(version_string: &str) -> u16 {
    match version_string {
        "pg10" => 10,
        "pg11" => 11,
        "pg12" => 12,
        _ => exit_with_error!("unrecognized Postgres version: {}", version_string),
    }
}
