//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Invocation checks for C statement macros with introduced local names.

const MAX_ARGUMENT_BYTES: usize = 4096;
const MAX_LOCALS: usize = 64;

/// Check stringified argument tokens for names that C substitution could capture.
///
/// Stringification retains identifiers inside forwarded Rust expression fragments,
/// which ordinary `macro_rules!` token matching cannot inspect. The generated
/// macro evaluates this check in a constant assertion before accepting an
/// invocation. Fields, paths, and string contents with the same spelling are
/// conservatively rejected too. Inputs exceeding either bound are rejected.
///
/// Non-ASCII bytes remain part of an identifier boundary, so an ASCII local name
/// inside a longer Unicode identifier does not match. Scanning complete words
/// avoids repeatedly comparing overlapping substrings; work is bounded by the
/// argument length times the number of locals, without allocations.
pub const fn local_scope_allowed(source: &str, locals: &[&str]) -> bool {
    if source.len() > MAX_ARGUMENT_BYTES || locals.len() > MAX_LOCALS {
        return false;
    }
    let source = source.as_bytes();
    let mut position = 0;
    while position < source.len() {
        if !identifier_byte(source[position]) {
            position += 1;
            continue;
        }
        let start = position;
        while position < source.len() && identifier_byte(source[position]) {
            position += 1;
        }
        let length = position - start;
        let mut local = 0;
        while local < locals.len() {
            let name = locals[local].as_bytes();
            if name.len() == length {
                let mut matched = 0;
                while matched < length && source[start + matched] == name[matched] {
                    matched += 1;
                }
                if matched == length {
                    return false;
                }
            }
            local += 1;
        }
    }
    true
}

const fn identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte >= 0x80
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_complete_local_names_even_inside_groups_and_raw_identifiers() {
        for source in ["rsi", "(rsi)", "[(rsi)]", "{ rsi }", "r#rsi", "value.rsi", "path::rsi"] {
            assert!(!local_scope_allowed(source, &["rsi"]), "{source}");
        }
        for source in ["rsi_next", "next_rsi", "rsi2", "αrsi", "rsiα", "result"] {
            assert!(local_scope_allowed(source, &["rsi"]), "{source}");
        }
        assert!(!local_scope_allowed("result", &["rsi", "result"]));
        assert!(!local_scope_allowed("αrsi", &["αrsi"]));
    }

    #[test]
    fn string_contents_are_rejected_conservatively() {
        assert!(!local_scope_allowed(r#""rsi""#, &["rsi"]));
        assert!(!local_scope_allowed(r##"r#"rsi"#"##, &["rsi"]));
        assert!(local_scope_allowed(r#""rsi_next""#, &["rsi"]));
    }

    #[test]
    fn stringification_reveals_forwarded_expression_identifiers() {
        macro_rules! check {
            ($($tokens:tt)*) => {
                local_scope_allowed(stringify!($($tokens)*), &["rsi"])
            };
        }
        macro_rules! forward {
            ($expression:expr) => {
                check!($expression)
            };
        }
        let allowed = const { forward!(result) };
        let rejected = const { forward!((rsi)) };
        assert!(allowed);
        assert!(!rejected);
    }

    #[test]
    fn oversized_inputs_and_local_sets_are_rejected() {
        assert!(local_scope_allowed(&" ".repeat(MAX_ARGUMENT_BYTES), &["rsi"]));
        assert!(!local_scope_allowed(&" ".repeat(MAX_ARGUMENT_BYTES + 1), &["rsi"]));
        assert!(local_scope_allowed("result", &["rsi"; MAX_LOCALS]));
        assert!(!local_scope_allowed("result", &["rsi"; MAX_LOCALS + 1]));
        assert!(local_scope_allowed("", &[]));
    }
}
