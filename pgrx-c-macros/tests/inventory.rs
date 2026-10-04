//! Test physical macro discovery independently of transpilation support.
//!
//! Headers with splicing, comments, redefinitions, diagnostics, and unusual file
//! boundaries establish exact tokens and line spans. The scanner must retain
//! preprocessor syntax and report invalid inputs rather than output partial facts.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use pgrx_c_macros::{
    DiagnosticSeverity, Error, MacroDefinition, MacroKind, MacroScanner, TokenKind,
};

// The clang wrapper permits one live Clang instance in a process.
/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Resolve fixture input relative to the crate, keeping tests independent of the invocation
/// directory.
fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

/// Require physical file identity, exact source spans, and complete definition text for a
/// discovered macro.
fn assert_provenance(definition: &MacroDefinition, file: &Path, start_line: u32, end_line: u32) {
    let span = definition.provenance.as_ref().expect("file-defined macro has physical provenance");
    assert!(span.file.is_absolute(), "{}", definition.name);
    assert_eq!(span.file.file_name(), file.file_name(), "{}", definition.name);
    assert_eq!(
        span.file.canonicalize().unwrap(),
        file.canonicalize().unwrap(),
        "{}",
        definition.name
    );
    assert_eq!((span.start_line, span.end_line), (start_line, end_line), "{}", definition.name);
}

