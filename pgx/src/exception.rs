use pgx_pg_sys::PgTryResult;

pub fn pg_try<Try, R>(try_func: Try) -> PgTryResult<R>
where
    Try: Fn() -> R + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    crate::guard::try_guard(try_func)
}
