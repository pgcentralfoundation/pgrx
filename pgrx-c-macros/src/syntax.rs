//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Bounded parsing of expressions and structured statements in C macro replacement lists.
//!
//! Parameters remain holes. This parser never expands a preprocessing token or guesses a
//! declaration. The arena preserves explicit grouping and parameter occurrences, and avoids
//! recursively serialized trees for long left-associated expressions.

//! The parser is deliberately separate from preprocessing and declaration discovery. Formal
//! arguments remain holes, and a known-type predicate resolves only syntax supported by the
//! inspected catalog. The node arena keeps source grouping and occurrences without a recursive
//! expression tree. Structured statement parsing shares that arena and tracks scopes, returns,
//! and invocation boundaries needed by later analysis and emission.

use crate::{Token, TokenKind};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Parse ordered blocks, conditionals, locals, and returns within the shared bounded expression
/// arena.
mod statements;

/// Bound replacement-list work before constructing a syntax arena.
const MAX_TOKENS: usize = 4096;
/// Bound nested parser recursion while leaving long left-associated expressions in the iterative
/// arena.
const MAX_DEPTH: usize = 64;

/// Half-open indices into a macro's replacement-token list, including comment tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenRange {
    /// Inclusive original replacement-token index, retaining comment offsets.
    pub start: usize,
    /// Exclusive original replacement-token index, including original comment positions.
    pub end: usize,
}

/// An index into [`Expression::nodes`]. Children precede their parents.
pub type NodeId = usize;

/// An ordered syntax arena retaining C grouping and parameter holes for analysis and emission.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expression {
    /// Arena nodes in child-before-parent order for iterative semantic analysis.
    pub nodes: Vec<ExpressionNode>,
    /// Complete expression root, first return operand, or an Empty node for void statement bodies.
    pub root: NodeId,
    /// A complete statement replacement with expressions in this same arena.
    /// The root is the first return operand, or `Empty` when there is no return.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_body: Option<StatementBody>,
}

/// A complete structured replacement with source scopes and return/boundary facts used by statement
/// emission.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatementBody {
    /// Source-order statements retained with explicit lexical scopes.
    pub statements: Vec<Statement>,
    /// Original replacement-list token coordinates used to explain syntax and reconstruct source.
    pub tokens: TokenRange,
    /// The first return in source order, including returns inside branches.
    pub return_tokens: Option<TokenRange>,
    /// Every path through the supported statement tree returns from its caller.
    pub always_returns: bool,
    /// An unbraced root `if` needs a caller boundary to preserve C's dangling-else contract.
    pub requires_boundary: bool,
}

/// Traverse explicit statement scopes in source order for later analysis and emission.
impl StatementBody {
    /// Visit statements in source order, yielding blocks before their children.
    /// Scopes remain explicit; callers that execute statements must honor them.
    pub fn walk(&self) -> impl Iterator<Item = &Statement> {
        let mut scopes = vec![self.statements.iter()];
        std::iter::from_fn(move || {
            loop {
                let scope = scopes.last_mut()?;
                if let Some(statement) = scope.next() {
                    match statement {
                        Statement::Block { statements, .. } => scopes.push(statements.iter()),
                        Statement::If { then_branch, else_branch, .. } => {
                            if let Some(otherwise) = else_branch {
                                scopes.push(std::slice::from_ref(otherwise.as_ref()).iter());
                            }
                            scopes.push(std::slice::from_ref(then_branch.as_ref()).iter());
                        }
                        _ => {}
                    }
                    return Some(statement);
                }
                scopes.pop();
            }
        })
    }
}

/// The bounded C statement grammar accepted for source-ordered, scope-preserving macro lowering.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Statement {
    /// Evaluate one complete C expression statement in source order and discard its result.
    Expression {
        /// Operand node in the shared expression arena, retaining source position and C evaluation
        /// rules.
        expression: NodeId,
        /// Original token range of this complete statement, including its scope or semicolon
        /// boundary.
        tokens: TokenRange,
    },
    /// Introduce a checked C local type/name and optional initializer within an explicit lexical
    /// scope.
    Declaration {
        /// Original local identifier checked for rebinding and definite initialization before
        /// emission.
        name: String,
        /// Original declared C type spelling checked against the inspected declaration catalog.
        type_name: String,
        /// Optional initializer node whose existence contributes to definite-initialization checks.
        initializer: Option<NodeId>,
        /// Original token range of this complete statement, including its scope or semicolon
        /// boundary.
        tokens: TokenRange,
    },
    /// Keep a nested lexical scope and source-order child statements explicit.
    Block {
        /// Source-order statements retained with explicit lexical scopes.
        statements: Vec<Statement>,
        /// Original token range of this complete statement, including its scope or semicolon
        /// boundary.
        tokens: TokenRange,
    },
    /// Return the typed operand from the macro caller rather than from a generated helper closure.
    Return {
        /// Operand node in the shared expression arena, retaining source position and C evaluation
        /// rules.
        expression: NodeId,
        /// Original token range of this complete statement, including its scope or semicolon
        /// boundary.
        tokens: TokenRange,
    },
    /// Execute only the selected statement branch and preserve the C dangling-else structure.
    If {
        /// Condition node whose C truth conversion controls one lazy branch.
        condition: NodeId,
        /// Statement executed only when the condition has C true value.
        then_branch: Box<Statement>,
        /// Optional statement executed only when the condition has C false value.
        else_branch: Option<Box<Statement>>,
        /// Original token range of this complete statement, including its scope or semicolon
        /// boundary.
        tokens: TokenRange,
    },
}

/// One arena operation and its original replacement-token range for diagnostics and source
/// reconstruction.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionNode {
    /// The original C operation, retained until analysis establishes types and emission selects
    /// capabilities.
    pub kind: ExpressionKind,
    /// Original replacement-list token coordinates used to explain syntax and reconstruct source.
    pub tokens: TokenRange,
}

/// Keep evaluated operands, places, type operands, and structural designators distinct until semantic
/// lowering.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExpressionKind {
    /// A void/empty statement root without an evaluated return operand.
    Empty,
    /// A formal hole whose role is supplied by the caller rather than one sampled type.
    Parameter {
        /// The arena, formal, or token position referenced by this structural representation.
        index: usize,
    },
    /// A source integer/basic-character spelling before target-dependent C type selection.
    IntegerLiteral {
        /// Original C spelling, magnitude, radix, and suffix before target-dependent type selection.
        literal: IntegerLiteral,
    },
    /// An original named constant/declaration or later explicit caller capture awaiting analysis.
    Identifier {
        /// Original identifier spelling awaiting declaration lookup or explicit caller-capture
        /// analysis.
        name: String,
    },
    /// Explicit delimiter grouping retained for source-preserving layout or C precedence.
    Group {
        /// The child expression node, retained separately so promotion, grouping, or unevaluated use
        /// remains visible.
        operand: NodeId,
    },
    /// A unary C operation whose operand remains a distinct arena node.
    Unary {
        /// Original C operator identity, including the underlying binary operation of compound
        /// assignment.
        operator: UnaryOperator,
        /// The child expression node, retained separately so promotion, grouping, or unevaluated use
        /// remains visible.
        operand: NodeId,
    },
    /// A binary C operation with original operator identity and independently typed operands.
    Binary {
        /// Original C operator identity, including the underlying binary operation of compound
        /// assignment.
        operator: BinaryOperator,
        /// The left operand node; its C type and evaluation context remain independently represented.
        left: NodeId,
        /// The right operand node; lazy or ordered evaluation is established by the enclosing
        /// operator.
        right: NodeId,
    },
    /// An independently established concrete C type conversion, retaining the source type spelling.
    Cast {
        /// Original concrete C type operand, resolved by analysis rather than guessed from Rust
        /// storage.
        type_name: String,
        /// The child expression node, retained separately so promotion, grouping, or unevaluated use
        /// remains visible.
        operand: NodeId,
    },
    /// A lazy C conditional expression with separate condition and value branches.
    Conditional {
        /// C condition node evaluated before choosing exactly one value branch.
        condition: NodeId,
        /// Value node evaluated only for a C true condition.
        then_value: NodeId,
        /// Value node evaluated only for a C false condition.
        else_value: NodeId,
    },
    /// C sequencing of a discarded left expression followed by the right value.
    Comma {
        /// The left operand node; its C type and evaluation context remain independently represented.
        left: NodeId,
        /// The right operand node; lazy or ordered evaluation is established by the enclosing
        /// operator.
        right: NodeId,
    },
    /// An evaluated callee and ordered argument nodes awaiting callable capability checks.
    Call {
        /// Evaluated callable node whose signature and native identity analysis must establish.
        callee: NodeId,
        /// Argument expression nodes in source order, without invented C sequencing guarantees.
        arguments: Vec<NodeId>,
    },
    /// A direct or indirect record member place, including structural member-name formals.
    Member {
        /// Record or array expression whose place/value semantics remain distinct during lowering.
        base: NodeId,
        /// An original C member designator validated before being embedded into probe or generated
        /// syntax.
        field: String,
        /// Formal index when the member designator comes from a caller identifier fragment.
        field_parameter: Option<usize>,
        /// Whether C uses -> through a pointer rather than . on a record place.
        indirect: bool,
    },
    /// An array/pointer element place with an evaluated C index expression.
    Index {
        /// Record or array expression whose place/value semantics remain distinct during lowering.
        base: NodeId,
        /// The arena, formal, or token position referenced by this structural representation.
        index: NodeId,
    },
    /// A pointed-to place whose loading and qualifiers are resolved during lowering.
    Dereference {
        /// The child expression node, retained separately so promotion, grouping, or unevaluated use
        /// remains visible.
        operand: NodeId,
    },
    /// An address computation over a place, distinct from loading its stored value.
    AddressOf {
        /// The child expression node, retained separately so promotion, grouping, or unevaluated use
        /// remains visible.
        operand: NodeId,
    },
    /// Simple or compound assignment retaining a place and its original result semantics.
    Assignment {
        /// Original C operator identity, including the underlying binary operation of compound
        /// assignment.
        operator: Option<BinaryOperator>,
        /// Assignment destination node, retained as a place instead of an eagerly loaded value.
        place: NodeId,
        /// Right-hand assignment operand, converted to the original destination’s C type.
        value: NodeId,
    },
    /// Prefix/postfix increment or decrement with distinct old-versus-new result behavior.
    Update {
        /// The child expression node, retained separately so promotion, grouping, or unevaluated use
        /// remains visible.
        operand: NodeId,
        /// Whether update adds one rather than subtracting one using the operand’s C type.
        increment: bool,
        /// Whether update returns the original value instead of the stored updated value.
        postfix: bool,
    },
    /// A non-evaluated sizeof operand resolved from the inspected C type catalog.
    SizeOfType {
        /// Original concrete C type operand, resolved by analysis rather than guessed from Rust
        /// storage.
        type_name: String,
    },
    /// An unevaluated expression whose type determines the C size result.
    SizeOfExpression {
        /// The child expression node, retained separately so promotion, grouping, or unevaluated use
        /// remains visible.
        operand: NodeId,
    },
    /// A non-evaluated alignment operand using an independently established C type.
    AlignOfType {
        /// Original concrete C type operand, resolved by analysis rather than guessed from Rust
        /// storage.
        type_name: String,
    },
    /// A formal type operand with bounded pointer/qualifier layers for sizeof.
    SizeOfTypeParameter {
        /// Formal index supplying a type operand, independently established from ambiguous call
        /// syntax.
        parameter: usize,
        /// Number of supported pointer layers applied to the supplied formal type.
        pointers: u8,
        /// The qualifier on this C layer, retained to restrict writes and type compatibility.
        is_const: bool,
    },
    /// A formal type operand with bounded pointer/qualifier layers for alignment.
    AlignOfTypeParameter {
        /// Formal index supplying a type operand, independently established from ambiguous call
        /// syntax.
        parameter: usize,
        /// Number of supported pointer layers applied to the supplied formal type.
        pointers: u8,
        /// The qualifier on this C layer, retained to restrict writes and type compatibility.
        is_const: bool,
    },
    /// A formal-type conversion admitted only with independent evidence for its type role.
    TypeParameterCast {
        /// Formal index supplying a type operand, independently established from ambiguous call
        /// syntax.
        parameter: usize,
        /// Number of supported pointer layers applied to the supplied formal type.
        pointers: u8,
        /// The qualifier on this C layer, retained to restrict writes and type compatibility.
        is_const: bool,
        /// The child expression node, retained separately so promotion, grouping, or unevaluated use
        /// remains visible.
        operand: NodeId,
    },
    /// A non-evaluated record/member path preserving structural type and field holes.
    OffsetOf {
        /// Non-evaluated record designator for an offsetof computation.
        record: OffsetRecord,
        /// Non-evaluated ordered member path used to establish a record offset.
        fields: Vec<OffsetComponent>,
    },
}

