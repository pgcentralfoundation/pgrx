//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Generated support identifiers derived from the C names they adapt.
//!
//! Support items are named after C declarations rather than hashes or catalog
//! positions, so regenerating with unrelated header changes keeps every other name.
//! A name with several parts joins them with `__`. A part that is not a plain C
//! identifier is hex-encoded behind a leading digit, so each joined name still
//! identifies exactly one sequence of parts.

use std::fmt::Write;

/// Join C identifier parts into one Rust and C identifier suffix.
///
/// A plain identifier has no leading, trailing, or doubled underscore, so `__` can
/// only be a separator. Any other part is written as `0` followed by its hex bytes;
/// no C identifier starts with a digit.
pub(super) fn join(parts: &[&str]) -> String {
    let mut joined = String::new();
    for (index, part) in parts.iter().enumerate() {
        if index != 0 {
            joined.push_str("__");
        }
        if is_plain(part) {
            joined.push_str(part);
        } else {
            joined.push('0');
            push_hex(&mut joined, part);
        }
    }
    joined
}

/// Name a native helper by role, keeping roles and parts unambiguous.
///
/// The role must not contain `__`, so the first separator after the prefix ends it.
pub(super) fn helper(prefix: &str, role: &str, parts: &[&str]) -> String {
    debug_assert!(!role.contains("__") && !role.is_empty());
    format!("{prefix}__{role}__{}", join(parts))
}

/// Encode a compiler type spelling such as `int (const void *)` as an identifier suffix.
///
/// Tokens are joined by `__`. Plain identifiers and decimal numbers keep their
/// spelling. Punctuation becomes a two-character code starting with `0`; any other
/// identifier is `0x` plus hex bytes and any other character is `0u` plus hex bytes.
/// Directories in anonymous type locations are dropped up to PostgreSQL's `server`
/// include root, so the result does not depend on where headers are installed.
pub(super) fn type_spelling(spelling: &str) -> String {
    let spelling = strip_location_directories(spelling);
    let mut tokens = Vec::new();
    let mut characters = spelling.char_indices().peekable();
    while let Some((start, character)) = characters.next() {
        if character.is_whitespace() {
            continue;
        }
        let mut end = start + character.len_utf8();
        if character == '_' || character.is_ascii_alphabetic() {
            while let Some(&(index, next)) = characters.peek() {
                if next != '_' && !next.is_ascii_alphanumeric() {
                    break;
                }
                end = index + next.len_utf8();
                characters.next();
            }
            let identifier = &spelling[start..end];
            if is_plain(identifier) {
                tokens.push(identifier.to_owned());
            } else {
                let mut encoded = String::from("0x");
                push_hex(&mut encoded, identifier);
                tokens.push(encoded);
            }
        } else if character.is_ascii_digit() {
            while let Some(&(index, next)) = characters.peek() {
                if !next.is_ascii_digit() {
                    break;
                }
                end = index + next.len_utf8();
                characters.next();
            }
            tokens.push(spelling[start..end].to_owned());
        } else if spelling[start..].starts_with("...") {
            characters.next();
            characters.next();
            tokens.push("0e".into());
        } else {
            tokens.push(match character {
                '*' => "0p".into(),
                '(' => "0l".into(),
                ')' => "0r".into(),
                ',' => "0c".into(),
                '[' => "0a".into(),
                ']' => "0z".into(),
                other => {
                    let mut encoded = String::from("0u");
                    push_hex(&mut encoded, &other.to_string());
                    encoded
                }
            });
        }
    }
    tokens.join("__")
}

/// Accept parts that are C identifiers with only single interior underscores.
fn is_plain(part: &str) -> bool {
    part.bytes().next().is_some_and(|byte| byte.is_ascii_alphabetic())
        && !part.ends_with('_')
        && !part.contains("__")
        && part.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

/// Append the lowercase hex bytes of a UTF-8 string.
fn push_hex(output: &mut String, text: &str) {
    for byte in text.bytes() {
        write!(output, "{byte:02x}").expect("String output");
    }
}

/// Replace the directory in each `(unnamed ... at PATH:LINE:COLUMN)` location.
///
/// Clang spells anonymous types with their absolute source location. Keep the path
/// below PostgreSQL's `server` include directory when present, otherwise the
/// filename, so names stay the same across installation prefixes.
fn strip_location_directories(spelling: &str) -> String {
    let mut output = String::with_capacity(spelling.len());
    let mut rest = spelling;
    while let Some(at) = rest.find(" at ") {
        let (before, after) = rest.split_at(at + 4);
        output.push_str(before);
        let end = after.find(')').unwrap_or(after.len());
        let location = &after[..end];
        let relative = location
            .rfind("/server/")
            .map(|index| &location[index + "/server/".len()..])
            .or_else(|| location.rfind(['/', '\\']).map(|index| &location[index + 1..]))
            .unwrap_or(location);
        output.push_str(relative);
        rest = &after[end..];
    }
    output.push_str(rest);
    output
}

/// Regression tests for readable, collision-free generated support names.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that plain identifiers stay readable and others cannot alias a separator.
    #[test]
    fn joined_parts_are_readable_and_unambiguous() {
        assert_eq!(join(&["ItemIdData", "lp_off"]), "ItemIdData__lp_off");
        assert_ne!(join(&["a__b", "c"]), join(&["a", "b__c"]));
        assert_ne!(join(&["a_", "b"]), join(&["a", "_b"]));
        assert_eq!(join(&["_x"]), "05f78");
        assert_eq!(helper("__pgrx_inline", "arg0", &["foo"]), "__pgrx_inline__arg0__foo");
        assert_ne!(
            helper("__pgrx_inline", "fn", &["arg0__foo"]),
            helper("__pgrx_inline", "arg0", &["foo"])
        );
    }

    /// Checks that type spellings keep identifiers and encode punctuation distinctly.
    #[test]
    fn type_spellings_name_c_function_types() {
        assert_eq!(
            type_spelling("unsigned long (struct FunctionCallInfoBaseData *)"),
            "unsigned__long__0l__struct__FunctionCallInfoBaseData__0p__0r"
        );
        assert_eq!(type_spelling("_Bool (int, ...)"), "0x5f426f6f6c__0l__int__0c__0e__0r");
        assert_eq!(type_spelling("enum NodeTag"), "enum__NodeTag");
        assert_ne!(type_spelling("void (int *)"), type_spelling("void (int) *"));
        assert_ne!(type_spelling("char [16]"), type_spelling("char [1] [6]"));
    }

    /// Checks that installation directories do not change anonymous type names.
    #[test]
    fn anonymous_type_locations_drop_installation_directories() {
        let first = "enum (unnamed at /opt/pg/include/postgresql/server/utils/rel.h:12:5)";
        let second = "enum (unnamed at /usr/include/postgresql/18/server/utils/rel.h:12:5)";
        assert_eq!(type_spelling(first), type_spelling(second));
        assert_ne!(
            type_spelling(first),
            type_spelling("enum (unnamed at /opt/pg/include/server/access/rel.h:12:5)")
        );
        assert_eq!(
            type_spelling("struct (anonymous at /usr/include/x86_64-linux-gnu/bits/types.h:3:1)"),
            type_spelling("struct (anonymous at /other/bits/types.h:3:1)")
        );
    }
}
