//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Prove caller input tags preserve C types after bindgen erases constant provenance.
//!
//! Rust storage alone cannot distinguish a bindgen u32 constant originating from
//! C int from a true C unsigned int. Explicit tags restore that information;
//! native usize inputs instead have the inspected size_t interpretation. The
//! same generated paths retain plain-char signedness and rank-sensitive
//! conversions across native LP64 and LLP64 profiles.

/// Collect the same fresh binding metadata used by the production build.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Compile and execute the original C replacement definitions.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile positive and negative Rust consumers with bounded resource use.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, FrontendOutput, IntegerKind, MacroScanner,
    generate_with_bindings, inspect, validate_support_profile,
};
use std::path::PathBuf;
use std::sync::Mutex;

/// Keep this process's native and cross-profile libclang ownership mutually exclusive.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Select target-dependent operations whose types, arithmetic, and pointer storage must retain
/// the inspected C profile without a handwritten Rust translation.
const PROFILE_MACROS: &[&str] = &[
    "PROFILE_CHAR",
    "PROFILE_CHAR_ADD",
    "PROFILE_CHAR_CALL",
    "PROFILE_CHAR_CALLBACK_GET",
    "PROFILE_CHAR_CALLBACK_CALL",
    "PROFILE_LONG_UINT",
    "PROFILE_ULONG_LLONG",
    "PROFILE_SIZE",
    "PROFILE_SIZE_OF",
    "PROFILE_ALIGNMENT",
    "PROFILE_ALIGNMENT_LONG_LONG",
    "PROFILE_ALIGNMENT_LONG_LONG_ARRAY",
    "PROFILE_ALIGNMENT_LONG_LONG_NESTED_ARRAY",
    "PROFILE_ALIGNMENT_DOUBLE",
    "PROFILE_ALIGNMENT_DOUBLE_ARRAY",
    "PROFILE_ALIGNMENT_DOUBLE_NESTED_ARRAY",
    "PROFILE_POINTER_DIFF",
    "PROFILE_CHAR_STORE",
    "PROFILE_LONG_STORE",
    "PROFILE_SIZE_STORE",
    "PROFILE_RECORD_CHAR",
    "PROFILE_RECORD_CHAR_STORE",
    "PROFILE_RECORD_LONG",
    "PROFILE_RECORD_LONG_STORE",
    "PROFILE_RECORD_SIZE",
    "PROFILE_RECORD_SIZE_STORE",
];

/// Generate paired Rust and C support artifacts from fresh aliases, records, and function
/// bindings, preserving compiler metadata rather than reproducing a macro implementation.
fn native_profile_source(scanner: &MacroScanner, frontend: &FrontendOutput) -> (String, String) {
    let profile = frontend.profile();
    let session = AnalysisSession::prepare(scanner, frontend, PROFILE_MACROS).unwrap();
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(profile.header.to_str().unwrap())
        .clang_args(&profile.arguments)
        .allowlist_type("Profile.*")
        .allowlist_function("profile_char_.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .unwrap()
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &profile.target,
    );
    let generated = generate_with_bindings(&session, PROFILE_MACROS, &catalog).unwrap();
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut source = format!(
        "#![allow(non_snake_case, non_camel_case_types, dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}",
        generated.support.rust,
    );
    for emission in generated.macros {
        let EmissionStatus::Emitted { rust, .. } = emission.status else {
            panic!("every native-profile fixture must emit: {emission:?}");
        };
        source.push_str(&rust);
    }
    assert!(
        generated.support.c_source.contains("profile_char_identity"),
        "plain-char direct calls must use the original compiler's native ABI"
    );
    (source, generated.support.c_source)
}

