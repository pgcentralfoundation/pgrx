//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;

use pgrx_c_macros::{
    AnalysisSession, ExpandedMacro, ExpansionBatch, ExpansionLimits, ExpansionResult,
    ExpansionSkipCode, FrontendError, MacroScanner, inspect, prepare_expansions,
    prepare_expansions_with_limits,
};
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

struct Directory(PathBuf);

impl Directory {
    fn new() -> Self {
        for attempt in 0..128 {
            let path = std::env::temp_dir()
                .join(format!("pgrx-expansion-tests-{}-{attempt}", std::process::id()));
            match std::fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("could not create expansion fixture: {error}"),
            }
        }
        panic!("could not reserve expansion fixture directory");
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn expanded<'a>(batch: &'a ExpansionBatch, name: &str) -> &'a ExpandedMacro {
    match &batch.results[name] {
        ExpansionResult::Expanded { expansion } => expansion,
        other => panic!("{name} must expand: {other:?}"),
    }
}

fn body(expansion: &ExpandedMacro) -> Vec<&str> {
    let start =
        expansion.definition.tokens.iter().position(|token| token.spelling == ")").unwrap() + 1;
    expansion.definition.tokens[start..]
        .iter()
        .map(|token| {
            expansion
                .symbolic_parameters
                .iter()
                .position(|marker| marker == &token.spelling)
                .map(|index| expansion.parameters[index].as_str())
                .unwrap_or(token.spelling.as_str())
        })
        .collect()
}

fn assert_skip(batch: &ExpansionBatch, name: &str, code: ExpansionSkipCode) {
    let ExpansionResult::Skipped { reason } = &batch.results[name] else {
        panic!("{name} must skip as {code:?}: {:?}", batch.results[name]);
    };
    assert_eq!(reason.code, code, "{name}: {}", reason.message);
    assert!(!reason.message.is_empty());
    assert!(!reason.spans.is_empty(), "{name} retains original source provenance");
}

#[test]
fn clang_expands_nested_late_prescanned_rescanned_and_suppressed_macros_as_one_batch() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = fixture("expansion.h");
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let inventory = frontend.inventory().clone();
    let batch = prepare_expansions(
        &scanner,
        &frontend,
        &[
            "EXP_NESTED",
            "EXP_LATE",
            "EXP_PRESCAN",
            "EXP_RESCAN",
            "EXP_RECURSIVE",
            "EXP_CYCLE_A",
            "EXP_UNUSED",
        ],
    )
    .unwrap();
    assert_eq!(frontend.inventory(), &inventory, "expansion preserves raw definition history");
    assert_eq!(
        body(expanded(&batch, "EXP_NESTED")),
        ["(", "(", "(", "(", "x", ")", "+", "(", "1", ")", ")", ")", "+", "(", "2", ")", ")"]
    );
    assert_eq!(body(expanded(&batch, "EXP_LATE")), ["(", "(", "x", ")", "+", "9", ")"]);
    assert_eq!(expanded(&batch, "EXP_PRESCAN").occurrences.len(), 4);
    assert_eq!(body(expanded(&batch, "EXP_RESCAN")), ["(", "(", "x", ")", ")"]);
    assert_eq!(
        body(expanded(&batch, "EXP_RECURSIVE")),
        ["(", "(", "x", ")", "+", "EXP_RECURSIVE", "(", "x", ")", ")"]
    );
    assert_eq!(body(expanded(&batch, "EXP_CYCLE_A")), ["EXP_CYCLE_A", "(", "x", ")"]);
    assert!(expanded(&batch, "EXP_UNUSED").occurrences.is_empty());
    assert_eq!(body(expanded(&batch, "EXP_UNUSED")), ["(", "7", ")"]);
    let nested = expanded(&batch, "EXP_NESTED");
    assert!(
        nested.dependencies.iter().any(
            |dependency| dependency.name == "EXP_ADD_HELPER" && dependency.provenance.is_some()
        )
    );
    assert_eq!(
        nested.definition.provenance,
        frontend.environment().active["EXP_NESTED"].definition.provenance
    );
    assert!(batch.inputs.files.contains(&header));
    assert!(!batch.inputs.files.iter().any(|path| {
        path.file_name().is_some_and(|name| name == "environment.c" || name == "expansion.c")
    }));

    let checked = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        "_Static_assert(EXP_NESTED(4) == 7, \"nested\");\n_Static_assert(EXP_LATE(4) == 13, \"late\");\n_Static_assert(EXP_PRESCAN(4) == 16, \"prescan\");\n_Static_assert(EXP_RESCAN(4) == 4, \"rescan\");\n_Static_assert(EXP_UNUSED(never_declared) == 7, \"unused argument\");\n",
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(checked.is_empty());
}

