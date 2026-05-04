//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![allow(clippy::type_complexity)]
#![allow(clippy::result_large_err)]

mod framework;

pub use framework::*;
#[cfg(feature = "proptest")]
pub mod proptest;

// ── Test stubs ──────────────────────────────────────────────────────────────
// When a test binary links pgrx-pg-sys, all the extern "C" statics declared in
// the generated bindings become undefined symbols.  Normally --gc-sections
// prunes unreachable references, but #[typetag::serde] / inventory::submit!
// ctors run before main() and pull pg_sys symbols into the live set.
//
// Because pgrx-tests is a dev-dependency, it is linked into test binaries but
// NOT into the extension .so.  These #[unsafe(no_mangle)] statics satisfy the linker
// without ever shadowing the real PostgreSQL symbols at runtime.
// ─────────────────────────────────────────────────────────────────────────────
mod stubs {
    use pgrx::pg_sys;

    #[unsafe(no_mangle)]
    pub static mut CurrentMemoryContext: pg_sys::MemoryContext = std::ptr::null_mut();

    #[unsafe(no_mangle)]
    pub static mut error_context_stack: *mut pg_sys::ErrorContextCallback = std::ptr::null_mut();

    #[unsafe(no_mangle)]
    pub static mut PG_exception_stack: *mut pg_sys::sigjmp_buf = std::ptr::null_mut();

    #[unsafe(no_mangle)]
    pub static mut emit_log_hook: pg_sys::emit_log_hook_type = None;

    #[unsafe(no_mangle)]
    pub static mut Log_error_verbosity: std::ffi::c_int = 0;

    #[unsafe(no_mangle)]
    pub static mut Log_line_prefix: *mut std::ffi::c_char = std::ptr::null_mut();

    #[unsafe(no_mangle)]
    pub static mut Log_destination: std::ffi::c_int = 0;

    #[unsafe(no_mangle)]
    pub static mut Log_destination_string: *mut std::ffi::c_char = std::ptr::null_mut();

    #[unsafe(no_mangle)]
    pub static mut syslog_sequence_numbers: bool = false;

    #[unsafe(no_mangle)]
    pub static mut syslog_split_messages: bool = false;

    #[unsafe(no_mangle)]
    pub static mut BufferBlocks: *mut std::ffi::c_void = std::ptr::null_mut();
}
