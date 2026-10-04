//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Conservative symbolic analysis for supported C macro expressions.
//!
//! A candidate has a parsed expression and compiler-derived declaration and operand facts.
//! It is pending Rust lowering and validation. Casts never imply
//! a unique input type. Macro dependencies require compiler-owned expansion; this module
//! deliberately does not substitute token strings or infer recursive preprocessing rules.
//! Candidates describe invocation scopes whose referenced C bindings and final macro
//! environment match the inspection. Unresolved caller-scope identifiers become explicit
//! operands; this does not permit rebinding declarations established by the compiler.

//! Analysis describes the family of supported invocations before Rust emission chooses
//! capabilities. Expression nodes retain source spans and symbolic C types, while parameter
//! roles, grouping contracts, and evaluation requirements tell the emitter which substitutions
//! are valid. A parsed candidate is not a claim of successful lowering or differential proof;
//! unsupported constructs carry structured reasons instead.

use crate::model::{
    ActiveMacro, ActiveProvenance, DeclarationCatalog, FrontendOutput, IntegerConstant,
    IntegerKind, IntegerType, IntegerValue, SignedOverflow, TargetFacts, TypeCategory, TypeInfo,
};
use crate::syntax::{
    BinaryOperator, Expression, ExpressionKind, IntegerLiteral, NodeId, OffsetComponent,
    OffsetRecord, Statement, SyntaxError, SyntaxErrorKind, TokenRange, UnaryOperator,
    parse_expression, parse_replacement,
};
use crate::{MacroKind, SourceSpan, Token, TokenKind};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// Analysis of one final active definition under the inspected compilation profile.
#[derive(Clone, Debug, Serialize)]
pub struct MacroAnalysis {
    /// The original C identifier retained for reports, symbol lookup, and readable generated output.
    pub name: String,
    /// Physical source origins or their resolution status used for ownership filtering and auditing.
    pub provenance: Option<SourceSpan>,
    /// Whether symbolic analysis produced a candidate or an explicit auditable refusal.
    pub status: AnalysisStatus,
    /// Analyzed formals followed by explicit caller captures, preserving original source labels.
    pub parameters: Vec<ParameterAnalysis>,
    /// Complete analyzed source arena when syntax and type facts establish a candidate.
    pub expression: Option<AnalyzedExpression>,
    /// The proven grouping/context contract required of generated macro callers.
    pub invocation: InvocationContract,
    /// The established const-context status; symbolic syntax alone does not promise const fn support.
    pub const_capability: ConstCapability,
    /// Occurrence, laziness, undefined-operation, and overflow obligations carried into lowering.
    pub evaluation: EvaluationContract,
    /// References established for this phase and used to explain or propagate downstream skips.
    pub dependencies: Vec<MacroDependency>,
    /// C conversion and operator helper families discovered during analysis.
    pub required_helpers: Vec<HelperRequirement>,
    /// The inspected C signed-overflow policy, retained for helper selection and refusal decisions.
    pub signed_overflow: SignedOverflow,
}

/// Distinguish analyzable symbolic candidates from explicit refusals without claiming Rust lowering
/// succeeded.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum AnalysisStatus {
    /// Parsed and analyzed; Rust lowering and differential validation are separate steps.
    Candidate,
    /// A required proof or supported construct is missing, with an explicit refusal attached.
    Skipped {
        /// The structured proof gap that prevents this candidate from being treated as supported.
        reason: SkipReason,
    },
}

/// Carry a stable skip category and auditable source detail through reports and downstream dependency
/// propagation.
#[derive(Clone, Debug, Serialize)]
pub struct SkipReason {
    /// Stable machine-readable skip classification, independent of the explanatory text.
    pub code: SkipReasonCode,
    /// Human-readable detail explaining the compiler observation or unsupported construct.
    pub message: String,
    /// Original replacement-token range of the refused syntax when available.
    pub tokens: Option<TokenRange>,
    /// Possible physical source ranges supporting the refusal or dependency explanation.
    pub spans: Vec<SourceSpan>,
}

/// Stable machine-readable categories, separate from explanatory wording.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SkipReasonCode {
    /// The selected name is absent from the final active preprocessing environment.
    NotActive,
    /// The selected definition has no function-style parameter list and is retained only as expansion
    /// context.
    NotFunctionLike,
    /// Multiple physical definitions match the final active body, so unique source ownership is
    /// unproved.
    ProvenanceAmbiguous,
    /// No physical discovery definition could be linked reliably to the final active body.
    ProvenanceUnresolved,
    /// Recorded compiler semantics exceed the modeled runtime/helper family.
    UnsupportedProfile,
    /// Required target representation or C language facts could not be established.
    InvalidTarget,
    /// The original macro signature cannot establish an ordinary bounded formal list.
    MalformedParameters,
    /// An open macro or callable argument tail falls outside the supported substitution contract.
    Variadic,
    /// Token pasting lacks the independent closed-expansion and dependency proof required for
    /// support.
    TokenPaste,
    /// Stringified preprocessing spelling cannot be preserved by the accepted Rust argument contract.
    Stringification,
    /// The replacement needs a prepared compiler-owned expansion before symbolic analysis.
    ExpansionRequired,
    /// The replacement has no supported expression/statement value under this analysis path.
    EmptyReplacement,
    /// The complete replacement cannot be parsed with the bounded supported grammar.
    InvalidExpression,
    /// The literal lies outside the verified ordinary integer/basic-character subset.
    UnsupportedLiteral,
    /// No supported target C literal type represents the parsed magnitude.
    LiteralOutOfRange,
    /// Pointer syntax or operand compatibility lacks the required semantic proof.
    PointerOperation,
    /// Assignment or update lacks a supported place/evaluation contract.
    Mutation,
    /// Statement syntax lies outside the bounded scope-preserving grammar.
    Statement,
    /// A callable signature or supported call construct could not be established.
    Call,
    /// A sizeof/alignment operand lacks the required structural type/value proof.
    UnevaluatedExpression,
    /// A formal type role is ambiguous or exceeds the supported declarator grammar.
    TypeParameter,
    /// Comma sequencing cannot be admitted under the selected invocation contract.
    CommaExpression,
    /// Source, token, dependency, depth, or compiler-pass work exceeded a finite configured limit.
    BudgetExceeded,
    /// A referenced C type or Rust representation cannot establish the needed semantic capabilities.
    UnsupportedType,
    /// An identifier cannot be resolved to an admitted declaration, constant, or explicit caller
    /// capture.
    UnknownIdentifier,
    /// Native variable access lacks verified C identity or compatible actual binding storage.
    VariableAccess,
    /// The C textual invocation requires a grouping boundary not established for ordinary Rust
    /// substitutions.
    InvocationGrouping,
    /// A context-dependent compiler builtin cannot be frozen into a faithful generated definition.
    DynamicBuiltin,
    /// The matched compiler rejected a required typed or preprocessing witness.
    CompilerRejected,
    /// Compiler output could not establish the required complete symbolic expansion.
    UnrecognizedExpansion,
    /// The actual bindgen constant disagrees with the compiler-established C value.
    BindingValueMismatch,
    /// An immediate dependency was refused, so its callers cannot be emitted reliably.
    DependencySkipped,
}

/// Describe one formal or explicit caller capture and the syntactic capabilities required by all its
/// occurrences.
#[derive(Clone, Debug, Serialize)]
pub struct ParameterAnalysis {
    /// Original formal or caller-capture label retained in generated macro argument names.
    pub name: String,
    /// Whether the operand is an original formal or an explicit caller-scope capture.
    pub origin: ParameterOrigin,
    /// All syntactic uses that must agree before the emitter can choose a Rust fragment grammar.
    pub roles: Vec<ParameterRole>,
    /// Every lexical replacement-list occurrence; unused arguments remain unevaluated.
    pub uses: Vec<ParameterUse>,
}

/// C macro formals and caller-scope identifiers have different hygiene rules.
/// Rust callers supply free identifiers explicitly after the original formals.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterOrigin {
    /// An original C macro formal, retaining source name and replacement occurrences.
    Formal,
    /// An unresolved caller-scope identifier supplied explicitly to preserve Rust macro hygiene.
    FreeIdentifier,
}

