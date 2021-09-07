// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! Handing for easily converting Postgres Datum types into their corresponding Rust types
//! and converting Rust types into their corresponding Postgres types
mod anyarray;
mod anyelement;
mod array;
mod date;
mod from;
mod geo;
mod inet;
mod internal;
mod into;
mod item_pointer_data;
mod json;
mod numeric;
mod time;
mod time_stamp;
mod time_stamp_with_timezone;
mod time_with_timezone;
mod tuples;
mod uuid;
mod varlena;

pub use self::time::*;
pub use self::uuid::*;
pub use anyarray::*;
pub use anyelement::*;
pub use array::*;
pub use date::*;
pub use from::*;
pub use geo::*;
pub use inet::*;
pub use internal::*;
pub use into::*;
pub use item_pointer_data::*;
pub use json::*;
pub use numeric::*;
pub use time_stamp::*;
pub use time_stamp_with_timezone::*;
pub use time_with_timezone::*;
pub use tuples::*;
pub use varlena::*;

/// A tagging trait to indicate a user type is also meant to be used by Postgres
/// Implemented automatically by `#[derive(PostgresType)]`
pub trait PostgresType {}
