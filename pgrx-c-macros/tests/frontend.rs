//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;

use pgrx_c_macros::{
    ActiveProvenance, FrontendError, IntegerKind, IntegerValue, MacroKind, MacroScanner,
    SignedOverflow, TypeCategory, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
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
