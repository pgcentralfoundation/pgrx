//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;

use pgrx_c_macros::{
    AnalysisSession, AnalysisStatus, ConstCapability, EmissionStatus, ExpressionKind,
    FrontendError, InputConstraint, InvocationContract, MacroAnalysis, MacroScanner, ParameterRole,
    SkipReasonCode, TypeCategory, TypeExpression, analyze, emit, inspect,
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
        "ANALYSIS_COMMA",
        "ANALYSIS_EMPTY",
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
    let empty = analyze(&frontend, "ANALYSIS_EMPTY");
    let empty = empty.expression.unwrap();
    assert!(matches!(empty.syntax.nodes[empty.syntax.root].kind, ExpressionKind::Empty));
    assert!(matches!(empty.types[empty.syntax.root], TypeExpression::Void));
    let comma = analyze(&frontend, "ANALYSIS_COMMA");
    let comma = comma.expression.unwrap();
    assert!(
        comma.syntax.nodes.iter().any(|node| matches!(node.kind, ExpressionKind::Comma { .. }))
    );
    assert!(matches!(comma.types[comma.syntax.root], TypeExpression::Parameter { index: 1 }));

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
        ("ANALYSIS_CONTROL_FLOW", SkipReasonCode::Statement),
        ("ANALYSIS_SIZEOF", SkipReasonCode::InvocationGrouping),
        ("ANALYSIS_FLOAT", SkipReasonCode::UnsupportedLiteral),
        ("ANALYSIS_UNKNOWN", SkipReasonCode::UnknownIdentifier),
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

    let statement = analyze(&frontend, "ANALYSIS_STATEMENT");
    assert!(matches!(statement.status, AnalysisStatus::Candidate));
    assert_eq!(statement.invocation, InvocationContract::Statements);
    assert_eq!(statement.const_capability, ConstCapability::RuntimeOnly);
    let expression = statement.expression.unwrap();
    assert!(expression.syntax.statement_body.is_some());
    assert!(matches!(expression.types[expression.syntax.root], TypeExpression::Void));

    let deep = inspect(&scanner, &fixture("analysis_budget.h"), &[], None).unwrap();
    assert_skip(&analyze(&deep, "ANALYSIS_DEEP"), SkipReasonCode::BudgetExceeded);
}

#[test]
fn typed_expression_candidates_preserve_parameters_and_original_c_expression_types() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("analysis_scalar.h");
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    for name in [
        "ANALYSIS_POINTER",
        "ANALYSIS_MEMBER",
        "ANALYSIS_SUBSCRIPT",
        "ANALYSIS_MUTATION",
        "ANALYSIS_ASSIGN",
        "ANALYSIS_VARIABLE",
        "ANALYSIS_CALL",
    ] {
        let analysis = analyze(&frontend, name);
        assert!(matches!(analysis.status, AnalysisStatus::Candidate), "{name}: {analysis:?}");
        let expression = analysis.expression.as_ref().expect("candidate retains its shared IR");
        assert!(
            expression.syntax.nodes.iter().any(|node| match (&node.kind, name) {
                (ExpressionKind::Dereference { .. }, "ANALYSIS_POINTER") => true,
                (ExpressionKind::Member { field, indirect: true, .. }, "ANALYSIS_MEMBER") =>
                    field == "field",
                (ExpressionKind::Index { .. }, "ANALYSIS_SUBSCRIPT") => true,
                (
                    ExpressionKind::Update { increment: true, postfix: false, .. },
                    "ANALYSIS_MUTATION",
                ) => true,
                (ExpressionKind::Assignment { operator: None, .. }, "ANALYSIS_ASSIGN") => true,
                (ExpressionKind::Identifier { name: symbol }, "ANALYSIS_VARIABLE") =>
                    symbol == "front_const_variable",
                (ExpressionKind::Call { arguments, .. }, "ANALYSIS_CALL") => arguments.len() == 1,
                _ => false,
            }),
            "{name} must preserve its C operation in the shared IR"
        );
        if matches!(name, "ANALYSIS_POINTER" | "ANALYSIS_MEMBER" | "ANALYSIS_SUBSCRIPT") {
            assert!(matches!(expression.types[expression.syntax.root], TypeExpression::Deferred));
        }
        assert_eq!(analysis.parameters.len(), 1);
        assert!(analysis.parameters[0].uses.iter().all(|usage| usage.grouped));
        assert_eq!(analysis.invocation, InvocationContract::ParenthesizedScalarExpressions);
    }
    let checked = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        r#"