/// Execute the same C macros under each real native plain-char compiler mode and compare
/// promotions, rank-dependent results, preferred alignment, pointer differences, and mutations
/// with emitted Rust.
#[test]
fn native_plain_char_profiles_match_original_c_scalar_and_pointer_semantics() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/native_profile.h");
    for (flag, signed) in [("-fsigned-char", true), ("-funsigned-char", false)] {
        let mut arguments = oracle::native_arguments();
        arguments.extend(["-std=c17".into(), flag.into()]);
        let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
        let profile = frontend.profile();
        assert_eq!(profile.target.char_is_signed, signed);
        assert_eq!(validate_support_profile(profile), Ok(()));
        let (rust, native) = native_profile_source(&scanner, &frontend);
        let rust = rust + include_str!("fixtures/native_profile.rs");
        let functions = include_str!("fixtures/native_profile_functions.c");
        let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let original = oracle::run_c(
            &profile.compiler.executable,
            &header,
            &format!("{functions}\n{}", include_str!("fixtures/native_profile.c")),
            &arguments,
            true,
        );
        let generated = rust_oracle::run_rust_linked_with_cfg(
            &rust,
            &profile.compiler.executable,
            &header,
            &format!("{functions}\n{native}"),
            &arguments,
            &pgrx_c_macros::support_rust_cfg(profile).unwrap(),
        );
        assert_eq!(original.lines().count(), 1563, "complete original-C native-profile corpus");
        assert_eq!(generated.lines().count(), original.lines().count());
        for (index, (rust, c)) in generated.lines().zip(original.lines()).enumerate() {
            assert_eq!(
                rust, c,
                "original C and emitted Rust differ under {flag} at record {index}"
            );
        }
    }
}

