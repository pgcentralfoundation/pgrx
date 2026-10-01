//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;

use pgrx_c_macros::{
    AnalysisStatus, ConstCapability, FrontendError, InputConstraint, InvocationContract,
    MacroAnalysis, MacroScanner, ParameterRole, SkipReasonCode, analyze, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

fn assert_skip(analysis: &MacroAnalysis, expected: SkipReasonCode) {
    let AnalysisStatus::Skipped { reason } = &analysis.status else {
        panic!("{} must be skipped as {expected:?}: {analysis:?}", analysis.name);
    };
    assert_eq!(reason.code, expected, "{}: {}", analysis.name, reason.message);
    assert!(!reason.message.is_empty());
    if let Some(provenance) = &analysis.provenance {
        assert!(reason.spans.contains(provenance), "skip must retain its original physical span");
    }
    assert!(analysis.expression.is_none(), "skips must not look like analyzed candidates");
}

#[test]
fn complete_integer_expressions_are_candidates_with_inferred_value_constraints() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("analysis_scalar.h");
    let frontend = inspect(&scanner, &header, &[], None).expect("inspect original scalar header");
    for name in [
        "ANALYSIS_ID",
        "ANALYSIS_ADD",
        "ANALYSIS_UNARY",
        "ANALYSIS_PRECEDENCE",
        "ANALYSIS_CAST",
        "ANALYSIS_LITERAL",
        "ANALYSIS_ENUM",
        "ANALYSIS_BOOL_CAST",
        "ANALYSIS_UNUSED",
        "ANALYSIS_REPEAT",
        "ANALYSIS_COMMENT",
        "ANALYSIS_UNSIGNED_LITERAL",
        "ANALYSIS_LAZY",
        "ANALYSIS_CHOOSE",
        "ANALYSIS_CHAR",
    ] {
        let analysis = analyze(&frontend, name);
        assert!(matches!(analysis.status, AnalysisStatus::Candidate), "{name}: {analysis:?}");
        assert!(analysis.expression.is_some(), "{name} has a complete analyzed expression");
        assert_eq!(analysis.invocation, InvocationContract::ParenthesizedScalarExpressions);
        assert_eq!(analysis.const_capability, ConstCapability::NotEstablished);
        assert!(analysis.dependencies.is_empty());
        assert!(analysis.provenance.is_some());
        for parameter in &analysis.parameters {
            if parameter.uses.is_empty() {
                assert_eq!(parameter.roles, [ParameterRole::Unused]);
                assert_eq!(parameter.constraint, None);
            } else {
                assert_eq!(parameter.roles, [ParameterRole::Value]);
                assert_eq!(parameter.constraint, Some(InputConstraint::IntegerScalar));
                assert!(parameter.uses.iter().all(|usage| usage.grouped));
            }
        }
    }
    assert_eq!(analyze(&frontend, "ANALYSIS_REPEAT").parameters[0].uses.len(), 2);
    assert!(analyze(&frontend, "ANALYSIS_UNUSED").parameters[0].uses.is_empty());

    // Concrete instantiations are checked by C, without constructing an expected syntax tree.
    // In particular the cast result does not constrain the original input to FrontByte.
    let original = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        r#"
#define EXPECT_TYPE(value, type) _Static_assert(_Generic((value), type: 1, default: 0), #value)
EXPECT_TYPE(ANALYSIS_CAST(-1), FrontByte);
EXPECT_TYPE(ANALYSIS_BOOL_CAST(-1), _Bool);
EXPECT_TYPE(ANALYSIS_LITERAL(0U), unsigned long);
EXPECT_TYPE(ANALYSIS_UNSIGNED_LITERAL(0U), unsigned long long);
EXPECT_TYPE(ANALYSIS_ENUM(0), int);
_Static_assert(ANALYSIS_CAST(-1) == 255, "cast value");
_Static_assert(ANALYSIS_BOOL_CAST(-1) == 1, "bool value");
_Static_assert(ANALYSIS_LITERAL(0U) == 255UL, "unsigned long value");
_Static_assert(ANALYSIS_UNSIGNED_LITERAL(0U) == 18446744073709551615ULL, "unsigned long long value");
_Static_assert(ANALYSIS_ENUM(0) == 7, "enum constant value");
_Static_assert(ANALYSIS_PRECEDENCE(4) == 20, "operator precedence");
"#,
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(original.is_empty());
}

