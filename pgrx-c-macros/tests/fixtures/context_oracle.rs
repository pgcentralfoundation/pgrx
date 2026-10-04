use __pgrx_c_macros::{CInteger, IntoCValue};

fn record<T: IntoCValue>(name: &str, value: T) {
    let value = value.into_c_value();
    // SAFETY: This fixture reads only its process-local native counter.
    println!("{name}:{}:{}", T::Kind::encode(value.get()) as u64, unsafe { context_count() });
}

fn record_type<T: IntoCValue>(name: &str, value: T) {
    let _ = value.into_c_value();
    let kind = core::any::type_name::<T::Kind>().rsplit("::").next().unwrap();
    // SAFETY: This fixture reads only its process-local native counter.
    println!("{name}:{kind}:{}:{}", T::Kind::BITS, unsafe { context_count() });
}

// Same-spelled unrelated Rust macros remain usable through the native escape.
mod native {
    macro_rules! native_field {
        ($value:expr) => {
            $value
        };
    }
    pub(crate) use native_field as CTX_FIELD;
}

fn main() {
    let mut storage = core::mem::MaybeUninit::<ContextRecord>::zeroed();
    let pointer = storage.as_mut_ptr();
    // SAFETY: Zero is valid for every field in this fixture record, and its live
    // allocation is aligned. Each access targets an initialized field except the
    // final plain store into the deliberately uninitialized partial record.
    unsafe {
        (*pointer).field = 4;
        (*pointer).array = [11, 12, 13];
        (*pointer).set_bit(2);
        (*pointer).volatile_field = 37;
        context_reset();
        record("field_size", CTX_SIZE!(CTX_FIELD!(context_base(pointer))));
        context_reset();
        record("field_grouped", CTX_IDENTITY!((CTX_FIELD!(context_base(pointer)))));
        context_reset();
        record("field_qualified", CTX_IDENTITY!(crate::CTX_FIELD!(context_base(pointer))));
        context_reset();
        record("array_size", CTX_SIZE!(CTX_ARRAY!(context_base(pointer))));
        context_reset();
        record(
            "field_address",
            CTX_ADDRESS!(CTX_FIELD!(context_base(pointer))).get()
                == core::ptr::addr_of_mut!((*pointer).field),
        );
        context_reset();
        record("field_set", CTX_SET!(CTX_FIELD!(context_base(pointer)), 65537_u32));
        record("field_stored", (*pointer).field);
        context_reset();
        record("identity_set", CTX_SET!(CTX_IDENTITY!(CTX_FIELD!(context_base(pointer))), 9_i32));
        context_reset();
        record("field_mod", CTX_MOD!(CTX_FIELD!(context_base(pointer)), 65535_u32));
        record("field_mod_stored", (*pointer).field);
        context_reset();
        record("field_post", CTX_POST!(CTX_FIELD!(context_base(pointer))));
        record("field_post_stored", (*pointer).field);
        context_reset();
        record("field_pre", CTX_PRE!(CTX_FIELD!(context_base(pointer))));
        context_reset();
        record("field_repeat", CTX_REPEAT!(CTX_FIELD!(context_base(pointer))));
        context_reset();
        record(
            "field_lazy_yes",
            CTX_LAZY!(1_i32, CTX_FIELD!(context_base(pointer)), CTX_FIELD!(context_base(pointer))),
        );
        context_reset();
        record(
            "field_lazy_no",
            CTX_LAZY!(0_i32, CTX_FIELD!(context_base(pointer)), CTX_FIELD!(context_base(pointer))),
        );
        context_reset();
        record(
            "field_comma",
            CTX_COMMA!(CTX_FIELD!(context_base(pointer)), CTX_FIELD!(context_base(pointer))),
        );
        context_reset();
        CTX_VOID!(CTX_VOLATILE!(context_base(pointer)));
        record("volatile_discard", 0_i32);
        context_reset();
        record("volatile_size", CTX_SIZE!(CTX_VOLATILE!(context_base(pointer))));
        context_reset();
        record("bit_set", CTX_SET!(CTX_BIT!(context_base(pointer)), 9_i32));
        context_reset();
        record("bit_mod", CTX_MOD!(CTX_BIT!(context_base(pointer)), 7_i32));
        context_reset();
        record("bit_post", CTX_POST!(CTX_BIT!(context_base(pointer))));
        context_reset();
        record("bit_pre", CTX_PRE!(CTX_BIT!(context_base(pointer))));
        context_reset();
        record("bit_assignment_size", CTX_SIZE!(CTX_SET!(CTX_BIT!(context_base(pointer)), 3_i32)));
        context_reset();
        record("bit_post_size", CTX_SIZE!(CTX_POST!(CTX_BIT!(context_base(pointer)))));
        context_reset();
        record("bit_comma_size", CTX_SIZE!(CTX_COMMA!(0_i32, CTX_BIT!(context_base(pointer)))));
        context_reset();
        record(
            "array_address",
            CTX_ADDRESS!(CTX_ARRAY!(context_base(pointer))).get()
                == core::ptr::addr_of_mut!((*pointer).array),
        );
        context_reset();
        record("type_hole", CTX_TYPE_PROVEN!(u16, 65537_u32));
        context_reset();
        record("atomic_identifier", CTX_ATOMIC!(((*pointer).field)));
        context_reset();
        record("atomic_grouped_expression", CTX_ATOMIC!((1_i32 + 2_i32)));
        context_reset();
        CTX_IGNORE!(context_base(pointer));
        record("unused", 0_i32);
        context_reset();
        CTX_IGNORE!();
        record("unused_empty", 0_i32);
        let mut partial = core::mem::MaybeUninit::<ContextRecord>::uninit();
        let partial = partial.as_mut_ptr();
        context_reset();
        record("partial_field_set", CTX_SET!(CTX_FIELD!(context_base(partial)), 259_i32));
        context_reset();
        record("partial_field_read", CTX_FIELD!(context_base(partial)));
        context_reset();
        record_type("type_field", CTX_FIELD!(context_base(pointer)));
        context_reset();
        record_type("type_assignment", CTX_SET!(CTX_FIELD!(context_base(pointer)), 11_i32));
        context_reset();
        record_type("type_compound", CTX_MOD!(CTX_FIELD!(context_base(pointer)), 1_i32));
        context_reset();
        record_type("type_post", CTX_POST!(CTX_FIELD!(context_base(pointer))));
        context_reset();
        record_type("type_pre", CTX_PRE!(CTX_FIELD!(context_base(pointer))));
        context_reset();
        record_type("type_repeat", CTX_REPEAT!(CTX_FIELD!(context_base(pointer))));
        context_reset();
        record_type(
            "type_conditional",
            CTX_LAZY!(1_i32, CTX_FIELD!(context_base(pointer)), CTX_FIELD!(context_base(pointer))),
        );
        context_reset();
        record_type("type_comma", CTX_COMMA!(0_i32, CTX_FIELD!(context_base(pointer))));
        context_reset();
        record_type("type_cast", CTX_TYPE_PROVEN!(u16, 65537_u32));
        context_reset();
        record_type("type_size", CTX_SIZE!(CTX_FIELD!(context_base(pointer))));
        CTX_IGNORE!(these tokens are not a Rust expression);
        assert_eq!(CTX_IDENTITY!((@__pgrx_c_native [native::CTX_FIELD!(7_i32)])).get(), 7);
    }
}

/// # Safety
/// `pointer` must designate a live initialized fixture record with valid access.
#[unsafe(no_mangle)]
pub unsafe fn context_observe_volatile(pointer: *mut ContextRecord) {
    // SAFETY: The caller establishes this field's volatile access contract.
    unsafe {
        CTX_VOID!(CTX_VOLATILE!(pointer));
    }
}

/// No storage is accessed; the pointer value only supplies the unevaluated type.
#[unsafe(no_mangle)]
pub fn context_observe_size(pointer: *mut ContextRecord) -> u64 {
    CTX_SIZE!(CTX_VOLATILE!(pointer)).get() as u64
}
