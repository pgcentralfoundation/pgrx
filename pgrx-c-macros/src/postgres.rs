//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! PostgreSQL resolution follows pgrx-pg-config and the version-specific pgrx wrapper header.
//!
//! The configuration carries server include ownership and recorded compiler flags into the
//! scanner/frontend. PostgreSQL function macros form the public inventory; object macros and
//! external-header definitions remain context for expansion. Canonical path identities enforce
//! ownership without changing include lookup spelling.

use crate::{
    ActiveProvenance, Error, FrontendOutput, MacroDefinition, MacroInventory, MacroKind,
    MacroScanner,
};
use pgrx_pg_config::{PgConfig, Pgrx};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::io::Write;
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
    /// Resolved installation descriptor supplying version, directories, and recorded build flags.
    pg_config: PgConfig,
    /// The selected version chooses the embedded wrapper without requiring a source checkout.
    major_version: u16,
    /// Server header ownership root used to select public PostgreSQL function macros.
    server_include_dir: PathBuf,
    /// Installation compiler flags and include arguments forwarded consistently into discovery and
    /// analysis.
    clang_args: Vec<String>,
}

/// Resolve the installation once and reuse its wrapper header, flags, and ownership root across
/// pipeline phases.
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
        let major_version = pg_config.major_version()?;
        let mut directories = vec![include_dir];
        if cfg!(target_env = "msvc") {
            directories.push(pg_config.pkgincludedir()?);
            directories.push(pg_config.includedir_server_port_win32()?);
            directories.push(pg_config.includedir_server_port_win32_msvc()?);
        }
        let mut clang_args = Vec::new();
        // PostgreSQL's Windows CPPFLAGS are for MSVC; the binding generator also
        // omits them when parsing with Clang.
        if !cfg!(target_os = "windows") {
            let flags = pg_config.cppflags()?;
            let flags = flags.to_str().ok_or(PostgresError::InvalidCppFlags)?;
            clang_args.extend(shlex::split(flags).ok_or(PostgresError::InvalidCppFlags)?);
        }
        for directory in directories {
            let path = directory
                .to_str()
                .ok_or_else(|| PostgresError::InvalidIncludeDirectory(directory.clone()))?;
            clang_args.push(format!("-I{path}"));
        }
        Ok(Self { pg_config, major_version, server_include_dir, clang_args })
    }

    /// The selected configuration, including the actual server version and pg_config path.
    pub fn pg_config(&self) -> &PgConfig {
        &self.pg_config
    }

    /// Materialize the embedded wrapper only when a default header is requested.
    ///
    /// Its content-derived cache path survives this configuration, so returned compiler
    /// snapshots and native generation retain a physical input after this object is dropped.
    /// Explicit header callers never create or consult this cache.
    pub fn header(&self) -> Result<PathBuf, PostgresError> {
        let home = match Pgrx::home() {
            Ok(home) | Err(pgrx_pg_config::PgrxHomeError::MissingPgrxHome(home)) => home,
            Err(source) => return Err(PostgresError::Configuration(source.into())),
        };
        materialize_wrapper(self.major_version, &home.join("c-macros/headers"))
    }

    /// Compiler arguments derived from the server include directory and CPPFLAGS.
    pub fn clang_args(&self) -> &[String] {
        &self.clang_args
    }

    /// Inspect the final C environment using recorded compiler flags when available.
    ///
    /// Explicit arguments follow the recorded flags, so callers can supply the target and
    /// overrides used by a binding generator. Discovery through [`Self::scan`] retains its
    /// existing preprocessing-only behavior. Missing historical metadata selects that current
    /// binding invocation; it never invents the installed server's optimization flags.
    pub fn inspect(
        &self,
        scanner: &MacroScanner,
        header: Option<&Path>,
        extra_clang_args: &[String],
        compiler: Option<&Path>,
    ) -> Result<FrontendOutput, PostgresError> {
        let mut arguments = Vec::new();
        if let Some(cflags) = self.pg_config.optional_cflags()? {
            let cflags = cflags.to_str().ok_or(PostgresError::InvalidCFlags)?;
            arguments
                .extend(crate::split_recorded_cflags(cflags, cfg!(windows))?.unwrap_or_default());
        }
        // PostgreSQL's COMPILE.c applies CFLAGS before CPPFLAGS. Preserve that
        // precedence before the same include defaults and explicit overrides as bindgen.
        arguments.extend_from_slice(&self.clang_args);
        arguments.extend_from_slice(extra_clang_args);
        let default_header;
        let header = match header {
            Some(header) => header,
            None => {
                default_header = self.header()?;
                &default_header
            }
        };
        self.inspect_with_arguments(scanner, header, &arguments, compiler)
    }

    /// Inspect the exact flags already resolved by a binding generator.
    ///
    /// The caller supplies the complete CFLAGS, CPPFLAGS, target, include directories and
    /// overrides in their intended order. No installation flags are added here. Inspection
    /// may append an explicit compiler resource directory; use the returned profile's
    /// arguments when parsing the bindings so both tools see the same environment.
    pub fn inspect_with_arguments(
        &self,
        scanner: &MacroScanner,
        header: &Path,
        arguments: &[String],
        compiler: Option<&Path>,
    ) -> Result<FrontendOutput, PostgresError> {
        let configured_compiler = if compiler.is_none()
            && std::env::var_os("CLANG_PATH").is_none()
            && self.pg_config.is_real()
        {
            self.pg_config
                .configure()?
                .get("CLANG")
                .filter(|value| !value.is_empty())
                .map(PathBuf::from)
        } else {
            None
        };
        let mut inspected = crate::frontend::inspect_with_compiler_hint(
            scanner,
            header,
            arguments,
            compiler,
            configured_compiler.as_deref(),
        )?;
        if let Some(pg_config) = self.pg_config.path() {
            inspected.profile.inputs.files.push(pg_config);
        }
        if let Ok(config) = Pgrx::config_toml() {
            inspected.profile.inputs.files.push(config);
        }
        for name in [
            "HOME",
            "USERPROFILE",
            "PGRX_HOME",
            "PGRX_PG_CONFIG_PATH",
            "PGRX_PG_CONFIG_AS_ENV",
            "PGRX_PG_CONFIG_VERSION",
            "PGRX_PG_CONFIG_INCLUDEDIR-SERVER",
            "PGRX_PG_CONFIG_CPPFLAGS",
            "PGRX_PG_CONFIG_CFLAGS",
            "PGRX_PG_CONFIG_CONFIGURE",
            "PGRX_PG_CONFIG_PKGINCLUDEDIR",
            "PG_CONFIG",
        ] {
            let value = match std::env::var(name) {
                Ok(value) => Some(value),
                Err(std::env::VarError::NotPresent) => None,
                Err(std::env::VarError::NotUnicode(_)) => {
                    return Err(PostgresError::Configuration(eyre::eyre!(
                        "compilation input {name} is not UTF-8"
                    )));
                }
            };
            inspected.profile.inputs.environment.insert(name.into(), value);
        }
        inspected.profile.inputs.files.sort();
        inspected.profile.inputs.files.dedup();
        let added = inspected
            .profile
            .inputs
            .files
            .iter()
            .filter(|file| !inspected.profile.inputs.fingerprints.contains_key(*file))
            .collect::<Vec<_>>();
        let identities = crate::frontend::file_identities(added.iter().copied())?;
        let fingerprints = crate::frontend::fingerprint_files(added.iter().copied())?;
        if crate::frontend::file_identities(added.iter().copied())? != identities {
            return Err(crate::FrontendError::Environment(
                "a PostgreSQL configuration target changed while recording fingerprints".into(),
            )
            .into());
        }
        inspected.profile.inputs.file_identities.extend(identities);
        inspected.profile.inputs.fingerprints.extend(fingerprints);
        Ok(inspected)
    }

    /// The physical server header tree that owns PostgreSQL macro candidates.
    pub fn server_include_dir(&self) -> &Path {
        &self.server_include_dir
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
        let default_header;
        let header = match header {
            Some(header) => header,
            None => {
                default_header = self.header()?;
                &default_header
            }
        };
        let mut clang_args = self.clang_args.clone();
        clang_args.extend_from_slice(extra_clang_args);
        let MacroInventory { macros, diagnostics } = scanner.scan(header, &clang_args)?;
        let mut inventory = MacroInventory { macros: Vec::new(), diagnostics };
        let mut context = Vec::new();
        // Resolve each source file once, rather than performing filesystem work per macro.
        let mut sources = HashMap::new();
        for definition in macros {
            let postgres = if let Some(span) = &definition.provenance {
                owns_source(&span.file, &server_include_dir, &mut sources)?
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

/// Embed the canonical versioned pgrx wrapper from this crate's published inputs, leaving every
/// included PostgreSQL header to the selected installation's actual compiler search paths.
fn embedded_wrapper(major_version: u16) -> Result<&'static str, PostgresError> {
    match major_version {
        15 => Ok(include_str!("../include/pg15.h")),
        16 => Ok(include_str!("../include/pg16.h")),
        17 => Ok(include_str!("../include/pg17.h")),
        18 => Ok(include_str!("../include/pg18.h")),
        19 => Ok(include_str!("../include/pg19.h")),
        _ => Err(PostgresError::UnsupportedWrapper(major_version)),
    }
}

/// Publish immutable embedded content atomically at a stable cache path without replacing
/// existing files; racing readers see either an absent file or the complete identical wrapper.
fn materialize_wrapper(major_version: u16, cache: &Path) -> Result<PathBuf, PostgresError> {
    let source = embedded_wrapper(major_version)?;
    let mut digest = String::with_capacity(64);
    for byte in Sha256::digest(source.as_bytes()) {
        write!(digest, "{byte:02x}").expect("writing a digest to a String is infallible");
    }
    let directory = cache.join(digest);
    let header = directory.join(format!("pg{major_version}.h"));
    let failure = |source| PostgresError::WrapperCache { path: header.clone(), source };
    match std::fs::read(&header) {
        Ok(content) if content == source.as_bytes() => return Ok(header),
        Ok(_) => {
            return Err(failure(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "cached wrapper disagrees with its embedded content",
            )));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(failure(error)),
    }
    std::fs::create_dir_all(&directory).map_err(failure)?;
    let mut staged = tempfile::NamedTempFile::new_in(&directory).map_err(failure)?;
    staged.write_all(source.as_bytes()).map_err(failure)?;
    match staged.persist_noclobber(&header) {
        Ok(_) => {}
        Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
            if std::fs::read(&header).map_err(failure)? != source.as_bytes() {
                return Err(failure(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "concurrently cached wrapper disagrees with its embedded content",
                )));
            }
        }
        Err(error) => return Err(failure(error.error)),
    }
    Ok(header)
}

