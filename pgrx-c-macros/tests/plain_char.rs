//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare every plain-char byte pattern, promotion, and datum conversion with native C.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]

/// Execute the original C header under the inspected compiler profile.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated Rust under the actual native target.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, MacroScanner, PostgresConfig,
    generate_with_bindings, inspect, support_rust_cfg, validate_support_profile,
};
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize the safe libclang owner across these native-profile tests.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());
/// Generic C operations exercising native char storage and integer conversions.
const NAMES: &[&str] = &["PLAIN_CHAR", "PLAIN_PROMOTE", "PLAIN_WIDEN"];

/// Print identical observations from generated generic char operations for all 256 bytes.
const RUST_RECORDING: &str = r#"
use __pgrx_c_macros::*;
fn main() {
    for byte in 0..=u8::MAX {
        let plain: CValue<CChar> = PLAIN_CHAR!(byte).into_value();
        let promoted: CValue<CInt> = PLAIN_PROMOTE!(plain).into_value();
        let widened: CValue<CUnsignedLongLong> = PLAIN_WIDEN!(plain).into_value();
        let roundtrip: CValue<CChar> = PLAIN_CHAR!(widened).into_value();
        let signed: CValue<CSignedChar> = value(byte as i8);
        let unsigned: CValue<CUnsignedChar> = value(byte);
        println!("{} {} {} {} {} {} {}", byte, plain.get() as i32,
            promoted.get(), widened.get(), roundtrip.get() as i32,
            promote(signed).get(), promote(unsigned).get());
    }
}
"#;

/// Require compiler-owned identities independently of matching numeric output.
const C_RECORDING: &str = r#"
#include <stdio.h>
_Static_assert(_Generic(PLAIN_CHAR(0), char: 1, default: 0), "plain char identity");
_Static_assert(_Generic(PLAIN_PROMOTE(0), int: 1, default: 0), "char promotes to int");
_Static_assert(_Generic(PLAIN_WIDEN(0), unsigned long long: 1, default: 0), "wide identity");
int main(void) {
    for (unsigned int byte = 0; byte <= 255; ++byte) {
        char plain = PLAIN_CHAR(byte);
        unsigned long long widened = PLAIN_WIDEN(plain);
        printf("%u %d %d %llu %d %d %d\n", byte, (int)plain,
            PLAIN_PROMOTE(plain), widened, (int)PLAIN_CHAR(widened),
            +(signed char)byte, +(unsigned char)byte);
    }
}
"#;

/// Compare generic generated conversion semantics against installed PostgreSQL's actual helpers.
const POSTGRES_RECORDING: &str = r#"
#include <stdio.h>
#undef printf
_Static_assert(_Generic(DatumGetChar((Datum)0), char: 1, default: 0), "DatumGetChar identity");
_Static_assert(_Generic(CharGetDatum((char)0), Datum: 1, default: 0), "CharGetDatum identity");
int main(void) {
    for (unsigned int byte = 0; byte <= 255; ++byte) {
        char plain = (char)byte;
        Datum datum = CharGetDatum(plain);
        printf("%u %d %d %llu %d %d %d\n", byte, (int)plain,
            +plain, (unsigned long long)datum, (int)DatumGetChar(datum),
            +(signed char)byte, +(unsigned char)byte);
    }
}
"#;

/// Locate the original C macro fixture used by both language consumers.
fn header() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/plain_char.h")
}

/// Emit actual char operations with the inspected native support assertions.
fn generated_source(scanner: &MacroScanner, arguments: &[String]) -> String {
    let frontend = inspect(scanner, &header(), arguments, None).unwrap();
    validate_support_profile(frontend.profile()).unwrap();
    let session = AnalysisSession::prepare(scanner, &frontend, NAMES).unwrap();
    let bindings = BindingCatalog::default();
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let generation = generate_with_bindings(&session, NAMES, &bindings).unwrap();
    let artifact = generation.support;
    assert!(artifact.c_source.is_empty(), "plain-char conversions require no native adapter");
    let mut source = format!("#[path = {support:?}] pub mod __pgrx_c_macros;\n{}", artifact.rust);
    for emission in generation.macros {
        let EmissionStatus::Emitted { rust, .. } = emission.status else {
            panic!("native char operation must emit: {emission:?}");
        };
        source.push_str(&rust);
    }
    source
}

/// Exercise actual native storage, promotion, widening, and narrowing for every byte pattern.
#[test]
fn all_plain_char_bytes_match_native_c() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c17".into());
    let frontend = inspect(&scanner, &header(), &arguments, None).unwrap();
    let source = generated_source(&scanner, &arguments);
    let generated = rust_oracle::run_rust_with_cfg(
        &format!("{source}\n{RUST_RECORDING}"),
        &support_rust_cfg(frontend.profile()).unwrap(),
    );
    let original = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header(),
        C_RECORDING,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    assert_eq!(original.lines().count(), 256);
    assert_eq!(generated, original);
    for family in ["CSignedChar", "CUnsignedChar"] {
        let rejected = rust_oracle::reject_rust(&format!(
            "{source}\nfn main() {{ let plain = __pgrx_c_macros::expression::Pointer::<__pgrx_c_macros::CChar>::new(core::ptr::null_mut()); let other = __pgrx_c_macros::expression::Pointer::<__pgrx_c_macros::{family}>::new(core::ptr::null_mut()); let _ = __pgrx_c_macros::expression::eq(plain, other); }}\n"
        ));
        assert!(rejected.contains("Compatible"), "{family}: {rejected}");
    }
}

/// Preserve every plain-char byte conversion when the C compiler overrides Rust's default
/// signedness; nominal char storage and arithmetic follow the inspected C profile.
#[test]
fn non_default_plain_char_profile_matches_native_c() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let signed = (core::ffi::c_char::MIN as i16) < 0;
    let mut arguments = oracle::native_arguments();
    arguments.push(if signed { "-funsigned-char" } else { "-fsigned-char" }.into());
    let frontend = inspect(&scanner, &header(), &arguments, None).unwrap();
    assert_eq!(frontend.profile().target.char_is_signed, !signed);
    validate_support_profile(frontend.profile()).unwrap();
    let source = generated_source(&scanner, &arguments);
    let generated = rust_oracle::run_rust_with_cfg(
        &format!("{source}\n{RUST_RECORDING}"),
        &support_rust_cfg(frontend.profile()).unwrap(),
    );
    let original = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header(),
        C_RECORDING,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    assert_eq!(original.lines().count(), 256);
    assert_eq!(generated, original);
}

/// Keep exact installed PG17/18 char-helper coverage independent of generated test expressions.
#[test]
#[ignore = "requires configured native PostgreSQL 17 and 18 installations"]
fn all_plain_char_bytes_match_installed_postgres_helpers() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    for major in [17, 18] {
        let postgres = PostgresConfig::resolve(&format!("pg{major}")).unwrap();
        let frontend = postgres.inspect(&scanner, None, &["-O2".into()], None).unwrap();
        validate_support_profile(frontend.profile()).unwrap();
        let source = generated_source(&scanner, &frontend.profile().arguments);
        let generated = rust_oracle::run_rust_with_cfg(
            &format!("{source}\n{RUST_RECORDING}"),
            &support_rust_cfg(frontend.profile()).unwrap(),
        );
        let profile = frontend.profile();
        let original = oracle::run_c(
            &profile.compiler.executable,
            &profile.header,
            POSTGRES_RECORDING,
            &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            true,
        );
        assert_eq!(original.lines().count(), 256, "PG{major}");
        assert_eq!(generated, original, "PG{major}");
    }
}
