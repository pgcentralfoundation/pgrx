//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! A safe wrapper around Postgres' internal [`List`][crate::pg_sys::List] structure.
//!
//! It functions similarly to a Rust [`Vec`][std::vec::Vec], including iterator support, but provides separate
//! understandings of [`List`][crate::pg_sys::List]s of [`Oid`][crate::pg_sys::Oid]s, Integers, and Pointers.

#[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
pub use flat_list::{Enlist, List, ListHead};

#[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
mod flat_list;

#[cfg(any(feature = "pg11", feature = "pg12"))]
pub use node_list::{Enlist, List, ListHead};

#[cfg(any(feature = "pg11", feature = "pg12"))]
mod node_list;

#[cfg(feature = "cshim")]
pub mod old_list;

#[cfg(feature = "cshim")]
pub use old_list::*;
