//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![cfg(not(target_os = "windows"))]

use extension::pg_sys;
use extension::pg_sys::__pgrx_c_macros::{CUnsignedInt, CValue};
use pgrx as extension;
use pgrx::{
    ACL_GRANT_OPTION_FOR as grant_options, BUFFERALIGN as buffer_align, Max as c_max, Min as c_min,
    TYPEALIGN as type_align,
};

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

#[test]
fn root_macro_reexports_keep_binding_alias_casts_in_the_defining_crate() {
    // A caller's same-named type must not replace the binding used by the cast.
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
