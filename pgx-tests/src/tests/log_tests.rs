// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_info() {
        info!("info message");
    }

    #[pg_test]
    fn test_log() {
        log!("log message");
    }

    #[pg_test]
    fn test_warn() {
        warning!("warn message");
    }

    #[pg_test]
    fn test_notice() {
        notice!("notice message");
    }

    #[pg_test]
    fn test_debug5() {
        debug5!("debug5 message");
    }

    #[pg_test]
    fn test_debug4() {
        debug4!("debug4 message");
    }

    #[pg_test]
    fn test_debug3() {
        debug3!("debug3 message");
    }

    #[pg_test]
    fn test_debug2() {
        debug2!("debug2 message");
    }

    #[pg_test]
    fn test_debug1() {
        debug1!("debug1 message");
    }

    #[pg_test(error = "error message")]
    fn test_error() {
        error!("error message");
    }

    #[pg_test]
    fn test_check_for_interrupts() {
        check_for_interrupts!();
    }

    #[pg_test(error = "ereport error")]
    fn test_ereport() {
        ereport(
            PgLogLevel::ERROR,
            PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
            "ereport error",
            file!(),
            line!(),
            column!(),
        )
    }

    #[pg_test(error = "panic message")]
    fn test_panic() {
        panic!("panic message")
    }
}
