//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Call functions whose prototypes need native transport only through generated adapters.
//!
//! A prototype that passes or returns plain `char`, an enum or a record by value has
//! no Rust binding storage that proves its call ABI. Every entry point must refuse a
//! direct call through the caller's binding, even when that storage looks compatible.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]

/// Compile generated adapters with the original C header under the inspected profile.
#[path = "support/oracle.rs"]
mod oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, FrontendOutput, FunctionBinding, MacroScanner,
    RustBindingType, SkipReasonCode, emit, emit_batch_with_bindings, emit_with_bindings,
    generate_with_bindings, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize the safe libclang owner across these tests.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Locate the native-transport fixture independently of the invocation directory.
fn header() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/native_transport.h")
}

/// Inspect the native-transport fixture.
fn frontend(scanner: &MacroScanner) -> FrontendOutput {
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c17".into());
    inspect(scanner, &header(), &arguments, None).unwrap()
}

/// Bind every fixture function with the shape of bindgen's static-function wrappers.
///
/// `char` uses Rust's `c_char`. Enum values use the integer storage of their
/// compatible type, `unsigned int`, which the lowering would otherwise accept, so
/// only the native-transport rule stands between a macro and a direct call.
fn bindings() -> BindingCatalog {
    use RustBindingType::{Integer, Named, NativeChar};
    let int = || Integer { signed: true, bits: 32 };
    let color = || Integer { signed: false, bits: 32 };
    let pair = || Named { path: vec!["transport_pair".into()] };
    let mut bindings = BindingCatalog::default();
    for (name, parameters, result) in [
        ("transport_widen", vec![NativeChar], int()),
        ("transport_inline_widen", vec![NativeChar], int()),
        ("transport_color_value", vec![color()], int()),
        ("transport_color_pick", vec![int()], color()),
        ("transport_pair_sum", vec![pair()], int()),
    ] {
        bindings.functions.insert(
            name.into(),
            FunctionBinding {
                path: vec![name.into()],
                parameters,
                result,
                abi: "C".into(),
                variadic: false,
                link_name: format!("{name}__pgrx_cshim"),
                guarded: false,
                uses_cshim: true,
            },
        );
    }
    bindings
}

/// Refuse a call whose required native-transport adapter was rejected, even when a binding exists.
///
/// The binding's `c_char` storage cannot prove the callee's extension attributes. That is
/// why plain-char calls go through generated C; falling back to the binding would skip it.
#[test]
fn rejected_native_transport_does_not_fall_back_to_the_binding() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = frontend(&scanner);
    let names = ["TRANSPORT_WIDEN"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let generation = generate_with_bindings(&session, &names, &bindings()).unwrap();
    let [emission] = generation.macros.as_slice() else { panic!("one selected macro") };
    let EmissionStatus::Skipped { reason } = &emission.status else {
        panic!("a rejected native-transport adapter must not fall back to the binding")
    };
    assert!(reason.message.contains("static inline definition"), "{reason:?}");
}

/// Refuse native-transport calls from single-macro emission, which generates no adapters.
///
/// `emit_with_bindings` returns only Rust, so it has no adapter to call and must not use
/// the binding instead. The reason names the C type and promises no other entry point:
/// batch generation also refuses `transport_widen`, a static function that is not inline.
#[test]
fn single_macro_emission_refuses_native_transport_without_an_adapter() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = frontend(&scanner);
    let calls = [
        ("TRANSPORT_WIDEN", "transport_widen", "char"),
        ("TRANSPORT_INLINE_WIDEN", "transport_inline_widen", "char"),
        ("TRANSPORT_COLOR_VALUE", "transport_color_value", "enum transport_color"),
        ("TRANSPORT_COLOR_PICK", "transport_color_pick", "transport_color_t"),
        ("TRANSPORT_PAIR_SUM", "transport_pair_sum", "struct transport_pair"),
    ];
    let names = calls.map(|(name, ..)| name);
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindings();
    for (name, function, ty) in calls {
        // An empty catalog has no binding to fall back to, but reports the same cause.
        for emission in [emit_with_bindings(&session, name, &bindings), emit(&session, name)] {
            let EmissionStatus::Skipped { reason } = &emission.status else {
                panic!("a call that needs native transport must not use the binding: {emission:?}")
            };
            assert_eq!(reason.code, SkipReasonCode::Call, "{reason:?}");
            assert_eq!(
                reason.message,
                format!(
                    "{function}: C prototype type `{ty}` needs a generated native adapter because binding storage cannot prove its call ABI, and this emission has none"
                ),
            );
        }
    }
}

/// Keep calling native-transport functions through their generated adapters in batch generation.
#[test]
fn batch_generation_calls_native_transport_through_its_adapter() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = frontend(&scanner);
    let calls = [
        ("TRANSPORT_INLINE_WIDEN", "transport_inline_widen"),
        ("TRANSPORT_COLOR_VALUE", "transport_color_value"),
        ("TRANSPORT_COLOR_PICK", "transport_color_pick"),
    ];
    let names = calls.map(|(name, _)| name);
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindings();
    let generation = generate_with_bindings(&session, &names, &bindings).unwrap();
    let batch = emit_batch_with_bindings(&session, &names, &bindings).unwrap();
    // The adapters compile under the inspected profile, which includes their static
    // assertion that each enum's compatible type is the transported integer storage.
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    oracle::run_c(
        &profile.compiler.executable,
        &header(),
        &generation.support.c_source,
        &arguments,
        false,
    );
    for (index, (name, function)) in calls.into_iter().enumerate() {
        // An enum argument is first converted from its compatible integer storage.
        assert!(
            generation.support.c_source.contains(&format!("{{ return ({function})(")),
            "the adapter must call the original C function: {}",
            generation.support.c_source
        );
        for emission in [&generation.macros[index], &batch[index]] {
            assert_eq!(emission.analysis.name, name);
            let EmissionStatus::Emitted { rust, .. } = &emission.status else {
                panic!("the generated adapter makes this call available: {emission:?}")
            };
            assert!(rust.contains("$crate::__pgrx_c_generated::Inline_"), "{rust}");
            assert!(!rust.contains(&format!("$crate::{function}(")), "{rust}");
        }
    }
}
