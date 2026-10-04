//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise generated macros through the pgrx crate's public exports.
//!
//! Renamed crates, imported macro names, caller-local aliases, and checked integer
//! wrappers stress hygiene across the reexport boundary. Results are compared
//! with the same build's pg_sys exports so nested calls and symbolic casts stay
//! anchored to their defining crate.

#![cfg(not(target_os = "windows"))]

/// Exercise the selected build's public bindings and macro exports from a downstream consumer.
use extension::pg_sys;
/// Use the production semantic markers and expression wrappers to retain exact C type identity
/// in the consumer.
use extension::pg_sys::__pgrx_c_macros::{CUnsignedInt, CValue};
/// Rename the consumer crate to test that generated macro paths remain hygienic across pgrx
/// reexports.
use pgrx as extension;
/// Import public macro aliases to test invocation through the pgrx crate root.
use pgrx::{
    ACL_GRANT_OPTION_FOR as grant_options, BUFFERALIGN as buffer_align, Max as c_max, Min as c_min,
    TYPEALIGN as type_align,
};

/// Checks nested alignment calls remain hygienic through a renamed pgrx crate and imported
/// macro aliases.
#[test]
fn renamed_pgrx_and_imported_macros_keep_nested_calls_in_the_defining_crate() {
    for size in [0_i32, 1, 33, 63, 64, 129] {
        let expected = pg_sys::BUFFERALIGN!(size).get();
        assert_eq!(extension::BUFFERALIGN!(size).get(), expected);
        assert_eq!(buffer_align!(size).get(), expected);
        assert_eq!(extension::TYPEALIGN!(pg_sys::ALIGNOF_BUFFER, size).get(), expected);
        assert_eq!(type_align!(pg_sys::ALIGNOF_BUFFER, size).get(), expected);
    }
}

/// Checks that a caller's AclMode alias cannot capture symbolic casts through root exports or
/// renamed macro imports.
#[test]
fn root_macro_reexports_keep_binding_alias_casts_in_the_defining_crate() {
    // A caller's same-named type must not replace the binding used by the cast.
    /// Shadow the binding alias in caller scope to prove generated casts stay anchored to the
    /// defining pg_sys crate.
    type AclMode = u8;
    let caller_privileges: AclMode = 0x34;
    for privileges in [u32::from(caller_privileges), u32::MAX] {
        let expected: pg_sys::AclMode = pg_sys::ACL_GRANT_OPTION_FOR!(privileges).get();
        let from_root: pg_sys::AclMode = extension::ACL_GRANT_OPTION_FOR!(privileges).get();
        let from_import: pg_sys::AclMode = grant_options!(privileges).get();
        assert_eq!(from_root, expected);
        assert_eq!(from_import, expected);
        assert_eq!(
            extension::BUFFERALIGN!(extension::ACL_GRANT_OPTION_FOR!(privileges)).get(),
            pg_sys::BUFFERALIGN!(pg_sys::ACL_GRANT_OPTION_FOR!(privileges)).get(),
        );
    }
}

/// Checks checked-wrapper inputs preserve C integer capabilities through pgrx root reexports
/// and imported min/max names.
#[test]
fn root_macro_reexports_preserve_oid_and_transaction_id_capabilities() {
    for oid in [pg_sys::Oid::INVALID, pg_sys::Oid::from_u32(u32::MAX)] {
        let expected: CValue<CUnsignedInt> = pg_sys::Max!(oid, 0_u32).into_value();
        let from_root: CValue<CUnsignedInt> = extension::Max!(oid, 0_u32).into_value();
        let from_import: CValue<CUnsignedInt> = c_max!(oid, 0_u32).into_value();
        assert_eq!(from_root.get(), expected.get());
        assert_eq!(from_import.get(), expected.get());
    }
    for xid in [pg_sys::TransactionId::INVALID, pg_sys::TransactionId::FIRST_NORMAL] {
        let expected: CValue<CUnsignedInt> = pg_sys::Min!(xid, 42_u32).into_value();
        let from_root: CValue<CUnsignedInt> = extension::Min!(xid, 42_u32).into_value();
        let from_import: CValue<CUnsignedInt> = c_min!(xid, 42_u32).into_value();
        assert_eq!(from_root.get(), expected.get());
        assert_eq!(from_import.get(), expected.get());
        assert_eq!(
            extension::TransactionIdIsNormal!(xid).get(),
            pg_sys::TransactionIdIsNormal!(xid).get(),
        );
    }
}
