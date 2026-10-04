//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Validate compiler profiles and the final active preprocessing environment.
//!
//! Independent C probes establish target and declaration facts. Inventory history,
//! restoration, ambiguity, and argument rejection checks ensure later phases use
//! the selected invocation rather than a guessed or partially replayed profile.
//!
//! Inspection also covers unsupported runtime ABIs. Only the native generated
//! consumer below requires the checked Linux/macOS LP64 target family.

/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
mod rust_oracle;

use pgrx_c_macros::{
    ActiveProvenance, AnalysisSession, AnalysisStatus, EmissionStatus, FrontendError, IntegerKind,
    IntegerValue, MacroKind, MacroScanner, SignedOverflow, TypeCategory, emit, inspect,
    support_abi_assertions, validate_support_profile,
};
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
use pgrx_c_macros::{BindingCatalog, emit_support_with_bindings};
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Resolve fixture input relative to the crate, keeping tests independent of the invocation
/// directory.
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

/// Checks that protection codegen profiles preserve macro values and original arguments.
#[test]
fn protection_codegen_profiles_preserve_macro_values_and_original_arguments() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("frontend_environment.h");
    let names = ["FRONT_REDEFINED", "FRONT_RESTORED"];
    // CET is an x86 codegen facility. Other native targets exercise its explicit
    // disabled form; the x86 profile below proves all enabled preprocessing modes.
    let native_cf = if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        "-fcf-protection"
    } else {
        "-fcf-protection=none"
    };
    let mut arguments = oracle::native_arguments();
    arguments.extend(["-fstack-clash-protection".into(), native_cf.into()]);
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    assert!(frontend.profile().unsupported_options.is_empty());
    for argument in &arguments {
        assert!(frontend.profile().arguments.contains(argument), "must retain {argument}");
    }
    #[cfg(all(
        target_pointer_width = "64",
        target_endian = "little",
        any(target_arch = "x86_64", target_arch = "aarch64"),
        any(target_os = "linux", target_os = "macos"),
    ))]
    {
        let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
        let runtime = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs");
        let mut generated = format!("#[path = {runtime:?}] pub mod __pgrx_c_macros;\n");
        generated.push_str(
            &emit_support_with_bindings(&session, &names, &BindingCatalog::default()).unwrap(),
        );
        for name in names {
            let EmissionStatus::Emitted { rust, .. } = emit(&session, name).status else {
                panic!("protection-only flags must permit ordinary C macros");
            };
            generated.push_str(&rust);
        }
        generated.push_str(
            r#"
unsafe extern "C" {
    fn protection_redefined(value: u32) -> u32;
    fn protection_restored(value: u32) -> u32;
}
fn main() {
    for value in [0u32, 1, 100, u32::MAX - 2, u32::MAX] {
        // SAFETY: These fixture C functions use the matching unsigned-int ABI
        // and perform only original, defined unsigned macro arithmetic.
        unsafe {
            assert_eq!(FRONT_REDEFINED!(value).get(), protection_redefined(value));
            assert_eq!(FRONT_RESTORED!(value).get(), protection_restored(value));
        }
    }
}
"#,
        );
        let arguments = frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>();
        assert!(
            rust_oracle::run_rust_linked(
                &generated,
                &frontend.profile().compiler.executable,
                &header,
                "unsigned int protection_redefined(unsigned int value) { return FRONT_REDEFINED(value); }\nunsigned int protection_restored(unsigned int value) { return FRONT_RESTORED(value); }\n",
                &arguments,
            )
            .is_empty()
        );
    }

    let x86_arguments = vec![
        "--target=x86_64-unknown-linux-gnu".into(),
        "-fstack-clash-protection".into(),
        "-fcf-protection".into(),
    ];
    let x86 = inspect(&scanner, &header, &x86_arguments, None).unwrap();
    assert!(x86.profile().unsupported_options.is_empty());
    for argument in &x86_arguments {
        assert!(x86.profile().arguments.contains(argument));
    }
    let session = AnalysisSession::prepare(&scanner, &x86, &names).unwrap();
    assert!(matches!(emit(&session, names[0]).status, EmissionStatus::Emitted { .. }));
    for (mode, expected) in [
        ("-fcf-protection", Some("3")),
        ("-fcf-protection=full", Some("3")),
        ("-fcf-protection=branch", Some("1")),
        ("-fcf-protection=return", Some("2")),
        ("-fcf-protection=none", None),
    ] {
        let arguments = ["--target=x86_64-unknown-linux-gnu", "-fstack-clash-protection", mode];
        let assertion = expected.map_or_else(
            || "#ifdef __CET__\n#error disabled CET must not define __CET__\n#endif\n".into(),
            |value| format!("_Static_assert(__CET__ == {value}, \"CET profile\");\n"),
        );
        let probe = oracle::run_c(
            &x86.profile().compiler.executable,
            &header,
            &format!("{assertion}_Static_assert(FRONT_REDEFINED(10) == 12, \"original macro\");\n"),
            &arguments,
            false,
        );
        assert!(probe.is_empty());
    }
    let frontend = inspect(&scanner, &header, &["-fpack-struct=1".into()], None).unwrap();
    assert_eq!(frontend.profile().unsupported_options, ["-fpack-struct=1"]);
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let EmissionStatus::Skipped { reason } = emit(&session, names[0]).status else {
        panic!("ABI-changing flags must retain the admission gate");
    };
    assert!(reason.message.contains("-fpack-struct=1"));
}

