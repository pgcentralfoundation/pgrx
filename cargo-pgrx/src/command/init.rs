/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::command::stop::stop_postgres;
use crate::command::version::pgrx_default;
use crate::CommandExecute;
use eyre::{eyre, WrapErr};
use owo_colors::OwoColorize;
use pgrx_pg_config::{
    get_c_locale_flags, prefix_path, PgConfig, PgConfigSelector, Pgrx, PgrxHomeError,
};
use rayon::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
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
    "LIBRARY_PATH", // see https://github.com/tcdi/pgrx/issues/16
];

/// Initialize pgrx development environment for the first time
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Init {
    /// If installed locally, the path to PG11's `pgconfig` tool, or `download` to have pgrx download/compile/install it
    #[clap(env = "PG11_PG_CONFIG", long)]
    pg11: Option<String>,
    /// If installed locally, the path to PG12's `pgconfig` tool, or `download` to have pgrx download/compile/install it
    #[clap(env = "PG12_PG_CONFIG", long)]
    pg12: Option<String>,
    /// If installed locally, the path to PG13's `pgconfig` tool, or `download` to have pgrx download/compile/install it
    #[clap(env = "PG13_PG_CONFIG", long)]
    pg13: Option<String>,
    /// If installed locally, the path to PG14's `pgconfig` tool, or `download` to have pgrx download/compile/install it
    #[clap(env = "PG14_PG_CONFIG", long)]
    pg14: Option<String>,
    /// If installed locally, the path to PG15's `pgconfig` tool, or `download` to have pgrx download/compile/install it
    #[clap(env = "PG15_PG_CONFIG", long)]
    pg15: Option<String>,
    /// If installed locally, the path to PG16's `pgconfig` tool, or `download` to have pgrx download/compile/install it
    #[clap(env = "PG16_PG_CONFIG", long)]
    pg16: Option<String>,
    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
    #[clap(long, help = "Base port number")]
    base_port: Option<u16>,
    #[clap(long, help = "Base testing port number")]
    base_testing_port: Option<u16>,
    #[clap(long, help = "Additional flags to pass to the configure script")]
    configure_flag: Vec<String>,
}

impl CommandExecute for Init {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let mut versions = HashMap::new();

        if let Some(ref version) = self.pg11 {
            versions.insert("pg11", version.clone());
        }
        if let Some(ref version) = self.pg12 {
            versions.insert("pg12", version.clone());
        }
        if let Some(ref version) = self.pg13 {
            versions.insert("pg13", version.clone());
        }
        if let Some(ref version) = self.pg14 {
            versions.insert("pg14", version.clone());
        }
        if let Some(ref version) = self.pg15 {
            versions.insert("pg15", version.clone());
        }
        if let Some(ref version) = self.pg16 {
            versions.insert("pg16", version.clone());
        }

