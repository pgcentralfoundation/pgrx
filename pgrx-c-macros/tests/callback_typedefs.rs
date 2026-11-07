//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Explicit C callback typedefs retain nominal identity when raw Rust storage is ambiguous.

/// Recover actual fresh binding facts with the production collector.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Execute independent original-header observations.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile positive and negative consumers of the actual generated support.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AliasBinding, AnalysisSession, EmissionStatus, MacroScanner, RustBindingType,
    generate_with_bindings, inspect,
};
use quote::ToTokens;
use std::path::PathBuf;
use std::sync::Mutex;
use syn::visit_mut::VisitMut;

/// Serialize the libclang owner because its safe wrapper admits one live instance per process.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Supply the production collector's PostgreSQL OID classification hook.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Match the guarded binding build's nested C ABI rewrite before collecting storage witnesses.
struct UnwindAbi;

impl VisitMut for UnwindAbi {
    /// Rewrite ABI syntax, leaving each parameter and result representation unchanged.
    fn visit_abi_mut(&mut self, abi: &mut syn::Abi) {
        if abi.name.as_ref().is_some_and(|name| name.value() == "C") {
            abi.name = Some(syn::parse_quote!("C-unwind"));
        }
    }
}

/// Compare explicit typedef callback use with original C and reject ABI or identity substitutions.
#[test]
fn explicit_typedefs_preserve_exact_storage_and_operand_evaluation() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/callback_typedefs.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c17".into());
    let names = [
        "EXPLICIT_APPLY",
        "EXPLICIT_TWICE",
        "EXPLICIT_LAZY",
        "EXPLICIT_PRESENT",
        "EXPLICIT_LONG",
        "EXPLICIT_WIDE",
    ];
    let frontend =
        inspect(&scanner, &header, &arguments, None).expect("inspect original callback header");
    let session =
        AnalysisSession::prepare(&scanner, &frontend, &names).expect("prepare callback macros");
    let raw = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .clang_args(&arguments)
        .layout_tests(false)
        .generate_comments(false)
        .use_core()
        .generate()
        .expect("generate fresh callback bindings")
        .to_string();
    let mut parsed = syn::parse_file(&raw).expect("parse actual bindings");
    UnwindAbi.visit_file_mut(&mut parsed);
    let mut catalog = binding_symbols::collect_bindings(
        &parsed,
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    // An unrelated unresolved edge cannot establish a global raw pointer identity.
    catalog.types.insert(
        "UnresolvedProfileAlias".into(),
        AliasBinding {
            path: vec!["UnresolvedProfileAlias".into()],
            target: RustBindingType::Named { path: vec!["MissingNativeStorage".into()] },
        },
    );
    let generated = generate_with_bindings(&session, &names, &catalog)
        .expect("generate validated explicit callback capabilities");
    assert!(!generated.support.rust.contains("::NativeType for"));
    assert!(!generated.support.rust.contains("transmute"));
    let runtime = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut base = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={runtime:?}] pub mod __pgrx_c_macros;\n{}\n{}",
        parsed.to_token_stream(),
        generated.support.rust,
    );
    for emission in &generated.macros {
        let EmissionStatus::Emitted { rust, .. } = &emission.status else {
            panic!("validated callback macros must emit: {emission:?}");
        };
        base.push_str(rust);
    }
    let consumer = r#"
