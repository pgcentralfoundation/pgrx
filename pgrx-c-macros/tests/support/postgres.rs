//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Select installed-header oracles from the same pgrx metadata as binding generation.
//!
//! Ordinary cargo tests exercise every configured supported installation, or the
//! exact PG_VER selected by a CI matrix. Explicit pg_config and metadata overrides
//! take precedence through pgrx-pg-config. Missing installations are reported as
//! omitted coverage only when no version was selected; explicit selection,
//! malformed configuration and compiler failures remain errors.
//! The oracles inspect packaged C wrappers and never enter a PostgreSQL backend.

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, FrontendOutput, MacroScanner, PostgresConfig, SkipReasonCode,
    SupportProfileError, emit, validate_support_profile,
};
use pgrx_pg_config::{PgConfig, PgConfigSelector, Pgrx};
use std::path::Path;

/// Resolve supported configured installations without creating pgrx state or requiring every
/// major version; an explicit CI selection must never silently test another installation.
pub fn configured() -> Vec<PostgresConfig> {
    let selected = match std::env::var("PG_VER") {
        Ok(version) => Some(version),
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(_)) => panic!("PG_VER must be UTF-8"),
    }
    .map(|version| {
        let major = version
            .strip_prefix("pg")
            .unwrap_or(&version)
            .parse::<u16>()
            .expect("PG_VER must select a PostgreSQL major version");
        assert!(pgrx_pg_config::is_supported_major_version(major), "unsupported PG_VER {major}");
        major
    });
    let overridden = match std::env::var("PGRX_PG_CONFIG_PATH") {
        Ok(_) => true,
        Err(std::env::VarError::NotPresent) => false,
        Err(std::env::VarError::NotUnicode(_)) => panic!("PGRX_PG_CONFIG_PATH must be UTF-8"),
    };
    let configurations = if PgConfig::is_in_environment() {
        Pgrx::default()
    } else {
        if !overridden {
            let configured = match Pgrx::config_toml() {
                Ok(path) => path.try_exists().expect("inspect pgrx configuration path"),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
                Err(error) => panic!("locate installed PostgreSQL configuration: {error}"),
            };
            if !configured {
                assert!(
                    selected.is_none(),
                    "PG_VER {selected:?} requires a configured PostgreSQL installation; no pgrx configuration"
                );
                eprintln!("installed PostgreSQL C oracle omitted: no pgrx configuration");
                return Vec::new();
            }
        }
        Pgrx::from_config().expect("read installed PostgreSQL configuration")
    };
    let mut postgres = Vec::new();
    let mut configured_versions = Vec::new();
    for configuration in configurations.iter(PgConfigSelector::All) {
        let configuration = configuration.expect("resolve installed PostgreSQL metadata");
        let major = configuration.major_version().expect("read installed PostgreSQL version");
        configured_versions.push(major);
        if selected.is_none_or(|selected| selected == major) {
            postgres.push(
                PostgresConfig::from_pg_config(configuration)
                    .expect("resolve installed PostgreSQL headers and flags"),
            );
        }
    }
    if configured_versions.is_empty() {
        assert!(
            selected.is_none(),
            "PG_VER {selected:?} requires a configured PostgreSQL installation; no supported version is configured"
        );
        eprintln!("installed PostgreSQL C oracle omitted: no supported version is configured");
    } else {
        assert!(
            !postgres.is_empty(),
            "PG_VER {selected:?} is not among configured versions {configured_versions:?}"
        );
    }
    postgres
}

/// Inspect the selected installation's real headers and flags without populating the user's
/// wrapper cache; packaged wrapper bytes are checked against the canonical pg-sys inputs.
pub fn inspect(
    scanner: &MacroScanner,
    postgres: &PostgresConfig,
    arguments: &[String],
) -> FrontendOutput {
    let major = postgres.pg_config().major_version().expect("read inspected PostgreSQL version");
    let header = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("include/pg{major}.h"));
    eprintln!("installed PostgreSQL C oracle: {}", postgres.pg_config());
    postgres
        .inspect(scanner, Some(&header), arguments, None)
        .unwrap_or_else(|error| panic!("inspect installed PG{major} profile: {error}"))
}

/// Require either a supported comparison profile or an exact fail-closed refusal of the real
/// TYPEALIGN definition, reporting unsupported runtime coverage without changing C flags.
#[allow(dead_code)] // The original-C-only oracle has no Rust runtime admission requirement.
pub fn supports_rust_comparisons(scanner: &MacroScanner, frontend: &FrontendOutput) -> bool {
    let Err(refusal) = validate_support_profile(frontend.profile()) else {
        return true;
    };
    let session = AnalysisSession::prepare(scanner, frontend, &["TYPEALIGN"])
        .expect("expand TYPEALIGN under the original unsupported profile");
    let emission = emit(&session, "TYPEALIGN");
    let EmissionStatus::Skipped { reason } = emission.status else {
        panic!("unsupported runtime profile must refuse TYPEALIGN: {emission:?}");
    };
    assert_eq!(reason.code, SkipReasonCode::UnsupportedProfile);
    let expected = match &refusal {
        SupportProfileError::UnsupportedOptions(options) => {
            format!("unmodeled semantic options: {options}")
        }
        _ => refusal.to_string(),
    };
    assert_eq!(reason.message, expected, "TYPEALIGN must retain the precise profile refusal");
    eprintln!(
        "installed PostgreSQL C/Rust comparison coverage excluded for {}: {refusal}; TYPEALIGN was refused with UnsupportedProfile",
        frontend.profile().target.triple,
    );
    false
}
