//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Turn C documentation comments into rustdoc that holds no Rust doctests.
//!
//! rustdoc compiles every indented code block and every fenced block without a language as
//! a Rust doctest. C headers' documentation comments are Doxygen, not Markdown, and their
//! examples are C: ICU's, which the Windows binaries' include tree contains, put them in
//! `\code` blocks and in indented paragraphs. So Doxygen code blocks become `text` fences,
//! fences without a language become `text` fences, and no line outside a fence keeps the
//! indentation that would make it code.

/// Rewrite one comment, already stripped of its C comment markers by bindgen.
pub(super) fn rustdoc(comment: &str) -> String {
    let mut lines = Vec::new();
    let mut block: Option<Block> = None;
    for line in comment.lines() {
        let trimmed = line.trim_start();
        match &mut block {
            None => {
                if let Some(first) = doxygen_start(trimmed) {
                    let first = first.trim();
                    block = Some(Block::Doxygen(
                        (!first.is_empty()).then(|| first.to_owned()).into_iter().collect(),
                    ));
                } else if let Some((fence, info)) = fence_open(trimmed) {
                    let info = if rust_info(info) { "text" } else { info };
                    lines.push(format!("{fence}{info}"));
                    block = Some(Block::Fenced(fence.to_owned()));
                } else {
                    lines.push(trimmed.to_owned());
                }
            }
            Some(Block::Doxygen(code)) => {
                if doxygen_end(trimmed) {
                    let code = std::mem::take(code);
                    fence_text(&mut lines, &code);
                    block = None;
                } else {
                    code.push(line.to_owned());
                }
            }
            Some(Block::Fenced(fence)) => {
                let closes = trimmed.starts_with(fence.as_str())
                    && trimmed[fence.len()..]
                        .chars()
                        .all(|c| c == fence.as_bytes()[0] as char || c.is_whitespace());
                lines.push(line.to_owned());
                if closes {
                    block = None;
                }
            }
        }
    }
    match block {
        // An unterminated Doxygen block still holds code.
        Some(Block::Doxygen(code)) => fence_text(&mut lines, &code),
        // Markdown closes an unterminated fence at the end of the comment, and so does rustdoc.
        Some(Block::Fenced(_)) | None => {}
    }
    lines.join("\n")
}

enum Block {
    /// Lines of a Doxygen `\code` block, emitted once its end shows how long a fence it needs.
    Doxygen(Vec<String>),
    /// An open Markdown fence and its delimiter run.
    Fenced(String),
}

/// The code after a line's opening Doxygen `\code` or `@code`, with any `{.lang}` removed. A
/// line that also closes the block is inline code, so it opens none.
fn doxygen_start(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("\\code").or_else(|| line.strip_prefix("@code"))?;
    let rest = match rest.strip_prefix('{') {
        Some(language) => &language[language.find('}')? + 1..],
        None if rest.is_empty() || rest.starts_with(char::is_whitespace) => rest,
        None => return None,
    };
    (!rest.contains("\\endcode") && !rest.contains("@endcode")).then_some(rest)
}

fn doxygen_end(line: &str) -> bool {
    line.starts_with("\\endcode") || line.starts_with("@endcode")
}

/// A Markdown fence's delimiter run and info string.
fn fence_open(line: &str) -> Option<(&str, &str)> {
    let delimiter = line.chars().next().filter(|c| *c == '`' || *c == '~')?;
    let run = line.len() - line.trim_start_matches(delimiter).len();
    (run >= 3).then(|| (&line[..run], line[run..].trim()))
}

/// Whether rustdoc would test a fenced block with this info string.
fn rust_info(info: &str) -> bool {
    info.is_empty()
        || info.split([',', ' ']).filter(|word| !word.is_empty()).all(|word| {
            matches!(
                word,
                "rust" | "ignore" | "no_run" | "should_panic" | "compile_fail" | "test_harness"
            ) || word.starts_with("edition")
        })
}

/// Fence `code` as text, with a backtick run longer than any inside it.
fn fence_text(lines: &mut Vec<String>, code: &[String]) {
    let longest =
        code.iter().flat_map(|line| line.split(|c| c != '`')).map(str::len).max().unwrap_or(0);
    let fence = "`".repeat(longest.max(2) + 1);
    lines.push(format!("{fence}text"));
    lines.extend(code.iter().cloned());
    lines.push(fence);
}

#[cfg(test)]
mod tests {
    use super::rustdoc;

    #[test]
    fn doxygen_code_becomes_a_text_fence() {
        let comment = " Error codes.\n\n \\code\n     UErrorCode errorCode = U_ZERO_ERROR;\n     if (U_FAILURE(errorCode)) {\n     }\n \\endcode\n More text.";
        assert_eq!(
            rustdoc(comment),
            "Error codes.\n\n```text\n     UErrorCode errorCode = U_ZERO_ERROR;\n     if (U_FAILURE(errorCode)) {\n     }\n```\nMore text."
        );
    }

    #[test]
    fn indented_examples_lose_the_indentation_that_makes_them_code() {
        let comment = " Usage:\n\n     foo(bar);\n     baz();";
        assert_eq!(rustdoc(comment), "Usage:\n\nfoo(bar);\nbaz();");
    }

    #[test]
    fn fences_that_rustdoc_would_test_become_text() {
        assert_eq!(rustdoc(" ```\n  x = 1;\n ```"), "```text\n  x = 1;\n ```");
        assert_eq!(rustdoc("~~~~ rust,ignore\nx\n~~~~"), "~~~~text\nx\n~~~~");
        // Another language is never tested, so it stays.
        assert_eq!(rustdoc("```c\n  int x;\n```"), "```c\n  int x;\n```");
    }

    #[test]
    fn code_blocks_get_a_fence_longer_than_their_backticks() {
        assert_eq!(rustdoc("@code{.c}\na ```` b\n@endcode"), "`````text\na ```` b\n`````");
    }

    #[test]
    fn an_unterminated_doxygen_block_is_still_fenced() {
        assert_eq!(rustdoc("\\code\n  f();"), "```text\n  f();\n```");
    }

    #[test]
    fn doxygen_words_that_only_start_like_code_stay_text() {
        assert_eq!(rustdoc("\\codepoint value"), "\\codepoint value");
        assert_eq!(rustdoc("\\code f(); \\endcode"), "\\code f(); \\endcode");
    }

    #[test]
    fn code_on_the_opening_line_is_kept() {
        assert_eq!(rustdoc("\\code f();\n  g();\n\\endcode"), "```text\nf();\n  g();\n```");
    }
}