#define TYPE_IS(value, type) _Static_assert(_Generic((value), type: 1, default: 0), #value)
void verify_original_types(void) {
    int value = 0;
    int array[3] = {0};
    struct { int field; } record = {0};
    TYPE_IS(ANALYSIS_POINTER(&value), int);
    TYPE_IS(ANALYSIS_MEMBER(&record), int);
    TYPE_IS(ANALYSIS_SUBSCRIPT(array), int);
    TYPE_IS(ANALYSIS_MUTATION(value), int);
    TYPE_IS(ANALYSIS_ASSIGN(value), int);
    TYPE_IS(ANALYSIS_VARIABLE(value), int);
    TYPE_IS(ANALYSIS_CALL(value), int);
}

"#,
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(checked.is_empty());
}

#[test]
fn type_argument_casts_retain_deferred_types_and_concrete_pointer_casts_use_compiler_facts() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let scalar = inspect(&scanner, &fixture("analysis_scalar.h"), &[], None).unwrap();
    let analysis = analyze(&scalar, "ANALYSIS_TYPE_PARAMETER");
    assert!(matches!(analysis.status, AnalysisStatus::Candidate), "{analysis:?}");
    let expression = analysis.expression.as_ref().expect("candidate retains its shared IR");
    let (cast, operand) = expression
        .syntax
        .nodes
        .iter()
        .enumerate()
        .find_map(|(index, node)| match node.kind {
            ExpressionKind::TypeParameterCast {
                parameter: 0,
                pointers: 0,
                is_const: false,
                operand,
            } => Some((index, operand)),
            _ => None,
        })
        .expect("the type argument remains a symbolic cast, not a sampled signature");
    assert!(matches!(expression.types[cast], TypeExpression::Deferred));
    assert!(matches!(expression.syntax.nodes[operand].kind, ExpressionKind::Group { .. }));
    assert_eq!(analysis.parameters[0].constraint, None);
    assert_eq!(analysis.parameters[0].roles, [ParameterRole::Type]);
    assert_eq!(analysis.parameters[0].uses.len(), 1);
    assert_eq!(analysis.parameters[1].uses.len(), 1);
    assert!(analysis.parameters[1].uses[0].grouped);

    let header = fixture("expression_oracle.h");
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let analysis = analyze(&frontend, "EXPR_POINTER_CAST");
    assert!(matches!(analysis.status, AnalysisStatus::Candidate), "{analysis:?}");
    let expression = analysis.expression.as_ref().unwrap();
    let TypeExpression::External { ty } = &expression.types[expression.syntax.root] else {
        panic!("the concrete pointer cast needs its original compiler type: {expression:?}");
    };
    assert_eq!(ty.category, TypeCategory::Pointer);
    assert_eq!(ty.canonical_spelling, "char *");
    let checked = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        "_Static_assert(_Generic(EXPR_POINTER_CAST((void *) 0), char *: 1, default: 0), \"original cast type\");\n",
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(checked.is_empty());
}

#[test]
fn ungrouped_parameter_uses_require_atomic_arguments_without_changing_the_c_body() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("expression_oracle.h");
    let frontend = inspect(&scanner, &header, &[], None).expect("inspect original atomic macro");
    let analysis = analyze(&frontend, "EXPR_ATOMIC_POW2");
    assert!(matches!(analysis.status, AnalysisStatus::Candidate), "{analysis:?}");
    assert_eq!(analysis.invocation, InvocationContract::AtomicArguments);
    assert_eq!(analysis.parameters[0].constraint, Some(InputConstraint::IntegerScalar));
    assert_eq!(analysis.parameters[0].uses.len(), 3);
    assert!(!analysis.parameters[0].uses[0].grouped);
    assert!(analysis.parameters[0].uses[1..].iter().all(|usage| usage.grouped));

    // Unrestricted C substitution really changes grouping, even though the entire
    // macro body is parenthesized. The Rust matcher must reject this argument shape.
    let checked = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        r#"
_Static_assert(EXPR_ATOMIC_POW2(1 ? 7 : 2) != EXPR_ATOMIC_POW2((1 ? 7 : 2)), "textual argument grouping affects semantics");
_Static_assert(EXPR_ATOMIC_POW2(16) == EXPR_ATOMIC_POW2((16)), "atomic grouping preserves semantics");
"#,
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
    assert!(checked.is_empty());
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
    for name in ["AnalysisAttributed", "AnalysisAttributedAlias"] {
        assert_eq!(frontend.declarations().types[name].category, TypeCategory::Other);
    }
    let names = ["ANALYSIS_ATTRIBUTED", "ANALYSIS_ATTRIBUTED_ALIAS"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    for name in names {
        assert_skip(&analyze(&frontend, name), SkipReasonCode::UnsupportedType);
        let emission = emit(&session, name);
        let EmissionStatus::Skipped { reason } = emission.status else {
            panic!("unmodeled attributed typedef {name} must not emit: {emission:?}");
        };
        assert_eq!(reason.code, SkipReasonCode::UnsupportedType, "{name}: {reason:?}");
    }
}
