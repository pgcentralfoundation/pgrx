//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Test compiler-owned preprocessing before expression analysis.
//!
//! The batch expansion path must retain prescan, rescan, suppression, original
//! include lookup, and late definitions. Malformed probes, unsafe preprocessing,
//! and stale inputs must produce isolated structured skips without contaminating
//! valid peers or changing the original header's interpretation.

/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;

use pgrx_c_macros::{
    ActiveProvenance, AnalysisSession, AnalysisStatus, ExpandedMacro, ExpansionBatch,
    ExpansionLimits, ExpansionResult, ExpansionSkipCode, FrontendError, FrontendOutput,
    MacroScanner, SkipReasonCode, inspect, prepare_expansions, prepare_expansions_with_limits,
};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Resolve fixture input relative to the crate, keeping tests independent of the invocation
/// directory.
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

/// Own the temporary header tree for this test and remove only its fixture files afterward.
struct Directory(
    /// Owned fixture path used for isolated inputs and cleanup.
    PathBuf,
);

/// Construct owned header trees for freshness and include-identity tests.
impl Directory {
    /// Create owned, uniquely named fixture storage so this test's headers and compiler outputs
    /// cannot collide with another invocation.
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

/// Release only temporary artifacts owned by this fixture, including on failed compiler or
/// assertion paths.
impl Drop for Directory {
    /// Remove only this fixture's owned temporary storage after the test or oracle completes.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Inspect and expand selected fixture macros under one verified frontend session.
fn expanded<'a>(batch: &'a ExpansionBatch, name: &str) -> &'a ExpandedMacro {
    match &batch.results[name] {
        ExpansionResult::Expanded { expansion } => expansion,
        other => panic!("{name} must expand: {other:?}"),
    }
}

/// Retrieve a supported expansion body and turn an unexpected structured skip into a useful
/// test failure.
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

/// Require the expected structured rejection, including the reason used to explain unsupported
/// C syntax.
fn assert_skip(batch: &ExpansionBatch, name: &str, code: ExpansionSkipCode) {
    let ExpansionResult::Skipped { reason } = &batch.results[name] else {
        panic!("{name} must skip as {code:?}: {:?}", batch.results[name]);
    };
    assert_eq!(reason.code, code, "{name}: {}", reason.message);
    assert!(!reason.message.is_empty());
    assert!(!reason.spans.is_empty(), "{name} retains original source provenance");
}

/// Require the expected structured rejection with its exact reason.
fn assert_skip_reason(batch: &ExpansionBatch, name: &str, code: ExpansionSkipCode, message: &str) {
    assert_skip(batch, name, code);
    let ExpansionResult::Skipped { reason } = &batch.results[name] else { unreachable!() };
    assert_eq!(reason.message, message, "{name}");
}