#[test]
fn parameter_markers_do_not_invent_grouping_and_cannot_collide_with_header_symbols() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &fixture("expansion.h"), &[], None).unwrap();
    let batch =
        prepare_expansions(&scanner, &frontend, &["EXP_RAW", "EXP_OUTER_GROUP", "EXP_HOSTILE"])
            .unwrap();
    assert_eq!(body(expanded(&batch, "EXP_RAW")), ["x", "+", "1"]);
    assert_eq!(body(expanded(&batch, "EXP_OUTER_GROUP")), ["(", "x", "+", "1", ")"]);
    assert_eq!(expanded(&batch, "EXP_RAW").occurrences[0].token, 0);
    assert_eq!(expanded(&batch, "EXP_OUTER_GROUP").occurrences[0].token, 1);
    assert_eq!(
        body(expanded(&batch, "EXP_HOSTILE")),
        ["(", "(", "__pgrx_c_expand_1_parameter_0_0", ")", ")"]
    );
    assert_eq!(expanded(&batch, "EXP_HOSTILE").occurrences.len(), 1);
}

#[test]
fn unsafe_preprocessing_constructs_reject_through_the_dependency_closure() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &fixture("expansion.h"), &[], None).unwrap();
    let batch = prepare_expansions(
        &scanner,
        &frontend,
        &[
            "EXP_STRING",
            "EXP_PASTE",
            "EXP_DYNAMIC",
            "EXP_COUNTER",
            "EXP_DATE",
            "EXP_PRAGMA",
            "EXP_HAS_INCLUDE",
            "EXP_TARGET_QUERY",
            "EXP_MODULE_QUERY",
            "EXP_UNKNOWN_QUERY",
            "EXP_VARIADIC",
            "EXP_AMBIG",
        ],
    )
    .unwrap();
    for (name, code) in [
        ("EXP_STRING", ExpansionSkipCode::Stringification),
        ("EXP_PASTE", ExpansionSkipCode::TokenPaste),
        ("EXP_DYNAMIC", ExpansionSkipCode::DynamicBuiltin),
        ("EXP_COUNTER", ExpansionSkipCode::DynamicBuiltin),
        ("EXP_DATE", ExpansionSkipCode::DynamicBuiltin),
        ("EXP_PRAGMA", ExpansionSkipCode::DynamicBuiltin),
        ("EXP_HAS_INCLUDE", ExpansionSkipCode::DynamicBuiltin),
        ("EXP_TARGET_QUERY", ExpansionSkipCode::DynamicBuiltin),
        ("EXP_MODULE_QUERY", ExpansionSkipCode::DynamicBuiltin),
        ("EXP_UNKNOWN_QUERY", ExpansionSkipCode::DynamicBuiltin),
        ("EXP_VARIADIC", ExpansionSkipCode::Variadic),
        ("EXP_AMBIG", ExpansionSkipCode::ProvenanceAmbiguous),
    ] {
        assert_skip(&batch, name, code);
    }
    let ExpansionResult::Skipped { reason } = &batch.results["EXP_STRING"] else { unreachable!() };
    assert_eq!(reason.dependency.as_deref(), Some("EXP_STRING_HELPER"));
    assert_eq!(reason.spans.len(), 2, "root and helper spans are retained");
    let ExpansionResult::Skipped { reason } = &batch.results["EXP_AMBIG"] else { unreachable!() };
    assert_eq!(reason.spans.len(), 3, "both ambiguous definitions and the root are retained");
    assert!(reason.spans.windows(2).all(|pair| pair[0].start_line <= pair[1].start_line));
}

