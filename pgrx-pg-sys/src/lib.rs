//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#![allow(clippy::missing_safety_doc)]
// sorry, Jubilee
// Rust currently diagnoses absolute calls to macro_export definitions loaded
// through include!, including their hygienic $crate helper calls. Generated
// macros are intentionally used inside pg-sys as well as by downstream crates.
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

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
    //! Preserve fresh bindgen symbol and type identities across crate-root re-exports.

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
pub use node::PgNode;
// Preserve the public Rust port API's precedence over generated binding globs.
// Generated C expressions resolve bindings through __pgrx_c_bindings instead.
pub use port::{
    BootstrapTransactionId, BufferIsLocal, FirstCommandId, FirstNormalTransactionId,
    FirstOffsetNumber, FrozenTransactionId, GETSTRUCT, GetMemoryChunkContext, InvalidBlockNumber,
    InvalidCommandId, InvalidOffsetNumber, InvalidOid, InvalidTransactionId, MAXALIGN,
    MaxOffsetNumber, MaxTransactionId, MemoryContextIsValid, MemoryContextSwitchTo, PageIsValid,
    PageValidateSpecialPointer, SizeOfPageHeaderData, TYPEALIGN, TransactionIdIsNormal, VARHDRSZ,
    VARHDRSZ_EXTERNAL, VARHDRSZ_SHORT, get_pg_major_minor_version_string, get_pg_major_version_num,
    get_pg_major_version_string, heap_tuple_get_struct, type_is_array,
};

#[cfg(any(not(target_env = "msvc"), feature = "pg17", feature = "pg18", feature = "pg19"))]
pub use port::get_pg_version_string;
#[cfg(all(target_env = "msvc", any(feature = "pg15", feature = "pg16")))]
pub use port::get_pg_version_string;

#[cfg(feature = "pg19")]
pub use port::{
    TransactionIdFollows, TransactionIdFollowsOrEquals, TransactionIdPrecedes,
    TransactionIdPrecedesOrEquals,
};

#[cfg(any(
    feature = "pg15",
    all(
        not(feature = "cshim"),
        any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19")
    )
))]
pub use port::{BufferGetBlock, BufferGetPage};

#[cfg(feature = "pg15")]
pub use port::{
    expression_tree_walker, planstate_tree_walker, query_or_expression_tree_walker,
    query_tree_walker, range_table_entry_walker, range_table_walker, raw_expression_tree_walker,
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
pub use port::{
    expression_tree_walker, planstate_tree_walker, query_or_expression_tree_walker,
    query_tree_walker, range_table_entry_walker, range_table_walker, raw_expression_tree_walker,
};

#[cfg(any(feature = "pg18", feature = "pg19"))]
pub use port::expression_tree_mutator;

#[cfg(feature = "pg15")]
pub use port::{
    BufferGetPageSize, BufferIsValid, ItemIdGetOffset, PageGetContents, PageGetItem, PageGetItemId,
    PageGetMaxOffsetNumber, PageGetPageLayoutVersion, PageGetPageSize, PageGetSpecialSize,
    PageIsEmpty, PageIsNew, PageSetPageSizeAndVersion, PageSizeIsValid,
};

#[cfg(any(feature = "pg15", feature = "pg18", feature = "pg19"))]
pub use port::PageGetSpecialPointer;

// Keep the legacy tuple helper API ahead of newly generated inline bindings.
pub use submodules::{
    HeapTupleGetRawCommandId, HeapTupleHeaderFrozen, HeapTupleHeaderGetNatts,
    HeapTupleHeaderGetRawXmin, HeapTupleHeaderGetXmin, HeapTupleHeaderIsHeapOnly,
    HeapTupleHeaderIsHotUpdated, HeapTupleHeaderXminInvalid, HeapTupleNoNulls, heap_getattr,
};

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
