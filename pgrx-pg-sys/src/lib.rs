//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#![allow(clippy::missing_safety_doc)] // sorry, Jubilee

#[cfg(
    // no features at all will cause problems
    not(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))
)]
std::compile_error!("exactly one feature must be provided (pg15, pg16, pg17, pg18, pg19)");

/// C expression semantics and storage capabilities used by generated PostgreSQL macros.
#[doc(hidden)]
#[path = "c_macros/support.rs"]
pub mod __pgrx_c_macros;

/// Public paths to the selected generated bindings, isolated from handwritten
/// crate-root helpers so macro expansions use the declarations they verified.
/// This namespace is public only because exported macros expand in other crates.
#[doc(hidden)]
pub mod __pgrx_c_bindings {
    #[cfg(feature = "pg15")]
    pub use crate::include::pg15::*;
    #[cfg(feature = "pg16")]
    pub use crate::include::pg16::*;
    #[cfg(feature = "pg17")]
    pub use crate::include::pg17::*;
    #[cfg(feature = "pg18")]
    pub use crate::include::pg18::*;
    #[cfg(feature = "pg19")]
    pub use crate::include::pg19::*;
    pub use crate::{Datum, MultiXactId, Oid, PgNode, TransactionId};
}

mod cshim;
mod cstr;
mod include;
mod node;
mod port;

pub mod libpq;
pub mod submodules;

// on pg19, `cshim` currently has nothing to re-export, so the glob import is "unused"
#[allow(unused_imports)]
#[cfg(feature = "cshim")]
pub use cshim::*;

pub use cstr::AsPgCStr;
pub use include::*;
pub use node::{NotNodeTag, PgNode};
pub use port::*;

// PostgreSQL defines these constants as casts to `Oid`, `RelFileNumber`,
// `TransactionId` or `MultiXactId`, and the generated bindings declare them as
// `u32`. These explicit items shadow that glob re-export with pgrx's wrapper
// types. Generated macros still read the `u32` bindings through
// `__pgrx_c_bindings`.

/// `InvalidOid` from `postgres_ext.h`.
pub const InvalidOid: Oid = Oid::INVALID;
/// `InvalidRelFileNumber` from `common/relpath.h`.
#[cfg(not(feature = "pg15"))]
pub const InvalidRelFileNumber: RelFileNumber = InvalidOid;
/// `InvalidTransactionId` from `access/transam.h`.
pub const InvalidTransactionId: TransactionId = TransactionId::INVALID;
/// `BootstrapTransactionId` from `access/transam.h`.
pub const BootstrapTransactionId: TransactionId = TransactionId::BOOTSTRAP;
/// `FrozenTransactionId` from `access/transam.h`.
pub const FrozenTransactionId: TransactionId = TransactionId::FROZEN;
/// `FirstNormalTransactionId` from `access/transam.h`.
pub const FirstNormalTransactionId: TransactionId = TransactionId::FIRST_NORMAL;
/// `MaxTransactionId` from `access/transam.h`.
pub const MaxTransactionId: TransactionId = TransactionId::MAX;
/// `InvalidMultiXactId` from `access/multixact.h`.
pub const InvalidMultiXactId: MultiXactId = MultiXactId::from_inner(include::InvalidMultiXactId);
/// `FirstMultiXactId` from `access/multixact.h`.
pub const FirstMultiXactId: MultiXactId = MultiXactId::from_inner(include::FirstMultiXactId);
/// `MaxMultiXactId` from `access/multixact.h`.
pub const MaxMultiXactId: MultiXactId = MultiXactId::from_inner(include::MaxMultiXactId);

// For postgres 18+, some functions will reexport when enabling `cshim` feature
#[allow(ambiguous_glob_reexports)]
pub use submodules::*;

mod seal {
    pub trait Sealed {}
}

// Hack to fix linker errors that we get under amazonlinux2 on some PG versions
// due to our wrappers for various system library functions. Should be fairly
// harmless, but ideally we would not wrap these functions
// (https://github.com/pgcentralfoundation/pgrx/issues/730).
#[cfg(target_os = "linux")]
#[link(name = "resolv")]
unsafe extern "C" {}
