//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use __pgrx_c_macros::expression::{Pointer, Qualifier, RawRecordValue};
use __pgrx_c_macros::{CInt, CInteger, CLong, CLongLong, CUnsignedLong, CUnsignedLongLong, CValue};

fn record<T: __pgrx_c_macros::IntoCValue>(name: &str, value: T) {
    let value = value.into_c_value();
    let kind = std::any::type_name::<T::Kind>().rsplit("::").next().unwrap();
    // SAFETY: These fixture getters only read native counters on this thread.
    let (trace, calls) = unsafe { (call_trace(), call_count()) };
    println!(
        "{name}\t{kind}\t{}\t{:032x}\t{trace}\t{calls}",
        T::Kind::BITS,
        T::Kind::encode(value.get())
    );
}
fn record_void<T: __pgrx_c_macros::expression::IntoExpression<Value = ()>>(name: &str, value: T) {
    let () = value.into_expression();
    // SAFETY: These fixture getters only read native counters on this thread.
    let (trace, calls) = unsafe { (call_trace(), call_count()) };
    println!("{name}\tCVoid\t0\t00000000000000000000000000000000\t{trace}\t{calls}");
}
fn record_pointer<
    Q: Qualifier,
    T: __pgrx_c_macros::expression::IntoExpression<Value = Pointer<CInt, Q>>,
>(
    name: &str,
    value: T,
    expected: *const i32,
) {
    let value = value.into_expression();
    let bits = u128::from(value.as_mut_address().cast_const() == expected);
    // SAFETY: These fixture getters only read native counters on this thread.
    let (trace, calls) = unsafe { (call_trace(), call_count()) };
    println!(
        "{name}\tCPointer\t{}\t{bits:032x}\t{trace}\t{calls}",
        std::mem::size_of::<*const i32>() * 8
    );
}
/// # Safety
/// The original fixture must have initialized every field of this CallRecord.
unsafe fn record_struct<
    T: __pgrx_c_macros::expression::IntoExpression<Value = RawRecordValue<CallRecord>>,
>(
    name: &str,
    value: T,
) {
    // SAFETY: The caller establishes the full fixture record invariant; the
    // original call_record initializes both fields before its C return.
    let value = unsafe { value.into_expression().assume_initialized() };
    let bits = (u128::from(value.small) << 64) | u128::from(value.wide as u32);
    // SAFETY: These fixture getters only read native counters on this thread.
    let (trace, calls) = unsafe { (call_trace(), call_count()) };
    println!(
        "{name}\tCRecord\t{}\t{bits:032x}\t{trace}\t{calls}",
        std::mem::size_of::<CallRecord>() * 8
    );
}

fn main() {
    // SAFETY: The C fixtures operate on local scalars and initialized Copy records.
    // Pointers refer to the live, aligned local `value`; each mutation is complete
    // before it is read again. No pointer or allocation escapes these native calls.
    unsafe {
        call_reset();
        record("byte_negative", CALL_BYTE!(-1_i32));
        call_reset();
        record("byte_narrow", CALL_BYTE!(0x1234_i32));
        call_reset();
        record("byte_wrap", CALL_BYTE!(256_u32));
        call_reset();
        record("byte_identity", CALL_BYTE!(255_u8));
        call_reset();
        record("short_negative", CALL_SHORT!(-1_i32));
        call_reset();
        record("short_narrow", CALL_SHORT!(0x12345_u32));
        call_reset();
        record("int_from_byte", CALL_INT!(255_u8));
        call_reset();
        record("int_negative", CALL_INT!(-19_i32));
        call_reset();
        record("long_from_int", CALL_LONG!(-1_i32));
        call_reset();
        record("long_identity", CALL_LONG!(CValue::<CLong>::new(-31)));
        call_reset();
        record("ulong_negative", CALL_ULONG!(-1_i32));
        call_reset();
        record("ulong_identity", CALL_ULONG!(CValue::<CUnsignedLong>::new(0xFEDCBA)));
        call_reset();
        record("llong_identity", CALL_LLONG!(CValue::<CLongLong>::new(-41)));
        call_reset();
        record("llong_from_uint", CALL_LLONG!(0xFFFFFFFF_u32));
        call_reset();
        record("ull_negative", CALL_ULL!(-1_i32));
        call_reset();
        record("ull_identity", CALL_ULL!(CValue::<CUnsignedLongLong>::new(0xFEDCBA9876543210)));
        call_reset();
        record("bool_zero", CALL_BOOL!(0_i32));
        call_reset();
        record("bool_negative", CALL_BOOL!(-3_i32));
        call_reset();
        record("bool_before_narrow", CALL_BOOL!(256_u32));
        let mut value = 37_i32;
        call_reset();
        record("bool_pointer", CALL_BOOL!((&mut value as *mut i32)));
        call_reset();
        record("bool_null_pointer", CALL_BOOL!(std::ptr::null::<i32>()));
        call_reset();
        record("read_const", CALL_READ!((&value as *const i32)));
        call_reset();
        record("read_mutable", CALL_READ!((&mut value as *mut i32)));
        call_reset();
        record("write", CALL_WRITE!((&mut value as *mut i32), -7_i32));
        record("write_observed", CValue::<CInt>::new(value));
        call_reset();
        record_void("void", CALL_VOID!(-8_i32));
        call_reset();
        record("lazy_yes", CALL_LAZY!(1_i32, -1_i32, 99_i32));
        call_reset();
        record("lazy_no", CALL_LAZY!(0_i32, -1_i32, 99_i32));
        call_reset();
        record_void("lazy_void_yes", CALL_LAZY_VOID!(1_i32, -1_i32, 99_i32));
        call_reset();
        record_void("lazy_void_no", CALL_LAZY_VOID!(0_i32, -1_i32, 99_i32));
        call_reset();
        record("comma", CALL_COMMA!(-2_i32, 5_i32));
        call_reset();
        record("repeat", CALL_REPEAT!(-3_i32));
        call_reset();
        record("repeat_argument", CALL_REPEAT!(call_int(-3_i32)));
        call_reset();
        record("comma_argument", CALL_COMMA!(call_int(-2_i32), call_int(5_i32)));
        call_reset();
        record("lazy_argument", CALL_LAZY!(call_int(1_i32), call_int(-2_i32), call_int(5_i32)));
        call_reset();
        record("nested", CALL_NESTED!(0x12345_i32));
        call_reset();
        record("sequence", CALL_SEQUENCE!(-9_i32));
        call_reset();
        record_void("void_discard", CALL_VOID_DISCARD!(-2_i32));
        call_reset();
        record_pointer("pointer_mutable", CALL_POINTER!((&mut value as *mut i32)), &value);
        call_reset();
        record_pointer("pointer_const", CALL_CONST_POINTER!((&value as *const i32)), &value);
        call_reset();
        record_struct("record", CALL_RECORD!(0x12345_u32, -71_i32));
        let record_value = CallRecord { small: 13, wide: -7 };
        call_reset();
        record("record_by_value", CALL_RECORD_SUM!(record_value));
        call_reset();
        record("record_nested", CALL_RECORD_NESTED!(0x12345_u32, -71_i32));
        call_reset();
        record("record_field", CALL_RECORD_FIELD!(0x12345_u32, -71_i32));
    }
}