        if versions.is_empty() {
            // no arguments specified, so we'll just install our defaults
            init_pgrx(&pgrx_default()?, &self)
        } else {
            // user specified arguments, so we'll only install those versions of Postgres
            let mut default_pgrx = None;
            let mut pgrx = Pgrx::default();

            for (pgver, pg_config_path) in versions {
                let config = if pg_config_path == "download" {
                    if default_pgrx.is_none() {
                        default_pgrx = Some(pgrx_default()?);
                    }
                    default_pgrx
                        .as_ref()
                        .unwrap() // We just set this
                        .get(&pgver)
                        .wrap_err_with(|| format!("{} is not a known Postgres version", pgver))?
                        .clone()
                } else {
                    PgConfig::new_with_defaults(pg_config_path.into())
                };
                pgrx.push(config);
            }

            init_pgrx(&pgrx, &self)
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn init_pgrx(pgrx: &Pgrx, init: &Init) -> eyre::Result<()> {
    let pgrx_home = match Pgrx::home() {
        Ok(path) => path,
        Err(e) => match e {
            PgrxHomeError::NoHomeDirectory => return Err(e.into()),
            PgrxHomeError::IoError(e) => return Err(e.into()),
            PgrxHomeError::MissingPgrxHome(path) => {
                // $PGRX_HOME doesn't exist, but that's okay as `cargo pgrx init` is the right time
                // to try and create it
                println!("{} PGRX_HOME at `{}`", "     Creating".bold().green(), path.display());
                std::fs::create_dir_all(&path)?;
                path
            }
        },
    };

    let output_configs = Arc::new(Mutex::new(Vec::new()));

    let mut pg_configs = Vec::new();
    for pg_config in pgrx.iter(PgConfigSelector::All) {
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
                pg_config = match download_postgres(&pg_config, &pgrx_home, init) {
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
            .unwrap_or_else(|e| panic!("{e}:  could not determine major version for: `{:?}`", a))
            .cmp(&b.major_version().ok().expect("could not determine major version"))
    });
    for pg_config in output_configs.iter() {
        validate_pg_config(pg_config)?;

        if is_root_user() {
            println!("{} initdb as current user is root user", "   Skipping".bold().green(),);
        } else {
            let datadir = pg_config.data_dir()?;
            let bindir = pg_config.bin_dir()?;
            if !datadir.exists() {
                initdb(&bindir, &datadir)?;
            }
        }
    }

    write_config(output_configs, init)?;
    Ok(())
}

#[tracing::instrument(level = "error", skip_all, fields(pg_version = %pg_config.version()?, pgrx_home))]
fn download_postgres(
    pg_config: &PgConfig,
    pgrx_home: &PathBuf,
    init: &Init,
) -> eyre::Result<PgConfig> {
    use crate::command::build_agent_for_url;

    println!(
        "{} Postgres v{} from {}",
        "  Downloading".bold().green(),
        pg_config.version()?,
        pg_config.url().expect("no url"),
    );
    let url = pg_config.url().expect("no url for pg_config").as_str();
    tracing::debug!(url = %url, "Fetching");
    let http_client = build_agent_for_url(url)?;
    let http_response = http_client.get(url).call()?;
    let status = http_response.status();
    tracing::trace!(status_code = %status, url = %url, "Fetched");
    if status != 200 {
        return Err(eyre!(
            "Problem downloading {}:\ncode={status}\n{}",
            pg_config.url().unwrap().to_string().yellow().bold(),
            http_response.into_string()?
        ));
    }
    let mut buf = Vec::new();
    let _count = http_response.into_reader().read_to_end(&mut buf)?;
    let pgdir = untar(&buf, pgrx_home, pg_config)?;
    configure_postgres(pg_config, &pgdir, init)?;
    make_postgres(pg_config, &pgdir)?;
    make_install_postgres(pg_config, &pgdir) // returns a new PgConfig object
}

fn untar(bytes: &[u8], pgrxdir: &PathBuf, pg_config: &PgConfig) -> eyre::Result<PathBuf> {
    let mut pgdir = pgrxdir.clone();
    pgdir.push(&pg_config.version()?);
    if pgdir.exists() {
        // delete everything at this path if it already exists
        println!("{} {}", "     Removing".bold().green(), pgdir.display());
        std::fs::remove_dir_all(&pgdir)?;
    }
    std::fs::create_dir_all(&pgdir)?;

    println!(
        "{} Postgres v{} to {}",
        "    Untarring".bold().green(),
        pg_config.version()?,
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
        .wrap_err("failed to spawn `tar`")?;

    let stdin = child.stdin.as_mut().expect("failed to get `tar`'s stdin");
    stdin.write_all(bytes)?;
    stdin.flush()?;
    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(pgdir)
    } else {
        Err(eyre!("Command error: {}", String::from_utf8(output.stderr)?))
    }
}

fn configure_postgres(pg_config: &PgConfig, pgdir: &PathBuf, init: &Init) -> eyre::Result<()> {
    println!("{} Postgres v{}", "  Configuring".bold().green(), pg_config.version()?);
    let mut configure_path = pgdir.clone();
    configure_path.push("configure");
    let mut command = std::process::Command::new(configure_path);

    command
        .arg(format!("--prefix={}", get_pg_installdir(pgdir).display()))
        .arg(format!("--with-pgport={}", pg_config.port()?))
        .arg("--enable-debug")
        .arg("--enable-cassert");
    for flag in init.configure_flag.iter() {
        command.arg(flag);
    }
    command
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
    println!("{} Postgres v{}", "    Compiling".bold().green(), pg_config.version()?);
    let mut command = std::process::Command::new("make");

    command
        .arg("-j")
        .arg(num_cpus.to_string())
        .arg("world-bin")
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
        "{} Postgres v{} to {}",
        "   Installing".bold().green(),
        version.version()?,
        get_pg_installdir(pgdir).display()
    );
    let mut command = std::process::Command::new("make");

    command
        .arg("install-world-bin")
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
        Ok(PgConfig::new_with_defaults(pg_config))
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

fn write_config(pg_configs: &Vec<PgConfig>, init: &Init) -> eyre::Result<()> {
    let config_path = Pgrx::config_toml()?;
    let mut file = File::create(&config_path)?;

    if let Some(port) = init.base_port {
        file.write_all(format!("base_port = {}\n", port).as_bytes())?;
    }
    if let Some(port) = init.base_testing_port {
        file.write_all(format!("base_testing_port = {}\n", port).as_bytes())?;
    }

    file.write_all(b"[configs]\n")?;
    for pg_config in pg_configs {
        file.write_all(
            format!(
                "{}=\"{}\"\n",
                pg_config.label()?,
                pg_config.path().ok_or(eyre!("no path for pg_config"))?.display()
            )
            .as_bytes(),
        )?;
    }

    Ok(())
}

fn get_pg_installdir(pgdir: &PathBuf) -> PathBuf {
    let mut dir = PathBuf::from(pgdir);
    dir.push("pgrx-install");
    dir
}

fn is_root_user() -> bool {
    use nix::unistd::Uid;
    Uid::effective().is_root()
}

pub(crate) fn initdb(bindir: &PathBuf, datadir: &PathBuf) -> eyre::Result<()> {
    println!(" {} data directory at {}", "Initializing".bold().green(), datadir.display());
    let mut command = std::process::Command::new(format!("{}/initdb", bindir.display()));
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(get_c_locale_flags())
        .arg("-D")
        .arg(&datadir);

    let command_str = format!("{:?}", command);
    tracing::debug!(command = %command_str, "Running");

    let output = command.output().wrap_err_with(|| eyre!("unable to execute: {}", command_str))?;
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