/// Checks that final environment tracks undefinition redefinition restoration and ambiguity.
#[test]
fn final_environment_tracks_undefinition_redefinition_restoration_and_ambiguity() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("frontend_environment.h");
    let arguments = ["-DFRONT_COMMAND_LINE(value)=((value)+73)".into()];
    let output = inspect(&scanner, &header, &arguments, None).expect("inspect original header");
    let active = &output.environment().active;
    assert!(!active.contains_key("FRONT_DELETED"));
    assert!(!active.contains_key("FRONT_INACTIVE"));
    assert!(!active.contains_key("FRONT_COMMAND_ABSENT"));
    assert!(!active.contains_key("FRONT_OPTIONAL_PRESENT"));
    assert!(active.contains_key("FRONT_COMMAND_PRESENT"));
    assert!(active.contains_key("FRONT_OPTIONAL_ABSENT"));
    assert!(active.contains_key("FRONT_EXTERNAL_STEP"), "external definitions remain context");
    for (name, expected) in [
        ("FRONT_REDEFINED", "2"),
        ("FRONT_RESTORED", "5"),
        ("FRONT_LATE_VALUE", "9"),
        ("FRONT_EXTERNAL_VALUE", "8"),
    ] {
        let definition = &active[name];
        assert!(matches!(definition.provenance, ActiveProvenance::Resolved), "{name}");
        assert!(
            definition.definition.tokens.iter().any(|token| token.spelling == expected),
            "{name}"
        );
    }
    let restored = active["FRONT_RESTORED"].definition.provenance.as_ref().unwrap();
    assert_eq!(restored.file.canonicalize().unwrap(), header.canonicalize().unwrap());
    assert_eq!((restored.start_line, restored.end_line), (9, 9));
    let ActiveProvenance::Ambiguous(spans) = &active["FRONT_IDENTICAL"].provenance else {
        panic!("identical historic definitions must not receive guessed provenance");
    };
    assert_eq!(spans.len(), 2);
    assert_eq!(spans.iter().map(|span| span.start_line).collect::<Vec<_>>(), [15, 17]);
    let command_line = &active["FRONT_COMMAND_LINE"];
    assert_eq!(command_line.definition.kind, MacroKind::FunctionLike);
    assert!(command_line.definition.provenance.is_none());
    assert!(!command_line.definition.builtin);
    assert!(matches!(command_line.provenance, ActiveProvenance::Resolved));

    // The reference invokes original definitions under the same command-line define.
    let values = oracle::run_c(
        &output.profile().compiler.executable,
        &header,
        "_Static_assert(FRONT_REDEFINED(10) == 12, \"redefined macro\");\n_Static_assert(FRONT_RESTORED(10) == 15, \"restored macro\");\n_Static_assert(FRONT_LATE_USER(10) == 19, \"late-bound macro\");\n_Static_assert(FRONT_EXTERNAL_STEP(10) == 18, \"external context macro\");\n_Static_assert(FRONT_COMMAND_PRESENT(10) == 83, \"command-line macro\");\n",
        &output.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(values.is_empty());
    for header in [&header, &fixture("frontend_context.h")] {
        let path = header.canonicalize().unwrap();
        assert!(
            output
                .profile()
                .inputs
                .files
                .iter()
                .any(|input| input.canonicalize().ok().as_ref() == Some(&path)),
            "missing include dependency {}",
            path.display()
        );
    }
    let quoted_directory = header.parent().unwrap().canonicalize().unwrap();
    assert!(
        output.profile().inputs.directories.iter().any(|directory| directory
            .canonicalize()
            .ok()
            .as_ref()
            == Some(&quoted_directory)),
        "quoted include directory must be tracked for future optional-header creation"
    );
    drop(scanner);
    assert!(output.environment().active.contains_key("FRONT_RESTORED"));
}