/// Preserve genuine Clang cross-target integer ranks, preferred scalar/array alignment, and
/// emitted result types without requiring a foreign linker, system headers, or execution.
#[test]
fn cross_target_profiles_preserve_c_integer_ranks_sizes_and_pointer_differences() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/native_profile.h");
    for (target, pointer_bits, long_bytes, signed_char, size_kind, long_uint, ulong_llong) in [
        (
            "x86_64-unknown-linux-gnu",
            64,
            8,
            true,
            IntegerKind::UnsignedLong,
            "CLong",
            "CUnsignedLongLong",
        ),
        (
            "aarch64-unknown-linux-gnu",
            64,
            8,
            false,
            IntegerKind::UnsignedLong,
            "CLong",
            "CUnsignedLongLong",
        ),
        (
            "x86_64-pc-windows-msvc",
            64,
            4,
            true,
            IntegerKind::UnsignedLongLong,
            "CUnsignedLong",
            "CLongLong",
        ),
        (
            "i686-unknown-linux-gnu",
            32,
            4,
            true,
            IntegerKind::UnsignedInt,
            "CUnsignedLong",
            "CLongLong",
        ),
        (
            "i686-pc-windows-msvc",
            32,
            4,
            true,
            IntegerKind::UnsignedInt,
            "CUnsignedLong",
            "CLongLong",
        ),
        (
            "armv7-unknown-linux-gnueabihf",
            32,
            4,
            false,
            IntegerKind::UnsignedInt,
            "CUnsignedLong",
            "CLongLong",
        ),
        (
            "powerpc64-unknown-linux-gnu",
            64,
            8,
            false,
            IntegerKind::UnsignedLong,
            "CLong",
            "CUnsignedLongLong",
        ),
        (
            "i686-unknown-openbsd",
            32,
            4,
            true,
            IntegerKind::UnsignedLong,
            "CUnsignedLong",
            "CLongLong",
        ),
    ] {
        let frontend =
            inspect(&scanner, &header, &[format!("--target={target}"), "-std=c17".into()], None)
                .unwrap_or_else(|error| {
                    panic!("inspect actual {target} compiler profile: {error}")
                });
        let profile = frontend.profile();
        assert_eq!(profile.target.pointer_bits, pointer_bits);
        assert_eq!(profile.target.integers[&IntegerKind::Long].bits, long_bytes * 8);
        assert_eq!(profile.target.char_is_signed, signed_char);
        assert_eq!(profile.target.size_type, size_kind);
        assert_eq!(validate_support_profile(profile), Ok(()));
        let c_long_uint = if long_bytes == 4 { "unsigned long" } else { "long" };
        let c_ulong_llong = if long_bytes == 4 { "long long" } else { "unsigned long long" };
        let (c_size, c_diff, ptrdiff_kind) = match size_kind {
            IntegerKind::UnsignedInt => ("unsigned int", "int", IntegerKind::Int),
            IntegerKind::UnsignedLong => ("unsigned long", "long", IntegerKind::Long),
            IntegerKind::UnsignedLongLong => {
                ("unsigned long long", "long long", IntegerKind::LongLong)
            }
            _ => unreachable!("the fixture table selects only pointer-sized integer families"),
        };
        assert_eq!(profile.target.ptrdiff_type, ptrdiff_kind);
        let pointer_bytes = pointer_bits / 8;
        let mut original = format!(
            "_Static_assert(sizeof(void *) == {pointer_bytes} && sizeof(long) == {long_bytes}, \"original target widths\");\n\
             _Static_assert(((char)-1 < 0) == {}, \"original plain-char signedness\");\n\
             _Static_assert(_Generic(PROFILE_CHAR_CALL(0), char: 1, default: 0), \"original direct char result identity\");\n\
             _Static_assert(_Generic(PROFILE_CHAR_CALLBACK_CALL(PROFILE_CHAR_CALLBACK_GET(), 0), char: 1, default: 0), \"original callback char result identity\");\n\
             _Static_assert(_Generic(PROFILE_LONG_UINT(-2), {c_long_uint}: 1, default: 0), \"original mixed long/unsigned-int rank\");\n\
             _Static_assert(_Generic(PROFILE_ULONG_LLONG(-1), {c_ulong_llong}: 1, default: 0), \"original mixed unsigned-long/long-long rank\");\n\
             _Static_assert(_Generic(PROFILE_SIZE_OF(0), {c_size}: 1, default: 0), \"original sizeof identity\");\n\
             _Static_assert(_Generic(PROFILE_POINTER_DIFF((int *)0, (int *)0), {c_diff}: 1, default: 0), \"original pointer-difference identity\");\n",
            u8::from(signed_char),
        );
        for (name, marker, c_type, array_type, nested_type) in [
            (
                "LONG_LONG",
                "CLongLong",
                "long long",
                "ProfileLongLongArray",
                "ProfileLongLongNestedArray",
            ),
            ("DOUBLE", "CFloat64", "double", "ProfileDoubleArray", "ProfileDoubleNestedArray"),
        ] {
            let observed = profile.target.preferred_alignments[marker];
            original.push_str(&format!(
                "_Static_assert(PROFILE_ALIGNMENT_{name}() == {} && PROFILE_ALIGNMENT({c_type}) == {}, \"original scalar preferred alignment\");\n\
                 _Static_assert(PROFILE_ALIGNMENT_{name}_ARRAY() == {} && PROFILE_ALIGNMENT({array_type}) == {}, \"original array preferred alignment\");\n\
                 _Static_assert(PROFILE_ALIGNMENT_{name}_NESTED_ARRAY() == {} && PROFILE_ALIGNMENT({nested_type}) == {}, \"original nested-array preferred alignment\");\n",
                observed.scalar,
                observed.scalar,
                observed.array,
                observed.array,
                observed.array,
                observed.array,
            ));
        }
        let (mut rust, native) = native_profile_source(&scanner, &frontend);
        assert!(
            oracle::run_c(
                &profile.compiler.executable,
                &header,
                &format!(
                    "{original}\n{}\n{native}",
                    include_str!("fixtures/native_profile_functions.c"),
                ),
                &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
                false,
            )
            .is_empty()
        );
        rust.push_str(&format!(
            "use __pgrx_c_macros::*;\n\
             const _: () = assert!(core::mem::size_of::<<CLong as CInteger>::Repr>() == {long_bytes});\n\
             const _: () = assert!(core::mem::size_of::<<CSize as CInteger>::Repr>() == {pointer_bytes});\n\
             const _: () = assert!(CChar::SIGNED == {signed_char});\n\
             /// Instantiate the actual generated operation types without executing foreign code.\n\
             /// # Safety\n\
             /// The pointer must belong to a live initialized i32 allocation, and native\n\
             /// functions must come from the same original C profile.\n\
             unsafe fn prove_profile(pointer: *const i32) {{\n\
               let _: CValue<{long_uint}> = PROFILE_LONG_UINT!(-2_i32).into_c_value();\n\
               let _: CValue<{ulong_llong}> = PROFILE_ULONG_LLONG!(-1_i32).into_c_value();\n\
               let _: CValue<CSize> = PROFILE_SIZE_OF!(0_i32).into_c_value();\n\
               let _: CValue<CSize> = PROFILE_SIZE!(-1_i32).into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT!(CValue<CLongLong>).into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT!([CValue<CLongLong>; 2]).into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT!([[CValue<CLongLong>; 2]; 2]).into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT!(f64).into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT!([f64; 2]).into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT!([[f64; 2]; 2]).into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT_LONG_LONG!().into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT_LONG_LONG_ARRAY!().into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT_LONG_LONG_NESTED_ARRAY!().into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT_DOUBLE!().into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT_DOUBLE_ARRAY!().into_c_value();\n\
               let _: CValue<CSize> = PROFILE_ALIGNMENT_DOUBLE_NESTED_ARRAY!().into_c_value();\n\
               // SAFETY: The fixture's original C functions accept every char bit pattern,\n\
               // and the factory returns its same-profile non-null C callback.\n\
               unsafe {{\n\
                 let _: CValue<CChar> = PROFILE_CHAR_CALL!(255_i32).into_c_value();\n\
                 let callback = PROFILE_CHAR_CALLBACK_GET!();\n\
                 let _: CValue<CChar> = PROFILE_CHAR_CALLBACK_CALL!(callback, 255_i32).into_c_value();\n\
               }}\n\
               // SAFETY: Both operands designate the caller's same live allocation.\n\
               let _: CValue<CPtrDiff> = unsafe {{ PROFILE_POINTER_DIFF!(pointer, pointer) }}.into_c_value();\n\
             }}\n"
        ));
        rust_oracle::check_rust_with_cfg(
            &rust,
            &pgrx_c_macros::support_rust_cfg(profile).unwrap(),
            target,
        );
    }
}

