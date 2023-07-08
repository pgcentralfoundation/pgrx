/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
//! Wrapper around Postgres' `pg_config` command-line tool
use eyre::{eyre, WrapErr};
use owo_colors::OwoColorize;
use serde_derive::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::ffi::OsString;
use std::fmt::{self, Debug, Display, Formatter};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use url::Url;

pub mod cargo;

pub static BASE_POSTGRES_PORT_NO: u16 = 28800;
pub static BASE_POSTGRES_TESTING_PORT_NO: u16 = 32200;

/// The flags to specify to get a "C.UTF-8" locale on this system, or "C" locale on systems without
/// a "C.UTF-8" locale equivalent.
pub fn get_c_locale_flags() -> &'static [&'static str] {
    #[cfg(target_os = "macos")]
    {
        &["--locale=C", "--lc-ctype=UTF-8"]
    }
    #[cfg(not(target_os = "macos"))]
    {
        match Command::new("locale").arg("-a").output() {
            Ok(cmd)
                if String::from_utf8_lossy(&cmd.stdout)
                    .lines()
                    .any(|l| l == "C.UTF-8" || l == "C.utf8") =>
            {
                &["--locale=C.UTF-8"]
            }
            // fallback to C if we can't list locales or don't have C.UTF-8
            _ => &["--locale=C"],
        }
    }
}

// These methods were originally in `pgrx-utils`, but in an effort to consolidate
// dependencies, the decision was made to package them into wherever made the
// most sense. In this case, it made the most sense to put them into this
// pgrx-pg-config crate. That doesn't mean they can't be moved at a later date.
mod path_methods;
pub use path_methods::{get_target_dir, prefix_path};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PgMinorVersion {
    Latest,
    Release(u16),
    Beta(u16),
    Rc(u16),
}

impl Display for PgMinorVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PgMinorVersion::Latest => write!(f, ".LATEST"),
            PgMinorVersion::Release(v) => write!(f, ".{}", v),
            PgMinorVersion::Beta(v) => write!(f, "beta{}", v),
            PgMinorVersion::Rc(v) => write!(f, "rc{}", v),
        }
    }
}

impl PgMinorVersion {
    fn version(&self) -> Option<u16> {
        match self {
            PgMinorVersion::Latest => None,
            PgMinorVersion::Release(v) | PgMinorVersion::Beta(v) | PgMinorVersion::Rc(v) => {
                Some(*v)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct PgVersion {
    pub major: u16,
    pub minor: PgMinorVersion,
    pub url: Option<Url>,
}

impl PgVersion {
    pub const fn new(major: u16, minor: PgMinorVersion, url: Option<Url>) -> PgVersion {
        PgVersion { major, minor, url }
    }

    pub fn minor(&self) -> Option<u16> {
        self.minor.version()
    }
}

impl Display for PgVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.major, self.minor)
    }
}

#[derive(Clone, Debug)]
pub struct PgConfig {
    version: Option<PgVersion>,
    pg_config: Option<PathBuf>,
    known_props: Option<BTreeMap<String, String>>,
    base_port: u16,
    base_testing_port: u16,
}

impl Display for PgConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version().expect("failed to create version string"))
    }
}

impl Default for PgConfig {
    fn default() -> Self {
        PgConfig {
            version: None,
            pg_config: None,
            known_props: None,
            base_port: BASE_POSTGRES_PORT_NO,
            base_testing_port: BASE_POSTGRES_TESTING_PORT_NO,
        }
    }
}

impl From<PgVersion> for PgConfig {
    fn from(version: PgVersion) -> Self {
        PgConfig { version: Some(version), pg_config: None, ..Default::default() }
    }
}

impl PgConfig {
    pub fn new(pg_config: PathBuf, base_port: u16, base_testing_port: u16) -> Self {
        PgConfig {
            version: None,
            pg_config: Some(pg_config),
            known_props: None,
            base_port,
            base_testing_port,
        }
    }

    pub fn new_with_defaults(pg_config: PathBuf) -> Self {
        PgConfig {
            version: None,
            pg_config: Some(pg_config),
            known_props: None,
            base_port: BASE_POSTGRES_PORT_NO,
            base_testing_port: BASE_POSTGRES_TESTING_PORT_NO,
        }
    }