/// The record type operand of offsetof, supplied by an inspected spelling or a caller type hole.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OffsetRecord {
    /// An original inspected spelling rather than a caller-supplied structural hole.
    Named {
        /// The original C identifier retained for reports, symbol lookup, and readable generated
        /// output.
        name: String,
    },
    /// A formal hole whose role is supplied by the caller rather than one sampled type.
    Parameter {
        /// Formal supplying the offsetof record type as a structural Rust type fragment.
        index: usize,
    },
}

/// One non-evaluated member designator in an offsetof path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OffsetComponent {
    /// An original inspected spelling rather than a caller-supplied structural hole.
    Named {
        /// The original C identifier retained for reports, symbol lookup, and readable generated
        /// output.
        name: String,
    },
    /// A formal hole whose role is supplied by the caller rather than one sampled type.
    Parameter {
        /// Formal supplying one offsetof member name as a structural Rust identifier fragment.
        index: usize,
    },
}

/// C unary operators whose promotions and truth rules are applied later by analysis/runtime helpers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnaryOperator {
    /// C unary plus, including the operand’s integer promotion.
    Plus,
    /// C unary minus within the inspected arithmetic and overflow domain.
    Negate,
    /// C bitwise complement after the required integer promotion.
    BitwiseNot,
    /// C logical negation producing the C truth result.
    LogicalNot,
}

/// C binary operator identity kept separate from Rust syntax and conversion semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinaryOperator {
    /// C multiplication with usual arithmetic conversions.
    Multiply,
    /// C quotient within the defined zero/overflow operand domain.
    Divide,
    /// C remainder paired with truncating integer division.
    Remainder,
    /// C addition, retaining arithmetic or pointer semantics for later lowering.
    Add,
    /// C subtraction, retaining arithmetic or pointer semantics for later lowering.
    Subtract,
    /// C left shift with independent promoted operand identities and shift-count restrictions.
    ShiftLeft,
    /// C right shift under the admitted signedness and target semantic profile.
    ShiftRight,
    /// C strict less-than comparison after compatible operand conversion.
    Less,
    /// C less-than-or-equal comparison after compatible operand conversion.
    LessEqual,
    /// C strict greater-than comparison after compatible operand conversion.
    Greater,
    /// C greater-than-or-equal comparison after compatible operand conversion.
    GreaterEqual,
    /// C equality after compatible operand conversion.
    Equal,
    /// C inequality after compatible operand conversion.
    NotEqual,
    /// C bitwise AND with usual integer conversions.
    BitAnd,
    /// C bitwise XOR with usual integer conversions.
    BitXor,
    /// C bitwise OR with usual integer conversions.
    BitOr,
    /// C short-circuit conjunction; the right operand is evaluated only after a true left operand.
    LogicalAnd,
    /// C short-circuit disjunction; the right operand is evaluated only after a false left operand.
    LogicalOr,
}

/// Integer spelling before target-dependent selection of its C type.
///
/// Supported ordinary character constants retain their spelling and use an unsuffixed
/// decimal magnitude, which has C `int` type for the bounded ASCII value range. Analysis
/// must establish the target's basic execution character set before accepting those constants.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegerLiteral {
    /// Full original C literal spelling preserved in readable generated Rust when supported.
    pub spelling: String,
    /// The positive magnitude; unary minus is a separate C operator.
    pub value: u128,
    /// Source base controlling C’s decimal versus nondecimal type-candidate order.
    pub radix: u8,
    /// Unsigned and long-rank suffix facts used during target-dependent literal selection.
    pub suffix: IntegerSuffix,
}

/// Retain unsignedness and long-rank suffixes for target-dependent C literal type selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegerSuffix {
    /// Whether a U suffix restricts type candidates to unsigned C identities.
    pub unsigned: bool,
    /// Zero, one, or two `L` characters.
    pub long: u8,
}

/// A parser refusal with original token coordinates rather than a partial syntax tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SyntaxError {
    /// The semantic or structural category kept separate from representation and source spelling.
    pub kind: SyntaxErrorKind,
    /// Original replacement-list token coordinates used to explain syntax and reconstruct source.
    pub tokens: TokenRange,
    /// Human-readable detail explaining the compiler observation or unsupported construct.
    pub message: String,
}

/// Stable parser failure families translated into analysis skip reasons.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SyntaxErrorKind {
    /// The complete replacement cannot be parsed with the bounded supported grammar.
    InvalidExpression,
    /// The literal lies outside the verified ordinary integer/basic-character subset.
    UnsupportedLiteral,
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
}

/// A significant token paired with its original replacement-list index after comments are filtered.
struct Lexeme<'a> {
    /// Borrowed significant token, keeping scanner spelling and lexical category.
    token: &'a Token,
    /// Original token index before comments are filtered, used by source-range diagnostics.
    index: usize,
}

/// Bounded parser state with one arena and independently established formal-type evidence.
struct Parser<'a, F> {
    /// Significant token view with original indices preserved.
    tokens: Vec<Lexeme<'a>>,
    /// Next significant token to consume during bounded parsing.
    position: usize,
    /// Formal-name to index lookup so every occurrence refers to the same operand hole.
    parameters: HashMap<&'a str, usize>,
    /// Formals independently established as type operands before ambiguous expressions are reparsed.
    type_parameters: HashSet<usize>,
    /// Potential formal applications that need independent type evidence before becoming casts.
    ambiguous_casts: HashSet<NodeId>,
    /// One shared expression arena for the complete expression or structured statement replacement.
    nodes: Vec<ExpressionNode>,
    /// Catalog-backed predicate admitting concrete C type spellings into the cast grammar.
    is_type: F,
    /// Original replacement-token count used to report complete boundaries after comment filtering.
    original_len: usize,
}

/// Parse only a complete expression; trailing tokens are never accepted as raw syntax.
pub(crate) fn parse_expression(
    tokens: &[Token],
    parameters: &[String],
    is_type: impl Fn(&str) -> bool,
) -> Result<Expression, SyntaxError> {
    parse(tokens, parameters, is_type, false)
}

/// Parse an expression or a complete structured statement replacement.
pub(crate) fn parse_replacement(
    tokens: &[Token],
    parameters: &[String],
    is_type: impl Fn(&str) -> bool,
) -> Result<Expression, SyntaxError> {
    parse(tokens, parameters, is_type, true)
}

/// Initialize the bounded parser, resolve supported type holes, and require a complete expression or
/// statement body.
fn parse(
    tokens: &[Token],
    parameters: &[String],
    is_type: impl Fn(&str) -> bool,
    allow_statements: bool,
) -> Result<Expression, SyntaxError> {
    if tokens.len() > MAX_TOKENS {
        return Err(SyntaxError {
            kind: SyntaxErrorKind::BudgetExceeded,
            tokens: TokenRange { start: 0, end: tokens.len() },
            message: format!("replacement exceeds the {MAX_TOKENS}-token analysis budget"),
        });
    }
    let mut parser = Parser {
        tokens: tokens
            .iter()
            .enumerate()
            .filter(|(_, token)| token.kind != TokenKind::Comment)
            .map(|(index, token)| Lexeme { token, index })
            .collect(),
        position: 0,
        parameters: parameters.iter().enumerate().map(|(i, name)| (name.as_str(), i)).collect(),
        type_parameters: HashSet::new(),
        ambiguous_casts: HashSet::new(),
        nodes: Vec::new(),
        is_type,
        original_len: tokens.len(),
    };
    if parser.tokens.is_empty() {
        return Ok(Expression {
            nodes: vec![ExpressionNode {
                kind: ExpressionKind::Empty,
                tokens: TokenRange { start: 0, end: tokens.len() },
            }],
            root: 0,
            statement_body: None,
        });
    }
    let (mut root, mut statement_body) = parser.replacement(allow_statements)?;
    if parser.position != parser.tokens.len() {
        return Err(parser.unexpected());
    }
    parser.type_parameters = parser
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(index, node)| match node.kind {
            ExpressionKind::TypeParameterCast { parameter, .. }
                if !parser.ambiguous_casts.contains(&index) =>
            {
                Some(parameter)
            }
            ExpressionKind::TypeParameterCast { .. } => None,
            ExpressionKind::SizeOfTypeParameter { parameter, .. } => Some(parameter),
            ExpressionKind::AlignOfTypeParameter { parameter, pointers, is_const }
                if pointers != 0 || is_const =>
            {
                Some(parameter)
            }
            ExpressionKind::OffsetOf { record: OffsetRecord::Parameter { index }, .. } => {
                Some(index)
            }
            _ => None,
        })
        .collect();
    if let Some(&index) = parser
        .ambiguous_casts
        .iter()
        .filter(|&&index| {
            let ExpressionKind::TypeParameterCast { parameter, .. } = parser.nodes[index].kind
            else {
                unreachable!("only provisional formal casts enter the ambiguity set")
            };
            !parser.type_parameters.contains(&parameter)
        })
        .min()
    {
        return Err(SyntaxError {
            kind: SyntaxErrorKind::TypeParameter,
            tokens: parser.nodes[index].tokens,
            message: "parenthesized macro parameter may be a C cast type or callable value; its role is not established".into(),
        });
    }
    // GNU alignment also accepts expressions, so bare alignment cannot prove
    // that an ambiguous application is a cast. Its existing admitted type
    // family can still resolve sizeof operands after that ambiguity check.
    parser.type_parameters.extend(parser.nodes.iter().filter_map(|node| match node.kind {
        ExpressionKind::AlignOfTypeParameter { parameter, .. } => Some(parameter),
        _ => None,
    }));
    if !parser.type_parameters.is_empty()
        && (!parser.ambiguous_casts.is_empty()
            || parser
                .nodes
                .iter()
                .any(|node| matches!(node.kind, ExpressionKind::SizeOfExpression { .. })))
    {
        // Reparse once so later type uses also resolve earlier sizeof operands.
        parser.position = 0;
        parser.nodes.clear();
        parser.ambiguous_casts.clear();
        (root, statement_body) = parser.replacement(allow_statements)?;
        if parser.position != parser.tokens.len() {
            return Err(parser.unexpected());
        }
    }
    Ok(Expression { nodes: parser.nodes, root, statement_body })
}

