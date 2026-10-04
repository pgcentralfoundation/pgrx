//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use pgrx_c_macros::format_rust_macros;
use proc_macro2::{Delimiter, Spacing, TokenStream, TokenTree};

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Group(Delimiter, Vec<Token>),
    Ident(String),
    Literal(String),
    Punctuation(String),
    Lifetime(String),
}

fn tokens(source: &str) -> Vec<Token> {
    fn normalize(stream: TokenStream) -> Vec<Token> {
        // Joint spacing after commas, '$', and other nonoperators can change
        // without changing Rust syntax. Compound operators and lifetimes keep
        // their lexical meaning, so those joints remain part of this comparison.
        const OPERATORS: &[&str] = &[
            "::", "->", "=>", "==", "!=", "<=", ">=", "<<", ">>", "&&", "||", "+=", "-=", "*=",
            "/=", "%=", "^=", "&=", "|=", "<<=", ">>=", "..", "...", "..=",
        ];
        let mut input = stream.into_iter().peekable();
        let mut result = Vec::new();
        while let Some(token) = input.next() {
            result.push(match token {
                TokenTree::Group(group) => {
                    Token::Group(group.delimiter(), normalize(group.stream()))
                }
                TokenTree::Ident(ident) => Token::Ident(ident.to_string()),
                TokenTree::Literal(literal) => Token::Literal(literal.to_string()),
                TokenTree::Punct(punct) => {
                    if punct.as_char() == '\''
                        && punct.spacing() == Spacing::Joint
                        && let Some(TokenTree::Ident(ident)) = input.peek()
                    {
                        let lifetime = Token::Lifetime(ident.to_string());
                        input.next();
                        result.push(lifetime);
                        continue;
                    }
                    let mut symbol = punct.as_char().to_string();
                    let mut joint = punct.spacing() == Spacing::Joint;
                    while joint {
                        let Some(TokenTree::Punct(next)) = input.peek() else { break };
                        let combined = format!("{symbol}{}", next.as_char());
                        if !OPERATORS.iter().any(|operator| operator.starts_with(&combined)) {
                            break;
                        }
                        symbol = combined;
                        joint = next.spacing() == Spacing::Joint;
                        input.next();
                    }
                    Token::Punctuation(symbol)
                }
            });
        }
        result
    }
    normalize(source.parse().expect("formatting fixture must tokenize"))
}

fn formatted(source: &str) -> String {
    let output = format_rust_macros(source).expect("valid macro definitions must format");
    assert_eq!(tokens(&output), tokens(source), "layout must preserve token and operator meaning");
    assert_eq!(format_rust_macros(&output).unwrap(), output, "layout must be idempotent");
    output
}

#[test]
fn qualified_calls_generics_and_control_flow_gain_indented_multiline_layout() {
    let prefix = "// outside αβ\npub const BEFORE : u32= 0xDEAD_BEEF;\n\n";
    let suffix = "\n\npub fn untouched( value:u32)->u32 {value+1}\n";
    let source = format!(
        "{prefix}macro_rules! ACL_GRANT_OPTION_FOR {{ ($privs:expr $(,)?) => {{ if $crate::__pgrx_c_macros::expression::truth($crate::__pgrx_c_macros::expression::compare::<$crate::AclMode, _>($privs, 0xFFFFFFFFu32)) {{ $crate::__pgrx_c_macros::expression::cast::<$crate::AclMode, _>($crate::__pgrx_c_macros::expression::choose::<Option<Result<&'static [u8], &'context str>>, _>($privs, $crate::__pgrx_c_macros::expression::literal(0xAB_CD))) }} else {{ {{ let result = $privs; $crate::__pgrx_c_macros::expression::finish(result) }} }} }}; }}{suffix}"
    );
    let output = formatted(&source);
    assert!(
        output.starts_with(prefix) && output.ends_with(suffix),
        "nonmacro bytes must remain exact"
    );
    for spelling in [
        "ACL_GRANT_OPTION_FOR",
        "$crate",
        "$privs",
        "0xFFFFFFFFu32",
        "0xAB_CD",
        "'static",
        "'context",
    ] {
        assert!(output.contains(spelling), "preserve original source spelling {spelling}");
    }
    assert!(output.lines().count() > 12, "nested calls must break across readable lines: {output}");
    for indent in [8, 12, 16] {
        assert!(
            output
                .lines()
                .any(|line| line.starts_with(&" ".repeat(indent)) && !line.trim().is_empty()),
            "nested bodies and arguments need indentation depth {indent}: {output}"
        );
    }
    assert!(
        output.lines().all(|line| line.chars().count() <= 100),
        "these fixtures have no indivisible overlong tokens: {output}"
    );
}

