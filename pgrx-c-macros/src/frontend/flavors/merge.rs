//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Merge the two definitions of a static inline function's body into one.
//!
//! A definition that holds `#ifdef USE_ASSERT_CHECKING` gives each build flavor its own
//! body tokens. Both flavors read them from one file, so a token both have sits at one
//! offset in both. The merged body keeps those tokens. Between two of them, the tokens
//! that only one flavor has become one statement,
//! `__pgrx_c_assert_checking { assert-enabled tokens } { release tokens }`, which the
//! statement parser admits only as an item of a block.
//!
//! The tokens of either block must close every `(`, `[` and `{` they open and nothing
//! else, so the braces written here pair with each other and never with a token of the
//! body. The statement must sit outside every `(` and `[`, so it is no part of a macro
//! invocation's arguments, which the preprocessor could drop or stringify.

use super::super::definitions::Located;
use crate::syntax::{BUILD_FLAVOR, build_flavor_keyword};
use crate::{Token, TokenKind};

/// Merge the body tokens of an assert-enabled build, `checked`, and those of a release
/// build, `unchecked`, each with its offset in the one file that holds the definition.
///
/// `None` when the bodies do not align, or when tokens that only one flavor has cannot
/// become a statement of their own.
pub(super) fn merge(checked: &[Located], unchecked: &[Located]) -> Option<Vec<Token>> {
    let mut merged = Vec::with_capacity(checked.len().max(unchecked.len()));
    let (mut checked, mut unchecked) = (checked.iter().peekable(), unchecked.iter().peekable());
    // The delimiters that the tokens both flavors have leave open.
    let mut open = Vec::new();
    // Where the last token read ends, at the earliest. A spelling has no line splices, so
    // the token's source text is at least as long.
    let mut end = 0_usize;
    let mut read = |(offset, token): &Located| {
        let start = usize::try_from(*offset).ok()?;
        if start < end || token.spelling == BUILD_FLAVOR {
            return None;
        }
        end = start.checked_add(token.spelling.len())?;
        Some(token.clone())
    };
    loop {
        let (mut only_checked, mut only_unchecked) = (Vec::new(), Vec::new());
        // Read up to the next token that both flavors have.
        let shared = loop {
            match (checked.peek(), unchecked.peek()) {
                (None, None) => break None,
                (Some(ours), Some(theirs)) if ours.0 == theirs.0 => {
                    let (ours, theirs) = (checked.next()?, unchecked.next()?);
                    if ours.1 != theirs.1 {
                        return None;
                    }
                    break Some(read(ours)?);
                }
                (Some(ours), theirs) if theirs.is_none_or(|theirs| ours.0 < theirs.0) => {
                    only_checked.push(read(checked.next()?)?);
                }
                _ => only_unchecked.push(read(unchecked.next()?)?),
            }
        };
        if !(only_checked.is_empty() && only_unchecked.is_empty()) {
            if open.iter().any(|delimiter| *delimiter != b'{')
                || !balanced(&only_checked)
                || !balanced(&only_unchecked)
            {
                return None;
            }
            let punctuation =
                |spelling: &str| Token { kind: TokenKind::Punctuation, spelling: spelling.into() };
            merged.push(build_flavor_keyword());
            for block in [only_checked, only_unchecked] {
                merged.push(punctuation("{"));
                merged.extend(block);
                merged.push(punctuation("}"));
            }
        }
        let Some(token) = shared else { break };
        if !track(&mut open, &token) {
            return None;
        }
        merged.push(token);
    }
    open.is_empty().then_some(merged)
}

/// Whether `tokens` close every delimiter they open, in order, and no other.
fn balanced(tokens: &[Token]) -> bool {
    let mut open = Vec::new();
    tokens.iter().all(|token| track(&mut open, token)) && open.is_empty()
}

