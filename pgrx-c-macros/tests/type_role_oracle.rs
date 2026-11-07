//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare formal casts proven by another type use with the original C.
//!
//! C reads `(T) - 1` as a cast of `-1` once `T` names a type, wherever the proving
//! `sizeof(T *)` appears. The emitted macro must compute that cast rather than a
//! subtraction from a value operand.
//!
//! These generated consumers use the runtime's Linux/macOS host family. Emission
//! still validates the inspected C ABI and flags; unsupported-profile checks remain portable.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]

/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, MacroScanner, emit,
    emit_support_with_bindings, inspect,
};
use std::path::PathBuf;

/// Fixture macros whose `(T)` operator use precedes or follows its type proof.
const NAMES: &[&str] = &["ROLE_CAST_THEN_SIZE", "ROLE_SIZE_THEN_CAST"];

/// Resolve platform include arguments for native oracle compilation without changing the
/// fixture's C definitions.
#[cfg(target_os = "macos")]
fn native_include_arguments() -> Vec<String> {
    let sdk = rust_oracle::run_tool(
        std::process::Command::new("xcrun").arg("--show-sdk-path"),
        "Apple SDK lookup",
    );
    let sdk = sdk.trim();
    assert!(!sdk.is_empty() && std::path::Path::new(sdk).is_dir());
    vec!["-isysroot".into(), sdk.into()]
}

/// Resolve platform include arguments for native oracle compilation without changing the
/// fixture's C definitions.
#[cfg(not(target_os = "macos"))]
fn native_include_arguments() -> Vec<String> {
    Vec::new()
}

/// Checks that a proven type formal before an operator is a cast in either source order.
#[test]
fn formal_operator_casts_proven_elsewhere_match_original_c() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/analysis_type_roles.h");
    let scanner = MacroScanner::new().expect("libclang must be available");
    let mut arguments = vec!["-std=c17".into()];
    arguments.extend(native_include_arguments());
    let frontend = inspect(&scanner, &header, &arguments, None).expect("inspect original C macros");
    let session =
        AnalysisSession::prepare(&scanner, &frontend, NAMES).expect("expand original C macros");
    let support = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut rust = format!("#[path = {support:?}]\npub mod __pgrx_c_macros;\n");
    rust.push_str(
        &emit_support_with_bindings(&session, NAMES, &BindingCatalog::default()).unwrap(),
    );
    for name in NAMES {
        let emission = emit(&session, name);
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("original macro {name} must emit: {emission:?}");
        };
        rust.push_str(&definition);
    }
    rust.push_str(
        "fn main() {\n    println!(\"{} {}\", ROLE_CAST_THEN_SIZE!(i32).get(), ROLE_SIZE_THEN_CAST!(u8, 257_i32).get());\n}\n",
    );
    let profile = frontend.profile();
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header,
        r#"
#include <stdio.h>
int main(void) {
    printf("%lu %lu\n", (unsigned long) ROLE_CAST_THEN_SIZE(int), (unsigned long) ROLE_SIZE_THEN_CAST(unsigned char, 257));
}
"#,
        &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    assert_eq!(original, "7 9\n", "C casts -1 and +257 rather than subtracting from T");
    assert_eq!(rust_oracle::run_rust(&rust), original);
}
