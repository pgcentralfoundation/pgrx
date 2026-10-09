//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare emitted PostgreSQL scalar macros with their installed C definitions.
//!
//! The configured installation supplies headers, flags, and compiler facts. A
//! standalone Rust consumer and an original-header C oracle record types, values,
//! and operand counts; checked-in bindings and handwritten ports supply no truth.
//!
//! These generated consumers use the actual inspected C profile on each native host.
//! Cross-profile checks retain original compiler facts without executing foreign code.

/// Select installed PostgreSQL header oracles from configured metadata.
#[path = "support/postgres.rs"]
mod installed;
/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, emit, inspect, validate_support_profile,
};
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize libclang-backed installed and cross-profile tests because its runtime permits
/// only one active scanner owner in this process.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Require explicit CI selections to resolve an installation, while preserving optional
/// omission without PG_VER; subprocesses isolate selection from the developer's environment.
#[test]
#[cfg(unix)]
fn installed_oracle_selection_requires_explicit_configuration() {
    use std::os::unix::fs::PermissionsExt;
    use std::process::Command;

    if std::env::var_os("PGRX_C_MACROS_TEST_SELECTION_PROBE").is_some() {
        assert!(installed::configured().is_empty());
        return;
    }
    let directory = tempfile::tempdir().unwrap();
    let home = directory.path().join("home");
    std::fs::create_dir(&home).unwrap();
    let run =
        |home: &std::path::Path, version: Option<&str>, pg_config: Option<&std::path::Path>| {
            let mut command = Command::new(std::env::current_exe().unwrap());
            command
                .args([
                    "--exact",
                    "installed_oracle_selection_requires_explicit_configuration",
                    "--nocapture",
                ])
                .env("PGRX_C_MACROS_TEST_SELECTION_PROBE", "1")
                .env("PGRX_HOME", home)
                .env_remove("PG_VER")
                .env_remove("PGRX_PG_CONFIG_PATH")
                .env_remove("PGRX_PG_CONFIG_AS_ENV")
                .env_remove("PG_CONFIG");
            if let Some(version) = version {
                command.env("PG_VER", version);
            }
            if let Some(pg_config) = pg_config {
                command.env("PGRX_PG_CONFIG_PATH", pg_config);
            }
            command.output().unwrap()
        };
    for absent in [&home, &directory.path().join("absent")] {
        let optional = run(absent, None, None);
        assert!(optional.status.success());
        assert!(String::from_utf8_lossy(&optional.stderr).contains("no pgrx configuration"));
        let required = run(absent, Some("18"), None);
        assert!(!required.status.success());
        assert!(
            String::from_utf8_lossy(&required.stderr)
                .contains("PG_VER Some(18) requires a configured PostgreSQL installation")
        );
    }
    let malformed = run(&home, Some("invalid"), None);
    assert!(!malformed.status.success());
    assert!(
        String::from_utf8_lossy(&malformed.stderr)
            .contains("PG_VER must select a PostgreSQL major version")
    );

    std::fs::write(home.join("config.toml"), "[configs]\n").unwrap();
    let optional = run(&home, None, None);
    assert!(optional.status.success());
    assert!(
        String::from_utf8_lossy(&optional.stderr).contains("no supported version is configured")
    );
    let required = run(&home, Some("pg18"), None);
    assert!(!required.status.success());
    assert!(
        String::from_utf8_lossy(&required.stderr).contains("no supported version is configured")
    );

    let pg_config = directory.path().join("pg_config");
    std::fs::write(
        &pg_config,
        "#!/bin/sh\ncase \"$1\" in\n  --version) echo 'PostgreSQL 18.0';;\n  *) exit 1;;\nesac\n",
    )
    .unwrap();
    std::fs::set_permissions(&pg_config, std::fs::Permissions::from_mode(0o700)).unwrap();
    let mismatch = run(&home, Some("19"), Some(&pg_config));
    assert!(!mismatch.status.success());
    assert!(
        String::from_utf8_lossy(&mismatch.stderr)
            .contains("PG_VER Some(19) is not among configured versions [18]")
    );
}

/// Prove actual unsigned-char AArch64 profiles emit without changing their flags, while
/// signed-char profiles retain their distinct compiler-owned representation.
#[test]
fn installed_oracle_runtime_gate_retains_actual_cross_profile_admission() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/installed_profile.h");
    let unsigned =
        inspect(&scanner, &header, &["--target=aarch64-unknown-linux-gnu".into()], None).unwrap();
    assert!(!unsigned.profile().target.char_is_signed);
    assert_eq!(validate_support_profile(unsigned.profile()), Ok(()));
    assert!(installed::supports_rust_comparisons(&scanner, &unsigned));
    assert!(!unsigned.profile().arguments.iter().any(|flag| flag == "-fsigned-char"));
    assert!(oracle::run_c(
        &unsigned.profile().compiler.executable,
        &header,
        "_Static_assert((char)-1 > 0, \"the original AArch64 profile keeps unsigned char\");\n_Static_assert(TYPEALIGN(8, 13) == 16, \"original C alignment remains valid\");\n",
        &unsigned.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    ).is_empty());
    let session = AnalysisSession::prepare(&scanner, &unsigned, &["TYPEALIGN"]).unwrap();
    assert!(matches!(emit(&session, "TYPEALIGN").status, EmissionStatus::Emitted { .. }));

    let signed =
        inspect(&scanner, &header, &["--target=x86_64-unknown-linux-gnu".into()], None).unwrap();
    assert!(signed.profile().target.char_is_signed);
    assert!(installed::supports_rust_comparisons(&scanner, &signed));
    let session = AnalysisSession::prepare(&scanner, &signed, &["TYPEALIGN"]).unwrap();
    assert!(matches!(emit(&session, "TYPEALIGN").status, EmissionStatus::Emitted { .. }));
}

