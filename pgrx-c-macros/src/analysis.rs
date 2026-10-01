//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Conservative symbolic analysis for the initial C integer-expression family.
//!
//! A candidate has a complete expression and constraints for C integer/bool inputs. It is
//! pending Rust lowering and validation, not a supported generated macro. Casts never imply
//! a unique input type. Macro dependencies require compiler-owned expansion; this module
//! deliberately does not substitute token strings or infer recursive preprocessing rules.
//! Candidates describe invocation scopes whose referenced C bindings and final macro
//! environment match the inspection, rather than arbitrary caller-local rebindings.

use crate::model::{
    ActiveMacro, ActiveProvenance, DeclarationCatalog, FrontendOutput, IntegerKind, IntegerType,
    IntegerValue, SignedOverflow, TargetFacts, TypeCategory,
};
use crate::syntax::{
    BinaryOperator, Expression, ExpressionKind, IntegerLiteral, NodeId, SyntaxError,
    SyntaxErrorKind, TokenRange, UnaryOperator, parse_expression,
};
use crate::{MacroKind, SourceSpan, Token, TokenKind};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Analysis of one final active definition under the inspected compilation profile.
#[derive(Clone, Debug, Serialize)]
pub struct MacroAnalysis {
    pub name: String,
    pub provenance: Option<SourceSpan>,
    pub status: AnalysisStatus,
    pub parameters: Vec<ParameterAnalysis>,
    pub expression: Option<AnalyzedExpression>,
    pub invocation: InvocationContract,
    pub const_capability: ConstCapability,
    pub evaluation: EvaluationContract,
    pub dependencies: Vec<MacroDependency>,
    pub required_helpers: Vec<HelperRequirement>,
    pub signed_overflow: SignedOverflow,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum AnalysisStatus {
    /// Analyzed for the bounded integer family; Rust emission has not been validated.
    Candidate,
    Skipped {
        reason: SkipReason,
    },
}

#[derive(Clone, Debug, Serialize)]
pub struct SkipReason {
    pub code: SkipReasonCode,
    pub message: String,
    pub tokens: Option<TokenRange>,
    pub spans: Vec<SourceSpan>,
}

/// Stable machine-readable categories, separate from explanatory wording.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SkipReasonCode {
    NotActive,
    NotFunctionLike,
    ProvenanceAmbiguous,
    ProvenanceUnresolved,
    UnsupportedProfile,
    InvalidTarget,
    MalformedParameters,
    Variadic,
    TokenPaste,
    Stringification,
    ExpansionRequired,
    EmptyReplacement,
    InvalidExpression,
    UnsupportedLiteral,
    LiteralOutOfRange,
    PointerOperation,
    Mutation,
    Statement,
    Call,
    UnevaluatedExpression,
    TypeParameter,
    CommaExpression,
    BudgetExceeded,
    UnsupportedType,
    UnknownIdentifier,
    VariableAccess,
    InvocationGrouping,
    DynamicBuiltin,
    CompilerRejected,
    UnrecognizedExpansion,
}

