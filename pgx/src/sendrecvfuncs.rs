/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Helper trait for the `#[derive(PostgresType)]` proc macro for overriding custom Postgres type
//! send/receive functions
//!

/// `#[derive(PostgresType)]` types need to implement this trait to provide the binary
/// send/receive functions for that type. They also *must* specify `#[sendrecvfuncs]` attribute.
pub trait SendRecvFuncs {
    /// Convert `Self` into a binary
    fn send(&self) -> Vec<u8>;

    /// Given a binary representation of `Self`, parse it into a `Self`.
    ///
    /// It is expected that malformed input will raise an `error!()` or `panic!()`
    fn recv(buffer: &[u8]) -> Self;
}
