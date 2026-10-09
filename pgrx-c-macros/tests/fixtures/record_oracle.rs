//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

/// Standalone Rust consumer for the record oracle C comparison.
///
/// The harness appends this consumer after generated bindings, semantic support,
/// and macro definitions. Its observations preserve types and operand effects
/// for comparison with the original C header; helpers instrument those effects
/// without replacing any C macro definition.
///
/// Exercise the generated definitions and print observations for the paired original-C oracle;
/// assertions cover cases with no scalar output.
fn main() {
    let mut allocation = core::mem::MaybeUninit::<[u64; 8]>::uninit();
    let anonymous = allocation.as_mut_ptr().cast::<AnonymousRecord>();
    // SAFETY: These live allocations have sufficient size/alignment. Only the
    // selected C fields/elements are initialized and accessed. Raw projection
    // never forms references or reads the unread bool and inactive union data.
    unsafe {
        let mut cast_only = core::mem::MaybeUninit::<CastOnlyRecord>::uninit();
        let cast_only = cast_only.as_mut_ptr();
        core::ptr::addr_of_mut!((*cast_only).first).write(61);
        let copied = RECORD_CAST_ONLY!(cast_only.cast::<core::ffi::c_void>()).get();
        println!("castonly\t{}", core::ptr::addr_of!((*copied.as_ptr()).first).read());
        println!(
            "volatilecastfield\t{}",
            RECORD_VOLATILE_CAST_FIELD!(cast_only.cast::<core::ffi::c_void>()).get()
        );
        println!(
            "volatilewholesize\t{}",
            RECORD_VOLATILE_WHOLE_SIZE!(cast_only.cast::<core::ffi::c_void>()).get()
        );
        println!(
            "volatileassignmentsize\t{}",
            RECORD_VOLATILE_ASSIGNMENT_SIZE!(
                core::ptr::null_mut::<core::ffi::c_void>(),
                core::mem::MaybeUninit::<CastOnlyRecord>::uninit()
            )
            .get()
        );
        println!(
            "volatilewholeaddress\t{}",
            u8::from(
                RECORD_VOLATILE_WHOLE_ADDRESS!(cast_only.cast::<core::ffi::c_void>()).get()
                    == cast_only
            )
        );
        let expanded =
            core::ptr::addr_of_mut!((*anonymous).expanded).cast::<AnonymousRecord__bindgen_ty_1>();
        core::ptr::addr_of_mut!((*expanded).header).write(17);
        core::ptr::addr_of_mut!((*expanded).bytes).cast::<i8>().add(1).write(65);
        println!("header\t{}", RECORD_HEADER!(anonymous).get());
        println!("byte\t{}", RECORD_BYTE!(anonymous, 1).get());
        println!("setbyte\t{}", RECORD_SET_BYTE!(anonymous, 1, 258).get());
        println!(
            "address\t{}",
            u8::from(
                RECORD_HEADER_ADDRESS!(anonymous).get()
                    == core::ptr::addr_of_mut!((*expanded).header)
            )
        );
        println!("size\t{}", RECORD_CHILD_SIZE!(anonymous).get());
        println!("anonymous\t{}", u8::from(RECORD_ANON_INLINE!(anonymous).get() == anonymous));
        let mut promoted = core::mem::MaybeUninit::<PromotedRecord>::uninit();
        let promoted = promoted.as_mut_ptr();
        let inner = core::ptr::addr_of_mut!((*promoted).__bindgen_anon_1.__bindgen_anon_1)
            .cast::<PromotedRecord__bindgen_ty_1__bindgen_ty_1>();
        core::ptr::addr_of_mut!((*inner).promoted).write(31);
        println!("promoted\t{}", RECORD_PROMOTED!(promoted).get());
        println!("setpromoted\t{}", RECORD_SET_PROMOTED!(promoted, 65539).get());
        let flexible = allocation.as_mut_ptr().cast::<FlexibleRecord>();
        let items = core::ptr::addr_of_mut!((*flexible).items).cast::<u32>();
        items.add(2).write(53);
        println!("flex\t{}", RECORD_FLEX!(flexible, 2).get());
        println!("setflex\t{}", RECORD_SET_FLEX!(flexible, 2, 99).get());
        println!("flexaddress\t{}", u8::from(RECORD_FLEX_ADDRESS!(flexible).get() == items));
        println!(
            "flexarrayaddress\t{}",
            u8::from(
                RECORD_FLEX_ARRAY_ADDRESS!(flexible).get()
                    == core::ptr::addr_of_mut!((*flexible).items)
            )
        );
        println!(
            "flexarrayequal\t{}",
            RECORD_FLEX_ARRAY_EQUAL!(flexible, items.cast::<[u32; 3]>()).get()
        );
        println!(
            "flexarrayvoid\t{}",
            u8::from(RECORD_FLEX_ARRAY_VOID!(flexible).get() == items.cast())
        );
        /// Represent an opaque C pointee identity without pretending its record layout is
        /// available.
        type Incomplete = __pgrx_c_macros::expression::CFlexibleArray<
            __pgrx_c_macros::CUnsignedInt,
            __IncompleteArrayField<u32>,
        >;
        let incomplete = __pgrx_c_macros::expression::implicit::<
            __pgrx_c_macros::expression::CPointer<Incomplete>,
            _,
        >(__pgrx_c_macros::expression::input(items.cast::<[u32; 3]>()));
        let completed = __pgrx_c_macros::expression::implicit::<
            __pgrx_c_macros::expression::CPointer<
                __pgrx_c_macros::expression::CArray<__pgrx_c_macros::CUnsignedInt, 3>,
            >,
            _,
        >(incomplete);
        println!("flexarrayconvert\t{}", u8::from(completed.get() == items.cast::<[u32; 3]>()));
        let opaque = allocation.as_mut_ptr().cast::<OpaqueRecord>();
        println!("opaque\t{}", u8::from(RECORD_OPAQUE!(opaque).get() == opaque));
        println!("opaqueinline\t{}", u8::from(RECORD_OPAQUE_INLINE!(opaque).get() == opaque));
        println!("compare\t{}", RECORD_OPAQUE_COMPARE!(opaque, opaque).get());
        println!(
            "void\t{}",
            u8::from(RECORD_OPAQUE_VOID!(opaque).get() == allocation.as_mut_ptr().cast())
        );
        let array = [7_u32, 11, 19];
        println!("array\t{}", RECORD_ARRAY!(&raw const array, 2).get());
        let mut signal = core::mem::MaybeUninit::<VolatilePromotedRecord>::uninit();
        let signal = signal.as_mut_ptr();
        let raw = core::ptr::addr_of_mut!((*signal).__bindgen_anon_1.signal);
        raw.write(29);
        println!("volatile\t{}", RECORD_VOLATILE_READ!(signal).get());
        println!("volatileindirect\t{}", RECORD_VOLATILE_INDIRECT!(signal).get());
        let qualified = RECORD_VOLATILE_ADDRESS!(signal);
        /// Keep a volatile-qualified operand in the consumer so generated accesses must retain
        /// their qualification.
        fn keep_volatile(
            _: __pgrx_c_macros::expression_result::CExpression<
                __pgrx_c_macros::expression::Pointer<
                    __pgrx_c_macros::expression::CVolatile<__pgrx_c_macros::CUnsignedInt>,
                >,
            >,
        ) {
        }
        keep_volatile(qualified);
        println!("volatileaddress\t{}", u8::from(qualified.get() == raw));
        let packed = allocation.as_mut_ptr().cast::<PackedArrayRecord>();
        let array = core::ptr::addr_of_mut!((*packed).array).cast::<u32>();
        array.add(1).write_unaligned(43);
        println!("packedarray\t{}", RECORD_PACKED_ARRAY!(packed, 1).get());
        println!("setpackedarray\t{}", RECORD_SET_PACKED_ARRAY!(packed, 1, 101).get());
        let packed_flex = allocation.as_mut_ptr().cast::<PackedFlexibleRecord>();
        let array = core::ptr::addr_of_mut!((*packed_flex).array).cast::<u32>();
        array.add(2).write_unaligned(47);
        println!("packedflex\t{}", RECORD_PACKED_FLEX!(packed_flex, 2).get());
    }
}