/// Admit identical object replacements from different headers as context while retaining all
/// origins, actual push/pop restoration, public-root refusals and dynamic-builtin refusals.
#[test]
fn matching_object_context_retains_ambiguous_origins_without_guessing_ownership() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let directory = Directory::new();
    let definitions = "#define CONTEXT_VALUE 7\n#define CONTEXT_COUNTER __COUNTER__\n";
    std::fs::write(directory.0.join("first.h"), definitions).unwrap();
    std::fs::write(
        directory.0.join("second.h"),
        format!(
            "{definitions}#pragma push_macro(\"CONTEXT_VALUE\")\n#undef CONTEXT_VALUE\n#define CONTEXT_VALUE 99\n#pragma pop_macro(\"CONTEXT_VALUE\")\n"
        ),
    )
    .unwrap();
    let header = directory.0.join("root.h");
    std::fs::write(
        &header,
        "#include \"first.h\"\n#include \"second.h\"\n#define CONTEXT_SUM(value) ((value) + CONTEXT_VALUE)\n#define CONTEXT_DYNAMIC() CONTEXT_COUNTER\n",
    )
    .unwrap();
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let active = &frontend.environment().active["CONTEXT_VALUE"];
    let ActiveProvenance::Ambiguous(origins) = &active.provenance else {
        panic!("matching definitions in two headers must retain ambiguous ownership");
    };
    assert_eq!(origins.len(), 2);
    assert_ne!(origins[0].file, origins[1].file);
    assert!(origins.iter().all(|span| span.start_line == 1));
    assert!(active.resolved_provenance().is_none());

    let session =
        AnalysisSession::prepare(&scanner, &frontend, &["CONTEXT_SUM", "CONTEXT_DYNAMIC"]).unwrap();
    let expansion = expanded(session.expansions(), "CONTEXT_SUM");
    let dependency =
        expansion.dependencies.iter().find(|item| item.name == "CONTEXT_VALUE").unwrap();
    assert!(dependency.provenance.is_none());
    let ActiveProvenance::Ambiguous(recorded) = &dependency.provenance_resolution else {
        panic!("expanded context must retain both source alternatives");
    };
    assert_eq!(recorded, origins);
    assert!(matches!(session.analyze("CONTEXT_SUM").status, AnalysisStatus::Candidate));
    assert_skip(session.expansions(), "CONTEXT_DYNAMIC", ExpansionSkipCode::DynamicBuiltin);

    let objects =
        AnalysisSession::prepare_objects(&scanner, &frontend, &["CONTEXT_VALUE"]).unwrap();
    assert_skip(objects.expansions(), "CONTEXT_VALUE", ExpansionSkipCode::ProvenanceAmbiguous);
    let object_analysis = objects.analyze("CONTEXT_VALUE");
    assert!(object_analysis.provenance.is_none(), "a refused root cannot claim one source owner");
    assert!(matches!(
        object_analysis.status,
        AnalysisStatus::Skipped { reason } if reason.code == SkipReasonCode::ProvenanceAmbiguous
    ));
    let profile = frontend.profile();
    assert!(
        oracle::run_c(
            &profile.compiler.executable,
            &header,
            "_Static_assert(CONTEXT_SUM(5) == 12, \"restored original value\");\n",
            &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            false,
        )
        .is_empty()
    );
}

/// Checks that clang expands nested late prescanned rescanned and suppressed macros as one
/// batch.
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

/// Checks that malformed ordinary probes are isolated before using clean output.
#[test]
fn malformed_ordinary_probes_are_isolated_before_using_clean_output() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("ordinary-errors.h");
    std::fs::write(
        &header,
        "#define EXPECTS_TWO(a,b) ((a)+(b))\n#define BAD(x) EXPECTS_TWO(x)\n#define HEALTHY(x) ((x)+1)\n",
    )
    .unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let batch = prepare_expansions(&scanner, &frontend, &["BAD", "HEALTHY"]).unwrap();
    assert_skip(&batch, "BAD", ExpansionSkipCode::CompilerRejected);
    assert_eq!(body(expanded(&batch, "HEALTHY")), ["(", "(", "x", ")", "+", "1", ")"]);
    assert_eq!(expanded(&batch, "HEALTHY").occurrences.len(), 1);
}

/// Checks that parameter markers do not invent grouping and cannot collide with header symbols.
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

/// Reject unsafe dependency preprocessing while retaining stringification for contextual analysis.
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
        ("EXP_PASTE", ExpansionSkipCode::TokenPaste),
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
    let string = expanded(&batch, "EXP_STRING");
    assert!(string.definition.provenance.is_some());
    assert!(string.dependencies.iter().any(|dependency| {
        dependency.name == "EXP_STRING_HELPER" && dependency.provenance.is_some()
    }));
    let session = AnalysisSession::prepare(&scanner, &frontend, &["EXP_STRING"]).unwrap();
    let analysis = session.analyze("EXP_STRING");
    let AnalysisStatus::Skipped { reason } = analysis.status else {
        panic!("a returned stringified operand must remain unsupported");
    };
    assert_eq!(reason.code, SkipReasonCode::Stringification);
    assert!(!reason.message.is_empty());
    let dynamic = expanded(&batch, "EXP_DYNAMIC");
    assert!(body(dynamic).contains(&"__LINE__"));
    assert!(dynamic.dependencies.iter().any(|dependency| {
        dependency.name == "EXP_DYNAMIC_HELPER" && dependency.provenance.is_some()
    }));
    let session = AnalysisSession::prepare(&scanner, &frontend, &["EXP_DYNAMIC"]).unwrap();
    let analysis = session.analyze("EXP_DYNAMIC");
    assert!(matches!(analysis.status, AnalysisStatus::Candidate), "{analysis:?}");
    let ExpansionResult::Skipped { reason } = &batch.results["EXP_AMBIG"] else { unreachable!() };
    assert_eq!(reason.spans.len(), 3, "both ambiguous definitions and the root are retained");
    assert!(reason.spans.windows(2).all(|pair| pair[0].start_line <= pair[1].start_line));
}

