//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Stop the process where translated C stops it, without a native call.
//!
//! An assert-enabled PostgreSQL reports a failed `Assert` through `ExceptionalCondition`,
//! which writes a `TRAP:` line and a backtrace to stderr and calls `abort()`. Translated
//! C reports the same failure here, so a failed assertion stops the backend whether its
//! function runs natively or as a translation. The file and line are the Rust invocation
//! that reached the check, as for other invocation diagnostics.
//!
//! These functions are safe: they read only their arguments, and aborting the process is
//! defined behavior on any thread. They never return or unwind, so a failure cannot become a
//! recoverable PostgreSQL `ERROR`.
//!
//! They are typed `()`, not `!`, because generated macros keep C's expression shapes, such as
//! `AssertMacro`'s `(failure, 0)`. A `!` call there would make rustc report the rest of the
//! expansion as unreachable code at every caller of the macro.

use std::io::Write;

/// Report a failed C assertion as PostgreSQL's `ExceptionalCondition` does, then abort.
///
/// PostgreSQL 15 also reports an error type, such as `FailedAssertion`; later versions
/// have none.
#[cold]
#[inline(never)]
pub fn assertion_failure(condition: &str, error_type: Option<&str>, file: &str, line: u32) {
    let pid = std::process::id();
    let mut stderr = std::io::stderr().lock();
    // Like PostgreSQL, ignore write errors: the process aborts either way.
    let _ = match error_type {
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

/// Stop at C code that its compiler may assume is unreachable, such as release
/// PostgreSQL's `pg_unreachable()`. Reaching it is undefined in C, so stopping is
/// always a valid translation.
#[cold]
#[inline(never)]
pub fn unreachable_reached(file: &str, line: u32) {
    let pid = std::process::id();
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(
        stderr,
        "TRAP: reached unreachable C code, File: \"{file}\", Line: {line}, PID: {pid}"
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
