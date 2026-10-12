use __pgrx_c_macros::{CUnsignedLong, IntoCValue, cast};

/// Print the observation format consumed by the paired oracle, retaining C kind and value
/// information rather than only the result.
fn record<T: IntoCValue>(name: &str, value: T) {
    let value = cast::<CUnsignedLong, _>(value.into_c_value());
    // SAFETY: The fixture counter belongs to this process and requires no backend.
    println!("{name}:{}:{}", value.get(), unsafe { null_count() });
}

/// Label pointer categories for oracle output while leaving the generated conversion contract
/// unchanged.
trait OraclePointerType {
    /// Observation tag distinguishing this C pointer category from equal-sized Rust storage.
    const KIND: &'static str;
}
/// Tag this pointer representation for original-C type observations without dereferencing it.
impl OraclePointerType for *mut core::ffi::c_void {
    /// Observation tag distinguishing this C pointer category from equal-sized Rust storage.
    const KIND: &'static str = "void_pointer";
}
/// Tag this pointer representation for original-C type observations without dereferencing it.
impl OraclePointerType for *mut i32 {
    /// Observation tag distinguishing this C pointer category from equal-sized Rust storage.
    const KIND: &'static str = "int_pointer";
}
/// Tag this pointer representation for original-C type observations without dereferencing it.
impl OraclePointerType for NullCallback {
    /// Observation tag distinguishing this C pointer category from equal-sized Rust storage.
    const KIND: &'static str = "callback";
}

/// Record the expression's C type identity and side effects so value equality alone cannot
/// bless an incorrect conversion.
fn record_type<T: OraclePointerType>(name: &str, value: T) {
    let _ = value;
    // SAFETY: The fixture counter belongs to this process and requires no backend.
    println!("{name}:{}:{}:{}", T::KIND, core::mem::size_of::<T>() * 8, unsafe { null_count() });
}