/// Checks that expansion budgets and compiler rejection are structured skips.
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

/// Checks that an over-budget batch skips only the invocations that do not fit, so the
/// preparation budget must account for every byte the authoritative pass adds.
#[test]
fn source_budget_skips_only_invocations_that_do_not_fit() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("budget.h");
    std::fs::write(&header, "#define F(x) (x)\n#define G(x) (x)\n").unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let prepare = |source_bytes| {
        let limits = ExpansionLimits { source_bytes, ..ExpansionLimits::default() };
        prepare_expansions_with_limits(&scanner, &frontend, &["F", "G"], limits).unwrap()
    };
    let both = |batch: &ExpansionBatch| {
        ["F", "G"]
            .iter()
            .all(|name| matches!(batch.results[*name], ExpansionResult::Expanded { .. }))
    };
    // Probe sizes depend on the temporary directory, so search for the smallest
    // budget that holds both invocations instead of predicting it.
    let (mut low, mut high) = (0, 64 * 1024);
    assert!(both(&prepare(high)));
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        if both(&prepare(middle)) {
            high = middle;
        } else {
            low = middle;
        }
    }
    let batch = prepare(high - 1);
    assert_eq!(expanded(&batch, "F").occurrences.len(), 1);
    assert_skip(&batch, "G", ExpansionSkipCode::BudgetExceeded);
}

/// Reject unsafe pastes and probe collisions without rejecting fully erased stringification.
#[test]
fn closed_pastes_reject_erased_operands_dynamic_helpers_and_probe_collisions() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("paste.h");
    let definitions = r#"