#[test]
fn unsupported_forms_have_stable_reasons_and_original_source_spans() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &fixture("analysis_scalar.h"), &[], None).unwrap();
    for (name, code) in [
        ("ANALYSIS_UNGROUPED", SkipReasonCode::InvocationGrouping),
        ("ANALYSIS_ROOT_UNGROUPED", SkipReasonCode::InvocationGrouping),
        ("ANALYSIS_POINTER", SkipReasonCode::PointerOperation),
        ("ANALYSIS_MEMBER", SkipReasonCode::PointerOperation),
        ("ANALYSIS_SUBSCRIPT", SkipReasonCode::PointerOperation),
        ("ANALYSIS_MUTATION", SkipReasonCode::Mutation),
        ("ANALYSIS_ASSIGN", SkipReasonCode::Mutation),
        ("ANALYSIS_CALL", SkipReasonCode::Call),
        ("ANALYSIS_STATEMENT", SkipReasonCode::Statement),
        ("ANALYSIS_SIZEOF", SkipReasonCode::UnevaluatedExpression),
        ("ANALYSIS_TYPE_PARAMETER", SkipReasonCode::TypeParameter),
        ("ANALYSIS_COMMA", SkipReasonCode::CommaExpression),
        ("ANALYSIS_FLOAT", SkipReasonCode::UnsupportedLiteral),
        ("ANALYSIS_UNKNOWN", SkipReasonCode::UnknownIdentifier),
        ("ANALYSIS_VARIABLE", SkipReasonCode::VariableAccess),
        ("ANALYSIS_EMPTY", SkipReasonCode::EmptyReplacement),
        ("ANALYSIS_STRINGIFY", SkipReasonCode::Stringification),
        ("ANALYSIS_PASTE", SkipReasonCode::TokenPaste),
        ("ANALYSIS_VARIADIC", SkipReasonCode::Variadic),
        ("ANALYSIS_AMBIGUOUS", SkipReasonCode::ProvenanceAmbiguous),
        ("ANALYSIS_OBJECT", SkipReasonCode::NotFunctionLike),
        ("ANALYSIS_LITERAL_TOO_LARGE", SkipReasonCode::LiteralOutOfRange),
    ] {
        assert_skip(&analyze(&frontend, name), code);
    }
    let absent = analyze(&frontend, "ANALYSIS_NOT_DEFINED");
    assert_skip(&absent, SkipReasonCode::NotActive);
    assert!(absent.provenance.is_none());

    let deep = inspect(&scanner, &fixture("analysis_budget.h"), &[], None).unwrap();
    assert_skip(&analyze(&deep, "ANALYSIS_DEEP"), SkipReasonCode::BudgetExceeded);
}

#[test]
fn dependencies_require_real_preprocessing_and_use_the_final_active_context() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("analysis_scalar.h");
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    for (name, dependency) in [
        ("ANALYSIS_LATE_USER", "ANALYSIS_LATE_VALUE"),
        ("ANALYSIS_NESTED", "ANALYSIS_ADD"),
        ("ANALYSIS_RECURSIVE", "ANALYSIS_RECURSIVE"),
        ("ANALYSIS_CYCLE", "ANALYSIS_CYCLE_A"),
    ] {
        let analysis = analyze(&frontend, name);
        assert_skip(&analysis, SkipReasonCode::ExpansionRequired);
        let observed = analysis.dependencies.iter().find(|item| item.name == dependency).unwrap();
        assert_eq!(
            observed.provenance,
            frontend.environment().active[dependency].definition.provenance
        );
        assert!(!observed.uses.is_empty());
    }
    let original = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        "_Static_assert(ANALYSIS_LATE_USER(10) == 19, \"late-bound object macro\");\n_Static_assert(ANALYSIS_NESTED(4) == 7, \"nested function macro\");\n_Static_assert(ANALYSIS_UNGROUPED(1 + 2) == 5, \"argument grouping affects the body\");\n_Static_assert(ANALYSIS_ROOT_UNGROUPED(2) * 3 == 5, \"whole replacement grouping affects the caller\");\n",
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(original.is_empty());
}

