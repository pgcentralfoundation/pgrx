//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use core::ptr;
use pgrx::prelude::*;

// if our Postgres ERROR and Rust panic!() handling is incorrect, this little bit of useless code
// will crash postgres.  If things are correct it'll simply raise an ERROR saying "panic in walker".
#[pg_extern]
fn crash() {
    use pgrx::{list::List, memcx};
    // SAFETY: the list allocation belongs to the live current memory context;
    // walker uses pg_guard so its intentional panic crosses C through pgrx's
    // callback guard rather than unwinding across an unguarded C frame.
    memcx::current_context(|current| unsafe {
        let mut list = List::downcast_ptr_in_memcx(ptr::null_mut(), current).unwrap();
        list.unstable_push_in_context(ptr::null_mut(), current);

        #[cfg(feature = "pg15")]
        pg_sys::raw_expression_tree_walker(list.as_mut_ptr().cast(), Some(walker), ptr::null_mut());
        #[cfg(not(feature = "pg15"))]
        {
            let callback: pg_sys::tree_walker_callback = Some(walker);
            pg_sys::raw_expression_tree_walker!(
                list.as_mut_ptr().cast::<pg_sys::Node>(),
                // Rust callback aliases erase C typedef identity. The generated
                // wrapper supplies it while checking the exact C-unwind storage.
                pg_sys::__pgrx_c_callbacks::tree_walker_callback::new(callback),
                ptr::null_mut::<core::ffi::c_void>()
            );
        }
    });
}

#[pg_guard]
extern "C-unwind" fn walker(_node: *mut pg_sys::Node, _void: *mut ::core::ffi::c_void) -> bool {
    panic!("panic in walker");
}

#[pg_extern]
fn get_relation_name(oid: pg_sys::Oid) -> String {
    PgTryBuilder::new(|| unsafe {
        // pg_sys::relation_open will raise a XX000 if the specified oid isn't a valid relation
        let rel = PgBox::from_pg(pg_sys::relation_open(oid, pg_sys::AccessShareLock as _));
        let pg_class_entry = PgBox::from_pg(rel.rd_rel);
        let relname = &pg_class_entry.relname;
        name_data_to_str(relname).to_string()
    })
    .catch_when(PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, |_error| {
        // so in the case the oid isn't a valid relation, just return a generic string
        format!("<{oid:?} is not a relation>")
    })
    .execute()
}

/// Runs `sql` and rethrows its error from the catch handler or, if `after_handler`, after the
/// handler has returned it.
#[pg_extern]
fn rethrow_caught_error(sql: &str, after_handler: bool) {
    let caught = PgTryBuilder::new(|| {
        Spi::run(sql).expect("SPI failed");
        None
    })
    .catch_others(|e| if after_handler { Some(e) } else { e.rethrow() })
    .execute();
    if let Some(e) = caught {
        e.rethrow()
    }
}