int __paste_native(int);
#define CAT(a,b) a##b
#define CAT3(a,b,c) a##b##c
#define DROP(x) 0
#define PRESCAN(x) DROP(x)
#define UNPAREN(x) x
#define STRIP(x) UNPAREN x
#define P_STRING(x) #x
#define P_VARIADIC(...) 0
#define pre___pgrx_c_expand_0_parameter_0_0 99
#define P_CLOSED(x) ((x)+CAT(0xFF,U))
#define P_REAL(x) CAT(__paste_,native)(x)
#define P_EMPTY_LEFT(x) (CAT(,x))
#define P_EMPTY_RIGHT(x) (CAT(x,))
#define P_ERASED_PREFIX(x) PRESCAN(CAT(prefix_,x))
#define P_ERASED_SUFFIX(x) PRESCAN(CAT(x,_suffix))
#define P_ERASED_STRIP(x) PRESCAN(CAT(STRIP(x),_suffix))
#define P_HIDDEN_COUNTER(x) PRESCAN(CAT(__COUN,TER__))
#define P_HIDDEN_LINE(x) CAT(__LI,NE__)
#define P_HIDDEN_PRAGMA(x) PRESCAN(CAT(_Pr,agma)("GCC poison P_CLOSED"))
#define P_HIDDEN_QUERY(x) PRESCAN(CAT(__has_in,clude)("never-created.h"))
#define P_HIDDEN_VARIANT(x) (CAT(__ha,s_target_variant)() + (x))
#define P_UNKNOWN(x) CAT(__builtin_,constant_p)(x)
#define P_SYNTH_STRING(x) PRESCAN(CAT(P_STR,ING)(x))
#define P_SYNTH_VARIADIC(x) PRESCAN(CAT(P_VARI,ADIC)(x))
#define P_FORGED_PARAMETER(x) PRESCAN(CAT3(__pgrx_,c_expand_1_,parameter_0_0))
#define P_FORGED_BOUNDARY(x) PRESCAN(CAT3(__pgrx_,c_expand_1_,end_0))
#define P_FORGED_CONSTANT(x) PRESCAN(CAT3(__pgrx_,c_expand_1_,c0))
#pragma clang diagnostic ignored "-Winvalid-token-paste"
#if __has_warning("-Wgnu-line-marker")
#pragma clang diagnostic ignored "-Wgnu-line-marker"
#endif
"#;
    // Preserve real definition provenance, then leave the original main file in
    // system-header status. The generator must reset that status for its probes.
    std::fs::write(&header, format!("{definitions}\n# 1 {:?} 3\n", header.to_str().unwrap()))
        .unwrap();
    let scanner = MacroScanner::new().unwrap();
    let good = ["P_CLOSED", "P_REAL", "P_EMPTY_LEFT", "P_EMPTY_RIGHT"];
    let bad = [
        "P_ERASED_PREFIX",
        "P_ERASED_SUFFIX",
        "P_ERASED_STRIP",
        "P_HIDDEN_COUNTER",
        "P_HIDDEN_LINE",
        "P_HIDDEN_PRAGMA",
        "P_HIDDEN_QUERY",
        "P_HIDDEN_VARIANT",
        "P_UNKNOWN",
        "P_SYNTH_VARIADIC",
        "P_FORGED_PARAMETER",
        "P_FORGED_BOUNDARY",
        "P_FORGED_CONSTANT",
    ];
    for arguments in [
        vec!["-std=c17".into()],
        vec!["-std=c17".into(), "-fms-extensions".into(), "-Wno-everything".into()],
        // Clang's default for MSVC targets, which can soften preprocessor errors.
        vec![
            "-std=c17".into(),
            "-fms-compatibility".into(),
            "-fms-extensions".into(),
            "-Wno-everything".into(),
        ],
        vec!["-std=c17".into(), "-Werror".into()],
        vec!["-std=c17".into(), "-pedantic-errors".into()],
    ] {
        let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
        let original = frontend.inventory().clone();
        let names = good.iter().chain(&bad).copied().chain(["P_SYNTH_STRING"]).collect::<Vec<_>>();
        let batch = prepare_expansions(&scanner, &frontend, &names).unwrap();
        for name in good {
            let expansion = expanded(&batch, name);
            assert_eq!(expansion.occurrences.len(), 1, "{name}: {arguments:?}");
            assert!(expansion.symbolic_parameters[0].starts_with("__pgrx_c_expand_1_"));
        }
        assert!(body(expanded(&batch, "P_CLOSED")).contains(&"0xFFU"));
        let erased_string = expanded(&batch, "P_SYNTH_STRING");
        assert_eq!(body(erased_string), ["0"]);
        assert!(erased_string.occurrences.is_empty());
        assert!(erased_string.dependencies.iter().any(|dependency| {
            dependency.name == "P_STRING" && dependency.provenance.is_some()
        }));
        let session = AnalysisSession::prepare(&scanner, &frontend, &["P_SYNTH_STRING"]).unwrap();
        let analysis = session.analyze("P_SYNTH_STRING");
        if arguments.iter().any(|argument| argument == "-fms-extensions") {
            // Preprocessing is valid in this profile, but its extra Unix semantic
            // option still must not bypass the independent runtime profile gate.
            assert!(
                matches!(analysis.status, AnalysisStatus::Skipped { ref reason }
                if reason.code == SkipReasonCode::UnsupportedProfile
                    && reason.message.contains("-fms-extensions")),
                "{analysis:?}"
            );
        } else {
            assert!(matches!(analysis.status, AnalysisStatus::Candidate), "{analysis:?}");
        }
        let checked = oracle::run_c(
            &frontend.profile().compiler.executable,
            &header,
            "_Static_assert(P_SYNTH_STRING(never_declared) == 0, \"erased stringification\");\n",
            &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            false,
        );
        assert!(checked.is_empty());
        for name in bad {
            let ExpansionResult::Skipped { reason } = &batch.results[name] else {
                panic!("{name} must reject under {arguments:?}");
            };
            assert!(!reason.message.is_empty());
            assert!(!reason.spans.is_empty());
        }
        // Apple clang 21 registers this query builtin; upstream Clang does not.
        assert_skip(&batch, "P_HIDDEN_VARIANT", ExpansionSkipCode::DynamicBuiltin);
        assert_skip(&batch, "P_SYNTH_VARIADIC", ExpansionSkipCode::Variadic);
        let ExpansionResult::Skipped { reason } = &batch.results["P_SYNTH_VARIADIC"] else {
            unreachable!()
        };
        assert_eq!(reason.dependency.as_deref(), Some("P_VARIADIC"));
        assert_eq!(reason.spans.len(), 2);
        assert_eq!(frontend.inventory(), &original);
        assert!(!frontend.environment().active.keys().any(|name| name.contains("paste_forbidden")));
    }
}

