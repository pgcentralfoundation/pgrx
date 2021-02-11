// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::commands::get::{find_control_file, get_property};
use crate::commands::schema::read_load_order;
use colored::Colorize;
use pgx_utils::pg_config::PgConfig;
use pgx_utils::{exit_with_error, get_target_dir, handle_result};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;

pub(crate) fn install_extension(
    pg_config: &PgConfig,
    is_release: bool,
    base_directory: Option<PathBuf>,
) -> Result<(), std::io::Error> {
    let base_directory = base_directory.unwrap_or("/".into());
    let (control_file, extname) = find_control_file();
    let major_version = pg_config.major_version()?;

    if get_property("relocatable") != Some("false".into()) {
        exit_with_error!(
            "{}:  The `relocatable` property MUST be `false`.  Please update your .control file.",
            control_file.display()
        )
    }

    build_extension(major_version, is_release);

    println!();
    println!("installing extension");
    let pkgdir = make_relative(pg_config.pkglibdir()?);
    let extdir = make_relative(pg_config.extension_dir()?);
    let shlibpath = find_library_file(&extname, is_release);

    {
        let mut dest = base_directory.clone();
        dest.push(&extdir);
        dest.push(&control_file);
        copy_file(control_file, dest, "control file");
    }

    {
        let mut dest = base_directory.clone();
        dest.push(&pkgdir);
        dest.push(format!("{}.so", extname));
        copy_file(shlibpath, dest, "shared library");
    }

    {
        handle_result!(crate::generate_schema(), "failed to generate SQL schema");
    }

    copy_sql_files(&extdir, &extname, &base_directory);

    println!("{} installing {}", "    Finished".bold().green(), extname);
    Ok(())
}

fn copy_file(src: PathBuf, dest: PathBuf, msg: &str) {
    if !dest.parent().unwrap().exists() {
        handle_result!(
            std::fs::create_dir_all(dest.parent().unwrap()),
            format!(
                "failed to create destination directory {}",
                dest.parent().unwrap().display()
            )
        );
    }

    println!(
        "{} {} to `{}`",
        "     Copying".bold().green(),
        msg,
        format_display_path(&dest)
    );

    handle_result!(
        std::fs::copy(&src, &dest),
        format!("failed copying `{}` to `{}`", src.display(), dest.display())
    );
}

fn build_extension(major_version: u16, is_release: bool) {
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

    for arg in flags.split_ascii_whitespace() {
        command.arg(arg);
    }

    let command = command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let command_str = format!("{:?}", command);
    println!(
        "building extension with features `{}`\n{}",
        features, command_str
    );
    let status = handle_result!(
        command.status(),
        format!("failed to spawn cargo: {}", command_str)
    );
    if !status.success() {
        exit_with_error!("failed to build extension");
    }
}

pub(crate) fn write_full_schema_file(dir: &PathBuf, extdir: Option<&PathBuf>) {
    let (_, extname) = find_control_file();
    let load_order = read_load_order(&PathBuf::from_str("./sql/load-order.txt").unwrap());
    let mut target_filename = dir.clone();
    if extdir.is_some() {
        target_filename.push(extdir.unwrap());
    }
    target_filename.push(format!("{}--{}.sql", extname, get_version()));

    let mut sql = std::fs::File::create(&target_filename).unwrap();
    println!(
        "{} extension schema to `{}`",
        "     Writing".bold().green(),
        format_display_path(&target_filename)
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

        let contents = filter_contents(contents);

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
}

fn copy_sql_files(extdir: &PathBuf, extname: &str, base_directory: &PathBuf) {
    write_full_schema_file(&base_directory, Some(extdir));

    // now copy all the version upgrade files too
    for sql in handle_result!(std::fs::read_dir("sql/"), "failed to read ./sql/ directory") {
        if let Ok(sql) = sql {
            let filename = sql.file_name().into_string().unwrap();

            if filename.starts_with(&format!("{}--", extname)) && filename.ends_with(".sql") {
                let mut dest = base_directory.clone();
                dest.push(extdir);
                dest.push(filename);

                copy_file(sql.path(), dest, "extension schema file");
            }
        }
    }
}

fn find_library_file(extname: &str, is_release: bool) -> PathBuf {
    let mut target_dir = get_target_dir();
    target_dir.push(if is_release { "release" } else { "debug" });

    if !target_dir.exists() {
        exit_with_error!("target directory does not exist: {}", target_dir.display());
    }

    for f in handle_result!(
        std::fs::read_dir(&target_dir),
        format!("Unable to read {}", target_dir.display())
    ) {
        if let Ok(f) = f {
            let filename = f.file_name().into_string().unwrap();

            if filename.contains(extname)
                && filename.starts_with("lib")
                && (filename.ends_with(".so")
                    || filename.ends_with(".dylib")
                    || filename.ends_with(".dll"))
            {
                return f.path();
            }
        }
    }

    if extname.contains('-') {
        exit_with_error!("library file not found in: `{}`.  It looks like your extension/crate name contains a dash (`-`).  The allowed set of characters is `{}`. Try renaming things, including your `{}.control` file", target_dir.display(), "[a-z0-9_]".green(), extname)
    } else {
        exit_with_error!("library file not found in: `{}`", target_dir.display())
    }
}

fn get_version() -> String {
    match get_property("default_version") {
        Some(v) => v,
        None => exit_with_error!("cannot determine extension version number.  Is the `default_version` property declared in the control file?"),
    }
}

fn get_git_hash() -> String {
    match get_property("git_hash") {
        Some(hash) => hash,
        None => exit_with_error!(
            "unable to determine git hash.  Is git installed and is this project a git repository?"
        ),
    }
}

fn make_relative(path: PathBuf) -> PathBuf {
    if path.is_relative() {
        return path;
    }
    let mut relative = PathBuf::new();
    let mut components = path.components();
    components.next(); // skip the root
    while let Some(part) = components.next() {
        relative.push(part)
    }
    relative
}

fn format_display_path(path: &PathBuf) -> String {
    path.strip_prefix(get_target_dir().parent().unwrap())
        .unwrap_or(&path)
        .display()
        .to_string()
}

fn filter_contents(mut input: String) -> String {
    if input.contains("@GIT_HASH@") {
        // avoid doing this if we don't actually have the token
        // the project might not be a git repo so running `git`
        // would fail
        input = input.replace("@GIT_HASH@", &get_git_hash());
    }

    input = input.replace("@DEFAULT_VERSION@", &get_version());

    input
}