unsafe extern "C-unwind" fn target(value: i32) -> i32 { value + 7 }
unsafe extern "C-unwind" fn long_target(value: core::ffi::c_long) -> core::ffi::c_long { value + 3 }
unsafe extern "C-unwind" fn wide_target(value: core::ffi::c_longlong) -> core::ffi::c_longlong { value + 5 }
fn main() {
    let callback: ExplicitCallback = Some(target);
    let mut evaluations = 0;
    // SAFETY: Each callback has the exact inspected ABI and operates only on
    // initialized scalars. No backend state, unwinding or nonlocal jump is used.
    unsafe {
        let result = EXPLICIT_TWICE!({ evaluations += 1; __pgrx_c_callbacks::ExplicitCallback::new(callback) }, 6_i32).get();
        println!("{result} {evaluations}");
        let lazy = EXPLICIT_LAZY!({ evaluations += 1; __pgrx_c_callbacks::ExplicitCallback::new(callback) }, 6_i32, 0_i32).get();
        println!("{lazy} {evaluations}");
        println!("{} {}", EXPLICIT_PRESENT!(__pgrx_c_callbacks::ExplicitCallback::new(callback)).get() as u8, EXPLICIT_PRESENT!(__pgrx_c_callbacks::ExplicitCallback::new(None)).get() as u8);
        let long: ExplicitLongCallback = Some(long_target);
        let wide: ExplicitWideCallback = Some(wide_target);
        println!("{} {}", EXPLICIT_LONG!(__pgrx_c_callbacks::ExplicitLongCallback::new(long), 11_i32).get(), EXPLICIT_WIDE!(__pgrx_c_callbacks::ExplicitWideCallback::new(wide), 11_i32).get());
    }
}
"#;
    let original = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        r#"
#include <stdio.h>
static unsigned int evaluations;
static int target(int value) { return value + 7; }
static long long_target(long value) { return value + 3; }
static long long wide_target(long long value) { return value + 5; }
static ExplicitCallback record_callback(void) { evaluations++; return target; }
int main(void) {
    int result = EXPLICIT_TWICE(record_callback(), 6);
    printf("%d %u\n", result, evaluations);
    int lazy = EXPLICIT_LAZY(record_callback(), 6, 0);
    printf("%d %u\n", lazy, evaluations);
    printf("%d %d\n", EXPLICIT_PRESENT(target), EXPLICIT_PRESENT(0));
    printf("%ld %lld\n", EXPLICIT_LONG(long_target, 11), EXPLICIT_WIDE(wide_target, 11));
    return 0;
}
"#,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    let actual = rust_oracle::run_rust(&format!("{base}\n{consumer}"));
    assert_eq!(actual, original, "explicit identities retain C callback effects and null values");
    assert_eq!(actual.lines().count(), 4);

    for invalid in [
        "unsafe extern \"C\" fn callback(value:i32)->i32{value} fn main(){let _=__pgrx_c_callbacks::ExplicitCallback::new(Some(callback));}",
        "unsafe extern \"C-unwind\" fn callback()->i32{7} fn main(){let _=__pgrx_c_callbacks::ExplicitCallback::new(Some(callback));}",
        "fn main(){let callback:ExplicitCallback=None; unsafe{let _=EXPLICIT_APPLY!(callback,1_i32);}}",
        "fn main(){let callback:ExplicitLongCallback=None; unsafe{let _=EXPLICIT_WIDE!(__pgrx_c_callbacks::ExplicitLongCallback::new(callback),1_i32);}}",
    ] {
        let diagnostic = rust_oracle::reject_rust(&format!("{base}\n{invalid}"));
        assert!(
            diagnostic.contains("mismatched types")
                || diagnostic.contains("CastTo")
                || diagnostic.contains("IntoExpression")
                || diagnostic.contains("NativeRecord"),
            "the exact callback ABI and C identity must be checked: {diagnostic}"
        );
    }
}