/// Select final active function macros owned by a PostgreSQL server header tree.
///
/// Canonical source paths establish ownership. An external final redefinition is excluded;
/// ambiguous or unresolved definitions with PostgreSQL history remain candidates so the
/// analyzer can report why they are skipped. Results are sorted by macro name.
pub fn postgres_function_macro_names(
    inspected: &FrontendOutput,
    server_include_dir: &Path,
) -> Result<Vec<String>, PostgresError> {
    let root = canonicalize_header_path(server_include_dir)?;
    let mut sources = HashMap::new();
    let mut owned_history = HashSet::new();
    for definition in &inspected.inventory().macros {
        if definition.kind == MacroKind::FunctionLike
            && let Some(span) = &definition.provenance
            && owns_source(&span.file, &root, &mut sources)?
        {
            owned_history.insert(definition.name.as_str());
        }
    }
    let mut names = Vec::new();
    for (name, active) in &inspected.environment().active {
        if active.definition.kind != MacroKind::FunctionLike {
            continue;
        }
        let owned = match &active.provenance {
            ActiveProvenance::Resolved => match &active.definition.provenance {
                Some(span) => owns_source(&span.file, &root, &mut sources)?,
                None => false,
            },
            ActiveProvenance::Ambiguous(_) | ActiveProvenance::Unresolved => {
                owned_history.contains(name.as_str())
            }
        };
        if owned {
            names.push(name.clone());
        }
    }
    Ok(names)
}