/// The substitution grammar needed by a parameter, independently of any one concrete caller type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterRole {
    /// An evaluated expression operand whose C conversion/evaluation rules remain symbolic.
    Value,
    /// A non-evaluated type operand requiring a Rust type fragment rather than an expression.
    Type,
    /// An identifier/designator substitution that must retain C structural spelling.
    Identifier,
    /// A member-name hole used structurally for field projection or offsetof.
    FieldDesignator,
    /// No surviving occurrence requires evaluating or resolving the supplied argument.
    Unused,
    /// No single supported substitution grammar has been established for this operand.
    Unknown,
}

/// One source occurrence whose grouping contributes to the valid macro invocation family.
#[derive(Clone, Debug, Serialize)]
pub struct ParameterUse {
    /// Original replacement-list token coordinates used to explain syntax and reconstruct source.
    pub tokens: TokenRange,
    /// Established from a complete parsed expression, never neighboring-token heuristics.
    pub grouped: bool,
}

/// The grouping and caller-context conditions under which generated Rust preserves the supported C
/// invocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InvocationContract {
    /// Each value hole has explicit C parentheses, and the complete body is parenthesized
    /// or atomic. Rust expression fragments therefore preserve
    /// grouping both inside the body and at its invocation. Referenced C typedefs and
    /// constants must resolve to the inspected declarations, and the final macro environment
    /// must match the inspected environment. Caller-local shadowing and subsequent macro
    /// redefinition/undefinition are outside this family. Type/domain constraints also apply.
    ParenthesizedScalarExpressions,
    /// Ungrouped parameter substitutions accept one Rust token tree, corresponding
    /// to an atomic C identifier, literal, or explicitly parenthesized expression.
    AtomicArguments,
    /// The original replacement is not an atomic C expression. Rust callers
    /// must explicitly request the semantics of a parenthesized C invocation;
    /// surrounding textual C precedence is outside this invocation contract.
    ExplicitExpressionBoundary,
    /// A complete statement body may return from the caller's function.
    /// Callers must supply a compatible C return type and preserve the recorded
    /// argument grouping. Introduced local names cannot occur in supplied arguments;
    /// C textual capture of those names is outside this invocation family.
    ReturningStatements,
    /// A complete C statement body executes in order and yields no value.
    /// Local capture and argument grouping constraints match returning statements.
    Statements,
    /// An unbraced C statement replacement can capture a caller's `else`.
    /// Callers explicitly request the semantics of a braced C invocation.
    ExplicitStatementBoundary,
    /// No proof has established this capability or invocation contract.
    NotEstablished,
}

/// Track whether analysis has established any const-context contract; runtime support is a separate
/// capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstCapability {
    /// The generated helper path requires runtime execution; this is not a const-context promise.
    RuntimeOnly,
    /// Integer syntax alone cannot establish const-compatible Rust helper calls.
    NotEstablished,
}

/// Collect obligations emission must preserve about occurrence count, lazy branches, and defined C
/// operations.
#[derive(Clone, Debug, Default, Serialize)]
pub struct EvaluationContract {
    /// Distinct semantic obligations needed to preserve the supported C invocation family.
    pub requirements: Vec<EvaluationRequirement>,
}

/// A semantic obligation that prevents convenient Rust lowering from changing observable C behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationRequirement {
    /// Each surviving C argument occurrence must retain its evaluation behavior.
    PreserveParameterOccurrences,
    /// Conditional and short-circuit paths must evaluate only the selected C branch.
    PreserveLazyBranches,
    /// The invocation family retains C’s allowed operand ordering rather than inventing a sequencing
    /// guarantee.
    UnspecifiedOperandOrder,
    /// Generated operations are valid only within the defined C operation domain.
    ExcludeUndefinedCOperations,
    /// Signed arithmetic must follow the inspected compiler overflow policy.
    RespectSignedOverflowProfile,
}

/// A referenced macro and its possible physical origins for generation ordering and skip
/// explanations.
#[derive(Clone, Debug, Serialize)]
pub struct MacroDependency {
    /// The original C identifier retained for reports, symbol lookup, and readable generated output.
    pub name: String,
    /// The semantic or structural category kept separate from representation and source spelling.
    pub kind: MacroKind,
    /// Physical source origins or their resolution status used for ownership filtering and auditing.
    pub provenance: Option<SourceSpan>,
    /// Exact lexical reference ranges where established; compiler closures can provide only possible
    /// origins.
    pub uses: Vec<TokenRange>,
}

/// The C semantic helper families identified during symbolic integer analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HelperRequirement {
    /// Promote narrow integer operands using C rank and range rather than Rust’s operand type rules.
    IntegerPromotions,
    /// Select common C arithmetic types while retaining rank and signedness identities.
    UsualArithmeticConversions,
    /// Convert integer representations with the original C cast semantics.
    IntegerCasts,
    /// Perform admitted arithmetic under the inspected C operation domain and overflow policy.
    IntegerArithmetic,
    /// Apply C integer promotions and result identities to bit operations.
    BitwiseOperations,
    /// Compare converted operands and produce the original C truth result.
    Comparisons,
    /// Interpret C truth values for conditionals and short-circuit operators.
    CTruth,
    /// Apply C division/remainder rules within their defined operand domain.
    DivisionRemainder,
    /// Apply C shift promotions and operand-domain constraints.
    Shifts,
    /// Select helpers according to the recorded undefined, wrapping, or trapping policy.
    SignedOverflowPolicy,
}

/// Combine the source arena with symbolic C types and compiler-resolved constants for Rust lowering.
#[derive(Clone, Debug, Serialize)]
pub struct AnalyzedExpression {
    /// Original arena and statement metadata shared by type analysis and Rust emission.
    pub syntax: Expression,
    /// Parallel to `syntax.nodes`; references are expression node indices.
    pub types: Vec<TypeExpression>,
    /// Resolved integer identifiers tied to source nodes without replacing their readable names.
    pub constants: Vec<ResolvedConstant>,
    /// Pure integer subexpressions whose zero ICE value both Clang frontends proved.
    /// Native operands and stored values never acquire this source-level identity.
    pub integer_zero_constants: BTreeSet<NodeId>,
}

/// Symbolic C result types retain promotion/common-type rules instead of selecting a
/// signature from one sample invocation or collapsing equal-width C identities.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypeExpression {
    /// No C value is produced; this identity supports statement and discard contexts.
    Void,
    /// A compiler intrinsic admitted only as the callee of a direct call.
    /// It has no address or first-class function value in generated Rust.
    Builtin,
    /// A concrete non-integer C type obtained from a declaration or type operand.
    External {
        /// Concrete inspected C type facts used by this symbolic result.
        ty: TypeInfo,
    },
    /// A generic expression whose C type is constrained by generated capabilities.
    Deferred,
    /// A target-resolved fundamental C integer result identity.
    Concrete {
        /// Concrete inspected C type facts used by this symbolic result.
        ty: IntegerType,
    },
    /// A formal hole whose role is supplied by the caller rather than one sampled type.
    Parameter {
        /// Analyzed parameter position whose caller-supplied C type remains symbolic.
        index: usize,
    },
    /// An operand-dependent integer promotion retained symbolically for generic lowering.
    Promotion {
        /// Node whose result type determines the C integer promotion.
        operand: NodeId,
    },
    /// The C usual-arithmetic-conversion result of two independently typed operands.
    Common {
        /// First operand type participating in C usual arithmetic conversions.
        left: NodeId,
        /// Second operand type participating in C usual arithmetic conversions.
        right: NodeId,
    },
}

/// Attach a verified C integer identity/value to its original syntax node while retaining readable
/// literal metadata.
#[derive(Clone, Debug, Serialize)]
pub struct ResolvedConstant {
    /// Expression node that originally referenced this named integer constant.
    pub node: NodeId,
    /// The original C identifier retained for reports, symbol lookup, and readable generated output.
    pub name: String,
    /// Concrete C integer identity of the referenced constant, before Rust binding reconciliation.
    pub ty: IntegerType,
    /// The exact constant magnitude, operand node, or wrapped storage retained by this
    /// representation.
    pub value: IntegerValue,
    /// Original C literal metadata when the resolved identifier has a literal-backed spelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub literal: Option<crate::IntegerLiteral>,
}

