//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Give a function-like macro that the build flavors define differently one definition.
//!
//! The one definition holds both. As an expression it is
//! `( __pgrx_c_assert_checking ? ( assert-enabled ) : ( release ) )`, and as a statement
//! `do { __pgrx_c_assert_checking { assert-enabled ; } { release ; } } while ( 0 )`. Each
//! build then evaluates its own definition and compiles the other. The keyword is a token
//! that no C source holds, so the parser reads these forms nowhere else.
//!
//! Wrapping a replacement list in parentheses or braces must not change what any use of the
//! macro means. So a definition takes part only when it is closed: one parenthesized
//! expression, one call, one `do { } while (0)` statement, one invocation of a macro that is
//! one of those, or nothing at all. `2 * M()` reads `1 + 1` differently from `(1 + 1)`, and
//! an unbraced `if` takes the `else` that follows its use.
//!
//! `Assert` and its relatives are expanded inside hundreds of macros and inline functions,
//! as the branch of an `if` and inside comma expressions, and their two definitions are a
//! statement and a `void` expression. They keep the assert-enabled definition in both
//! flavors, which is sound when the release definition does nothing and the assert-enabled
//! one, with its checks off, does nothing either. [`checks`] recognizes such a definition by
//! its tokens, more narrowly than the emitter recognizes a check, and the emitter proves it
//! again on the analyzed definition before anything that expands it is generated.

use super::ASSERTION_REPORT;
use crate::syntax::build_flavor_keyword;
use crate::{Token, TokenKind};
use std::collections::BTreeMap;

/// Bound the tokens read to decide that a replacement list does nothing.
const MAX_NOTHING_TOKENS: usize = 256;

/// What a replacement list yields when it evaluates nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Inert {
    /// No value: a statement, or an expression of type `void`.
    Nothing,
    /// An integer constant.
    Integer(u128),
}

