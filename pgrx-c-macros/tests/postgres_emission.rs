//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare emitted PostgreSQL scalar macros with their installed C definitions.
//!
//! The configured installation supplies headers, flags, and compiler facts. A
//! standalone Rust consumer and an original-header C oracle record types, values,
//! and operand counts; checked-in bindings and handwritten ports supply no truth.

/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{AnalysisSession, EmissionStatus, MacroScanner, PostgresConfig, emit};
use std::path::PathBuf;

/// Checks that generated PostgreSQL 18 macros match original C types values and occurrences.
#[test]
#[ignore = "requires a configured native PostgreSQL 18 installation"]
fn generated_postgres_18_macros_match_original_c_types_values_and_occurrences() {
    let postgres = PostgresConfig::resolve("pg18").expect("a native PG18 must be configured");
    assert_eq!(postgres.pg_config().major_version().unwrap(), 18);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend =
        postgres.inspect(&scanner, None, &[], None).expect("inspect native PG18 profile");
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
    let session =
        AnalysisSession::prepare(&scanner, &frontend, &names).expect("expand PG18 originals");
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!("#[path = {:?}]\npub mod __pgrx_c_macros;\n", support.to_str().unwrap());
    let artifact = pgrx_c_macros::emit_support_artifact_with_bindings(
        &session,
        &names,
        &pgrx_c_macros::BindingCatalog::default(),
    )
    .unwrap();
    assert!(artifact.c_source.is_empty(), "integer PostgreSQL corpus requires no native adapters");
    rust.push_str(&artifact.rust);
    for name in required {
        let emission = emit(&session, name);
        let EmissionStatus::Emitted { rust: generated, .. } = emission.status else {
            panic!("required original PG18 macro {name} must emit: {emission:?}");
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
                    "PG18 optional {name} unavailable: {:?}: {}",
                    reason.code, reason.message
                );
            }
        }
    }
    rust.push_str(
        &include_str!("fixtures/emit_postgres.rs").replace("// @TRANSACTION_CASES@", &transactions),
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
    let emitted = rust_oracle::run_rust(&rust);
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
            _ => panic!("original C and generated Rust have different record counts after {rows}"),
        }
    }
    assert_eq!(rows, 7 * 4096 * 3 + 4096 * 3 + 6 + 256 * 2 + 256 + 256 * 2 + 8 + optional_records);
    assert!(
        original
            .lines()
            .any(|row| row.starts_with("TYPEALIGN_MAX\t8\t0\t") && row.ends_with("\t0\t0\t0"))
    );
    assert!(
        original
            .lines()
            .any(|row| row.starts_with("TYPEALIGN_NEGATIVE\t8\t0\t") && row.ends_with("\t0\t0\t0"))
    );
}
