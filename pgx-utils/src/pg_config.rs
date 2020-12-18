//! Wrapper around Postgres' `pg_config` command-line tool
use colored::Colorize;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use url::Url;

#[derive(Clone)]
pub struct PgVersion {
    major_version: u16,
    minor_version: u16,
    url: Url,
}

impl Display for PgVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major_version, self.minor_version)
    }
}

#[derive(Clone)]
pub struct PgConfig {
    version: Option<PgVersion>,
    pg_config: Option<PathBuf>,
}

impl Display for PgConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let major = self
            .major_version()
            .expect("could not determine major version");
        let minor = self
            .minor_version()
            .expect("could not determine minor version");
        let path = match self.pg_config.as_ref() {
            Some(path) => path.display().to_string(),
            None => self.version.as_ref().unwrap().url.to_string(),
        };
        write!(f, "{}.{}={}", major, minor, path)
    }
}

impl Default for PgConfig {
    fn default() -> Self {
        PgConfig {
            version: None,
            pg_config: None,
        }
    }
}

impl PgConfig {
    pub fn new(pg_config: PathBuf) -> Self {
        PgConfig {
            version: None,
            pg_config: Some(pg_config),
        }
    }

    pub fn from_path() -> Self {
        PgConfig::new("pg_config".into())
    }

    pub fn is_real(&self) -> bool {
        self.pg_config.is_some()
    }

    pub fn label(&self) -> Result<String, std::io::Error> {
        Ok(format!("pg{}", self.major_version()?))
    }

    pub fn path(&self) -> Option<PathBuf> {
        self.pg_config.clone()
    }

    pub fn parent_path(&self) -> PathBuf {
        self.path().unwrap().parent().unwrap().to_path_buf()
    }

    pub fn major_version(&self) -> Result<u16, std::io::Error> {
        match &self.version {
            Some(version) => Ok(version.major_version),
            None => {
                let version_string = self.run("--version")?;
                let version_parts = version_string.split_whitespace().collect::<Vec<&str>>();
                let version = match version_parts.get(1) {
                    Some(v) => v,
                    None => {
                        return Err(std::io::Error::new(
                            ErrorKind::InvalidInput,
                            format!("invalid version string: {}", version_string),
                        ));
                    }
                };
                let version = match f64::from_str(version) {
                    Ok(f) => f,
                    Err(e) => {
                        return Err(std::io::Error::new(
                            ErrorKind::InvalidInput,
                            format!("invalid major version number `{}`: {:?}", version, e),
                        ));
                    }
                };
                Ok(version.floor() as u16)
            }
        }
    }

    pub fn minor_version(&self) -> Result<u16, std::io::Error> {
        match &self.version {
            Some(version) => Ok(version.minor_version),
            None => {
                let version_string = self.run("--version")?;
                let version_parts = version_string.split_whitespace().collect::<Vec<&str>>();
                let version = match version_parts.get(1) {
                    Some(v) => v.split('.').next().unwrap(),
                    None => {
                        return Err(std::io::Error::new(
                            ErrorKind::InvalidInput,
                            format!("invalid version string: {}", version_string),
                        ));
                    }
                };
                let version = match u16::from_str(version) {
                    Ok(u) => u,
                    Err(e) => {
                        return Err(std::io::Error::new(
                            ErrorKind::InvalidInput,
                            format!("invalid minor version number `{}`: {:?}", version, e),
                        ));
                    }
                };
                Ok(version)
            }
        }
    }

    pub fn url(&self) -> Option<&Url> {
        match &self.version {
            Some(version) => Some(&version.url),
            None => None,
        }
    }

    pub fn port(&self) -> Result<u16, std::io::Error> {
        Ok(BASE_POSTGRES_PORT_NO + self.major_version()?)
    }

    pub fn test_port(&self) -> Result<u16, std::io::Error> {
        Ok(BASE_POSTGRES_TESTING_PORT_NO + self.major_version()?)
    }

