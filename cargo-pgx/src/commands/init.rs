// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use colored::Colorize;
use pgx_utils::{
    exit_with_error, get_pgx_config_path, get_pgx_home, handle_result, prefix_path,
    BASE_POSTGRES_PORT_NO,
};
use rayon::prelude::*;
use rttp_client::HttpClient;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use syn::export::Formatter;

static PROCESS_ENV_DENYLIST: &'static [&'static str] = &[
    "DEBUG",
    "MAKEFLAGS",
    "MAKELEVEL",
    "MFLAGS",
    "DYLD_FALLBACK_LIBRARY_PATH",
    "OPT_LEVEL",
    "TARGET",
    "PROFILE",
    "OUT_DIR",
    "HOST",
    "NUM_JOBS",
    "LIBRARY_PATH", // see https://github.com/zombodb/pgx/issues/16
];

#[derive(Debug)]
struct PgVersion {
    major: u16,
    minor: u16,
    url: &'static str,
}

impl PgVersion {
    pub const fn new(major: u16, minor: u16, url: &'static str) -> Self {
        PgVersion { major, minor, url }
    }

    fn port(&self) -> u16 {
        BASE_POSTGRES_PORT_NO + self.major
    }

    fn label(&self) -> String {
        format!("pg{}", self.major)
    }
}

impl Display for PgVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Postgres v{}.{}", self.major, self.minor)
    }
}

const PG10_VERSION: PgVersion = PgVersion::new(
    10,
    13,
    "https://ftp.postgresql.org/pub/source/v10.13/postgresql-10.13.tar.bz2",
);
const PG11_VERSION: PgVersion = PgVersion::new(
    11,
    8,
    "https://ftp.postgresql.org/pub/source/v11.8/postgresql-11.8.tar.bz2",
);
const PG12_VERSION: PgVersion = PgVersion::new(
    12,
    3,
    "https://ftp.postgresql.org/pub/source/v12.3/postgresql-12.3.tar.bz2",
);

pub(crate) fn init_pgx(
    pg10_config: Option<&str>,
    pg11_config: Option<&str>,
    pg12_config: Option<&str>,
) -> std::result::Result<(), std::io::Error> {
    let dir = get_pgx_home();

    let input_configs = vec![
        (pg10_config, &PG10_VERSION),
        (pg11_config, &PG11_VERSION),
        (pg12_config, &PG12_VERSION),
    ];
    let output_configs = Arc::new(Mutex::new(Vec::new()));

    input_configs
        .into_par_iter()
        .for_each(|(pg_config, version)| {
            let pg_config = pg_config.map_or_else(
                || download_postgres(version, &dir),
                |v| PathBuf::from_str(v).unwrap(),
            );

            let mut mutex = output_configs.lock();
            let output_configs = mutex.as_mut().expect("failed to get output_configs lock");

            output_configs.push((pg_config, version));
        });

    let mut mutex = output_configs.lock();
    let output_configs = mutex.as_mut().unwrap();

    output_configs.sort_by(|(_, a), (_, b)| a.major.cmp(&b.major));
    for (pg_config, version) in output_configs.iter() {
        validate_pg_config(pg_config, version);
    }

    write_config(output_configs)
}

fn download_postgres(version: &PgVersion, pgxdir: &PathBuf) -> PathBuf {
    println!(
        "{} {} from {}",
        " Downloading".bold().green(),
        version,
        version.url
    );

    let result = handle_result!("", HttpClient::new().get().url(version.url).emit());
    let pgdir = untar(result.body().binary(), pgxdir, version);
    configure_postgres(version, &pgdir);
    make_postgres(version, &pgdir);
    make_install_postgres(version, &pgdir) // returns the path to pg_config
}

fn untar(bytes: &[u8], pgxdir: &PathBuf, version: &PgVersion) -> PathBuf {
    let mut pgdir = pgxdir.clone();
    pgdir.push(format!("{}.{}", version.major, version.minor));
    if pgdir.exists() {
        // delete everything at this path if it already exists
        println!("{} {}", "    Removing".bold().green(), pgdir.display());
        handle_result!(
            format!("deleting {}", pgdir.display()),
            std::fs::remove_dir_all(&pgdir)
        );
    }
    handle_result!(
        format!("creating {}", pgdir.display()),
        std::fs::create_dir_all(&pgdir)
    );

    println!(
        "{} Postgres v{}.{} to {}",
        "   Untarring".bold().green(),
        version.major,
        version.minor,
        pgdir.display()
    );
    let mut child = std::process::Command::new("tar")
        .arg("-C")
        .arg(pgdir.display().to_string())
        .arg("--strip-components=1")
        .arg("-xjf")
        .arg("-")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn `tar`");

    let stdin = child.stdin.as_mut().expect("failed to get `tar`'s stdin");
    handle_result!("writing tarball to `tar` process", stdin.write_all(bytes));
    let output = handle_result!("waiting for `tar` to finish", child.wait_with_output());

    if !output.status.success() {
        exit_with_error!(String::from_utf8(output.stderr).unwrap())
    }

    pgdir
}