#[test]
fn expansion_budgets_and_compiler_rejection_are_structured_skips() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &fixture("expansion.h"), &[], None).unwrap();
    let batch = prepare_expansions_with_limits(
        &scanner,
        &frontend,
        &["EXP_BIG", "EXP_ID"],
        ExpansionLimits { macro_tokens: 20, ..ExpansionLimits::default() },
    )
    .unwrap();
    assert_skip(&batch, "EXP_BIG", ExpansionSkipCode::BudgetExceeded);
    assert_eq!(expanded(&batch, "EXP_ID").occurrences.len(), 1);
    for limits in [
        ExpansionLimits { macros: 0, ..ExpansionLimits::default() },
        ExpansionLimits { source_bytes: 0, ..ExpansionLimits::default() },
        ExpansionLimits { expanded_bytes: 0, ..ExpansionLimits::default() },
        ExpansionLimits { total_tokens: 0, ..ExpansionLimits::default() },
        ExpansionLimits { dependency_tokens: 0, ..ExpansionLimits::default() },
        ExpansionLimits { dependencies_per_macro: 0, ..ExpansionLimits::default() },
    ] {
        let batch =
            prepare_expansions_with_limits(&scanner, &frontend, &["EXP_ID"], limits).unwrap();
        assert_skip(&batch, "EXP_ID", ExpansionSkipCode::BudgetExceeded);
    }
    let batch = prepare_expansions(&scanner, &frontend, &["EXP_BAD_ARITY"]).unwrap();
    assert_skip(&batch, "EXP_BAD_ARITY", ExpansionSkipCode::CompilerRejected);
}

#[test]
fn original_main_file_context_preserves_macros_and_conditional_declarations() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &fixture("expansion_context.h"), &[], None).unwrap();
    let batch = prepare_expansions(&scanner, &frontend, &["EXP_CONTEXT"]).unwrap();
    assert_eq!(body(expanded(&batch, "EXP_CONTEXT")), ["(", "(", "x", ")", "+", "1", ")"]);
    assert_eq!(frontend.declarations().types["ExpansionContextType"].size, Some(1));
    let batch = prepare_expansions(&scanner, &frontend, &["EXP_CONTEXT_TYPE"]).unwrap();
    assert!(body(expanded(&batch, "EXP_CONTEXT_TYPE")).contains(&"ExpansionContextType"));
}

#[test]
fn nested_expansion_does_not_capture_an_identifier_named_like_an_unused_formal() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &fixture("expansion.h"), &[], None).unwrap();
    let batch = prepare_expansions(&scanner, &frontend, &["EXP_CAPTURE"]).unwrap();
    let expansion = expanded(&batch, "EXP_CAPTURE");
    assert_eq!(expansion.parameters, ["x", "captured"]);
    assert_eq!(expansion.occurrences.len(), 1);
    assert_eq!(expansion.occurrences[0].parameter, 0);
    let tokens = &expansion.definition.tokens;
    assert!(tokens.iter().any(|token| token.spelling == expansion.symbolic_parameters[0]));
    assert!(tokens.iter().any(|token| token.spelling == expansion.symbolic_parameters[1]));
    assert!(body(expansion).contains(&"captured"));
    assert_ne!(expansion.symbolic_parameters[1], "captured");
}

#[test]
#[cfg(unix)]
fn original_symlink_path_keeps_quoted_include_lookup_and_main_file_context() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let source = directory.0.join("source");
    let entry = directory.0.join("entry");
    std::fs::create_dir(&source).unwrap();
    std::fs::create_dir(&entry).unwrap();
    std::fs::write(source.join("main.h"), "#include \"choice.h\"\n#if __INCLUDE_LEVEL__ == 0\n#define EXP_MAIN_VALUE 3\n#else\n#define EXP_MAIN_VALUE 100\n#endif\n#define EXP_PATH(x) ((x) + CHOICE + EXP_MAIN_VALUE)\n").unwrap();
    std::fs::write(source.join("choice.h"), "#define CHOICE 41\n").unwrap();
    std::fs::write(entry.join("choice.h"), "#define CHOICE 7\n").unwrap();
    let header = entry.join("main.h");
    std::os::unix::fs::symlink(source.join("main.h"), &header).unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let batch = prepare_expansions(&scanner, &frontend, &["EXP_PATH"]).unwrap();
    assert_eq!(body(expanded(&batch, "EXP_PATH")), ["(", "(", "x", ")", "+", "7", "+", "3", ")"]);
}