/// Check canonical physical source ownership while caching repeated header resolutions across the
/// macro inventory.
fn owns_source(
    file: &Path,
    root: &Path,
    cache: &mut HashMap<PathBuf, bool>,
) -> Result<bool, PostgresError> {
    if let Some(owned) = cache.get(file) {
        return Ok(*owned);
    }
    let owned = canonicalize_header_path(file)?.starts_with(root);
    cache.insert(file.to_path_buf(), owned);
    Ok(owned)
}

/// Resolve a discovered header identity and retain the originating path when reporting filesystem
/// errors.
fn canonicalize_header_path(path: &Path) -> Result<PathBuf, PostgresError> {
    path.canonicalize().map_err(|source| PostgresError::HeaderPath { path: path.into(), source })
}

/// Failure to resolve a PostgreSQL installation or inventory its macros.
#[derive(Debug, thiserror::Error)]
pub enum PostgresError {
    /// No canonical wrapper is packaged for this PostgreSQL major version.
    #[error("no embedded pgrx wrapper header is available for PostgreSQL {0}")]
    UnsupportedWrapper(
        /// Requested major version whose header list cannot be supplied accurately.
        u16,
    ),
    /// The owned wrapper cache could not be published or its contents were inconsistent.
    #[error("could not materialize embedded wrapper at {}: {source}", path.display())]
    WrapperCache {
        /// Content-derived wrapper path retained for diagnostics.
        path: PathBuf,
        /// Publication or content-verification failure; existing files are never repaired silently.
        source: std::io::Error,
    },
    /// pgrx could not resolve or query the selected PostgreSQL installation.
    #[error("PostgreSQL configuration: {0:#}")]
    Configuration(
        /// Original pgrx installation configuration failure.
        #[from]
        eyre::Report,
    ),
    /// A requested wrapper/header path could not be resolved for inspection.
    #[error("could not resolve header path {}: {source}", path.display())]
    HeaderPath {
        /// The actual Rust binding path or filesystem path consumed by this phase.
        path: PathBuf,
        /// The underlying I/O or parsing failure retained for actionable diagnostics.
        source: std::io::Error,
    },
    /// The selected installation lacks a usable PostgreSQL server header ownership root.
    #[error("Clang requires a UTF-8 include directory: {}", .0.display())]
    InvalidIncludeDirectory(
        /// The unusable configured server include root.
        PathBuf,
    ),
    /// Recorded PostgreSQL preprocessing flags could not be decoded safely.
    #[error("PostgreSQL CPPFLAGS must be UTF-8 with balanced shell quoting")]
    InvalidCppFlags,
    /// Recorded PostgreSQL compiler flags could not be decoded safely.
    #[error("PostgreSQL CFLAGS must be UTF-8 with balanced shell quoting")]
    InvalidCFlags,
    /// Compiler inspection or a required probe failed.
    #[error(transparent)]
    Frontend(
        /// Original coherent-inspection or probe error.
        #[from]
        crate::FrontendError,
    ),
    /// The underlying scanner could not produce a complete reliable inventory.
    #[error(transparent)]
    Discovery(
        /// Original scanner failure preserved through the frontend/CLI boundary.
        #[from]
        Error,
    ),
}

