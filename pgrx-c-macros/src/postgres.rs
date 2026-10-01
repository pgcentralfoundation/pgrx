//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use crate::{Error, MacroInventory, MacroScanner};
use pgrx_pg_config::{PgConfig, Pgrx};
use std::path::{Path, PathBuf};

/// PostgreSQL headers and preprocessing options resolved through pgrx's configuration.
#[derive(Debug)]
pub struct PostgresConfig {
    pg_config: PgConfig,
    include_dir: PathBuf,
    clang_args: Vec<String>,
}

impl PostgresConfig {
    /// Select a configured major version, such as `18` or `pg18`.
    ///
    /// Uses the same `Pgrx::from_config().get(...)` resolution as cargo-pgrx's
    /// explicit version selector, including `PGRX_HOME` and `PGRX_PG_CONFIG_PATH`.
    pub fn resolve(version: &str) -> Result<Self, PostgresError> {
        let label =
            if version.starts_with("pg") { version.to_owned() } else { format!("pg{version}") };
        Self::from_pg_config(Pgrx::from_config()?.get(&label)?)
    }

    /// Use a PostgreSQL configuration already selected by a binding generator.
    pub fn from_pg_config(pg_config: PgConfig) -> Result<Self, PostgresError> {
        let include_dir = pg_config.includedir_server()?;
        let mut directories = vec![include_dir.clone()];
        if cfg!(target_env = "msvc") {
            directories.push(pg_config.pkgincludedir()?);
            directories.push(pg_config.includedir_server_port_win32()?);
            directories.push(pg_config.includedir_server_port_win32_msvc()?);
        }
        let mut clang_args = Vec::new();
        for directory in directories {
            let path = directory
                .to_str()
                .ok_or_else(|| PostgresError::InvalidIncludeDirectory(directory.clone()))?;
            clang_args.push(format!("-I{path}"));
        }

        // PostgreSQL's Windows CPPFLAGS are for MSVC; the binding generator also
        // omits them when parsing with Clang.
        if !cfg!(target_os = "windows") {
            let flags = pg_config.cppflags()?;
            let flags = flags.to_str().ok_or(PostgresError::InvalidCppFlags)?;
            clang_args.extend(shlex::split(flags).ok_or(PostgresError::InvalidCppFlags)?);
        }
        Ok(Self { pg_config, include_dir, clang_args })
    }

    /// The selected configuration, including the actual server version and pg_config path.
    pub fn pg_config(&self) -> &PgConfig {
        &self.pg_config
    }

    /// The default entry header for the selected PostgreSQL installation.
    pub fn header(&self) -> PathBuf {
        self.include_dir.join("postgres.h")
    }

    /// Compiler arguments derived from the server include directory and CPPFLAGS.
    pub fn clang_args(&self) -> &[String] {
        &self.clang_args
    }

    /// Inventory the selected server's `postgres.h`, or a supplied wrapper header.
    ///
    /// Extra compiler arguments follow the installation's arguments, allowing
    /// callers to supply the same target, include, and define options as bindgen.
    pub fn scan(
        &self,
        scanner: &MacroScanner,
        header: Option<&Path>,
        extra_clang_args: &[String],
    ) -> Result<MacroInventory, PostgresError> {
        let default_header = self.header();
        let header = header.unwrap_or(&default_header);
        let mut clang_args = self.clang_args.clone();
        clang_args.extend_from_slice(extra_clang_args);
        Ok(scanner.scan(header, &clang_args)?)
    }
}

/// Failure to resolve a PostgreSQL installation or inventory its macros.
#[derive(Debug, thiserror::Error)]
pub enum PostgresError {
    #[error("PostgreSQL configuration: {0:#}")]
    Configuration(#[from] eyre::Report),
    #[error("Clang requires a UTF-8 include directory: {}", .0.display())]
    InvalidIncludeDirectory(PathBuf),
    #[error("PostgreSQL CPPFLAGS must be UTF-8 with balanced shell quoting")]
    InvalidCppFlags,
    #[error(transparent)]
    Discovery(#[from] Error),
}