/// Checks that inspection rejects arguments that change its language or write outputs.
#[test]
fn inspection_rejects_arguments_that_change_its_language_or_write_outputs() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("frontend_environment.h");
    for arguments in [
        vec!["-x".into(), "c++".into()],
        vec!["-xc++".into()],
        vec!["-o".into(), "unrequested-output".into()],
        vec!["-fsyntax-only".into()],
        vec!["@untracked-arguments".into()],
    ] {
        assert!(
            matches!(
                inspect(&scanner, &header, &arguments, None),
                Err(FrontendError::Arguments(_))
            ),
            "{arguments:?}"
        );
    }
}

/// Checks that compiler profile and declarations match an independent native C probe.
#[test]
fn compiler_profile_and_declarations_match_an_independent_native_c_probe() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("frontend_environment.h");
    let arguments = ["-funsigned-char".into(), "-fwrapv".into()];
    let output =
        inspect(&scanner, &header, &arguments, None).expect("inspect target and declarations");
    let target = &output.profile().target;
    // Profile facts are hypotheses here: Clang independently computes each C fact
    // and rejects compilation if the reported values disagree with the original types.
    let mut source = format!(
        "_Static_assert(sizeof(void *) * __CHAR_BIT__ == {}, \"pointer width\");\n\
         _Static_assert(__CHAR_BIT__ == {}, \"char width\");\n\
         _Static_assert(((char) -1 < 0) == {}, \"char signedness\");\n",
        target.pointer_bits,
        target.char_bits,
        u32::from(target.char_is_signed),
    );
    assert!(!target.char_is_signed);
    for (kind, spelling) in [
        (IntegerKind::Char, "char"),
        (IntegerKind::SignedChar, "signed char"),
        (IntegerKind::UnsignedChar, "unsigned char"),
        (IntegerKind::Short, "short"),
        (IntegerKind::UnsignedShort, "unsigned short"),
        (IntegerKind::Int, "int"),
        (IntegerKind::UnsignedInt, "unsigned int"),
        (IntegerKind::Long, "long"),
        (IntegerKind::UnsignedLong, "unsigned long"),
        (IntegerKind::LongLong, "long long"),
        (IntegerKind::UnsignedLongLong, "unsigned long long"),
    ] {
        let ty = target.integers[&kind];
        assert_eq!(ty.kind, kind);
        source.push_str(&format!(
            "_Static_assert(sizeof({spelling}) * __CHAR_BIT__ == {}, \"{spelling} width\");\n\
             _Static_assert((((({spelling}) -1) < 0)) == {}, \"{spelling} signedness\");\n",
            ty.bits,
            u32::from(ty.signed),
        ));
    }
    let probe = oracle::run_c(
        &output.profile().compiler.executable,
        &header,
        &source,
        &output.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(probe.is_empty());
    assert_eq!(output.profile().signed_overflow, SignedOverflow::Wrapping);
    assert!(
        target.integers[&IntegerKind::Long].rank < target.integers[&IntegerKind::LongLong].rank
    );

    let catalog = output.declarations();
    assert_eq!(
        catalog.types["FrontByte"].category,
        TypeCategory::Integer(IntegerKind::UnsignedChar)
    );
    assert_eq!(
        catalog.types["FrontWord"].category,
        TypeCategory::Integer(IntegerKind::UnsignedShort)
    );
    assert_eq!(catalog.types["FrontBool"].category, TypeCategory::Integer(IntegerKind::Bool));
    assert_eq!(catalog.types["FrontByte"].size, Some(1));
    assert_eq!(catalog.types["FrontWord"].size, Some(2));
    assert!(matches!(
        catalog.integer_constants["FRONT_ENUM_SEVEN"].value,
        IntegerValue::Signed(7) | IntegerValue::Unsigned(7)
    ));
    assert!(
        !catalog.integer_constants.contains_key("front_const_variable"),
        "const storage is not a compiler integer constant expression"
    );
    assert!(catalog.variables["front_const_variable"].is_const);
    assert!(catalog.variables["front_volatile_counter"].is_volatile);
    assert_eq!(catalog.variables["front_pointer"].category, TypeCategory::Pointer);

    let default =
        inspect(&scanner, &header, &[], None).expect("inspect the default overflow profile");
    assert_eq!(default.profile().signed_overflow, SignedOverflow::Undefined);
}

