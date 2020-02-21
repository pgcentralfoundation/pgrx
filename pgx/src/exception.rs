#[macro_export]
macro_rules! pg_rethrow {
    () => {
        return Err(());
    };
}

#[macro_export]
macro_rules! pg_ok {
    () => {
        return Ok(());
    };
}

pub fn pg_try<Try, Catch, R>(try_func: Try, catch_func: Catch) -> Option<R>
where
    Try: Fn() -> R + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Catch: Fn() -> std::result::Result<(), ()> + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    crate::guard::catch_guard(crate::guard::try_guard(try_func), catch_func)
}
