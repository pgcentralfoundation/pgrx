// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::command::stop::stop_postgres;
use crate::{CommandExecute, SUPPORTED_MAJOR_VERSIONS};
use colored::Colorize;
use eyre::{eyre, WrapErr};
use pgx_utils::pg_config::{PgConfig, PgConfigSelector, Pgx};
use pgx_utils::prefix_path;
use rayon::prelude::*;
use rttp_client::{types::Proxy, HttpClient};

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Stdio;

use std::sync::{Arc, Mutex};

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

/// Initize pgx development environment for the first time
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Init {
    /// If installed locally, the path to PG10's `pgconfig` tool, or `downLoad` to have pgx download/compile/install it
    #[clap(env = "PG10_PG_CONFIG", long, help = "")]
    pg10: Option<String>,
    /// If installed locally, the path to PG11's `pgconfig` tool, or `downLoad` to have pgx download/compile/install it
    #[clap(env = "PG11_PG_CONFIG", long)]
    pg11: Option<String>,
    /// If installed locally, the path to PG12's `pgconfig` tool, or `downLoad` to have pgx download/compile/install it
    #[clap(env = "PG12_PG_CONFIG", long)]
    pg12: Option<String>,
    /// If installed locally, the path to PG13's `pgconfig` tool, or `downLoad` to have pgx download/compile/install it
    #[clap(env = "PG13_PG_CONFIG", long)]
    pg13: Option<String>,
    /// If installed locally, the path to PG14's `pgconfig` tool, or `downLoad` to have pgx download/compile/install it
    #[clap(env = "PG14_PG_CONFIG", long)]
    pg14: Option<String>,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Init {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let mut versions = HashMap::new();

        if let Some(version) = self.pg10 {
            versions.insert("pg10", version.clone());
        }
        if let Some(version) = self.pg11 {
            versions.insert("pg11", version.clone());
        }
        if let Some(version) = self.pg12 {
            versions.insert("pg12", version.clone());
        }
        if let Some(version) = self.pg13 {
            versions.insert("pg13", version.clone());
        }
        if let Some(version) = self.pg14 {
            versions.insert("pg14", version.clone());
        }

        if versions.is_empty() {
            // no arguments specified, so we'll just install our defaults
            init_pgx(&Pgx::default(SUPPORTED_MAJOR_VERSIONS)?)
        } else {
            // user specified arguments, so we'll only install those versions of Postgres
            let mut default_pgx = None;
            let mut pgx = Pgx::new();

            for (pgver, pg_config_path) in versions {
                let config = if pg_config_path == "download" {
                    if default_pgx.is_none() {
                        default_pgx = Some(Pgx::default(SUPPORTED_MAJOR_VERSIONS)?);
                    }
                    default_pgx
                        .as_ref()
                        .unwrap() // We just set this
                        .get(&pgver)
                        .wrap_err_with(|| format!("{} is not a known Postgres version", pgver))?
                        .clone()
                } else {
                    PgConfig::new(pg_config_path.into())
                };
                pgx.push(config);
            }

            init_pgx(&pgx)
        }
    }
}

#[tracing::instrument(skip_all, fields(pgx_home = %Pgx::home()?.display()))]
pub(crate) fn init_pgx(pgx: &Pgx) -> eyre::Result<()> {
    let dir = Pgx::home()?;

    let output_configs = Arc::new(Mutex::new(Vec::new()));

    let mut pg_configs = Vec::new();
    for pg_config in pgx.iter(PgConfigSelector::All) {
        pg_configs.push(pg_config?);
    }

    let span = tracing::Span::current();
    pg_configs
        .into_par_iter()
        .map(|pg_config| {
            let _span = span.clone().entered();
            let mut pg_config = pg_config.clone();
            stop_postgres(&pg_config).ok(); // no need to fail on errors trying to stop postgres while initializing
            if !pg_config.is_real() {
                pg_config = match download_postgres(&pg_config, &dir) {
                    Ok(pg_config) => pg_config,
                    Err(e) => return Err(eyre!(e)),
                }
            }

            let mut mutex = output_configs.lock();
            // PoisonError doesn't implement std::error::Error, can't `?` it.
            let output_configs = mutex.as_mut().expect("failed to get output_configs lock");

            output_configs.push(pg_config);
            Ok(())
        })
        .collect::<eyre::Result<()>>()?;

    let mut mutex = output_configs.lock();
    // PoisonError doesn't implement std::error::Error, can't `?` it.
    let output_configs = mutex.as_mut().unwrap();

    output_configs.sort_by(|a, b| {
        a.major_version()
            .ok()
            .expect("could not determine major version")
            .cmp(
                &b.major_version()
                    .ok()
                    .expect("could not determine major version"),
            )
    });
    for pg_config in output_configs.iter() {
        validate_pg_config(pg_config)?;

        let datadir = pg_config.data_dir()?;
        let bindir = pg_config.bin_dir()?;
        if !datadir.exists() {
            initdb(&bindir, &datadir)?;
        }
    }

    write_config(output_configs)?;
    Ok(())
}