/// Preserve cross compiler selection through driver and libclang inspection, then verify the
/// target's C widths and macro result identities independently without a target linker or SDK.
#[test]
fn cross_gcc_driver_selection_preserves_the_inspected_aarch64_target() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("frontend_cross_target.h");
    let include = header.parent().unwrap().to_str().unwrap();
    let arguments = [
        "-target".into(),
        "aarch64-unknown-linux-gnu".into(),
        "-isystem".into(),
        include.into(),
        "-ccc-gcc-name".into(),
        "aarch64-linux-gnu-gcc".into(),
        "-std=c17".into(),
    ];
    let frontend = inspect(&scanner, &header, &arguments, None)
        .expect("driver and libclang must agree on the cross GCC selection profile");
    let profile = frontend.profile();
    assert_eq!(&profile.arguments[..arguments.len()], &arguments);
    assert!(profile.target.triple.starts_with("aarch64-"), "{:?}", profile.target);
    assert_eq!(profile.target.pointer_bits, 64);
    assert_eq!(profile.target.integers[&IntegerKind::Long].bits, 64);
    assert_eq!(profile.target.size_type, IntegerKind::UnsignedLong);
    assert!(
        oracle::run_c(
            &profile.compiler.executable,
            &header,
            r#"
#ifndef __aarch64__
#error original C must use the requested AArch64 target
#endif
_Static_assert(sizeof(void *) == 8 && sizeof(long) == 8 && sizeof(int) == 4,
               "original compiler's AArch64 LP64 layout");
_Static_assert(_Generic(FRONT_CROSS_LONG(1), long: 1, default: 0),
               "original long cast identity");
_Static_assert(_Generic(FRONT_CROSS_SIZE(1), unsigned long: 1, default: 0),
               "original macro sizeof identity");
"#,
            &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            false,
        )
        .is_empty()
    );
}