/// Parse bounded replacement grammar while preserving original indices and independently proved type
/// holes.
impl<F: Fn(&str) -> bool> Parser<'_, F> {
    /// Parse precedence-sensitive C operators into arena nodes while retaining source grouping and
    /// occurrence order.
    fn expression(&mut self, minimum: u8, depth: usize) -> Result<NodeId, SyntaxError> {
        if depth >= MAX_DEPTH {
            return Err(self.error(
                SyntaxErrorKind::BudgetExceeded,
                format!("expression exceeds the {MAX_DEPTH}-level analysis budget"),
            ));
        }
        let mut left = self.prefix(depth)?;
        loop {
            if minimum <= 15 {
                let start = self.nodes[left].tokens.start;
                match self.spelling() {
                    Some("(") => {
                        self.position += 1;
                        let mut arguments = Vec::new();
                        if self.spelling() != Some(")") {
                            loop {
                                arguments.push(self.expression(1, depth + 1)?);
                                if self.spelling() != Some(",") {
                                    break;
                                }
                                self.position += 1;
                            }
                        }
                        self.expect(")")?;
                        let end = self.tokens[self.position - 1].index + 1;
                        left = self.push(
                            ExpressionKind::Call { callee: left, arguments },
                            TokenRange { start, end },
                        );
                        continue;
                    }
                    Some(".") | Some("->") => {
                        let indirect = self.spelling() == Some("->");
                        self.position += 1;
                        let Some(field) = self.tokens.get(self.position) else {
                            return Err(self
                                .error(SyntaxErrorKind::InvalidExpression, "missing member name"));
                        };
                        if !matches!(field.token.kind, TokenKind::Identifier | TokenKind::Keyword) {
                            return Err(self.unexpected());
                        }
                        let name = field.token.spelling.clone();
                        let end = field.index + 1;
                        self.position += 1;
                        left = self.push(
                            ExpressionKind::Member {
                                base: left,
                                field_parameter: self.parameters.get(name.as_str()).copied(),
                                field: name,
                                indirect,
                            },
                            TokenRange { start, end },
                        );
                        continue;
                    }
                    Some("[") => {
                        self.position += 1;
                        let index = self.expression(0, depth + 1)?;
                        self.expect("]")?;
                        let end = self.tokens[self.position - 1].index + 1;
                        left = self.push(
                            ExpressionKind::Index { base: left, index },
                            TokenRange { start, end },
                        );
                        continue;
                    }
                    Some("++") | Some("--") => {
                        let increment = self.spelling() == Some("++");
                        let end = self.tokens[self.position].index + 1;
                        self.position += 1;
                        left = self.push(
                            ExpressionKind::Update { operand: left, increment, postfix: true },
                            TokenRange { start, end },
                        );
                        continue;
                    }
                    _ => {}
                }
            }
            if self.spelling() == Some(",") && minimum == 0 {
                self.position += 1;
                let right = self.expression(1, depth + 1)?;
                let tokens = TokenRange {
                    start: self.nodes[left].tokens.start,
                    end: self.nodes[right].tokens.end,
                };
                left = self.push(ExpressionKind::Comma { left, right }, tokens);
                continue;
            }
            if let Some(operator) = self.spelling().and_then(assignment_operator)
                && minimum <= 1
            {
                self.position += 1;
                let value = self.expression(1, depth + 1)?;
                let tokens = TokenRange {
                    start: self.nodes[left].tokens.start,
                    end: self.nodes[value].tokens.end,
                };
                left =
                    self.push(ExpressionKind::Assignment { operator, place: left, value }, tokens);
                continue;
            }
            if self.spelling() == Some("?") && minimum <= 2 {
                self.position += 1;
                let then_value = self.expression(0, depth + 1)?;
                self.expect(":")?;
                let else_value = self.expression(2, depth + 1)?;
                let tokens = TokenRange {
                    start: self.nodes[left].tokens.start,
                    end: self.nodes[else_value].tokens.end,
                };
                left = self.push(
                    ExpressionKind::Conditional { condition: left, then_value, else_value },
                    tokens,
                );
                continue;
            }
            let Some((operator, precedence)) = self.spelling().and_then(binary_operator) else {
                break;
            };
            let precedence = precedence + 2;
            if precedence < minimum {
                break;
            }
            self.position += 1;
            let right = self.expression(precedence + 1, depth + 1)?;
            let tokens = TokenRange {
                start: self.nodes[left].tokens.start,
                end: self.nodes[right].tokens.end,
            };
            left = self.push(ExpressionKind::Binary { operator, left, right }, tokens);
        }
        Ok(left)
    }

    /// Parse a primary or prefix expression, preserving casts, unevaluated operands, and mutation as
    /// distinct syntax.
    fn prefix(&mut self, depth: usize) -> Result<NodeId, SyntaxError> {
        let Some(lexeme) = self.tokens.get(self.position) else {
            return Err(self.error(SyntaxErrorKind::InvalidExpression, "expected an operand"));
        };
        let start = lexeme.index;
        let spelling = lexeme.token.spelling.as_str();
        // Preprocessing parameters can have spellings that are C keywords. Their
        // replacement-list occurrences are holes before C syntax is interpreted.
        if let Some(&index) = self.parameters.get(spelling) {
            self.position += 1;
            return Ok(self
                .push(ExpressionKind::Parameter { index }, TokenRange { start, end: start + 1 }));
        }
        if let Some(operator) = match spelling {
            "+" => Some(UnaryOperator::Plus),
            "-" => Some(UnaryOperator::Negate),
            "~" => Some(UnaryOperator::BitwiseNot),
            "!" => Some(UnaryOperator::LogicalNot),
            _ => None,
        } {
            self.position += 1;
            let operand = self.expression(14, depth + 1)?;
            let tokens = TokenRange { start, end: self.nodes[operand].tokens.end };
            return Ok(self.push(ExpressionKind::Unary { operator, operand }, tokens));
        }
        match spelling {
            "__builtin_offsetof" => return self.offset_of(start),
            "(" => {
                if let Some((end, type_name)) = self.cast_type() {
                    self.position = end + 1;
                    let operand = self.expression(14, depth + 1)?;
                    let tokens = TokenRange { start, end: self.nodes[operand].tokens.end };
                    return Ok(self.push(ExpressionKind::Cast { type_name, operand }, tokens));
                }
                if let Some((end, parameter, pointers, is_const)) = self.type_parameter() {
                    let ambiguous = pointers == 0
                        && !is_const
                        && !self.type_parameters.contains(&parameter)
                        && self.tokens[end + 1].token.spelling == "(";
                    self.position = end + 1;
                    let operand = self.expression(14, depth + 1)?;
                    let tokens = TokenRange { start, end: self.nodes[operand].tokens.end };
                    let node = self.push(
                        ExpressionKind::TypeParameterCast {
                            parameter,
                            pointers,
                            is_const,
                            operand,
                        },
                        tokens,
                    );
                    if ambiguous {
                        self.ambiguous_casts.insert(node);
                    }
                    return Ok(node);
                }
                self.position += 1;
                let operand = self.expression(0, depth + 1)?;
                self.expect(")")?;
                let end = self.tokens[self.position - 1].index + 1;
                return Ok(self.push(ExpressionKind::Group { operand }, TokenRange { start, end }));
            }
            "*" | "&" => {
                let address = spelling == "&";
                self.position += 1;
                let operand = self.expression(14, depth + 1)?;
                let tokens = TokenRange { start, end: self.nodes[operand].tokens.end };
                let kind = if address {
                    ExpressionKind::AddressOf { operand }
                } else {
                    ExpressionKind::Dereference { operand }
                };
                return Ok(self.push(kind, tokens));
            }
            "++" | "--" => {
                let increment = spelling == "++";
                self.position += 1;
                let operand = self.expression(14, depth + 1)?;
                let tokens = TokenRange { start, end: self.nodes[operand].tokens.end };
                return Ok(self
                    .push(ExpressionKind::Update { operand, increment, postfix: false }, tokens));
            }
            "sizeof" | "_Alignof" | "__alignof__" | "__alignof" => {
                let alignment = spelling != "sizeof";
                self.position += 1;
                if self.spelling() == Some("(")
                    && let Some((end, type_name)) = self.cast_type()
                {
                    self.position = end + 1;
                    let tokens = TokenRange { start, end: self.tokens[end].index + 1 };
                    let kind = if alignment {
                        ExpressionKind::AlignOfType { type_name }
                    } else {
                        ExpressionKind::SizeOfType { type_name }
                    };
                    return Ok(self.push(kind, tokens));
                }
                if self.spelling() == Some("(")
                    && let Some((end, parameter, pointers, is_const)) =
                        self.parenthesized_type_parameter()
                    && (alignment
                        || pointers != 0
                        || is_const
                        || self.type_parameters.contains(&parameter))
                {
                    self.position = end + 1;
                    let tokens = TokenRange { start, end: self.tokens[end].index + 1 };
                    let kind = if alignment {
                        ExpressionKind::AlignOfTypeParameter { parameter, pointers, is_const }
                    } else {
                        ExpressionKind::SizeOfTypeParameter { parameter, pointers, is_const }
                    };
                    return Ok(self.push(kind, tokens));
                }
                if alignment {
                    return Err(self.error(
                        SyntaxErrorKind::UnevaluatedExpression,
                        "alignment expression requires an established type operand",
                    ));
                }
                let operand = self.expression(14, depth + 1)?;
                let tokens = TokenRange { start, end: self.nodes[operand].tokens.end };
                return Ok(self.push(ExpressionKind::SizeOfExpression { operand }, tokens));
            }
            _ => {}
        }
        if lexeme.token.kind == TokenKind::Literal {
            let literal = parse_literal(spelling)
                .map_err(|message| self.error(SyntaxErrorKind::UnsupportedLiteral, message))?;
            self.position += 1;
            return Ok(self.push(
                ExpressionKind::IntegerLiteral { literal },
                TokenRange { start, end: start + 1 },
            ));
        }
        if matches!(lexeme.token.kind, TokenKind::Identifier | TokenKind::Keyword) {
            // These constructs are declarations or compile-time selectors, never
            // runtime calls. Some preprocessor token streams label their names as
            // identifiers, so spelling is checked as well as the keyword kind.
            if lexeme.token.kind == TokenKind::Keyword
                || matches!(
                    spelling,
                    "_Static_assert" | "static_assert" | "_Generic" | "_Alignas" | "alignas"
                )
            {
                return Err(self.error(
                    SyntaxErrorKind::Statement,
                    "compiler and declaration keywords require a dedicated syntax contract",
                ));
            }
            if matches!(
                spelling,
                "do" | "if"
                    | "for"
                    | "while"
                    | "return"
                    | "goto"
                    | "switch"
                    | "break"
                    | "continue"
                    | "else"
                    | "case"
                    | "default"
            ) {
                return Err(self.error(
                    SyntaxErrorKind::Statement,
                    "unsupported statement or caller control-flow shape",
                ));
            }
            let kind = ExpressionKind::Identifier { name: spelling.into() };
            self.position += 1;
            return Ok(self.push(kind, TokenRange { start, end: start + 1 }));
        }
        Err(self.unexpected())
    }

    /// Parse offsetof record and field designators as structural operands rather than evaluated
    /// expressions.
    fn offset_of(&mut self, start: usize) -> Result<NodeId, SyntaxError> {
        self.position += 1;
        self.expect("(")?;
        let type_start = self.position;
        while self.spelling().is_some_and(|token| token != ",") {
            if matches!(self.spelling(), Some("(" | ")" | "[" | "]" | "{" | "}" | ";")) {
                return Err(self.error(
                    SyntaxErrorKind::TypeParameter,
                    "offsetof requires a flat named C type or one type parameter",
                ));
            }
            self.position += 1;
        }
        let type_tokens = &self.tokens[type_start..self.position];
        let record = if type_tokens.len() == 1
            && let Some(&index) = self.parameters.get(type_tokens[0].token.spelling.as_str())
        {
            OffsetRecord::Parameter { index }
        } else {
            let name = type_tokens
                .iter()
                .map(|token| token.token.spelling.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            if type_tokens.is_empty()
                || type_tokens
                    .iter()
                    .any(|token| self.parameters.contains_key(token.token.spelling.as_str()))
                || !(self.is_type)(&name)
            {
                return Err(self.error(
                    SyntaxErrorKind::TypeParameter,
                    "offsetof requires an established C record type",
                ));
            }
            OffsetRecord::Named { name }
        };
        self.expect(",")?;
        let mut fields = Vec::new();
        loop {
            let Some(token) = self.tokens.get(self.position) else {
                return Err(self.error(
                    SyntaxErrorKind::InvalidExpression,
                    "offsetof is missing its member path",
                ));
            };
            if !matches!(token.token.kind, TokenKind::Identifier | TokenKind::Keyword) {
                return Err(self.error(SyntaxErrorKind::PointerOperation, "offsetof requires a nonempty path of member identifiers; array indices and indirect access are not modeled"));
            }
            fields.push(if let Some(&index) = self.parameters.get(token.token.spelling.as_str()) {
                OffsetComponent::Parameter { index }
            } else {
                OffsetComponent::Named { name: token.token.spelling.clone() }
            });
            self.position += 1;
            if fields.len() > MAX_DEPTH {
                return Err(self.error(
                    SyntaxErrorKind::BudgetExceeded,
                    "offsetof member path exceeds the 64-component bound",
                ));
            }
            if self.spelling() == Some(".") {
                self.position += 1;
            } else {
                break;
            }
        }
        if self.spelling() != Some(")") {
            return Err(self.error(SyntaxErrorKind::PointerOperation, "offsetof array indices, indirect access, and evaluated member operands are not modeled"));
        }
        self.position += 1;
        let end = self.tokens[self.position - 1].index + 1;
        Ok(self.push(ExpressionKind::OffsetOf { record, fields }, TokenRange { start, end }))
    }

    // Only a flat, catalog-recognized concrete type is a cast. Parenthesized expressions
    // stay expressions; a macro parameter is never guessed to be a typedef.
    /// Recognize concrete cast syntax only with independent catalog evidence for the named type.
    fn cast_type(&self) -> Option<(usize, String)> {
        let mut end = self.position + 1;
        while let Some(lexeme) = self.tokens.get(end) {
            match lexeme.token.spelling.as_str() {
                ")" => break,
                "(" | "{" | "}" | ";" | "," => return None,
                _ => end += 1,
            }
        }
        if self.tokens.get(end)?.token.spelling != ")" || end == self.position + 1 {
            return None;
        }
        if self.tokens[self.position + 1..end]
            .iter()
            .any(|token| self.parameters.contains_key(token.token.spelling.as_str()))
        {
            return None;
        }
        let type_name = self.tokens[self.position + 1..end]
            .iter()
            .map(|token| token.token.spelling.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        (self.is_type)(&type_name).then_some((end, type_name))
    }

    /// Decode a supported formal type operand with its pointer and qualifier layers.
    fn type_parameter(&self) -> Option<(usize, usize, u8, bool)> {
        let (end, parameter, pointers, is_const) = self.parenthesized_type_parameter()?;
        let next = self.tokens.get(end + 1)?;
        (matches!(next.token.kind, TokenKind::Identifier | TokenKind::Keyword | TokenKind::Literal)
            || matches!(next.token.spelling.as_str(), "(" | "~" | "!")
            || ((pointers != 0 || is_const || self.type_parameters.contains(&parameter))
                && matches!(next.token.spelling.as_str(), "+" | "-" | "*" | "&" | "++" | "--")))
        .then_some((end, parameter, pointers, is_const))
    }

    /// Recognize an independently established formal type inside parentheses without proving an
    /// ambiguous invocation by itself.
    fn parenthesized_type_parameter(&self) -> Option<(usize, usize, u8, bool)> {
        let mut end = self.position + 1;
        let mut has_parameter = false;
        let mut parameter = 0;
        let mut pointers = 0_u8;
        let mut is_const = false;
        while let Some(token) = self.tokens.get(end) {
            match token.token.spelling.as_str() {
                ")" => break,
                "const" if pointers == 0 && !self.parameters.contains_key("const") => {
                    is_const = true
                }
                "*" if has_parameter => pointers = pointers.checked_add(1)?,
                name if self.parameters.contains_key(name) && !has_parameter && pointers == 0 => {
                    has_parameter = true;
                    parameter = self.parameters[name];
                }
                _ => return None,
            }
            end += 1;
        }
        let close = self.tokens.get(end)?;
        (has_parameter && close.token.spelling == ")")
            .then_some((end, parameter, pointers, is_const))
    }

    /// Append a node after its children so later analysis can traverse the arena in dependency order.
    fn push(&mut self, kind: ExpressionKind, tokens: TokenRange) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(ExpressionNode { kind, tokens });
        id
    }

    /// Borrow the next significant token spelling without changing original token offsets.
    fn spelling(&self) -> Option<&str> {
        self.tokens.get(self.position).map(|token| token.token.spelling.as_str())
    }

    /// Consume an exact punctuation token or report a source-ranged parser error.
    fn expect(&mut self, expected: &str) -> Result<(), SyntaxError> {
        if self.spelling() != Some(expected) {
            // Unsupported postfix/place syntax can occur inside parentheses. Keep its
            // category instead of hiding it behind a missing-delimiter error.
            let unexpected = self.unexpected();
            if unexpected.kind != SyntaxErrorKind::InvalidExpression {
                return Err(unexpected);
            }
            return Err(
                self.error(SyntaxErrorKind::InvalidExpression, format!("expected {expected:?}"))
            );
        }
        self.position += 1;
        Ok(())
    }

    /// Describe the unexpected current token or end of input for syntax diagnostics.
    fn unexpected(&self) -> SyntaxError {
        let spelling = self.spelling().unwrap_or("end of replacement list");
        let (kind, message) = match spelling {
            "[" | "." | "->" => {
                (SyntaxErrorKind::PointerOperation, "unsupported array or member access shape")
            }
            "++" | "--" | "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "<<=" | ">>=" | "&=" | "^="
            | "|=" => {
                (SyntaxErrorKind::Mutation, "unsupported mutation or place-valued expression shape")
            }
            "{" | "}" | ";" => (SyntaxErrorKind::Statement, "unexpected statement token"),
            "(" => (SyntaxErrorKind::Call, "unsupported call shape"),
            "," => (SyntaxErrorKind::CommaExpression, "unexpected comma expression"),
            _ => (SyntaxErrorKind::InvalidExpression, "not a complete supported C expression"),
        };
        self.error(kind, format!("{message}: {spelling:?}"))
    }

    /// Attach an error category and original replacement-token range to a parser refusal.
    fn error(&self, kind: SyntaxErrorKind, message: impl Into<String>) -> SyntaxError {
        let start = self.tokens.get(self.position).map_or(self.original_len, |token| token.index);
        SyntaxError {
            kind,
            tokens: TokenRange { start, end: (start + 1).min(self.original_len) },
            message: message.into(),
        }
    }
}