/// Supply the PostgreSQL-only classifier required by the shared binding collector.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Compare tagged bindgen constants and native size_t inputs with independently compiled C.
#[test]
fn original_c_input_types_survive_bindgen_constant_storage_and_native_sizes() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = root.join("tests/fixtures/input_types.h");
    let support = root.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c11".into());
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let names = ["INPUT_MIN", "INPUT_WIDTH"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let native = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .clang_args(&arguments)
        .allowlist_var("INPUT_INT")
        .generate_comments(false)
        .layout_tests(false)
        .generate()
        .unwrap()
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&native).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let artifact = generate_with_bindings(&session, &names, &catalog).unwrap();
    let mut emitted = artifact.support.rust;
    for emission in artifact.macros {
        let EmissionStatus::Emitted { rust, .. } = emission.status else {
            panic!("input fixture must emit: {emission:?}");
        };
        emitted.push_str(&rust);
    }
    let preamble = format!("#[path={support:?}] pub mod __pgrx_c_macros; {native} {emitted}");
    let actual = rust_oracle::run_rust_with_cfg(
        &format!(
            r#"{preamble}
use __pgrx_c_macros::{{CValue, CInt}};
fn main() {{
    let _: u32 = INPUT_INT;
    let signed = INPUT_MIN!(CValue::<CInt>::new(INPUT_INT as i32), -1_i32);
    let unsigned = INPUT_MIN!(INPUT_INT, -1_i32);
    let sizes = INPUT_MIN!(usize::MAX, 1_usize);
    println!("{{}} {{}} {{}} {{}}", signed.get(), unsigned.get(), sizes.get(), INPUT_WIDTH!(usize::MAX).get());
}}"#,
        ),
        &pgrx_c_macros::support_rust_cfg(frontend.profile()).unwrap(),
    );
    let expected = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        "#include <stdio.h>\nint main(void) { printf(\"%d %u %llu %zu\\n\", INPUT_MIN(INPUT_INT, -1), INPUT_MIN((unsigned int)INPUT_INT, -1), (unsigned long long)INPUT_MIN((__SIZE_TYPE__)-1, (__SIZE_TYPE__)1), INPUT_WIDTH((__SIZE_TYPE__)-1)); }\n",
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    assert_eq!(actual, expected, "caller input interpretation must agree with original C");
    for literal in ["1_u64", "1_i64"] {
        let diagnostics = rust_oracle::reject_rust(&format!(
            "{preamble} fn main() {{ let _ = INPUT_MIN!({literal}, 1_i32); }}",
        ));
        assert!(diagnostics.contains("CValue::<CLong>"), "{diagnostics}");
        assert!(diagnostics.contains("CUnsignedLongLong"), "{diagnostics}");
    }
}
