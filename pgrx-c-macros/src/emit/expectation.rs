//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Plan compiler expectations without changing expression evaluation or C values.
//!
//! The builtin returns its first operand, even when its second operand cannot
//! provide a static optimization hint. Planning separates fixed expected values from
//! effect-free evaluation, carries advice through supported casts, and lets an
//! enclosing expectation override inner advice without removing either operand.

use crate::analysis::{MacroAnalysis, resolve_type_info};
use crate::syntax::{ExpressionKind, NodeId, OffsetRecord};
use crate::{BuiltinKind, FrontendOutput, TypeCategory};

/// Advice classification for one expression node, separate from the evaluation it still requires.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Decision {
    /// Expected value is fixed at compilation or instantiation and can carry branch advice.
    Static,
    /// No fixed expectation is proved; preserve both operand evaluations without static advice.
    Dynamic,
    /// An enclosing expectation owns the advice for this projected value.
    Overridden,
}

/// Decisions parallel the expression arena. An empty plan means no verified expect
/// builtin occurs. A fixed value can still have effects: the renderer must evaluate
/// both operands even when it suppresses or cannot establish an optimization hint.
pub(super) fn plan(frontend: &FrontendOutput, analysis: &MacroAnalysis) -> Vec<Decision> {
    let Some(expression) = &analysis.expression else {
        return Vec::new();
    };
    let catalog = frontend.declarations();
    let nodes = &expression.syntax.nodes;
    if !nodes.iter().any(|node| {
        matches!(&node.kind, ExpressionKind::Identifier { name }
            if catalog.builtins.get(name).is_some_and(|builtin| builtin.kind == BuiltinKind::Expect))
    }) {
        return Vec::new();
    }

    let target = &frontend.profile().target;
    let mut decisions = vec![Decision::Dynamic; nodes.len()];
    let mut fixed = vec![false; nodes.len()];
    for constant in &expression.constants {
        fixed[constant.node] = true;
    }
    let mut ungrouped = Vec::with_capacity(nodes.len());
    let mut projected_expectation: Vec<Option<NodeId>> = Vec::with_capacity(nodes.len());

    // Children precede parents. Each edge is inspected once; grouped callees and
    // cast projections never require rescanning a growing expression chain.
    for (index, node) in nodes.iter().enumerate() {
        ungrouped.push(match node.kind {
            ExpressionKind::Group { operand } => ungrouped[operand],
            _ => index,
        });
        let expect = match &node.kind {
            ExpressionKind::Call { callee, arguments } if arguments.len() == 2 => {
                matches!(&nodes[ungrouped[*callee]].kind, ExpressionKind::Identifier { name }
                    if catalog.builtins.get(name).is_some_and(|builtin| builtin.kind == BuiltinKind::Expect))
            }
            _ => false,
        };
        fixed[index] = match &node.kind {
            ExpressionKind::IntegerLiteral { .. } => true,
            ExpressionKind::Identifier { .. } => fixed[index],
            ExpressionKind::Group { operand }
            | ExpressionKind::Cast { operand, .. }
            | ExpressionKind::TypeParameterCast { operand, .. }
            | ExpressionKind::Unary { operand, .. } => fixed[*operand],
            ExpressionKind::Binary { left, right, .. } => fixed[*left] && fixed[*right],
            ExpressionKind::Conditional { condition, then_value, else_value } => {
                fixed[*condition] && fixed[*then_value] && fixed[*else_value]
            }
            ExpressionKind::Comma { right, .. } => fixed[*right],
            ExpressionKind::Call { arguments, .. } if expect => {
                if fixed[arguments[1]] {
                    decisions[index] = Decision::Static;
                }
                // The expected operand can be impure or runtime-dependent while
                // the builtin's returned first operand has a fixed value.
                fixed[arguments[0]]
            }
            ExpressionKind::SizeOfType { type_name }
            | ExpressionKind::AlignOfType { type_name } => {
                resolve_type_info(type_name, catalog, target).is_some_and(|ty| {
                    ty.size.is_some()
                        && !matches!(ty.category, TypeCategory::Void | TypeCategory::Function)
                })
            }
            // Every emitted sizeof context requires CompleteObject, including
            // deferred values and places. NativeType admits only fixed storage;
            // variably modified C types have no Rust capability.
            ExpressionKind::SizeOfExpression { .. }
            | ExpressionKind::SizeOfTypeParameter { .. }
            | ExpressionKind::AlignOfTypeParameter { .. } => true,
            ExpressionKind::OffsetOf { record, .. } => {
                // Field holes select generated OffsetField capabilities. Every
                // admitted path is an unevaluated associated constant, including
                // when its record or field identity is supplied at invocation.
                target.offsetof_supported
                    && match record {
                        OffsetRecord::Named { name } => resolve_type_info(name, catalog, target)
                            .is_some_and(|ty| {
                                ty.category == TypeCategory::Record && ty.size.is_some()
                            }),
                        OffsetRecord::Parameter { .. } => true,
                    }
            }
            _ => false,
        };
        projected_expectation.push(match &node.kind {
            ExpressionKind::Call { .. } if expect => Some(index),
            ExpressionKind::Group { operand }
            | ExpressionKind::Cast { operand, .. }
            | ExpressionKind::TypeParameterCast { operand, .. } => projected_expectation[*operand],
            _ => None,
        });
    }

    // An enclosing expectation owns the advice for its cast-projected value;
    // a dynamic outer expected value also cancels an inner static hint. Expect
    // is an evaluated identity operation, so suppressing inner advice preserves
    // C values and effects even through narrowing, floating or pointer casts.
    // The renderer retains every operand, cast and implicit conversion. Reverse
    // order propagates this priority through each direct expectation chain.
    for index in (0..nodes.len()).rev() {
        if projected_expectation[index] == Some(index)
            && let ExpressionKind::Call { arguments, .. } = &nodes[index].kind
            && let Some(inner) = projected_expectation[arguments[0]]
        {
            decisions[inner] = Decision::Overridden;
        }
    }
    decisions
}

