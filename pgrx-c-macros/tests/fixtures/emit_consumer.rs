use renamed_generated::__pgrx_c_macros::{
    CBool, CInt, CLong, CLongLong, CShort, CUnsignedChar, CUnsignedInt, CUnsignedLong,
    CUnsignedLongLong, CUnsignedShort, CValue,
};

// A caller's identically named module and helpers must not capture generated bindings.
mod __pgrx_c_macros {
    pub fn value(_: u8) -> ! {
        panic!("caller-local helper must remain unused")
    }
}

fn main() {
    let _: CValue<CBool> = renamed_generated::EMIT_ID!(true);
    let identity: CValue<CUnsignedChar> = renamed_generated::EMIT_ID!(255_u8);
    assert_eq!(identity.get(), 255);
    let short: CValue<CShort> = renamed_generated::EMIT_ID!(-7_i16);
    assert_eq!(short.get(), -7);
    let promoted: CValue<CInt> = renamed_generated::EMIT_ADD!(255_u8, 1_u8);
    assert_eq!(promoted.get(), 256);
    let unsigned: CValue<CUnsignedInt> = renamed_generated::EMIT_ADD!(-1_i32, 1_u32);
    assert_eq!(unsigned.get(), 0);
    let distinct_rank: CValue<CUnsignedLongLong> = renamed_generated::EMIT_ADD!(
        CValue::<CUnsignedLong>::new(u64::MAX),
        CValue::<CLongLong>::new(0),
    );
    assert_eq!(distinct_rank.get(), u64::MAX);
    let _: CValue<CLong> = renamed_generated::EMIT_LONG!(unused_unknown_binding);
    let maximum: CValue<CUnsignedLongLong> =
        renamed_generated::EMIT_UNSIGNED_LONG_LONG!(unused_unknown_binding);
    assert_eq!(maximum.get(), u64::MAX);

    let narrow: CValue<CUnsignedShort> = renamed_generated::EMIT_WORD!(-1_i32);
    assert_eq!(narrow.get(), u16::MAX);
    let truth: CValue<CBool> = renamed_generated::EMIT_BOOL!(256_i32);
    assert!(truth.get());
    let _: CValue<CInt> = renamed_generated::EMIT_COMPARE!(-1_i32, 1_u32);
    assert_eq!(renamed_generated::EMIT_COMPARE!(-1_i32, 1_u32).get(), 0);
    assert_eq!(renamed_generated::EMIT_NOT!(0_u8).get(), 1);
    assert_eq!(renamed_generated::EMIT_PLUS!(255_u8).get(), 255);
    assert_eq!(renamed_generated::EMIT_NEGATE!(i32::MIN).get(), i32::MIN);
    assert_eq!(renamed_generated::EMIT_DIVIDE!(-7_i32, 3_i32).get(), -2);
    assert_eq!(renamed_generated::EMIT_REMAINDER!(-7_i32, 3_i32).get(), -1);
    let shifted: CValue<CLong> = renamed_generated::EMIT_SHIFT!(
        CValue::<CLong>::new(1),
        CValue::<CUnsignedLongLong>::new(8),
    );
    assert_eq!(shifted.get(), 256);
    assert_eq!(renamed_generated::EMIT_SHIFT_RIGHT!(-4_i32, 1_i32).get(), -2);
    assert_eq!(renamed_generated::EMIT_BITS!(1_u8, 0_u8).get(), -2);
    assert_eq!(renamed_generated::EMIT_KEYWORDS!(2_u8, 3_i16).get(), 5);
    assert_eq!(renamed_generated::r#match!(3_i32).get(), 4);
    assert_eq!(renamed_generated::EMIT_CONSTANT!(unused_unknown_binding).get(), 7);
    assert_eq!(renamed_generated::EMIT_UNUSED!(unused_unknown_binding).get(), 17);

    let mut calls = 0;
    let mut read = || {
        calls += 1;
        3_u8
    };
    assert_eq!(renamed_generated::EMIT_REPEAT!(read()).get(), 6);
    assert_eq!(calls, 2);
    let mut unused_calls = 0;
    assert_eq!(
        renamed_generated::EMIT_UNUSED!({
            unused_calls += 1;
            0_f64
        })
        .get(),
        17
    );
    assert_eq!(unused_calls, 0);

    let mut lazy_calls = 0;
    assert_eq!(
        renamed_generated::EMIT_LOGICAL_AND!(0_i32, {
            lazy_calls += 1;
            9_i32
        })
        .get(),
        0
    );
    assert_eq!(
        renamed_generated::EMIT_LOGICAL_OR!(1_i32, {
            lazy_calls += 1;
            9_i32
        })
        .get(),
        1
    );
    assert_eq!(lazy_calls, 0);
    assert_eq!(
        renamed_generated::EMIT_LOGICAL_AND!(1_i32, {
            lazy_calls += 1;
            9_i32
        })
        .get(),
        1
    );
    assert_eq!(lazy_calls, 1);

    // Both arms determine the result type; the unchosen arm remains unevaluated.
    // Mutating the same counter in both arms also catches closure-capture lowering.
    let mut chosen_calls = 0;
    let chosen: CValue<CUnsignedInt> = renamed_generated::EMIT_CHOOSE!(
        1_i32,
        {
            chosen_calls += 1;
            -1_i32
        },
        {
            chosen_calls += 100;
            1_u32
        },
    );
    assert_eq!(chosen.get(), u32::MAX);
    assert_eq!(chosen_calls, 1);
    let promoted_arm: CValue<CInt> = renamed_generated::EMIT_CHOOSE!(0_i32, 2_u8, 3_u8);
    assert_eq!(promoted_arm.get(), 3);

    // The module remains a meaningful shadowing fixture without warning suppression.
    let _ = __pgrx_c_macros::value as fn(u8) -> !;
}