/// Exercise the generated definitions and print observations for the paired original-C oracle;
/// assertions cover cases with no scalar output.
fn main() {
    let mut value = 7_i32;
    let pointer = &raw mut value;
    let zero = 0_i32;
    // SAFETY: All calls target this fixture's linked C functions, and pointer
    // arguments identify initialized live local storage with the exact prototype.
    unsafe {
        null_reset();
        record("constant", NULL_CONST!().get().is_null());
        null_reset();
        record("integer_cast", NULL_CAST_CONST!().get().is_null());
        null_reset();
        record("enum_constant", NULL_ENUM_CONST!().get().is_null());
        null_reset();
        record("alias", NULL_ALIAS!().get().is_null());
        null_reset();
        record("inline", NULL_PASS_INLINE!().get().is_none());
        null_reset();
        record("present_inline", NULL_PRESENT_INLINE!());
        null_reset();
        record("typed_inline", NULL_TYPED_INLINE!().get().is_none());
        null_reset();
        record("nested_function", NULL_PASS_FUN!(NULL_CONST!()).get().is_none());
        null_reset();
        record("nested_int_cast", NULL_PASS_FUN!(NULL_INT_CONST!()).get().is_none());
        null_reset();
        record("nested_alias", NULL_PASS_FUN!(NULL_ALIAS!()).get().is_none());
        null_reset();
        record("nested_object", NULL_PASS_OBJECT!(NULL_CONST!()).get().is_null());
        null_reset();
        record("nested_enum", NULL_PASS_FUN!(NULL_ENUM_CONST!()).get().is_none());
        null_reset();
        record("function_equal", NULL_EQUAL!(NULL_GET!(0_i32)));
        null_reset();
        record("function_equal_reverse", NULL_EQUAL_REVERSE!(NULL_GET!(0_i32)));
        null_reset();
        record("function_not_equal", NULL_NOT_EQUAL!(NULL_GET!(1_i32)));
        null_reset();
        record("object_equal", NULL_EQUAL!(core::ptr::null_mut::<i32>()));
        null_reset();
        record("object_not_equal", NULL_NOT_EQUAL!(pointer));
        null_reset();
        record("null_pair", NULL_COMPARE!(NULL_CONST!(), NULL_ALIAS!()));
        null_reset();
        record("integer_null_pair", NULL_COMPARE!(NULL_CONST!(), NULL_INT_CONST!()));
        null_reset();
        record("lazy_null", NULL_EQUAL!(NULL_PICK!(1_i32, NULL_GET!(1_i32))));
        null_reset();
        record("lazy_function", NULL_NOT_EQUAL!(NULL_PICK!(0_i32, NULL_GET!(1_i32))));
        null_reset();
        record("lazy_reverse", NULL_EQUAL!(NULL_PICK_REVERSE!(0_i32, NULL_GET!(1_i32))));
        null_reset();
        record("object_selected", NULL_PICK!(0_i32, pointer).get() == pointer);
        null_reset();
        record("object_null", NULL_PICK!(1_i32, pointer).get().is_null());
        null_reset();
        record("nested_size", NULL_SIZE!(NULL_PASS_FUN!(NULL_CONST!())));
        null_reset();
        record("self_size", NULL_SELF_SIZE!());
        null_reset();
        NULL_DISCARD!(NULL_PASS_FUN!(NULL_CONST!()));
        record("discard", 0_i32);
        null_reset();
        record_type("constant_type", NULL_CONST!().get());
        null_reset();
        record_type("function_type", NULL_PICK!(0_i32, NULL_GET!(1_i32)).get());
        null_reset();
        record_type("object_type", NULL_PICK!(1_i32, pointer).get());
        null_reset();
        record_type("both_null_type", NULL_BOTH!(1_i32).get());
        null_reset();
        record("compound", NULL_PASS_FUN!(NULL_COMPOUND_CONST!()).get().is_none());
        null_reset();
        record("unsigned_cast", NULL_PASS_FUN!(NULL_UNSIGNED_CAST!()).get().is_none());
        null_reset();
        record("lazy_zero", NULL_PASS_FUN!(NULL_LAZY_ZERO!()).get().is_none());
        null_reset();
        record("logical_zero", NULL_PASS_FUN!(NULL_LOGICAL_ZERO!()).get().is_none());
        null_reset();
        record("compound_integer", NULL_PASS_FUN!(NULL_COMPOUND_INT!()).get().is_none());
        null_reset();
        record("signed_integer", NULL_PASS_FUN!(NULL_SIGNED_INT!()).get().is_none());
        null_reset();
        record("delegated_integer", NULL_PASS_FUN!(NULL_DELEGATED_ZERO!()).get().is_none());
        null_reset();
        record("literal_decimal", NULL_PASS_FUN!(0).get().is_none());
        null_reset();
        record("literal_hex", NULL_PASS_FUN!(0x00_00_u32).get().is_none());
        null_reset();
        record("literal_unsigned_short", NULL_PASS_FUN!(0b0000_u16).get().is_none());
        null_reset();
        record("literal_negative_zero", NULL_PASS_FUN!(-0).get().is_none());
        null_reset();
        record("literal_negative_suffixed_zero", NULL_PASS_FUN!(-0_i32).get().is_none());
        null_reset();
        record("literal_trailing_comma", NULL_PASS_FUN!(-0,).get().is_none());
        null_reset();
        record("literal_grouped_zero", NULL_PASS_FUN!((0)).get().is_none());
        null_reset();
        record("literal_bool_false", NULL_PASS_FUN!(false).get().is_none());
        null_reset();
        record("literal_object", NULL_PASS_OBJECT!(0).get().is_null());
        null_reset();
        record("literal_function_equal", NULL_COMPARE!(NULL_GET!(0), 0));
        null_reset();
        record("literal_lazy", NULL_LITERAL_PICK!(1, 0_i32, NULL_GET!(1)).get().is_none());
        null_reset();
        record_type("literal_function_type", NULL_LITERAL_PICK!(0, 0_i32, NULL_GET!(1)).get());
        null_reset();
        record_type("literal_void_type", NULL_LITERAL_PICK!(1, NULL_CONST!(), 0_i32).get());
        null_reset();
        record("literal_int_size", NULL_SIZE!(0));
        null_reset();
        record(
            "literal_ull_size",
            NULL_SIZE!(__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLongLong>::new(0)),
        );
        null_reset();
        record("negative_native_expression", NULL_NEGATIVE_VALUE!(-zero + 1));
        null_reset();
        record("negative_native_call", NULL_NEGATIVE_VALUE!(-null_count() + 1));
        null_reset();
        record("negative_default_literal", NULL_NEGATIVE_VALUE!(-1));
        null_reset();
        record("negative_suffixed_literal", NULL_NEGATIVE_VALUE!(-7_i32));
        null_reset();
        record("negative_suffixed_size", NULL_SIZE!(-7_i32));
        null_reset();
        record(
            "tagged_long_long_size",
            NULL_SIZE!(__pgrx_c_macros::CValue::<__pgrx_c_macros::CLongLong>::new(-7)),
        );
        null_reset();
        record_type("cancelled_void_type", NULL_CANCEL_VOID!().get());
        null_reset();
        record("cancelled_void_equal", NULL_COMPARE!(NULL_CANCEL_VOID!(), NULL_CONST!()));
    }
}
