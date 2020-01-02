#[macro_use]
extern crate clap;
use clap::App;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::result::Result;
use std::str::FromStr;

fn main() -> std::result::Result<(), std::io::Error> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from(yaml);

    let matches = app.get_matches();

    if let Some(extension) = matches.subcommand_matches("pgx") {
        if let Some(create) = extension.subcommand_matches("create") {
            let name = create
                .value_of("name")
                .expect("<NAME> argument to create is required");

            let path = PathBuf::from_str(&format!("{}/", name)).unwrap();

            create_directory_structure(&path)?;
            create_control_file(&path, name)?;
            create_cargo_toml(&path, name)?;
            create_dotcargo_config(&path, name)?;
            create_lib_rs(&path, name)?;
            git_init(&path)?;
        } else if let Some(_install) = extension.subcommand_matches("install") {
            println!("installing extension");
        } else {
            eprintln!("{}", extension.usage());
        }
    } else {
        eprintln!("{}", matches.usage());
    }

    Ok(())
}

fn create_directory_structure(path: &PathBuf) -> Result<(), std::io::Error> {
    let mut src_dir = path.clone();

    src_dir.push("src");
    std::fs::create_dir_all(&src_dir)?;

    src_dir.pop();
    src_dir.push(".cargo");
    std::fs::create_dir_all(&src_dir)
}

fn create_control_file(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push(format!("{}.control", name));
    let mut file = std::fs::File::create(filename)?;

    file.write_all(
        &format!(
            "
comment = '{name}:  Created by pg-rs-bridge'
default_version = '1.0'
module_pathname = '$libdir/{name}'
relocatable = true
schema = {name}
superuser = false
",
            name = name
        )
        .as_bytes(),
    )?;

    Ok(())
}

fn create_cargo_toml(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push("Cargo.toml");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(
        &format!(
            "
[package]
name = \"{name}\"
version = \"0.1.0\"
edition = \"2018\"

[lib]
crate-type = [\"cdylib\"]

[dependencies]
pg-bridge = {{ path = \"../pg-rs-bridge/pg-bridge/\", features = [\"pg11\"], default-features = false }}
pg-guard-attr = {{ path = \"../pg-rs-bridge/pg-guard-attr\" }}
pg-guard = {{ path = \"../pg-rs-bridge/pg-guard\" }}

[profile.dev]
panic = \"unwind\"

[profile.release]
panic = \"unwind\"
opt-level = 3
lto = true
codegen-units = 1                                                                                                        
",
            name = name
        )
        .as_bytes(),
    )?;

    Ok(())
}

fn create_dotcargo_config(path: &PathBuf, _name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push(".cargo");
    filename.push("config");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(
        &format!(
            "
[build]
# Postgres symbols won't ve available until runtime
rustflags = [\"-C\", \"link-args=-Wl,-undefined,dynamic_lookup\"]
"
        )
        .as_bytes(),
    )?;

    Ok(())
}

fn create_lib_rs(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push("src");
    filename.push("lib.rs");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(
        &format!(
            "
extern crate pg_guard;

use pg_bridge::*;

pg_module_magic!();

#[pg_extern]
fn hello() {{
    info!(\"Hello from {name}!\");
}}
",
            name = name
        )
        .as_bytes(),
    )?;

    Ok(())
}

fn git_init(path: &PathBuf) -> Result<(), std::io::Error> {
    let output = Command::new("git")
        .arg("init")
        .arg(".")
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        Err(std::io::Error::from_raw_os_error(
            output.status.code().unwrap(),
        ))
    } else {
        Ok(())
    }
}