#[tracing::instrument(level = "error", skip_all, fields(pg_version = %pg_config.version()?, pgx_home))]
fn download_postgres(pg_config: &PgConfig, pgx_home: &PathBuf) -> eyre::Result<PgConfig> {
    println!(
        "{} Postgres v{}.{} from {}",
        "  Downloading".bold().green(),
        pg_config.major_version()?,
        pg_config.minor_version()?,
        pg_config.url().expect("no url"),
    );
    let url = pg_config.url().expect("no url for pg_config").as_str();
    tracing::debug!(url = %url, "Fetching");
    let mut http_client = HttpClient::new();
    http_client.get().url(url);
    if let Some((host, port)) =
        env_proxy::for_url_str(pg_config.url().expect("no url for pg_config")).host_port()
    {
        http_client.proxy(Proxy::https(host, port as u32));
    }
    let http_response = http_client.emit()?;
    tracing::trace!(status_code = %http_response.code(), url = %url, "Fetched");
    if http_response.code() != 200 {
        return Err(eyre!(
            "Problem downloading {}:\ncode={}\n{}",
            pg_config.url().unwrap().to_string().yellow().bold(),
            http_response.code(),
            http_response.body().to_string()
        ));
    }
    let pgdir = untar(http_response.body().binary(), pgx_home, pg_config)?;
    configure_postgres(pg_config, &pgdir)?;
    make_postgres(pg_config, &pgdir)?;
    make_install_postgres(pg_config, &pgdir) // returns a new PgConfig object
}

fn untar(bytes: &[u8], pgxdir: &PathBuf, pg_config: &PgConfig) -> eyre::Result<PathBuf> {
    let mut pgdir = pgxdir.clone();
    pgdir.push(format!(
        "{}.{}",
        pg_config.major_version()?,
        pg_config.minor_version()?
    ));
    if pgdir.exists() {
        // delete everything at this path if it already exists
        println!("{} {}", "     Removing".bold().green(), pgdir.display());
        std::fs::remove_dir_all(&pgdir)?;
    }
    std::fs::create_dir_all(&pgdir)?;

    println!(
        "{} Postgres v{}.{} to {}",
        "    Untarring".bold().green(),
        pg_config.major_version()?,
        pg_config.minor_version()?,
        pgdir.display()
    );
    let mut child = std::process::Command::new("tar")
        .arg("-C")
        .arg(&pgdir)
        .arg("--strip-components=1")
        .arg("-xjf")
        .arg("-")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn `tar`");

    let stdin = child.stdin.as_mut().expect("failed to get `tar`'s stdin");
    stdin.write_all(bytes)?;
    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(pgdir)
    } else {
        Err(eyre!(
            "Command error: {}",
            String::from_utf8(output.stderr)?
        ))
    }
}

fn configure_postgres(pg_config: &PgConfig, pgdir: &PathBuf) -> eyre::Result<()> {
    println!(
        "{} Postgres v{}.{}",
        "  Configuring".bold().green(),
        pg_config.major_version()?,
        pg_config.minor_version()?
    );
    let mut configure_path = pgdir.clone();
    configure_path.push("configure");
    let mut command = std::process::Command::new(configure_path);

    command
        .arg(format!("--prefix={}", get_pg_installdir(pgdir).display()))
        .arg(format!("--with-pgport={}", pg_config.port()?))
        .arg("--enable-debug")
        .arg("--enable-cassert")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::null())
        .env("PATH", prefix_path(pgdir))
        .current_dir(&pgdir);
    for var in PROCESS_ENV_DENYLIST {
        command.env_remove(var);
    }

    let command_str = format!("{:?}", command);
    tracing::debug!(command = %command_str, "Running");
    let child = command.spawn()?;
    let output = child.wait_with_output()?;
    tracing::trace!(status_code = %output.status, command = %command_str, "Finished");

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "{}\n{}{}",
                command_str,
                String::from_utf8(output.stdout).unwrap(),
                String::from_utf8(output.stderr).unwrap()
            ),
        ))?
    }
}