    pub fn from_path() -> Self {
        let path =
            pathsearch::find_executable_in_path("pg_config").unwrap_or_else(|| "pg_config".into());
        Self::new_with_defaults(path)
    }

    /// Construct a new [`PgConfig`] from the set of environment variables that are prefixed with
    /// `PGRX_PG_CONFIG_`.
    ///
    /// It also requires that the `PGRX_PG_CONFIG_AS_ENV` variable be set to some value that isn't
    /// the string `"false"`.
    pub fn from_env() -> eyre::Result<Self> {
        if !Self::is_in_environment() {
            Err(eyre::eyre!("`PgConfig` not described in the environment"))
        } else {
            const PREFIX: &str = "PGRX_PG_CONFIG_";

            let mut known_props = BTreeMap::new();
            for (k, v) in std::env::vars() {
                if k.starts_with(PREFIX) {
                    // reformat the key to look like an argument option to `pg_config`
                    let prop = format!("--{}", k.trim_start_matches(PREFIX).to_lowercase());
                    known_props.insert(prop, v);
                }
            }

            Ok(Self {
                version: None,
                pg_config: None,
                known_props: Some(known_props),
                base_port: 0,
                base_testing_port: 0,
            })
        }
    }

    pub fn is_in_environment() -> bool {
        match std::env::var("PGRX_PG_CONFIG_AS_ENV") {
            Ok(value) => value == "true",
            _ => false,
        }
    }

    pub fn is_real(&self) -> bool {
        self.pg_config.is_some()
    }

    pub fn label(&self) -> eyre::Result<String> {
        Ok(format!("pg{}", self.major_version()?))
    }

    pub fn path(&self) -> Option<PathBuf> {
        self.pg_config.clone()
    }

    pub fn parent_path(&self) -> PathBuf {
        self.path().unwrap().parent().unwrap().to_path_buf()
    }

    fn parse_version_str(version_str: &str) -> eyre::Result<(u16, PgMinorVersion)> {
        let version_parts = version_str.split_whitespace().collect::<Vec<&str>>();
        let mut version = version_parts
            .get(1)
            .ok_or_else(|| eyre!("invalid version string: {}", version_str))?
            .split('.')
            .collect::<Vec<&str>>();

        let mut beta = false;
        let mut rc = false;

        if version.len() == 1 {
            // it's hopefully a "beta" or "rc" release
            let first = &version[0];

            if first.contains("beta") {
                beta = true;
                version = first.split("beta").collect();
            } else if first.contains("rc") {
                rc = true;
                version = first.split("rc").collect();
            } else {
                return Err(eyre!("invalid version string: {}", version_str));
            }
        }

        let major = u16::from_str(version[0])
            .map_err(|e| eyre!("invalid major version number `{}`: {:?}", version[0], e))?;
        let mut minor = version[1];
        let mut end_index = minor.len();
        for (i, c) in minor.chars().enumerate() {
            if !c.is_ascii_digit() {
                end_index = i;
                break;
            }
        }
        minor = &minor[0..end_index];
        let minor = u16::from_str(minor)
            .map_err(|e| eyre!("invalid minor version number `{}`: {:?}", minor, e))?;
        let minor = if beta {
            PgMinorVersion::Beta(minor)
        } else if rc {
            PgMinorVersion::Rc(minor)
        } else {
            PgMinorVersion::Release(minor)
        };
        return Ok((major, minor));
    }

    fn get_version(&self) -> eyre::Result<(u16, PgMinorVersion)> {
        let version_string = self.run("--version")?;
        Self::parse_version_str(&version_string)
    }

    pub fn major_version(&self) -> eyre::Result<u16> {
        match &self.version {
            Some(version) => Ok(version.major),
            None => Ok(self.get_version()?.0),
        }
    }

    fn minor_version(&self) -> eyre::Result<PgMinorVersion> {
        match &self.version {
            Some(version) => Ok(version.minor),
            None => Ok(self.get_version()?.1),
        }
    }

    pub fn version(&self) -> eyre::Result<String> {
        match self.version.as_ref() {
            Some(pgver) => Ok(pgver.to_string()),
            None => {
                let major = self.major_version()?;
                let minor = self.minor_version()?;
                let version = format!("{}{}", major, minor);
                Ok(version)
            }
        }
    }

