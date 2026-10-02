//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Bounded parsing of expressions and structured statements in C macro replacement lists.
//!
//! Parameters remain holes. This parser never expands a preprocessing token or guesses a
//! declaration. The arena preserves explicit grouping and parameter occurrences, and avoids
//! recursively serialized trees for long left-associated expressions.

use crate::{Token, TokenKind};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

mod statements;

const MAX_TOKENS: usize = 4096;
const MAX_DEPTH: usize = 64;

/// Half-open indices into a macro's replacement-token list, including comment tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenRange {
    pub start: usize,
    pub end: usize,
}

/// An index into [`Expression::nodes`]. Children precede their parents.
pub type NodeId = usize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expression {
    pub nodes: Vec<ExpressionNode>,
    pub root: NodeId,
    /// A complete statement replacement with expressions in this same arena.
    /// The root is the first return operand, or `Empty` when there is no return.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_body: Option<StatementBody>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatementBody {
    pub statements: Vec<Statement>,
    pub tokens: TokenRange,
    /// The first return in source order, including returns inside branches.
    pub return_tokens: Option<TokenRange>,
    /// Every path through the supported statement tree returns from its caller.
    pub always_returns: bool,
    /// An unbraced root `if` needs a caller boundary to preserve C's dangling-else contract.
    pub requires_boundary: bool,
}

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Statement {
    Expression {
        expression: NodeId,
        tokens: TokenRange,
    },
    Declaration {
        name: String,
        type_name: String,
        initializer: Option<NodeId>,
        tokens: TokenRange,
    },
    Block {
        statements: Vec<Statement>,
        tokens: TokenRange,
    },
    Return {
        expression: NodeId,
        tokens: TokenRange,
    },
    If {
        condition: NodeId,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
        tokens: TokenRange,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionNode {
    pub kind: ExpressionKind,
    pub tokens: TokenRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExpressionKind {
    Empty,
    Parameter { index: usize },
    IntegerLiteral { literal: IntegerLiteral },
    Identifier { name: String },
    Group { operand: NodeId },
    Unary { operator: UnaryOperator, operand: NodeId },
    Binary { operator: BinaryOperator, left: NodeId, right: NodeId },
    Cast { type_name: String, operand: NodeId },
    Conditional { condition: NodeId, then_value: NodeId, else_value: NodeId },
    Comma { left: NodeId, right: NodeId },
    Call { callee: NodeId, arguments: Vec<NodeId> },
    Member { base: NodeId, field: String, field_parameter: Option<usize>, indirect: bool },
    Index { base: NodeId, index: NodeId },
    Dereference { operand: NodeId },
    AddressOf { operand: NodeId },
    Assignment { operator: Option<BinaryOperator>, place: NodeId, value: NodeId },
    Update { operand: NodeId, increment: bool, postfix: bool },
    SizeOfType { type_name: String },
    SizeOfExpression { operand: NodeId },
    AlignOfType { type_name: String },
    SizeOfTypeParameter { parameter: usize, pointers: u8, is_const: bool },
    AlignOfTypeParameter { parameter: usize, pointers: u8, is_const: bool },
    TypeParameterCast { parameter: usize, pointers: u8, is_const: bool, operand: NodeId },
    OffsetOf { record: OffsetRecord, fields: Vec<OffsetComponent> },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OffsetRecord {
    Named { name: String },
    Parameter { index: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OffsetComponent {
    Named { name: String },
    Parameter { index: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnaryOperator {
    Plus,
    Negate,
    BitwiseNot,
    LogicalNot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinaryOperator {
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
    ShiftLeft,
    ShiftRight,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    BitAnd,
    BitXor,
    BitOr,
    LogicalAnd,
    LogicalOr,
}

/// Integer spelling before target-dependent selection of its C type.
///
/// Supported ordinary character constants retain their spelling and use an unsuffixed
/// decimal magnitude, which has C `int` type for the bounded ASCII value range. Analysis
/// must establish the target's basic execution character set before accepting those constants.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegerLiteral {
    pub spelling: String,
    /// The positive magnitude; unary minus is a separate C operator.
    pub value: u128,
    pub radix: u8,
    pub suffix: IntegerSuffix,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegerSuffix {
    pub unsigned: bool,
    /// Zero, one, or two `L` characters.
    pub long: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SyntaxError {
    pub kind: SyntaxErrorKind,
    pub tokens: TokenRange,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SyntaxErrorKind {
    InvalidExpression,
    UnsupportedLiteral,
    PointerOperation,
    Mutation,
    Statement,
    Call,
    UnevaluatedExpression,
    TypeParameter,
    CommaExpression,
    BudgetExceeded,
}

struct Lexeme<'a> {
    token: &'a Token,
    index: usize,
}

struct Parser<'a, F> {
    tokens: Vec<Lexeme<'a>>,
    position: usize,
    parameters: HashMap<&'a str, usize>,
    type_parameters: HashSet<usize>,
    nodes: Vec<ExpressionNode>,
    is_type: F,
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
        .filter_map(|node| match node.kind {
            ExpressionKind::TypeParameterCast { parameter, .. }
            | ExpressionKind::SizeOfTypeParameter { parameter, .. }
            | ExpressionKind::AlignOfTypeParameter { parameter, .. } => Some(parameter),
            ExpressionKind::OffsetOf { record: OffsetRecord::Parameter { index }, .. } => {
                Some(index)
            }
            _ => None,
        })
        .collect();
    if !parser.type_parameters.is_empty()
        && parser
            .nodes
            .iter()
            .any(|node| matches!(node.kind, ExpressionKind::SizeOfExpression { .. }))
    {
        // A later type-only use can establish an earlier sizeof operand. Reparse
        // once rather than leaving orphaned value nodes and duplicate hole uses.
        parser.position = 0;
        parser.nodes.clear();
        (root, statement_body) = parser.replacement(allow_statements)?;
        if parser.position != parser.tokens.len() {
            return Err(parser.unexpected());
        }
    }
    Ok(Expression { nodes: parser.nodes, root, statement_body })
}

impl<F: Fn(&str) -> bool> Parser<'_, F> {
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
                    self.position = end + 1;
                    let operand = self.expression(14, depth + 1)?;
                    let tokens = TokenRange { start, end: self.nodes[operand].tokens.end };
                    return Ok(self.push(
                        ExpressionKind::TypeParameterCast {
                            parameter,
                            pointers,
                            is_const,
                            operand,
                        },
                        tokens,
                    ));
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
                    "statement and caller control-flow macros are deferred",
                ));
            }
            let kind = ExpressionKind::Identifier { name: spelling.into() };
            self.position += 1;
            return Ok(self.push(kind, TokenRange { start, end: start + 1 }));
        }
        Err(self.unexpected())
    }

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

    fn type_parameter(&self) -> Option<(usize, usize, u8, bool)> {
        let (end, parameter, pointers, is_const) = self.parenthesized_type_parameter()?;
        let next = self.tokens.get(end + 1)?;
        (matches!(next.token.kind, TokenKind::Identifier | TokenKind::Keyword | TokenKind::Literal)
            || matches!(next.token.spelling.as_str(), "(" | "~" | "!")
            || ((pointers != 0 || is_const || self.type_parameters.contains(&parameter))
                && matches!(next.token.spelling.as_str(), "+" | "-" | "*" | "&" | "++" | "--")))
        .then_some((end, parameter, pointers, is_const))
    }

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

    fn push(&mut self, kind: ExpressionKind, tokens: TokenRange) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(ExpressionNode { kind, tokens });
        id
    }

    fn spelling(&self) -> Option<&str> {
        self.tokens.get(self.position).map(|token| token.token.spelling.as_str())
    }

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

    fn unexpected(&self) -> SyntaxError {
        let spelling = self.spelling().unwrap_or("end of replacement list");
        let (kind, message) = match spelling {
            "[" | "." | "->" => (
                SyntaxErrorKind::PointerOperation,
                "array/member access requires a memory contract",
            ),
            "++" | "--" | "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "<<=" | ">>=" | "&=" | "^="
            | "|=" => {
                (SyntaxErrorKind::Mutation, "mutation and place-valued expressions are deferred")
            }
            "{" | "}" | ";" => (SyntaxErrorKind::Statement, "statement macros are deferred"),
            "(" => (
                SyntaxErrorKind::Call,
                "calls require preprocessing/declaration and evaluation contracts",
            ),
            "," => (SyntaxErrorKind::CommaExpression, "comma expressions are deferred"),
            _ => (SyntaxErrorKind::InvalidExpression, "not a complete supported C expression"),
        };
        self.error(kind, format!("{message}: {spelling:?}"))
    }

    fn error(&self, kind: SyntaxErrorKind, message: impl Into<String>) -> SyntaxError {
        let start = self.tokens.get(self.position).map_or(self.original_len, |token| token.index);
        SyntaxError {
            kind,
            tokens: TokenRange { start, end: (start + 1).min(self.original_len) },
            message: message.into(),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
            &tokens(&["(", "sizeof", "(", "t", ")", ",", "(", "t", ")", "(", "x", ")", ")"]),
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

    #[test]
    fn type_hole_reparse_keeps_return_statements_in_one_arena() {
        let parsed = parse_replacement(
            &tokens(&[
                "{", "int", "tmp", "=", "sizeof", "(", "t", ")", ";", "return", "(", "t", ")", "(",
                "x", ")", ";", "}",
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