/// Prove real Windows LLP64 identities survive inspection, emission, and downstream target guards.
/// No system headers or target linker are needed, so every host can exercise this ABI boundary.
#[test]
fn inspected_llp64_types_emit_with_their_actual_integer_ranks() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("frontend_llp64.h");
    let arguments = ["--target=x86_64-pc-windows-msvc".into(), "-std=c17".into()];
    let frontend = inspect(&scanner, &header, &arguments, None)
        .expect("driver and libclang must agree on the header-only LLP64 profile");
    let profile = frontend.profile();
    assert_eq!(profile.target.pointer_bits, 64);
    assert_eq!(profile.target.size_type, IntegerKind::UnsignedLongLong);
    assert_eq!(profile.target.integers[&IntegerKind::Long].bits, 32);
    assert_eq!(profile.target.integers[&IntegerKind::LongLong].bits, 64);
    assert_eq!(profile.target.ptrdiff_type, IntegerKind::LongLong);
    validate_support_profile(profile).unwrap();
    let assertions = support_abi_assertions(profile).unwrap();
    assert!(assertions.contains("target_os = \"windows\""));
    assert!(assertions.contains("CLong::BITS == 32"));
    assert!(assertions.contains("CSize::RANK == 5"));
    assert!(assertions.contains("CPtrDiff::RANK == 5"));

    let original = oracle::run_c(
        &profile.compiler.executable,
        &header,
        r#"
_Static_assert(sizeof(void *) == 8 && sizeof(long) == 4 && sizeof(long long) == 8,
               "original compiler's LLP64 layout");
_Static_assert(__builtin_types_compatible_p(__typeof__(sizeof(0)), unsigned long long),
               "original sizeof identity");
_Static_assert(__builtin_types_compatible_p(__typeof__((char *)0 - (char *)0), long long),
               "original pointer difference identity");
_Static_assert(_Generic(FRONT_LLP64_LONG(1), long: 1, default: 0),
               "original long cast identity");
_Static_assert(_Generic(FRONT_LLP64_SIZE(1), unsigned long long: 1, default: 0),
               "original macro sizeof identity");
"#,
        &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(original.is_empty());

    let names = ["FRONT_LLP64_LONG", "FRONT_LLP64_SIZE"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    for name in names {
        assert!(matches!(session.analyze(name).status, AnalysisStatus::Candidate));
        let emission = emit(&session, name);
        let EmissionStatus::Emitted { rust, .. } = emission.status else {
            panic!("proved LLP64 candidate must emit: {emission:?}");
        };
        assert!(
            rust.contains(if name == "FRONT_LLP64_LONG" { "CLong" } else { "size_of" }),
            "{name}: {rust}"
        );
        assert_eq!(emission.analysis.name, name);
    }
}

/// Prove distro preprocessor forwarding preserves macro definitions and undefinitions without
/// permitting forged compiler facts.
#[test]
fn packaging_preprocessor_forwarding_matches_original_c() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = fixture("frontend_environment.h");
    let arguments = ["-Wp,-DFRONT_COMMAND_LINE(value)=((value)+73),-UFRONT_DELETED".into()];
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    assert!(frontend.environment().active.contains_key("FRONT_COMMAND_LINE"));
    assert!(
        oracle::run_c(
            &frontend.profile().compiler.executable,
            &header,
            "_Static_assert(FRONT_COMMAND_PRESENT(10) == 83, \"forwarded definition\");\n",
            &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            false,
        )
        .is_empty()
    );
    for argument in ["-Wp,-D__CHAR_BIT__=16", "-Wp,-U__SIZE_TYPE__", "-Wp,-include,other.h"] {
        assert!(matches!(
            inspect(&scanner, &header, &[argument.into()], None),
            Err(FrontendError::Arguments(_))
        ));
    }
}

/// ARM register conventions follow effective compiler controls, including overrides that disagree
/// with the triple's soft/hard-float spelling; generated guards require the matching Rust ABI.
#[test]
fn arm_procedure_call_guards_follow_original_compiler_controls() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = fixture("frontend_environment.h");
    for (triple, float_abi, expected, cfg_abi) in [
        ("armv7-unknown-linux-gnueabi", "soft", pgrx_c_macros::ArmFloatAbi::Base, "eabi"),
        ("armv7-unknown-linux-gnueabi", "softfp", pgrx_c_macros::ArmFloatAbi::Base, "eabi"),
        ("armv7-unknown-linux-gnueabi", "hard", pgrx_c_macros::ArmFloatAbi::Vfp, "eabihf"),
        ("armv7-unknown-linux-gnueabihf", "soft", pgrx_c_macros::ArmFloatAbi::Base, "eabi"),
    ] {
        let arguments =
            [format!("--target={triple}"), format!("-mfloat-abi={float_abi}"), "-std=c17".into()];
        let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
        assert_eq!(frontend.profile().target.arm_float_abi, Some(expected));
        assert!(frontend.profile().unsupported_options.is_empty());
        let guard = support_abi_assertions(frontend.profile()).unwrap();
        assert!(guard.contains(&format!("target_abi = {cfg_abi:?}")), "{guard}");
        let vfp = if expected == pgrx_c_macros::ArmFloatAbi::Vfp { "!" } else { "" };
        let proof = format!(
            "#if !defined(__ARM_EABI__) || !defined(__ARM_PCS)\n#error missing ARM PCS\n#endif\n\
             #if {vfp}defined(__ARM_PCS_VFP)\n#error unexpected ARM float register convention\n#endif\n\
             _Static_assert(sizeof(void *) == 4, \"ARM32 pointer proof\");\n",
        );
        assert!(
            oracle::run_c(
                &frontend.profile().compiler.executable,
                &header,
                &proof,
                &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
                false,
            )
            .is_empty()
        );
    }
}