/// Checks that the scanner discovers source macros without losing preprocessor syntax.
#[test]
fn discovers_source_macros_without_losing_preprocessor_syntax() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let header = fixture("definitions.h");
    let inventory = scanner.scan(&header, &[]).expect("fixture must parse");
    assert!(inventory.diagnostics.is_empty(), "{:?}", inventory.diagnostics);

    let find = |name: &str| {
        inventory.macros.iter().find(|definition| definition.name == name).unwrap_or_else(|| {
            panic!("missing macro {name}");
        })
    };
    for name in ["OBJECT_PARENS", "OBJECT_SPACED", "EMPTY_OBJECT", "STRING_LITERAL"] {
        assert_eq!(find(name).kind, MacroKind::ObjectLike, "{name}");
    }
    for name in [
        "INCLUDED_FUNCTION",
        "FUNCTION",
        "EMPTY_FUNCTION",
        "MULTILINE",
        "COMMENTED",
        "STRINGIFY",
        "TOKEN_PASTE",
        "VARIADIC",
        "GNU_VARIADIC",
        "IN_BODY",
    ] {
        assert_eq!(find(name).kind, MacroKind::FunctionLike, "{name}");
    }

    let spellings = |name: &str| {
        find(name).tokens.iter().map(|token| token.spelling.as_str()).collect::<Vec<_>>()
    };
    assert_eq!(spellings("EMPTY_OBJECT"), ["EMPTY_OBJECT"]);
    assert_eq!(spellings("EMPTY_FUNCTION"), ["EMPTY_FUNCTION", "(", ")"]);
    assert_eq!(spellings("OBJECT_SPACED"), ["OBJECT_SPACED", "(", "value", ")", "(", "value", ")"]);
    assert_eq!(
        spellings("MULTILINE"),
        ["MULTILINE", "(", "value", ")", "(", "(", "value", ")", "+", "2", ")"]
    );
    assert_eq!(spellings("STRINGIFY"), ["STRINGIFY", "(", "value", ")", "#", "value"]);
    assert_eq!(
        spellings("TOKEN_PASTE"),
        ["TOKEN_PASTE", "(", "left", ",", "right", ")", "left", "##", "right"]
    );
    assert_eq!(
        spellings("VARIADIC"),
        ["VARIADIC", "(", "first", ",", "...", ")", "first", ",", "__VA_ARGS__"]
    );
    assert_eq!(
        spellings("GNU_VARIADIC"),
        ["GNU_VARIADIC", "(", "first", ",", "rest", "...", ")", "first", ",", "rest"]
    );
    let comment = find("COMMENTED").tokens.iter().find(|token| token.kind == TokenKind::Comment);
    assert_eq!(
        comment.expect("inline comments must survive discovery").spelling,
        "/* retained comment */"
    );
    let literal = find("STRING_LITERAL").tokens.last().expect("string macro has a replacement");
    assert_eq!(literal.kind, TokenKind::Literal);
    assert_eq!(literal.spelling, r#""quoted \\ path""#);

    assert!(find("FUNCTION").to_string().starts_with("#define FUNCTION("));
    assert!(find("EMPTY_FUNCTION").to_string().starts_with("#define EMPTY_FUNCTION("));
    assert!(find("OBJECT_SPACED").to_string().starts_with("#define OBJECT_SPACED ("));
    assert_eq!(find("EMPTY_OBJECT").to_string(), "#define EMPTY_OBJECT");
    assert!(find("COMMENTED").to_string().contains("/* retained comment */"));
    assert!(find("TOKEN_PASTE").to_string().contains("##"));
    assert!(find("STRING_LITERAL").to_string().contains(&literal.spelling));

    let included = find("INCLUDED_VALUE");
    let included_location =
        included.location.as_ref().expect("included macro has a source location");
    assert_eq!(
        included_location.file.canonicalize().unwrap(),
        fixture("included.h").canonicalize().unwrap()
    );
    assert_eq!((included_location.line, included_location.column), (1, 9));
    assert!(!included.builtin);
    assert_provenance(included, &fixture("included.h"), 1, 1);
    assert_provenance(find("INCLUDED_FUNCTION"), &fixture("included.h"), 2, 2);
    for (name, start, end) in [
        ("OBJECT_PARENS", 3, 3),
        ("EMPTY_OBJECT", 6, 6),
        ("EMPTY_FUNCTION", 7, 7),
        ("MULTILINE", 8, 10),
        ("COMMENTED", 11, 11),
        ("IN_BODY", 23, 23),
    ] {
        assert_provenance(find(name), &header, start, end);
    }

    let source = std::fs::read_to_string(&header).unwrap();
    let main_file = header.canonicalize().unwrap();
    let mut source_offsets = Vec::new();
    for definition in &inventory.macros {
        let Some(location) = &definition.location else { continue };
        if location.file.canonicalize().ok().as_ref() != Some(&main_file) {
            continue;
        }
        let offset = usize::try_from(location.offset).unwrap();
        assert!(source[offset..].starts_with(&definition.name), "{}", definition.name);
        let prefix = &source[..offset];
        assert_eq!(
            usize::try_from(location.line).unwrap(),
            prefix.bytes().filter(|byte| *byte == b'\n').count() + 1
        );
        assert_eq!(
            usize::try_from(location.column).unwrap(),
            prefix.rsplit('\n').next().unwrap().len() + 1
        );
        source_offsets.push(location.offset);
        assert!(!definition.builtin);
    }
    assert!(source_offsets.windows(2).all(|offsets| offsets[0] < offsets[1]));

    let redefined = inventory
        .macros
        .iter()
        .filter(|definition| definition.name == "REDEFINED")
        .collect::<Vec<_>>();
    assert_eq!(redefined.len(), 2, "definition history must include definitions removed by #undef");
    assert_eq!(redefined[0].tokens.last().unwrap().spelling, "1");
    assert_eq!(redefined[1].tokens.last().unwrap().spelling, "2");
    assert_provenance(redefined[0], &header, 17, 17);
    assert_provenance(redefined[1], &header, 19, 19);
    assert!(inventory.macros.iter().any(|definition| definition.builtin));
    assert!(
        inventory
            .macros
            .iter()
            .filter(|definition| definition.builtin)
            .all(|definition| definition.provenance.is_none())
    );

    // No token, source location, or definition may borrow from Clang's translation unit.
    drop(scanner);
    assert_provenance(find("MULTILINE"), &header, 8, 10);
    assert_provenance(find("INCLUDED_FUNCTION"), &fixture("included.h"), 2, 2);
    assert!(inventory.macros.iter().any(|definition| definition.name == "IN_BODY"));
    assert!(inventory.macros.iter().filter(|definition| !definition.builtin).all(|definition| {
        definition.to_string().starts_with("#define ") && !definition.tokens.is_empty()
    }));
}

