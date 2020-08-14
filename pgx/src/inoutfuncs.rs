// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! Helper trait for the `#[derive(PostgresType)]` proc macro for overriding custom Postgres type
//! input/output functions.
//!
//! The default implementations use `serde_json` to serialize a custom type to human-readable strings,
//! and `serde_cbor` to serialize internally as a `varlena *` for storage on disk.

use crate::*;

/// `#[derive(Copy, Clone, PostgresType)]` types need to implement this trait to provide the text
/// input/output functions for that type
pub trait PgVarlenaInOutFuncs {
    /// Given a string representation of `Self`, parse it into a `PgVarlena<Self>`.
    ///
    /// It is expected that malformed input will raise an `error!()` or `panic!()`
    fn input(input: &std::ffi::CStr) -> PgVarlena<Self>
    where
        Self: Copy + Sized;

    /// Convert `Self` into text by writing to the supplied `StringInfo` buffer
    fn output(&self, buffer: &mut StringInfo);
}

/// `#[derive(Serialize, Deserialize, PostgresType)]` types may implement this trait if they prefer
/// a textual representation that isn't JSON
pub trait InOutFuncs {
    /// Given a string representation of `Self`, parse it into `Self`.
    ///
    /// It is expected that malformed input will raise an `error!()` or `panic!()`
    fn input(input: &std::ffi::CStr) -> Self
    where
        Self: Copy + Sized;

    /// Convert `Self` into text by writing to the supplied `StringInfo` buffer
    fn output(&self, buffer: &mut StringInfo);
}

/// Automatically implemented for `#[derive(Serialize, Deserialize, PostgresType)]` types that do
/// **not** also have the `#[inoutfuncs]` attribute macro
pub trait JsonInOutFuncs<'de>: serde::de::Deserialize<'de> + serde::ser::Serialize {
    /// Uses `serde_json` to deserialize the input, which is assumed to be JSON
    fn input(input: &'de std::ffi::CStr) -> Self {
        serde_json::from_str(input.to_str().expect("text input is not valid UTF8"))
            .expect("failed to deserialize json")
    }

    /// Users `serde_json` to serialize `Self` into JSON
    fn output(&self, buffer: &mut StringInfo)
    where
        Self: serde::ser::Serialize,
    {
        serde_json::to_writer(buffer, self).expect("failed to serialize to json")
    }
}