#[derive(Clone, Debug, Serialize)]
pub struct ParameterAnalysis {
    pub name: String,
    pub roles: Vec<ParameterRole>,
    /// Every lexical replacement-list occurrence; unused arguments remain unevaluated.
    pub uses: Vec<ParameterUse>,
    pub constraint: Option<InputConstraint>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterRole {
    Value,
    Type,
    Place,
    Identifier,
    Unused,
    Unknown,
}

#[derive(Clone, Debug, Serialize)]
pub struct ParameterUse {
    pub tokens: TokenRange,
    /// Established from a complete parsed expression, never neighboring-token heuristics.
    pub grouped: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InputConstraint {
    /// Target-supported C integer or bool values; pointers, floats and arbitrary Rust
    /// operator implementations are outside this candidate family.
    IntegerScalar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InvocationContract {
    /// Each value hole has explicit C parentheses, and the complete body is parenthesized
    /// or an atomic integer literal/constant. Rust expression fragments therefore preserve
    /// grouping both inside the body and at its invocation. Referenced C typedefs and
    /// constants must resolve to the inspected declarations, and the final macro environment
    /// must match the inspected environment. Caller-local shadowing and subsequent macro
    /// redefinition/undefinition are outside this family. Type/domain constraints also apply.
    ParenthesizedScalarExpressions,
    NotEstablished,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstCapability {
    KnownConst,
    RuntimeOnly,
    /// Integer syntax alone cannot establish const-compatible Rust helper calls.
    NotEstablished,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct EvaluationContract {
    pub requirements: Vec<EvaluationRequirement>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationRequirement {
    PreserveParameterOccurrences,
    PreserveLazyBranches,
    UnspecifiedOperandOrder,
    ExcludeUndefinedCOperations,
    RespectSignedOverflowProfile,
}

#[derive(Clone, Debug, Serialize)]
pub struct MacroDependency {
    pub name: String,
    pub kind: MacroKind,
    pub provenance: Option<SourceSpan>,
    pub uses: Vec<TokenRange>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HelperRequirement {
    IntegerPromotions,
    UsualArithmeticConversions,
    IntegerCasts,
    IntegerArithmetic,
    BitwiseOperations,
    Comparisons,
    CTruth,
    DivisionRemainder,
    Shifts,
    SignedOverflowPolicy,
}

#[derive(Clone, Debug, Serialize)]
pub struct AnalyzedExpression {
    pub syntax: Expression,
    /// Parallel to `syntax.nodes`; references are expression node indices.
    pub types: Vec<TypeExpression>,
    pub constants: Vec<ResolvedConstant>,
}

/// Symbolic C result types retain promotion/common-type rules instead of selecting a
/// signature from one sample invocation or collapsing equal-width C identities.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypeExpression {
    Concrete { ty: IntegerType },
    Parameter { index: usize },
    Promotion { operand: NodeId },
    Common { left: NodeId, right: NodeId },
}

#[derive(Clone, Debug, Serialize)]
pub struct ResolvedConstant {
    pub node: NodeId,
    pub name: String,
    pub ty: IntegerType,
    pub value: IntegerValue,
}

/// Analyze a definition only in the profile/environment/catalog resolved together by
/// the frontend. Work is bounded by replacement tokens; dependencies are recorded,
/// never recursively expanded. Catalog/environment lookups use their ordered maps.
pub fn analyze(frontend: &FrontendOutput, name: &str) -> MacroAnalysis {
    analyze_active(frontend, name, frontend.environment().active.get(name))
}

pub(crate) fn analyze_active(
    frontend: &FrontendOutput,
    name: &str,
    active: Option<&ActiveMacro>,
) -> MacroAnalysis {
    let profile = frontend.profile();
    let mut result = MacroAnalysis {
        name: name.into(),
        provenance: None,
        status: AnalysisStatus::Candidate,
        parameters: Vec::new(),
        expression: None,
        invocation: InvocationContract::NotEstablished,
        const_capability: ConstCapability::NotEstablished,
        evaluation: EvaluationContract::default(),
        dependencies: Vec::new(),
        required_helpers: Vec::new(),
        signed_overflow: profile.signed_overflow,
    };
    let Some(active) = active else {
        return result.skip(
            SkipReasonCode::NotActive,
            "macro is not in the final active environment",
            None,
        );
    };
    let definition = &active.definition;
    result.provenance = definition.provenance.clone();
    match &active.provenance {
        ActiveProvenance::Resolved => {}
        ActiveProvenance::Ambiguous(spans) => {
            result.status = AnalysisStatus::Skipped {
                reason: SkipReason {
                    code: SkipReasonCode::ProvenanceAmbiguous,
                    message: "the final definition matches multiple source definitions".into(),
                    tokens: None,
                    spans: spans.clone(),
                },
            };
            return result;
        }
        ActiveProvenance::Unresolved => {
            return result.skip(
                SkipReasonCode::ProvenanceUnresolved,
                "the final definition has no resolved source identity",
                None,
            );
        }
    }
    if definition.kind != MacroKind::FunctionLike {
        return result.skip(
            SkipReasonCode::NotFunctionLike,
            "only function-like macros are primary analysis candidates",
            None,
        );
    }
    if definition.tokens.len() > 8192 {
        return result.skip(
            SkipReasonCode::BudgetExceeded,
            "definition exceeds the 8192-token analysis budget",
            None,
        );
    }
    let (parameters, body_start) = match formal_parameters(&definition.tokens, name) {
        Ok(value) => value,
        Err((code, message)) => return result.skip(code, message, None),
    };
    let body = &definition.tokens[body_start..];
    let parameter_indices = parameters
        .iter()
        .enumerate()
        .map(|(index, name)| (name.as_str(), index))
        .collect::<HashMap<_, _>>();
    result.parameters = parameters
        .iter()
        .map(|name| ParameterAnalysis {
            name: name.clone(),
            roles: vec![ParameterRole::Unused],
            uses: Vec::new(),
            constraint: None,
        })
        .collect();
    let mut dependencies = BTreeMap::<String, MacroDependency>::new();
    for (index, token) in
        body.iter().enumerate().filter(|(_, token)| token.kind != TokenKind::Comment)
    {
        let range = TokenRange { start: index, end: index + 1 };
        if let Some(&parameter) = parameter_indices.get(token.spelling.as_str()) {
            let parameter = &mut result.parameters[parameter];
            parameter.roles = vec![ParameterRole::Unknown];
            parameter.uses.push(ParameterUse { tokens: range, grouped: false });
        } else if let Some(dependency) = frontend.environment().active.get(&token.spelling) {
            dependencies
                .entry(token.spelling.clone())
                .or_insert_with(|| MacroDependency {
                    name: token.spelling.clone(),
                    kind: dependency.definition.kind,
                    provenance: dependency.definition.provenance.clone(),
                    uses: Vec::new(),
                })
                .uses
                .push(range);
        }
    }
    result.dependencies = dependencies.into_values().collect();
    if let Some((index, token)) = body
        .iter()
        .enumerate()
        .find(|(_, token)| token.spelling == "##" || token.spelling == "%:%:")
    {
        return result.skip(
            SkipReasonCode::TokenPaste,
            format!(
                "token pasting ({}) needs a preprocessing-specific invocation contract",
                token.spelling
            ),
            Some(TokenRange { start: index, end: index + 1 }),
        );
    }
    if let Some((index, _)) =
        body.iter().enumerate().find(|(_, token)| token.spelling == "#" || token.spelling == "%:")
    {
        return result.skip(
            SkipReasonCode::Stringification,
            "stringification needs a preprocessing-specific invocation contract",
            Some(TokenRange { start: index, end: index + 1 }),
        );
    }
    if !profile.unsupported_options.is_empty() {
        return result.skip(
            SkipReasonCode::UnsupportedProfile,
            format!("unmodeled semantic options: {}", profile.unsupported_options.join(", ")),
            None,
        );
    }
    if let Some(dependency) = result.dependencies.first() {
        let tokens = dependency.uses.first().copied();
        let message = format!(
            "{} requires compiler-owned macro prescan/rescan and recursive suppression",
            dependency.name
        );
        return result.skip(SkipReasonCode::ExpansionRequired, message, tokens);
    }
    if let Err(message) = validate_target(&profile.target) {
        return result.skip(SkipReasonCode::InvalidTarget, message, None);
    }
    let catalog = frontend.declarations();
    let syntax = match parse_expression(body, &parameters, |name| {
        recognized_type(name, catalog, &profile.target)
    }) {
        Ok(syntax) => syntax,
        Err(error) => return result.syntax_error(error),
    };
    // Parent links establish whether each hole, rather than a larger surrounding
    // expression, is explicitly grouped. Traversal remains linear in arena size.
    let mut grouped = vec![false; syntax.nodes.len()];
    for node in &syntax.nodes {
        if let ExpressionKind::Group { operand } = node.kind {
            grouped[operand] = true;
        }
    }
    let mut next_use = vec![0; result.parameters.len()];
    for (index, node) in syntax.nodes.iter().enumerate() {
        if let ExpressionKind::Parameter { index: parameter } = node.kind {
            // Leaves are parsed in lexical order, so the next recorded use is the
            // corresponding hole. Repeated parameters do not need a linear search.
            let usage = &mut result.parameters[parameter].uses[next_use[parameter]];
            debug_assert_eq!(usage.tokens, node.tokens);
            usage.grouped = grouped[index];
            next_use[parameter] += 1;
            result.parameters[parameter].roles = vec![ParameterRole::Value];
            result.parameters[parameter].constraint = Some(InputConstraint::IntegerScalar);
        }
    }
    if let Some(usage) =
        result.parameters.iter().flat_map(|parameter| &parameter.uses).find(|usage| !usage.grouped)
    {
        let tokens = usage.tokens;
        return result.skip(SkipReasonCode::InvocationGrouping, "a parameter occurrence is not explicitly parenthesized; Rust expr fragments would change textual C grouping", Some(tokens));
    }
    match analyze_types(&syntax, catalog, &profile.target) {
        Ok((types, constants, helpers)) => {
            let root = &syntax.nodes[syntax.root];
            if !matches!(
                root.kind,
                ExpressionKind::Group { .. }
                    | ExpressionKind::IntegerLiteral { .. }
                    | ExpressionKind::Identifier { .. }
            ) {
                return result.skip(
                    SkipReasonCode::InvocationGrouping,
                    "the complete replacement expression is not parenthesized or atomic; Rust expression grouping would change its interaction with caller operators",
                    Some(root.tokens),
                );
            }
            result.expression = Some(AnalyzedExpression { syntax, types, constants });
            result.required_helpers = helpers.into_iter().collect();
            result.invocation = InvocationContract::ParenthesizedScalarExpressions;
            result.evaluation.requirements = vec![
                EvaluationRequirement::PreserveParameterOccurrences,
                EvaluationRequirement::ExcludeUndefinedCOperations,
                EvaluationRequirement::RespectSignedOverflowProfile,
            ];
            if result.expression.as_ref().is_some_and(|expression| {
                expression
                    .syntax
                    .nodes
                    .iter()
                    .any(|node| matches!(node.kind, ExpressionKind::Binary { operator, .. } if !matches!(operator, BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr)))
            }) {
                result.evaluation.requirements.push(EvaluationRequirement::UnspecifiedOperandOrder);
            }
            if result.expression.as_ref().is_some_and(|expression| {
                expression.syntax.nodes.iter().any(|node| {
                    matches!(
                        node.kind,
                        ExpressionKind::Conditional { .. }
                            | ExpressionKind::Binary {
                                operator: BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr,
                                ..
                            }
                    )
                })
            }) {
                result.evaluation.requirements.push(EvaluationRequirement::PreserveLazyBranches);
            }
            result
        }
        Err((code, message, tokens)) => result.skip(code, message, Some(tokens)),
    }
}

impl MacroAnalysis {
    fn skip(
        mut self,
        code: SkipReasonCode,
        message: impl Into<String>,
        tokens: Option<TokenRange>,
    ) -> Self {
        self.status = AnalysisStatus::Skipped {
            reason: SkipReason {
                code,
                message: message.into(),
                tokens,
                spans: self.provenance.iter().cloned().collect(),
            },
        };
        self
    }

    fn syntax_error(mut self, error: SyntaxError) -> Self {
        if error.kind == SyntaxErrorKind::TypeParameter {
            for parameter in &mut self.parameters {
                if parameter.uses.iter().any(|usage| {
                    usage.tokens.start >= error.tokens.start && usage.tokens.end <= error.tokens.end
                }) {
                    parameter.roles = vec![ParameterRole::Type];
                }
            }
        }
        let code = match error.kind {
            SyntaxErrorKind::EmptyReplacement => SkipReasonCode::EmptyReplacement,
            SyntaxErrorKind::InvalidExpression => SkipReasonCode::InvalidExpression,
            SyntaxErrorKind::UnsupportedLiteral => SkipReasonCode::UnsupportedLiteral,
            SyntaxErrorKind::PointerOperation => SkipReasonCode::PointerOperation,
            SyntaxErrorKind::Mutation => SkipReasonCode::Mutation,
            SyntaxErrorKind::Statement => SkipReasonCode::Statement,
            SyntaxErrorKind::Call => SkipReasonCode::Call,
            SyntaxErrorKind::UnevaluatedExpression => SkipReasonCode::UnevaluatedExpression,
            SyntaxErrorKind::TypeParameter => SkipReasonCode::TypeParameter,
            SyntaxErrorKind::CommaExpression => SkipReasonCode::CommaExpression,
            SyntaxErrorKind::BudgetExceeded => SkipReasonCode::BudgetExceeded,
        };
        self.skip(code, error.message, Some(error.tokens))
    }
}

fn formal_parameters(
    tokens: &[Token],
    name: &str,
) -> Result<(Vec<String>, usize), (SkipReasonCode, String)> {
    let mut iter = tokens.iter().enumerate().filter(|(_, token)| token.kind != TokenKind::Comment);
    if iter.next().is_none_or(|(_, token)| token.spelling != name)
        || iter.next().is_none_or(|(_, token)| token.spelling != "(")
    {
        return Err((
            SkipReasonCode::MalformedParameters,
            "function macro tokens have no complete name/parameter prefix".into(),
        ));
    }
    let mut parameters = Vec::new();
    let mut names = BTreeSet::new();
    let mut expect_name = true;
    for (index, token) in iter {
        match token.spelling.as_str() {
            "..." => {
                return Err((
                    SkipReasonCode::Variadic,
                    "variadic macros require a separate invocation contract".into(),
                ));
            }
            ")" if !expect_name || parameters.is_empty() => return Ok((parameters, index + 1)),
            "," if !expect_name => expect_name = true,
            _ if expect_name
                && matches!(token.kind, TokenKind::Identifier | TokenKind::Keyword)
                && names.insert(token.spelling.as_str()) =>
            {
                parameters.push(token.spelling.clone());
                expect_name = false;
            }
            _ => {
                return Err((
                    SkipReasonCode::MalformedParameters,
                    format!("invalid parameter-list token: {:?}", token.spelling),
                ));
            }
        }
    }
    Err((SkipReasonCode::MalformedParameters, "unterminated parameter list".into()))
}

type TypeFailure = (SkipReasonCode, String, TokenRange);
type TypeAnalysis = (Vec<TypeExpression>, Vec<ResolvedConstant>, BTreeSet<HelperRequirement>);

fn analyze_types(
    expression: &Expression,
    catalog: &DeclarationCatalog,
    target: &TargetFacts,
) -> Result<TypeAnalysis, TypeFailure> {
    let mut types = Vec::<TypeExpression>::with_capacity(expression.nodes.len());
    let mut constants = Vec::new();
    let mut helpers = BTreeSet::new();
    let int = target.integers[&IntegerKind::Int];
    for (index, node) in expression.nodes.iter().enumerate() {
        let ty = match &node.kind {
            ExpressionKind::Parameter { index } => TypeExpression::Parameter { index: *index },
            ExpressionKind::IntegerLiteral { literal } => {
                if literal.spelling.starts_with('\'') && !target.ascii_execution_charset {
                    return Err((SkipReasonCode::UnsupportedLiteral,
                        "ordinary character literals require a verified ASCII execution character set".into(), node.tokens));
                }
                if literal.radix == 2 && target.c_standard.is_none_or(|standard| standard < 202311)
                {
                    return Err((
                        SkipReasonCode::UnsupportedLiteral,
                        "binary integer literals require a C23-or-later profile in this bounded analyzer".into(),
                        node.tokens,
                    ));
                }
                TypeExpression::Concrete { ty: literal_type(literal, target).ok_or_else(|| (
                    SkipReasonCode::LiteralOutOfRange,
                    "no standard C integer literal type in the target profile represents this magnitude/suffix".into(),
                    node.tokens,
                ))? }
            }
            ExpressionKind::Identifier { name } => {
                let Some(constant) = catalog.integer_constants.get(name) else {
                    let (code, message) = if catalog.variables.contains_key(name) {
                        (
                            SkipReasonCode::VariableAccess,
                            "variable access requires binding, lifetime and evaluation contracts",
                        )
                    } else {
                        (
                            SkipReasonCode::UnknownIdentifier,
                            "identifier is not a compiler-owned integer constant",
                        )
                    };
                    return Err((code, format!("{name}: {message}"), node.tokens));
                };
                let TypeCategory::Integer(kind) = constant.ty.category else {
                    return Err((
                        SkipReasonCode::UnsupportedType,
                        format!(
                            "{name}: integer constant has no established canonical integer kind"
                        ),
                        node.tokens,
                    ));
                };
                let Some(&ty) = target.integers.get(&kind) else {
                    return Err((
                        SkipReasonCode::UnsupportedType,
                        format!("{name}: integer kind is absent from the target profile"),
                        node.tokens,
                    ));
                };
                constants.push(ResolvedConstant {
                    node: index,
                    name: name.clone(),
                    ty,
                    value: constant.value,
                });
                TypeExpression::Concrete { ty }
            }
            ExpressionKind::Group { operand } => types[*operand].clone(),
            ExpressionKind::Cast { type_name, .. } => {
                let Some(ty) = integer_type(type_name, catalog, target) else {
                    let code = if type_name.contains('*') {
                        SkipReasonCode::PointerOperation
                    } else {
                        SkipReasonCode::UnsupportedType
                    };
                    return Err((
                        code,
                        format!(
                            "cast type {type_name:?} is outside the established integer family"
                        ),
                        node.tokens,
                    ));
                };
                helpers.insert(HelperRequirement::IntegerCasts);
                TypeExpression::Concrete { ty }
            }
            ExpressionKind::Unary { operator, operand } => {
                helpers.insert(HelperRequirement::IntegerPromotions);
                match operator {
                    UnaryOperator::LogicalNot => {
                        helpers.insert(HelperRequirement::CTruth);
                        TypeExpression::Concrete { ty: int }
                    }
                    UnaryOperator::BitwiseNot => {
                        helpers.insert(HelperRequirement::BitwiseOperations);
                        promoted_type(&types, *operand, target)
                    }
                    UnaryOperator::Plus | UnaryOperator::Negate => {
                        helpers.insert(HelperRequirement::IntegerArithmetic);
                        if *operator == UnaryOperator::Negate {
                            helpers.insert(HelperRequirement::SignedOverflowPolicy);
                        }
                        promoted_type(&types, *operand, target)
                    }
                }
            }
            ExpressionKind::Binary { operator, left, right } => {
                helpers.insert(HelperRequirement::IntegerPromotions);
                match operator {
                    BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => {
                        helpers.insert(HelperRequirement::Shifts);
                        promoted_type(&types, *left, target)
                    }
                    BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual => {
                        helpers.insert(HelperRequirement::UsualArithmeticConversions);
                        helpers.insert(HelperRequirement::Comparisons);
                        TypeExpression::Concrete { ty: int }
                    }
                    BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
                        helpers.insert(HelperRequirement::CTruth);
                        TypeExpression::Concrete { ty: int }
                    }
                    _ => {
                        helpers.insert(HelperRequirement::UsualArithmeticConversions);
                        match operator {
                            BinaryOperator::BitAnd
                            | BinaryOperator::BitXor
                            | BinaryOperator::BitOr => {
                                helpers.insert(HelperRequirement::BitwiseOperations);
                            }
                            BinaryOperator::Divide | BinaryOperator::Remainder => {
                                helpers.insert(HelperRequirement::DivisionRemainder);
                            }
                            _ => {
                                helpers.insert(HelperRequirement::IntegerArithmetic);
                                helpers.insert(HelperRequirement::SignedOverflowPolicy);
                            }
                        }
                        common_type(&types, *left, *right, target)
                    }
                }
            }
            ExpressionKind::Conditional { then_value, else_value, .. } => {
                helpers.insert(HelperRequirement::CTruth);
                helpers.insert(HelperRequirement::UsualArithmeticConversions);
                common_type(&types, *then_value, *else_value, target)
            }
        };
        types.push(ty);
    }
    Ok((types, constants, helpers))
}

fn validate_target(target: &TargetFacts) -> Result<(), String> {
    for kind in [
        IntegerKind::Bool,
        IntegerKind::Char,
        IntegerKind::SignedChar,
        IntegerKind::UnsignedChar,
        IntegerKind::Short,
        IntegerKind::UnsignedShort,
        IntegerKind::Int,
        IntegerKind::UnsignedInt,
        IntegerKind::Long,
        IntegerKind::UnsignedLong,
        IntegerKind::LongLong,
        IntegerKind::UnsignedLongLong,
    ] {
        let Some(ty) = target.integers.get(&kind) else {
            return Err(format!("target facts omit {kind:?}"));
        };
        if ty.kind != kind || ty.bits == 0 || ty.bits > 128 {
            return Err(format!(
                "target facts contain an unsupported integer representation for {kind:?}"
            ));
        }
    }
    for (&kind, ty) in &target.integers {
        if ty.kind != kind || ty.bits == 0 || ty.bits > 128 {
            return Err(format!(
                "target facts contain an unsupported integer representation for {kind:?}"
            ));
        }
    }
    for (signed, unsigned) in [
        (IntegerKind::SignedChar, IntegerKind::UnsignedChar),
        (IntegerKind::Short, IntegerKind::UnsignedShort),
        (IntegerKind::Int, IntegerKind::UnsignedInt),
        (IntegerKind::Long, IntegerKind::UnsignedLong),
        (IntegerKind::LongLong, IntegerKind::UnsignedLongLong),
        (IntegerKind::Int128, IntegerKind::UnsignedInt128),
    ] {
        match (target.integers.get(&signed), target.integers.get(&unsigned)) {
            (None, None) => {}
            (Some(signed), Some(unsigned))
                if signed.signed
                    && !unsigned.signed
                    && signed.bits == unsigned.bits
                    && signed.rank == unsigned.rank => {}
            _ => {
                return Err(
                    "target facts do not establish matching signed/unsigned integer pairs".into()
                );
            }
        }
    }
    let char = target.integers[&IntegerKind::Char];
    let signed_char = target.integers[&IntegerKind::SignedChar];
    if char.signed != target.char_is_signed
        || char.bits != target.char_bits
        || char.bits != signed_char.bits
        || char.rank != signed_char.rank
    {
        return Err("target char facts are inconsistent".into());
    }
    let bool_ = target.integers[&IntegerKind::Bool];
    if bool_.signed || bool_.rank >= signed_char.rank {
        return Err("target bool facts are inconsistent".into());
    }
    let mut previous = signed_char;
    for kind in [
        IntegerKind::Short,
        IntegerKind::Int,
        IntegerKind::Long,
        IntegerKind::LongLong,
        IntegerKind::Int128,
    ] {
        if let Some(&next) = target.integers.get(&kind) {
            if next.rank <= previous.rank || next.bits < previous.bits {
                return Err("target integer ranks/widths are inconsistent".into());
            }
            previous = next;
        }
    }
    Ok(())
}

fn recognized_type(name: &str, catalog: &DeclarationCatalog, target: &TargetFacts) -> bool {
    integer_type(name, catalog, target).is_some()
        || catalog.types.contains_key(name)
        || matches!(name, "void" | "float" | "double" | "long double")
        || name.contains('*')
            && name.split_whitespace().next().is_some_and(|first| {
                integer_type(first, catalog, target).is_some() || catalog.types.contains_key(first)
            })
}

fn integer_type(
    name: &str,
    catalog: &DeclarationCatalog,
    target: &TargetFacts,
) -> Option<IntegerType> {
    if let Some(info) = catalog.types.get(name) {
        if let TypeCategory::Integer(kind) = info.category {
            return target.integers.get(&kind).copied();
        }
        return None;
    }
    let words = name
        .split_whitespace()
        .filter(|word| !matches!(*word, "const" | "volatile"))
        .collect::<Vec<_>>();
    if words.len() == 1
        && let Some(info) = catalog.types.get(words[0])
    {
        if let TypeCategory::Integer(kind) = info.category {
            return target.integers.get(&kind).copied();
        }
        return None;
    }
    let kind = fundamental_kind(&words)?;
    target.integers.get(&kind).copied()
}

// C declaration specifiers can appear in different orders (`long unsigned int`).
// Count the permitted specifiers rather than accepting only one printed spelling.
fn fundamental_kind(words: &[&str]) -> Option<IntegerKind> {
    let (mut signed, mut unsigned, mut short, mut long, mut int, mut char, mut bool_, mut int128) =
        (0, 0, 0, 0, 0, 0, 0, 0);
    for word in words {
        match *word {
            "signed" => signed += 1,
            "unsigned" => unsigned += 1,
            "short" => short += 1,
            "long" => long += 1,
            "int" => int += 1,
            "char" => char += 1,
            "_Bool" => bool_ += 1,
            "__int128" => int128 += 1,
            _ => return None,
        }
    }
    if words.is_empty()
        || signed + unsigned > 1
        || short > 1
        || long > 2
        || int > 1
        || char > 1
        || bool_ > 1
        || int128 > 1
    {
        return None;
    }
    if bool_ == 1 {
        return (words.len() == 1).then_some(IntegerKind::Bool);
    }
    if char == 1 {
        if short + long + int + int128 != 0 {
            return None;
        }
        return Some(if unsigned == 1 {
            IntegerKind::UnsignedChar
        } else if signed == 1 {
            IntegerKind::SignedChar
        } else {
            IntegerKind::Char
        });
    }
    if int128 == 1 {
        if short + long + int != 0 {
            return None;
        }
        return Some(if unsigned == 1 { IntegerKind::UnsignedInt128 } else { IntegerKind::Int128 });
    }
    if short == 1 && long != 0 {
        return None;
    }
    Some(match (unsigned == 1, short, long) {
        (false, 1, 0) => IntegerKind::Short,
        (true, 1, 0) => IntegerKind::UnsignedShort,
        (false, 0, 0) => IntegerKind::Int,
        (true, 0, 0) => IntegerKind::UnsignedInt,
        (false, 0, 1) => IntegerKind::Long,
        (true, 0, 1) => IntegerKind::UnsignedLong,
        (false, 0, 2) => IntegerKind::LongLong,
        (true, 0, 2) => IntegerKind::UnsignedLongLong,
        _ => return None,
    })
}

fn literal_type(literal: &IntegerLiteral, target: &TargetFacts) -> Option<IntegerType> {
    use IntegerKind::{Int, Long, LongLong, UnsignedInt, UnsignedLong, UnsignedLongLong};
    let decimal = literal.radix == 10;
    let candidates: &[IntegerKind] = match (literal.suffix.unsigned, literal.suffix.long, decimal) {
        (false, 0, true) => &[Int, Long, LongLong],
        (false, 0, false) => &[Int, UnsignedInt, Long, UnsignedLong, LongLong, UnsignedLongLong],
        (true, 0, _) => &[UnsignedInt, UnsignedLong, UnsignedLongLong],
        (false, 1, true) => &[Long, LongLong],
        (false, 1, false) => &[Long, UnsignedLong, LongLong, UnsignedLongLong],
        (true, 1, _) => &[UnsignedLong, UnsignedLongLong],
        (false, 2, true) => &[LongLong],
        (false, 2, false) => &[LongLong, UnsignedLongLong],
        (true, 2, _) => &[UnsignedLongLong],
        _ => return None,
    };
    candidates
        .iter()
        .filter_map(|kind| target.integers.get(kind))
        .find(|ty| literal.value <= maximum(**ty))
        .copied()
}

fn maximum(ty: IntegerType) -> u128 {
    if ty.kind == IntegerKind::Bool {
        return 1;
    }
    let value_bits = ty.bits - u32::from(ty.signed);
    if value_bits == 128 { u128::MAX } else { (1_u128 << value_bits) - 1 }
}

fn promotion(ty: IntegerType, target: &TargetFacts) -> IntegerType {
    let int = target.integers[&IntegerKind::Int];
    if ty.rank > int.rank || matches!(ty.kind, IntegerKind::Int | IntegerKind::UnsignedInt) {
        return ty;
    }
    if maximum(ty) <= maximum(int) { int } else { target.integers[&IntegerKind::UnsignedInt] }
}

fn promoted_type(
    types: &[TypeExpression],
    operand: NodeId,
    target: &TargetFacts,
) -> TypeExpression {
    match types[operand] {
        TypeExpression::Concrete { ty } => TypeExpression::Concrete { ty: promotion(ty, target) },
        _ => TypeExpression::Promotion { operand },
    }
}

fn common_type(
    types: &[TypeExpression],
    left: NodeId,
    right: NodeId,
    target: &TargetFacts,
) -> TypeExpression {
    match (&types[left], &types[right]) {
        (TypeExpression::Concrete { ty: left }, TypeExpression::Concrete { ty: right }) => {
            let left = promotion(*left, target);
            let right = promotion(*right, target);
            let ty = if left.signed == right.signed {
                if left.rank >= right.rank { left } else { right }
            } else {
                let (signed, unsigned) = if left.signed { (left, right) } else { (right, left) };
                if unsigned.rank >= signed.rank {
                    unsigned
                } else if maximum(signed) >= maximum(unsigned) {
                    signed
                } else {
                    target.integers[&unsigned_kind(signed.kind)]
                }
            };
            TypeExpression::Concrete { ty }
        }
        _ => TypeExpression::Common { left, right },
    }
}

fn unsigned_kind(kind: IntegerKind) -> IntegerKind {
    match kind {
        IntegerKind::SignedChar | IntegerKind::Char => IntegerKind::UnsignedChar,
        IntegerKind::Short => IntegerKind::UnsignedShort,
        IntegerKind::Int => IntegerKind::UnsignedInt,
        IntegerKind::Long => IntegerKind::UnsignedLong,
        IntegerKind::LongLong => IntegerKind::UnsignedLongLong,
        IntegerKind::Int128 => IntegerKind::UnsignedInt128,
        _ => unreachable!(
            "usual arithmetic conversions request the unsigned counterpart of a signed promoted type"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        ActiveMacro, BuildInputs, ByteOrder, CompilationProfile, CompilerIdentity, MacroEnvironment,
    };
    use crate::{MacroDefinition, MacroInventory};

    fn target(long_bits: u32) -> TargetFacts {
        let integers = [
            (IntegerKind::Bool, 8, false, 0),
            (IntegerKind::Char, 8, true, 1),
            (IntegerKind::SignedChar, 8, true, 1),
            (IntegerKind::UnsignedChar, 8, false, 1),
            (IntegerKind::Short, 16, true, 2),
            (IntegerKind::UnsignedShort, 16, false, 2),
            (IntegerKind::Int, 32, true, 3),
            (IntegerKind::UnsignedInt, 32, false, 3),
            (IntegerKind::Long, long_bits, true, 4),
            (IntegerKind::UnsignedLong, long_bits, false, 4),
            (IntegerKind::LongLong, 64, true, 5),
            (IntegerKind::UnsignedLongLong, 64, false, 5),
        ]
        .into_iter()
        .map(|(kind, bits, signed, rank)| (kind, IntegerType { kind, bits, signed, rank }))
        .collect();
        TargetFacts {
            triple: "fixture".into(),
            pointer_bits: 64,
            char_bits: 8,
            char_is_signed: true,
            ascii_execution_charset: true,
            byte_order: ByteOrder::Little,
            c_standard: Some(201710),
            integers,
        }
    }

    fn literal(spelling: &str, value: u128, radix: u8, unsigned: bool, long: u8) -> IntegerLiteral {
        IntegerLiteral {
            spelling: spelling.into(),
            value,
            radix,
            suffix: crate::syntax::IntegerSuffix { unsigned, long },
        }
    }

    #[test]
    fn literal_types_follow_radix_suffix_and_target() {
        let lp64 = target(64);
        let llp64 = target(32);
        let decimal = literal("2147483648", 2147483648, 10, false, 0);
        assert_eq!(literal_type(&decimal, &lp64).unwrap().kind, IntegerKind::Long);
        assert_eq!(literal_type(&decimal, &llp64).unwrap().kind, IntegerKind::LongLong);
        let hex = literal("0x80000000", 0x80000000, 16, false, 0);
        assert_eq!(literal_type(&hex, &lp64).unwrap().kind, IntegerKind::UnsignedInt);
        let largest = literal("18446744073709551615", u128::from(u64::MAX), 10, false, 0);
        assert!(literal_type(&largest, &lp64).is_none());
        let largest = literal("18446744073709551615ULL", u128::from(u64::MAX), 10, true, 2);
        assert_eq!(literal_type(&largest, &lp64).unwrap().kind, IntegerKind::UnsignedLongLong);
    }

    #[test]
    fn promotions_and_common_types_preserve_c_identity() {
        let lp64 = target(64);
        let llp64 = target(32);
        for kind in [
            IntegerKind::Bool,
            IntegerKind::Char,
            IntegerKind::SignedChar,
            IntegerKind::UnsignedChar,
            IntegerKind::Short,
            IntegerKind::UnsignedShort,
        ] {
            assert_eq!(promotion(lp64.integers[&kind], &lp64).kind, IntegerKind::Int);
        }
        assert_eq!(maximum(lp64.integers[&IntegerKind::Bool]), 1);
        let operands = |target: &TargetFacts, left, right| {
            vec![
                TypeExpression::Concrete { ty: target.integers[&left] },
                TypeExpression::Concrete { ty: target.integers[&right] },
            ]
        };
        let TypeExpression::Concrete { ty } =
            common_type(&operands(&lp64, IntegerKind::Long, IntegerKind::UnsignedInt), 0, 1, &lp64)
        else {
            panic!("concrete operands have a concrete common type")
        };
        assert_eq!(ty.kind, IntegerKind::Long);
        let TypeExpression::Concrete { ty } = common_type(
            &operands(&llp64, IntegerKind::Long, IntegerKind::UnsignedInt),
            0,
            1,
            &llp64,
        ) else {
            panic!("concrete operands have a concrete common type")
        };
        assert_eq!(ty.kind, IntegerKind::UnsignedLong);
        let TypeExpression::Concrete { ty } = common_type(
            &operands(&lp64, IntegerKind::LongLong, IntegerKind::UnsignedLong),
            0,
            1,
            &lp64,
        ) else {
            panic!("concrete operands have a concrete common type")
        };
        assert_eq!(ty.kind, IntegerKind::UnsignedLongLong);
    }

    #[test]
    fn character_constants_require_the_verified_execution_charset() {
        let mut frontend = frontend(&["(", "'x'", ")"]);
        assert!(matches!(analyze(&frontend, "F").status, AnalysisStatus::Candidate));
        frontend.profile.target.ascii_execution_charset = false;
        assert!(matches!(
            analyze(&frontend, "F").status,
            AnalysisStatus::Skipped {
                reason: SkipReason { code: SkipReasonCode::UnsupportedLiteral, .. }
            }
        ));
    }

    fn frontend(body: &[&str]) -> FrontendOutput {
        let tokens = ["F", "(", "x", ",", "unused", ")"]
            .into_iter()
            .chain(body.iter().copied())
            .map(|spelling| Token {
                spelling: spelling.into(),
                kind: if spelling.as_bytes().first().is_some_and(u8::is_ascii_digit)
                    || spelling.starts_with('\'')
                {
                    TokenKind::Literal
                } else if spelling.as_bytes().first().is_some_and(u8::is_ascii_alphabetic) {
                    TokenKind::Identifier
                } else {
                    TokenKind::Punctuation
                },
            })
            .collect();
        let definition = MacroDefinition {
            name: "F".into(),
            kind: MacroKind::FunctionLike,
            location: None,
            provenance: None,
            tokens,
            builtin: false,
            main_file: true,
        };
        FrontendOutput {
            profile: CompilationProfile {
                header: "fixture.h".into(),
                compiler: CompilerIdentity {
                    executable: "clang".into(),
                    version: "fixture".into(),
                    libclang_version: "fixture".into(),
                },
                arguments: Vec::new(),
                target: target(64),
                signed_overflow: SignedOverflow::Wrapping,
                unsupported_options: Vec::new(),
                inputs: BuildInputs::default(),
            },
            environment: MacroEnvironment {
                active: [(
                    "F".into(),
                    ActiveMacro { definition, provenance: ActiveProvenance::Resolved },
                )]
                .into_iter()
                .collect(),
            },
            declarations: DeclarationCatalog::default(),
            inventory: MacroInventory { macros: Vec::new(), diagnostics: Vec::new() },
        }
    }

    #[test]
    fn constraints_occurrences_and_skips_are_explicit() {
        let input = frontend(&["(", "(", "x", ")", "+", "(", "x", ")", ")"]);
        let analysis = analyze(&input, "F");
        assert!(matches!(analysis.status, AnalysisStatus::Candidate));
        assert_eq!(analysis.parameters[0].uses.len(), 2);
        assert!(analysis.parameters[0].uses.iter().all(|usage| usage.grouped));
        assert_eq!(analysis.parameters[0].constraint, Some(InputConstraint::IntegerScalar));
        assert_eq!(analysis.parameters[1].roles, vec![ParameterRole::Unused]);
        assert!(analysis.parameters[1].constraint.is_none());
        assert_eq!(analysis.const_capability, ConstCapability::NotEstablished);
        for (body, expected) in [
            (vec!["x", "+", "1"], SkipReasonCode::InvocationGrouping),
            (vec!["(", "x", ")", "?", "1", ":", "2"], SkipReasonCode::InvocationGrouping),
            (vec!["(", "unused", ")", "(", "x", ")"], SkipReasonCode::TypeParameter),
            (vec!["(", "char", "*", ")", "(", "x", ")"], SkipReasonCode::PointerOperation),
        ] {
            let AnalysisStatus::Skipped { reason } = analyze(&frontend(&body), "F").status else {
                panic!("unsupported syntax must skip")
            };
            assert_eq!(reason.code, expected);
        }
    }

    #[test]
    fn replacement_grouping_is_checked_separately_from_parameter_grouping() {
        for body in [
            vec!["(", "x", ")", "+", "1"],
            vec!["-", "(", "x", ")"],
            vec!["(", "int", ")", "(", "x", ")"],
        ] {
            let analysis = analyze(&frontend(&body), "F");
            assert!(analysis.parameters[0].uses.iter().all(|usage| usage.grouped));
            assert!(analysis.expression.is_none());
            let AnalysisStatus::Skipped { reason } = analysis.status else {
                panic!("unwrapped bodies must skip even when every parameter is grouped")
            };
            assert_eq!(reason.code, SkipReasonCode::InvocationGrouping);
            assert_eq!(reason.tokens, Some(TokenRange { start: 0, end: body.len() }));
        }
        for body in [vec!["(", "(", "x", ")", "+", "1", ")"], vec!["1"]] {
            assert!(matches!(analyze(&frontend(&body), "F").status, AnalysisStatus::Candidate));
        }
    }

    #[test]
    fn binary_literal_candidates_require_the_inspected_c23_profile() {
        let mut input = frontend(&["(", "(", "x", ")", "+", "0b10", ")"]);
        for standard in [None, Some(201112), Some(201710), Some(202000)] {
            input.profile.target.c_standard = standard;
            let AnalysisStatus::Skipped { reason } = analyze(&input, "F").status else {
                panic!("pre-C23 binary literals must conservatively skip")
            };
            assert_eq!(reason.code, SkipReasonCode::UnsupportedLiteral);
        }
        input.profile.target.c_standard = Some(202311);
        assert!(matches!(analyze(&input, "F").status, AnalysisStatus::Candidate));
    }

    #[test]
    fn concrete_specifier_order_and_missing_pairs() {
        let target = target(64);
        assert_eq!(
            integer_type("long unsigned int", &DeclarationCatalog::default(), &target)
                .unwrap()
                .kind,
            IntegerKind::UnsignedLong
        );
        assert!(integer_type("long short", &DeclarationCatalog::default(), &target).is_none());
        let mut target = target;
        target.integers.insert(
            IntegerKind::Int128,
            IntegerType { kind: IntegerKind::Int128, bits: 128, signed: true, rank: 6 },
        );
        assert!(validate_target(&target).is_err());
    }
}