    pub fn url(&self) -> Option<&Url> {
        match &self.version {
            Some(version) => version.url.as_ref(),
            None => None,
        }
    }

    pub fn port(&self) -> eyre::Result<u16> {
        Ok(self.base_port + self.major_version()?)
    }

    pub fn test_port(&self) -> eyre::Result<u16> {
        Ok(self.base_testing_port + self.major_version()?)
    }

    pub fn host(&self) -> &'static str {
        "localhost"
    }

    pub fn bin_dir(&self) -> eyre::Result<PathBuf> {
        Ok(Path::new(&self.run("--bindir")?).to_path_buf())
    }

    pub fn postmaster_path(&self) -> eyre::Result<PathBuf> {
        let mut path = self.bin_dir()?;
        path.push("postgres");

        Ok(path)
    }

    pub fn initdb_path(&self) -> eyre::Result<PathBuf> {
        let mut path = self.bin_dir()?;
        path.push("initdb");
        Ok(path)
    }

    pub fn createdb_path(&self) -> eyre::Result<PathBuf> {
        let mut path = self.bin_dir()?;
        path.push("createdb");
        Ok(path)
    }

    pub fn dropdb_path(&self) -> eyre::Result<PathBuf> {
        let mut path = self.bin_dir()?;
        path.push("dropdb");
        Ok(path)
    }

    pub fn psql_path(&self) -> eyre::Result<PathBuf> {
        let mut path = self.bin_dir()?;
        path.push("psql");
        Ok(path)
    }

    pub fn data_dir(&self) -> eyre::Result<PathBuf> {
        let mut path = Pgrx::home()?;
        path.push(format!("data-{}", self.major_version()?));
        Ok(path)
    }

    pub fn log_file(&self) -> eyre::Result<PathBuf> {
        let mut path = Pgrx::home()?;
        path.push(format!("{}.log", self.major_version()?));
        Ok(path)
    }

    pub fn includedir_server(&self) -> eyre::Result<PathBuf> {
        Ok(self.run("--includedir-server")?.into())
    }

    pub fn pkglibdir(&self) -> eyre::Result<PathBuf> {
        Ok(self.run("--pkglibdir")?.into())
    }

    pub fn sharedir(&self) -> eyre::Result<PathBuf> {
        Ok(self.run("--sharedir")?.into())
    }

    pub fn cppflags(&self) -> eyre::Result<OsString> {
        Ok(self.run("--cppflags")?.into())
    }

    pub fn extension_dir(&self) -> eyre::Result<PathBuf> {
        let mut path = self.sharedir()?;
        path.push("extension");
        Ok(path)
    }

    fn run(&self, arg: &str) -> eyre::Result<String> {
        if self.known_props.is_some() {
            // we have some known properties, so use them.  We'll return an `ErrorKind::InvalidData`
            // if the caller asks for a property we don't have
            Ok(self
                .known_props
                .as_ref()
                .unwrap()
                .get(arg)
                .ok_or_else(|| {
                    std::io::Error::new(
                        ErrorKind::InvalidData,
                        format!("`PgConfig` has no known property named {arg}"),
                    )
                })
                .cloned()?)
        } else {
            // we don't have any known properties, so fall through to asking the `pg_config`
            // that's either in the environment or on the PATH
            let pg_config = self.pg_config.clone().unwrap_or_else(|| {
                std::env::var("PG_CONFIG").unwrap_or_else(|_| "pg_config".to_string()).into()
            });

            match Command::new(&pg_config).arg(arg).output() {
                Ok(output) => Ok(String::from_utf8(output.stdout).unwrap().trim().to_string()),
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => Err(e).wrap_err_with(|| {
                        format!("Unable to find `{}` on the system $PATH", "pg_config".yellow())
                    }),
                    _ => Err(e.into()),
                },
            }
        }
    }
}

#[derive(Debug)]
pub struct Pgrx {
    pg_configs: Vec<PgConfig>,
    base_port: u16,
    base_testing_port: u16,
}

