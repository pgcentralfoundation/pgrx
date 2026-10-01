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
    AnalysisStatus, AnalyzedExpression, ConstCapability, MacroAnalysis, SkipReason, SkipReasonCode,
    TypeExpression,
};
use crate::model::{IntegerKind, IntegerType, IntegerValue, SignedOverflow};
use crate::syntax::{BinaryOperator, ExpressionKind, NodeId, TokenRange, UnaryOperator};
use crate::{AnalysisSession, support_generation::support_abi_assertions};
use serde::Serialize;
use std::fmt::Write;

const SUPPORT: &str = "$crate::__pgrx_c_macros";
const MAX_EMISSION_BYTES: usize = 1024 * 1024;

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
    let analysis = session.analyze(name);
    let lowered = if let AnalysisStatus::Skipped { reason } = &analysis.status {
        Err(reason.clone())
    } else {
        support_abi_assertions(session.frontend().profile())
            .map_err(|error| {
                skip(&analysis, SkipReasonCode::UnsupportedProfile, error.to_string(), None)
            })
            .and_then(|assertions| render(&analysis, &assertions))
    };
    let status = match lowered {
        Ok(rust) => {
            EmissionStatus::Emitted { rust, const_capability: ConstCapability::RuntimeOnly }
        }
        Err(reason) => EmissionStatus::Skipped { reason },
    };
    MacroEmission { analysis, status }
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

fn render(analysis: &MacroAnalysis, assertions: &str) -> Result<String, SkipReason> {
    let Some(identifier) = rust_identifier(&analysis.name) else {
        return Err(skip(
            analysis,
            SkipReasonCode::UnsupportedType,
            "the C macro name cannot be represented by a Rust macro identifier",
            None,
        ));
    };
    let Some(expression) = &analysis.expression else {
        return Err(skip(
            analysis,
            SkipReasonCode::InvalidExpression,
            "candidate has no complete expression",
            None,
        ));
    };
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
    let mut rust = String::from(assertions);
    let mut doc = format!("C macro {}", analysis.name);
    if let Some(span) = &analysis.provenance
        && let Some(file) = span.file.file_name()
    {
        write!(&mut doc, " from {}:{}", file.to_string_lossy(), span.start_line)
            .expect("writing to a String cannot fail");
    }
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

    // The arena is a tree from the bounded parser. Render it once into a single
    // buffer, rather than copying each rendered subtree into all its ancestors.
    let mut tasks = vec![RenderTask::Node(expression.syntax.root)];
    let mut constants = vec![None; expression.syntax.nodes.len()];
    for constant in &expression.constants {
        constants[constant.node] = Some(constant);
    }
    while let Some(task) = tasks.pop() {
        match task {
            RenderTask::Text(text) => rust.push_str(text),
            RenderTask::Node(index) => {
                let node = &expression.syntax.nodes[index];
                match &node.kind {
                    ExpressionKind::Parameter { index } => {
                        write!(&mut rust, "{SUPPORT}::value($__pgrx_c_arg{index})")
                            .expect("writing to a String cannot fail");
                    }
                    ExpressionKind::IntegerLiteral { literal } => {
                        let ty = concrete_type(expression, index, analysis)?;
                        write_value(&mut rust, ty, &literal.value.to_string());
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
                        let value = match constant.value {
                            IntegerValue::Signed(value) => value.to_string(),
                            IntegerValue::Unsigned(value) => value.to_string(),
                        };
                        write_value(&mut rust, constant.ty, &value);
                    }
                    ExpressionKind::Group { operand } => {
                        rust.push('(');
                        tasks.push(RenderTask::Text(")"));
                        tasks.push(RenderTask::Node(*operand));
                    }
                    ExpressionKind::Cast { operand, .. } => {
                        let ty = concrete_type(expression, index, analysis)?;
                        write!(&mut rust, "{SUPPORT}::cast::<{SUPPORT}::{}, _>(", marker(ty.kind))
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
                        write!(&mut rust, "{SUPPORT}::{helper}")
                            .expect("writing to a String cannot fail");
                        if *operator == UnaryOperator::Negate {
                            write!(&mut rust, "::<{SUPPORT}::{policy}, _>")
                                .expect("writing to a String cannot fail");
                        }
                        rust.push('(');
                        tasks.push(RenderTask::Text(")"));
                        tasks.push(RenderTask::Node(*operand));
                    }
                    ExpressionKind::Binary { operator, left, right } => match operator {
                        BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
                            write!(
                                &mut rust,
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
                            write!(&mut rust, "{SUPPORT}::{helper}")
                                .expect("writing to a String cannot fail");
                            if matches!(
                                operator,
                                BinaryOperator::Add
                                    | BinaryOperator::Subtract
                                    | BinaryOperator::Multiply
                                    | BinaryOperator::ShiftLeft
                            ) {
                                write!(&mut rust, "::<{SUPPORT}::{policy}, _, _>")
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
                        write!(&mut rust, "{SUPPORT}::select(if {SUPPORT}::truth(")
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
    use super::rust_identifier;

    #[test]
    fn macro_names_preserve_identity_without_using_c_parameters_as_rust_identifiers() {
        assert_eq!(rust_identifier("TYPEALIGN").as_deref(), Some("TYPEALIGN"));
        assert_eq!(rust_identifier("match").as_deref(), Some("r#match"));
        for rejected in ["", "_", "self", "Self", "super", "crate", "bad-name", "1name", "é"] {
            assert_eq!(rust_identifier(rejected), None);
        }
    }
}