/// Follow one token's effect on the `open` delimiters. `false` when it closes a delimiter
/// that is not the last one opened, or is a digraph, which the statement parser does not
/// read as the delimiter it stands for.
fn track(open: &mut Vec<u8>, token: &Token) -> bool {
    if token.kind != TokenKind::Punctuation {
        return true;
    }
    match token.spelling.as_str() {
        "(" => open.push(b'('),
        "[" => open.push(b'['),
        "{" => open.push(b'{'),
        ")" => return open.pop() == Some(b'('),
        "]" => return open.pop() == Some(b'['),
        "}" => return open.pop() == Some(b'{'),
        "<:" | ":>" | "<%" | "%>" | "%:" | "%:%:" => return false,
        _ => {}
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{BUILD_FLAVOR, Located, merge};
    use crate::{Token, TokenKind};

    /// The two flavors of one source text. Words are tokens at their offsets; a word that
    /// starts with `+` belongs to the assert-enabled flavor only and one that starts with
    /// `-` to the release flavor only.
    fn flavors(text: &str) -> (Vec<Located>, Vec<Located>) {
        let (mut checked, mut unchecked) = (Vec::new(), Vec::new());
        let mut offset = 0;
        for word in text.split(' ') {
            let (spelling, flavor) = match word.split_at_checked(1) {
                Some((flavor @ ("+" | "-"), spelling)) if !spelling.is_empty() => {
                    (spelling, Some(flavor))
                }
                _ => (word, None),
            };
            let kind = if spelling.starts_with(|first: char| first.is_ascii_alphabetic()) {
                TokenKind::Identifier
            } else {
                TokenKind::Punctuation
            };
            let token = (offset, Token { kind, spelling: spelling.into() });
            if flavor != Some("-") {
                checked.push(token.clone());
            }
            if flavor != Some("+") {
                unchecked.push(token);
            }
            offset += u32::try_from(word.len()).unwrap() + 1;
        }
        (checked, unchecked)
    }

    /// The merged body of a source text, spelled with spaces.
    fn merged(text: &str) -> Option<String> {
        let (checked, unchecked) = flavors(text);
        let tokens = merge(&checked, &unchecked)?;
        assert!(tokens.iter().all(|token| {
            (token.spelling == BUILD_FLAVOR) == (token.kind == TokenKind::Keyword)
        }));
        let spellings = tokens.iter().map(|token| token.spelling.as_str()).collect::<Vec<_>>();
        Some(spellings.join(" ").replace(BUILD_FLAVOR, "FLAVOR"))
    }

    /// Tokens that only one flavor has become one statement between the tokens both have,
    /// and tokens of both flavors between the same two share it.
    #[test]
    fn tokens_of_one_flavor_become_a_statement() {
        assert_eq!(merged("a ; return a ;").as_deref(), Some("a ; return a ;"));
        assert_eq!(
            merged("a ; +check +( +a +) +; return a ;").as_deref(),
            Some("a ; FLAVOR { check ( a ) ; } { } return a ;")
        );
        assert_eq!(
            merged("-plain -( -) -; a ;").as_deref(),
            Some("FLAVOR { } { plain ( ) ; } a ;")
        );
        assert_eq!(
            merged("{ +return +a +; -return -b -; }").as_deref(),
            Some("{ FLAVOR { return a ; } { return b ; } }")
        );
        // Each flavor reads its own tokens in order, whatever lies between them in the file.
        assert_eq!(
            merged("+{ -b -; +a +; -c -; +}").as_deref(),
            Some("FLAVOR { { a ; } } { b ; c ; }")
        );
        assert_eq!(merged("a ; +b +;").as_deref(), Some("a ; FLAVOR { b ; } { }"));
    }

    /// The braces written around one flavor's tokens must pair with each other.
    #[test]
    fn unbalanced_tokens_are_refused() {
        assert_eq!(merged("if ( a ) { b ; +} +else +{ c ; }"), None);
        assert_eq!(merged("{ a ; +} +{ b ; }"), None);
        assert_eq!(merged("a ; +( b ;"), None);
        assert_eq!(merged("a ; +{ +( +} +) ;"), None);
        assert_eq!(merged("a ; +<% +%> b ;"), None);
        assert_eq!(merged("{ a ; ) }"), None);
        assert_eq!(merged("{ a ;"), None);
    }

    /// A statement cannot sit inside parentheses or brackets, where it could also be part
    /// of a macro invocation's arguments.
    #[test]
    fn tokens_inside_an_expression_are_refused() {
        assert_eq!(merged("f ( a +, +b ) ;"), None);
        assert_eq!(merged("a [ +b -c ] ;"), None);
        assert_eq!(merged("( { +a +; } ) ;"), None);
        assert_eq!(merged("{ +a +; } ( b ) ;").as_deref(), Some("{ FLAVOR { a ; } { } } ( b ) ;"));
    }

    /// Bodies that are not two readings of one text do not align.
    #[test]
    fn misaligned_bodies_are_refused() {
        let token = |offset, spelling: &str| {
            (offset, Token { kind: TokenKind::Identifier, spelling: spelling.into() })
        };
        // One offset, two tokens.
        assert_eq!(merge(&[token(0, "a")], &[token(0, "b")]), None);
        let keyword = (0, Token { kind: TokenKind::Keyword, spelling: "a".into() });
        assert_eq!(merge(&[token(0, "a")], &[keyword]), None);
        // A token that starts inside another.
        assert_eq!(merge(&[token(0, "abc")], &[token(2, "c")]), None);
        assert_eq!(merge(&[token(4, "a"), token(0, "b")], &[]), None);
        // The statement's own keyword.
        assert_eq!(merge(&[token(0, BUILD_FLAVOR)], &[token(0, BUILD_FLAVOR)]), None);
        assert_eq!(merge(&[token(0, BUILD_FLAVOR)], &[]), None);
        assert!(merge(&[token(0, "abc"), token(3, "c")], &[token(3, "c")]).is_some());
    }
}
