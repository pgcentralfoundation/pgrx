//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![cfg(all(pgrx_c_macros, not(docsrs)))]
#![deny(unsafe_op_in_unsafe_fn)]

use core::mem::MaybeUninit;
use core::ptr::{addr_of, addr_of_mut};
use pgrx_pg_sys as pg;

#[test]
fn generated_function_call_initialization_writes_only_the_documented_fields() {
    let mut storage = MaybeUninit::<pg::FunctionCallInfoBaseData>::uninit();
    let fcinfo = addr_of_mut!(storage).cast::<pg::FunctionCallInfoBaseData>();
    let flinfo = core::ptr::null_mut::<pg::FmgrInfo>();
    let context = core::ptr::null_mut::<pg::Node>();
    let result_info = core::ptr::null_mut::<pg::Node>();
    let collation = pg::Oid::from_u32(123);
    // SAFETY: fcinfo points into its owned, aligned live slot with exclusive
    // access. InitFunctionCallInfoData initializes exactly these six fields;
    // the incomplete args array is never accessed or materialized as a value.
    // The other arguments are typed null pointers or valid scalar storage.
    unsafe {
        let _: () =
            pg::InitFunctionCallInfoData!(*fcinfo, flinfo, 7_i32, collation, context, result_info);
        assert_eq!(addr_of!((*fcinfo).flinfo).read(), flinfo);
        assert_eq!(addr_of!((*fcinfo).context).read(), context);
        assert_eq!(addr_of!((*fcinfo).resultinfo).read(), result_info);
        assert_eq!(addr_of!((*fcinfo).fncollation).read().to_u32(), 123);
        assert!(!addr_of!((*fcinfo).isnull).read());
        assert_eq!(addr_of!((*fcinfo).nargs).read(), 7);
    }
}

#[test]
fn generated_object_address_statements_initialize_fields_and_clear_subid() {
    let mut storage = MaybeUninit::<pg::ObjectAddress>::uninit();
    let address = addr_of_mut!(storage).cast::<pg::ObjectAddress>();
    // SAFETY: address identifies one owned, aligned live slot. Both macros write
    // all three scalar fields without reading uninitialized bytes. Each read
    // below targets a field initialized by that preceding invocation. No record
    // reference or whole-record value is formed, and no backend function runs.
    unsafe {
        let _: () = pg::ObjectAddressSubSet!(
            *address,
            pg::Oid::from_u32(11),
            pg::Oid::from_u32(u32::MAX),
            -3_i32
        );
        assert_eq!(addr_of!((*address).classId).read().to_u32(), 11);
        assert_eq!(addr_of!((*address).objectId).read().to_u32(), u32::MAX);
        assert_eq!(addr_of!((*address).objectSubId).read(), -3);

        let _: () = pg::ObjectAddressSet!(*address, pg::Oid::from_u32(21), pg::Oid::from_u32(31));
        assert_eq!(addr_of!((*address).classId).read().to_u32(), 21);
        assert_eq!(addr_of!((*address).objectId).read().to_u32(), 31);
        assert_eq!(addr_of!((*address).objectSubId).read(), 0);
    }
}

#[test]
fn generated_checksum_round_matches_original_c_boundary_vectors() {
    // Evaluated with CHECKSUM_COMP from storage/checksum_impl.h under PG18's
    // recorded compiler profile. These are C results, not the handwritten pgrx port.
    let cases = [
        (0x00000000_u32, 0x00000000_u32, 0x00000000_u32),
        (0x00000000, 0x00000001, 0x01000193),
        (0x00000001, 0x00000000, 0x01000193),
        (0xFFFFFFFF, 0x00000000, 0xFEFF8192),
        (0x00000000, 0xFFFFFFFF, 0xFEFF8192),
        (0x80000000, 0x00000001, 0x81004193),
        (0x12345678, 0x9ABCDEF0, 0x76EEAA5C),
        (0xFFFFFFFF, 0xFFFFFFFF, 0x00000000),
    ];
    for (seed, word, expected) in cases {
        let mut checksum = seed;
        // SAFETY: checksum is an initialized, aligned exclusively writable local.
        // The generated temporary is initialized from scalar inputs and remains
        // local to this invocation. The unsigned arithmetic accepts all u32 values.
        unsafe { pg::CHECKSUM_COMP!(checksum, word) }
        assert_eq!(checksum, expected, "seed={seed:#010X}, word={word:#010X}");
    }
}