/// Check generated scalar macros against every selected installed PostgreSQL C definition,
/// preserving original result types, values, and argument evaluation counts.
#[test]
fn generated_installed_postgres_macros_match_original_c_types_values_and_occurrences() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let installations = installed::configured();
    if installations.is_empty() {
        return;
    }
    let scanner = MacroScanner::new().expect("libclang must be available");
    for postgres in installations {
        let major = postgres.pg_config().major_version().unwrap();
        let frontend = installed::inspect(&scanner, &postgres, &[]);
        if !installed::supports_rust_comparisons(&scanner, &frontend) {
            continue;
        }
        let required = [
            "TYPEALIGN",
            "TYPEALIGN_DOWN",
            "TYPEALIGN64",
            "MAXALIGN",
            "MAXALIGN_DOWN",
            "MAXALIGN64",
            "OffsetNumberNext",
            "OffsetNumberPrev",
            "IS_HIGHBIT_SET",
            "PG_PROTOCOL_MAJOR",
            "PG_PROTOCOL_MINOR",
        ];
        let optional = ["TransactionIdEquals", "TransactionIdIsValid", "TransactionIdIsNormal"];
        let names = required.iter().chain(&optional).copied().collect::<Vec<_>>();
        let session = AnalysisSession::prepare(&scanner, &frontend, &names)
            .expect("expand installed PostgreSQL originals");
        let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap();
        let mut rust =
            format!("#[path = {:?}]\npub mod __pgrx_c_macros;\n", support.to_str().unwrap());
        let artifact = pgrx_c_macros::emit_support_artifact_with_bindings(
            &session,
            &names,
            &pgrx_c_macros::BindingCatalog::default(),
        )
        .unwrap();
        assert!(
            artifact.c_source.is_empty(),
            "integer PostgreSQL corpus requires no native adapters"
        );
        rust.push_str(&artifact.rust);
        for name in required {
            let emission = emit(&session, name);
            let EmissionStatus::Emitted { rust: generated, .. } = emission.status else {
                panic!("required original PG{major} macro {name} must emit: {emission:?}");
            };
            rust.push_str(&generated);
        }
        let mut c_prefix = String::new();
        let mut transactions = String::new();
        let mut optional_records = 0;
        for (name, c_flag, invocation) in [
            (
                "TransactionIdEquals",
                "ORACLE_TRANSACTION_EQUALS",
                "TransactionIdEquals!(first_transaction(), second_transaction())",
            ),
            (
                "TransactionIdIsValid",
                "ORACLE_TRANSACTION_VALID",
                "TransactionIdIsValid!(first_transaction())",
            ),
            (
                "TransactionIdIsNormal",
                "ORACLE_TRANSACTION_NORMAL",
                "TransactionIdIsNormal!(first_transaction())",
            ),
        ] {
            let emission = emit(&session, name);
            match emission.status {
                EmissionStatus::Emitted { rust: generated, .. } => {
                    rust.push_str(&generated);
                    c_prefix.push_str(&format!("#define {c_flag} 1\n"));
                    transactions.push_str(&format!(
                    "first.set(0); second.set(0);\nlet value = {invocation};\nrecord({name:?}, 0, 42, value, first.get(), second.get());\n"
                ));
                    optional_records += 1;
                }
                EmissionStatus::Skipped { reason } => {
                    eprintln!(
                        "PG{major} optional {name} unavailable: {:?}: {}",
                        reason.code, reason.message
                    );
                }
            }
        }
        rust.push_str(
            &include_str!("fixtures/emit_postgres.rs")
                .replace("// @TRANSACTION_CASES@", &transactions),
        );
        let c = format!("{c_prefix}{}", include_str!("fixtures/emit_postgres.c"));
        let profile = frontend.profile();
        let original = oracle::run_c(
            &profile.compiler.executable,
            &profile.header,
            &c,
            &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            true,
        );
        let emitted = rust_oracle::run_rust_with_cfg(
            &rust,
            &pgrx_c_macros::support_rust_cfg(profile).unwrap(),
        );
        let mut original_rows = original.lines();
        let mut emitted_rows = emitted.lines();
        let mut rows = 0;
        loop {
            match (original_rows.next(), emitted_rows.next()) {
                (Some(c), Some(rust)) => {
                    assert_eq!(rust, c, "original C and generated Rust differ at record {rows}");
                    rows += 1;
                }
                (None, None) => break,
                _ => panic!(
                    "original C and generated Rust have different record counts after {rows}"
                ),
            }
        }
        assert_eq!(
            rows,
            7 * 4096 * 3 + 4096 * 3 + 6 + 256 * 2 + 256 + 256 * 2 + 8 + optional_records
        );
        assert!(
            original
                .lines()
                .any(|row| row.starts_with("TYPEALIGN_MAX\t8\t0\t") && row.ends_with("\t0\t0\t0"))
        );
        assert!(
            original
                .lines()
                .any(|row| row.starts_with("TYPEALIGN_NEGATIVE\t8\t0\t")
                    && row.ends_with("\t0\t0\t0"))
        );
        eprintln!("PG{major}: {rows} scalar C/Rust records");
    }
}
