//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Bounded parsing of the scalar-expression portion of C macro replacement lists.
//!
//! Parameters remain holes. This parser never expands a preprocessing token or guesses a
//! declaration. The arena preserves explicit grouping and parameter occurrences, and avoids
//! recursively serialized trees for long left-associated expressions.

use crate::{Token, TokenKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionNode {
    pub kind: ExpressionKind,
    pub tokens: TokenRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExpressionKind {
    Parameter { index: usize },
    IntegerLiteral { literal: IntegerLiteral },
    Identifier { name: String },
    Group { operand: NodeId },
    Unary { operator: UnaryOperator, operand: NodeId },
    Binary { operator: BinaryOperator, left: NodeId, right: NodeId },
    Cast { type_name: String, operand: NodeId },
    Conditional { condition: NodeId, then_value: NodeId, else_value: NodeId },
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
    EmptyReplacement,
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
        nodes: Vec::new(),
        is_type,
        original_len: tokens.len(),
    };
    if parser.tokens.is_empty() {
        return Err(parser.error(SyntaxErrorKind::EmptyReplacement, "empty replacement list"));
    }
    let root = parser.expression(0, 0)?;
    if parser.position != parser.tokens.len() {
        return Err(parser.unexpected());
    }
    Ok(Expression { nodes: parser.nodes, root })
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
            if self.spelling() == Some("?") && minimum <= 1 {
                self.position += 1;
                let then_value = self.expression(0, depth + 1)?;
                self.expect(":")?;
                let else_value = self.expression(1, depth + 1)?;
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
            let operand = self.expression(12, depth + 1)?;
            let tokens = TokenRange { start, end: self.nodes[operand].tokens.end };
            return Ok(self.push(ExpressionKind::Unary { operator, operand }, tokens));
        }
        match spelling {
            "(" => {
                if let Some((end, type_name)) = self.cast_type() {
                    self.position = end + 1;
                    let operand = self.expression(12, depth + 1)?;
                    let tokens = TokenRange { start, end: self.nodes[operand].tokens.end };
                    return Ok(self.push(ExpressionKind::Cast { type_name, operand }, tokens));
                }
                if let Some(tokens) = self.type_parameter() {
                    return Err(SyntaxError {
                        kind: SyntaxErrorKind::TypeParameter,
                        tokens,
                        message: "a macro parameter occupies a possible C type position".into(),
                    });
                }
                self.position += 1;
                let operand = self.expression(0, depth + 1)?;
                self.expect(")")?;
                let end = self.tokens[self.position - 1].index + 1;
                return Ok(self.push(ExpressionKind::Group { operand }, TokenRange { start, end }));
            }
            "*" | "&" => {
                return Err(self.error(
                    SyntaxErrorKind::PointerOperation,
                    "dereference and address-of require a memory contract",
                ));
            }
            "sizeof" | "_Alignof" | "__alignof__" | "__alignof" => {
                return Err(self.error(
                    SyntaxErrorKind::UnevaluatedExpression,
                    "size/alignment expressions require an unevaluated-operand contract",
                ));
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
            if matches!(spelling, "do" | "if" | "for" | "while" | "return" | "goto" | "switch") {
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

    fn type_parameter(&self) -> Option<TokenRange> {
        let mut end = self.position + 1;
        let mut has_parameter = false;
        while let Some(token) = self.tokens.get(end) {
            match token.token.spelling.as_str() {
                ")" => break,
                "const" | "volatile" | "*" => {}
                name if self.parameters.contains_key(name) && !has_parameter => {
                    has_parameter = true
                }
                _ => return None,
            }
            end += 1;
        }
        let close = self.tokens.get(end)?;
        let next = self.tokens.get(end + 1)?;
        (has_parameter
            && close.token.spelling == ")"
            && (matches!(
                next.token.kind,
                TokenKind::Identifier | TokenKind::Keyword | TokenKind::Literal
            ) || matches!(next.token.spelling.as_str(), "(" | "~" | "!")))
        .then_some(TokenRange { start: self.tokens[self.position].index, end: close.index + 1 })
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
                } else if spelling.as_bytes().first().is_some_and(u8::is_ascii_alphabetic) {
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
        assert_eq!(
            parse_expression(&tokens(&["x", "(", "1", ")"]), &["x".into()], |_| false)
                .unwrap_err()
                .kind,
            SyntaxErrorKind::Call
        );
        assert_eq!(
            parse_expression(&tokens(&["(", "t", ")", "x"]), &["t".into(), "x".into()], |_| false)
                .unwrap_err()
                .kind,
            SyntaxErrorKind::TypeParameter
        );
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
    fn unsupported_constructs_inside_grouping_keep_their_categories() {
        for (body, expected) in [
            (vec!["(", "(", "x", ")", ".", "field", ")"], SyntaxErrorKind::PointerOperation),
            (vec!["(", "x", "->", "field", ")"], SyntaxErrorKind::PointerOperation),
            (vec!["(", "x", "[", "0", "]", ")"], SyntaxErrorKind::PointerOperation),
            (vec!["(", "x", "++", ")"], SyntaxErrorKind::Mutation),
            (vec!["(", "x", ",", "1", ")"], SyntaxErrorKind::CommaExpression),
            (vec!["(", "x", "(", "1", ")", ")"], SyntaxErrorKind::Call),
        ] {
            assert_eq!(
                parse_expression(&tokens(&body), &["x".into()], |_| false).unwrap_err().kind,
                expected
            );
        }
    }
}