/// The paste proof poisons every active macro name, so its source grows with the environment.
/// Windows SDK headers add enough macros that a fixed source budget refused every closed paste
/// in PostgreSQL's Windows headers. Here the names alone exceed that budget, yet the proof still
/// applies its whole poison table: it finds a synthesized macro and rejects a forged constant
/// marker whose name sorts after every filler.
#[test]
fn closed_paste_proof_poisons_environments_larger_than_the_source_budget() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("environment.h");
    let fillers = 48_000;
    let last = format!("ENVIRONMENT_FILLER_{}", fillers - 1);
    // Like system headers, the environment is included rather than part of the main file,
    // which counts against the source budget.
    let environment = (0..fillers)
        .map(|index| format!("#define ENVIRONMENT_FILLER_{index} 7\n"))
        .collect::<String>();
    std::fs::write(directory.0.join("fillers.h"), environment).unwrap();
    let definitions = format!(
        r#"
#include "fillers.h"
#define CAT(a,b) a##b
#define CAT3(a,b,c) a##b##c
#define DROP(x) 0
#define PRESCAN(x) DROP(x)
#define P_CLOSED(x) ((x)+CAT(0xFF,U))
#define P_SYNTHESIZED(x) ((x)+CAT(ENVIRONMENT_FILLER_,{}))
#define P_FORGED_CONSTANT(x) PRESCAN(CAT3(__pgrx_,c_expand_0_,c{fillers}))
"#,
        fillers - 1
    );
    std::fs::write(&header, definitions).unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &["-std=c17".into()], None).unwrap();
    let names = frontend.environment().active.keys().map(|name| name.len() + 1).sum::<usize>();
    assert!(names > ExpansionLimits::default().source_bytes, "{names} name bytes");
    let batch = prepare_expansions(
        &scanner,
        &frontend,
        &["P_CLOSED", "P_SYNTHESIZED", "P_FORGED_CONSTANT"],
    )
    .unwrap();
    assert!(body(expanded(&batch, "P_CLOSED")).contains(&"0xFFU"));
    let synthesized = expanded(&batch, "P_SYNTHESIZED");
    assert!(synthesized.dependencies.iter().any(|dependency| dependency.name == last));
    assert_skip(&batch, "P_FORGED_CONSTANT", ExpansionSkipCode::TokenPaste);
}

/// Clang's `-E` output starts with every declaration in the inspected headers, so it grows with
/// the environment, like the paste proof's poison table. Here the headers alone print more than
/// the expansion byte budget, yet that budget still bounds only probe output: a closed paste
/// expands, while an operand paste and a forged constant marker are still refused.
#[test]
fn closed_paste_proof_budgets_probe_output_rather_than_header_output() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("environment.h");
    // Each filler line prints as eight 66-byte literals.
    let mut environment = format!(
        "#define FILL \"{}\"\n#define FILL8 FILL FILL FILL FILL FILL FILL FILL FILL\nstatic const char environment_filler[] =\n",
        "x".repeat(64)
    );
    environment.push_str(&"FILL8\n".repeat(20_000));
    environment.push_str(";\n");
    std::fs::write(directory.0.join("fillers.h"), environment).unwrap();
    std::fs::write(
        &header,
        r#"
#include "fillers.h"
#define CAT(a,b) a##b
#define CAT3(a,b,c) a##b##c
#define DROP(x) 0
#define PRESCAN(x) DROP(x)
#define P_CLOSED(x) ((x)+CAT(0xFF,U))
#define P_OPERAND(x) PRESCAN(CAT(x,_suffix))
#define P_FORGED_CONSTANT(x) PRESCAN(CAT3(__pgrx_,c_expand_0_,c0))
"#,
    )
    .unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &["-std=c17".into()], None).unwrap();
    let printed = std::process::Command::new(&frontend.profile().compiler.executable)
        .args(&frontend.profile().arguments)
        .args(["-E", "-P"])
        .arg(&header)
        .output()
        .unwrap();
    assert!(printed.status.success());
    let limit = ExpansionLimits::default().expanded_bytes;
    // The frontend refuses output over 16 MiB.
    assert!((limit..16 << 20).contains(&printed.stdout.len()), "{} bytes", printed.stdout.len());
    let batch =
        prepare_expansions(&scanner, &frontend, &["P_CLOSED", "P_OPERAND", "P_FORGED_CONSTANT"])
            .unwrap();
    assert!(body(expanded(&batch, "P_CLOSED")).contains(&"0xFFU"));
    assert_skip(&batch, "P_OPERAND", ExpansionSkipCode::TokenPaste);
    assert_skip(&batch, "P_FORGED_CONSTANT", ExpansionSkipCode::TokenPaste);
}

