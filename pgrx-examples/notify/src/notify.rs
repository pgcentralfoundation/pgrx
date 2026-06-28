//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Safe wrappers around PostgreSQL's asynchronous notification interface
//! (`commands/async.h`): `NOTIFY` / `LISTEN` / `UNLISTEN`.
//!
//! All of these must be called inside a transaction; notifications are
//! delivered to listening sessions when the transaction commits.

use std::ffi::{CString, NulError};

/// Emit a `NOTIFY` on `channel` with `payload`.
///
/// Must be called inside a transaction. The notification is delivered to listening sessions at commit. Returns `Err` if either argument contains an interior NUL byte. Over-length arguments (channel name > 63 bytes, payload beyond `NOTIFY_PAYLOAD_MAX_LENGTH`) or being called outside a transaction are reported by PostgreSQL as an `ERROR`.
pub fn notify(channel: &str, payload: &str) -> Result<(), NulError> {
    let channel = CString::new(channel)?;
    let payload = CString::new(payload)?;
    // SAFETY: both pointers are valid, NUL-terminated C strings that stay alive for the duration of the call. On the success path `Async_Notify` copies them into Postgres-managed memory, so dropping the CStrings afterwards is fine. If `Async_Notify` raises a Postgres ERROR it does so via `longjmp`, which is sound across this `extern "C-unwind"` boundary but — like every  pg_sys call in pgrx — skips this frame's Rust destructors; the only thing that leaks is these two transient CString allocations, on an already aborting transaction. That is bounded and matches pgrx's memory model.
    unsafe { pgrx::pg_sys::Async_Notify(channel.as_ptr(), payload.as_ptr()) };
    Ok(())
}

/// Begin listening on `channel` (server-side `LISTEN`). Must be in a transaction.
pub fn listen(channel: &str) -> Result<(), NulError> {
    let channel = CString::new(channel)?;
    // SAFETY: see `notify` — valid NUL-terminated pointer alive for the call.
    unsafe { pgrx::pg_sys::Async_Listen(channel.as_ptr()) };
    Ok(())
}

/// Stop listening on `channel` (server-side `UNLISTEN`). Must be in a transaction.
pub fn unlisten(channel: &str) -> Result<(), NulError> {
    let channel = CString::new(channel)?;
    // SAFETY: see `notify` — valid NUL-terminated pointer alive for the call.
    unsafe { pgrx::pg_sys::Async_Unlisten(channel.as_ptr()) };
    Ok(())
}

/// Stop listening on all channels (`UNLISTEN *`). Must be in a transaction.
pub fn unlisten_all() {
    // SAFETY: takes no arguments; any Postgres ERROR propagates as a longjmp,
    // see `notify`.
    unsafe { pgrx::pg_sys::Async_UnlistenAll() };
}