    pub fn host(&self) -> &'static str {
        "localhost"
    }

    pub fn bin_dir(&self) -> Result<PathBuf, std::io::Error> {
        Ok(Path::new(&self.run("--bindir")?).to_path_buf())
    }

    pub fn postmaster_path(&self) -> Result<PathBuf, std::io::Error> {
        let mut path = self.bin_dir()?;
        path.push("postmaster");
        Ok(path)
    }

    pub fn initdb_path(&self) -> Result<PathBuf, std::io::Error> {
        let mut path = self.bin_dir()?;
        path.push("initdb");
        Ok(path)
    }

    pub fn createdb_path(&self) -> Result<PathBuf, std::io::Error> {
        let mut path = self.bin_dir()?;
        path.push("createdb");
        Ok(path)
    }

    pub fn dropdb_path(&self) -> Result<PathBuf, std::io::Error> {
        let mut path = self.bin_dir()?;
        path.push("dropdb");
        Ok(path)
    }

    pub fn psql_path(&self) -> Result<PathBuf, std::io::Error> {
        let mut path = self.bin_dir()?;
        path.push("psql");
        Ok(path)
    }

    pub fn data_dir(&self) -> Result<PathBuf, std::io::Error> {
        let mut path = Pgx::home()?;
        path.push(format!("data-{}", self.major_version()?));
        Ok(path)
    }

    pub fn log_file(&self) -> Result<PathBuf, std::io::Error> {
        let mut path = Pgx::home()?;
        path.push(format!("{}.log", self.major_version()?));
        Ok(path)
    }

    pub fn includedir_server(&self) -> Result<PathBuf, std::io::Error> {
        Ok(self.run("--includedir-server")?.into())
    }

    pub fn pkglibdir(&self) -> Result<PathBuf, std::io::Error> {
        Ok(self.run("--pkglibdir")?.into())
    }

    pub fn sharedir(&self) -> Result<PathBuf, std::io::Error> {
        Ok(self.run("--sharedir")?.into())
    }

    pub fn extension_dir(&self) -> Result<PathBuf, std::io::Error> {
        let mut path = self.sharedir()?;
        path.push("extension");
        Ok(path)
    }

    fn run(&self, arg: &str) -> Result<String, std::io::Error> {
        let pg_config = self.pg_config.clone().unwrap_or_else(|| {
            std::env::var("PG_CONFIG")
                .unwrap_or_else(|_| "pg_config".to_string())
                .into()
        });

        match Command::new(&pg_config).arg(arg).output() {
            Ok(output) => Ok(String::from_utf8(output.stdout).unwrap().trim().to_string()),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Err(std::io::Error::new(
                    ErrorKind::NotFound,
                    format!("Unable to find `{}`: {:?}", "pg_config".yellow(), e),
                )),
                _ => Err(e),
            },
        }
    }
}

pub struct Pgx {
    pg_configs: Vec<PgConfig>,
}

use crate::{BASE_POSTGRES_PORT_NO, BASE_POSTGRES_TESTING_PORT_NO};
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;
use syn::export::Formatter;

#[derive(Debug, Serialize, Deserialize)]
struct ConfigToml {
    configs: HashMap<String, PathBuf>,
}

pub enum PgConfigSelector<'a> {
    All,
    Specific(&'a str),
}

impl<'a> PgConfigSelector<'a> {
    pub fn new(label: &'a str) -> Self {
        if label == "all" {
            PgConfigSelector::All
        } else {
            PgConfigSelector::Specific(label)
        }
    }
}

impl Pgx {
    pub fn new() -> Self {
        Pgx { pg_configs: vec![] }
    }