/// Map accepted operator spelling to precedence and semantic operator identity.
fn binary_operator(spelling: &str) -> Option<(BinaryOperator, u8)> {
    Some(match spelling {
        "||" => (BinaryOperator::LogicalOr, 2),
        "&&" => (BinaryOperator::LogicalAnd, 3),
        "|" => (BinaryOperator::BitOr, 4),
        "^" => (BinaryOperator::BitXor, 5),
        "&" => (BinaryOperator::BitAnd, 6),
        "==" => (BinaryOperator::Equal, 7),
        "!=" => (BinaryOperator::NotEqual, 7),
        "<" => (BinaryOperator::Less, 8),
        "<=" => (BinaryOperator::LessEqual, 8),
        ">" => (BinaryOperator::Greater, 8),
        ">=" => (BinaryOperator::GreaterEqual, 8),
        "<<" => (BinaryOperator::ShiftLeft, 9),
        ">>" => (BinaryOperator::ShiftRight, 9),
        "+" => (BinaryOperator::Add, 10),
        "-" => (BinaryOperator::Subtract, 10),
        "*" => (BinaryOperator::Multiply, 11),
        "/" => (BinaryOperator::Divide, 11),
        "%" => (BinaryOperator::Remainder, 11),
        _ => return None,
    })
}

/// Decode compound assignment as its underlying binary operation, retaining simple assignment
/// separately.
fn assignment_operator(spelling: &str) -> Option<Option<BinaryOperator>> {
    Some(match spelling {
        "=" => None,
        "+=" => Some(BinaryOperator::Add),
        "-=" => Some(BinaryOperator::Subtract),
        "*=" => Some(BinaryOperator::Multiply),
        "/=" => Some(BinaryOperator::Divide),
        "%=" => Some(BinaryOperator::Remainder),
        "<<=" => Some(BinaryOperator::ShiftLeft),
        ">>=" => Some(BinaryOperator::ShiftRight),
        "&=" => Some(BinaryOperator::BitAnd),
        "^=" => Some(BinaryOperator::BitXor),
        "|=" => Some(BinaryOperator::BitOr),
        _ => return None,
    })
}

/// Parse magnitude, radix, and suffix without choosing a target-dependent C type.
fn parse_integer_literal(spelling: &str) -> Result<IntegerLiteral, String> {
    let bytes = spelling.as_bytes();
    let (radix, start) = if spelling.starts_with("0x") || spelling.starts_with("0X") {
        (16, 2)
    } else if spelling.starts_with("0b") || spelling.starts_with("0B") {
        // Clang accepts this C23/GNU spelling; selecting its type follows the nondecimal rules.
        (2, 2)
    } else if spelling.starts_with('0') {
        (8, 0)
    } else {
        (10, 0)
    };
    let mut end = start;
    let mut value = 0_u128;
    while let Some(&byte) = bytes.get(end) {
        let digit = match byte {
            b'0'..=b'9' => u32::from(byte - b'0'),
            b'a'..=b'f' => u32::from(byte - b'a') + 10,
            b'A'..=b'F' => u32::from(byte - b'A') + 10,
            _ => break,
        };
        if digit >= radix {
            break;
        }
        value = value
            .checked_mul(u128::from(radix))
            .and_then(|value| value.checked_add(u128::from(digit)))
            .ok_or_else(|| "integer literal exceeds the supported 128-bit magnitude".to_string())?;
        end += 1;
    }
    if end == start {
        return Err("only integer and verified basic character literals are supported; string and floating literals are deferred".into());
    }
    let suffix = &spelling[end..];
    if suffix.contains("lL") || suffix.contains("Ll") {
        return Err("mixed-case long-long suffix is not a supported C integer suffix".into());
    }
    let suffix = match suffix.to_ascii_lowercase().as_str() {
        "" => IntegerSuffix { unsigned: false, long: 0 },
        "u" => IntegerSuffix { unsigned: true, long: 0 },
        "l" => IntegerSuffix { unsigned: false, long: 1 },
        "ul" | "lu" => IntegerSuffix { unsigned: true, long: 1 },
        "ll" => IntegerSuffix { unsigned: false, long: 2 },
        "ull" | "llu" => IntegerSuffix { unsigned: true, long: 2 },
        _ => return Err(format!("unsupported integer literal or suffix: {spelling:?}")),
    };
    Ok(IntegerLiteral { spelling: spelling.into(), value, radix: radix as u8, suffix })
}

/// Select the supported integer or basic character literal grammar while retaining the original
/// spelling.
fn parse_literal(spelling: &str) -> Result<IntegerLiteral, String> {
    if spelling.starts_with('\'') {
        return parse_character_literal(spelling);
    }
    if ["L'", "u'", "U'", "u8'"].iter().any(|prefix| spelling.starts_with(prefix)) {
        return Err(
            "wide and prefixed character constants require an encoding/type contract".into()
        );
    }
    parse_integer_literal(spelling)
}

