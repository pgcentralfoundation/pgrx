//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Lower trusted integer analyses to hygienic Rust expression macros.
//!
//! Each C parameter occurrence remains a separate Rust expression occurrence. Results
//! retain their C integer identity in the semantic support's `CValue`; callers use
//! `.get()` when they deliberately need its Rust storage representation. Emission does
//! not establish differential validation, and the generic helpers are runtime calls.

use crate::analysis::{
    AnalysisStatus, AnalyzedExpression, ConstCapability, MacroAnalysis, ResolvedConstant,
    SkipReason, SkipReasonCode, TypeExpression,
};
use crate::model::{IntegerKind, IntegerType, IntegerValue, SignedOverflow};
use crate::syntax::{
    BinaryOperator, ExpressionKind, IntegerLiteral, NodeId, TokenRange, UnaryOperator,
};
use crate::{AnalysisSession, MacroDefinition, support_generation::support_abi_assertions};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

const SUPPORT: &str = "$crate::__pgrx_c_macros";
const MAX_EMISSION_BYTES: usize = 1024 * 1024;

/// Names and values in the defining Rust crate, supplied by its binding generator.
///
/// These values are checked against independently resolved C constants before use.
/// Paths are relative to `$crate`; `macros` contains macros actually emitted there.
#[derive(Clone, Debug, Default, Serialize)]
pub struct BindingCatalog {
    pub integer_constants: BTreeMap<String, IntegerBinding>,
    pub macros: BTreeSet<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IntegerBinding {
    pub path: Vec<String>,
    pub value: IntegerValue,
    pub representation: IntegerBindingRepresentation,
}

/// Storage introduced by the binding generator, including pgrx's OID rewrite.
#[derive(Clone, Copy, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegerBindingRepresentation {
    #[default]
    Primitive,
    Oid,
}