/// Under Microsoft extensions, which Clang enables by default for MSVC targets, a paste that
/// forms `//` comments out the rest of the invoking line, or inside a macro argument the rest of
/// that argument. A generated macro cannot erase code, so the proof refuses such pastes,
/// including one in a dependency. Clang reports them as errors through the probe pragmas. When
/// those errors are dropped, as `-w` does from Clang 9 on, no closed paste is admitted.
#[test]
fn closed_pastes_that_form_comments_are_refused_under_microsoft_extensions() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("comment.h");
    // P_ARGUMENT_COMMENT keeps its line and becomes a well-formed expression without `+ 2`.
    // Clang 14 to 19 and Apple clang 21 crash on a comment paste in an argument unless the
    // expansion ends with a function-like macro name, as this one does.
    std::fs::write(
        &header,
        r#"
#define CAT(a,b) a##b
#define ID(x) x
#define FUNCTION(x) x
#define SYNTHESIZED 7
#define COMMENT_OBJECT CAT(/,/) 3
#define P_COMMENT(x) ((x) + 1) CAT(/,/) + 2
#define P_COMMENT_DEPENDENCY(x) ((x) COMMENT_OBJECT + 2)
#define P_ARGUMENT_COMMENT(x) ((x) + ID(1 CAT(/,/) + 2)) + FUNCTION
#define P_CLOSED(x) ((x)+CAT(0xFF,U))
#define P_SYNTHESIZED(x) ((x)+CAT(SYNTHE,SIZED))
"#,
    )
    .unwrap();
    let scanner = MacroScanner::new().unwrap();
    let comments = ["P_COMMENT", "P_COMMENT_DEPENDENCY", "P_ARGUMENT_COMMENT"];
    let closed = ["P_CLOSED", "P_SYNTHESIZED"];
    let msvc = ["--target=x86_64-pc-windows-msvc", "-std=c17"];
    let explicit = ["-std=c17", "-fms-compatibility", "-fms-extensions"];
    let silenced = ["--target=x86_64-pc-windows-msvc", "-std=c17", "-w"];
    // Without Microsoft extensions `//` is an invalid paste, an error `-w` cannot drop.
    let no_extensions = ["-std=c17", "-w"];
    for arguments in [&msvc[..], &explicit, &silenced, &no_extensions] {
        let owned = arguments.iter().map(|argument| argument.to_string()).collect::<Vec<_>>();
        let frontend = inspect(&scanner, &header, &owned, None).unwrap();
        let batch =
            prepare_expansions(&scanner, &frontend, &[&comments[..], &closed].concat()).unwrap();
        let reported = reports_comment_pastes(&frontend, &directory.0);
        if arguments == silenced {
            let version = &frontend.profile().compiler.version;
            let major = version
                .split("version ")
                .nth(1)
                .and_then(|version| version.split('.').next()?.parse::<u32>().ok());
            // Clang 6 to 8 keep the error under -w, and Apple clang 11.0.0 is based on LLVM 8.
            let first = if version.starts_with("Apple ") { 12 } else { 9 };
            assert!(!reported || major.is_none_or(|major| major < first), "{version}");
        } else {
            assert!(reported, "{arguments:?}");
        }
        let unreported = "Clang does not report token pastes that form '//' comments under these compiler options";
        for name in comments {
            if !reported {
                assert_skip_reason(&batch, name, ExpansionSkipCode::TokenPaste, unreported);
            } else if arguments == no_extensions {
                assert_skip(&batch, name, ExpansionSkipCode::TokenPaste);
            } else {
                assert_skip_reason(
                    &batch,
                    name,
                    ExpansionSkipCode::TokenPaste,
                    "token pasting forms a '//' comment under Microsoft extensions",
                );
            }
        }
        if reported {
            assert!(body(expanded(&batch, "P_CLOSED")).contains(&"0xFFU"), "{arguments:?}");
            let synthesized = expanded(&batch, "P_SYNTHESIZED");
            assert!(
                synthesized.dependencies.iter().any(|dependency| dependency.name == "SYNTHESIZED")
            );
        } else {
            for name in closed {
                assert_skip_reason(&batch, name, ExpansionSkipCode::TokenPaste, unreported);
            }
        }
    }
}