/// GNU and musl PowerPC64 defaults differ in function descriptors despite matching endian and
/// scalar widths; original compiler witnesses must select distinct downstream Rust ABI guards.
#[test]
fn ppc64_elf_guards_preserve_original_function_representation() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = fixture("frontend_environment.h");
    for (triple, expected, value, cfg_abi) in [
        ("powerpc64-unknown-linux-gnu", pgrx_c_macros::Ppc64ElfAbi::V1, 1, "elfv1"),
        ("powerpc64-unknown-linux-musl", pgrx_c_macros::Ppc64ElfAbi::V2, 2, "elfv2"),
        ("powerpc64le-unknown-linux-gnu", pgrx_c_macros::Ppc64ElfAbi::V2, 2, "elfv2"),
    ] {
        let arguments = [format!("--target={triple}"), "-std=c17".into()];
        let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
        assert_eq!(frontend.profile().target.ppc64_elf_abi, Some(expected));
        let guard = support_abi_assertions(frontend.profile()).unwrap();
        assert!(guard.contains(&format!("target_abi = {cfg_abi:?}")), "{guard}");
        assert!(
            oracle::run_c(
                &frontend.profile().compiler.executable,
                &header,
                &format!("_Static_assert(_CALL_ELF == {value}, \"native ELF ABI\");\n"),
                &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
                false,
            )
            .is_empty()
        );
    }
}

/// Reviewed packaging codegen choices keep their original arguments and preserve C integer
/// expressions; unknown ABI modes continue to fail the support gate separately.
#[test]
fn packaging_codegen_profiles_preserve_original_c_arithmetic() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = fixture("frontend_environment.h");
    let arguments = [
        "-fexceptions",
        "-fno-plt",
        "-fno-semantic-interposition",
        "-ffile-prefix-map=/build=/source",
        "-fdebug-prefix-map=/build=/source",
    ]
    .map(str::to_owned);
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    assert!(frontend.profile().unsupported_options.is_empty());
    for argument in &arguments {
        assert!(frontend.profile().arguments.contains(argument));
    }
    let session = AnalysisSession::prepare(&scanner, &frontend, &["FRONT_REDEFINED"]).unwrap();
    assert!(matches!(emit(&session, "FRONT_REDEFINED").status, EmissionStatus::Emitted { .. }));
    assert!(
        oracle::run_c(
            &frontend.profile().compiler.executable,
            &header,
            "_Static_assert(FRONT_REDEFINED(10) == 12, \"packaging arithmetic\");\n",
            &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            false,
        )
        .is_empty()
    );
}

/// Translate recorded Windows flags through both compiler paths while independently validating
/// original clang-cl definitions, unsigned plain-char behavior, and the selected DLL runtime macros.
#[test]
fn recorded_windows_controls_match_original_clang_cl_c_observations() {
    use std::process::Command;
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = fixture("frontend_llp64.h");
    let arguments = [
        "--target=x86_64-pc-windows-msvc",
        "/nologo",
        "/TC",
        "/DFRONT_MSVC=73",
        "/O2",
        "/J",
        "/MD",
        "/std:c17",
    ]
    .map(str::to_owned);
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    validate_support_profile(frontend.profile()).unwrap();
    assert!(!frontend.profile().target.char_is_signed);
    assert!(frontend.environment().active.contains_key("FRONT_MSVC"));
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("original-cl.c");
    std::fs::write(&source, "_Static_assert(FRONT_MSVC == 73 && (char)-1 > 0, \"original recorded control\");\n#ifndef _DLL\n#error original DLL runtime flag lost\n#endif\n#ifndef _MT\n#error original multithreaded runtime flag lost\n#endif\n_Static_assert(sizeof(long) == 4 && sizeof(void *) == 8, \"original LLP64 layout\");\n").unwrap();
    let original = Command::new(&frontend.profile().compiler.executable)
        .arg("--driver-mode=cl")
        .args(&arguments)
        .arg("-fsyntax-only")
        .arg(source)
        .output()
        .unwrap();
    assert!(original.status.success(), "{}", String::from_utf8_lossy(&original.stderr));
}