impl Default for Pgrx {
    fn default() -> Self {
        Self {
            pg_configs: vec![],
            base_port: BASE_POSTGRES_PORT_NO,
            base_testing_port: BASE_POSTGRES_TESTING_PORT_NO,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigToml {
    configs: HashMap<String, PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_testing_port: Option<u16>,
}

pub enum PgConfigSelector<'a> {
    All,
    Specific(&'a str),
    Environment,
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

#[derive(Debug)]
pub enum PgrxHomeError {
    NoHomeDirectory,
    MissingPgrxHome(PathBuf),
    IoError(std::io::Error),
}

impl From<std::io::Error> for PgrxHomeError {
    fn from(value: std::io::Error) -> Self {
        PgrxHomeError::IoError(value)
    }
}

impl From<PgrxHomeError> for std::io::Error {
    fn from(value: PgrxHomeError) -> Self {
        match value {
            PgrxHomeError::NoHomeDirectory => {
                std::io::Error::new(ErrorKind::NotFound, value.to_string())
            }
            PgrxHomeError::MissingPgrxHome(_) => {
                std::io::Error::new(ErrorKind::NotFound, value.to_string())
            }
            PgrxHomeError::IoError(e) => e,
        }
    }
}

impl Display for PgrxHomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PgrxHomeError::NoHomeDirectory => write!(f, "You don't seem to have a home directory"),
            PgrxHomeError::MissingPgrxHome(_) => write!(f, "$PGRX_HOME does not exist"),
            PgrxHomeError::IoError(e) => write!(f, "{e}"),
        }
    }
}

impl Error for PgrxHomeError {}

impl Pgrx {
    pub fn new(base_port: u16, base_testing_port: u16) -> Self {
        Pgrx { pg_configs: vec![], base_port, base_testing_port }
    }

    pub fn from_config() -> eyre::Result<Self> {
        match std::env::var("PGRX_PG_CONFIG_PATH") {
            Ok(pg_config) => {
                // we have an environment variable that tells us the pg_config to use
                let mut pgrx = Pgrx::default();
                pgrx.push(PgConfig::new(pg_config.into(), pgrx.base_port, pgrx.base_testing_port));
                Ok(pgrx)
            }
            Err(_) => {
                // we'll get what we need from cargo-pgrx' config.toml file
                let path = Pgrx::config_toml()?;
                if !path.exists() {
                    return Err(eyre!(
                        "{} not found.  Have you run `{}` yet?",
                        path.display(),
                        "cargo pgrx init".bold().yellow()
                    ));
                }

                match toml::from_str::<ConfigToml>(&std::fs::read_to_string(&path)?) {
                    Ok(configs) => {
                        let mut pgrx = Pgrx::new(
                            configs.base_port.unwrap_or(BASE_POSTGRES_PORT_NO),
                            configs.base_testing_port.unwrap_or(BASE_POSTGRES_TESTING_PORT_NO),
                        );

                        for (_, v) in configs.configs {
                            pgrx.push(PgConfig::new(v, pgrx.base_port, pgrx.base_testing_port));
                        }
                        Ok(pgrx)
                    }
                    Err(e) => {
                        Err(e).wrap_err_with(|| format!("Could not read `{}`", path.display()))
                    }
                }
            }
        }
    }

    pub fn push(&mut self, pg_config: PgConfig) {
        self.pg_configs.push(pg_config);
    }

    /// Returns an iterator of all "configured" `PgConfig`s we know about.
    ///
    /// If the `which` argument is [`PgConfigSelector::All`] **and** the environment variable
    /// `PGRX_PG_CONFIG_AS_ENV` is set to a value that isn't `"false"`then this function will return
    /// a one-element iterator that represents that single "pg_config".
    ///
    /// Otherwise, we'll follow the rules of [`PgConfigSelector::All`] being everything in `$PGRX_HOME/config.toml`,
    /// [`PgConfigSelector::Specific`] being that specific version from `$PGRX_HOME/config.toml`, and
    /// [`PgConfigSelector::Environment`] being the one described in the environment.
    pub fn iter(
        &self,
        which: PgConfigSelector,
    ) -> impl std::iter::Iterator<Item = eyre::Result<PgConfig>> {
        match (which, PgConfig::is_in_environment()) {
            (PgConfigSelector::All, true) | (PgConfigSelector::Environment, _) => {
                vec![PgConfig::from_env()].into_iter()
            }

            (PgConfigSelector::All, _) => {
                let mut configs = self.pg_configs.iter().collect::<Vec<_>>();
                configs.sort_by(|a, b| {
                    a.major_version()
                        .expect("no major version")
                        .cmp(&b.major_version().expect("no major version"))
                });

                configs.into_iter().map(|c| Ok(c.clone())).collect::<Vec<_>>().into_iter()
            }
            (PgConfigSelector::Specific(label), _) => vec![self.get(label)].into_iter(),
        }
    }

