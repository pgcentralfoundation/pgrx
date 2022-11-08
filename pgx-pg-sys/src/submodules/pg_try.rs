use crate::errcodes::PgSqlErrorCode;
use crate::panic::{downcast_panic_payload, CaughtError};
use std::collections::BTreeMap;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe, RefUnwindSafe, UnwindSafe};

pub struct PgTryBuilder<'a, R, F: FnOnce() -> R + UnwindSafe> {
    func: F,
    when: BTreeMap<
        PgSqlErrorCode,
        Box<dyn FnMut(CaughtError) -> R + 'a + UnwindSafe + RefUnwindSafe>,
    >,
    others: Option<Box<dyn FnMut(CaughtError) -> R + 'a + UnwindSafe + RefUnwindSafe>>,
    rust: Option<Box<dyn FnMut(CaughtError) -> R + 'a + UnwindSafe + RefUnwindSafe>>,
    finally: Option<Box<dyn FnMut() + 'a>>,
}

impl<'a, R, F: FnOnce() -> R + UnwindSafe> PgTryBuilder<'a, R, F> {
    #[must_use = "must call `PgTryBuilder::execute(self)` in order for it to run"]
    pub fn new(func: F) -> Self {
        Self { func, when: Default::default(), others: None, rust: None, finally: None }
    }

    #[must_use = "must call `PgTryBuilder::execute(self)` in order for it to run"]
    pub fn catch_when(
        mut self,
        error: PgSqlErrorCode,
        f: impl FnMut(CaughtError) -> R + 'a + UnwindSafe + RefUnwindSafe,
    ) -> Self {
        self.when.insert(error, Box::new(f));
        self
    }

    #[must_use = "must call `PgTryBuilder::execute(self)` in order for it to run"]
    pub fn catch_others(
        mut self,
        f: impl FnMut(CaughtError) -> R + 'a + UnwindSafe + RefUnwindSafe,
    ) -> Self {
        self.others = Some(Box::new(f));
        self
    }

    #[must_use = "must call `PgTryBuilder::execute(self)` in order for it to run"]
    pub fn catch_rust_panic(
        mut self,
        f: impl FnMut(CaughtError) -> R + 'a + UnwindSafe + RefUnwindSafe,
    ) -> Self {
        self.rust = Some(Box::new(f));
        self
    }

    #[must_use = "must call `PgTryBuilder::execute(self)` in order for it to run"]
    pub fn finally(mut self, f: impl FnMut() + 'a) -> Self {
        self.finally = Some(Box::new(f));
        self
    }

    pub fn execute(mut self) -> R {
        let result = catch_unwind(self.func);

        fn finally<F: FnMut()>(f: &mut Option<F>) {
            if let Some(f) = f {
                f()
            }
        }

        let result = match result {
            Ok(result) => result,
            Err(error) => {
                let (sqlerrcode, root_cause) = match downcast_panic_payload(error) {
                    CaughtError::RustPanic { ereport, payload } => {
                        let sqlerrcode = ereport.inner.sqlerrcode;
                        let panic = CaughtError::RustPanic { ereport, payload };
                        (sqlerrcode, panic)
                    }
                    CaughtError::ErrorReport(ereport) => {
                        let sqlerrcode = ereport.inner.sqlerrcode;
                        let panic = CaughtError::ErrorReport(ereport);
                        (sqlerrcode, panic)
                    }
                    CaughtError::PostgresError(ereport) => {
                        let sqlerrcode = ereport.inner.sqlerrcode;
                        let panic = CaughtError::PostgresError(ereport);
                        (sqlerrcode, panic)
                    }
                };

                // Postgres source docs says that a PG_TRY/PG_CATCH/PG_FINALLY block can't have
                // both a CATCH and a FINALLY.
                //
                // We allow it by wrapping handler execution in its own `catch_unwind()` block
                // and deferring the finally block execution until after it is complete
                let handler_result = catch_unwind(AssertUnwindSafe(|| {
                    if let Some(mut handler) = self.when.remove(&sqlerrcode) {
                        // we have a registered catch handler for the error code we caught
                        return handler(root_cause);
                    } else if let Some(mut handler) = self.others {
                        // we have a registered "catch others" handler
                        return handler(root_cause);
                    } else if let Some(mut handler) = self.rust {
                        // we have a registered catch handler for a rust panic
                        if let cause @ CaughtError::RustPanic { .. } = root_cause {
                            // and we have a rust panic
                            return handler(cause);
                        }
                    }

                    // we have no handler capable of handling whatever error we have, so rethrow the root cause
                    root_cause.rethrow();
                }));

                let handler_result = match handler_result {
                    Ok(result) => result,
                    Err(caught) => {
                        let catch_handler_error = downcast_panic_payload(caught);

                        // make sure to run the finally block and then resume unwinding
                        // with this new panic
                        finally(&mut self.finally);
                        resume_unwind(Box::new(catch_handler_error))
                    }
                };

                // Being here means the catch handler didn't raise an error.
                //
                // Postgres says:
                unsafe {
                    /*
                     * FlushErrorState --- flush the error state after error recovery
                     *
                     * This should be called by an error handler after it's done processing
                     * the error; or as soon as it's done CopyErrorData, if it intends to
                     * do stuff that is likely to provoke another error.  You are not "out" of
                     * the error subsystem until you have done this.
                     */
                    crate::FlushErrorState();
                }
                handler_result
            }
        };

        // `result` could be from a successful execution or returned from a catch handler
        //
        // Either way, the finally block needs to be run before we return it
        finally(&mut self.finally);
        result
    }
}
