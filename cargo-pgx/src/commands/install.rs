// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::get::{find_control_file, get_property};
use crate::commands::schema::read_load_order;
use colored::Colorize;
use pgx_utils::{exit_with_error, get_target_dir, handle_result, run_pg_config};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;

pub(crate) fn install_extension(pg_config: &Option<String>, is_release: bool) {
    let (control_file, extname) = find_control_file();
    let major_version = get_pg_config_major_version(pg_config);

    build_extension(major_version, is_release);

    println!();
    println!("installing extension");
    let pkgdir = get_pkglibdir(pg_config);
    let extdir = get_extensiondir(pg_config);
    let (libpath, libfile) = find_library_file(&extname, is_release);

    let src = control_file.clone();
    let dest = format!("{}/{}", extdir, control_file);
    handle_result!(
        format!(
            "failed copying control file `{}` to `{}`",
            control_file, extdir
        ),
        std::fs::copy(&src, &dest)
    );

    println!("{} control file to {}", "     Copying".bold().green(), dest);

    let src = format!("{}/{}", libpath, libfile);
    let dest = format!("{}/{}.so", pkgdir, extname);
    handle_result!(
        format!("failed copying `{}` to `{}`", libfile, pkgdir),
        std::fs::copy(&src, &dest)
    );
    println!(
        "{} shared library to {}",
        "     Copying".bold().green(),
        dest
    );

    handle_result!("failed to generate SQL schema", crate::generate_schema());
    copy_sql_files(&extdir, &extname);

    println!("{} installing {}", "    Finished".bold().green(), extname);
}

fn build_extension(major_version: u16, is_release: bool) {
    let target_dir = get_target_dir();
    let features = std::env::var("PGX_BUILD_FEATURES").unwrap_or(format!("pg{}", major_version));
    let flags = std::env::var("PGX_BUILD_FLAGS").unwrap_or_default();
    let mut command = Command::new("cargo");
    command.arg("build");
    if is_release {
        command.arg("--release");
    }

    if !features.trim().is_empty() {
        command.arg("--features");
        command.arg(&features);
        command.arg("--no-default-features");
    }

    command.arg("--target-dir");
    command.arg(target_dir.display().to_string());

    for arg in flags.split_ascii_whitespace() {
        command.arg(arg);
    }

    let command = command
        .env("CARGO_TARGET_DIR", target_dir.display().to_string())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    let command_str = format!("{:?}", command);
    println!(
        "building extension with features `{}`\n{}",
        features, command_str
    );
    let status = handle_result!(
        format!("failed to spawn cargo: {}", command_str),
        command.status()
    );
    if !status.success() {
        exit_with_error!("failed to build extension");
    }
}

fn copy_sql_files(extdir: &str, extname: &str) {
    let load_order = read_load_order(&PathBuf::from_str("./sql/load-order.txt").unwrap());
    let target_filename =
        PathBuf::from_str(&format!("{}/{}--{}.sql", extdir, extname, get_version())).unwrap();
    let mut sql = std::fs::File::create(&target_filename).unwrap();
    println!(
        "{} {}",
        "     Writing".bold().green(),
        target_filename.display()
    );

    // write each sql file from load-order.txt to the version.sql file
    for file in load_order {
        let file = PathBuf::from_str(&format!("sql/{}", file)).unwrap();
        let pwd = std::env::current_dir().expect("no current directory");
        let contents = std::fs::read_to_string(&file).expect(&format!(
            "could not open {}/{}",
            pwd.display(),
            file.display()
        ));

        sql.write_all(b"--\n")
            .expect("couldn't write version SQL file");
        sql.write_all(format!("-- {}\n", file.display()).as_bytes())
            .expect("couldn't write version SQL file");
        sql.write_all(b"--\n")
            .expect("couldn't write version SQL file");
        sql.write_all(contents.as_bytes())
            .expect("couldn't write version SQL file");
        sql.write_all(b"\n\n\n")
            .expect("couldn't write version SQL file");
    }

    // now copy all the version upgrade files too
    for f in handle_result!("failed to read ./sql/ directory", std::fs::read_dir("sql/")) {
        if let Ok(f) = f {
            let filename = f.file_name().into_string().unwrap();

            if filename.starts_with(&format!("{}--", extname)) && filename.ends_with(".sql") {
                let dest = format!("{}/{}", extdir, filename);

                if let Err(e) = std::fs::copy(f.path(), &dest) {
                    exit_with_error!("failed copying SQL {} to {}:  {}", filename, dest, e)
                }
            }
        }
    }
}

fn find_library_file(extname: &str, is_release: bool) -> (String, String) {
    let mut target_dir = get_target_dir();
    target_dir.push(if is_release { "release" } else { "debug" });

    if !target_dir.exists() {
        exit_with_error!("target directory does not exist: {}", target_dir.display());
    }

    for f in handle_result!(
        format!("Unable to read {}", target_dir.display()),
        std::fs::read_dir(&target_dir)
    ) {
        if let Ok(f) = f {
            let filename = f.file_name().into_string().unwrap();

            if filename.contains(extname)
                && filename.starts_with("lib")
                && (filename.ends_with(".so")
                    || filename.ends_with(".dylib")
                    || filename.ends_with(".dll"))
            {
                return (target_dir.display().to_string(), filename);
            }
        }
    }

    exit_with_error!("couldn't find library file in: {}", target_dir.display())
}

fn get_version() -> String {
    match get_property("default_version") {
        Some(v) => v,
        None => exit_with_error!("couldn't determine version number"),
    }
}

fn get_pkglibdir(pg_config: &Option<String>) -> String {
    run_pg_config(pg_config, "--pkglibdir")
}

fn get_extensiondir(pg_config: &Option<String>) -> String {
    let mut dir = run_pg_config(pg_config, "--sharedir");

    dir.push_str("/extension");
    dir
}

fn get_pg_config_major_version(pg_config: &Option<String>) -> u16 {
    let version_string = run_pg_config(&pg_config, "--version");
    let version_parts = version_string.split_whitespace().collect::<Vec<&str>>();
    let version = version_parts.get(1);
    let version = f64::from_str(&version.unwrap()).expect("not a valid version number");
    version.floor() as u16
}