/// Whether the profile's Clang reports a paste that forms `//` when a pragma makes the comment
/// paste an error, as the paste proof requires before it admits a closed paste.
fn reports_comment_pastes(frontend: &FrontendOutput, directory: &Path) -> bool {
    let source = directory.join("canary.c");
    std::fs::write(
        &source,
        "#pragma clang diagnostic error \"-Wmicrosoft-comment-paste\"\n#define CAT(a,b) a##b\nCAT(/,/)\n",
    )
    .unwrap();
    let output = std::process::Command::new(&frontend.profile().compiler.executable)
        .args(&frontend.profile().arguments)
        .args(["-E", "-P"])
        .arg(&source)
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stderr).contains("error: pasting")
}

/// Checks that the probes' own pragmas compile when warnings are errors. Clang 14 has no
/// `-Wgnu-line-marker` group, and naming it unguarded failed every probe under `-Werror` or
/// `-pedantic-errors`. The header is not a system header, which would silence that warning.
#[test]
fn probe_pragmas_compile_when_warnings_are_errors() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("strict.h");
    std::fs::write(
        &header,
        "#define CAT(a,b) a##b\n#define P_CLOSED(x) ((x)+CAT(0xFF,U))\n#define P_PLAIN(x) ((x)+1)\nint strict_declaration;\n",
    )
    .unwrap();
    let scanner = MacroScanner::new().unwrap();
    for arguments in [["-std=c17", "-Werror"], ["-std=c17", "-pedantic-errors"]] {
        let owned = arguments.iter().map(|argument| argument.to_string()).collect::<Vec<_>>();
        let frontend = inspect(&scanner, &header, &owned, None).unwrap();
        let batch = prepare_expansions(&scanner, &frontend, &["P_CLOSED", "P_PLAIN"]).unwrap();
        assert!(body(expanded(&batch, "P_CLOSED")).contains(&"0xFFU"), "{arguments:?}");
        assert!(body(expanded(&batch, "P_PLAIN")).contains(&"1"), "{arguments:?}");
    }
}

/// Checks that malformed paste probes cannot swallow a later operand validation.
#[test]
fn malformed_paste_probes_cannot_swallow_a_later_operand_validation() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("swallowed.h");
    std::fs::write(
        &header,
        r#"
#define CAT(a,b) a##b
#define ARG(x) x
#define OPEN() ARG(
#define DROP(x) 0
#define PRESCAN(x) DROP(x)
#define A_OPEN(x) CAT(0,U) OPEN()
#define B_ERASED(x) PRESCAN(CAT(x,_suffix))
#define C_HEALTHY(x) ((x)+CAT(1,U))
"#,
    )
    .unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let batch =
        prepare_expansions(&scanner, &frontend, &["A_OPEN", "B_ERASED", "C_HEALTHY"]).unwrap();
    assert_skip(&batch, "A_OPEN", ExpansionSkipCode::TokenPaste);
    assert_skip(&batch, "B_ERASED", ExpansionSkipCode::TokenPaste);
    assert_eq!(expanded(&batch, "C_HEALTHY").occurrences.len(), 1);
}

/// Checks that original main file context preserves macros and conditional declarations.
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

/// Checks that nested expansion does not capture an identifier named like an unused formal.
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

/// Checks that original symlink path keeps quoted include lookup and main file context.
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

/// Checks that stale declarations and new shadowing headers reject even with identical macro
/// maps.
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

/// Checks that appending probes cannot turn a trailing backslash into a new macro body.
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

/// Checks that snapshot line protection preserves original comments and literal backslashes.
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