#[test]
fn stale_declarations_and_new_shadowing_headers_reject_even_with_identical_macro_maps() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("main.h");
    std::fs::write(&header, "typedef char ValueType;\n#define EXP_STALE(x) ((ValueType)(x))\n")
        .unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let metadata = std::fs::metadata(&header).unwrap();
    std::fs::write(&header, "typedef long ValueType;\n#define EXP_STALE(x) ((ValueType)(x))\n")
        .unwrap();
    std::fs::OpenOptions::new()
        .write(true)
        .open(&header)
        .unwrap()
        .set_times(std::fs::FileTimes::new().set_modified(metadata.modified().unwrap()))
        .unwrap();
    assert_eq!(std::fs::metadata(&header).unwrap().len(), metadata.len());
    assert_eq!(
        std::fs::metadata(&header).unwrap().modified().unwrap(),
        metadata.modified().unwrap()
    );
    assert!(
        matches!(prepare_expansions(&scanner, &frontend, &["EXP_STALE"]), Err(FrontendError::Environment(message)) if message.contains("changed"))
    );
    assert!(
        matches!(AnalysisSession::prepare(&scanner, &frontend, &["EXP_STALE"]), Err(FrontendError::Environment(message)) if message.contains("changed"))
    );

    let earlier = directory.0.join("earlier");
    let later = directory.0.join("later");
    std::fs::create_dir(&earlier).unwrap();
    std::fs::create_dir(&later).unwrap();
    std::fs::write(later.join("choice.h"), "typedef char ValueType;\n").unwrap();
    std::fs::write(&header, "#include <choice.h>\n#define EXP_STALE(x) ((ValueType)(x))\n")
        .unwrap();
    let frontend = inspect(
        &scanner,
        &header,
        &[format!("-I{}", earlier.display()), format!("-I{}", later.display())],
        None,
    )
    .unwrap();
    std::fs::write(earlier.join("choice.h"), "typedef long ValueType;\n").unwrap();
    assert!(
        matches!(prepare_expansions(&scanner, &frontend, &["EXP_STALE"]), Err(FrontendError::Environment(message)) if message.contains("new header dependency"))
    );
    assert!(
        matches!(AnalysisSession::prepare(&scanner, &frontend, &["EXP_STALE"]), Err(FrontendError::Environment(message)) if message.contains("new header dependency"))
    );
}

#[test]
fn appending_probes_cannot_turn_a_trailing_backslash_into_a_new_macro_body() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("main.h");
    std::fs::write(&header, "#define EXP_TRAILING(x) (x) \\").unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    assert!(
        matches!(prepare_expansions(&scanner, &frontend, &["EXP_TRAILING"]), Err(FrontendError::Environment(message)) if message.contains("final macro environment"))
    );
}

#[test]
fn snapshot_line_protection_preserves_original_comments_and_literal_backslashes() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &fixture("expansion.h"), &[], None).unwrap();
    let comment = &frontend.environment().active["EXP_SNAPSHOT_COMMENT"].definition;
    assert!(comment.tokens.iter().any(|token| token.spelling == "/*__pgrx_c_snapshot_end_0__*/"));
    let original = &frontend.environment().active["EXP_SNAPSHOT_STRING"].definition;
    let batch = prepare_expansions(&scanner, &frontend, &["EXP_SNAPSHOT_STRING"]).unwrap();
    let expansion = expanded(&batch, "EXP_SNAPSHOT_STRING");
    let old_literal = original.tokens.iter().find(|token| token.spelling.starts_with('"')).unwrap();
    let new_literal =
        expansion.definition.tokens.iter().find(|token| token.spelling.starts_with('"')).unwrap();
    assert_eq!(new_literal, old_literal);
}