#[test]
fn repetitions_raw_context_modes_and_operators_retain_their_meaning() {
    let source = r#"
macro_rules! PROTOCOL { (@__pgrx_c_discard; $($raw:tt)*) => { $crate::PROTOCOL!(@__pgrx_emit_discard; $($raw)*) }; (@__pgrx_emit_discard; $($value:expr),* $(,)?) => {{ $(let _ = $value;)* }}; ($privs:expr, $count:expr) => {{ let mut value = $privs; value <<= $count; value >>= 1; value += 3; if value >= 7 && value != 0 || value <= 2 { value - 1 } else { value } }}; }
"#;
    let output = formatted(source);
    for spelling in [
        "@__pgrx_c_discard",
        "@__pgrx_emit_discard",
        "$raw",
        "$value",
        "$privs",
        "<<=",
        ">>=",
        "&&",
        "||",
        ">=",
        "!=",
        "<=",
    ] {
        assert!(output.contains(spelling), "preserve macro DSL and operators: {spelling}");
    }
    assert_ne!(tokens("left << right"), tokens("left < < right"));
    assert_ne!(tokens("left >= right"), tokens("left > = right"));
    assert_eq!(tokens("call::<T,>()"), tokens("call::<T, >()"));
}

#[test]
fn source_comments_and_opaque_literals_are_preserved_byte_for_byte() {
    const DOC: &str = "/// C macro SOURCE from header.h:17\n///\n/// ```text\n/// #define SOURCE(x) ((x) + 0xFFFFFFFF)\n/// ```\n";
    const BLOCK: &str = "/* PGRX: compiler resolved λ because binding was absent;\n   nested /* annotation #//! { ) ] */ remains literal. */";
    const LINE: &str = "// ordinary αβ comment with #//! and unmatched } ) ]";
    const RAW: &str = r####"r###"raw }; ] ) #//! "quotes" λ
still raw with a backslash \ and opening ( { ["###"####;
    const BYTES: &str = r####"br##"bytes }; ] ) #//! "quotes" \x23"##"####;
    let source = [
        DOC,
        "#[macro_export]\nmacro_rules! SOURCE { ($privs:expr) => {{ let raw = ",
        RAW,
        "; let bytes = ",
        BYTES,
        "; let chars = ('λ', '}', b'#', b'\\x7D'); ",
        BLOCK,
        "\n",
        LINE,
        "\n let number = 0xFFFFFFFFu32; let _ = (raw, bytes, chars, number, $privs); }}; }\n",
    ]
    .concat();
    let output = formatted(&source);
    for exact in [DOC, BLOCK, LINE, RAW, BYTES, "'λ'", "'}'", "b'#'", "b'\\x7D'", "0xFFFFFFFFu32"]
    {
        assert_eq!(output.matches(exact).count(), 1, "retain exact comment/literal: {exact}");
    }
}

#[test]
fn doc_comments_inside_macro_groups_preserve_overlapping_synthetic_spans() {
    const OUTER: &str = "/// outer docs λ with unmatched } ) ] #//!";
    const BLOCK: &str = "/** block docs λ with nested /* ordinary } ) #//! */ text */";
    const INNER: &str = "//! inner docs αβ with unmatched } ) ]";
    let source = format!(
        "macro_rules! DOCUMENTED {{ () => {{ {OUTER}\n fn first() -> u32 {{ 0xFFFF }} {BLOCK}\n fn second() -> u32 {{ 7 }} mod nested {{ {INNER}\n pub const VALUE: u32 = 11; }} }}; }}"
    );
    let output = formatted(&source);
    // The lexer synthesizes #[doc] groups whose spans overlap the originating
    // comments. The formatter must preserve the source once rather than copying
    // each synthetic token's span or printing synthetic attributes.
    for exact in [OUTER, BLOCK, INNER] {
        assert_eq!(output.matches(exact).count(), 1, "preserve each doc comment once: {output}");
    }
    for line in [OUTER, INNER] {
        assert!(
            output.split_once(line).unwrap().1.starts_with('\n'),
            "line docs must terminate before the next item: {output}"
        );
    }
    assert!(!output.contains("#[doc"), "source comments must remain comments: {output}");
    for item in ["fn first", "fn second", "mod nested", "pub const VALUE"] {
        assert!(
            output.contains(item),
            "following item must remain outside its doc comment: {output}"
        );
    }
}

#[test]
fn comments_in_empty_groups_and_after_tokens_cannot_swallow_delimiters() {
    const BRACE: &str = "// brace-only λ with } ) ] #//!";
    const PAREN: &str = "// paren-only αβ with } ) ]";
    const BRACKET: &str = "// bracket-only λ with } ) ]";
    const TRAILING: &str = "// trailing comment λ with } ) ]";
    let source = format!(
        "macro_rules! COMMENT_GROUPS {{ () => {{ {BRACE}\n }}; (@paren) => {{ ( {PAREN}\n ) }}; (@bracket) => {{ [ {BRACKET}\n ] }}; ($value:expr) => {{ $crate::helper($value) {TRAILING}\n }}; }}"
    );
    let output = formatted(&source);
    for line in [BRACE, PAREN, BRACKET, TRAILING] {
        assert_eq!(output.matches(line).count(), 1, "retain each line comment once: {output}");
        assert!(
            output.split_once(line).unwrap().1.starts_with('\n'),
            "a closing delimiter must not join the physical line comment: {output}"
        );
    }
}

#[test]
fn trailing_mixed_block_and_line_comments_keep_group_delimiters_visible() {
    const EMPTY_BLOCK: &str = "/* empty block λ */";
    const EMPTY_LINE: &str = "// empty line αβ";
    const VALUE_BLOCK: &str = "/* value block αβ */";
    const VALUE_LINE: &str = "// value line λ";
    let source = format!(
        "macro_rules! MIXED_COMMENTS {{ () => {{ ({EMPTY_BLOCK} {EMPTY_LINE}\n) }}; (@value) => {{ (1 {VALUE_BLOCK} {VALUE_LINE}\n) }}; }}"
    );
    let output = formatted(&source);
    for exact in [EMPTY_BLOCK, EMPTY_LINE, VALUE_BLOCK, VALUE_LINE] {
        assert_eq!(output.matches(exact).count(), 1, "preserve each mixed comment once: {output}");
    }
    for line in [EMPTY_LINE, VALUE_LINE] {
        assert!(
            output.split_once(line).unwrap().1.starts_with('\n'),
            "the closing parenthesis must stay outside the trailing line comment: {output}"
        );
    }
}

#[test]
fn trailing_doc_and_ordinary_comment_whitespace_remains_exact() {
    const DOC: &str = "/// documentation with a hard line break λ  ";
    const SPACES: &str = "// ordinary trailing spaces αβ  ";
    const TABS: &str = "// ordinary trailing tabs λ\t\t";
    let source = format!(
        "macro_rules! COMMENT_WHITESPACE {{ () => {{ {DOC}\n fn documented() -> u32 {{ 1 }} {SPACES}\n {TABS}\n documented() }}; }}"
    );
    let output = formatted(&source);
    // Doc whitespace is part of the synthesized attribute literal; ordinary
    // comment whitespace also belongs to the original source, despite lexing
    // without tokens.
    for exact in [DOC, SPACES, TABS] {
        assert_eq!(output.matches(exact).count(), 1, "retain trailing comment bytes: {output:?}");
        assert!(
            output.split_once(exact).unwrap().1.starts_with('\n'),
            "the complete comment must end at its original physical newline: {output:?}"
        );
    }
}

#[test]
fn indivisible_long_literals_identifiers_and_comments_may_exceed_width() {
    let identifier = format!("long_{}", "identifier".repeat(15));
    let literal = format!("r##\"{} #//! () [] {{}} λ\"##", "opaque".repeat(25));
    let comment = format!("/* PGRX: {} */", "indivisible annotation ".repeat(8));
    let source = format!(
        "macro_rules! LONG {{ () => {{{{ {comment} let {identifier} = {literal}; {identifier} }}}}; }}"
    );
    let output = formatted(&source);
    for exact in [&identifier, &literal, &comment] {
        assert!(output.contains(exact), "indivisible text must stay exact");
    }
    assert!(output.lines().any(|line| line.chars().count() > 100));
    for line in output.lines().filter(|line| line.chars().count() > 100) {
        assert!(
            line.contains(&identifier) || line.contains(&literal) || line.contains(&comment),
            "only indivisible source atoms justify an overlong line: {line}"
        );
    }
}

#[test]
fn nested_module_macro_definitions_preserve_the_surrounding_source() {
    let prefix = "pub mod nested {\n    pub const BEFORE :u32= 2;\n    ";
    let suffix = "\n    pub const AFTER :u32= 3;\n}\n";
    let source = format!(
        "{prefix}macro_rules! INNER {{ ($value:expr) => {{ $crate::helper($crate::other($value, 1), 2) }}; }}{suffix}"
    );
    let output = formatted(&source);
    assert!(output.starts_with(prefix) && output.ends_with(suffix));
    assert!(
        output.lines().count() > source.lines().count(),
        "nested definitions must receive layout too: {output}"
    );
}

#[test]
fn files_without_definitions_are_untouched_and_malformed_tokens_are_errors() {
    let source = "// macro_rules! imaginary {\nconst TEXT :&str= \"macro_rules! fake { unmatched\";\nfn unchanged( ){ordinary!( a ,b );}\n";
    assert_eq!(format_rust_macros(source).unwrap(), source);
    for malformed in [
        "macro_rules! BROKEN { () => { call() };",
        "macro_rules! BROKEN { () => { r#\"unterminated }; }",
        "macro_rules! BROKEN { () => { /* unterminated }; }",
    ] {
        let error = format_rust_macros(malformed).expect_err("malformed input must be rejected");
        assert!(!error.is_empty());
    }
}