// C ordinary single-character constants have int type, including numeric escapes.
// Only the basic execution set and positive ASCII escape range are decoded here;
// the independently verified compilation profile establishes their actual C values.
/// Decode only the verified basic character subset whose C int value can be established for supported
/// targets.
fn parse_character_literal(spelling: &str) -> Result<IntegerLiteral, String> {
    let body = spelling
        .strip_prefix('\'')
        .and_then(|body| body.strip_suffix('\''))
        .ok_or_else(|| "unterminated ordinary character constant".to_string())?;
    if !body.is_ascii() {
        return Err("non-ASCII ordinary character constants require an encoding contract".into());
    }
    let bytes = body.as_bytes();
    let value = if bytes.first() == Some(&b'\\') {
        match bytes.get(1) {
            Some(b'0'..=b'7') => {
                if bytes.len() > 4 || bytes[1..].iter().any(|byte| !matches!(byte, b'0'..=b'7')) {
                    return Err(
                        "multi-character constants are implementation-defined and deferred".into(),
                    );
                }
                bytes[1..].iter().fold(0_u32, |value, byte| value * 8 + u32::from(byte - b'0'))
            }
            Some(b'x') => {
                if bytes.len() == 2 {
                    return Err("hexadecimal character escape requires at least one digit".into());
                }
                let mut value = 0_u32;
                for &byte in &bytes[2..] {
                    let digit =
                        match byte {
                            b'0'..=b'9' => u32::from(byte - b'0'),
                            b'a'..=b'f' => u32::from(byte - b'a') + 10,
                            b'A'..=b'F' => u32::from(byte - b'A') + 10,
                            _ => return Err(
                                "multi-character constants are implementation-defined and deferred"
                                    .into(),
                            ),
                        };
                    value = value * 16 + digit;
                    if value > 127 {
                        return Err(
                            "character escape exceeds the supported ASCII range 0..=127".into()
                        );
                    }
                }
                value
            }
            Some(b'u' | b'U') => {
                return Err("universal character escapes require an encoding contract".into());
            }
            Some(escaped) if bytes.len() == 2 => match escaped {
                b'\'' => 39,
                b'"' => 34,
                b'?' => 63,
                b'\\' => 92,
                b'a' => 7,
                b'b' => 8,
                b'f' => 12,
                b'n' => 10,
                b'r' => 13,
                b't' => 9,
                b'v' => 11,
                _ => {
                    return Err(
                        "unknown character escape is outside the supported basic execution set"
                            .into(),
                    );
                }
            },
            _ => {
                return Err("multi-character or incomplete character constants are deferred".into());
            }
        }
    } else {
        let [byte] = bytes else {
            return Err("multi-character or empty character constants are deferred".into());
        };
        if !(byte.is_ascii_alphanumeric() || b" !\"#%&()*+,-./:;<=>?[\\]^_{|}~".contains(byte)) {
            return Err("character is outside the portable basic execution character set".into());
        }
        u32::from(*byte)
    };
    if value > 127 {
        return Err("character escape exceeds the supported ASCII range 0..=127".into());
    }
    Ok(IntegerLiteral {
        spelling: spelling.into(),
        value: u128::from(value),
        radix: 10,
        suffix: IntegerSuffix { unsigned: false, long: 0 },
    })
}

/// Exercise this phase’s semantic boundaries with owned fixtures.
/// These regressions check accepted proofs and explicit refusals without changing production
/// headers or weakening the C identity and evaluation contracts.
#[cfg(test)]
mod tests {
    use super::*;

    /// Construct lexer-category fixtures for parser regressions without requiring a live Clang
    /// runtime.
    fn tokens(spellings: &[&str]) -> Vec<Token> {
        spellings
            .iter()
            .map(|&spelling| Token {
                kind: if spelling.as_bytes().first().is_some_and(u8::is_ascii_digit) {
                    TokenKind::Literal
                } else if spelling
                    .as_bytes()
                    .first()
                    .is_some_and(|byte| byte.is_ascii_alphabetic() || *byte == b'_')
                {
                    TokenKind::Identifier
                } else {
                    TokenKind::Punctuation
                },
                spelling: spelling.into(),
            })
            .collect()
    }

    /// Checks precedence and left associativity.
    #[test]
    fn precedence_and_left_associativity() {
        let expression = parse_expression(
            &tokens(&["x", "-", "2", "-", "3", "*", "4", "<<", "1", "==", "0"]),
            &["x".into()],
            |_| false,
        )
        .unwrap();
        let ExpressionKind::Binary { operator: BinaryOperator::Equal, left, .. } =
            expression.nodes[expression.root].kind
        else {
            panic!("comparison must be the root")
        };
        let ExpressionKind::Binary { operator: BinaryOperator::ShiftLeft, left, .. } =
            expression.nodes[left].kind
        else {
            panic!("shift must precede comparison")
        };
        let ExpressionKind::Binary { operator: BinaryOperator::Subtract, left, right } =
            expression.nodes[left].kind
        else {
            panic!("subtraction must precede shift")
        };
        assert!(matches!(
            expression.nodes[left].kind,
            ExpressionKind::Binary { operator: BinaryOperator::Subtract, .. }
        ));
        assert!(matches!(
            expression.nodes[right].kind,
            ExpressionKind::Binary { operator: BinaryOperator::Multiply, .. }
        ));
    }

    /// Checks offset operands are structural holes and establish earlier type uses.
    #[test]
    fn offset_operands_are_structural_holes_and_establish_earlier_type_uses() {
        let parsed = parse_expression(
            &tokens(&[
                "sizeof",
                "(",
                "T",
                ")",
                "+",
                "__builtin_offsetof",
                "(",
                "T",
                ",",
                "outer",
                ".",
                "member",
                ")",
            ]),
            &["T".into(), "member".into()],
            |_| false,
        )
        .unwrap();
        assert!(matches!(
            parsed.nodes[0].kind,
            ExpressionKind::SizeOfTypeParameter { parameter: 0, .. }
        ));
        let offset = parsed
            .nodes
            .iter()
            .find_map(|node| match &node.kind {
                ExpressionKind::OffsetOf { record, fields } => Some((record, fields)),
                _ => None,
            })
            .unwrap();
        assert_eq!(offset.0, &OffsetRecord::Parameter { index: 0 });
        assert_eq!(
            offset.1,
            &[
                OffsetComponent::Named { name: "outer".into() },
                OffsetComponent::Parameter { index: 1 }
            ]
        );
        assert!(
            !parsed.nodes.iter().any(|node| matches!(node.kind, ExpressionKind::Parameter { .. }))
        );
    }

    /// Checks offset paths reject evaluated and malformed operands.
    #[test]
    fn offset_paths_reject_evaluated_and_malformed_operands() {
        for fields in [
            vec!["array", "[", "0", "]"],
            vec!["array", "[", "i", "++", "]"],
            vec!["next", "->", "field"],
            vec!["field", "(", ")"],
            vec!["field", "."],
            vec![],
        ] {
            let mut source = vec!["__builtin_offsetof", "(", "Record", ","];
            source.extend(fields);
            source.push(")");
            assert!(
                parse_expression(&tokens(&source), &[], |ty| ty == "Record").is_err(),
                "{source:?}"
            );
        }
        let mut source = vec!["__builtin_offsetof", "(", "Record", ",", "field"];
        for _ in 0..64 {
            source.extend([".", "field"]);
        }
        source.push(")");
        let error = parse_expression(&tokens(&source), &[], |ty| ty == "Record").unwrap_err();
        assert_eq!(error.kind, SyntaxErrorKind::BudgetExceeded);
    }

    /// Checks cast grouping and negative literal are preserved.
    #[test]
    fn cast_grouping_and_negative_literal_are_preserved() {
        let expression = parse_expression(
            &tokens(&["(", "unsigned", "long", ")", "(", "x", ")", "+", "-", "1", "ULL"]),
            &["x".into()],
            |name| name == "unsigned long",
        );
        assert!(
            expression.is_err(),
            "a suffix is part of one token, never an identifier following a literal"
        );
        let expression = parse_expression(
            &tokens(&["(", "unsigned", "long", ")", "(", "x", ")", "+", "-", "1ULL"]),
            &["x".into()],
            |name| name == "unsigned long",
        )
        .unwrap();
        assert!(matches!(expression.nodes[0].kind, ExpressionKind::Parameter { index: 0 }));
        assert!(matches!(expression.nodes[1].kind, ExpressionKind::Group { .. }));
        assert!(matches!(expression.nodes[2].kind, ExpressionKind::Cast { .. }));
        assert!(matches!(
            expression.nodes[4].kind,
            ExpressionKind::Unary { operator: UnaryOperator::Negate, .. }
        ));
    }

    /// Checks literal radix suffix and overflow.
    #[test]
    fn literal_radix_suffix_and_overflow() {
        assert_eq!(parse_integer_literal("077UL").unwrap().value, 63);
        assert_eq!(parse_integer_literal("0xdeadBEEFULL").unwrap().value, 0xdeadbeef);
        assert_eq!(parse_integer_literal("0b101u").unwrap().value, 5);
        for literal in [
            "09",
            "0x",
            "1.0",
            "1e2",
            "'x'",
            "\"x\"",
            "1lL",
            "1uu",
            "340282366920938463463374607431768211456",
        ] {
            assert!(parse_integer_literal(literal).is_err(), "{literal}");
        }
    }

