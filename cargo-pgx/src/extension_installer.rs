// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::property_inspector::{find_control_file, get_property};
use colored::Colorize;
use pgx_utils::{get_pg_download_dir, run_pg_config};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::result::Result;
use std::str::FromStr;

macro_rules! exit {
    () => ({ exit!("explicit panic") });
    ($msg:expr) => ({ exit!("{}", $msg) });
    ($msg:expr,) => ({ exit!($msg) });
    ($fmt:expr, $($arg:tt)+) => ({
        eprint!("{} ", "     [error]".bold().red());
        eprintln!($fmt, $($arg)+);
        std::process::exit(1);
    });
}

pub(crate) fn install_extension(is_release: bool) -> Result<(), std::io::Error> {
    let (control_file, extname) = find_control_file()?;
    let target_dir = get_target_dir();

    if &std::env::var("PGX_NO_BUILD").unwrap_or_default() != "true" {
        eprintln!("building extension");
        build_extension(is_release, target_dir.to_str().unwrap())?;
    } else {
        eprintln!(
            "Skipping build due to $PGX_NO_BUILD=true in {}",
            std::env::current_dir().unwrap().display()
        );
    }

    eprintln!();
    eprintln!("installing extension");
    let pkgdir = get_pkglibdir();
    let extdir = get_extensiondir();
    let (libpath, libfile) =
        find_library_file(&target_dir.display().to_string(), &extname, is_release)?;

    let src = control_file.clone();
    let dest = format!("{}/{}", extdir, control_file);
    if let Err(e) = std::fs::copy(&src, &dest) {
        exit!(
            "failed copying control file ({}) to {}:  {}",
            control_file,
            extdir,
            e
        );
    }
    println!(
        "{} control file to:   {}",
        "     Copying".bold().green(),
        dest
    );

    let src = format!("{}/{}", libpath, libfile);
    let dest = format!("{}/{}.so", pkgdir, extname);
    if let Err(e) = std::fs::copy(&src, &dest) {
        exit!("failed copying library ({}) to {}:  {}", libfile, pkgdir, e);
    }
    println!(
        "{} shared library to: {}",
        "     Copying".bold().green(),
        dest
    );

    crate::generate_schema()?;
    copy_sql_files(&extdir, &extname)?;

    println!("{} installing {}", "    Finished".bold().green(), extname);
    Ok(())
}

fn build_extension(is_release: bool, target_dir: &str) -> Result<(), std::io::Error> {
    let features = std::env::var("PGX_BUILD_FEATURES").unwrap_or_default();
    let flags = std::env::var("PGX_BUILD_FLAGS").unwrap_or_default();
    let mut command = Command::new("cargo");
    command.arg("build");
    if is_release {
        command.arg("--release");
    }

    if !features.trim().is_empty() {
        command.arg("--features");
        command.arg(features);
    }

    for arg in flags.split_ascii_whitespace() {
        command.arg(arg);
    }

    let mut process = command
        .env("CARGO_TARGET_DIR", target_dir.to_string())
        .env("PG_DOWNLOAD_TARGET_DIR", get_pg_download_dir())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    let status = process.wait()?;
    if !status.success() {
        return Err(std::io::Error::from_raw_os_error(status.code().unwrap()));
    }
    Ok(())
}

fn copy_sql_files(extdir: &str, extname: &str) -> Result<(), std::io::Error> {
    let load_order = crate::schema_generator::read_load_order(
        &PathBuf::from_str("./sql/load-order.txt").unwrap(),
    );
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
    for f in std::fs::read_dir("sql/")? {
        if let Ok(f) = f {
            let filename = f.file_name().into_string().unwrap();

            if filename.starts_with(&format!("{}--", extname)) && filename.ends_with(".sql") {
                let dest = format!("{}/{}", extdir, filename);

                if let Err(e) = std::fs::copy(f.path(), &dest) {
                    exit!("failed copying SQL {} to {}:  {}", filename, dest, e)
                }
            }
        }
    }

    Ok(())
}

fn find_library_file(
    target_dir: &str,
    extname: &str,
    is_release: bool,
) -> Result<(String, String), std::io::Error> {
    let path = PathBuf::from(if is_release {
        format!("{}/release", target_dir)
    } else {
        format!("{}/debug", target_dir)
    });

    if !path.exists() {
        exit!("target directory does not exist: {}", path.display());
    }

    for f in std::fs::read_dir(&path)? {
        if let Ok(f) = f {
            let filename = f.file_name().into_string().unwrap();

            if filename.contains(extname)
                && filename.starts_with("lib")
                && (filename.ends_with(".so")
                    || filename.ends_with(".dylib")
                    || filename.ends_with(".dll"))
            {
                return Ok((path.display().to_string(), filename));
            }
        }
    }

    exit!("couldn't find library file in: {}", target_dir);
}
fn get_version() -> String {
    match get_property("default_version") {
        Some(v) => v,
        None => exit!("couldn't determine version number"),
    }
}

fn get_target_dir() -> PathBuf {
    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| {
        let mut out_dir = PathBuf::from(std::env::current_dir().unwrap());
        out_dir.push("target");
        out_dir.display().to_string()
    });

    PathBuf::from(&format!("{}/pgx-install", target_dir))
}

fn get_pkglibdir() -> String {
    run_pg_config(&None, "--pkglibdir")
}

fn get_extensiondir() -> String {
    let mut dir = run_pg_config(&None, "--sharedir");

    dir.push_str("/extension");
    dir
}