/// Raises an ERROR `depth` SPI calls down, so it crosses that many guards on the way back up.
#[pg_extern]
fn raise_through_guards(depth: i32) {
    if depth == 0 {
        error!("raised through guards");
    }
    Spi::run(&format!("SELECT raise_through_guards({})", depth - 1)).expect("SPI failed");
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use std::collections::BTreeMap;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicBool, Ordering};

    use pgrx::prelude::*;

    // see: c5cd61d7bfdfb5236ef0f8b98f433b35a2444346
    #[pg_test]
    fn test_we_dont_blow_out_errdata_stack_size() {
        Spi::run("SELECT get_relation_name(x) FROM generate_series(1, 1000) x")
            .expect("SPI failed");
    }

    #[pg_test(error = "panic in walker")]
    fn test_panic_in_extern_c_fn() -> Result<(), pgrx::spi::Error> {
        Spi::get_one::<()>("SELECT crash()").map(|_| ())
    }

    /// When a `pg_sys::` FFI call raises a Postgres ERROR (via longjmp), its `CaughtError`
    /// should carry a Rust backtrace for catch handlers to inspect.
    #[pg_test]
    fn test_postgres_error_contains_backtrace() {
        use pgrx::pg_sys::panic::CaughtError;

        let has_backtrace = PgTryBuilder::new(|| {
            // relation_open with a bogus OID will raise an ERROR inside Postgres
            unsafe {
                pg_sys::relation_open(pg_sys::Oid::INVALID, pg_sys::AccessShareLock as _);
            }
            unreachable!("relation_open should have raised an ERROR");
        })
        .catch_others(|e| match e {
            CaughtError::PostgresError(ref ereport) => ereport.backtrace().is_some(),
            _ => panic!("expected CaughtError::PostgresError, got {e:?}"),
        })
        .execute();

        assert!(has_backtrace, "PostgresError from a pg_sys FFI call should contain a backtrace");
    }

    /// When a Postgres ERROR propagates through Rust frames, destructors must run via panic
    /// unwinding and catch handlers should be able to inspect the captured Rust backtrace.
    #[pg_test]
    fn test_postgres_error_runs_drop_and_has_backtrace() {
        use pgrx::pg_sys::panic::CaughtError;

        let dropped = Rc::new(AtomicBool::new(false));

        struct Guard(Rc<AtomicBool>);
        impl Drop for Guard {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }

        let (drop_ran, has_backtrace) = PgTryBuilder::new(|| {
            let _guard = Guard(dropped.clone());

            // relation_open with a bogus OID raises an ERROR inside Postgres, which
            // pg_guard_ffi_boundary converts to a panic.  The panic unwinds through
            // this frame, running Guard's destructor.
            unsafe {
                pg_sys::relation_open(pg_sys::Oid::INVALID, pg_sys::AccessShareLock as _);
            }
            unreachable!("relation_open should have raised an ERROR");
        })
        .catch_others(|e| {
            let bt = match e {
                CaughtError::PostgresError(ref ereport) => ereport.backtrace().is_some(),
                _ => panic!("expected CaughtError::PostgresError, got {e:?}"),
            };
            (dropped.load(Ordering::SeqCst), bt)
        })
        .execute();

        assert!(drop_ran, "Guard's Drop impl should have run during panic unwinding");
        assert!(has_backtrace, "PostgresError should carry a Rust backtrace");
    }

    #[pg_test]
    fn test_pg_try_execute_no_error_no_catch() {
        let result = PgTryBuilder::new(|| 42).execute();
        assert_eq!(42, result)
    }

    #[pg_test(error = "unwrapped a panic")]
    fn test_pg_try_execute_with_error() {
        PgTryBuilder::new(|| panic!("unwrapped a panic")).execute();
    }

    #[pg_test(error = "raised an error")]
    fn test_pg_try_execute_with_error_report() {
        PgTryBuilder::new(|| error!("raised an error")).execute();
    }

    #[pg_test(error = "panic in walker")]
    fn test_pg_try_execute_with_crash() {
        PgTryBuilder::new(super::crash).execute();
    }

    #[pg_test]
    fn test_pg_try_execute_with_crash_ignore() {
        PgTryBuilder::new(super::crash)
            .catch_when(PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, |_| ())
            .catch_others(|e| panic!("{e:?}"))
            .execute();
    }

    #[pg_test(error = "panic in walker")]
    fn test_pg_try_execute_with_crash_rethrow() {
        PgTryBuilder::new(super::crash)
            .catch_when(PgSqlErrorCode::ERRCODE_INTERNAL_ERROR, |e| e.rethrow())
            .execute();
    }

    #[pg_test]
    fn test_pg_try_execute_no_error() {
        let result = PgTryBuilder::new(|| 42).catch_others(|_| 99).execute();
        assert_eq!(42, result);
    }

    #[pg_test]
    fn test_pg_try_ignore_panic() {
        let result =
            PgTryBuilder::new(|| panic!("unwrapped a panic")).catch_others(|_| 99).execute();
        assert_eq!(99, result);
    }

    #[pg_test]
    fn test_pg_try_no_error_with_catch() {
        let result = PgTryBuilder::new(|| 42).catch_others(|_| 99).execute();
        assert_eq!(42, result);
    }

    #[pg_test(error = "panic in catch")]
    fn test_pg_try_throw_different_error() {
        PgTryBuilder::new(|| panic!("unwrapped a panic"))
            .catch_others(|_| panic!("panic in catch"))
            .execute();
    }

    #[pg_test]
    fn test_pg_try_catch_and_rethrow_no_error() {
        let result = PgTryBuilder::new(|| 42).catch_others(|e| e.rethrow()).execute();
        assert_eq!(42, result);
    }

    #[pg_test(error = "rethrow a panic")]
    fn test_pg_try_catch_and_rethrow_with_error() {
        PgTryBuilder::new(|| panic!("rethrow a panic")).catch_others(|e| e.rethrow()).execute();
    }

    #[pg_test]
    fn test_pg_try_finally() {
        let mut finally = false;
        let result = PgTryBuilder::new(|| 42).finally(|| finally = true).execute();
        assert_eq!(42, result);
        assert!(finally);
    }

    #[pg_test]
    fn test_pg_try_finally_with_catch() {
        let mut finally = false;
        let result = PgTryBuilder::new(|| panic!("panic"))
            .catch_others(|_| 99)
            .finally(|| finally = true)
            .execute();
        assert_eq!(99, result);
        assert!(finally);
    }

    #[pg_test]
    fn test_pg_try_finally_with_catch_rethrow() {
        let finally = Rc::new(AtomicBool::new(false));
        let result = std::panic::catch_unwind(|| {
            PgTryBuilder::new(|| panic!("panic"))
                .catch_others(|e| e.rethrow())
                .finally(|| finally.store(true, Ordering::SeqCst))
                .execute()
        });
        // finally block does run, even if a catch block throws
        assert!(finally.load(Ordering::SeqCst));
        assert!(result.is_err());
    }

    #[pg_test]
    fn test_drop() {
        struct Foo(Rc<AtomicBool>);
        impl Drop for Foo {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }

        let outer = Rc::new(AtomicBool::new(false));
        let inner = Rc::new(AtomicBool::new(false));
        {
            let _outer = Foo(outer.clone());

            PgTryBuilder::new(|| {
                let _inner = Foo(inner.clone());
            })
            .execute();
        }

        assert!(inner.load(Ordering::SeqCst));
        assert!(outer.load(Ordering::SeqCst));
    }

    #[pg_test]
    fn test_drop_with_panic_no_catch() {
        struct Foo(Rc<AtomicBool>);
        impl Drop for Foo {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }

        let outer = Rc::new(AtomicBool::new(false));
        let inner = Rc::new(AtomicBool::new(false));
        {
            let _outer = Foo(outer.clone());

            std::panic::catch_unwind(|| {
                PgTryBuilder::new(|| {
                    let _inner = Foo(inner.clone());
                    panic!("get out")
                })
                .execute()
            })
            .ok();
        }
        assert!(inner.load(Ordering::SeqCst));
        assert!(outer.load(Ordering::SeqCst));
    }

    #[pg_test]
    fn test_drop_with_panic_catch_and_rethrow() {
        struct Foo(Rc<AtomicBool>);
        impl Drop for Foo {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }

        let outer = Rc::new(AtomicBool::new(false));
        let inner = Rc::new(AtomicBool::new(false));
        {
            let _outer = Foo(outer.clone());

            std::panic::catch_unwind(|| {
                PgTryBuilder::new(|| {
                    let _inner = Foo(inner.clone());
                    panic!("get out")
                })
                .catch_others(|e| e.rethrow())
                .execute()
            })
            .ok();
        }
        assert!(inner.load(Ordering::SeqCst));
        assert!(outer.load(Ordering::SeqCst));
    }

    #[pg_test]
    fn test_drop_with_panic_catch_and_ignore() {
        struct Foo(Rc<AtomicBool>);
        impl Drop for Foo {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }

        let outer = Rc::new(AtomicBool::new(false));
        let inner = Rc::new(AtomicBool::new(false));
        {
            let _outer = Foo(outer.clone());

            std::panic::catch_unwind(|| {
                PgTryBuilder::new(|| {
                    let _inner = Foo(inner.clone());
                    panic!("get out")
                })
                .catch_others(|_| ())
                .execute()
            })
            .ok();
        }
        assert!(inner.load(Ordering::SeqCst));
        assert!(outer.load(Ordering::SeqCst));
    }

    #[pg_test]
    fn test_drop_with_panic_and_ignore_and_finally() {
        struct Foo(Rc<AtomicBool>);
        impl Drop for Foo {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }

        let outer = Rc::new(AtomicBool::new(false));
        let inner = Rc::new(AtomicBool::new(false));
        let finally = Rc::new(AtomicBool::new(false));
        {
            let _outer = Foo(outer.clone());

            std::panic::catch_unwind(|| {
                PgTryBuilder::new(|| {
                    let _inner = Foo(inner.clone());
                    panic!("get out")
                })
                .catch_others(|_| ())
                .finally(|| finally.store(true, Ordering::SeqCst))
                .execute()
            })
            .ok();
        }
        assert!(inner.load(Ordering::SeqCst));
        assert!(outer.load(Ordering::SeqCst));

        // really just testing that the finally block ran
        assert!(finally.load(Ordering::SeqCst));
    }

    /// Raises PostgreSQL's "could not open relation with OID {oid}" ERROR.
    fn open_missing_relation(oid: u32) {
        let lockmode = pg_sys::AccessShareLock as pg_sys::LOCKMODE;
        // SAFETY: relation_open raises an ERROR for an OID that names no relation. A relation it
        // does open is closed with the lock mode it was opened with.
        unsafe {
            let rel = pg_sys::relation_open(pg_sys::Oid::from(oid), lockmode);
            pg_sys::relation_close(rel, lockmode);
        }
    }

    /// The items that `pg_temp.diagnostics()` reads, in the order it returns them.
    const DIAGNOSTICS: [&str; 10] = [
        "RETURNED_SQLSTATE",
        "MESSAGE_TEXT",
        "PG_EXCEPTION_DETAIL",
        "PG_EXCEPTION_HINT",
        "PG_EXCEPTION_CONTEXT",
        "SCHEMA_NAME",
        "TABLE_NAME",
        "COLUMN_NAME",
        "PG_DATATYPE_NAME",
        "CONSTRAINT_NAME",
    ];

    /// Creates `pg_temp.diagnostics(after_handler)`, which has `rethrow_caught_error` rethrow a
    /// unique violation into PL/pgSQL and returns what `GET STACKED DIAGNOSTICS` reads from it.
    fn create_diagnostics_function() {
        Spi::run(
            "CREATE TEMP TABLE caught_error (k int CONSTRAINT caught_error_k_unique UNIQUE);
             INSERT INTO caught_error VALUES (1);
             CREATE FUNCTION pg_temp.diagnostics(after_handler bool) RETURNS text[]
             LANGUAGE plpgsql AS $$
             DECLARE
                 d_state text;
                 d_message text;
                 d_detail text;
                 d_hint text;
                 d_context text;
                 d_schema text;
                 d_table text;
                 d_column text;
                 d_datatype text;
                 d_constraint text;
             BEGIN
                 PERFORM rethrow_caught_error('INSERT INTO caught_error VALUES (1)', after_handler);
                 RETURN NULL;
             EXCEPTION WHEN OTHERS THEN
                 GET STACKED DIAGNOSTICS
                     d_state = RETURNED_SQLSTATE,
                     d_message = MESSAGE_TEXT,
                     d_detail = PG_EXCEPTION_DETAIL,
                     d_hint = PG_EXCEPTION_HINT,
                     d_context = PG_EXCEPTION_CONTEXT,
                     d_schema = SCHEMA_NAME,
                     d_table = TABLE_NAME,
                     d_column = COLUMN_NAME,
                     d_datatype = PG_DATATYPE_NAME,
                     d_constraint = CONSTRAINT_NAME;
                 RETURN ARRAY[d_state, d_message, d_detail, d_hint, d_context,
                              d_schema, d_table, d_column, d_datatype, d_constraint];
             END $$",
        )
        .expect("SPI failed");
    }

    fn diagnostics(after_handler: bool) -> BTreeMap<&'static str, String> {
        // the flag is a parameter, since CONTEXT quotes the query
        let values = Spi::get_one_with_args::<Vec<Option<String>>>(
            "SELECT pg_temp.diagnostics($1)",
            &[after_handler.into()],
        )
        .expect("SPI failed")
        .expect("rethrow_caught_error should have raised an ERROR");
        DIAGNOSTICS
            .into_iter()
            .zip(values)
            .map(|(item, value)| (item, value.unwrap_or_default()))
            .collect()
    }

    /// `execute` clears PostgreSQL's error state when a handler returns, so a later rethrow
    /// raises pgrx's copy of the error, which PL/pgSQL can't tell from the original.
    #[pg_test]
    fn test_rethrow_after_handler_reports_copy() {
        create_diagnostics_function();
        let copy = diagnostics(true);
        assert_eq!(copy["CONSTRAINT_NAME"], "caught_error_k_unique");
        assert_eq!(copy, diagnostics(false));
    }

    #[pg_test(error = "could not open relation with OID 0")]
    fn test_rethrow_after_nested_pg_try_handled_error() {
        PgTryBuilder::new(|| open_missing_relation(0))
            .catch_others(|e| {
                PgTryBuilder::new(|| open_missing_relation(1)).catch_others(|_| ()).execute();
                e.rethrow()
            })
            .execute();
    }

    #[pg_test(error = "could not open relation with OID 0")]
    fn test_rethrow_after_plpgsql_handled_error() {
        PgTryBuilder::new(|| open_missing_relation(0))
            .catch_others(|e| {
                Spi::run(
                    "DO $$ BEGIN PERFORM 1 / 0; EXCEPTION WHEN division_by_zero THEN NULL; END $$",
                )
                .expect("SPI failed");
                e.rethrow()
            })
            .execute();
    }

    #[pg_test(error = "could not open relation with OID 0")]
    fn test_rethrow_under_newer_swallowed_error() {
        PgTryBuilder::new(|| open_missing_relation(0))
            .catch_others(|e| {
                std::panic::catch_unwind(|| open_missing_relation(1)).unwrap_err();
                e.rethrow()
            })
            .execute();
    }

    #[pg_test(error = "could not open relation with OID 0")]
    fn test_rethrow_over_older_swallowed_error() {
        let caught = PgTryBuilder::new(|| {
            open_missing_relation(0);
            None
        })
        .catch_others(Some)
        .execute();
        std::panic::catch_unwind(|| open_missing_relation(1)).unwrap_err();
        caught.expect("relation_open should have raised an ERROR").rethrow();
    }

    /// The boundary's own `CopyErrorData()` raises when C code flushed the stack before it
    /// rethrew.  Postgres' exception and error context stacks must already point where they did
    /// before the boundary, not at its jump buffer, whose frame has returned.
    #[pg_test]
    fn test_boundary_restores_stacks_before_copying_error() {
        use pgrx::pg_sys::panic::CaughtError;

        #[cfg_attr(target_os = "windows", link(name = "postgres"))]
        unsafe extern "C-unwind" {
            fn pg_re_throw() -> !;
        }

        // SAFETY: tests run on the backend thread
        let before = unsafe { (pg_sys::PG_exception_stack, pg_sys::error_context_stack) };
        let message = PgTryBuilder::new(|| unsafe {
            // SAFETY: the closure holds nothing that needs dropping when pg_re_throw() jumps
            pg_sys::ffi::pg_guard_ffi_boundary(|| -> String {
                pg_sys::FlushErrorState();
                pg_re_throw()
            })
        })
        .catch_others(|e| match e {
            CaughtError::PostgresError(ereport) => ereport.message().to_string(),
            e => e.rethrow(),
        })
        .execute();

        // SAFETY: as above.  Put the stacks back first, so a failed assertion can raise its ERROR
        let after = unsafe { (pg_sys::PG_exception_stack, pg_sys::error_context_stack) };
        unsafe { (pg_sys::PG_exception_stack, pg_sys::error_context_stack) = before };
        assert_eq!(message, "errstart was not called");
        assert_eq!(after, before);
    }

    /// Catches a Rust panic and returns.  As an error context callback, it runs while Postgres
    /// handles an ERROR.
    #[pg_guard]
    unsafe extern "C-unwind" fn swallow_rust_panic(_arg: *mut core::ffi::c_void) {
        PgTryBuilder::new(|| panic!("swallowed")).catch_others(|_| ()).execute();
    }

    /// Catching a Rust panic while Postgres handles an ERROR leaves that ERROR on Postgres' stack.
    #[pg_test(error = "could not open relation with OID 0")]
    fn test_rust_panic_caught_while_postgres_handles_error() {
        // SAFETY: tests run on the backend thread
        let previous = unsafe { pg_sys::error_context_stack };
        let mut callback = pg_sys::ErrorContextCallback {
            previous,
            callback: Some(swallow_rust_panic),
            arg: core::ptr::null_mut(),
        };
        PgTryBuilder::new(std::panic::AssertUnwindSafe(|| {
            // SAFETY: `finally` takes `callback` off the stack before it goes out of scope
            unsafe { pg_sys::error_context_stack = &mut callback };
            open_missing_relation(0);
        }))
        .finally(|| unsafe { pg_sys::error_context_stack = previous })
        .execute();
    }

    /// Raises PostgreSQL's "invalid input syntax for type integer" ERROR without allocating in
    /// the current memory context.
    fn parse_bad_int4() -> i32 {
        // SAFETY: the argument is a NUL-terminated string
        unsafe { pg_sys::pg_strtoint32(c"x".as_ptr()) }
    }

    /// Whether Postgres' error data stack is empty, which `geterrcode()` reports by raising
    fn error_stack_is_empty() -> bool {
        use pgrx::pg_sys::panic::CaughtError;

        PgTryBuilder::new(|| {
            // SAFETY: geterrcode() reads the top entry's SQLSTATE, and raises if there isn't one
            unsafe { pg_sys::geterrcode() };
            false
        })
        .catch_others(|e| match e {
            CaughtError::PostgresError(ereport)
                if ereport.message() == "errstart was not called" =>
            {
                true
            }
            e => e.rethrow(),
        })
        .execute()
    }

    /// An inner handler replaces a Postgres ERROR with a Rust error, which an outer handler
    /// swallows.
    fn swallow_replaced_postgres_error() {
        PgTryBuilder::new(|| {
            PgTryBuilder::new(|| open_missing_relation(0))
                .catch_others(|_| error!("replaced"))
                .execute()
        })
        .catch_others(|_| ())
        .execute();
    }

    #[pg_test]
    fn test_replaced_postgres_error_leaves_stack_empty() {
        swallow_replaced_postgres_error();
        assert!(error_stack_is_empty(), "the replaced ERROR is still on Postgres' error stack");
    }

    #[pg_test]
    fn test_replaced_postgres_errors_do_not_fill_stack() {
        for _ in 0..6 {
            swallow_replaced_postgres_error();
        }
    }

    /// Each guard rethrows the original ERROR.  Raising a copy instead would push one more entry
    /// onto Postgres' five-entry error data stack per guard.
    #[pg_test(error = "raised through guards")]
    fn test_rethrow_through_seven_guards() {
        Spi::run("SELECT raise_through_guards(7)").expect("SPI failed");
    }

    /// Code that runs while `CurrentMemoryContext` is `ErrorContext`, such as a C `PG_CATCH()`
    /// block before it switches back, can catch a Postgres ERROR and stays in `ErrorContext`.
    #[pg_test]
    fn test_catch_postgres_error_in_error_context() {
        // SAFETY: tests run on the backend thread, and the previous context is switched back to
        let (caught, current) = unsafe {
            let previous = pg_sys::MemoryContextSwitchTo(pg_sys::ErrorContext);
            let caught =
                PgTryBuilder::new(|| parse_bad_int4() == 0).catch_others(|_| true).execute();
            (caught, pg_sys::MemoryContextSwitchTo(previous))
        };
        assert!(caught);
        assert_eq!(current, unsafe { pg_sys::ErrorContext });
    }

    /// Catching a Postgres ERROR leaves nothing behind in the current memory context.
    #[pg_test]
    fn test_catch_postgres_error_frees_its_copy() {
        // SAFETY: tests run on the backend thread.  The context is switched away from before
        // it's deleted.
        let (before, after) = unsafe {
            let context = pg_sys::AllocSetContextCreateInternal(
                pg_sys::CurrentMemoryContext,
                c"caught errors".as_ptr(),
                pg_sys::ALLOCSET_DEFAULT_MINSIZE as usize,
                pg_sys::ALLOCSET_DEFAULT_INITSIZE as usize,
                pg_sys::ALLOCSET_DEFAULT_MAXSIZE as usize,
            );
            let previous = pg_sys::MemoryContextSwitchTo(context);
            let before = pg_sys::MemoryContextMemAllocated(context, false);
            for _ in 0..1000 {
                PgTryBuilder::new(parse_bad_int4).catch_others(|_| 0).execute();
            }
            let after = pg_sys::MemoryContextMemAllocated(context, false);
            pg_sys::MemoryContextSwitchTo(previous);
            pg_sys::MemoryContextDelete(context);
            (before, after)
        };
        assert_eq!(after, before);
    }
}