fn configure_postgres(version: &PgVersion, pgdir: &PathBuf) {
    println!("{} {}", " Configuring".bold().green(), version);
    let mut command = std::process::Command::new("./configure");

    command
        .arg(format!("--prefix={}", get_pg_installdir(pgdir).display()))
        .arg(format!("--with-pgport={}", version.port()))
        .arg("--enable-debug")
        .arg("--enable-cassert")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::null())
        .env("PATH", prefix_path(pgdir))
        .current_dir(pgdir.display().to_string());
    for var in PROCESS_ENV_DENYLIST {
        command.env_remove(var);
    }

    let command_str = format!("{:?}", command);

    let child = handle_result!(format!("Failed: {:?}", command_str), command.spawn());

    let output = handle_result!(
        format!("could not receive configure's output: {}", command_str),
        child.wait_with_output()
    );

    if !output.status.success() {
        exit_with_error!(format!(
            "{}\n{}{}",
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        ))
    }
}

fn make_postgres(version: &PgVersion, pgdir: &PathBuf) {
    let num_cpus = 1.max(num_cpus::get() / 3);
    println!("{} {}", "   Compiling".bold().green(), version);
    let mut command = std::process::Command::new("make");

    command
        .arg("-j")
        .arg(num_cpus.to_string())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::null())
        .current_dir(pgdir.display().to_string());

    for var in PROCESS_ENV_DENYLIST {
        command.env_remove(var);
    }

    let command_str = format!("{:?}", command);

    let child = handle_result!(format!("Failed: {:?}", command_str), command.spawn());

    let output = handle_result!(
        format!("could not receive make's output: {}", command_str),
        child.wait_with_output()
    );

    if !output.status.success() {
        exit_with_error!(format!(
            "{}\n{}{}",
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        ))
    }
}

fn make_install_postgres(version: &PgVersion, pgdir: &PathBuf) -> PathBuf {
    println!(
        "{} {} to {}",
        "  Installing".bold().green(),
        version,
        get_pg_installdir(pgdir).display()
    );
    let mut command = std::process::Command::new("make");

    command
        .arg("install")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::null())
        .current_dir(pgdir.display().to_string());
    for var in PROCESS_ENV_DENYLIST {
        command.env_remove(var);
    }

    let command_str = format!("{:?}", command);

    let child = handle_result!(format!("Failed: {:?}", command_str), command.spawn());

    let output = handle_result!(
        format!("could not receive make's output: {}", command_str),
        child.wait_with_output()
    );

    if !output.status.success() {
        exit_with_error!(format!(
            "{}\n{}{}",
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        ))
    }

    let mut pg_config = get_pg_installdir(pgdir);
    pg_config.push("bin");
    pg_config.push("pg_config");
    pg_config
}

fn validate_pg_config(pg_config: &PathBuf, version: &PgVersion) {
    println!("{} {}", "  Validating".bold().green(), pg_config.display());
    let mut command = std::process::Command::new(pg_config);

    command
        .arg("--includedir-server")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::null());
    let command_str = format!("{:?}", command);

    let child = handle_result!(format!("Failed: {:?}", command_str), command.spawn());

    let output = handle_result!(
        format!("could not receive pg_config's output: {}", command_str),
        child.wait_with_output()
    );

    if !output.status.success() {
        exit_with_error!(
            "{}: {}\n{}{}",
            version,
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        )
    }

    let includedir = PathBuf::from_str(&String::from_utf8(output.stdout).unwrap().trim()).unwrap();
    if !includedir.exists() {
        exit_with_error!(
            "{}:  {} --includedir-server\n     `{}` does not exist",
            version,
            pg_config.display(),
            includedir.display().to_string().bold().yellow()
        );
    }
}

fn write_config(pg_configs: &Vec<(PathBuf, &PgVersion)>) -> Result<(), std::io::Error> {
    let config_path = get_pgx_config_path();
    let mut file = handle_result!(
        format!("Unable to create {}", config_path.display()),
        File::create(&config_path)
    );
    file.write_all(b"[configs]\n")?;
    for (pg_config, version) in pg_configs {
        file.write_all(format!("{}=\"{}\"\n", version.label(), pg_config.display()).as_bytes())?;
    }

    Ok(())
}

fn get_pg_installdir(pgdir: &PathBuf) -> PathBuf {
    let mut dir = PathBuf::from(pgdir);
    dir.push("pgx-install");
    dir
}