/// Analyze a definition only in the profile/environment/catalog resolved together by
/// the frontend. Work is bounded by replacement tokens; dependencies are recorded,
/// never recursively expanded. Catalog/environment lookups use their ordered maps.
pub fn analyze(frontend: &FrontendOutput, name: &str) -> MacroAnalysis {
    analyze_active(frontend, name, frontend.environment().active.get(name))
}

/// Analyze the selected final active definition with the frontend declaration constants.
pub(crate) fn analyze_active(
    frontend: &FrontendOutput,
    name: &str,
    active: Option<&ActiveMacro>,
) -> MacroAnalysis {
    analyze_active_with_constants(frontend, name, active, &BTreeMap::new(), false)
}

/// Build symbolic syntax, operand roles, type expressions, and invocation contracts from one resolved
/// active definition.
pub(crate) fn analyze_active_with_constants(
    frontend: &FrontendOutput,
    name: &str,
    active: Option<&ActiveMacro>,
    object_constants: &BTreeMap<String, IntegerConstant>,
    compiler_expanded: bool,
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
            origin: ParameterOrigin::Formal,
            roles: vec![ParameterRole::Unused],
            uses: Vec::new(),
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
        } else if let Some(dependency) = frontend.environment().active.get(&token.spelling)
            && !object_constants.contains_key(&token.spelling)
        {
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
    if let Some(dependency) = result.dependencies.first().filter(|_| !compiler_expanded) {
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
    let mut syntax = match parse_replacement(body, &parameters, |name| {
        recognized_type(name, catalog, &profile.target)
    }) {
        Ok(syntax) => syntax,
        Err(error) => return result.syntax_error(error),
    };
    // A C replacement list can capture identifiers from the invocation's scope.
    // macro_rules hygiene prevents that capture. Expose precisely those AST
    // value identifiers as extra holes, without confusing member names or types
    // with values and without inventing a signature from one sample invocation.
    if compiler_expanded {
        let mut captures = BTreeMap::new();
        let locals = syntax
            .statement_body
            .iter()
            .flat_map(|body| body.walk())
            .filter_map(|statement| match statement {
                Statement::Declaration { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        let captures_name = |name: &str| {
            !locals.contains(name)
                && !catalog.integer_constants.contains_key(name)
                && !object_constants.contains_key(name)
                && !catalog.variables.contains_key(name)
                && !catalog.functions.contains_key(name)
                && !catalog.builtins.contains_key(name)
                && !catalog.builtin_unavailable.contains_key(name)
        };
        for node in &syntax.nodes {
            let ExpressionKind::Group { operand } = node.kind else { continue };
            let ExpressionKind::Identifier { name } = &syntax.nodes[operand].kind else {
                continue;
            };
            if captures_name(name) && could_start_cast_operand(&body[node.tokens.end..]) {
                return result.skip(
                    SkipReasonCode::UnsupportedType,
                    format!("parenthesized caller-scope identifier {name:?} may be a C cast type or value; its role is not established"),
                    Some(node.tokens),
                );
            }
        }
        for node in &mut syntax.nodes {
            let ExpressionKind::Identifier { name } = &node.kind else { continue };
            if !captures_name(name) {
                continue;
            }
            let parameter = *captures.entry(name.clone()).or_insert_with(|| {
                let index = result.parameters.len();
                result.parameters.push(ParameterAnalysis {
                    name: name.clone(),
                    origin: ParameterOrigin::FreeIdentifier,
                    roles: vec![ParameterRole::Unknown],
                    uses: Vec::new(),
                });
                index
            });
            result.parameters[parameter]
                .uses
                .push(ParameterUse { tokens: node.tokens, grouped: true });
            node.kind = ExpressionKind::Parameter { index: parameter };
        }
        if result.parameters.len() > 64 {
            return result.skip(SkipReasonCode::BudgetExceeded, "macro formals and caller-scope identifiers exceed the 64-argument normalization budget", None);
        }
        for parameter in &mut result.parameters[parameters.len()..] {
            parameter.uses.sort_by_key(|usage| usage.tokens.start);
        }
    }
    // Parent links establish whether each hole, rather than a larger surrounding
    // expression, is explicitly grouped. Traversal remains linear in arena size.
    let mut grouped = vec![false; syntax.nodes.len()];
    for node in &syntax.nodes {
        if let ExpressionKind::Group { operand } = node.kind {
            grouped[operand] = true;
        }
        if let ExpressionKind::Call { arguments, .. } = &node.kind {
            // Call delimiters protect a whole argument's precedence. They do
            // not protect a hole inside a larger argument, such as f(x * 2).
            for argument in arguments {
                grouped[*argument] = true;
            }
        }
    }
    for (index, node) in syntax.nodes.iter().enumerate() {
        if let ExpressionKind::OffsetOf { record, fields } = &node.kind {
            let roles = match record {
                OffsetRecord::Parameter { index } => Some((*index, ParameterRole::Type)),
                OffsetRecord::Named { .. } => None,
            }
            .into_iter()
            .chain(fields.iter().filter_map(|field| match field {
                OffsetComponent::Parameter { index } => {
                    Some((*index, ParameterRole::FieldDesignator))
                }
                OffsetComponent::Named { .. } => None,
            }));
            for (parameter, role) in roles {
                let parameter = &mut result.parameters[parameter];
                parameter.roles.retain(|role| *role != ParameterRole::Unknown);
                if !parameter.roles.contains(&role) {
                    parameter.roles.push(role);
                }
                for usage in &mut parameter.uses {
                    if usage.tokens.start >= node.tokens.start
                        && usage.tokens.end <= node.tokens.end
                    {
                        usage.grouped = true;
                    }
                }
            }
        }
        if let ExpressionKind::Member { field_parameter: Some(parameter), .. } = node.kind {
            let parameter = &mut result.parameters[parameter];
            parameter.roles.retain(|role| *role != ParameterRole::Unknown);
            if !parameter.roles.contains(&ParameterRole::Identifier) {
                parameter.roles.push(ParameterRole::Identifier);
            }
            for usage in &mut parameter.uses {
                if usage.tokens.start + 1 == node.tokens.end {
                    usage.grouped = true;
                }
            }
        }
        if let ExpressionKind::Parameter { index: parameter } = node.kind {
            // Type operands consume lexical holes without creating value leaves.
            // Uses are sorted by token offset, so locating a value leaf is bounded.
            let occurrence = result.parameters[parameter]
                .uses
                .binary_search_by_key(&node.tokens.start, |usage| usage.tokens.start)
                .expect("each parsed hole was recorded lexically");
            let is_capture = result.parameters[parameter].origin == ParameterOrigin::FreeIdentifier;
            let usage = &mut result.parameters[parameter].uses[occurrence];
            debug_assert_eq!(usage.tokens, node.tokens);
            usage.grouped = grouped[index] || is_capture;
            if !result.parameters[parameter].roles.contains(&ParameterRole::Value) {
                result.parameters[parameter].roles.retain(|role| *role != ParameterRole::Unknown);
                result.parameters[parameter].roles.push(ParameterRole::Value);
            }
        }
        if let ExpressionKind::TypeParameterCast { parameter, operand, .. } = node.kind {
            let parameter = &mut result.parameters[parameter];
            parameter.roles.retain(|role| *role != ParameterRole::Unknown);
            if !parameter.roles.contains(&ParameterRole::Type) {
                parameter.roles.push(ParameterRole::Type);
            }
            for usage in &mut parameter.uses {
                if usage.tokens.start >= node.tokens.start
                    && usage.tokens.end <= syntax.nodes[operand].tokens.start
                {
                    usage.grouped = true;
                }
            }
        }
        if let ExpressionKind::SizeOfTypeParameter { parameter, .. }
        | ExpressionKind::AlignOfTypeParameter { parameter, .. } = node.kind
        {
            let parameter = &mut result.parameters[parameter];
            parameter.roles = vec![ParameterRole::Type];
            for usage in &mut parameter.uses {
                if usage.tokens.start >= node.tokens.start && usage.tokens.end <= node.tokens.end {
                    usage.grouped = true;
                }
            }
        }
    }
    let atomic_arguments =
        result.parameters.iter().flat_map(|parameter| &parameter.uses).any(|usage| !usage.grouped);
    match analyze_types(&syntax, catalog, &profile.target, object_constants) {
        Ok((types, constants, mut helpers)) => {
            let root = &syntax.nodes[syntax.root];
            let statements = syntax.statement_body.is_some();
            let returning =
                syntax.statement_body.as_ref().is_some_and(|body| body.return_tokens.is_some());
            let statement_boundary =
                syntax.statement_body.as_ref().is_some_and(|body| body.requires_boundary);
            if syntax.statement_body.as_ref().is_some_and(|body| {
                body.walk().any(|statement| matches!(statement, Statement::If { .. }))
            }) {
                helpers.insert(HelperRequirement::CTruth);
            }
            let expression_boundary = !statements
                && !matches!(
                    root.kind,
                    ExpressionKind::Group { .. }
                        | ExpressionKind::IntegerLiteral { .. }
                        | ExpressionKind::Identifier { .. }
                        | ExpressionKind::Empty
                        | ExpressionKind::Parameter { .. }
                        | ExpressionKind::Call { .. }
                        | ExpressionKind::Member { .. }
                        | ExpressionKind::Index { .. }
                        | ExpressionKind::OffsetOf { .. }
                );
            if expression_boundary && !compiler_expanded {
                return result.skip(
                    SkipReasonCode::InvocationGrouping,
                    "the complete replacement expression is not parenthesized or atomic; Rust expression grouping would change its interaction with caller operators",
                    Some(root.tokens),
                );
            }
            result.expression = Some(AnalyzedExpression {
                syntax,
                types,
                constants,
                integer_zero_constants: BTreeSet::new(),
            });
            result.required_helpers = helpers.into_iter().collect();
            result.invocation = if statement_boundary {
                InvocationContract::ExplicitStatementBoundary
            } else if returning {
                InvocationContract::ReturningStatements
            } else if statements {
                InvocationContract::Statements
            } else if expression_boundary {
                InvocationContract::ExplicitExpressionBoundary
            } else if atomic_arguments {
                InvocationContract::AtomicArguments
            } else {
                InvocationContract::ParenthesizedScalarExpressions
            };
            result.evaluation.requirements = vec![
                EvaluationRequirement::PreserveParameterOccurrences,
                EvaluationRequirement::ExcludeUndefinedCOperations,
                EvaluationRequirement::RespectSignedOverflowProfile,
            ];
            if statements {
                result.const_capability = ConstCapability::RuntimeOnly;
            }
            if result.expression.as_ref().is_some_and(|expression| {
                expression.syntax.nodes.iter().any(|node| match &node.kind {
                    ExpressionKind::Binary { operator, .. } => {
                        !matches!(operator, BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr)
                    }
                    ExpressionKind::Call { arguments, .. } => arguments.len() >= 2,
                    _ => false,
                })
            }) {
                result.evaluation.requirements.push(EvaluationRequirement::UnspecifiedOperandOrder);
            }
            if result.expression.as_ref().is_some_and(|expression| {
                expression.syntax.statement_body.as_ref().is_some_and(|body| {
                    body.walk().any(|statement| matches!(statement, Statement::If { .. }))
                }) || expression.syntax.nodes.iter().any(|node| {
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

/// Turn parser/analysis refusals into stable reports while retaining source provenance.
impl MacroAnalysis {
    /// Replace candidate state with a structured refusal while retaining the macro identity and
    /// provenance.
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

    /// Translate a parser failure into stable analysis categories and original-token ranges.
    fn syntax_error(self, error: SyntaxError) -> Self {
        let code = match error.kind {
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

/// Recognize the ambiguity after an unresolved single-name group without mistaking empty calls
/// or terminal postfix updates for casts; comments do not change the C token grammar.
fn could_start_cast_operand(tokens: &[Token]) -> bool {
    let mut tokens = tokens.iter().filter(|token| token.kind != TokenKind::Comment);
    let Some(next) = tokens.next() else { return false };
    match next.spelling.as_str() {
        "(" => tokens.next().is_some_and(|token| token.spelling != ")"),
        "++" | "--" => tokens.next().is_some_and(|token| {
            matches!(token.kind, TokenKind::Identifier | TokenKind::Keyword | TokenKind::Literal)
                || matches!(token.spelling.as_str(), "(" | "+" | "-" | "*" | "&" | "~" | "!")
        }),
        "+" | "-" | "*" | "&" | "~" | "!" => true,
        _ => matches!(next.kind, TokenKind::Identifier | TokenKind::Keyword | TokenKind::Literal),
    }
}

/// Read the unexpanded function signature and reject malformed or variadic formal lists before
/// parsing the body.
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

/// The stable skip category and explanation produced when a symbolic C type cannot be established.
type TypeFailure = (SkipReasonCode, String, TokenRange);
/// Parallel symbolic result types and resolved constants returned by the type-analysis pass.
type TypeAnalysis = (Vec<TypeExpression>, Vec<ResolvedConstant>, BTreeSet<HelperRequirement>);

/// Constant probes accept only expressions already covered by the integer model.
/// In particular, evaluating a variable or a function is never a way to discover a constant.
pub(crate) fn validate_constant_expression(
    frontend: &FrontendOutput,
    tokens: &[Token],
) -> Result<(), String> {
    let catalog = frontend.declarations();
    let target = &frontend.profile().target;
    let expression = parse_expression(tokens, &[], |name| recognized_type(name, catalog, target))
        .map_err(|error| error.message)?;
    let (types, _, _) = analyze_types(&expression, catalog, target, &BTreeMap::new())
        .map_err(|(_, message, _)| message)?;
    for (index, node) in expression.nodes.iter().enumerate() {
        if !matches!(
            node.kind,
            ExpressionKind::IntegerLiteral { .. }
                | ExpressionKind::Identifier { .. }
                | ExpressionKind::Group { .. }
                | ExpressionKind::Cast { .. }
                | ExpressionKind::Unary { .. }
                | ExpressionKind::Binary { .. }
                | ExpressionKind::Conditional { .. }
        ) || matches!(&node.kind, ExpressionKind::Identifier { name } if !catalog.integer_constants.contains_key(name))
        {
            return Err(
                "constant probes cannot evaluate calls, variables, memory or mutation".into()
            );
        }
        let TypeExpression::Concrete { ty } = types[index] else {
            return Err("constant expression has no concrete integer type".into());
        };
        if ty.bits > 64 {
            return Err(
                "constant folding requires every intermediate integer to fit within 64 bits".into(),
            );
        }
        let literal_operand = |mut operand: NodeId| {
            while let ExpressionKind::Group { operand: child } = expression.nodes[operand].kind {
                operand = child;
            }
            match &expression.nodes[operand].kind {
                ExpressionKind::IntegerLiteral { literal } => Some(literal.value),
                _ => None,
            }
        };
        // Compiler constant evaluation alone does not prove a defined C operation.
        // Keep uncertain operations in the expansion, where the semantic support
        // checks their domain instead of silently replacing them with a value.
        match node.kind {
            ExpressionKind::Binary {
                operator: BinaryOperator::Divide | BinaryOperator::Remainder,
                right,
                ..
            } if literal_operand(right).is_none_or(|value| value == 0) => {
                return Err("constant division requires a positive literal divisor to exclude zero and signed division overflow".into());
            }
            ExpressionKind::Binary {
                operator: BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight,
                right,
                ..
            } if literal_operand(right).is_none_or(|value| value >= u128::from(ty.bits)) => {
                return Err("constant shifting requires a literal count within the promoted left operand's width".into());
            }
            ExpressionKind::Binary {
                operator:
                    BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::ShiftLeft,
                ..
            } if ty.signed && frontend.profile().signed_overflow != SignedOverflow::Wrapping => {
                return Err(
                    "signed constant arithmetic must remain checked under this overflow policy"
                        .into(),
                );
            }
            ExpressionKind::Unary { operator: UnaryOperator::Negate, operand }
                if ty.signed
                    && frontend.profile().signed_overflow != SignedOverflow::Wrapping
                    && literal_operand(operand).is_none() =>
            {
                return Err("signed constant negation needs a representable literal operand under this overflow policy".into());
            }
            _ => {}
        }
    }
    Ok(())
}

/// Propagate symbolic C types through the expression arena and validate catalog references without
/// choosing sample argument types.
fn analyze_types(
    expression: &Expression,
    catalog: &DeclarationCatalog,
    target: &TargetFacts,
    object_constants: &BTreeMap<String, IntegerConstant>,
) -> Result<TypeAnalysis, TypeFailure> {
    let mut locals = HashMap::new();
    for statement in expression.statement_body.iter().flat_map(|body| body.walk()) {
        let Statement::Declaration { name, type_name, tokens, .. } = statement else {
            continue;
        };
        if catalog.variables.contains_key(name)
            || catalog.functions.contains_key(name)
            || catalog.builtins.contains_key(name)
            || catalog.builtin_unavailable.contains_key(name)
            || catalog.integer_constants.contains_key(name)
            || object_constants.contains_key(name)
            || catalog.types.contains_key(name)
        {
            return Err((
                SkipReasonCode::Statement,
                format!("local {name:?} would shadow a compiler-owned declaration"),
                *tokens,
            ));
        }
        let Some(ty) = resolve_type_info(type_name, catalog, target) else {
            return Err((
                SkipReasonCode::UnsupportedType,
                format!("local {name:?} has no established compiler type {type_name:?}"),
                *tokens,
            ));
        };
        if matches!(ty.category, TypeCategory::Void | TypeCategory::Function | TypeCategory::Other)
        {
            return Err((
                SkipReasonCode::UnsupportedType,
                format!("local {name:?} does not have a supported C object type"),
                *tokens,
            ));
        }
        locals.insert(name.as_str(), ty);
    }
    validate_local_initialization(expression, &locals)?;
    let mut types = Vec::<TypeExpression>::with_capacity(expression.nodes.len());
    let mut constants = Vec::new();
    let mut helpers = BTreeSet::new();
    let int = target.integers[&IntegerKind::Int];
    let mut direct_callees = vec![false; expression.nodes.len()];
    for node in &expression.nodes {
        let ExpressionKind::Call { mut callee, .. } = node.kind else { continue };
        loop {
            direct_callees[callee] = true;
            match expression.nodes[callee].kind {
                ExpressionKind::Group { operand } => callee = operand,
                _ => break,
            }
        }
    }
    for (index, node) in expression.nodes.iter().enumerate() {
        let ty = match &node.kind {
            ExpressionKind::Empty => TypeExpression::Void,
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
                if let Some(reason) = catalog.builtin_unavailable.get(name) {
                    return Err((
                        SkipReasonCode::DynamicBuiltin,
                        format!("{name}: {reason}"),
                        node.tokens,
                    ));
                }
                if catalog.builtins.contains_key(name) {
                    if !direct_callees[index] {
                        return Err((
                            SkipReasonCode::DynamicBuiltin,
                            format!(
                                "{name}: compiler intrinsic is supported only in a direct call"
                            ),
                            node.tokens,
                        ));
                    }
                    types.push(TypeExpression::Builtin);
                    continue;
                }
                if let Some(ty) = locals.get(name.as_str()) {
                    types.push(declared_type(ty, target));
                    continue;
                }
                let Some(constant) =
                    object_constants.get(name).or_else(|| catalog.integer_constants.get(name))
                else {
                    if let Some(ty) =
                        catalog.variables.get(name).or_else(|| catalog.functions.get(name))
                    {
                        if ty.category == TypeCategory::Other
                            && !matches!(
                                catalog
                                    .type_shapes
                                    .get(&ty.canonical_spelling)
                                    .map(|shape| &shape.kind),
                                Some(crate::TypeShapeKind::Array { .. })
                            )
                        {
                            return Err((
                                SkipReasonCode::UnsupportedType,
                                format!("{name}: declaration has an unmodeled compiler type"),
                                node.tokens,
                            ));
                        }
                        types.push(declared_type(ty, target));
                        continue;
                    }
                    return Err((
                        SkipReasonCode::UnknownIdentifier,
                        format!(
                            "{name}: identifier has no compiler-owned declaration or integer constant"
                        ),
                        node.tokens,
                    ));
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
                    literal: constant.literal.clone(),
                });
                TypeExpression::Concrete { ty }
            }
            ExpressionKind::Group { operand } => types[*operand].clone(),
            ExpressionKind::Cast { type_name, .. } => {
                if type_name == "void" {
                    types.push(TypeExpression::Void);
                    continue;
                }
                let Some(ty) = integer_type(type_name, catalog, target) else {
                    if let Some(info) = resolve_type_info(type_name, catalog, target) {
                        if info.category == TypeCategory::Other {
                            return Err((
                                SkipReasonCode::UnsupportedType,
                                format!("cast type {type_name:?} has an unmodeled compiler type"),
                                node.tokens,
                            ));
                        }
                        types.push(declared_type(&info, target));
                        continue;
                    }
                    return Err((
                        SkipReasonCode::UnsupportedType,
                        format!("cast type {type_name:?} has no established compiler type"),
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
            ExpressionKind::Comma { right, .. } => types[*right].clone(),
            ExpressionKind::Call { callee, arguments } => {
                let mut callee = *callee;
                while let ExpressionKind::Group { operand } = expression.nodes[callee].kind {
                    callee = operand;
                }
                let direct = if let ExpressionKind::Identifier { name } =
                    &expression.nodes[callee].kind
                {
                    catalog.builtins.get(name).map(|builtin| &builtin.signature).or_else(|| {
                        catalog.function_signatures.get(name).map(|function| &function.signature)
                    })
                } else {
                    None
                };
                let indirect = if let (None, TypeExpression::External { ty }) =
                    (direct, &types[callee])
                {
                    let ty = match catalog
                        .type_shapes
                        .get(&ty.canonical_spelling)
                        .map(|shape| &shape.kind)
                    {
                        Some(crate::TypeShapeKind::Pointer { pointee }) => pointee,
                        _ => ty,
                    };
                    match catalog.type_shapes.get(&ty.canonical_spelling).map(|shape| &shape.kind) {
                        Some(crate::TypeShapeKind::Function { signature }) => Some(signature),
                        _ => {
                            return Err((
                                SkipReasonCode::Call,
                                "callee's compiler-owned type is not a function pointer".into(),
                                node.tokens,
                            ));
                        }
                    }
                } else {
                    None
                };
                if let Some(signature) = direct.or(indirect) {
                    if signature.parameters.is_none() {
                        return Err((
                            SkipReasonCode::Call,
                            "callee has no complete C prototype".into(),
                            node.tokens,
                        ));
                    }
                    if signature.variadic {
                        return Err((
                            SkipReasonCode::Variadic,
                            "variadic calls require a separate argument promotion contract".into(),
                            node.tokens,
                        ));
                    }
                    if signature
                        .parameters
                        .as_ref()
                        .is_none_or(|parameters| parameters.len() != arguments.len())
                    {
                        return Err((
                            SkipReasonCode::Call,
                            "invocation does not match a complete C prototype".into(),
                            node.tokens,
                        ));
                    }
                    declared_type(&signature.result, target)
                } else if matches!(
                    types[callee],
                    TypeExpression::Concrete { .. } | TypeExpression::Void
                ) {
                    return Err((
                        SkipReasonCode::Call,
                        "callee is a noncallable C scalar expression".into(),
                        node.tokens,
                    ));
                } else {
                    // Generic arguments and field projections acquire their exact
                    // prototype through generated, compiler-owned Call capabilities.
                    TypeExpression::Deferred
                }
            }
            ExpressionKind::Member { .. }
            | ExpressionKind::Index { .. }
            | ExpressionKind::Dereference { .. }
            | ExpressionKind::AddressOf { .. } => TypeExpression::Deferred,
            ExpressionKind::Assignment { place, .. } => types[*place].clone(),
            ExpressionKind::Update { operand, .. } => types[*operand].clone(),
            ExpressionKind::SizeOfType { .. }
            | ExpressionKind::SizeOfExpression { .. }
            | ExpressionKind::AlignOfType { .. }
            | ExpressionKind::SizeOfTypeParameter { .. }
            | ExpressionKind::AlignOfTypeParameter { .. } => {
                TypeExpression::Concrete { ty: target.integers[&target.size_type] }
            }
            ExpressionKind::OffsetOf { record, .. } => {
                if !target.offsetof_supported {
                    return Err((SkipReasonCode::DynamicBuiltin, "compiler offsetof type and semantics are not established under this profile".into(), node.tokens));
                }
                if let OffsetRecord::Named { name } = record
                    && resolve_type_info(name, catalog, target).is_none_or(|ty| {
                        ty.category != TypeCategory::Record
                            || ty.size.is_none()
                            || ty.alignment.is_none()
                    })
                {
                    return Err((
                        SkipReasonCode::UnsupportedType,
                        format!("{name}: offsetof requires a complete C record"),
                        node.tokens,
                    ));
                }
                TypeExpression::Concrete { ty: target.integers[&target.size_type] }
            }
            ExpressionKind::TypeParameterCast { .. } => TypeExpression::Deferred,
        };
        types.push(ty);
    }
    Ok((types, constants, helpers))
}

/// Initialization is proved at complete statement and condition boundaries.
/// Every path that can continue must initialize a local before its later read.
/// Taking an address or passing it to a function does not prove that C wrote it.
fn validate_local_initialization(
    expression: &Expression,
    locals: &HashMap<&str, TypeInfo>,
) -> Result<(), TypeFailure> {
    let Some(body) = expression.statement_body.as_ref().filter(|_| !locals.is_empty()) else {
        return Ok(());
    };
    /// Reject full-expression combinations whose evaluation or mutation constraints cannot be safely
    /// established.
    fn check_full_expression<'a>(
        expression: &Expression,
        root: NodeId,
        locals: &HashMap<&'a str, TypeInfo>,
        initialized: &mut HashSet<&'a str>,
    ) -> Result<(), TypeFailure> {
        check_local_reads(expression, root, locals, initialized)?;
        let mut root = root;
        while let ExpressionKind::Group { operand } = expression.nodes[root].kind {
            root = operand;
        }
        if let ExpressionKind::Assignment { operator: None, mut place, .. } =
            expression.nodes[root].kind
        {
            while let ExpressionKind::Group { operand } = expression.nodes[place].kind {
                place = operand;
            }
            if let ExpressionKind::Identifier { name } = &expression.nodes[place].kind
                && let Some((&name, _)) = locals.get_key_value(name.as_str())
            {
                initialized.insert(name);
            }
        }
        Ok(())
    }
    // A terminating path contributes no state to a later join: C never reaches it.
    // Each surviving branch must establish every local read after the conditional.
    /// Validate statement scopes and declared locals alongside the expression arena before emission.
    fn check_statements<'a>(
        expression: &Expression,
        statements: &'a [Statement],
        locals: &HashMap<&'a str, TypeInfo>,
        initialized: &mut HashSet<&'a str>,
    ) -> Result<bool, TypeFailure> {
        for statement in statements {
            match statement {
                Statement::Declaration { name, initializer, .. } => {
                    if let Some(initializer) = initializer {
                        check_full_expression(expression, *initializer, locals, initialized)?;
                        initialized.insert(name.as_str());
                    }
                }
                Statement::Expression { expression: root, .. } => {
                    check_full_expression(expression, *root, locals, initialized)?;
                }
                Statement::Block { statements, .. } => {
                    if check_statements(expression, statements, locals, initialized)? {
                        return Ok(true);
                    }
                }
                Statement::Return { expression: root, .. } => {
                    check_local_reads(expression, *root, locals, initialized)?;
                    return Ok(true);
                }
                Statement::If { condition, then_branch, else_branch, .. } => {
                    check_full_expression(expression, *condition, locals, initialized)?;
                    let mut then_initialized = initialized.clone();
                    let then_returns = check_statements(
                        expression,
                        std::slice::from_ref(then_branch.as_ref()),
                        locals,
                        &mut then_initialized,
                    )?;
                    let mut else_initialized = initialized.clone();
                    let else_returns = if let Some(else_branch) = else_branch {
                        check_statements(
                            expression,
                            std::slice::from_ref(else_branch.as_ref()),
                            locals,
                            &mut else_initialized,
                        )?
                    } else {
                        false
                    };
                    match (then_returns, else_returns) {
                        (true, true) => return Ok(true),
                        (false, true) => *initialized = then_initialized,
                        (true, false) => *initialized = else_initialized,
                        (false, false) => {
                            then_initialized.retain(|name| else_initialized.contains(name));
                            *initialized = then_initialized;
                        }
                    }
                }
            }
        }
        Ok(false)
    }
    check_statements(expression, &body.statements, locals, &mut HashSet::new()).map(|_| ())
}

/// Require supported straight-line initialization before a local read can become a generated Rust
/// expression.
fn check_local_reads(
    expression: &Expression,
    root: NodeId,
    locals: &HashMap<&str, TypeInfo>,
    initialized: &HashSet<&str>,
) -> Result<(), TypeFailure> {
    // Keep the walk iterative: left-associated C expressions can have a deep arena
    // even when the recursive parser's nesting budget is satisfied.
    let mut work = vec![(root, true)];
    while let Some((index, read)) = work.pop() {
        let node = &expression.nodes[index];
        match &node.kind {
            ExpressionKind::Identifier { name }
                if read
                    && locals.contains_key(name.as_str())
                    && !initialized.contains(name.as_str()) =>
            {
                return Err((
                    SkipReasonCode::Statement,
                    format!("local {name:?} is read before definite initialization is established"),
                    node.tokens,
                ));
            }
            ExpressionKind::Group { operand } => work.push((*operand, read)),
            ExpressionKind::AddressOf { operand } => work.push((*operand, false)),
            ExpressionKind::Unary { operand, .. }
            | ExpressionKind::Cast { operand, .. }
            | ExpressionKind::TypeParameterCast { operand, .. }
            | ExpressionKind::Dereference { operand }
            | ExpressionKind::Update { operand, .. } => work.push((*operand, true)),
            ExpressionKind::Member { base, indirect, .. } => work.push((*base, read || *indirect)),
            ExpressionKind::Index { base, index } => work.extend([(*base, true), (*index, true)]),
            ExpressionKind::Assignment { operator, place, value } => {
                work.extend([(*place, operator.is_some()), (*value, true)]);
            }
            ExpressionKind::Binary { left, right, .. } | ExpressionKind::Comma { left, right } => {
                work.extend([(*left, true), (*right, true)])
            }
            ExpressionKind::Conditional { condition, then_value, else_value } => {
                work.extend([(*condition, true), (*then_value, true), (*else_value, true)]);
            }
            ExpressionKind::Call { callee, arguments } => {
                work.push((*callee, true));
                work.extend(arguments.iter().map(|argument| (*argument, true)));
            }
            // sizeof is unevaluated; type operands have no value reads.
            _ => {}
        }
    }
    Ok(())
}

/// Reject target facts that cannot support the analysis rules used for integer and literal semantics.
fn validate_target(target: &TargetFacts) -> Result<(), String> {
    if target
        .integers
        .get(&target.size_type)
        .is_none_or(|ty| ty.signed || ty.kind == IntegerKind::Bool)
    {
        return Err("target facts do not establish an unsigned size_t integer identity".into());
    }
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

/// Ask whether a spelling resolves through the compiler catalog or supported fundamental type
/// grammar.
fn recognized_type(name: &str, catalog: &DeclarationCatalog, target: &TargetFacts) -> bool {
    resolve_type_info(name, catalog, target).is_some()
}

/// Find an original named declaration, including qualifiers, rather than inferring identity from Rust
/// storage.
fn declared_type(info: &TypeInfo, target: &TargetFacts) -> TypeExpression {
    match info.category {
        TypeCategory::Integer(kind) => TypeExpression::Concrete { ty: target.integers[&kind] },
        TypeCategory::Void => TypeExpression::Void,
        _ => TypeExpression::External { ty: info.clone() },
    }
}

/// Resolve C specifiers and pointer qualifier layers against the inspected catalog and target facts.
pub(crate) fn resolve_type_info(
    name: &str,
    catalog: &DeclarationCatalog,
    target: &TargetFacts,
) -> Option<TypeInfo> {
    if let Some(info) = catalog.types.get(name) {
        return Some(info.clone());
    }
    if let Some(shape) = catalog.type_shapes.get(name) {
        return Some(shape.ty.clone());
    }
    // Typedefs can carry their own qualifiers. Preserve those compiler facts
    // before reducing a qualified integer name to its fundamental kind.
    // Pointer declarators qualify each star separately, so only strip leaf names.
    if !name.contains('*') {
        let unqualified = name
            .split_whitespace()
            .filter(|word| !matches!(*word, "const" | "volatile"))
            .collect::<Vec<_>>()
            .join(" ");
        if let Some(mut ty) = catalog
            .types
            .get(&unqualified)
            .or_else(|| catalog.type_shapes.get(&unqualified).map(|shape| &shape.ty))
            .cloned()
        {
            ty.is_const |= name.split_whitespace().any(|word| word == "const");
            ty.is_volatile |= name.split_whitespace().any(|word| word == "volatile");
            return Some(ty);
        }
    }
    if let Some(ty) = integer_type(name, catalog, target) {
        return Some(TypeInfo {
            spelling: name.into(),
            canonical_spelling: name.into(),
            category: TypeCategory::Integer(ty.kind),
            size: Some(u64::from(ty.bits / target.char_bits)),
            alignment: None,
            is_const: name.split_whitespace().any(|word| word == "const"),
            is_volatile: name.split_whitespace().any(|word| word == "volatile"),
        });
    }
    if name == "void" {
        return Some(TypeInfo {
            spelling: name.into(),
            canonical_spelling: name.into(),
            category: TypeCategory::Void,
            size: None,
            alignment: None,
            is_const: false,
            is_volatile: false,
        });
    }
    if let Some((pointee, _)) = name.rsplit_once('*') {
        resolve_type_info(pointee.trim(), catalog, target)?;
        return Some(TypeInfo {
            spelling: name.into(),
            canonical_spelling: name.into(),
            category: TypeCategory::Pointer,
            size: Some(u64::from(target.pointer_bits / target.char_bits)),
            alignment: None,
            is_const: name.rsplit_once('*')?.1.split_whitespace().any(|word| word == "const"),
            is_volatile: name.rsplit_once('*')?.1.split_whitespace().any(|word| word == "volatile"),
        });
    }
    None
}

/// Resolve a supported C integer spelling while retaining the fundamental identity and target rank.
pub(crate) fn integer_type(
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
/// Interpret the supported combinations and orderings of fundamental C integer specifiers.
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

/// Select a literal C type using radix, suffix, value, and the target-specific standard candidate
/// order.
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

/// Compute the maximum positive literal magnitude representable by a C integer type.
fn maximum(ty: IntegerType) -> u128 {
    if ty.kind == IntegerKind::Bool {
        return 1;
    }
    let value_bits = ty.bits - u32::from(ty.signed);
    if value_bits == 128 { u128::MAX } else { (1_u128 << value_bits) - 1 }
}

/// Describe an integer promotion symbolically so generic operands retain their C conversion rules.
fn promotion(ty: IntegerType, target: &TargetFacts) -> IntegerType {
    let int = target.integers[&IntegerKind::Int];
    if ty.rank > int.rank || matches!(ty.kind, IntegerKind::Int | IntegerKind::UnsignedInt) {
        return ty;
    }
    if maximum(ty) <= maximum(int) { int } else { target.integers[&IntegerKind::UnsignedInt] }
}

/// Apply integer promotion to a concrete C kind using target range and rank facts.
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

/// Apply C usual arithmetic conversions without replacing distinct C identities by equal Rust widths.
fn common_type(
    types: &[TypeExpression],
    left: NodeId,
    right: NodeId,
    target: &TargetFacts,
) -> TypeExpression {
    match (&types[left], &types[right]) {
        (TypeExpression::Void, TypeExpression::Void) => TypeExpression::Void,
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

/// Select the corresponding unsigned C identity needed by usual arithmetic conversions.
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

/// Exercise this phase’s semantic boundaries with owned fixtures.
/// These regressions check accepted proofs and explicit refusals without changing production
/// headers or weakening the C identity and evaluation contracts.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        ActiveMacro, BuildInputs, ByteOrder, CompilationProfile, CompilerIdentity, MacroEnvironment,
    };
    use crate::{MacroDefinition, MacroInventory};

    /// Construct explicit LP64 integer facts for pure analysis tests rather than borrowing the host
    /// ABI.
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
            size_type: IntegerKind::UnsignedLong,
            ptrdiff_type: crate::IntegerKind::Long,
            preferred_alignments: Default::default(),
            arm_float_abi: None,
            ppc64_elf_abi: None,
            offsetof_supported: true,
            function_pointer: crate::PointerLayout { size: 8, alignment: 8 },
            char_bits: 8,
            char_is_signed: true,
            ascii_execution_charset: true,
            byte_order: ByteOrder::Little,
            c_standard: Some(201710),
            integers,
            floating_point: Default::default(),
        }
    }

    /// Parse one test literal with production token rules before checking its C type selection.
    fn literal(spelling: &str, value: u128, radix: u8, unsigned: bool, long: u8) -> IntegerLiteral {
        IntegerLiteral {
            spelling: spelling.into(),
            value,
            radix,
            suffix: crate::syntax::IntegerSuffix { unsigned, long },
        }
    }

    /// Checks literal types follow radix suffix and target.
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

    /// Checks promotions and common types preserve c identity.
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

    /// Checks character constants require the verified execution charset.
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

    /// Construct a small owned declaration environment for symbolic-analysis regression tests.
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
        let environment = MacroEnvironment {
            active: [(
                "F".into(),
                ActiveMacro { definition, provenance: ActiveProvenance::Resolved },
            )]
            .into_iter()
            .collect(),
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
            dependencies: crate::MacroDependencyGraph::from_environment(&environment),
            environment,
            declarations: DeclarationCatalog::default(),
            inventory: MacroInventory { macros: Vec::new(), diagnostics: Vec::new() },
        }
    }

    /// Checks constraints occurrences and skips are explicit.
    #[test]
    fn constraints_occurrences_and_skips_are_explicit() {
        let input = frontend(&["(", "(", "x", ")", "+", "(", "x", ")", ")"]);
        let analysis = analyze(&input, "F");
        assert!(matches!(analysis.status, AnalysisStatus::Candidate));
        assert_eq!(analysis.parameters[0].uses.len(), 2);
        assert!(analysis.parameters[0].uses.iter().all(|usage| usage.grouped));
        assert_eq!(analysis.parameters[1].roles, vec![ParameterRole::Unused]);
        assert_eq!(analysis.const_capability, ConstCapability::NotEstablished);
        for (body, expected) in [
            (vec!["x", "+", "1"], SkipReasonCode::InvocationGrouping),
            (vec!["(", "x", ")", "?", "1", ":", "2"], SkipReasonCode::InvocationGrouping),
            (vec!["(", "unused", ")", "(", "x", ")"], SkipReasonCode::TypeParameter),
            (vec!["(", "char", "*", ")", "(", "x", ")"], SkipReasonCode::InvocationGrouping),
        ] {
            let AnalysisStatus::Skipped { reason } = analyze(&frontend(&body), "F").status else {
                panic!("unsupported syntax must skip")
            };
            assert_eq!(reason.code, expected);
        }
    }

    /// Checks replacement grouping is checked separately from parameter grouping.
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

    /// Checks binary literal candidates require the inspected C23 profile.
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

    /// Checks concrete specifier order and missing pairs.
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

    /// Checks qualified integer typedefs retain intrinsic qualifiers and identity.
    #[test]
    fn qualified_integer_typedefs_retain_intrinsic_qualifiers_and_identity() {
        let target = target(64);
        let mut catalog = DeclarationCatalog::default();
        for (name, canonical, kind, is_const, is_volatile, spellings) in [
            (
                "ConstWord",
                "const unsigned long",
                IntegerKind::UnsignedLong,
                true,
                false,
                ["volatile ConstWord", "ConstWord volatile", "const volatile ConstWord"],
            ),
            (
                "VolatileWord",
                "volatile unsigned long long",
                IntegerKind::UnsignedLongLong,
                false,
                true,
                ["const VolatileWord", "VolatileWord const", "volatile const VolatileWord"],
            ),
        ] {
            let mut expected = TypeInfo {
                spelling: canonical.into(),
                canonical_spelling: canonical.into(),
                category: TypeCategory::Integer(kind),
                size: Some(8),
                alignment: Some(8),
                is_const,
                is_volatile,
            };
            catalog.types.insert(name.into(), expected.clone());
            expected.is_const = true;
            expected.is_volatile = true;
            for spelling in spellings {
                assert_eq!(resolve_type_info(spelling, &catalog, &target), Some(expected.clone()));
            }
        }
    }

    /// Checks pointer qualifiers apply to their own declarator layer.
    #[test]
    fn pointer_qualifiers_apply_to_their_own_declarator_layer() {
        let target = target(64);
        let mut catalog = DeclarationCatalog::default();
        catalog.types.insert(
            "ConstWord".into(),
            TypeInfo {
                spelling: "const unsigned long".into(),
                canonical_spelling: "const unsigned long".into(),
                category: TypeCategory::Integer(IntegerKind::UnsignedLong),
                size: Some(8),
                alignment: Some(8),
                is_const: true,
                is_volatile: false,
            },
        );
        for (spelling, is_const, is_volatile) in [
            ("volatile ConstWord *", false, false),
            ("ConstWord * volatile", false, true),
            ("ConstWord * const", true, false),
            ("ConstWord * const * volatile", false, true),
        ] {
            let ty = resolve_type_info(spelling, &catalog, &target).unwrap();
            assert_eq!(ty.category, TypeCategory::Pointer);
            assert_eq!((ty.is_const, ty.is_volatile), (is_const, is_volatile));
        }
        let leaf = resolve_type_info("volatile ConstWord", &catalog, &target).unwrap();
        assert!(leaf.is_const && leaf.is_volatile);

        let pointer_alias = TypeInfo {
            spelling: "unsigned long *const".into(),
            canonical_spelling: "unsigned long *const".into(),
            category: TypeCategory::Pointer,
            size: Some(8),
            alignment: Some(8),
            is_const: true,
            is_volatile: false,
        };
        catalog.types.insert("ConstPointer".into(), pointer_alias.clone());
        let qualified = resolve_type_info("volatile ConstPointer", &catalog, &target).unwrap();
        assert!(qualified.is_const && qualified.is_volatile);
        assert_eq!(qualified.canonical_spelling, pointer_alias.canonical_spelling);
        let outer = resolve_type_info("volatile ConstPointer *", &catalog, &target).unwrap();
        assert!(!outer.is_const && !outer.is_volatile);
    }

    /// Checks terminal return locals use declared types and are never caller captures.
    #[test]
    fn terminal_return_locals_use_declared_types_and_are_never_caller_captures() {
        let input = frontend(&[
            "do", "{", "unsigned", "short", "local", ";", "local", "=", "(", "x", ")", ";",
            "return", "local", ";", "}", "while", "(", "0", ")",
        ]);
        let analysis = analyze_active_with_constants(
            &input,
            "F",
            input.environment.active.get("F"),
            &BTreeMap::new(),
            true,
        );
        assert!(matches!(analysis.status, AnalysisStatus::Candidate));
        assert_eq!(analysis.invocation, InvocationContract::ReturningStatements);
        assert_eq!(analysis.const_capability, ConstCapability::RuntimeOnly);
        assert_eq!(analysis.parameters.len(), 2);
        let expression = analysis.expression.unwrap();
        for (index, node) in expression.syntax.nodes.iter().enumerate() {
            if matches!(&node.kind, ExpressionKind::Identifier { name } if name == "local") {
                assert!(matches!(
                    expression.types[index],
                    TypeExpression::Concrete { ty } if ty.kind == IntegerKind::UnsignedShort
                ));
            }
        }
    }

    /// Checks return declarations cannot rebind compiler owned names.
    #[test]
    fn return_declarations_cannot_rebind_compiler_owned_names() {
        let mut input =
            frontend(&["{", "int", "local", "=", "1", ";", "return", "local", ";", "}"]);
        let ty = resolve_type_info("int", &input.declarations, &input.profile.target).unwrap();
        input.declarations.variables.insert("local".into(), ty);
        let AnalysisStatus::Skipped { reason } = analyze(&input, "F").status else {
            panic!("global shadowing must stay outside the supported invocation family")
        };
        assert_eq!(reason.code, SkipReasonCode::Statement);
        assert_eq!(reason.tokens, Some(TokenRange { start: 1, end: 6 }));
        assert!(reason.message.contains("compiler-owned declaration"));
    }

    /// Checks returning local reads require proved straight line initialization.
    #[test]
    fn returning_local_reads_require_proved_straight_line_initialization() {
        for body in [
            vec!["{", "int", "local", ";", "return", "local", ";", "}"],
            vec!["{", "int", "local", ";", "local", "+=", "1", ";", "return", "local", ";", "}"],
            vec!["{", "int", "local", "=", "local", ";", "return", "local", ";", "}"],
            vec![
                "{", "int", "local", ";", "(", "void", ")", "&", "local", ";", "return", "local",
                ";", "}",
            ],
        ] {
            let AnalysisStatus::Skipped { reason } = analyze(&frontend(&body), "F").status else {
                panic!("unproved initialization must skip: {body:?}")
            };
            assert_eq!(reason.code, SkipReasonCode::Statement);
            assert!(reason.message.contains("definite initialization"));
        }
        for body in [
            vec!["{", "int", "local", "=", "1", ";", "return", "local", ";", "}"],
            vec![
                "{", "int", "local", ";", "(", "local", ")", "=", "1", ";", "return", "local", ";",
                "}",
            ],
            vec!["{", "int", "local", ";", "return", "sizeof", "local", ";", "}"],
        ] {
            assert!(
                matches!(analyze(&frontend(&body), "F").status, AnalysisStatus::Candidate),
                "{body:?}"
            );
        }
    }

    /// Checks nested terminal return preserves caller captures.
    #[test]
    fn nested_terminal_return_preserves_caller_captures() {
        let input = frontend(&[
            "do", "{", "fcinfo", "->", "isnull", "=", "1", ";", "do", "{", "return", "(", "int",
            ")", "0", ";", "}", "while", "(", "0", ")", ";", "}", "while", "(", "0", ")",
        ]);
        let analysis = analyze_active_with_constants(
            &input,
            "F",
            input.environment.active.get("F"),
            &BTreeMap::new(),
            true,
        );
        assert!(matches!(analysis.status, AnalysisStatus::Candidate));
        let captures = analysis
            .parameters
            .iter()
            .filter(|parameter| parameter.origin == ParameterOrigin::FreeIdentifier)
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(captures, ["fcinfo"]);
        assert_eq!(analysis.invocation, InvocationContract::ReturningStatements);
        let body = analysis.expression.unwrap().syntax.statement_body.unwrap();
        assert_eq!(
            body.walk()
                .filter(|statement| matches!(statement, Statement::Expression { .. }))
                .count(),
            1
        );
        assert_eq!(
            body.walk().filter(|statement| matches!(statement, Statement::Return { .. })).count(),
            1
        );
    }
}