/// Checks that the scanner applies Clang defines and undefines before recording active macros.
#[test]
fn applies_clang_defines_and_undefines_before_recording_active_macros() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let header = fixture("configured.h");
    let disabled = scanner.scan(&header, &[]).unwrap();
    assert!(disabled.macros.iter().any(|definition| definition.name == "DISABLED_VALUE"));
    assert!(!disabled.macros.iter().any(|definition| matches!(
        definition.name.as_str(),
        "ENABLED_VALUE" | "INACTIVE_VALUE" | "COMMAND_LINE_PRESENT"
    )));

    let enabled = scanner
        .scan(&header, &["-DENABLE_BRANCH".into(), "-DCOMMAND_LINE_VALUE=73".into()])
        .unwrap();
    assert!(enabled.macros.iter().any(|definition| definition.name == "ENABLED_VALUE"));
    assert!(enabled.macros.iter().any(|definition| definition.name == "COMMAND_LINE_PRESENT"));
    assert!(
        !enabled.macros.iter().any(|definition| matches!(
            definition.name.as_str(),
            "DISABLED_VALUE" | "INACTIVE_VALUE"
        ))
    );
    let command_line = enabled
        .macros
        .iter()
        .find(|definition| definition.name == "COMMAND_LINE_VALUE")
        .expect("command-line definitions must be available in the library inventory");
    assert_eq!(command_line.kind, MacroKind::ObjectLike);
    assert_eq!(command_line.tokens.last().unwrap().spelling, "73");
    assert!(!command_line.builtin);
    assert!(command_line.location.is_none());
    assert!(command_line.provenance.is_none());
    let function_define =
        scanner.scan(&header, &["-DFROM_COMMAND_LINE(value)=value".into()]).unwrap();
    let function_define = function_define
        .macros
        .iter()
        .find(|definition| definition.name == "FROM_COMMAND_LINE")
        .unwrap();
    assert_eq!(function_define.kind, MacroKind::FunctionLike);
    assert!(!function_define.builtin);
    assert!(function_define.location.is_none());
    assert!(function_define.provenance.is_none());

    let undefined =
        scanner.scan(&header, &["-DENABLE_BRANCH".into(), "-UENABLE_BRANCH".into()]).unwrap();
    assert!(undefined.macros.iter().any(|definition| definition.name == "DISABLED_VALUE"));
    assert!(!undefined.macros.iter().any(|definition| definition.name == "ENABLED_VALUE"));
}

/// Checks that the pipeline rejects error diagnostics but preserves warning diagnostics.
#[test]
fn rejects_error_diagnostics_but_preserves_warning_diagnostics() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let header = fixture("diagnostics.h");
    for (argument, expected_message) in [
        ("-DTEST_ERROR", "expected macro scanner failure"),
        ("-DTEST_MISSING_INCLUDE", "pgrx_macro_fixture_missing_header.h"),
    ] {
        let error = scanner
            .scan(&header, &[argument.into()])
            .expect_err("error diagnostics must not produce a partial inventory");
        let Error::Diagnostics(diagnostics) = error else {
            panic!("expected diagnostics, got {error}")
        };
        assert!(
            diagnostics.iter().any(|diagnostic| {
                matches!(diagnostic.severity, DiagnosticSeverity::Error | DiagnosticSeverity::Fatal)
                    && diagnostic.message.contains(expected_message)
            }),
            "{diagnostics:?}"
        );
    }
    let warned = scanner
        .scan(&header, &["-DTEST_WARNING".into()])
        .expect("warnings must not reject a translation unit");
    assert!(warned.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Warning
            && diagnostic.message.contains("expected macro scanner warning")
    }));
    assert!(warned.macros.iter().any(|definition| definition.name == "AFTER_DIAGNOSTIC"));
}