/// Arena-planning regressions for fixed expectations, nested advice, casts, and unevaluated capabilities.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ActiveMacro, ActiveProvenance, AnalysisStatus, BuildInputs, BuiltinInfo, ByteOrder,
        CompilationProfile, CompilerIdentity, DeclarationCatalog, FunctionInfo, FunctionSignature,
        IntegerConstant, IntegerKind, IntegerType, IntegerValue, MacroDefinition,
        MacroDependencyGraph, MacroEnvironment, MacroInventory, MacroKind, PointerLayout,
        SignedOverflow, TargetFacts, Token, TokenKind, TypeInfo, analyze,
    };

    /// Build a trusted synthetic macro frontend with verified builtin and target type facts.
    fn frontend(body: &[&str]) -> FrontendOutput {
        let integers = [
            (IntegerKind::Bool, 8, false, 0),
            (IntegerKind::Char, 8, true, 1),
            (IntegerKind::SignedChar, 8, true, 1),
            (IntegerKind::UnsignedChar, 8, false, 1),
            (IntegerKind::Short, 16, true, 2),
            (IntegerKind::UnsignedShort, 16, false, 2),
            (IntegerKind::Int, 32, true, 3),
            (IntegerKind::UnsignedInt, 32, false, 3),
            (IntegerKind::Long, 64, true, 4),
            (IntegerKind::UnsignedLong, 64, false, 4),
            (IntegerKind::LongLong, 64, true, 5),
            (IntegerKind::UnsignedLongLong, 64, false, 5),
            (IntegerKind::Int128, 128, true, 6),
            (IntegerKind::UnsignedInt128, 128, false, 6),
        ]
        .into_iter()
        .map(|(kind, bits, signed, rank)| (kind, IntegerType { kind, bits, signed, rank }))
        .collect();
        let tokens = ["F", "(", "x", ",", "y", ")"]
            .into_iter()
            .chain(body.iter().copied())
            .map(|spelling| Token {
                spelling: spelling.into(),
                kind: if spelling.as_bytes().first().is_some_and(u8::is_ascii_digit) {
                    TokenKind::Literal
                } else if spelling.starts_with('_')
                    || spelling.as_bytes().first().is_some_and(u8::is_ascii_alphabetic)
                {
                    TokenKind::Identifier
                } else {
                    TokenKind::Punctuation
                },
            })
            .collect();
        let environment = MacroEnvironment {
            active: [(
                "F".into(),
                ActiveMacro {
                    definition: MacroDefinition {
                        name: "F".into(),
                        kind: MacroKind::FunctionLike,
                        location: None,
                        provenance: None,
                        tokens,
                        builtin: false,
                        main_file: true,
                    },
                    provenance: ActiveProvenance::Resolved,
                },
            )]
            .into_iter()
            .collect(),
        };
        let long = TypeInfo {
            spelling: "long".into(),
            canonical_spelling: "long".into(),
            category: TypeCategory::Integer(IntegerKind::Long),
            size: Some(8),
            alignment: Some(8),
            is_const: false,
            is_volatile: false,
        };
        let signature = FunctionSignature {
            result: long.clone(),
            parameters: Some(vec![long.clone(), long.clone()]),
            variadic: false,
            calling_convention: Some("Cdecl".into()),
        };
        let mut declarations = DeclarationCatalog::default();
        declarations.builtins.insert(
            "__builtin_expect".into(),
            BuiltinInfo { kind: BuiltinKind::Expect, signature: signature.clone() },
        );
        declarations.function_signatures.insert(
            "hint".into(),
            FunctionInfo {
                signature: FunctionSignature { parameters: Some(Vec::new()), ..signature },
                linkage: None,
                is_static: false,
                is_inline: false,
                definition_available: false,
            },
        );
        declarations.functions.insert(
            "hint".into(),
            TypeInfo {
                spelling: "long (void)".into(),
                canonical_spelling: "long (void)".into(),
                category: TypeCategory::Function,
                size: None,
                alignment: None,
                ..long.clone()
            },
        );
        declarations.types.insert("LongAlias".into(), long.clone());
        declarations.types.insert("long".into(), long.clone());
        declarations.types.insert(
            "Record".into(),
            TypeInfo {
                spelling: "Record".into(),
                canonical_spelling: "struct Record".into(),
                category: TypeCategory::Record,
                size: Some(16),
                alignment: Some(8),
                ..long.clone()
            },
        );
        declarations.types.insert(
            "double".into(),
            TypeInfo {
                spelling: "double".into(),
                canonical_spelling: "double".into(),
                category: TypeCategory::Floating,
                ..long.clone()
            },
        );
        declarations.variables.insert("flag".into(), long.clone());
        declarations.integer_constants.insert(
            "FLAG".into(),
            IntegerConstant { ty: long, value: IntegerValue::Signed(1), literal: None },
        );
        FrontendOutput {
            profile: CompilationProfile {
                header: "fixture.h".into(),
                compiler: CompilerIdentity {
                    executable: "clang".into(),
                    version: "fixture".into(),
                    libclang_version: "fixture".into(),
                },
                arguments: Vec::new(),
                target: TargetFacts {
                    triple: "fixture".into(),
                    pointer_bits: 64,
                    function_pointer: PointerLayout { size: 8, alignment: 8 },
                    size_type: IntegerKind::UnsignedLong,
                    offsetof_supported: true,
                    char_bits: 8,
                    char_is_signed: true,
                    ascii_execution_charset: true,
                    byte_order: ByteOrder::Little,
                    c_standard: Some(201710),
                    integers,
                    floating_point: Default::default(),
                },
                signed_overflow: SignedOverflow::Wrapping,
                unsupported_options: Vec::new(),
                inputs: BuildInputs::default(),
            },
            dependencies: MacroDependencyGraph::from_environment(&environment),
            environment,
            declarations,
            inventory: MacroInventory { macros: Vec::new(), diagnostics: Vec::new() },
        }
    }

    /// Analyze the fixture and extract decisions at each verified expectation call.
    fn decisions(body: &[&str]) -> Vec<Decision> {
        let frontend = frontend(body);
        let analysis = analyze(&frontend, "F");
        assert!(matches!(analysis.status, AnalysisStatus::Candidate), "{analysis:?}");
        let planned = plan(&frontend, &analysis);
        analysis
            .expression
            .unwrap()
            .syntax
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(index, node)| {
                matches!(node.kind, ExpressionKind::Call { .. }).then_some(planned[index])
            })
            .collect()
    }

    /// Check fixed expected values can include comma effects or compound constants without requiring purity.
    #[test]
    fn fixed_expected_values_include_effectful_comma_and_compound_constants() {
        for expected in [
            vec!["0"],
            vec!["FLAG"],
            vec!["(", "long", ")", "-", "1"],
            vec!["!", "0"],
            vec!["(", "1", "+", "2", ")"],
            vec!["(", "1", "?", "2", ":", "3", ")"],
            vec!["sizeof", "(", "long", ")"],
            vec!["_Alignof", "(", "long", ")"],
            vec!["sizeof", "(", "(", "long", ")", "(", "y", ")", ")"],
            vec!["sizeof", "(", "(", "y", ")", ")"],
        ] {
            let mut body = vec!["__builtin_expect", "(", "(", "x", ")", ","];
            body.extend(expected);
            body.push(")");
            assert_eq!(decisions(&body), [Decision::Static], "{body:?}");
        }
        assert_eq!(
            decisions(&[
                "__builtin_expect",
                "(",
                "(",
                "x",
                ")",
                ",",
                "(",
                "hint",
                "(",
                ")",
                ",",
                "0",
                ")",
                ")"
            ]),
            [Decision::Dynamic, Decision::Static],
        );
    }

    /// Check generic casts and sizeof, alignment, and offset capabilities yield fixed instantiation-time expectations.
    #[test]
    fn generic_casts_and_unevaluated_capabilities_are_fixed_at_instantiation() {
        for expected in [
            vec!["(", "y", ")", "0"],
            vec!["sizeof", "(", "y", ")"],
            vec!["sizeof", "(", "y", "*", ")"],
            vec!["_Alignof", "(", "y", ")"],
            vec!["(", "_Alignof", "(", "y", ")", ",", "sizeof", "(", "y", ")", ")"],
            vec!["__builtin_offsetof", "(", "y", ",", "field", ")"],
            vec!["__builtin_offsetof", "(", "Record", ",", "y", ")"],
        ] {
            let mut body = vec!["__builtin_expect", "(", "(", "x", ")", ","];
            body.extend(expected);
            body.push(")");
            assert_eq!(decisions(&body), [Decision::Static], "{body:?}");
        }
        assert_eq!(
            decisions(&[
                "__builtin_expect",
                "(",
                "(",
                "x",
                ")",
                ",",
                "(",
                "sizeof",
                "(",
                "y",
                "*",
                ")",
                ",",
                "(",
                "y",
                ")",
                "(",
                "hint",
                "(",
                ")",
                ",",
                "0",
                ")",
                ")",
                ")",
            ]),
            [Decision::Dynamic, Decision::Static],
            "a fixed generic cast still evaluates its effectful operand"
        );
        assert_eq!(
            decisions(&["__builtin_expect", "(", "(", "x", ")", ",", "(", "y", ")", "x", ")"]),
            [Decision::Dynamic],
            "a generic cast of a runtime operand does not invent fixedness"
        );
    }

    /// Check unknown caller values cannot become static optimization advice.
    #[test]
    fn unknown_expected_values_remain_dynamic() {
        for expected in [
            vec!["(", "y", ")"],
            vec!["flag"],
            vec!["hint", "(", ")"],
            vec!["(", "(", "y", ")", "?", "1", ":", "2", ")"],
        ] {
            let mut body = vec!["__builtin_expect", "(", "(", "x", ")", ","];
            body.extend(expected);
            body.push(")");
            assert!(
                decisions(&body).iter().all(|decision| *decision == Decision::Dynamic),
                "{body:?}"
            );
        }
    }

    /// Check outer expectations supersede inner advice through groups and supported cast projections.
    #[test]
    fn outer_hint_overrides_nested_calls_through_groups_and_bit_preserving_casts() {
        for cast in [
            vec![],
            vec!["(", "long", ")"],
            vec!["(", "LongAlias", ")"],
            vec!["(", "long", "long", ")"],
            vec!["(", "unsigned", "long", ")"],
            vec!["(", "unsigned", "long", "long", ")"],
            vec!["(", "__int128", ")"],
            vec!["(", "unsigned", "__int128", ")"],
            vec!["(", "long", "long", ")", "(", "unsigned", "__int128", ")"],
        ] {
            let mut body = vec!["(", "(", "__builtin_expect", ")", ")", "("];
            body.extend(cast);
            body.extend([
                "(",
                "__builtin_expect",
                "(",
                "(",
                "x",
                ")",
                ",",
                "1",
                ")",
                ")",
                ",",
                "0",
                ")",
            ]);
            assert_eq!(decisions(&body), [Decision::Overridden, Decision::Static], "{body:?}");
        }
        assert_eq!(
            decisions(&[
                "__builtin_expect",
                "(",
                "__builtin_expect",
                "(",
                "__builtin_expect",
                "(",
                "(",
                "x",
                ")",
                ",",
                "1",
                ")",
                ",",
                "hint",
                "(",
                ")",
                ")",
                ",",
                "0",
                ")"
            ]),
            [Decision::Overridden, Decision::Dynamic, Decision::Overridden, Decision::Static],
        );
    }

    /// Check cast-induced value changes do not prevent an outer expectation from owning advice.
    #[test]
    fn outer_hint_overrides_inner_advice_even_when_casts_change_values() {
        for cast in [vec!["int"], vec!["unsigned", "int"], vec!["_Bool"], vec!["double"], vec!["y"]]
        {
            let mut body = vec!["__builtin_expect", "(", "("];
            body.extend(cast);
            body.extend([
                ")",
                "__builtin_expect",
                "(",
                "(",
                "x",
                ")",
                ",",
                "1",
                ")",
                ",",
                "0",
                ")",
            ]);
            assert_eq!(decisions(&body), [Decision::Overridden, Decision::Static], "{body:?}");
        }
        assert_eq!(
            decisions(&[
                "__builtin_expect",
                "(",
                "(",
                "long",
                ")",
                "(",
                "void",
                "*",
                ")",
                "__builtin_expect",
                "(",
                "(",
                "x",
                ")",
                ",",
                "1",
                ")",
                ",",
                "0",
                ")",
            ]),
            [Decision::Overridden, Decision::Static],
        );
    }

    /// Check an outer runtime expectation suppresses conflicting static advice from its projected operand.
    #[test]
    fn dynamic_outer_expectation_cancels_a_projected_static_inner_hint() {
        assert_eq!(
            decisions(&[
                "__builtin_expect",
                "(",
                "(",
                "int",
                ")",
                "__builtin_expect",
                "(",
                "(",
                "x",
                ")",
                ",",
                "1",
                ")",
                ",",
                "(",
                "y",
                ")",
                ")",
            ]),
            [Decision::Overridden, Decision::Dynamic],
        );
    }

    /// Check fixed first-operand results remain fixed even when the expected operand is dynamic or effectful.
    #[test]
    fn fixed_builtin_result_does_not_require_a_fixed_or_pure_hint() {
        assert_eq!(
            decisions(&[
                "__builtin_expect",
                "(",
                "(",
                "x",
                ")",
                ",",
                "__builtin_expect",
                "(",
                "0",
                ",",
                "hint",
                "(",
                ")",
                ")",
                ")"
            ]),
            [Decision::Dynamic, Decision::Dynamic, Decision::Static],
        );
    }

    /// Check macros without a verified expect builtin return an empty plan rather than allocating arena state.
    #[test]
    fn macros_without_verified_expectation_skip_planning_allocations() {
        let ordinary = frontend(&["(", "(", "x", ")", "+", "1", ")"]);
        assert!(plan(&ordinary, &analyze(&ordinary, "F")).is_empty());
        let mut frontend = frontend(&["__builtin_expect", "(", "(", "x", ")", ",", "1", ")"]);
        let analysis = analyze(&frontend, "F");
        assert!(!plan(&frontend, &analysis).is_empty());
        frontend.declarations.builtins.clear();
        assert!(plan(&frontend, &analysis).is_empty(), "a matching name is not a builtin proof");
        let mut analysis = analysis;
        analysis.expression = None;
        assert!(plan(&frontend, &analysis).is_empty());
    }
}
