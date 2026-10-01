//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use crate::{Error, MacroDefinition, MacroInventory, MacroKind, MacroScanner};
use pgrx_pg_config::{PgConfig, Pgrx};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// PostgreSQL function macros and the other definitions needed to interpret them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostgresInventory {
    /// Function-like macros from the selected server header tree, in preprocessing order.
    pub inventory: MacroInventory,
    /// Other definitions, in preprocessing order, retained only as conversion context.
    ///
    /// Includes PostgreSQL object-like macros, macros in external headers, compiler-provided
    /// macros, command-line definitions, and definitions in a wrapper outside the server tree.
    pub context: Vec<MacroDefinition>,
}

/// PostgreSQL headers and preprocessing options resolved through pgrx's configuration.
#[derive(Debug)]
pub struct PostgresConfig {
    pg_config: PgConfig,
    header: PathBuf,
    server_include_dir: PathBuf,
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
        let server_include_dir = include_dir.clone();
        let header = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/include")
            .join(format!("pg{}.h", pg_config.major_version()?));
        let mut directories = vec![include_dir];
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
        Ok(Self { pg_config, header, server_include_dir, clang_args })
    }

    /// The selected configuration, including the actual server version and pg_config path.
    pub fn pg_config(&self) -> &PgConfig {
        &self.pg_config
    }

    /// The selected version's pgrx wrapper header in this source checkout.
    pub fn header(&self) -> PathBuf {
        self.header.clone()
    }

    /// Compiler arguments derived from the server include directory and CPPFLAGS.
    pub fn clang_args(&self) -> &[String] {
        &self.clang_args
    }

    /// Inventory PostgreSQL function macros from the version's pgrx wrapper or supplied header.
    ///
    /// Only definitions physically inside the selected server include directory become
    /// candidates. Other definitions are retained separately as conversion context. Paths
    /// are resolved before classification, so symlinks and `..` cannot disguise their origin.
    /// A supplied wrapper or extra include directory does not expand the PostgreSQL tree.
    ///
    /// Extra compiler arguments follow the installation's arguments, allowing
    /// callers to supply the same target, include, and define options as bindgen.
    pub fn scan(
        &self,
        scanner: &MacroScanner,
        header: Option<&Path>,
        extra_clang_args: &[String],
    ) -> Result<PostgresInventory, PostgresError> {
        let server_include_dir = canonicalize_header_path(&self.server_include_dir)?;
        let header = header.unwrap_or(&self.header);
        let mut clang_args = self.clang_args.clone();
        clang_args.extend_from_slice(extra_clang_args);
        let MacroInventory { macros, diagnostics } = scanner.scan(header, &clang_args)?;
        let mut inventory = MacroInventory { macros: Vec::new(), diagnostics };
        let mut context = Vec::new();
        // Resolve each source file once, rather than performing filesystem work per macro.
        let mut sources = HashMap::new();
        for definition in macros {
            let postgres = if let Some(span) = &definition.provenance {
                if let Some(postgres) = sources.get(&span.file) {
                    *postgres
                } else {
                    let postgres =
                        canonicalize_header_path(&span.file)?.starts_with(&server_include_dir);
                    sources.insert(span.file.clone(), postgres);
                    postgres
                }
            } else {
                false
            };
            if postgres && definition.kind == MacroKind::FunctionLike {
                inventory.macros.push(definition);
            } else {
                context.push(definition);
            }
        }
        Ok(PostgresInventory { inventory, context })
    }
}

fn canonicalize_header_path(path: &Path) -> Result<PathBuf, PostgresError> {
    path.canonicalize().map_err(|source| PostgresError::HeaderPath { path: path.into(), source })
}

/// Failure to resolve a PostgreSQL installation or inventory its macros.
#[derive(Debug, thiserror::Error)]
pub enum PostgresError {
    #[error("PostgreSQL configuration: {0:#}")]
    Configuration(#[from] eyre::Report),
    #[error("could not resolve header path {}: {source}", path.display())]
    HeaderPath { path: PathBuf, source: std::io::Error },
    #[error("Clang requires a UTF-8 include directory: {}", .0.display())]
    InvalidIncludeDirectory(PathBuf),
    #[error("PostgreSQL CPPFLAGS must be UTF-8 with balanced shell quoting")]
    InvalidCppFlags,
    #[error(transparent)]
    Discovery(#[from] Error),
}