    pub fn default() -> Result<Self, std::io::Error> {
        Ok(Pgx {
            pg_configs: vec![
                PgConfig {
                    version: Some(PgVersion {
                        major_version: 10,
                        minor_version: 14,
                        url: Url::parse(
                            "https://ftp.postgresql.org/pub/source/v10.14/postgresql-10.14.tar.bz2",
                        )
                        .expect("invalid url"),
                    }),
                    pg_config: None,
                },
                PgConfig {
                    version: Some(PgVersion {
                        major_version: 11,
                        minor_version: 9,
                        url: Url::parse(
                            "https://ftp.postgresql.org/pub/source/v11.9/postgresql-11.9.tar.bz2",
                        )
                        .expect("invalid url"),
                    }),
                    pg_config: None,
                },
                PgConfig {
                    version: Some(PgVersion {
                        major_version: 12,
                        minor_version: 4,
                        url: Url::parse(
                            "https://ftp.postgresql.org/pub/source/v12.4/postgresql-12.4.tar.bz2",
                        )
                        .expect("invalid url"),
                    }),
                    pg_config: None,
                },
                PgConfig {
                    version: Some(PgVersion {
                        major_version: 13,
                        minor_version: 0,
                        url: Url::parse(
                            "https://ftp.postgresql.org/pub/source/v13.0/postgresql-13.0.tar.bz2",
                        )
                        .expect("invalid url"),
                    }),
                    pg_config: None,
                },
            ],
        })
    }

    pub fn from_config() -> Result<Self, std::io::Error> {
        match std::env::var("PGX_PG_CONFIG_PATH") {
            Ok(pg_config) => {
                // we have an environment variable that tells us the pg_config to use
                let mut pgx = Pgx::new();
                pgx.push(PgConfig::new(pg_config.into()));
                Ok(pgx)
            }
            Err(_) => {
                // we'll get what we need from cargo-pgx' config.toml file
                let path = Pgx::config_toml()?;
                if !path.exists() {
                    return Err(std::io::Error::new(
                        ErrorKind::NotFound,
                        format!(
                            "{} not found.  Have you run `{}` yet?",
                            path.display(),
                            "cargo pgx init".bold().yellow()
                        ),
                    ));
                }

                match toml::from_str::<ConfigToml>(&std::fs::read_to_string(path)?) {
                    Ok(configs) => {
                        let mut pgx = Pgx::new();

                        for (_, v) in configs.configs {
                            pgx.push(PgConfig::new(v));
                        }
                        Ok(pgx)
                    }
                    Err(e) => Err(std::io::Error::new(ErrorKind::InvalidInput, e)),
                }
            }
        }
    }

    pub fn push(&mut self, pg_config: PgConfig) {
        self.pg_configs.push(pg_config);
    }

    pub fn iter(
        &self,
        which: PgConfigSelector,
    ) -> impl std::iter::Iterator<Item = Result<&PgConfig, std::io::Error>> {
        match which {
            PgConfigSelector::All => {
                let mut configs = self.pg_configs.iter().collect::<Vec<_>>();
                configs.sort_by(|a, b| {
                    a.major_version()
                        .expect("no major version")
                        .cmp(&b.major_version().expect("no major version"))
                });

                configs
                    .into_iter()
                    .map(|c| Ok(c))
                    .collect::<Vec<_>>()
                    .into_iter()
            }
            PgConfigSelector::Specific(label) => vec![self.get(label)].into_iter(),
        }
    }

    pub fn get(&self, label: &str) -> Result<&PgConfig, std::io::Error> {
        for pg_config in self.pg_configs.iter() {
            if pg_config.label()? == label {
                return Ok(pg_config);
            }
        }
        Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            format!("Postgres '{}' is not managed by pgx", label),
        ))
    }

    pub fn home() -> Result<PathBuf, std::io::Error> {
        std::env::var("PGX_HOME").map_or_else(
            |_| {
                let mut dir = match dirs::home_dir() {
                    Some(dir) => dir,
                    None => {
                        return Err(std::io::Error::new(
                            ErrorKind::NotFound,
                            "You don't seem to have a home directory",
                        ));
                    }
                };
                dir.push(".pgx");
                if !dir.exists() {
                    if let Err(e) = std::fs::create_dir_all(&dir) {
                        return Err(std::io::Error::new(
                            ErrorKind::InvalidInput,
                            format!("could not create PGX_HOME at `{}`: {:?}", dir.display(), e),
                        ));
                    }
                }

                Ok(dir)
            },
            |v| Ok(v.into()),
        )
    }

    pub fn config_toml() -> Result<PathBuf, std::io::Error> {
        let mut path = Pgx::home()?;
        path.push("config.toml");
        Ok(path)
    }
}