    /// Checks basic character constants keep c int literal metadata.
    #[test]
    fn basic_character_constants_keep_c_int_literal_metadata() {
        for (spelling, value) in [
            ("'0'", 48),
            ("'A'", 65),
            ("'z'", 122),
            ("' '", 32),
            ("'~'", 126),
            (r"'\''", 39),
            (r#"'\"'"#, 34),
            (r"'\?'", 63),
            (r"'\\'", 92),
            (r"'\a'", 7),
            (r"'\b'", 8),
            (r"'\f'", 12),
            (r"'\n'", 10),
            (r"'\r'", 13),
            (r"'\t'", 9),
            (r"'\v'", 11),
            (r"'\0'", 0),
            (r"'\1'", 1),
            (r"'\12'", 10),
            (r"'\177'", 127),
            (r"'\x00'", 0),
            (r"'\x7f'", 127),
            (r"'\x0000000000000000000000007F'", 127),
        ] {
            let literal = parse_literal(spelling).unwrap();
            assert_eq!(literal.spelling, spelling);
            assert_eq!(literal.value, value, "{spelling}");
            assert_eq!(literal.radix, 10);
            assert_eq!(literal.suffix, IntegerSuffix { unsigned: false, long: 0 });
        }
        for value in 0..=127 {
            assert_eq!(parse_literal(&format!("'\\{value:03o}'")).unwrap().value, value);
            assert_eq!(parse_literal(&format!("'\\x{value:02x}'")).unwrap().value, value);
        }
    }

    /// Checks character constants outside the verified basic subset are rejected.
    #[test]
    fn character_constants_outside_the_verified_basic_subset_are_rejected() {
        for spelling in [
            "''",
            "'''",
            "'ab'",
            "'é'",
            "'$'",
            "'@'",
            "'`'",
            "'\n'",
            "L'a'",
            "u'a'",
            "U'a'",
            "u8'a'",
            r"'\u0041'",
            r"'\U00000041'",
            r"'\e'",
            r"'\q'",
            r"'\8'",
            r"'\x'",
            r"'\200'",
            r"'\777'",
            r"'\0000'",
            r"'\x80'",
            r"'\x41g'",
            r"'\xFFFFFFFFFFFFFFFF'",
            "'unterminated",
        ] {
            assert!(parse_literal(spelling).is_err(), "{spelling}");
        }
    }

    /// Checks complete expressions and budgets.
    #[test]
    fn complete_expressions_and_budgets() {
        let call =
            parse_expression(&tokens(&["x", "(", "1", ")"]), &["x".into()], |_| false).unwrap();
        assert!(
            matches!(call.nodes[call.root].kind, ExpressionKind::Call { callee: 0, ref arguments } if arguments == &[1])
        );
        let cast =
            parse_expression(&tokens(&["(", "t", ")", "x"]), &["t".into(), "x".into()], |_| false)
                .unwrap();
        assert!(matches!(
            cast.nodes[cast.root].kind,
            ExpressionKind::TypeParameterCast { parameter: 0, operand: 0, pointers: 0, .. }
        ));
        for malformed in [vec!["x", "(", "1", ",", ")"], vec!["1", "2"], vec!["x", "[", "0"]] {
            assert_eq!(
                parse_expression(&tokens(&malformed), &["x".into()], |_| false).unwrap_err().kind,
                SyntaxErrorKind::InvalidExpression
            );
        }
        let mut deep = vec!["("; 70];
        deep.push("1");
        deep.extend(vec![")"; 70]);
        assert_eq!(
            parse_expression(&tokens(&deep), &[], |_| false).unwrap_err().kind,
            SyntaxErrorKind::BudgetExceeded
        );
        let mut long = vec!["1"];
        for _ in 0..1000 {
            long.extend(["+", "1"]);
        }
        let expression = parse_expression(&tokens(&long), &[], |_| false).unwrap();
        assert_eq!(expression.nodes.len(), 2001);
    }

    /// Checks postfix places calls and comma keep their structure inside grouping.
    #[test]
    fn postfix_places_calls_and_comma_keep_their_structure_inside_grouping() {
        for (body, expected) in [
            (vec!["(", "(", "x", ")", ".", "field", ")"], "member"),
            (vec!["(", "x", "->", "field", ")"], "member"),
            (vec!["(", "x", "[", "0", "]", ")"], "index"),
            (vec!["(", "x", "++", ")"], "update"),
            (vec!["(", "x", ",", "1", ")"], "comma"),
            (vec!["(", "x", "(", "1", ")", ")"], "call"),
        ] {
            let parsed = parse_expression(&tokens(&body), &["x".into()], |_| false).unwrap();
            let ExpressionKind::Group { operand } = parsed.nodes[parsed.root].kind else {
                panic!("outer group must remain")
            };
            let actual = match parsed.nodes[operand].kind {
                ExpressionKind::Member { .. } => "member",
                ExpressionKind::Index { .. } => "index",
                ExpressionKind::Update { .. } => "update",
                ExpressionKind::Comma { .. } => "comma",
                ExpressionKind::Call { .. } => "call",
                _ => panic!("wrong postfix expression: {parsed:?}"),
            };
            assert_eq!(actual, expected);
        }
    }

    /// Checks sizEOF keeps ambiguous value holes until another use establishes a type.
    #[test]
    fn sizeof_keeps_ambiguous_value_holes_until_another_use_establishes_a_type() {
        let value = parse_expression(&tokens(&["sizeof", "(", "t", ")"]), &["t".into()], |_| false)
            .unwrap();
        assert!(matches!(value.nodes[value.root].kind, ExpressionKind::SizeOfExpression { .. }));
        assert!(
            value
                .nodes
                .iter()
                .any(|node| matches!(node.kind, ExpressionKind::Parameter { index: 0 }))
        );
        let typed = parse_expression(
            &tokens(&["(", "sizeof", "(", "t", ")", "+", "_Alignof", "(", "t", ")", ")"]),
            &["t".into()],
            |_| false,
        )
        .unwrap();
        assert!(typed.nodes.iter().any(|node| matches!(
            node.kind,
            ExpressionKind::SizeOfTypeParameter { parameter: 0, pointers: 0, is_const: false }
        )));
        assert!(typed.nodes.iter().any(|node| matches!(
            node.kind,
            ExpressionKind::AlignOfTypeParameter { parameter: 0, pointers: 0, is_const: false }
        )));
        assert!(
            !typed.nodes.iter().any(|node| matches!(node.kind, ExpressionKind::Parameter { .. })),
            "reparsing must not leave orphaned value holes"
        );
        let cast = parse_expression(
            &tokens(&["(", "sizeof", "(", "t", ")", ",", "(", "t", ")", "x", ")"]),
            &["t".into(), "x".into()],
            |_| false,
        )
        .unwrap();
        assert!(cast.nodes.iter().any(|node| matches!(
            node.kind,
            ExpressionKind::SizeOfTypeParameter { parameter: 0, .. }
        )));
        assert_eq!(
            cast.nodes
                .iter()
                .filter(|node| matches!(node.kind, ExpressionKind::Parameter { .. }))
                .count(),
            1
        );
    }

    /// Checks parenthesized formal application does not establish its own type role.
    #[test]
    fn parenthesized_formal_application_does_not_establish_its_own_type_role() {
        for body in [
            vec!["(", "t", ")", "(", "x", ")"],
            vec!["(", "t", ")", "(", "x", ",", "y", ")"],
            vec!["sizeof", "(", "t", ")", ",", "(", "t", ")", "(", "x", ")"],
            vec!["(", "t", ")", "(", "x", ")", "+", "(", "t", ")", "(", "y", ")"],
            vec!["_Alignof", "(", "u", ")", ",", "(", "t", ")", "(", "x", ")"],
            vec!["_Alignof", "(", "t", ")", ",", "(", "t", ")", "(", "x", ")"],
            vec!["(", "t", ")", "(", "x", ")", ",", "_Alignof", "(", "t", ")"],
            vec![
                "sizeof", "(", "t", ")", ",", "_Alignof", "(", "t", ")", ",", "(", "t", ")", "(",
                "x", ")",
            ],
            vec![
                "_Alignof", "(", "t", ")", ",", "sizeof", "(", "t", ")", ",", "(", "t", ")", "(",
                "x", ")",
            ],
        ] {
            let error = parse_expression(
                &tokens(&body),
                &["t".into(), "x".into(), "y".into(), "u".into()],
                |_| false,
            )
            .unwrap_err();
            assert_eq!(error.kind, SyntaxErrorKind::TypeParameter, "{body:?}");
            assert!(error.message.contains("cast type or callable value"), "{error:?}");
            assert_eq!(&body[error.tokens.start..error.tokens.start + 3], &["(", "t", ")"]);
        }
    }

    /// Checks independent type uses resolve formal applications in either source order.
    #[test]
    fn independent_type_uses_resolve_formal_applications_in_either_source_order() {
        for proof in [
            vec!["_Alignof", "(", "t", "*", ")"],
            vec!["_Alignof", "(", "const", "t", ")"],
            vec!["sizeof", "(", "t", "*", ")"],
            vec!["(", "t", "*", ")", "x"],
            vec!["(", "const", "t", ")", "x"],
            vec!["(", "t", ")", "x"],
            vec!["(", "t", ")", "1"],
            vec!["__builtin_offsetof", "(", "t", ",", "field", ")"],
        ] {
            let application = ["(", "t", ")", "(", "x", ")"];
            for first in [true, false] {
                let body = if first {
                    [proof.as_slice(), &[","], application.as_slice()].concat()
                } else {
                    [application.as_slice(), &[","], proof.as_slice()].concat()
                };
                let expression =
                    parse_expression(&tokens(&body), &["t".into(), "x".into()], |_| false).unwrap();
                assert!(
                    expression.nodes.iter().any(|node| matches!(
                        node.kind,
                        ExpressionKind::TypeParameterCast { parameter: 0, pointers: 0, .. }
                    )),
                    "{body:?}"
                );
                assert!(
                    !expression
                        .nodes
                        .iter()
                        .any(|node| matches!(node.kind, ExpressionKind::Parameter { index: 0 })),
                    "a proven type must not leave a value hole: {body:?}"
                );
            }
        }
        let expression = parse_expression(
            &tokens(&[
                "sizeof", "(", "t", ")", ",", "(", "t", ")", "(", "x", ")", ",", "sizeof", "(",
                "t", "*", ")",
            ]),
            &["t".into(), "x".into()],
            |_| false,
        )
        .unwrap();
        assert!(expression.nodes.iter().any(|node| matches!(
            node.kind,
            ExpressionKind::SizeOfTypeParameter { parameter: 0, .. }
        )));
        assert_eq!(
            expression
                .nodes
                .iter()
                .filter(|node| matches!(node.kind, ExpressionKind::Parameter { .. }))
                .count(),
            1,
            "reparsing must not leave the earlier sizeof value hole"
        );
    }

    /// Checks unambiguous callable grouping and catalog casts keep their roles.
    #[test]
    fn unambiguous_callable_grouping_and_catalog_casts_keep_their_roles() {
        for body in [vec!["fn", "(", "x", ")"], vec!["(", "(", "fn", ")", ")", "(", "x", ")"]] {
            let expression =
                parse_expression(&tokens(&body), &["fn".into(), "x".into()], |_| false).unwrap();
            assert!(matches!(expression.nodes[expression.root].kind, ExpressionKind::Call { .. }));
            assert!(
                !expression
                    .nodes
                    .iter()
                    .any(|node| matches!(node.kind, ExpressionKind::TypeParameterCast { .. }))
            );
        }
        let expression = parse_expression(
            &tokens(&["(", "KnownType", ")", "(", "x", ")"]),
            &["x".into()],
            |name| name == "KnownType",
        )
        .unwrap();
        assert!(matches!(expression.nodes[expression.root].kind, ExpressionKind::Cast { .. }));
    }

    /// Checks explicit pointer and const size operands preserve type holes and qualifiers.
    #[test]
    fn explicit_pointer_and_const_size_operands_preserve_type_holes_and_qualifiers() {
        for (body, pointers, is_const) in [
            (vec!["sizeof", "(", "t", "*", ")"], 1, false),
            (vec!["sizeof", "(", "const", "t", "*", "*", ")"], 2, true),
            (vec!["sizeof", "(", "t", "const", ")"], 0, true),
        ] {
            let parsed = parse_expression(&tokens(&body), &["t".into()], |_| false).unwrap();
            assert!(
                matches!(parsed.nodes[parsed.root].kind, ExpressionKind::SizeOfTypeParameter { parameter: 0, pointers: actual_pointers, is_const: actual_const } if actual_pointers == pointers && actual_const == is_const)
            );
        }
        let alignment = parse_expression(
            &tokens(&["_Alignof", "(", "const", "t", "*", ")"]),
            &["t".into()],
            |_| false,
        )
        .unwrap();
        assert!(matches!(
            alignment.nodes[alignment.root].kind,
            ExpressionKind::AlignOfTypeParameter { parameter: 0, pointers: 1, is_const: true }
        ));
        assert!(
            parse_expression(
                &tokens(&["(", "t", "*", "const", ")", "x"]),
                &["t".into(), "x".into()],
                |_| false
            )
            .is_err(),
            "one const bit cannot represent pointer-level qualifiers"
        );
    }

    /// Checks terminal returns are statements without becoming expression operands.
    #[test]
    fn terminal_returns_are_statements_without_becoming_expression_operands() {
        for body in [vec!["return", "x"], vec!["return", "x", ";"]] {
            let parsed = parse_replacement(&tokens(&body), &["x".into()], |_| false).unwrap();
            assert_eq!(parsed.statement_body.as_ref().unwrap().tokens.end, body.len());
            assert!(parsed.statement_body.as_ref().unwrap().always_returns);
            assert!(!parsed.statement_body.as_ref().unwrap().requires_boundary);
            assert!(matches!(
                parsed.nodes[parsed.root].kind,
                ExpressionKind::Parameter { index: 0 }
            ));
            assert_eq!(
                parse_expression(&tokens(&body), &["x".into()], |_| false).unwrap_err().kind,
                SyntaxErrorKind::Statement,
            );
        }
        for body in [
            vec!["(", "return", "x", ")"],
            vec!["f", "(", "return", "x", ")"],
            vec!["return", ";"],
            vec!["return", "x", ";", "x", "++"],
        ] {
            assert!(parse_replacement(&tokens(&body), &["x".into()], |_| false).is_err());
        }
    }

    /// Checks compiler and declaration constructs never become runtime captures.
    #[test]
    fn compiler_and_declaration_constructs_never_become_runtime_captures() {
        for name in ["_Static_assert", "static_assert", "_Generic", "_Alignas", "alignas"] {
            let invocation = [name, "(", "1", ",", "message", ")"];
            for kind in [TokenKind::Identifier, TokenKind::Keyword] {
                let mut root = tokens(&invocation);
                root[0].kind = kind;
                assert_eq!(
                    parse_replacement(&root, &[], |_| false).unwrap_err().kind,
                    SyntaxErrorKind::Statement,
                    "{name} with {kind:?}",
                );
                let mut wrapped = tokens(&["do", "{"]);
                wrapped.extend(root);
                wrapped.extend(tokens(&[";", "}", "while", "(", "0", ")"]));
                assert_eq!(
                    parse_replacement(&wrapped, &[], |_| false).unwrap_err().kind,
                    SyntaxErrorKind::Statement,
                    "wrapped {name} with {kind:?}",
                );
            }
        }
        for name in ["_Atomic", "asm", "typeof", "__extension__", "int"] {
            let input = [Token { kind: TokenKind::Keyword, spelling: name.into() }];
            assert_eq!(
                parse_expression(&input, &[], |_| false).unwrap_err().kind,
                SyntaxErrorKind::Statement,
            );
        }
    }

    /// Checks keyword formals remain expression holes inside blocks.
    #[test]
    fn keyword_formals_remain_expression_holes_inside_blocks() {
        for name in ["return", "do", "if", "else", "while", "_Static_assert", "_Generic"] {
            let mut body = tokens(&["{", name, ";", "}"]);
            body[1].kind = TokenKind::Keyword;
            let parsed = parse_replacement(&body, &[name.into()], |_| false).unwrap();
            let body = parsed.statement_body.unwrap();
            assert_eq!(body.return_tokens, None);
            assert!(matches!(
                body.statements.as_slice(),
                [Statement::Expression { expression, .. }]
                    if matches!(parsed.nodes[*expression].kind, ExpressionKind::Parameter { index: 0 })
            ));
        }
        let parsed = parse_replacement(
            &tokens(&["if", "(", "condition", ")", "else", ";"]),
            &["condition".into(), "else".into()],
            |_| false,
        )
        .unwrap();
        let Statement::If { then_branch, else_branch: None, .. } =
            &parsed.statement_body.unwrap().statements[0]
        else {
            panic!("a formal called else must be the then operand")
        };
        assert!(matches!(
            then_branch.as_ref(),
            Statement::Expression { expression, .. }
                if matches!(parsed.nodes[*expression].kind, ExpressionKind::Parameter { index: 1 })
        ));
        assert_eq!(
            parse_replacement(
                &tokens(&["do", "{", "}", "while", "(", "0", ")"]),
                &["while".into()],
                |_| false,
            )
            .unwrap_err()
            .kind,
            SyntaxErrorKind::Statement,
        );
    }

    /// Checks return body preserves statement order arena and original offsets.
    #[test]
    fn return_body_preserves_statement_order_arena_and_original_offsets() {
        let mut body = tokens(&[
            "do", "{", "int", "tmp", "=", "(", "x", ")", ";", "tmp", "+=", "1", ";", "return",
            "tmp", ";", "}", "while", "(", "0", ")",
        ]);
        body.insert(2, Token { kind: TokenKind::Comment, spelling: "/* source */".into() });
        let parsed = parse_replacement(&body, &["x".into()], |name| name == "int").unwrap();
        let statement_body = parsed.statement_body.unwrap();
        assert_eq!(statement_body.tokens, TokenRange { start: 0, end: 22 });
        assert_eq!(statement_body.return_tokens, Some(TokenRange { start: 14, end: 17 }));
        assert!(statement_body.always_returns);
        assert_eq!(
            statement_body.statements,
            vec![
                Statement::Declaration {
                    name: "tmp".into(),
                    type_name: "int".into(),
                    initializer: Some(1),
                    tokens: TokenRange { start: 3, end: 10 },
                },
                Statement::Expression { expression: 4, tokens: TokenRange { start: 10, end: 14 } },
                Statement::Return { expression: 5, tokens: TokenRange { start: 14, end: 17 } },
            ]
        );
        assert!(matches!(parsed.nodes[1].kind, ExpressionKind::Group { operand: 0 }));
        assert!(matches!(
            parsed.nodes[4].kind,
            ExpressionKind::Assignment { place: 2, value: 3, .. }
        ));
        assert_eq!(parsed.root, 5);
        assert_eq!(parsed.nodes[parsed.root].tokens, TokenRange { start: 15, end: 16 });
    }

    /// Checks return local bindings and control flow are bounded.
    #[test]
    fn return_local_bindings_and_control_flow_are_bounded() {
        for body in [
            vec!["{", "int", "tmp", ";", "int", "tmp", ";", "return", "tmp", ";", "}"],
            vec!["{", "int", "x", ";", "return", "x", ";", "}"],
            vec!["{", "tmp", "=", "1", ";", "int", "tmp", ";", "return", "tmp", ";", "}"],
            vec!["{", "while", "(", "x", ")", "x", "--", ";", "return", "x", ";", "}"],
            vec!["{", "break", ";", "return", "x", ";", "}"],
            vec!["{", "continue", ";", "return", "x", ";", "}"],
            vec!["{", "goto", "label", ";", "return", "x", ";", "}"],
            vec!["{", "int", "tmp", "[", "1", "]", ";", "return", "x", ";", "}"],
            vec!["{", "int", "tmp", ",", "other", ";", "return", "x", ";", "}"],
            vec!["{", "return", "x", ";", "x", "++", ";", "}"],
            vec!["do", "{", "return", "x", ";", "}", "while", "(", "1", ")"],
        ] {
            assert!(
                parse_replacement(&tokens(&body), &["x".into()], |name| name == "int").is_err(),
                "{body:?}"
            );
        }
        let parsed = parse_replacement(
            &tokens(&["{", "int", "*", "tmp", ";", "return", "tmp", ";", "}"]),
            &[],
            |name| name == "int *",
        )
        .unwrap();
        assert!(matches!(
            &parsed.statement_body.unwrap().statements[0],
            Statement::Declaration { type_name, initializer: None, .. } if type_name == "int *"
        ));
    }

    /// Checks type hole reparse keeps return statements in one arena.
    #[test]
    fn type_hole_reparse_keeps_return_statements_in_one_arena() {
        let parsed = parse_replacement(
            &tokens(&[
                "{", "int", "tmp", "=", "sizeof", "(", "t", ")", "+", "sizeof", "(", "t", "*", ")",
                ";", "return", "(", "t", ")", "(", "x", ")", ";", "}",
            ]),
            &["t".into(), "x".into()],
            |name| name == "int",
        )
        .unwrap();
        assert!(matches!(parsed.nodes[0].kind, ExpressionKind::SizeOfTypeParameter { .. }));
        assert_eq!(parsed.statement_body.unwrap().statements.len(), 2);
        assert_eq!(
            parsed
                .nodes
                .iter()
                .filter(|node| matches!(node.kind, ExpressionKind::Parameter { .. }))
                .count(),
            1,
        );
    }

    /// Checks nested terminal wrappers keep order and reject scope changes.
    #[test]
    fn nested_terminal_wrappers_keep_order_and_reject_scope_changes() {
        let parsed = parse_replacement(
            &tokens(&[
                "do", "{", "int", "outer", "=", "1", ";", "do", "{", "int", "inner", "=", "2", ";",
                "outer", "+=", "inner", ";", "return", "outer", ";", "}", "while", "(", "0", ")",
                ";", "}", "while", "(", "0", ")",
            ]),
            &[],
            |name| name == "int",
        )
        .unwrap();
        let body = parsed.statement_body.unwrap();
        assert_eq!(body.statements.len(), 2);
        assert!(
            matches!(&body.statements[0], Statement::Declaration { name, .. } if name == "outer")
        );
        let Statement::Block { statements, .. } = &body.statements[1] else {
            panic!("the nested wrapper must retain its own C scope")
        };
        assert_eq!(statements.len(), 3);
        assert!(matches!(&statements[0], Statement::Declaration { name, .. } if name == "inner"));
        assert!(matches!(statements[1], Statement::Expression { .. }));
        assert!(
            matches!(statements[2], Statement::Return { expression, .. } if expression == parsed.root)
        );
        assert_eq!(body.walk().count(), 5);
        assert_eq!(body.tokens, TokenRange { start: 0, end: 32 });
        assert_eq!(body.return_tokens, Some(TokenRange { start: 18, end: 21 }));
        assert!(body.always_returns);
        for body in [
            vec![
                "{", "int", "local", "=", "1", ";", "{", "int", "local", "=", "2", ";", "return",
                "local", ";", "}", "}",
            ],
            vec![
                "{", "local", "=", "1", ";", "{", "int", "local", "=", "2", ";", "return", "local",
                ";", "}", "}",
            ],
        ] {
            assert_eq!(
                parse_replacement(&tokens(&body), &[], |name| name == "int").unwrap_err().kind,
                SyntaxErrorKind::Statement,
            );
        }
    }

    /// Checks straight line and noop blocks have a void root without a return.
    #[test]
    fn straight_line_and_noop_blocks_have_a_void_root_without_a_return() {
        for body in [
            vec![";"],
            vec!["{", "}"],
            vec!["{", ";", ";", "}"],
            vec!["do", "{", "}", "while", "(", "0", ")"],
            vec!["do", "{", ";", "}", "while", "(", "0x0U", ")", ";"],
        ] {
            let parsed = parse_replacement(&tokens(&body), &[], |_| false).unwrap();
            assert!(matches!(parsed.nodes[parsed.root].kind, ExpressionKind::Empty));
            let statement_body = parsed.statement_body.unwrap();
            assert!(statement_body.statements.is_empty());
            assert_eq!(statement_body.return_tokens, None);
            assert!(!statement_body.always_returns);
            assert!(!statement_body.requires_boundary);
            assert_eq!(statement_body.tokens.end, body.len());
        }
        let parsed = parse_replacement(
            &tokens(&[
                "do", "{", "int", "tmp", "=", "x", ";", "x", "=", "tmp", ";", "}", "while", "(",
                "0", ")",
            ]),
            &["x".into()],
            |name| name == "int",
        )
        .unwrap();
        assert!(matches!(parsed.nodes[parsed.root].kind, ExpressionKind::Empty));
        let body = parsed.statement_body.unwrap();
        assert_eq!(body.statements.len(), 2);
        assert!(
            matches!(&body.statements[0], Statement::Declaration { name, .. } if name == "tmp")
        );
        assert!(matches!(body.statements[1], Statement::Expression { .. }));
        assert_eq!(body.return_tokens, None);
        assert!(!body.always_returns);
    }

    /// Checks nested nonterminal scopes preserve order and local visibility.
    #[test]
    fn nested_nonterminal_scopes_preserve_order_and_local_visibility() {
        let parsed = parse_replacement(
            &tokens(&[
                "{", "int", "outer", "=", "1", ";", "{", "int", "inner", "=", "2", ";", "outer",
                "+=", "inner", ";", "}", "outer", "++", ";", "}",
            ]),
            &[],
            |name| name == "int",
        )
        .unwrap();
        let body = parsed.statement_body.unwrap();
        assert_eq!(body.statements.len(), 3);
        assert!(matches!(body.statements[1], Statement::Block { .. }));
        assert!(matches!(body.statements[2], Statement::Expression { .. }));
        let order = body
            .walk()
            .map(|statement| match statement {
                Statement::Declaration { name, .. } => name.as_str(),
                Statement::Block { .. } => "block",
                Statement::Expression { .. } => "expression",
                Statement::Return { .. } => "return",
                Statement::If { .. } => "if",
            })
            .collect::<Vec<_>>();
        assert_eq!(order, ["outer", "block", "inner", "expression", "expression"]);
        for body in [
            vec!["{", "{", "int", "tmp", "=", "1", ";", "}", "tmp", "++", ";", "}"],
            vec!["{", "{", "int", "tmp", "=", "1", ";", "}", "return", "tmp", ";", "}"],
            vec!["{", "{", "int", "tmp", "=", "1", ";", "}", "{", "tmp", "++", ";", "}", "}"],
            vec!["{", "tmp", "++", ";", "{", "int", "tmp", "=", "1", ";", "}", "}"],
            vec![
                "{", "{", "int", "tmp", "=", "1", ";", "}", "{", "int", "tmp", "=", "2", ";", "}",
                "}",
            ],
        ] {
            assert_eq!(
                parse_replacement(&tokens(&body), &[], |name| name == "int").unwrap_err().kind,
                SyntaxErrorKind::Statement,
                "{body:?}"
            );
        }
    }

    /// Checks nested terminal returns cannot be followed by effects.
    #[test]
    fn nested_terminal_returns_cannot_be_followed_by_effects() {
        for body in [
            vec!["{", "{", "return", "x", ";", "}", "x", "++", ";", "}"],
            vec![
                "{", "do", "{", "return", "x", ";", "}", "while", "(", "0", ")", ";", "{", "}", "}",
            ],
        ] {
            assert!(
                parse_replacement(&tokens(&body), &["x".into()], |_| false).is_err(),
                "{body:?}"
            );
        }
        let nested = std::iter::repeat_n("{", MAX_DEPTH + 1)
            .chain(std::iter::repeat_n("}", MAX_DEPTH + 1))
            .collect::<Vec<_>>();
        assert_eq!(
            parse_replacement(&tokens(&nested), &[], |_| false).unwrap_err().kind,
            SyntaxErrorKind::BudgetExceeded
        );
    }

    /// Checks conditionals attach else to the nearest if and group condition tokens.
    #[test]
    fn conditionals_attach_else_to_the_nearest_if_and_group_condition_tokens() {
        let source = tokens(&[
            "if", "(", "outer", ")", "if", "(", "inner", ")", "x", "++", ";", "else", "x", "--",
            ";",
        ]);
        let parsed =
            parse_replacement(&source, &["outer".into(), "inner".into(), "x".into()], |_| false)
                .unwrap();
        let body = parsed.statement_body.unwrap();
        assert!(body.requires_boundary);
        assert!(!body.always_returns);
        assert_eq!(body.return_tokens, None);
        assert!(matches!(parsed.nodes[parsed.root].kind, ExpressionKind::Empty));
        let Statement::If { condition, then_branch, else_branch: None, tokens: range } =
            &body.statements[0]
        else {
            panic!("the outer if must remain unmatched")
        };
        assert_eq!(*range, TokenRange { start: 0, end: source.len() });
        assert_eq!(parsed.nodes[*condition].tokens, TokenRange { start: 1, end: 4 });
        let ExpressionKind::Group { operand } = parsed.nodes[*condition].kind else {
            panic!("the explicit condition delimiters protect its whole expression")
        };
        assert!(matches!(parsed.nodes[operand].kind, ExpressionKind::Parameter { index: 0 }));
        let Statement::If { condition, else_branch: Some(_), .. } = then_branch.as_ref() else {
            panic!("else must attach to the inner if")
        };
        assert_eq!(parsed.nodes[*condition].tokens, TokenRange { start: 5, end: 8 });
        assert_eq!(body.walk().count(), 4);

        let parsed = parse_replacement(
            &tokens(&["if", "(", "x", "&&", "y", ")", ";"]),
            &["x".into(), "y".into()],
            |_| false,
        )
        .unwrap();
        let Statement::If { condition, .. } = parsed.statement_body.unwrap().statements[0] else {
            panic!("expected an if statement")
        };
        let ExpressionKind::Group { operand } = parsed.nodes[condition].kind else {
            panic!("condition delimiters must form a group")
        };
        assert!(matches!(
            parsed.nodes[operand].kind,
            ExpressionKind::Binary { operator: BinaryOperator::LogicalAnd, .. }
        ));
    }

    /// Checks conditional null branches and complete wrappers preserve boundaries.
    #[test]
    fn conditional_null_branches_and_complete_wrappers_preserve_boundaries() {
        let branch = ["if", "(", "x", ")", ";", "else", "{", "x", "++", ";", "}"];
        for (source, requires_boundary) in [
            (branch.to_vec(), true),
            ([&["{"][..], &branch, &["}"]].concat(), false),
            ([&["do", "{"][..], &branch, &["}", "while", "(", "0", ")"]].concat(), false),
        ] {
            let parsed = parse_replacement(&tokens(&source), &["x".into()], |_| false).unwrap();
            let body = parsed.statement_body.unwrap();
            assert_eq!(body.requires_boundary, requires_boundary);
            let Statement::If { then_branch, else_branch: Some(otherwise), .. } =
                &body.statements[0]
            else {
                panic!("the conditional must retain both branches")
            };
            assert!(matches!(
                then_branch.as_ref(),
                Statement::Block { statements, .. } if statements.is_empty()
            ));
            assert!(matches!(
                otherwise.as_ref(),
                Statement::Block { statements, .. } if statements.len() == 1
            ));
        }
        let parsed = parse_replacement(
            &tokens(&[
                "if", "(", "x", ",", "y", ")", "do", "{", "x", "++", ";", "}", "while", "(", "0",
                ")", ";", "else", "y", "++", ";",
            ]),
            &["x".into(), "y".into()],
            |_| false,
        )
        .unwrap();
        let Statement::If { condition, .. } = parsed.statement_body.unwrap().statements[0] else {
            panic!("expected an if statement")
        };
        let ExpressionKind::Group { operand } = parsed.nodes[condition].kind else {
            panic!("condition delimiters must form a group")
        };
        assert!(matches!(parsed.nodes[operand].kind, ExpressionKind::Comma { .. }));
    }

    /// Checks conditional returns keep first return metadata separate from fallthrough.
    #[test]
    fn conditional_returns_keep_first_return_metadata_separate_from_fallthrough() {
        for (source, always_returns, returns) in [
            (vec!["{", "if", "(", "x", ")", "return", "1", ";", "x", "++", ";", "}"], false, 1),
            (
                vec![
                    "{", "if", "(", "x", ")", "{", "return", "1", ";", "}", "x", "++", ";",
                    "return", "2", ";", "}",
                ],
                true,
                2,
            ),
            (vec!["if", "(", "x", ")", "return", "1", ";", "else", "return", "2", ";"], true, 2),
        ] {
            let parsed = parse_replacement(&tokens(&source), &["x".into()], |_| false).unwrap();
            let body = parsed.statement_body.unwrap();
            assert_eq!(body.always_returns, always_returns);
            assert!(matches!(
                &parsed.nodes[parsed.root].kind,
                ExpressionKind::IntegerLiteral { literal } if literal.value == 1
            ));
            let returned = body
                .walk()
                .filter_map(|statement| match statement {
                    Statement::Return { expression, tokens } => Some((*expression, *tokens)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(returned.len(), returns);
            assert_eq!(returned[0], (parsed.root, body.return_tokens.unwrap()));
        }
        assert!(
            parse_replacement(
                &tokens(&[
                    "{", "if", "(", "x", ")", "return", "1", ";", "else", "return", "2", ";", "x",
                    "++", ";", "}",
                ]),
                &["x".into()],
                |_| false,
            )
            .is_err()
        );
    }

    /// Checks conditional block locals remain lexically scoped.
    #[test]
    fn conditional_block_locals_remain_lexically_scoped() {
        let parsed = parse_replacement(
            &tokens(&[
                "{", "int", "outer", "=", "0", ";", "if", "(", "x", ")", "{", "int", "tmp", "=",
                "1", ";", "outer", "+=", "tmp", ";", "}", "else", "{", "int", "other", "=", "2",
                ";", "outer", "+=", "other", ";", "}", "outer", "++", ";", "}",
            ]),
            &["x".into()],
            |name| name == "int",
        )
        .unwrap();
        assert_eq!(
            parsed
                .statement_body
                .unwrap()
                .walk()
                .filter(|statement| matches!(statement, Statement::Declaration { .. }))
                .count(),
            3,
        );
        for source in [
            vec!["if", "(", "x", ")", "int", "tmp", "=", "1", ";"],
            vec![
                "if", "(", "x", ")", "{", "int", "tmp", "=", "1", ";", "}", "else", "tmp", "++",
                ";",
            ],
            vec![
                "{", "if", "(", "x", ")", "{", "int", "tmp", "=", "1", ";", "}", "tmp", "++", ";",
                "}",
            ],
            vec!["if", "(", "tmp", ")", "{", "int", "tmp", "=", "1", ";", "}"],
            vec![
                "if", "(", "x", ")", "{", "int", "tmp", "=", "1", ";", "}", "else", "{", "int",
                "tmp", "=", "2", ";", "}",
            ],
        ] {
            assert_eq!(
                parse_replacement(&tokens(&source), &["x".into()], |name| name == "int")
                    .unwrap_err()
                    .kind,
                SyntaxErrorKind::Statement,
                "{source:?}",
            );
        }
    }

    /// Checks conditional statement grammar does not swallow semicolons or missing operands.
    #[test]
    fn conditional_statement_grammar_does_not_swallow_semicolons_or_missing_operands() {
        for source in [
            vec!["if"],
            vec!["if", "("],
            vec!["if", "(", "x", ")"],
            vec!["if", "(", ")", ";"],
            vec!["if", "(", "x", ")", ";", "else"],
            vec!["if", "(", "x", ")", "{", "x", "++", ";", "}", ";", "else", "x", "--", ";"],
            vec![
                "if", "(", "x", ")", "do", "{", "x", "++", ";", "}", "while", "(", "0", ")",
                "else", "x", "--", ";",
            ],
            vec!["if", "(", "x", ")", "do", "{", "x", "++", ";", "}", "while", "(", "1", ")", ";"],
        ] {
            assert!(
                parse_replacement(&tokens(&source), &["x".into()], |_| false).is_err(),
                "{source:?}",
            );
        }
        let nested = std::iter::repeat_n(["if", "(", "x", ")"], MAX_DEPTH + 1)
            .flatten()
            .chain([";"])
            .collect::<Vec<_>>();
        assert_eq!(
            parse_replacement(&tokens(&nested), &["x".into()], |_| false).unwrap_err().kind,
            SyntaxErrorKind::BudgetExceeded,
        );
    }

    /// Checks root conditional fragments allow a caller supplied final semicolon only at EOF.
    #[test]
    fn root_conditional_fragments_allow_a_caller_supplied_final_semicolon_only_at_eof() {
        for source in [
            vec!["if", "(", "x", ")", "x", "++"],
            vec!["if", "(", "x", ")", "return", "1"],
            vec!["if", "(", "x", ")", "x", "++", ";", "else", "x", "--"],
            vec!["if", "(", "x", ")", "if", "(", "y", ")", "x", "++", ";", "else", "y", "--"],
            vec![
                "if", "(", "x", ")", "return", "1", ";", "else", "if", "(", "y", ")", "return", "2",
            ],
            vec!["if", "(", "x", ")", "do", "{", "}", "while", "(", "0", ")"],
            vec!["if", "(", "x", ")", "{", "}", "else", "do", "{", "}", "while", "(", "0", ")"],
        ] {
            let parsed = parse_replacement(&tokens(&source), &["x".into(), "y".into()], |_| false)
                .unwrap_or_else(|error| panic!("{source:?}: {error:?}"));
            let body = parsed.statement_body.unwrap();
            assert!(body.requires_boundary);
            assert_eq!(body.tokens.end, source.len());
        }
        for source in [
            vec!["if", "(", "x", ")", "x", "++", "else", "x", "--"],
            vec!["if", "(", "x", ")", "return", "1", "else", "return", "2"],
            vec!["{", "if", "(", "x", ")", "x", "++", "}"],
            vec!["if", "(", "x", ")", "{", "x", "++", "}"],
            vec!["if", "(", "x", ")", "do", "{", "}", "while", "(", "0", ")", "else", "x", "--"],
            vec!["if", "(", "x", ")", "{", "}", ";", "else", "x", "--"],
        ] {
            assert!(
                parse_replacement(&tokens(&source), &["x".into()], |_| false).is_err(),
                "{source:?}",
            );
        }
    }
}