/// The index of the token that closes the bracket opened at `open`, when every bracket
/// between them pairs.
fn closing(tokens: &[&str], open: usize) -> Option<usize> {
    let mut expected = Vec::new();
    for (index, token) in tokens.iter().enumerate().skip(open) {
        match *token {
            "(" => expected.push(")"),
            "[" => expected.push("]"),
            "{" => expected.push("}"),
            ")" | "]" | "}" => {
                if expected.pop() != Some(*token) {
                    return None;
                }
                if expected.is_empty() {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

/// Whether `tokens` is one bracketed group and nothing else.
fn grouped(tokens: &[&str]) -> bool {
    tokens.len() >= 2 && closing(tokens, 0) == Some(tokens.len() - 1)
}

/// The value of `value` converted by a cast to the type spelled `ty`, for the types whose
/// conversion is known from the spelling alone.
fn cast(ty: &str, value: Inert) -> Option<Inert> {
    match (ty, value) {
        ("void", _) => Some(Inert::Nothing),
        // Zero and one are themselves in every type a header could name `bool`.
        ("bool" | "_Bool", Inert::Integer(0 | 1)) => Some(value),
        _ => None,
    }
}

/// What the replacement list `body` yields, when it names no operand, function or variable
/// and writes nothing, so it does nothing wherever it is used.
///
/// PostgreSQL defines such a macro as `((void)true)`, `((void)0)` or `do {} while (0)` where
/// a release build has no assertion to make. `object` gives the replacement list of an
/// object-like macro, such as `true`. Only grouping, a cast to `void` or `bool`, integer
/// constants, an empty block and an empty `do`/`while (0)` are read; anything else is not
/// nothing.
pub(super) fn nothing<'a>(
    body: &[&'a str],
    object: &impl Fn(&str) -> Option<Vec<&'a str>>,
) -> Option<Inert> {
    let mut tokens = Vec::new();
    flatten(body, object, &mut Vec::new(), &mut tokens)?;
    // As a statement it may end with its semicolon.
    let statement = match tokens.split_last() {
        Some((&";", rest)) => rest,
        _ => tokens.as_slice(),
    };
    let block = |tokens: &[&str]| {
        grouped(tokens)
            && tokens[0] == "{"
            && tokens[1..tokens.len() - 1].iter().all(|token| *token == ";")
    };
    if statement.is_empty() || block(statement) {
        return Some(Inert::Nothing);
    }
    if let ["do", rest @ ..] = statement
        && let Some(close) = closing(rest, 0)
        && block(&rest[..=close])
        && let ["while", condition @ ..] = &rest[close + 1..]
        && grouped(condition)
        && condition[0] == "("
    {
        // Any other condition would loop.
        return (constant(condition)? == Inert::Integer(0)).then_some(Inert::Nothing);
    }
    constant(&tokens)
}

/// Replace each object-like macro in `body` by its replacement list, as the preprocessor
/// does. A macro that names itself, at any depth, stays an identifier there, which is not
/// nothing.
fn flatten<'a>(
    body: &[&'a str],
    object: &impl Fn(&str) -> Option<Vec<&'a str>>,
    expanding: &mut Vec<&'a str>,
    tokens: &mut Vec<&'a str>,
) -> Option<()> {
    for token in body {
        match object(token) {
            Some(_) if expanding.contains(token) => return None,
            Some(replacement) => {
                expanding.push(token);
                flatten(&replacement, object, expanding, tokens)?;
                expanding.pop();
            }
            None => tokens.push(token),
        }
        if tokens.len() > MAX_NOTHING_TOKENS {
            return None;
        }
    }
    Some(())
}

/// The value of an expression that is only grouping, casts and one integer constant.
fn constant(tokens: &[&str]) -> Option<Inert> {
    match tokens {
        ["true"] => Some(Inert::Integer(1)),
        ["false"] => Some(Inert::Integer(0)),
        [literal] => crate::syntax::integer_literal_value(literal).map(Inert::Integer),
        ["(", ..] => {
            let close = closing(tokens, 0)?;
            if close == tokens.len() - 1 {
                constant(&tokens[1..close])
            } else if close == 2 {
                cast(tokens[1], constant(&tokens[3..])?)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// What an assert-enabled replacement list yields with its checks off, when it is nothing
/// but checks: it reports through `ExceptionalCondition` and does no other work.
///
/// It is one of these, where `report` is a call to `ExceptionalCondition`:
///
/// - `do { if ( condition ) report ; } while ( 0 )`, as `Assert`;
/// - `( ( void ) ( holds || ( report , 0 ) ) )`, as `AssertMacro`, where `holds` is
///   `( condition )` or `! ( condition )` and the cast may also be to `bool`, as PostgreSQL
///   15's `TrapMacro`;
/// - one invocation of a macro in `checked`, as PostgreSQL 15's `AssertPointerAlignment`
///   invokes `Trap`.
///
/// Each is closed, so no use can reach into it. The emitter turns off more shapes than
/// these, and proves each macro admitted here on its analyzed definition.
pub(super) fn checks(body: &[&str], checked: &BTreeMap<String, Inert>) -> Option<Inert> {
    if let ["do", "{", "if", "(", ..] = body {
        let condition = closing(body, 3)?;
        let report = report(&body[condition + 1..])?;
        let rest = &body[condition + 1 + report..];
        return (rest == [";", "}", "while", "(", "0", ")"]).then_some(Inert::Nothing);
    }
    check(body, checked)
}

/// The length of the call to `ExceptionalCondition` that `tokens` begins with.
fn report(tokens: &[&str]) -> Option<usize> {
    match tokens {
        [ASSERTION_REPORT, "(", ..] => Some(closing(tokens, 1)? + 1),
        _ => None,
    }
}

/// What a closed expression that only checks yields with its checks off.
fn check(tokens: &[&str], checked: &BTreeMap<String, Inert>) -> Option<Inert> {
    if let [name, "(", ..] = tokens
        && *name != "("
    {
        return checked
            .get(*name)
            .copied()
            .filter(|_| closing(tokens, 1) == Some(tokens.len() - 1));
    }
    if !grouped(tokens) || tokens[0] != "(" {
        return None;
    }
    let inner = &tokens[1..tokens.len() - 1];
    // `holds || ( report , 0 )`, which is true unless the report stops the process.
    let mut depth = 0_usize;
    let mut alternatives = inner.iter().enumerate().filter(|(_, token)| {
        match **token {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => depth = depth.saturating_sub(1),
            _ => {}
        }
        depth == 0 && **token == "||"
    });
    match (alternatives.next(), alternatives.next()) {
        (Some((position, _)), None) => {
            let (holds, failure) = (&inner[..position], &inner[position + 1..]);
            let holds = holds.strip_prefix(&["!"][..]).unwrap_or(holds);
            let reported = grouped(holds)
                && holds[0] == "("
                && grouped(failure)
                && failure[0] == "("
                && report(&failure[1..])
                    .is_some_and(|length| failure[1 + length..] == [",", "0", ")"]);
            return reported.then_some(Inert::Integer(1));
        }
        (Some(_), Some(_)) => return None,
        _ => {}
    }
    // `( type ) operand`
    if let ["(", ty @ ("void" | "bool" | "_Bool"), ")", operand @ ..] = inner {
        return cast(ty, check(operand, checked)?);
    }
    check(inner, checked)
}

/// Whether putting parentheses around the expression `body` changes no use of it.
///
/// It is one parenthesized group, one constant or name, or one call. `formal` says that a
/// token is a parameter of the macro, whose operand can be any tokens. `callee` says that
/// an identifier stands for itself or for a closed expression: it is no macro, or it is a
/// function-like macro whose own replacement list is closed. A keyword such as `sizeof` is
/// no callee: it binds what follows its use.
pub(super) fn closed_expression(
    body: &[&Token],
    formal: &impl Fn(&str) -> bool,
    callee: &impl Fn(&str) -> bool,
) -> bool {
    let spellings = body.iter().map(|token| token.spelling.as_str()).collect::<Vec<_>>();
    match body {
        [] => false,
        [only] => match only.kind {
            TokenKind::Literal => true,
            TokenKind::Identifier => !formal(&only.spelling) && callee(&only.spelling),
            _ => false,
        },
        [open, ..] if open.spelling == "(" => grouped(&spellings),
        _ => invoked(body).is_some_and(|name| !formal(name) && callee(name)),
    }
}

/// Whether `body` is `do { statements } while ( 0 )` and nothing else, so that it is one
/// statement wherever it is used and takes no `else` that follows.
pub(super) fn closed_statement(body: &[&Token]) -> bool {
    let spellings = body.iter().map(|token| token.spelling.as_str()).collect::<Vec<_>>();
    matches!(spellings.as_slice(), ["do", "{", ..])
        && closing(&spellings, 1)
            .is_some_and(|close| spellings[close + 1..] == ["while", "(", "0", ")"])
}

/// The identifier that `body` invokes, when `body` is one invocation with its arguments and
/// nothing else.
pub(super) fn invoked<'a>(body: &[&'a Token]) -> Option<&'a str> {
    let spellings = body.iter().map(|token| token.spelling.as_str()).collect::<Vec<_>>();
    match body {
        [name, open, ..]
            if name.kind == TokenKind::Identifier
                && open.spelling == "("
                && closing(&spellings, 1) == Some(body.len() - 1) =>
        {
            Some(&name.spelling)
        }
        _ => None,
    }
}

/// One build flavor's definition of a macro, for [`select`].
pub(super) struct Side<'a> {
    /// The replacement list, with the parameter names both flavors' definitions use.
    pub(super) body: Vec<&'a Token>,
    /// Whether it is a closed expression.
    pub(super) expression: bool,
    /// Whether it is a closed statement.
    pub(super) statement: bool,
    /// Whether it does nothing.
    pub(super) nothing: bool,
}

/// The replacement list that holds both `checked`, of a build with assertions, and
/// `unchecked`, of a build without, or why none does.
pub(super) fn select(checked: &Side<'_>, unchecked: &Side<'_>) -> Result<Vec<Token>, &'static str> {
    let punctuation =
        |spelling: &str| Token { kind: TokenKind::Punctuation, spelling: spelling.into() };
    let mut tokens = Vec::new();
    if checked.expression && unchecked.expression {
        tokens.extend([punctuation("("), build_flavor_keyword(), punctuation("?")]);
        for (side, after) in [(checked, ":"), (unchecked, ")")] {
            tokens.push(punctuation("("));
            tokens.extend(side.body.iter().map(|token| (*token).clone()));
            tokens.extend([punctuation(")"), punctuation(after)]);
        }
        return Ok(tokens);
    }
    let statement = |side: &Side<'_>| side.statement || side.nothing;
    if statement(checked) && statement(unchecked) {
        let word = |spelling: &str| Token { kind: TokenKind::Keyword, spelling: spelling.into() };
        tokens.extend([word("do"), punctuation("{"), build_flavor_keyword()]);
        for side in [checked, unchecked] {
            tokens.push(punctuation("{"));
            // A definition that does nothing needs no statement.
            if !side.nothing {
                tokens.extend(side.body.iter().map(|token| (*token).clone()));
                tokens.push(punctuation(";"));
            }
            tokens.push(punctuation("}"));
        }
        tokens.extend([punctuation("}"), word("while"), punctuation("(")]);
        tokens.push(Token { kind: TokenKind::Literal, spelling: "0".into() });
        tokens.push(punctuation(")"));
        return Ok(tokens);
    }
    if (checked.statement && unchecked.expression) || (checked.expression && unchecked.statement) {
        return Err("one build flavor defines it as a statement and the other as a value");
    }
    Err(
        "its two definitions are not each one parenthesized expression, one call or one `do { } while (0)` statement, so no one definition can hold both",
    )
}

/// `body` with each parameter in `from` renamed to the one at its position in `to`, or
/// `None` when the lists differ in length or variadics, or when a renamed parameter would
/// capture another name the body uses.
pub(super) fn rename(body: &[&Token], from: &[&str], to: &[&str]) -> Option<Vec<Token>> {
    if from.len() != to.len()
        || from.iter().zip(to).any(|(from, to)| (*from == "...") != (*to == "..."))
    {
        return None;
    }
    body.iter()
        .map(|token| {
            if !matches!(token.kind, TokenKind::Identifier | TokenKind::Keyword) {
                return Some((*token).clone());
            }
            match from.iter().position(|name| *name == token.spelling) {
                Some(position) => {
                    Some(Token { kind: token.kind, spelling: to[position].to_owned() })
                }
                None if to.contains(&token.spelling.as_str()) => None,
                None => Some((*token).clone()),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        Inert, Side, checks, closed_expression, closed_statement, nothing, rename, select,
    };
    use crate::syntax::BUILD_FLAVOR;
    use crate::{Token, TokenKind};
    use std::collections::BTreeMap;

    /// Tokens from space-separated spellings, with the kinds a C lexer gives them.
    fn tokens(text: &str) -> Vec<Token> {
        text.split_ascii_whitespace()
            .map(|spelling| {
                let first = spelling.as_bytes()[0];
                let kind = if matches!(spelling, "do" | "while" | "if" | "void" | "sizeof") {
                    TokenKind::Keyword
                } else if first.is_ascii_digit() || first == b'"' {
                    TokenKind::Literal
                } else if first.is_ascii_alphabetic() || first == b'_' {
                    TokenKind::Identifier
                } else {
                    TokenKind::Punctuation
                };
                Token { kind, spelling: spelling.into() }
            })
            .collect()
    }

    /// The object-like macros a release definition may name.
    fn object<'a>(name: &str) -> Option<Vec<&'a str>> {
        match name {
            "true" => Some(vec!["1"]),
            "NOTHING" => Some(vec!["(", "(", "void", ")", "true", ")"]),
            "COUNTED" => Some(vec!["counter", "++"]),
            "LOOP" => Some(vec!["LOOP"]),
            "TWICE" => Some(vec!["ONCE"]),
            "ONCE" => Some(vec!["TWICE"]),
            _ => None,
        }
    }

    /// What a replacement list yields, when it does nothing.
    fn yields(text: &str) -> Option<Inert> {
        nothing(&text.split_ascii_whitespace().collect::<Vec<_>>(), &object)
    }

    /// A release definition does nothing only when it names no operand, function or
    /// variable and writes nothing, also through the object-like macros it names.
    #[test]
    fn nothing_reads_only_grouping_void_casts_constants_and_empty_statements() {
        for text in [
            "",
            ";",
            "{ }",
            "( ( void ) true )",
            "( ( void ) 0 )",
            "( ( void ) ( ( bool ) 1 ) )",
            "do { } while ( 0 )",
            "do { ; } while ( ( 0 ) ) ;",
            "NOTHING",
        ] {
            assert_eq!(yields(text), Some(Inert::Nothing), "{text}");
        }
        assert_eq!(yields("( true )"), Some(Inert::Integer(1)));
        assert_eq!(yields("( ( bool ) 0 )"), Some(Inert::Integer(0)));
        assert_eq!(yields("0x10"), Some(Inert::Integer(16)));
        for text in [
            // It evaluates its operand, which may have an effect.
            "( ( void ) ( x ) )",
            "( * ( x ) = 0 )",
            "counter ++",
            "function ( )",
            "COUNTED",
            "( ( void ) \"text\" )",
            // An identifier that is left after expansion names something.
            "LOOP",
            "TWICE",
            // The loop never ends.
            "do { } while ( 1 )",
            "do { counter ++ ; } while ( 0 )",
            // A conversion that the spelling does not decide.
            "( ( bool ) 2 )",
            "( ( Oid ) 0 )",
            "( ( void ) 0",
            "1 2",
        ] {
            assert_eq!(yields(text), None, "{text}");
        }
    }

    /// Only the closed shapes of an assertion are checks, with what they yield when off.
    #[test]
    fn checks_reads_the_closed_shapes_of_an_assertion() {
        let mut checked = BTreeMap::new();
        let yields = |text: &str, checked: &BTreeMap<String, Inert>| {
            checks(&text.split_ascii_whitespace().collect::<Vec<_>>(), checked)
        };
        let report = "ExceptionalCondition ( # c , __FILE__ , __LINE__ )";
        for (text, expected) in [
            (format!("do {{ if ( ! ( c ) ) {report} ; }} while ( 0 )"), Inert::Nothing),
            (format!("do {{ if ( c ) {report} ; }} while ( 0 )"), Inert::Nothing),
            (format!("( ( void ) ( ( c ) || ( {report} , 0 ) ) )"), Inert::Nothing),
            (format!("( ( bool ) ( ! ( c ) || ( {report} , 0 ) ) )"), Inert::Integer(1)),
            (format!("( ( c ) || ( {report} , 0 ) )"), Inert::Integer(1)),
        ] {
            assert_eq!(yields(&text, &checked), Some(expected), "{text}");
        }
        for text in [
            // Work beside the check.
            format!("do {{ counter ++ ; if ( ! ( c ) ) {report} ; }} while ( 0 )"),
            format!("do {{ if ( ! ( c ) ) {report} ; else counter ++ ; }} while ( 0 )"),
            format!("( counter ++ , ( void ) ( ( c ) || ( {report} , 0 ) ) )"),
            // The condition's operator would bind an operand of the use.
            format!("( c ) || ( {report} , 0 )"),
            format!("( ( void ) ( a || ( c ) || ( {report} , 0 ) ) )"),
            format!("( ( void ) ( c || ( {report} , 0 ) ) )"),
            // Not a report, or a report with something after it.
            "do { if ( ! ( c ) ) abort ( ) ; } while ( 0 )".to_owned(),
            format!("( ( void ) ( ( c ) || ( {report} , 1 ) ) )"),
            format!("( ( int ) ( ( c ) || ( {report} , 0 ) ) )"),
            "Trap ( c , \"BadState\" )".to_owned(),
            "( ( void ) true )".to_owned(),
        ] {
            assert_eq!(yields(&text, &checked), None, "{text}");
        }
        // An invocation of a macro that is itself only checks.
        checked.insert("Trap".to_owned(), Inert::Nothing);
        assert_eq!(yields("Trap ( c , \"BadState\" )", &checked), Some(Inert::Nothing));
        assert_eq!(yields("( Trap ( c , \"BadState\" ) )", &checked), Some(Inert::Nothing));
        assert_eq!(yields("Trap ( c ) + 1", &checked), None);
    }

    /// One side of a selection, from its replacement list.
    fn side(body: &[Token]) -> Side<'_> {
        let body = body.iter().collect::<Vec<_>>();
        let spellings = body.iter().map(|token| token.spelling.as_str()).collect::<Vec<_>>();
        Side {
            expression: closed_expression(&body, &|name| name == "x", &|name| name != "MACRO"),
            statement: closed_statement(&body),
            nothing: nothing(&spellings, &object).is_some(),
            body,
        }
    }

    /// The spellings of the one definition for two replacement lists.
    fn selected(checked: &str, unchecked: &str) -> Result<String, &'static str> {
        let (checked, unchecked) = (tokens(checked), tokens(unchecked));
        let selected = select(&side(&checked), &side(&unchecked))?;
        // The one token that no C source holds selects the build flavor.
        let keywords =
            selected.iter().filter(|token| token.spelling == BUILD_FLAVOR).collect::<Vec<_>>();
        assert!(matches!(keywords.as_slice(), [token] if token.kind == TokenKind::Keyword));
        Ok(selected
            .iter()
            .map(|token| token.spelling.as_str())
            .collect::<Vec<_>>()
            .join(" ")
            .replace(BUILD_FLAVOR, "FLAVOR"))
    }

    /// Two closed expressions become one expression, and statements one statement.
    #[test]
    fn select_holds_two_closed_definitions_in_one() {
        assert_eq!(
            selected("( ( x ) + 1 )", "( ( ( x ) + 1 ) % MAX )").as_deref(),
            Ok("( FLAVOR ? ( ( ( x ) + 1 ) ) : ( ( ( ( x ) + 1 ) % MAX ) ) )")
        );
        assert_eq!(
            selected("abort ( )", "__builtin_unreachable ( )").as_deref(),
            Ok("( FLAVOR ? ( abort ( ) ) : ( __builtin_unreachable ( ) ) )")
        );
        assert_eq!(
            selected("do { counter ++ ; } while ( 0 )", "( ( void ) true )").as_deref(),
            Ok("do { FLAVOR { do { counter ++ ; } while ( 0 ) ; } { } } while ( 0 )")
        );
        assert_eq!(
            selected("do { } while ( 0 )", "do { counter ++ ; } while ( 0 )").as_deref(),
            Ok("do { FLAVOR { } { do { counter ++ ; } while ( 0 ) ; } } while ( 0 )")
        );
        // Two void expressions stay an expression, which a comma expression can hold.
        assert_eq!(
            selected("( counter ++ , ( void ) 0 )", "( ( void ) true )").as_deref(),
            Ok("( FLAVOR ? ( ( counter ++ , ( void ) 0 ) ) : ( ( ( void ) true ) ) )")
        );
    }

    /// A definition that a use could reach into, or a statement beside a value, has no one
    /// definition.
    #[test]
    fn select_refuses_open_and_mixed_definitions() {
        let mixed = "one build flavor defines it as a statement and the other as a value";
        assert_eq!(selected("do { counter ++ ; } while ( 0 )", "( x )"), Err(mixed));
        assert_eq!(selected("( x )", "do { counter ++ ; } while ( 0 )"), Err(mixed));
        for (checked, unchecked) in [
            // An operator of the use would bind part of the definition.
            ("( x ) + 1", "( x )"),
            ("( x )", "x"),
            ("( x )", "- 1"),
            ("( int ) ( x )", "( x )"),
            // A parameter's operand, or a macro's replacement list, can be any tokens.
            ("x ( 1 )", "( x )"),
            ("MACRO ( 1 )", "( x )"),
            ("MACRO", "( x )"),
            // `sizeof` binds a subscript that follows the use.
            ("sizeof ( x )", "( x )"),
            // An unbraced `if` takes the `else` that follows the use.
            ("if ( x ) counter ++", "do { } while ( 0 )"),
            ("do { counter ++ ; } while ( 1 )", "do { } while ( 0 )"),
            ("{ counter ++ ; }", "do { } while ( 0 )"),
            ("( x )", ""),
        ] {
            assert!(selected(checked, unchecked).is_err(), "{checked} / {unchecked}");
        }
    }

    /// A release definition takes the assert-enabled parameter names unless one would
    /// capture a name it uses.
    #[test]
    fn rename_gives_both_definitions_one_set_of_parameters() {
        let renamed = |body: &str, from: &[&str], to: &[&str]| {
            let body = tokens(body);
            rename(&body.iter().collect::<Vec<_>>(), from, to).map(|tokens| {
                tokens.iter().map(|token| token.spelling.as_str()).collect::<Vec<_>>().join(" ")
            })
        };
        assert_eq!(renamed("( a + b )", &["a", "b"], &["a", "b"]).as_deref(), Some("( a + b )"));
        assert_eq!(renamed("( a - b )", &["a", "b"], &["b", "a"]).as_deref(), Some("( b - a )"));
        assert_eq!(renamed("( p # p )", &["p"], &["q"]).as_deref(), Some("( q # q )"));
        // `global` would become the parameter.
        assert_eq!(renamed("( p + global )", &["p"], &["global"]), None);
        assert_eq!(renamed("( p )", &["p"], &["a", "b"]), None);
        assert_eq!(renamed("( p )", &["p"], &["..."]), None);
    }
}
