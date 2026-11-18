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
    use pgrx::pg_sys::panic::CaughtError;
    use pgrx::prelude::*;
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};

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

    /// A domain length that nothing else in these tests allocates, with or without a NUL
    const LARGE_DOMAIN_LEN: usize = 1_048_583;

    /// Live allocations of a large domain's length, with or without its NUL
    static LARGE_DOMAINS: AtomicIsize = AtomicIsize::new(0);

    fn is_large_domain(size: usize) -> bool {
        size == LARGE_DOMAIN_LEN || size == LARGE_DOMAIN_LEN + 1
    }

    /// The system allocator, counting [`LARGE_DOMAINS`]
    struct CountLargeDomains;

    // SAFETY: every call goes to the system allocator with the same arguments
    unsafe impl GlobalAlloc for CountLargeDomains {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let ptr = unsafe { System.alloc(layout) };
            if !ptr.is_null() && is_large_domain(layout.size()) {
                LARGE_DOMAINS.fetch_add(1, Ordering::Relaxed);
            }
            ptr
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            if is_large_domain(layout.size()) {
                LARGE_DOMAINS.fetch_sub(1, Ordering::Relaxed);
            }
            unsafe { System.dealloc(ptr, layout) }
        }

        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            let new_ptr = unsafe { System.realloc(ptr, layout, new_size) };
            if !new_ptr.is_null() {
                if is_large_domain(layout.size()) {
                    LARGE_DOMAINS.fetch_sub(1, Ordering::Relaxed);
                }
                if is_large_domain(new_size) {
                    LARGE_DOMAINS.fetch_add(1, Ordering::Relaxed);
                }
            }
            new_ptr
        }
    }

    #[global_allocator]
    static ALLOCATOR: CountLargeDomains = CountLargeDomains;

    #[pg_extern]
    fn raise_with_large_domain() {
        pgrx::ereport_domain!(
            PgLogLevel::ERROR,
            "d".repeat(LARGE_DOMAIN_LEN),
            PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
            "raised with a large domain"
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

    /// Runs `sql`, which calls a function that raises an ERROR, and returns the domain of the
    /// ERROR.  The function's guard reports it to Postgres, which jumps back to SPI.
    fn domain_of_error_from(sql: &str) -> Option<String> {
        PgTryBuilder::new(|| {
            Spi::run(sql).expect("SPI failed");
            panic!("{sql} returned normally")
        })
        .catch_others(|e| match e {
            CaughtError::PostgresError(ereport) => ereport.domain().map(str::to_string),
            e => e.rethrow(),
        })
        .execute()
    }

    /// Each ERROR reported with the same domain leaves no copy of it behind.
    #[pg_test]
    fn test_ereport_domain_error_keeps_one_copy() {
        let before = LARGE_DOMAINS.load(Ordering::Relaxed);
        for _ in 0..4 {
            let domain = domain_of_error_from("SELECT tests.raise_with_large_domain()");
            assert_eq!(domain.map(|d| d.len()), Some(LARGE_DOMAIN_LEN));
        }
        let kept = LARGE_DOMAINS.load(Ordering::Relaxed) - before;
        assert!(kept <= 1, "{kept} copies of the domain are still allocated");
    }

    /// Postgres reads a domain as a C string, so it ends at its first NUL, like the message.
    #[pg_test]
    fn test_ereport_domain_with_nul() {
        let domain = domain_of_error_from("SELECT tests.raise_with_nul_in_domain()");
        assert_eq!(domain.as_deref(), Some("test_extension_domain"));
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