/// Check published header inputs and cache publication without a PostgreSQL installation or
/// writes to the user's configured PGRX_HOME.
#[cfg(test)]
mod tests {
    use super::*;

    /// Require the embedded bytes to match every supported canonical pg-sys wrapper exactly,
    /// so a header update or a new PostgreSQL version cannot silently narrow CLI discovery.
    #[test]
    fn embedded_wrappers_match_canonical_headers_for_every_supported_version() {
        let canonical = Path::new(env!("CARGO_MANIFEST_DIR")).join("../pgrx-pg-sys/include");
        for version in pgrx_pg_config::SUPPORTED_VERSIONS() {
            let source = embedded_wrapper(version.major).unwrap();
            let packaged = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(format!("include/pg{}.h", version.major));
            assert_eq!(source.as_bytes(), std::fs::read(packaged).unwrap());
            // Published crates omit the sibling source tree. Workspace runs enforce its
            // canonical contents; installed-crate runs still verify every packaged version.
            if canonical.is_dir() {
                assert_eq!(
                    source.as_bytes(),
                    std::fs::read(canonical.join(format!("pg{}.h", version.major))).unwrap()
                );
            }
        }
    }

    /// Cache publication is stable under racing writers, survives owners, and refuses content
    /// changed after publication instead of silently substituting new compiler inputs.
    #[test]
    fn wrapper_cache_is_stable_and_rejects_changed_content() {
        let directory = tempfile::tempdir().unwrap();
        let barrier = std::sync::Barrier::new(2);
        let (header, second) = std::thread::scope(|scope| {
            let first = scope.spawn(|| {
                barrier.wait();
                materialize_wrapper(18, directory.path()).unwrap()
            });
            let second = scope.spawn(|| {
                barrier.wait();
                materialize_wrapper(18, directory.path()).unwrap()
            });
            (first.join().unwrap(), second.join().unwrap())
        });
        assert_eq!(header, second);
        assert_eq!(std::fs::read_to_string(&header).unwrap(), embedded_wrapper(18).unwrap());
        std::fs::write(&header, "#error unexpected cached content\n").unwrap();
        assert!(
            matches!(materialize_wrapper(18, directory.path()), Err(PostgresError::WrapperCache { source, .. }) if source.kind() == std::io::ErrorKind::InvalidData)
        );
        assert_eq!(std::fs::read_to_string(&header).unwrap(), "#error unexpected cached content\n");
    }
}
