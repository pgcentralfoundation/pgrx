use pgrx::pg_sys::ffi::pg_guard_ffi_boundary;
use pgrx::prelude::*;
use std::ffi::CStr;

::pgrx::pg_module_magic!(name, version);

static mut PREV_POST_PARSE_ANALYZE_HOOK: pg_sys::post_parse_analyze_hook_type = None;

// `build.rs` compiles "c_ext.c" with PGXS and links it into this extension.
#[cfg(not(target_os = "windows"))]
unsafe extern "C-unwind" {
    /// Runs SPI on a new thread and waits for it, raising an ERROR when the work fails.
    fn pgthread_run_spi_in_thread();
}

/// Run SPI on a pthread through the C code, which is not how Postgres is meant to be used.
///
/// Rust exports this function. A C function would need its own version script to stay
/// exported, and GNU ld refuses to combine one with the version script rustc writes.
#[cfg(not(target_os = "windows"))]
#[pg_extern]
fn start_thread() {
    // SAFETY: pgrx calls `#[pg_extern]` functions on the backend thread. The closure only
    // calls the C function, so a raised ERROR jumps over nothing that needs dropping.
    unsafe { pg_guard_ffi_boundary(|| pgthread_run_spi_in_thread()) }
}

#[pg_guard]
pub unsafe extern "C-unwind" fn _PG_init() {
    PREV_POST_PARSE_ANALYZE_HOOK = pg_sys::post_parse_analyze_hook;
    pg_sys::post_parse_analyze_hook = Some(parse_analyze_hook);
}

#[cfg(not(feature = "pg19"))]
#[pg_guard(unsafe_entry_thread)]
unsafe extern "C-unwind" fn parse_analyze_hook(
    pstate: *mut pg_sys::ParseState,
    query: *mut pg_sys::Query,
    jstate: *mut pg_sys::JumbleState,
) {
    do_the_hook(pstate);
    if let Some(prev_hook) = PREV_POST_PARSE_ANALYZE_HOOK {
        pg_guard_ffi_boundary(|| prev_hook(pstate, query, jstate));
    }
}
// Postgres 19 changed the `JumbleState` argument to `*const`
#[cfg(feature = "pg19")]
#[pg_guard(unsafe_entry_thread)]
unsafe extern "C-unwind" fn parse_analyze_hook(
    pstate: *mut pg_sys::ParseState,
    query: *mut pg_sys::Query,
    jstate: *const pg_sys::JumbleState,
) {
    do_the_hook(pstate);
    if let Some(prev_hook) = PREV_POST_PARSE_ANALYZE_HOOK {
        pg_guard_ffi_boundary(|| prev_hook(pstate, query, jstate));
    }
}

unsafe fn do_the_hook(pstate: *mut pg_sys::ParseState) {
    pg_sys::palloc(1);

    let query_source = CStr::from_ptr((*pstate).p_sourcetext);

    if query_source.eq(c"SELECT 1;") {
        panic!("oh no");
    }
}

#[pg_extern]
fn hello_pgthread() -> &'static str {
    "Hello, pgthread"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pgthread() {
        assert_eq!("Hello, pgthread", crate::hello_pgthread());
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