fn make_postgres(pg_config: &PgConfig, pgdir: &PathBuf) -> eyre::Result<()> {
    let num_cpus = 1.max(num_cpus::get() / 3);
    println!(
        "{} Postgres v{}.{}",
        "    Compiling".bold().green(),
        pg_config.major_version()?,
        pg_config.minor_version()?
    );
    let mut command = std::process::Command::new("make");

    command
        .arg("-j")
        .arg(num_cpus.to_string())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::null())
        .current_dir(&pgdir);

    for var in PROCESS_ENV_DENYLIST {
        command.env_remove(var);
    }

    let command_str = format!("{:?}", command);
    tracing::debug!(command = %command_str, "Running");
    let child = command.spawn()?;
    let output = child.wait_with_output()?;
    tracing::trace!(status_code = %output.status, command = %command_str, "Finished");

    if output.status.success() {
        Ok(())
    } else {
        Err(eyre!(
            "{}\n{}{}",
            command_str,
            String::from_utf8(output.stdout)?,
            String::from_utf8(output.stderr)?
        ))
    }
}

fn make_install_postgres(version: &PgConfig, pgdir: &PathBuf) -> eyre::Result<PgConfig> {
    println!(
        "{} Postgres v{}.{} to {}",
        "   Installing".bold().green(),
        version.major_version()?,
        version.minor_version()?,
        get_pg_installdir(pgdir).display()
    );
    let mut command = std::process::Command::new("make");

    command
        .arg("install")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::null())
        .current_dir(&pgdir);
    for var in PROCESS_ENV_DENYLIST {
        command.env_remove(var);
    }

    let command_str = format!("{:?}", command);
    tracing::debug!(command = %command_str, "Running");
    let child = command.spawn()?;
    let output = child.wait_with_output()?;
    tracing::trace!(status_code = %output.status, command = %command_str, "Finished");

    if output.status.success() {
        let mut pg_config = get_pg_installdir(pgdir);
        pg_config.push("bin");
        pg_config.push("pg_config");
        Ok(PgConfig::new(pg_config))
    } else {
        Err(eyre!(
            "{}\n{}{}",
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        ))
    }
}

fn validate_pg_config(pg_config: &PgConfig) -> eyre::Result<()> {
    println!(
        "{} {}",
        "   Validating".bold().green(),
        pg_config.path().expect("no path for pg_config").display()
    );

    pg_config.includedir_server()?;
    pg_config.pkglibdir()?;
    Ok(())
}

fn write_config(pg_configs: &Vec<PgConfig>) -> eyre::Result<()> {
    let config_path = Pgx::config_toml()?;
    let mut file = File::create(&config_path)?;
    file.write_all(b"[configs]\n")?;
    for pg_config in pg_configs {
        file.write_all(
            format!(
                "{}=\"{}\"\n",
                pg_config.label()?,
                pg_config
                    .path()
                    .ok_or(eyre!("no path for pg_config"))?
                    .display()
            )
            .as_bytes(),
        )?;
    }

    Ok(())
}

fn get_pg_installdir(pgdir: &PathBuf) -> PathBuf {
    let mut dir = PathBuf::from(pgdir);
    dir.push("pgx-install");
    dir
}

pub(crate) fn initdb(bindir: &PathBuf, datadir: &PathBuf) -> eyre::Result<()> {
    println!(
        " {} data directory at {}",
        "Initializing".bold().green(),
        datadir.display()
    );
    let mut command = std::process::Command::new(format!("{}/initdb", bindir.display()));
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("-D")
        .arg(&datadir);

    let command_str = format!("{:?}", command);
    tracing::debug!(command = %command_str, "Running");

    let output = command.output()?;
    tracing::trace!(command = %command_str, status_code = %output.status, "Finished");

    if !output.status.success() {
        return Err(eyre!(
            "problem running initdb: {}\n{}",
            command_str,
            String::from_utf8(output.stderr).unwrap()
        ));
    }

    Ok(())
}