/// The original analysis and the outcome of lowering it under the same trusted profile.
#[derive(Clone, Debug, Serialize)]
pub struct MacroEmission {
    pub analysis: MacroAnalysis,
    pub status: EmissionStatus,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EmissionStatus {
    /// Rust source was produced; downstream compilation and C comparison are separate checks.
    Emitted {
        rust: String,
        const_capability: ConstCapability,
    },
    Skipped {
        reason: SkipReason,
    },
}

/// Generate one macro from an immutable analysis session, never from caller-mutated IR.
///
/// The defining crate must provide the matching semantic support at
/// `__pgrx_c_macros`. Each emitted string includes a target guard. `$crate` paths
/// keep the helper bindings valid when that crate is renamed.
pub fn emit(session: &AnalysisSession<'_>, name: &str) -> MacroEmission {
    emit_with_bindings(session, name, &BindingCatalog::default())
}

/// Emit with verified references to constants and other generated macros.
pub fn emit_with_bindings(
    session: &AnalysisSession<'_>,
    name: &str,
    bindings: &BindingCatalog,
) -> MacroEmission {
    let analysis = session.analyze(name);
    let lowered = if let Some(reason) = binding_mismatch(session, &analysis, bindings) {
        Err(reason)
    } else if let AnalysisStatus::Skipped { reason } = &analysis.status {
        Err(reason.clone())
    } else {
        let original = &session
            .frontend()
            .environment()
            .active
            .get(name)
            .expect("analyzed candidates come from the immutable final active macro map")
            .definition;
        support_abi_assertions(session.frontend().profile())
            .map_err(|error| {
                skip(&analysis, SkipReasonCode::UnsupportedProfile, error.to_string(), None)
            })
            .and_then(|assertions| render(session, &analysis, original, &assertions, bindings))
    };
    let status = match lowered {
        Ok(rust) => {
            EmissionStatus::Emitted { rust, const_capability: ConstCapability::RuntimeOnly }
        }
        Err(reason) => EmissionStatus::Skipped { reason },
    };
    MacroEmission { analysis, status }
}

/// Emit a set of macros, preserving available calls and propagating value mismatches.
///
/// A disagreement never changes bindgen's output. Reverse dependency traversal
/// also prevents callers from hiding a skipped macro by expanding its body.
pub fn emit_batch_with_bindings(
    session: &AnalysisSession<'_>,
    names: &[impl AsRef<str>],
    bindings: &BindingCatalog,
) -> Vec<MacroEmission> {
    let initial = names
        .iter()
        .map(|name| emit_with_bindings(session, name.as_ref(), bindings))
        .collect::<Vec<_>>();
    let mut roots = initial
        .iter()
        .filter_map(|emission| match &emission.status {
            EmissionStatus::Skipped { reason }
                if reason.code == SkipReasonCode::BindingValueMismatch =>
            {
                Some((emission.analysis.name.clone(), reason.clone()))
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let graph = session.frontend().dependencies();
    // Seed failures from the entire active environment, not only requested
    // functions: a retained object binding can hide a callee's enum identifier.
    for (name, binding) in &bindings.integer_constants {
        let constant = if session.frontend().environment().active.contains_key(name) {
            session.integer_constants().get(name)
        } else {
            session.frontend().declarations().integer_constants.get(name).filter(|constant| {
                let crate::TypeCategory::Integer(kind) = constant.ty.category else { return false };
                session
                    .frontend()
                    .profile()
                    .target
                    .integers
                    .get(&kind)
                    .is_some_and(|ty| ty.bits <= 64)
            })
        };
        let Some(constant) = constant else { continue };
        if integer_numeric(binding.value) == integer_numeric(constant.value) {
            continue;
        }
        for caller in graph.dependents(name).chain(graph.constant_users(name)) {
            roots.entry(caller.into()).or_insert_with(|| {
                mismatch_reason(&session.analyze(caller), name, binding.value, constant.value)
            });
        }
    }
    let impacts = graph.impacts(&roots.keys().collect::<Vec<_>>());
    let impacted =
        impacts.iter().map(|impact| (impact.name.as_str(), impact)).collect::<BTreeMap<_, _>>();
    let mut available = bindings.clone();
    available.macros.extend(
        initial
            .iter()
            .filter(|emission| {
                matches!(emission.status, EmissionStatus::Emitted { .. })
                    && !impacted.contains_key(emission.analysis.name.as_str())
            })
            .map(|emission| emission.analysis.name.clone()),
    );
    for name in roots.keys().map(String::as_str).chain(impacted.keys().copied()) {
        available.macros.remove(name);
    }
    let mut emissions = initial
        .into_iter()
        .map(|emission| {
            let name = &emission.analysis.name;
            let failure = if let Some(impact) = impacted.get(name.as_str()) {
                Some((impact.dependency.as_str(), impact.root.as_str()))
            } else if roots.contains_key(name) {
                // A wrapper may independently see the same folded constant.
                // Prefer its direct dependency as the explanation where possible.
                graph.dependencies(name).find_map(|dependency| {
                    if dependency != name && roots.contains_key(dependency) {
                        Some((dependency, dependency))
                    } else {
                        impacted.get(dependency).and_then(|impact| {
                            (impact.root != *name).then_some((dependency, impact.root.as_str()))
                        })
                    }
                })
            } else {
                None
            };
            if let Some((dependency, root)) = failure {
                let reason = dependency_skip(&emission.analysis, dependency, root, &roots[root]);
                MacroEmission {
                    analysis: emission.analysis,
                    status: EmissionStatus::Skipped { reason },
                }
            } else if roots.contains_key(name) {
                let reason = roots[name].clone();
                MacroEmission {
                    analysis: emission.analysis,
                    status: EmissionStatus::Skipped { reason },
                }
            } else {
                emit_with_bindings(session, name, &available)
            }
        })
        .collect::<Vec<_>>();
    // If a final rendering hits an output limit, reject its dependents too rather
    // than leaving a preserved call to a macro whose definition is absent.
    let late = emissions
        .iter()
        .filter_map(|emission| match &emission.status {
            EmissionStatus::Skipped { reason }
                if available.macros.contains(&emission.analysis.name) =>
            {
                Some((emission.analysis.name.clone(), reason.clone()))
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let impacts = graph.impacts(&late.keys().collect::<Vec<_>>());
    let impacted =
        impacts.iter().map(|impact| (impact.name.as_str(), impact)).collect::<BTreeMap<_, _>>();
    for emission in &mut emissions {
        if let Some(impact) = impacted.get(emission.analysis.name.as_str()) {
            emission.status = EmissionStatus::Skipped {
                reason: dependency_skip(
                    &emission.analysis,
                    &impact.dependency,
                    &impact.root,
                    &late[&impact.root],
                ),
            };
        }
    }
    emissions
}

fn dependency_skip(
    analysis: &MacroAnalysis,
    dependency: &str,
    root: &str,
    reason: &SkipReason,
) -> SkipReason {
    skip(
        analysis,
        SkipReasonCode::DependencySkipped,
        format!(
            "depends on skipped macro `{dependency}` (root `{root}`): {}; skipping macro `{}`",
            reason.message, analysis.name
        ),
        None,
    )
}

fn integer_numeric(value: IntegerValue) -> i128 {
    match value {
        IntegerValue::Signed(value) => i128::from(value),
        IntegerValue::Unsigned(value) => i128::from(value),
    }
}

fn mismatch_reason(
    analysis: &MacroAnalysis,
    name: &str,
    bindgen: IntegerValue,
    clang: IntegerValue,
) -> SkipReason {
    skip(
        analysis,
        SkipReasonCode::BindingValueMismatch,
        format!(
            "bindgen and clang do not agree on {name}'s value; bindgen={}, clang={}; skipping macro `{}`",
            integer_numeric(bindgen),
            integer_numeric(clang),
            analysis.name
        ),
        None,
    )
}

fn binding_mismatch(
    session: &AnalysisSession<'_>,
    analysis: &MacroAnalysis,
    bindings: &BindingCatalog,
) -> Option<SkipReason> {
    let mut constants = BTreeMap::new();
    if let Some(expression) = &analysis.expression {
        constants.extend(
            expression.constants.iter().map(|constant| (constant.name.as_str(), constant.value)),
        );
    }
    // Non-atomic object expansions may already be literals in the analyzed tree.
    // Check their trusted standalone C facts as well as retained identifiers.
    for dependency in &analysis.dependencies {
        if dependency.kind == crate::MacroKind::ObjectLike
            && let Some(constant) = session.integer_constants().get(&dependency.name)
        {
            constants.insert(dependency.name.as_str(), constant.value);
        }
    }
    constants.into_iter().find_map(|(name, value)| {
        let binding = bindings.integer_constants.get(name)?;
        (integer_numeric(binding.value) != integer_numeric(value))
            .then(|| mismatch_reason(analysis, name, binding.value, value))
    })
}

fn skip(
    analysis: &MacroAnalysis,
    code: SkipReasonCode,
    message: impl Into<String>,
    tokens: Option<TokenRange>,
) -> SkipReason {
    SkipReason {
        code,
        message: message.into(),
        tokens,
        spans: analysis.provenance.iter().cloned().collect(),
    }
}

fn render(
    session: &AnalysisSession<'_>,
    analysis: &MacroAnalysis,
    original: &MacroDefinition,
    assertions: &str,
    bindings: &BindingCatalog,
) -> Result<String, SkipReason> {
    let Some(identifier) = rust_identifier(&analysis.name) else {
        return Err(skip(
            analysis,
            SkipReasonCode::UnsupportedType,
            "the C macro name cannot be represented by a Rust macro identifier",
            None,
        ));
    };
    let mut rust = String::from(assertions);
    let mut doc = format!("C macro {}", analysis.name);
    if let Some(span) = &analysis.provenance
        && let Some(file) = span.file.file_name()
    {
        write!(&mut doc, " from {}:{}", file.to_string_lossy(), span.start_line)
            .expect("writing to a String cannot fail");
    }
    doc.push_str(&definition_doc(original).ok_or_else(|| {
        skip(
            analysis,
            SkipReasonCode::BudgetExceeded,
            "original C macro documentation exceeds the bounded output size",
            None,
        )
    })?);
    write!(&mut rust, "#[doc = {doc:?}]\n#[macro_export]\nmacro_rules! {identifier} {{\n    (")
        .expect("writing to a String cannot fail");
    for index in 0..analysis.parameters.len() {
        if index != 0 {
            rust.push_str(", ");
        }
        write!(&mut rust, "$__pgrx_c_arg{index}:expr").expect("writing to a String cannot fail");
    }
    if !analysis.parameters.is_empty() {
        rust.push_str(" $(,)?");
    }
    rust.push_str(") => {\n        ");

    if let Some(crate::ExpansionResult::Expanded { expansion }) =
        session.expansions().results.get(&analysis.name)
    {
        for fallback in &expansion.constant_fallbacks {
            write_fallback(&mut rust, &fallback.name, &fallback.reason);
        }
    }
    let expression = analysis.expression.as_ref().ok_or_else(|| {
        skip(
            analysis,
            SkipReasonCode::InvalidExpression,
            "candidate has no complete expression",
            None,
        )
    })?;
    let mut constants = vec![None; expression.syntax.nodes.len()];
    for constant in &expression.constants {
        constants[constant.node] = Some(constant);
    }
    let delegation = match crate::delegation::direct_delegation(session, analysis) {
        Ok(Some(delegation)) if bindings.macros.contains(&delegation.callee) => {
            if let Some(callee) = rust_identifier(&delegation.callee) {
                write!(&mut rust, "$crate::{callee}!(").expect("writing to a String cannot fail");
                for (index, root) in delegation.arguments.iter().enumerate() {
                    if index != 0 {
                        rust.push_str(", ");
                    }
                    render_expression(analysis, *root, bindings, &constants, &mut rust)?;
                }
                rust.push(')');
                true
            } else {
                write_fallback(
                    &mut rust,
                    &delegation.callee,
                    "its name cannot be represented as a Rust macro identifier",
                );
                false
            }
        }
        Ok(Some(delegation)) => {
            write_fallback(
                &mut rust,
                &delegation.callee,
                "the callee is not in the set of emitted Rust macros",
            );
            false
        }
        Err(reason) => {
            write_fallback(&mut rust, &analysis.name, &reason);
            false
        }
        Ok(None) => {
            for dependency in &analysis.dependencies {
                if dependency.kind == crate::MacroKind::FunctionLike {
                    write_fallback(
                        &mut rust,
                        &dependency.name,
                        "preserving a call inside this expression has not been proved equivalent to C substitution",
                    );
                }
            }
            false
        }
    };
    if !delegation {
        render_expression(analysis, expression.syntax.root, bindings, &constants, &mut rust)?;
    }
    rust.push_str("\n    };\n}\n");
    if rust.len() > MAX_EMISSION_BYTES {
        return Err(skip(
            analysis,
            SkipReasonCode::BudgetExceeded,
            "generated macro exceeds the bounded output size",
            None,
        ));
    }
    Ok(rust)
}

fn render_expression(
    analysis: &MacroAnalysis,
    root: NodeId,
    bindings: &BindingCatalog,
    constants: &[Option<&ResolvedConstant>],
    rust: &mut String,
) -> Result<(), SkipReason> {
    let expression = analysis.expression.as_ref().ok_or_else(|| {
        skip(
            analysis,
            SkipReasonCode::InvalidExpression,
            "candidate has no complete expression",
            None,
        )
    })?;
    let policy = match analysis.signed_overflow {
        SignedOverflow::Undefined => "Undefined",
        SignedOverflow::Wrapping => "Wrapping",
        SignedOverflow::Trapping => {
            return Err(skip(
                analysis,
                SkipReasonCode::UnsupportedProfile,
                "trapping signed arithmetic has no established support policy",
                None,
            ));
        }
    };

    // The arena is a tree from the bounded parser. Render it once into a single
    // buffer, rather than copying each rendered subtree into all its ancestors.
    let mut tasks = vec![RenderTask::Node(root)];
    while let Some(task) = tasks.pop() {
        match task {
            RenderTask::Text(text) => rust.push_str(text),
            RenderTask::Node(index) => {
                let node = &expression.syntax.nodes[index];
                match &node.kind {
                    ExpressionKind::Parameter { index } => {
                        write!(rust, "{SUPPORT}::value($__pgrx_c_arg{index})")
                            .expect("writing to a String cannot fail");
                    }
                    ExpressionKind::IntegerLiteral { literal } => {
                        let ty = concrete_type(expression, index, analysis)?;
                        write_literal(rust, ty, literal);
                    }
                    ExpressionKind::Identifier { .. } => {
                        let Some(constant) = constants[index] else {
                            return Err(skip(
                                analysis,
                                SkipReasonCode::InvalidExpression,
                                "identifier has no compiler-resolved integer value",
                                Some(node.tokens),
                            ));
                        };
                        match binding_path(bindings, &constant.name) {
                            Ok((path, representation)) => {
                                write!(
                                    rust,
                                    "{SUPPORT}::CValue::<{SUPPORT}::{}>::new(",
                                    marker(constant.ty.kind)
                                )
                                .expect("writing to a String cannot fail");
                                if constant.ty.kind == IntegerKind::Bool {
                                    rust.push('(');
                                }
                                write!(rust, "$crate::{path}")
                                    .expect("writing to a String cannot fail");
                                if matches!(representation, IntegerBindingRepresentation::Oid) {
                                    rust.push_str(".to_u32()");
                                }
                                if constant.ty.kind == IntegerKind::Bool {
                                    rust.push_str(" as u8 != 0)");
                                } else {
                                    write!(
                                        rust,
                                        " as {}{}",
                                        if constant.ty.signed { 'i' } else { 'u' },
                                        constant.ty.bits
                                    )
                                    .expect("writing to a String cannot fail");
                                }
                                rust.push(')');
                            }
                            Err(reason) => {
                                write_fallback(rust, &constant.name, reason);
                                let value = match constant.value {
                                    IntegerValue::Signed(value) => value.to_string(),
                                    IntegerValue::Unsigned(value) => value.to_string(),
                                };
                                if let Some(literal) = &constant.literal {
                                    write_literal(rust, constant.ty, literal);
                                } else {
                                    write_value(rust, constant.ty, &value);
                                }
                            }
                        }
                    }
                    ExpressionKind::Group { operand } => {
                        rust.push('(');
                        tasks.push(RenderTask::Text(")"));
                        tasks.push(RenderTask::Node(*operand));
                    }
                    ExpressionKind::Cast { operand, .. } => {
                        let ty = concrete_type(expression, index, analysis)?;
                        write!(rust, "{SUPPORT}::cast::<{SUPPORT}::{}, _>(", marker(ty.kind))
                            .expect("writing to a String cannot fail");
                        tasks.push(RenderTask::Text(")"));
                        tasks.push(RenderTask::Node(*operand));
                    }
                    ExpressionKind::Unary { operator, operand } => {
                        let helper = match operator {
                            UnaryOperator::Plus => "promote",
                            UnaryOperator::Negate => "neg",
                            UnaryOperator::BitwiseNot => "bitnot",
                            UnaryOperator::LogicalNot => "logical_not",
                        };
                        write!(rust, "{SUPPORT}::{helper}")
                            .expect("writing to a String cannot fail");
                        if *operator == UnaryOperator::Negate {
                            write!(rust, "::<{SUPPORT}::{policy}, _>")
                                .expect("writing to a String cannot fail");
                        }
                        rust.push('(');
                        tasks.push(RenderTask::Text(")"));
                        tasks.push(RenderTask::Node(*operand));
                    }
                    ExpressionKind::Binary { operator, left, right } => match operator {
                        BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
                            write!(
                                rust,
                                "{SUPPORT}::CValue::<{SUPPORT}::CInt>::new(if {SUPPORT}::truth("
                            )
                            .expect("writing to a String cannot fail");
                            tasks.push(RenderTask::Text(")"));
                            tasks.push(RenderTask::Text(" { 1 } else { 0 }"));
                            tasks.push(RenderTask::Text(")"));
                            tasks.push(RenderTask::Node(*right));
                            tasks.push(RenderTask::Text(
                                if *operator == BinaryOperator::LogicalAnd {
                                    ") && $crate::__pgrx_c_macros::truth("
                                } else {
                                    ") || $crate::__pgrx_c_macros::truth("
                                },
                            ));
                            tasks.push(RenderTask::Node(*left));
                        }
                        _ => {
                            let helper = binary_helper(*operator);
                            write!(rust, "{SUPPORT}::{helper}")
                                .expect("writing to a String cannot fail");
                            if matches!(
                                operator,
                                BinaryOperator::Add
                                    | BinaryOperator::Subtract
                                    | BinaryOperator::Multiply
                                    | BinaryOperator::ShiftLeft
                            ) {
                                write!(rust, "::<{SUPPORT}::{policy}, _, _>")
                                    .expect("writing to a String cannot fail");
                            }
                            rust.push('(');
                            tasks.push(RenderTask::Text(")"));
                            tasks.push(RenderTask::Node(*right));
                            tasks.push(RenderTask::Text(", "));
                            tasks.push(RenderTask::Node(*left));
                        }
                    },
                    ExpressionKind::Conditional { condition, then_value, else_value } => {
                        write!(rust, "{SUPPORT}::select(if {SUPPORT}::truth(")
                            .expect("writing to a String cannot fail");
                        tasks.push(RenderTask::Text(")"));
                        tasks.push(RenderTask::Text(") }"));
                        tasks.push(RenderTask::Node(*else_value));
                        tasks.push(RenderTask::Text(
                            ") } else { $crate::__pgrx_c_macros::Either::Right(",
                        ));
                        tasks.push(RenderTask::Node(*then_value));
                        tasks.push(RenderTask::Text(") { $crate::__pgrx_c_macros::Either::Left("));
                        tasks.push(RenderTask::Node(*condition));
                    }
                }
            }
        }
        if rust.len() > MAX_EMISSION_BYTES {
            return Err(skip(
                analysis,
                SkipReasonCode::BudgetExceeded,
                "generated macro exceeds the bounded output size",
                None,
            ));
        }
    }
    Ok(())
}

fn binding_path(
    bindings: &BindingCatalog,
    name: &str,
) -> Result<(String, IntegerBindingRepresentation), &'static str> {
    let binding = bindings
        .integer_constants
        .get(name)
        .ok_or("no integer constant binding is available in the defining Rust crate")?;
    if binding.path.is_empty() {
        return Err("the Rust binding has no usable path");
    }
    binding
        .path
        .iter()
        .map(|part| {
            rust_identifier(part).ok_or("the Rust binding path cannot be represented hygienically")
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|parts| (parts.join("::"), binding.representation))
}

fn write_fallback(rust: &mut String, name: &str, reason: &str) {
    // Diagnostic text can contain header tokens; prevent it from ending or nesting
    // the comment, or putting source directives on a new physical line.
    let clean =
        |text: &str| text.replace("/*", "/ *").replace("*/", "* /").replace(['\n', '\r'], " ");
    write!(
        rust,
        "/* PGRX: {} remains expanded because {}. */ ",
        clean(name),
        clean(reason.trim_end_matches('.'))
    )
    .expect("writing to a String cannot fail");
}

fn definition_doc(definition: &MacroDefinition) -> Option<String> {
    // Bound normalization before allocating: one retained comment can be much
    // larger than the token count suggests. Display adds at most one separator
    // per token, and omits the repeated name token and line comments.
    let bytes = definition
        .tokens
        .iter()
        .fold("#define ".len().saturating_add(definition.name.len()), |bytes, token| {
            bytes.saturating_add(token.spelling.len()).saturating_add(1)
        });
    if bytes > MAX_EMISSION_BYTES {
        return None;
    }
    let definition = definition.to_string();
    let fence_length = definition.split(|character| character != '`').map(str::len).max()?;
    let fence_length = fence_length.saturating_add(1).max(3);
    if definition.len().saturating_add(fence_length.saturating_mul(2)).saturating_add(9)
        > MAX_EMISSION_BYTES
    {
        return None;
    }
    let fence = "`".repeat(fence_length);
    Some(format!("\n\n{fence}text\n{definition}\n{fence}\n"))
}

enum RenderTask {
    Node(NodeId),
    Text(&'static str),
}

fn concrete_type(
    expression: &AnalyzedExpression,
    index: NodeId,
    analysis: &MacroAnalysis,
) -> Result<IntegerType, SkipReason> {
    match &expression.types[index] {
        TypeExpression::Concrete { ty } => Ok(*ty),
        _ => Err(skip(
            analysis,
            SkipReasonCode::InvalidExpression,
            "literal, constant or cast has no concrete C integer type",
            Some(expression.syntax.nodes[index].tokens),
        )),
    }
}

fn write_value(rust: &mut String, ty: IntegerType, magnitude: &str) {
    let value = if ty.kind == IntegerKind::Bool {
        if magnitude == "0" { "false" } else { "true" }
    } else {
        magnitude
    };
    write!(rust, "{SUPPORT}::CValue::<{SUPPORT}::{}>::new({value}", marker(ty.kind))
        .expect("writing to a String cannot fail");
    if ty.kind != IntegerKind::Bool {
        write!(rust, "{}{}", if ty.signed { 'i' } else { 'u' }, ty.bits)
            .expect("writing to a String cannot fail");
    }
    rust.push(')');
}

fn write_literal(rust: &mut String, ty: IntegerType, literal: &IntegerLiteral) {
    if literal.spelling.starts_with('\'') {
        write!(rust, "{SUPPORT}::CValue::<{SUPPORT}::{}>::new(", marker(ty.kind))
            .expect("writing to a String cannot fail");
        let body = &literal.spelling[1..literal.spelling.len() - 1];
        if !body.starts_with('\\')
            || matches!(body, r"\'" | r#"\""# | r"\\" | r"\n" | r"\r" | r"\t" | r"\0")
            || (body.starts_with(r"\x") && body.len() == 4)
        {
            rust.push_str(&literal.spelling);
        } else if body == r"\?" {
            rust.push_str("'?'");
        } else {
            // Analysis established an ASCII C int value. Rust has no C octal
            // character escapes and requires exactly two hexadecimal digits.
            write!(rust, "'\\x{:02x}'", literal.value).expect("writing to a String cannot fail");
        }
        write!(rust, " as {}{})", if ty.signed { 'i' } else { 'u' }, ty.bits)
            .expect("writing to a String cannot fail");
        return;
    }

    // The immutable session's parser already validated this ASCII spelling.
    // Remove the known suffix length rather than trimming digit-like letters.
    let suffix_bytes = usize::from(literal.suffix.unsigned) + usize::from(literal.suffix.long);
    let digits = &literal.spelling[..literal.spelling.len() - suffix_bytes];
    let magnitude = match literal.radix {
        16 => format!("0x{}", &digits[2..]),
        2 => format!("0b{}", &digits[2..]),
        8 if digits.len() > 1 => format!("0o{}", &digits[1..]),
        _ => digits.to_owned(),
    };
    write_value(rust, ty, &magnitude);
}

fn marker(kind: IntegerKind) -> &'static str {
    match kind {
        IntegerKind::Bool => "CBool",
        IntegerKind::Char => "CChar",
        IntegerKind::SignedChar => "CSignedChar",
        IntegerKind::UnsignedChar => "CUnsignedChar",
        IntegerKind::Short => "CShort",
        IntegerKind::UnsignedShort => "CUnsignedShort",
        IntegerKind::Int => "CInt",
        IntegerKind::UnsignedInt => "CUnsignedInt",
        IntegerKind::Long => "CLong",
        IntegerKind::UnsignedLong => "CUnsignedLong",
        IntegerKind::LongLong => "CLongLong",
        IntegerKind::UnsignedLongLong => "CUnsignedLongLong",
        IntegerKind::Int128 => "CInt128",
        IntegerKind::UnsignedInt128 => "CUnsignedInt128",
    }
}

fn binary_helper(operator: BinaryOperator) -> &'static str {
    match operator {
        BinaryOperator::Multiply => "mul",
        BinaryOperator::Divide => "div",
        BinaryOperator::Remainder => "rem",
        BinaryOperator::Add => "add",
        BinaryOperator::Subtract => "sub",
        BinaryOperator::ShiftLeft => "shl",
        BinaryOperator::ShiftRight => "shr",
        BinaryOperator::Less => "lt",
        BinaryOperator::LessEqual => "le",
        BinaryOperator::Greater => "gt",
        BinaryOperator::GreaterEqual => "ge",
        BinaryOperator::Equal => "eq",
        BinaryOperator::NotEqual => "ne",
        BinaryOperator::BitAnd => "bitand",
        BinaryOperator::BitXor => "bitxor",
        BinaryOperator::BitOr => "bitor",
        BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
            unreachable!("logical operators have their own lazy lowering")
        }
    }
}

fn rust_identifier(name: &str) -> Option<String> {
    let mut bytes = name.bytes();
    if !bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        || matches!(name, "_" | "self" | "Self" | "super" | "crate")
    {
        return None;
    }
    let keyword = matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "static"
            | "struct"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "gen"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    );
    Some(if keyword { format!("r#{name}") } else { name.into() })
}

#[cfg(test)]
mod tests {
    use super::{MAX_EMISSION_BYTES, definition_doc, rust_identifier};
    use crate::{MacroDefinition, MacroKind, Token, TokenKind};

    #[test]
    fn documentation_budget_includes_comment_bytes_and_markdown_fences() {
        for comment in [
            format!("/* {} */", "x".repeat(MAX_EMISSION_BYTES)),
            format!("/* {} */", "`".repeat(MAX_EMISSION_BYTES / 2)),
        ] {
            let mut tokens = [
                (TokenKind::Identifier, "F"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Identifier, "x"),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Identifier, "x"),
            ]
            .into_iter()
            .map(|(kind, spelling)| Token { kind, spelling: spelling.into() })
            .collect::<Vec<_>>();
            tokens.push(Token { kind: TokenKind::Comment, spelling: comment });
            let definition = MacroDefinition {
                name: "F".into(),
                kind: MacroKind::FunctionLike,
                location: None,
                provenance: None,
                tokens,
                builtin: false,
                main_file: true,
            };
            assert!(definition_doc(&definition).is_none());
        }
    }

    #[test]
    fn macro_names_preserve_identity_without_using_c_parameters_as_rust_identifiers() {
        assert_eq!(rust_identifier("TYPEALIGN").as_deref(), Some("TYPEALIGN"));
        assert_eq!(rust_identifier("match").as_deref(), Some("r#match"));
        for rejected in ["", "_", "self", "Self", "super", "crate", "bad-name", "1name", "é"] {
            assert_eq!(rust_identifier(rejected), None);
        }
    }
}