/// Namespace fresh Rust callback storage without changing its compiler-owned C identity or
/// adding the private binding module to the established public callback typedef path.
#[test]
fn namespaced_typedefs_keep_public_identity_and_exact_native_storage() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/callback_typedefs.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c17".into());
    let names = ["EXPLICIT_APPLY", "EXPLICIT_PRESENT"];
    let frontend = inspect(&scanner, &header, &arguments, None)
        .expect("inspect original namespaced callback header");
    let session = AnalysisSession::prepare(&scanner, &frontend, &names)
        .expect("prepare namespaced callback macros");
    let raw = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .clang_args(&arguments)
        .layout_tests(false)
        .generate_comments(false)
        .use_core()
        .generate()
        .expect("generate fresh namespaced callback bindings")
        .to_string();
    let mut parsed = syn::parse_file(&raw).expect("parse namespaced callback bindings");
    UnwindAbi.visit_file_mut(&mut parsed);
    let catalog = binding_symbols::collect_bindings_at(
        &parsed,
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
        &["__pgrx_c_bindings".into()],
    );
    assert_eq!(
        catalog.type_alias("ExplicitCallback").unwrap().path,
        ["__pgrx_c_bindings", "ExplicitCallback"]
    );
    let generated = generate_with_bindings(&session, &names, &catalog)
        .expect("generate namespaced callback capabilities");
    let runtime = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut base = format!(
        r#"#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]
/// Actual runtime marker implementation used by emitted callback capabilities.
#[path={runtime:?}] pub mod __pgrx_c_macros;
/// Raw binding declarations are kept private to the defining crate.
mod raw_bindings {{ {} }}
/// Public storage namespace mirrors production without exposing root typedef aliases.
pub mod __pgrx_c_bindings {{ pub use crate::raw_bindings::*; }}
/// Deliberately incompatible root storage cannot satisfy a native callback signature.
pub type ExplicitCallback = usize;
{}
"#,
        parsed.to_token_stream(),
        generated.support.rust,
    );
    for emission in &generated.macros {
        let EmissionStatus::Emitted { rust, .. } = &emission.status else {
            panic!("namespaced callback macros must emit: {emission:?}");
        };
        base.push_str(rust);
    }
    let consumer = r#"
/// Return an initialized scalar through the inspected native callback ABI.
///
/// # Safety
/// The caller must invoke this function using its exact C-unwind signature.
unsafe extern "C-unwind" fn callback(value: i32) -> i32 { value }
/// Require the public semantic typedef and namespaced native storage to remain distinct.
fn main() {
let native: __pgrx_c_bindings::ExplicitCallback = Some(callback);
let typed = __pgrx_c_callbacks::ExplicitCallback::new(native);
let root_shadow: ExplicitCallback = 37;
assert_eq!(root_shadow, 37);
// SAFETY: The callback has the exact inspected ABI, reads only initialized scalars, and
// neither unwinds nor touches backend state. The absent callback is only compared to null.
unsafe {
    let result = EXPLICIT_APPLY!(typed, 41_i32).get();
    let present = EXPLICIT_PRESENT!(typed).get() as u8;
    let absent = EXPLICIT_PRESENT!(__pgrx_c_callbacks::ExplicitCallback::new(None)).get() as u8;
    println!("{result} {present} {absent}");
}
}
"#;
    let original = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        r#"
#include <stdio.h>
static int callback(int value) { return value; }
int main(void) {
printf("%d %d %d\n", EXPLICIT_APPLY(callback, 41), EXPLICIT_PRESENT(callback), EXPLICIT_PRESENT(0));
return 0;
}
"#,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    let configurations = pgrx_c_macros::support_rust_cfg(frontend.profile())
        .expect("validate callback oracle runtime profile");
    let actual = rust_oracle::run_rust_with_cfg(&format!("{base}\n{consumer}"), &configurations);
    assert_eq!(
        actual, original,
        "callback namespaces must preserve original C type and value behavior"
    );
    assert_eq!(actual, "41 1 0\n");
    let wrong_abi = r#"
/// A C-only pointer cannot replace the inspected C-unwind callback storage.
unsafe extern "C" fn callback(value: i32) -> i32 { value }
/// Require the explicit semantic typedef to reject a different native ABI.
fn main() { let _ = __pgrx_c_callbacks::ExplicitCallback::new(Some(callback)); }
"#;
    let diagnostic = rust_oracle::reject_rust(&format!("{base}\n{wrong_abi}"));
    assert!(diagnostic.contains("mismatched types"), "{diagnostic}");
    let hidden_path = r#"
/// The private storage namespace is not part of the public callback typedef API.
fn main() {
let _ = __pgrx_c_callbacks::__pgrx_c_bindings::ExplicitCallback::new(None);
}
"#;
    let diagnostic = rust_oracle::reject_rust(&format!("{base}\n{hidden_path}"));
    assert!(
        diagnostic.contains("could not find") && diagnostic.contains("__pgrx_c_bindings"),
        "the public callback typedef path cannot acquire the storage prefix: {diagnostic}"
    );
}