/// Checks that the pipeline preserves spliced macro syntax and physical locations and rejects
/// invalid inputs.
#[test]
fn preserves_spliced_macro_syntax_and_physical_locations_and_rejects_invalid_inputs() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let header = fixture("edgecases.h");
    let inventory = scanner.scan(&header, &[]).unwrap();
    let find = |name: &str| {
        inventory.macros.iter().find(|definition| definition.name == name).unwrap_or_else(|| {
            panic!("missing macro {name}");
        })
    };
    let spliced_name = find("SPLICED_NAME");
    assert_eq!(spliced_name.tokens[0].spelling, "SPLICED_NAME");
    assert_eq!(spliced_name.kind, MacroKind::ObjectLike);
    assert!(spliced_name.to_string().starts_with("#define SPLICED_NAME "));
    assert_provenance(spliced_name, &header, 1, 2);
    let function = find("SPLICED_FUNCTION");
    assert_eq!(function.kind, MacroKind::FunctionLike);
    assert!(function.to_string().starts_with("#define SPLICED_FUNCTION("));
    assert_provenance(function, &header, 3, 4);
    let object = find("OBJECT_COMMENT");
    assert_eq!(object.kind, MacroKind::ObjectLike);
    assert!(object.to_string().starts_with("#define OBJECT_COMMENT "));

    let physical = find("PHYSICAL_LINE").location.as_ref().unwrap();
    assert_eq!(physical.file.canonicalize().unwrap(), header.canonicalize().unwrap());
    assert_eq!((physical.line, physical.column), (13, 9));
    assert_provenance(find("PHYSICAL_LINE"), &header, 13, 13);

    for (name, expected) in
        [("DIRECT_REDEFINED", ["old", "new"]), ("REPEATED_IDENTICAL", ["3", "3"])]
    {
        let definitions = inventory
            .macros
            .iter()
            .filter(|definition| definition.name == name)
            .collect::<Vec<_>>();
        assert_eq!(definitions.len(), 2, "both definitions of {name} must survive");
        assert_eq!(
            definitions
                .iter()
                .map(|definition| definition.tokens.last().unwrap().spelling.as_str())
                .collect::<Vec<_>>(),
            expected
        );
        assert!(
            definitions[0].location.as_ref().unwrap().offset
                < definitions[1].location.as_ref().unwrap().offset
        );
        let first_line = if name == "DIRECT_REDEFINED" { 7 } else { 9 };
        assert_provenance(definitions[0], &header, first_line, first_line);
        assert_provenance(definitions[1], &header, first_line + 1, first_line + 1);
    }

    assert!(matches!(scanner.scan(&header, &["-DVALUE=\0".into()]), Err(Error::InvalidInput(_))));
    for arguments in [
        vec!["-working-directory".into(), "somewhere".into()],
        vec!["-working-directory=somewhere".into()],
        vec!["-Xclang".into(), "-working-directory".into(), "-Xclang".into(), "somewhere".into()],
    ] {
        assert!(matches!(
            scanner.scan(&header, &arguments),
            Err(Error::InvalidInput(message)) if message.contains("-working-directory")
        ));
    }
    assert!(matches!(scanner.scan(Path::new("header\0.h"), &[]), Err(Error::InvalidInput(_))));
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        let non_utf8 = PathBuf::from(std::ffi::OsString::from_vec(b"header\xff.h".to_vec()));
        assert!(matches!(scanner.scan(&non_utf8, &[]), Err(Error::InvalidInput(_))));
    }
}

/// Checks that records token extents with trailing trivia crlf and end of file.
#[test]
fn records_token_extents_with_trailing_trivia_crlf_and_end_of_file() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let header = fixture("provenance.h");
    let inventory = scanner.scan(&header, &[]).unwrap();
    for (name, start, end) in [
        ("SINGLE_LINE", 1, 1),
        ("EMPTY_OBJECT_RANGE", 2, 2),
        ("EMPTY_FUNCTION_RANGE", 3, 3),
        ("EMPTY_CONTINUED", 4, 4),
        ("TRAILING_COMMENT", 7, 7),
        ("NEXT_VALUE", 9, 9),
        ("LAST_TOKEN", 10, 11),
        ("AFTER_LINE_DIRECTIVE", 14, 14),
        ("MIDDLE_COMMENT", 15, 16),
    ] {
        let definition =
            inventory.macros.iter().find(|definition| definition.name == name).unwrap();
        assert_provenance(definition, &header, start, end);
    }

    /// Own a synthetic header and its temporary directory so profile-sensitive generation has
    /// an isolated source of C facts.
    struct TemporaryHeader(
        /// Owned fixture path used for isolated inputs and cleanup.
        PathBuf,
    );
    /// Write an owned synthetic header whose source and compiler inputs can be varied
    /// independently.
    impl TemporaryHeader {
        /// Create owned, uniquely named fixture storage so this test's headers and compiler
        /// outputs cannot collide with another invocation.
        fn new(name: &str, bytes: &[u8]) -> Self {
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let header = Self(
                std::env::temp_dir()
                    .join(format!("pgrx-macro-{name}-{}-{nonce}.h", std::process::id())),
            );
            std::fs::write(&header.0, bytes).unwrap();
            header
        }
    }
    /// Release only temporary artifacts owned by this fixture, including on failed compiler or
    /// assertion paths.
    impl Drop for TemporaryHeader {
        /// Remove only this fixture's owned temporary storage after the test or oracle
        /// completes.
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
    for (name, contents, expected_end) in [
        ("eof", b"#define AT_END(value) \\\n    value".as_slice(), 2),
        ("crlf", b"#define AT_END(value) \\\r\n    value\r\n".as_slice(), 2),
    ] {
        let header = TemporaryHeader::new(name, contents);
        let inventory = scanner.scan(&header.0, &[]).unwrap();
        let definition =
            inventory.macros.iter().find(|definition| definition.name == "AT_END").unwrap();
        assert_provenance(definition, &header.0, 1, expected_end);
    }
}
