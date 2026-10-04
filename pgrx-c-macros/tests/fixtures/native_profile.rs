use __pgrx_c_macros::{CChar, CInteger, CLong, CLongLong, CSize, CValue};

/// Record original C identity and sign-extended bits so equal widths cannot hide rank mistakes.
fn record<T: __pgrx_c_macros::IntoCValue>(name: &str, value: T) {
    let value = value.into_c_value();
    let kind = std::any::type_name::<T::Kind>().rsplit("::").next().unwrap();
    println!(
        "{name}\t{kind}\t{}\t{}\t{}\t{:032x}",
        T::Kind::BITS,
        u8::from(T::Kind::SIGNED),
        T::Kind::RANK,
        T::Kind::encode(value.get()),
    );
}

/// Compare promotions, preferred scalar/array alignment, pointer differences, and mutations
/// with original C, retaining explicit C identity where native integer storage is ambiguous.
fn main() {
    for value in 0_u32..256 {
        record("char", PROFILE_CHAR!(value));
        record("char_add", PROFILE_CHAR_ADD!(value));
    }
    // SAFETY: The direct function and the non-null callback pointer are supplied by the same
    // original C translation unit under the inspected profile. They accept every char bit
    // pattern, retain no pointer or reference, and only return the supplied scalar value.
    unsafe {
        let callback = PROFILE_CHAR_CALLBACK_GET!();
        for value in 0_u32..256 {
            record("char_call", PROFILE_CHAR_CALL!(value));
            record("char_callback_call", PROFILE_CHAR_CALLBACK_CALL!(callback, value));
        }
    }
    record("long_unsigned_int", PROFILE_LONG_UINT!(-2_i32));
    record("unsigned_long_long_long", PROFILE_ULONG_LLONG!(-1_i32));
    record("size", PROFILE_SIZE!(-1_i32));
    record("sizeof", PROFILE_SIZE_OF!(PROFILE_CHAR!(255_i32)));
    record("alignment_long_long", PROFILE_ALIGNMENT!(CValue<CLongLong>));
    record("alignment_long_long_array", PROFILE_ALIGNMENT!([CValue<CLongLong>; 2]));
    record("alignment_long_long_nested_array", PROFILE_ALIGNMENT!([[CValue<CLongLong>; 2]; 2]));
    record("alignment_double", PROFILE_ALIGNMENT!(f64));
    record("alignment_double_array", PROFILE_ALIGNMENT!([f64; 2]));
    record("alignment_double_nested_array", PROFILE_ALIGNMENT!([[f64; 2]; 2]));
    record("alignment_typed_long_long", PROFILE_ALIGNMENT_LONG_LONG!());
    record("alignment_typed_long_long_array", PROFILE_ALIGNMENT_LONG_LONG_ARRAY!());
    record("alignment_typed_long_long_nested_array", PROFILE_ALIGNMENT_LONG_LONG_NESTED_ARRAY!());
    record("alignment_typed_double", PROFILE_ALIGNMENT_DOUBLE!());
    record("alignment_typed_double_array", PROFILE_ALIGNMENT_DOUBLE_ARRAY!());
    record("alignment_typed_double_nested_array", PROFILE_ALIGNMENT_DOUBLE_NESTED_ARRAY!());
    let values = [1_i32, 2, 3, 4];
    let mut character = 0 as <CChar as CInteger>::Repr;
    let mut signed_long = 0 as <CLong as CInteger>::Repr;
    let mut size = 0_usize;
    let mut item = ProfileRecord { character: 0, signed_long: 0, count: 0 };
    // SAFETY: Both pointers used for subtraction belong to the same live array. Mutation
    // operands point to initialized, aligned storage of the actual inspected C representation,
    // exclusively accessed here and retained until the generated operations complete.
    unsafe {
        record(
            "pointer_difference",
            PROFILE_POINTER_DIFF!(values.as_ptr().add(3), values.as_ptr()),
        );
        record("char_store", PROFILE_CHAR_STORE!(&raw mut character, 255_i32));
        record("char_storage", CValue::<CChar>::new(character));
        let long_pointer = __pgrx_c_macros::expression::Pointer::<CLong>::new(&raw mut signed_long);
        record("long_store", PROFILE_LONG_STORE!(long_pointer, -7_i32));
        record("long_storage", CValue::<CLong>::new(signed_long));
        record("size_store", PROFILE_SIZE_STORE!(&raw mut size, -1_i32));
        record("size_storage", CValue::<CSize>::new(size as _));
        for value in 0_u32..256 {
            record("record_char_store", PROFILE_RECORD_CHAR_STORE!(&raw mut item, value));
            record("record_char_storage", PROFILE_RECORD_CHAR!(&raw mut item));
        }
        record("record_long_store", PROFILE_RECORD_LONG_STORE!(&raw mut item, -7_i32));
        record("record_long_storage", PROFILE_RECORD_LONG!(&raw mut item));
        record("record_size_store", PROFILE_RECORD_SIZE_STORE!(&raw mut item, -1_i32));
        record("record_size_storage", PROFILE_RECORD_SIZE!(&raw mut item));
    }
}
