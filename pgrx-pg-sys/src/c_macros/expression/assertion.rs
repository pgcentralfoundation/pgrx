//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Stop the process where translated C stops it, without a native call.
//!
//! An assert-enabled PostgreSQL reports a failed `Assert` through `ExceptionalCondition`,
//! which writes a `TRAP:` line and a backtrace to stderr and calls `abort()`. Translated
//! C reports the same failure here, so a failed assertion stops the backend whether its
//! function runs natively or as a translation. Generated macros call these functions only
//! where the C reaches them, and reach an assertion's report only against a PostgreSQL
//! built with assertions.
//!
//! The file and line are those of the Rust code that used the macro. Each function tracks
//! its caller, so a translated inline function that is itself called as a Rust function
//! reports that call.
//!
//! These functions are safe: they read only their arguments, and aborting the process is
//! defined behavior on any thread. They never return or unwind, so a failure cannot become a
//! recoverable PostgreSQL `ERROR`.
//!
//! They are typed `()`, not `!`, because generated macros keep C's expression shapes, such as
//! `AssertMacro`'s `(failure, 0)`. A `!` call there would make rustc report the rest of the
//! expansion as unreachable code at every caller of the macro.

use core::panic::Location;
use std::io::Write;

/// Report a failed C assertion as PostgreSQL's `ExceptionalCondition` does, then abort.
///
/// PostgreSQL 15 also reports an error type, such as `FailedAssertion`; later versions
/// have none. An error type that a macro's caller supplies arrives as that operand's
/// tokens, so a string literal is reported without its quotes, as C reports its value.
#[cold]
#[inline(never)]
#[track_caller]
pub fn assertion_failure(condition: &str, error_type: Option<&str>) {
    let location = Location::caller();
    let (file, line) = (location.file(), location.line());
    let pid = std::process::id();
    let mut stderr = std::io::stderr().lock();
    // Like PostgreSQL, ignore write errors: the process aborts either way.
    let _ = match error_type.map(literal_text) {
        Some(error_type) => writeln!(
            stderr,
            "TRAP: {error_type}(\"{condition}\", File: \"{file}\", Line: {line}, PID: {pid})"
        ),
        None => writeln!(
            stderr,
            "TRAP: failed Assert(\"{condition}\"), File: \"{file}\", Line: {line}, PID: {pid}"
        ),
    };
    let _ = writeln!(stderr, "{}", std::backtrace::Backtrace::force_capture());
    drop(stderr);
    std::process::abort()
}

/// The text of a Rust string or C string literal's tokens, or the tokens themselves.
fn literal_text(tokens: &str) -> &str {
    let literal = tokens.strip_prefix('c').unwrap_or(tokens);
    literal.strip_prefix('"').and_then(|text| text.strip_suffix('"')).unwrap_or(tokens)
}

/// Stop at C code that its compiler may assume is unreachable, such as release
/// PostgreSQL's `pg_unreachable()`. Reaching it is undefined in C, so stopping is
/// always a valid translation.
#[cold]
#[inline(never)]
#[track_caller]
pub fn unreachable_reached() {
    let location = Location::caller();
    let pid = std::process::id();
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(
        stderr,
        "TRAP: reached unreachable C code, File: \"{}\", Line: {}, PID: {pid}",
        location.file(),
        location.line()
    );
    let _ = writeln!(stderr, "{}", std::backtrace::Backtrace::force_capture());
    drop(stderr);
    std::process::abort()
}

/// C's `abort()`, which assert-enabled PostgreSQL uses for `pg_unreachable()`.
#[cold]
#[inline(never)]
pub fn abort() {
    std::process::abort()
}

#[cfg(test)]
mod tests {
    use super::literal_text;

    /// A caller's literal error type is reported as C reports the string it names.
    #[test]
    fn literal_error_types_lose_their_quotes() {
        assert_eq!(literal_text("\"BadState\""), "BadState");
        assert_eq!(literal_text("c\"BadState\""), "BadState");
        assert_eq!(literal_text("kind"), "kind");
        assert_eq!(literal_text("\"unterminated"), "\"unterminated");
    }
}
