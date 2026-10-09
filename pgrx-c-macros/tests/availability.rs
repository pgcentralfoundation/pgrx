//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Prove that availability describes the emitted API even without operand adapters.
//!
//! Header-only cross profiles separate compiler facts from the host Rust ABI. Consumers expand
//! only availability queries, which must need neither runtime types nor native code. Invalid
//! bodies are deliberately retained in those queries to prove unavailable tokens never escape.

/// Compile and reject consumers with bounded compiler output and runtime.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, MacroScanner, SkipReasonCode,
    emit_unavailable_macro_support, generate_with_bindings, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize the libclang runtime, which permits only one live owner per test process.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Prove explicit unavailability needs no runtime module and does not accept malformed queries.
#[test]
fn unavailable_support_answers_queries_without_runtime_capabilities() {
    let support = emit_unavailable_macro_support();
    rust_oracle::run_rust(&format!(
        "{support}\nfn main() {{ __pgrx_c_classify!(@if_available MISSING {{ compile_error!(\"unavailable body escaped\"); missing::function(); }}); }}"
    ));
    let rejection = rust_oracle::reject_rust(&format!(
        "{support}\n__pgrx_c_classify!(@if_available); fn main() {{}}"
    ));
    assert!(rejection.contains("unexpected end of macro invocation"), "{rejection}");
    assert!(!support.contains("__pgrx_c_operand"));
    assert!(!support.contains("__pgrx_c_macros"));
}

/// Prove genuinely unsupported trapping-overflow profiles and empty selections publish no
/// available C macros, independently of the target's supported plain-char signedness.
#[test]
fn all_skipped_and_empty_batches_still_answer_availability_queries() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/availability.h");
    let frontend = inspect(
        &scanner,
        &header,
        &["--target=aarch64-unknown-linux-gnu".into(), "-ftrapv".into()],
        None,
    )
    .unwrap();
    assert!(!frontend.profile().target.char_is_signed);
    let names = ["AVAIL_ZERO", "AVAIL_VALUE"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let refused = generate_with_bindings(&session, &names, &BindingCatalog::default()).unwrap();
    assert_eq!(refused.macros.len(), names.len());
    for emission in &refused.macros {
        let EmissionStatus::Skipped { reason } = &emission.status else {
            panic!("trapping-overflow runtime must remain unsupported: {emission:?}");
        };
        assert_eq!(reason.code, SkipReasonCode::UnsupportedProfile);
        assert!(reason.message.contains("trapping signed overflow"), "{reason:?}");
    }
    let empty =
        generate_with_bindings(&session, &[] as &[&str], &BindingCatalog::default()).unwrap();
    assert!(empty.macros.is_empty());
    for generated in [refused, empty] {
        assert!(generated.support.c_source.is_empty());
        assert!(!generated.support.rust.contains("__pgrx_c_operand"));
        assert!(!generated.support.rust.contains("__pgrx_c_generated"));
        rust_oracle::run_rust(&format!(
            "{}\n__pgrx_c_classify!(@if_available AVAIL_ZERO {{ compile_error!(\"refused macro appeared\"); }});\n__pgrx_c_classify!(@if_available AVAIL_VALUE {{ compile_error!(\"refused macro appeared\"); }});\nfn main() {{}}",
            generated.support.rust,
        ));
    }
}

/// Prove a successful zero-argument macro remains available without generating operand code.
#[test]
fn zero_argument_exports_are_available_without_operand_adapters() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/availability.h");
    let frontend =
        inspect(&scanner, &header, &["--target=x86_64-unknown-linux-gnu".into()], None).unwrap();
    let names = ["AVAIL_ZERO"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let generated = generate_with_bindings(&session, &names, &BindingCatalog::default()).unwrap();
    let EmissionStatus::Emitted { .. } = &generated.macros[0].status else {
        panic!("zero-argument literal macro must emit: {:?}", generated.macros[0]);
    };
    assert!(generated.support.c_source.is_empty());
    assert!(!generated.support.rust.contains("__pgrx_c_operand"));
    rust_oracle::run_rust(&format!(
        "{}\n__pgrx_c_classify!(@if_available AVAIL_ZERO {{ const PRESENT: bool = true; }});\n__pgrx_c_classify!(@if_available AVAIL_VALUE {{ compile_error!(\"unrequested macro appeared\"); }});\nfn main() {{ assert!(PRESENT); }}",
        generated.support.rust,
    ));
}