    pub fn get(&self, label: &str) -> eyre::Result<PgConfig> {
        for pg_config in self.pg_configs.iter() {
            if pg_config.label()? == label {
                return Ok(pg_config.clone());
            }
        }
        Err(eyre!("Postgres `{}` is not managed by pgrx", label))
    }

    /// Returns true if the specified `label` represents a Postgres version number feature flag,
    /// such as `pg14` or `pg15`
    pub fn is_feature_flag(&self, label: &str) -> bool {
        for pgver in SUPPORTED_VERSIONS() {
            if label == &format!("pg{}", pgver.major) {
                return true;
            }
        }
        false
    }

    pub fn home() -> Result<PathBuf, PgrxHomeError> {
        let pgrx_home = std::env::var("PGRX_HOME").map_or_else(
            |_| {
                let mut pgrx_home = match dirs::home_dir() {
                    Some(home) => home,
                    None => return Err(PgrxHomeError::NoHomeDirectory),
                };

                pgrx_home.push(".pgrx");
                Ok(pgrx_home)
            },
            |v| Ok(v.into()),
        )?;

        if pgrx_home.exists() {
            Ok(pgrx_home)
        } else {
            Err(PgrxHomeError::MissingPgrxHome(pgrx_home))
        }
    }

    /// Get the postmaster stub directory
    ///
    /// We isolate postmaster stubs to an independent directory instead of alongside the postmaster
    /// because in the case of `cargo pgrx install` the `pg_config` may not necessarily be one managed
    /// by pgrx.
    pub fn postmaster_stub_dir() -> Result<PathBuf, std::io::Error> {
        let mut stub_dir = Self::home()?;
        stub_dir.push("postmaster_stubs");
        Ok(stub_dir)
    }

