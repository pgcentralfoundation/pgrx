//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use pgrx::prelude::*;
    use std::sync::atomic::{AtomicBool, Ordering};

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
        // SAFETY: pg_test runs on PostgreSQL's initialized backend thread;
        // ProcessInterrupts is called through its generated error guard.
        unsafe { pg_sys::CHECK_FOR_INTERRUPTS!() };
    }

    #[pg_test(error = "ereport error")]
    fn test_ereport() {
        pgrx::ereport!(PgLogLevel::ERROR, PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, "ereport error")
    }

    #[pg_test(error = "ereport error")]
    fn test_ereport_domain() {
        pgrx::ereport_domain!(
            PgLogLevel::ERROR,
            "test_extension_domain",
            PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
            "ereport error"
        )
    }

    #[pg_test]
    fn test_ereport_domain_value() {
        let caught = pgrx::PgTryBuilder::new(|| -> Option<pgrx::pg_sys::panic::CaughtError> {
            pgrx::ereport_domain!(
                PgLogLevel::ERROR,
                "test_extension_domain",
                PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                "ereport error"
            );
            None
        })
        .catch_when(PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, |e| Some(e))
        .execute();

        match caught {
            Some(pgrx::pg_sys::panic::CaughtError::ErrorReport(report))
            | Some(pgrx::pg_sys::panic::CaughtError::PostgresError(report)) => {
                assert_eq!(report.sql_error_code(), PgSqlErrorCode::ERRCODE_INTERNAL_ERROR);
                assert_eq!(report.message(), "ereport error");
                assert_eq!(report.domain(), Some("test_extension_domain"));
            }
            Some(other) => panic!("unexpected error kind: {other:?}"),
            None => panic!("expected error, but code returned normally"),
        }
    }

    #[pg_extern]
    fn raise_with_domain() {
        pgrx::ereport_domain!(
            PgLogLevel::ERROR,
            "test_extension_domain",
            PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
            "raised with a domain"
        )
    }

    #[pg_extern]
    fn raise_with_nul_in_domain() {
        pgrx::ereport_domain!(
            PgLogLevel::ERROR,
            "test_extension_domain\0ignored",
            PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
            "raised with a NUL in its domain"
        )
    }

    /// An error context callback that raises an ERROR with a domain.  `arg` points to its own
    /// `ErrorContextCallback`, which it takes off the stack so the ERROR doesn't call it again.
    #[pg_guard]
    unsafe extern "C-unwind" fn raise_with_domain_from_context_callback(
        arg: *mut core::ffi::c_void,
    ) {
        // SAFETY: the callback is on top of the stack, and `arg` points to it
        unsafe {
            pg_sys::error_context_stack = (*arg.cast::<pg_sys::ErrorContextCallback>()).previous
        };
        pgrx::ereport_domain!(
            PgLogLevel::ERROR,
            "test_extension_domain",
            PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
            "raised while Postgres reports another ERROR"
        )
    }

    /// Raises an ERROR whose report calls [`raise_with_domain_from_context_callback`].
    #[pg_extern]
    fn raise_with_domain_while_reporting() {
        // SAFETY: Postgres calls this function on the backend thread
        let previous = unsafe { pg_sys::error_context_stack };
        // The guard reports the ERROR below once this frame is gone, so the callback's entry
        // can't be a local.  It lasts for the transaction, and the callback takes it off the
        // stack.
        let callback = pgrx::PgMemoryContexts::TopTransactionContext.leak_and_drop_on_delete(
            pg_sys::ErrorContextCallback {
                previous,
                callback: Some(raise_with_domain_from_context_callback),
                arg: core::ptr::null_mut(),
            },
        );
        // SAFETY: `callback` points to the entry allocated above
        unsafe {
            (*callback).arg = callback.cast();
            pg_sys::error_context_stack = callback;
        }
        error!("reported with an error context callback that raises an ERROR");
    }

    /// Runs `sql`, which calls a function that raises an ERROR, and returns the domain Postgres
    /// has for that ERROR.  The function's guard reports the ERROR to Postgres, which jumps back
    /// to SPI's caller.
    fn domain_of_error_from(sql: &str) -> Option<String> {
        use pgrx::pg_sys::panic::CaughtError;

        pgrx::PgTryBuilder::new(|| {
            Spi::run(sql).expect("SPI failed");
            panic!("{sql} returned normally")
        })
        .catch_others(|e| match e {
            CaughtError::PostgresError(ereport) => ereport.domain().map(str::to_string),
            e => e.rethrow(),
        })
        .execute()
    }

    /// Postgres keeps the pointer to an ERROR's domain and reads it after the function that
    /// raised the ERROR is gone.
    #[pg_test]
    fn test_ereport_domain_reaches_postgres() {
        let domain = domain_of_error_from("SELECT tests.raise_with_domain()");
        assert_eq!(domain.as_deref(), Some("test_extension_domain"));
    }

    /// Postgres reads a domain as a C string, so it ends at its first NUL, like the message.
    #[pg_test]
    fn test_ereport_domain_with_nul() {
        let domain = domain_of_error_from("SELECT tests.raise_with_nul_in_domain()");
        assert_eq!(domain.as_deref(), Some("test_extension_domain"));
    }

    /// Postgres resets `ErrorContext` when it starts an ERROR that is raised while it reports
    /// another.  The new ERROR's domain is still there afterward.
    #[pg_test]
    fn test_ereport_domain_raised_while_reporting() {
        let domain = domain_of_error_from("SELECT tests.raise_with_domain_while_reporting()");
        assert_eq!(domain.as_deref(), Some("test_extension_domain"));
    }

    /// PostgreSQL spells each SQLSTATE that pgrx added after PostgreSQL 12 the way
    /// errcodes.txt does, and converting the code back returns the same variant rather than
    /// `ERRCODE_INTERNAL_ERROR`.
    #[pg_test]
    fn test_sql_error_codes_match_postgres_sqlstates() {
        for (sqlstate, code) in [
            ("10608", PgSqlErrorCode::ERRCODE_INVALID_ARGUMENT_FOR_XQUERY),
            ("22P07", PgSqlErrorCode::ERRCODE_DATUM_VERSION_MISMATCH),
            ("22031", PgSqlErrorCode::ERRCODE_INVALID_ARGUMENT_FOR_SQL_JSON_DATETIME_FUNCTION),
            ("2203G", PgSqlErrorCode::ERRCODE_SQL_JSON_ITEM_CANNOT_BE_CAST_TO_TARGET_TYPE),
            ("25P04", PgSqlErrorCode::ERRCODE_TRANSACTION_TIMEOUT),
            ("57P05", PgSqlErrorCode::ERRCODE_IDLE_SESSION_TIMEOUT),
            ("58P03", PgSqlErrorCode::ERRCODE_FILE_NAME_TOO_LONG),
        ] {
            // SAFETY: unpack_sql_state writes the five characters and a NUL to a static
            // buffer, which this reads before the next call.
            let unpacked =
                unsafe { core::ffi::CStr::from_ptr(pg_sys::unpack_sql_state(code as i32)) };
            assert_eq!(unpacked.to_str(), Ok(sqlstate));
            assert_eq!(PgSqlErrorCode::from(code as i32), code);
        }
    }

    #[pg_test(error = "panic message")]
    fn test_panic() {
        panic!("panic message")
    }

    static FORMAT_ARG_EVALUATED: AtomicBool = AtomicBool::new(false);

    fn evaluate_format_arg() -> &'static str {
        FORMAT_ARG_EVALUATED.store(true, Ordering::SeqCst);
        "evaluated"
    }

    // With default settings (log_min_messages=WARNING, client_min_messages=NOTICE),
    // DEBUG-level messages are not interesting, so we shouldn't be evaluating format arguments
    // eagerly.
    #[pg_test]
    fn test_debug_skips_arg_evaluation() {
        FORMAT_ARG_EVALUATED.store(false, Ordering::SeqCst);
        debug5!("{}", evaluate_format_arg());
        debug4!("{}", evaluate_format_arg());
        debug3!("{}", evaluate_format_arg());
        debug2!("{}", evaluate_format_arg());
        debug1!("{}", evaluate_format_arg());
        assert!(
            !FORMAT_ARG_EVALUATED.load(Ordering::SeqCst),
            "DEBUG-level messages should not evaluate format arguments eagerly at default log level"
        );
    }

    #[pg_test]
    fn test_warning_evaluates_args() {
        // WARNING is interesting at default settings (log_min_messages=WARNING)
        FORMAT_ARG_EVALUATED.store(false, Ordering::SeqCst);
        warning!("{}", evaluate_format_arg());
        assert!(
            FORMAT_ARG_EVALUATED.load(Ordering::SeqCst),
            "warning! should evaluate format arguments at default log level"
        );
    }
}