#[test]
fn binary_literals_require_c23_and_compile_under_the_original_c23_profile() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("analysis_profile.h");
    let c11 = inspect(&scanner, &header, &["-std=c11".into(), "-pedantic-errors".into()], None)
        .expect("inspect original definitions under strict C11");
    assert_skip(&analyze(&c11, "ANALYSIS_BINARY"), SkipReasonCode::UnsupportedLiteral);

    let c23 = match inspect(
        &scanner,
        &header,
        &["-std=c23".into(), "-pedantic-errors".into()],
        None,
    ) {
        Ok(frontend) => frontend,
        Err(FrontendError::CompilerFailed { diagnostics, .. })
            if diagnostics
                .lines()
                .any(|line| line.ends_with("error: invalid value 'c23' in '-std=c23'")) =>
        {
            eprintln!(
                "C23 positive probe unavailable: {} rejects -std=c23; strict C11 regression passed",
                c11.profile().compiler.executable.display()
            );
            return;
        }
        Err(error) => panic!("inspect original definitions under strict C23: {error}"),
    };
    assert!(matches!(analyze(&c23, "ANALYSIS_BINARY").status, AnalysisStatus::Candidate));
    let checked = oracle::run_c(
        &c23.profile().compiler.executable,
        &header,
        "_Static_assert(ANALYSIS_BINARY(1) == 2, \"binary literals are valid in C23\");\n",
        &c23.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(checked.is_empty());
}

#[test]
fn original_c_shadowing_demonstrates_the_fixed_binding_invocation_boundary() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("analysis_profile.h");
    let frontend = inspect(&scanner, &header, &[], None).expect("inspect header bindings");
    for name in ["ANALYSIS_BINDING_ENUM", "ANALYSIS_BINDING_TYPE"] {
        let analysis = analyze(&frontend, name);
        assert!(matches!(analysis.status, AnalysisStatus::Candidate), "{analysis:?}");
        assert_eq!(analysis.invocation, InvocationContract::ParenthesizedScalarExpressions);
    }
    let checked = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        r#"
_Static_assert(ANALYSIS_BINDING_ENUM(0) == 7, "inspected enum binding");
_Static_assert(_Generic(ANALYSIS_BINDING_TYPE(2), int: 1, default: 0), "inspected typedef binding");

static unsigned long shadow_function(int value) { return (unsigned long) value + 100UL; }
void check_caller_scope(void) {
    enum { ANALYSIS_HEADER_ENUM = 99 };
    unsigned long (*AnalysisHeaderType)(int) = shadow_function;
    _Static_assert(ANALYSIS_BINDING_ENUM(0) == 99, "caller-local enum changes the value");
    _Static_assert(_Generic(ANALYSIS_BINDING_TYPE(2), unsigned long: 1, default: 0),
                   "caller-local identifier turns the typedef cast into a function call");
}
"#,
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(checked.is_empty());
}

#[test]
fn attributed_integer_typedefs_and_their_aliases_need_a_separate_semantic_contract() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &fixture("analysis_profile.h"), &[], None).unwrap();
    for name in ["ANALYSIS_ATTRIBUTED", "ANALYSIS_ATTRIBUTED_ALIAS"] {
        assert_skip(&analyze(&frontend, name), SkipReasonCode::UnsupportedType);
    }
}
