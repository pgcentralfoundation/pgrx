//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

fn main() {
    use __pgrx_c_macros::{CUnsignedInt, CUnsignedLong, CValue};
    let mut record = core::mem::MaybeUninit::<EnumRecord>::uninit();
    let pointer = record.as_mut_ptr();
    // SAFETY: Only the selected enum integer bytes are initialized and accessed.
    // Raw C storage intentionally contains unnamed enum values; no Rust enum
    // value or record reference is formed and the unread bool stays uninitialized.
    unsafe {
        core::ptr::addr_of_mut!((*pointer).small).cast::<EnumSmallStorage>().write(7);
        core::ptr::addr_of_mut!((*pointer).signed_value).cast::<EnumSignedStorage>().write(-17);
        core::ptr::addr_of_mut!((*pointer).wide).cast::<u64>().write(0x8000000000000001);
        println!("read\t{}", ENUM_READ!(pointer).get() as u64);
        println!("set\t{}", ENUM_SET!(pointer, 129).get() as u64);
        println!("add\t{}", ENUM_ADD!(pointer, 2).get() as u64);
        println!("post\t{}", ENUM_POST!(pointer).get() as u64);
        println!("prefix\t{}", ENUM_PREFIX!(pointer).get() as u64);
        println!("cast\t{}", ENUM_CAST!(253).get() as u64);
        println!("not\t{}", ENUM_NOT!(pointer).get() as u64);
        println!("size\t{}", ENUM_SIZE!(pointer).get());
        println!(
            "address\t{}",
            u8::from(ENUM_ADDRESS!(pointer).get() == core::ptr::addr_of_mut!((*pointer).small))
        );
        println!("indirect\t{}", ENUM_INDIRECT!(pointer).get() as u64);
        println!("volatile\t{}", ENUM_VOLATILE!(pointer).get() as u64);
        println!("volatileset\t{}", ENUM_VOLATILE_SET!(pointer, 251).get() as u64);
        println!("signed\t{}", ENUM_SIGNED!(pointer).get() as i64);
        println!("signedset\t{}", ENUM_SIGNED_SET!(pointer, -123).get() as i64);
        println!("wide\t{}", ENUM_WIDE!(pointer).get() as u64);
        println!(
            "wideset\t{}",
            ENUM_WIDE_SET!(pointer, CValue::<CUnsignedLong>::new(0xFFFFFFFFFFFFFFFE)).get() as u64
        );
        let mut allocation = core::mem::MaybeUninit::<[u64; 2]>::uninit();
        let packed = allocation.as_mut_ptr().cast::<PackedEnumRecord>();
        core::ptr::addr_of_mut!((*packed).small).cast::<EnumSmallStorage>().write_unaligned(231);
        println!("packed\t{}", ENUM_PACKED!(packed).get() as u64);
        println!("packedset\t{}", ENUM_PACKED_SET!(packed, 229).get() as u64);
        println!("valid\t{}", ENUM_IDENTITY!(EnumSmall::SmallOne).get() as u64);
        println!("inline\t{}", ENUM_INLINE!(254).get() as u64);
        let mut keyword = core::mem::MaybeUninit::<EnumKeywordRecord>::uninit();
        let keyword = keyword.as_mut_ptr();
        core::ptr::addr_of_mut!((*keyword).type_).cast::<EnumSmallStorage>().write(17);
        core::ptr::addr_of_mut!((*keyword).u32_).write(21);
        println!("keyword\t{}", ENUM_KEYWORD!(keyword).get() as u64);
        println!("keywordset\t{}", ENUM_KEYWORD_SET!(keyword, 19).get() as u64);
        println!("primitivename\t{}", ENUM_PRIMITIVE_NAME!(keyword).get());
        println!("constant\t{}", ENUM_CONSTANT!(2).get() as u64);
        println!("constantuse\t{}", ENUM_CONSTANT_USE!(2).get() as i64);
        println!("constantzero\t{}", ENUM_CONSTANT_ZERO!(core::ptr::null_mut::<EnumSmall>()).get());
        println!(
            "compare\t{}",
            ENUM_COMPARE!(
                pointer,
                core::ptr::addr_of_mut!((*pointer).small).cast::<EnumSmallStorage>()
            )
            .get()
        );
        // Only addresses are compared/subtracted. All pointers retain one array's
        // provenance, including its one-past endpoint; enum fields are never read.
        let mut values = core::mem::MaybeUninit::<[EnumSmall; 4]>::uninit();
        let first = values.as_mut_ptr().cast::<EnumSmall>();
        let end = first.add(4).cast::<EnumSmallStorage>();
        println!("orderforward\t{}", ENUM_ORDER!(first, end).get());
        println!("orderreverse\t{}", ENUM_ORDER!(end, first).get());
        println!("orderequal\t{}", ENUM_ORDER!(first, first.cast::<EnumSmallStorage>()).get());
        println!("orderqualified\t{}", ENUM_ORDER!(ENUM_QUALIFIED_POINTER!(first.add(1)), end).get());
        println!("differenceforward\t{}", ENUM_DIFFERENCE!(end, first).get());
        println!("differencereverse\t{}", ENUM_DIFFERENCE!(first, end).get());
        println!("differencezero\t{}", ENUM_DIFFERENCE!(first, first.cast::<EnumSmallStorage>()).get());
        println!("differencequalified\t{}", ENUM_DIFFERENCE!(ENUM_QUALIFIED_POINTER!(first.add(1)), end).get());
        let mut integer_values = [CValue::<CUnsignedInt>::new(0); 4];
        let stored_first = integer_values.as_mut_ptr();
        let primitive_end = stored_first.add(4).cast::<u32>();
        println!("storedifference\t{}", ENUM_DIFFERENCE!(primitive_end, stored_first).get());
        println!("storereversedifference\t{}", ENUM_DIFFERENCE!(stored_first, primitive_end).get());
        for i in 0..256_i32 {
            println!("boundary{i}\t{}", ENUM_SET!(pointer, i).get() as u64);
        }
    }
    use __pgrx_c_macros::expression::{CType, NativeType, cast, input};
    type Marker = <EnumSmall as NativeType>::Marker;
    let zero = Marker::into_storage(cast::<Marker, _>(input(0)));
    assert!(matches!(zero, EnumSmall::SmallZero));
    std::panic::set_hook(Box::new(|_| {}));
    assert!(
        std::panic::catch_unwind(|| Marker::into_storage(cast::<Marker, _>(input(253)))).is_err()
    );
}
