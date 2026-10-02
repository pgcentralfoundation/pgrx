//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    ActiveProvenance, AnalysisSession, BindingCatalog, EmissionStatus, FrontendError, IntegerKind,
    IntegerValue, MacroKind, MacroScanner, SignedOverflow, TypeCategory, emit,
    emit_support_with_bindings, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

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
    let arguments = vec!["-fstack-clash-protection".into(), native_cf.into()];
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    assert!(frontend.profile().unsupported_options.is_empty());
    for argument in &arguments {
        assert!(frontend.profile().arguments.contains(argument), "must retain {argument}");
    }
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let runtime =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../pgrx-pg-sys/src/c_macros/support.rs");
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

#[test]
fn inspection_rejects_arguments_that_change_its_language_or_write_outputs() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("frontend_environment.h");
    for arguments in [
        vec!["-x".into(), "c++".into()],
        vec!["-xc++".into()],
        vec!["-o".into(), "unrequested-output".into()],
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