    pub fn config_toml() -> Result<PathBuf, std::io::Error> {
        let mut path = Pgrx::home()?;
        path.push("config.toml");
        Ok(path)
    }
}

#[allow(non_snake_case)]
pub fn SUPPORTED_VERSIONS() -> Vec<PgVersion> {
    vec![
        PgVersion::new(11, PgMinorVersion::Latest, None),
        PgVersion::new(12, PgMinorVersion::Latest, None),
        PgVersion::new(13, PgMinorVersion::Latest, None),
        PgVersion::new(14, PgMinorVersion::Latest, None),
        PgVersion::new(15, PgMinorVersion::Latest, None),
        PgVersion::new(
            16,
            PgMinorVersion::Beta(2),
            Some(
                Url::parse(
                    "https://ftp.postgresql.org/pub/source/v16beta2/postgresql-16beta2.tar.bz2",
                )
                .expect("invalid url for v16beta2"),
            ),
        ),
    ]
}

pub fn is_supported_major_version(v: u16) -> bool {
    SUPPORTED_VERSIONS().into_iter().any(|pgver| pgver.major == v)
}

pub fn createdb(
    pg_config: &PgConfig,
    dbname: &str,
    is_test: bool,
    if_not_exists: bool,
) -> eyre::Result<bool> {
    if if_not_exists && does_db_exist(pg_config, dbname)? {
        return Ok(false);
    }

    println!("{} database {}", "     Creating".bold().green(), dbname);
    let mut command = Command::new(pg_config.createdb_path()?);
    command
        .env_remove("PGDATABASE")
        .env_remove("PGHOST")
        .env_remove("PGPORT")
        .env_remove("PGUSER")
        .arg("-h")
        .arg(pg_config.host())
        .arg("-p")
        .arg(if is_test {
            pg_config.test_port()?.to_string()
        } else {
            pg_config.port()?.to_string()
        })
        .arg(dbname)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let command_str = format!("{:?}", command);

    let child = command.spawn().wrap_err_with(|| {
        format!("Failed to spawn process for creating database using command: '{command_str}': ")
    })?;

    let output = child.wait_with_output().wrap_err_with(|| {
        format!(
            "failed waiting for spawned process to create database using command: '{command_str}': "
        )
    })?;

    if !output.status.success() {
        return Err(eyre!(
            "problem running createdb: {}\n\n{}{}",
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        ));
    }

    Ok(true)
}

fn does_db_exist(pg_config: &PgConfig, dbname: &str) -> eyre::Result<bool> {
    let mut command = Command::new(pg_config.psql_path()?);
    command
        .arg("-XqAt")
        .env_remove("PGUSER")
        .arg("-h")
        .arg(pg_config.host())
        .arg("-p")
        .arg(pg_config.port()?.to_string())
        .arg("template1")
        .arg("-c")
        .arg(&format!(
            "select count(*) from pg_database where datname = '{}';",
            dbname.replace("'", "''")
        ))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let command_str = format!("{:?}", command);
    let output = command.output()?;

    if !output.status.success() {
        return Err(eyre!(
            "problem checking if database '{}' exists: {}\n\n{}{}",
            dbname,
            command_str,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        ));
    } else {
        let count = i32::from_str(&String::from_utf8(output.stdout).unwrap().trim())
            .wrap_err("result is not a number")?;
        Ok(count > 0)
    }
}

#[test]
fn parse_version() {
    // Check some valid version strings
    let versions = [
        ("PostgreSQL 10.22", 10, 22),
        ("PostgreSQL 11.2", 11, 2),
        ("PostgreSQL 11.17", 11, 17),
        ("PostgreSQL 12.12", 12, 12),
        ("PostgreSQL 13.8", 13, 8),
        ("PostgreSQL 14.5", 14, 5),
        ("PostgreSQL 11.2-FOO-BAR+", 11, 2),
        ("PostgreSQL 10.22-", 10, 22),
    ];
    for (s, major_expected, minor_expected) in versions {
        let (major, minor) =
            PgConfig::parse_version_str(s).expect("Unable to parse version string");
        assert_eq!(major, major_expected, "Major version should match");
        assert_eq!(minor.version(), Some(minor_expected), "Minor version should match");
    }

    // Check some invalid version strings
    let _ = PgConfig::parse_version_str("10.22").expect_err("Parsed invalid version string");
    let _ =
        PgConfig::parse_version_str("PostgresSQL 10").expect_err("Parsed invalid version string");
    let _ =
        PgConfig::parse_version_str("PostgresSQL 10.").expect_err("Parsed invalid version string");
    let _ =
        PgConfig::parse_version_str("PostgresSQL 12.f").expect_err("Parsed invalid version string");
    let _ =
        PgConfig::parse_version_str("PostgresSQL .53").expect_err("Parsed invalid version string");
}

#[test]
fn from_empty_env() -> eyre::Result<()> {
    // without "PGRX_PG_CONFIG_AS_ENV" we can't get one of these
    let pg_config = PgConfig::from_env();
    assert!(pg_config.is_err());

    // but now we can
    std::env::set_var("PGRX_PG_CONFIG_AS_ENV", "true");
    std::env::set_var("PGRX_PG_CONFIG_VERSION", "PostgresSQL 15.1");
    std::env::set_var("PGRX_PG_CONFIG_INCLUDEDIR-SERVER", "/path/to/server/headers");
    std::env::set_var("PGRX_PG_CONFIG_CPPFLAGS", "some cpp flags");

    let pg_config = PgConfig::from_env().unwrap();
    assert_eq!(pg_config.major_version()?, 15, "Major version should match");
    assert_eq!(
        pg_config.minor_version()?,
        PgMinorVersion::Release(1),
        "Minor version should match"
    );
    assert_eq!(
        pg_config.includedir_server()?,
        PathBuf::from("/path/to/server/headers"),
        "includdir_server should match"
    );
    assert_eq!(pg_config.cppflags()?, OsString::from("some cpp flags"), "cppflags should match");

    // we didn't set this one in our environment
    assert!(pg_config.sharedir().is_err());
    Ok(())
}
