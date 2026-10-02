static int base_calls;
ContextRecord *context_base(ContextRecord *record) {
    ++base_calls;
    return record;
}
void context_reset(void) { base_calls = 0; }
int context_count(void) { return base_calls; }

void context_native_observe_volatile(ContextRecord *pointer) {
    CTX_VOID(CTX_VOLATILE(pointer));
}
unsigned long context_native_observe_size(ContextRecord *pointer) {
    return CTX_SIZE(CTX_VOLATILE(pointer));
}

#ifdef PGRX_CONTEXT_ORACLE_MAIN
#include <stdio.h>
#include <limits.h>
#define RECORD(label, expression) do { \
    unsigned long __pgrx_context_oracle_result = (unsigned long)(expression); \
    printf("%s:%lu:%d\n", label, __pgrx_context_oracle_result, context_count()); \
} while (0)
#define KIND(value) _Generic((value), unsigned short: "CUnsignedShort", \
    int: "CInt", unsigned int: "CUnsignedInt", unsigned long: "CUnsignedLong", \
    unsigned long long: "CUnsignedLongLong")
#define RECORD_TYPE(label, expression) do { \
    __typeof__(expression) __pgrx_context_oracle_type = (expression); \
    printf("%s:%s:%u:%d\n", label, KIND(__pgrx_context_oracle_type), \
        (unsigned)(sizeof(__pgrx_context_oracle_type) * CHAR_BIT), context_count()); \
} while (0)
#define RESET() context_reset()
int main(void) {
    ContextRecord record = { .field = 4, .array = { 11, 12, 13 }, .bit = 2, .volatile_field = 37 };
    ContextRecord *pointer = &record;
    RESET(); RECORD("field_size", CTX_SIZE(CTX_FIELD(context_base(pointer))));
    RESET(); RECORD("field_grouped", CTX_IDENTITY(((CTX_FIELD(context_base(pointer))))));
    RESET(); RECORD("field_qualified", CTX_IDENTITY(CTX_FIELD(context_base(pointer))));
    RESET(); RECORD("array_size", CTX_SIZE(CTX_ARRAY(context_base(pointer))));
    RESET(); RECORD("field_address", CTX_ADDRESS(CTX_FIELD(context_base(pointer))) == &record.field);
    RESET(); RECORD("field_set", CTX_SET(CTX_FIELD(context_base(pointer)), 65537u));
    RECORD("field_stored", record.field);
    RESET(); RECORD("identity_set", CTX_SET(CTX_IDENTITY(CTX_FIELD(context_base(pointer))), 9));
    RESET(); RECORD("field_mod", CTX_MOD(CTX_FIELD(context_base(pointer)), 65535u));
    RECORD("field_mod_stored", record.field);
    RESET(); RECORD("field_post", CTX_POST(CTX_FIELD(context_base(pointer))));
    RECORD("field_post_stored", record.field);
    RESET(); RECORD("field_pre", CTX_PRE(CTX_FIELD(context_base(pointer))));
    RESET(); RECORD("field_repeat", CTX_REPEAT(CTX_FIELD(context_base(pointer))));
    RESET(); RECORD("field_lazy_yes", CTX_LAZY(1, CTX_FIELD(context_base(pointer)), CTX_FIELD(context_base(pointer))));
    RESET(); RECORD("field_lazy_no", CTX_LAZY(0, CTX_FIELD(context_base(pointer)), CTX_FIELD(context_base(pointer))));
    RESET(); RECORD("field_comma", CTX_COMMA(CTX_FIELD(context_base(pointer)), CTX_FIELD(context_base(pointer))));
    RESET(); CTX_VOID(CTX_VOLATILE(context_base(pointer))); RECORD("volatile_discard", 0);
    RESET(); RECORD("volatile_size", CTX_SIZE(CTX_VOLATILE(context_base(pointer))));
    RESET(); RECORD("bit_set", CTX_SET(CTX_BIT(context_base(pointer)), 9));
    RESET(); RECORD("bit_mod", CTX_MOD(CTX_BIT(context_base(pointer)), 7));
    RESET(); RECORD("bit_post", CTX_POST(CTX_BIT(context_base(pointer))));
    RESET(); RECORD("bit_pre", CTX_PRE(CTX_BIT(context_base(pointer))));
    RESET(); RECORD("bit_assignment_size", CTX_SIZE(CTX_SET(CTX_BIT(context_base(pointer)), 3)));
    RESET(); RECORD("bit_post_size", CTX_SIZE(CTX_POST(CTX_BIT(context_base(pointer)))));
    RESET(); RECORD("bit_comma_size", CTX_SIZE(CTX_COMMA(0, CTX_BIT(context_base(pointer)))));
    RESET(); RECORD("array_address", CTX_ADDRESS(CTX_ARRAY(context_base(pointer))) == &record.array);
    RESET(); RECORD("type_hole", CTX_TYPE(unsigned short, 65537u));
    RESET(); RECORD("atomic_identifier", CTX_ATOMIC(record.field));
    RESET(); RECORD("atomic_grouped_expression", CTX_ATOMIC((1 + 2)));
    RESET(); CTX_IGNORE(context_base(pointer)); RECORD("unused", 0);
    RESET(); CTX_IGNORE(); RECORD("unused_empty", 0);
    ContextRecord partial;
    RESET(); RECORD("partial_field_set", CTX_SET(CTX_FIELD(context_base(&partial)), 259));
    RESET(); RECORD("partial_field_read", CTX_FIELD(context_base(&partial)));
    RESET(); RECORD_TYPE("type_field", CTX_FIELD(context_base(pointer)));
    RESET(); RECORD_TYPE("type_assignment", CTX_SET(CTX_FIELD(context_base(pointer)), 11));
    RESET(); RECORD_TYPE("type_compound", CTX_MOD(CTX_FIELD(context_base(pointer)), 1));
    RESET(); RECORD_TYPE("type_post", CTX_POST(CTX_FIELD(context_base(pointer))));
    RESET(); RECORD_TYPE("type_pre", CTX_PRE(CTX_FIELD(context_base(pointer))));
    RESET(); RECORD_TYPE("type_repeat", CTX_REPEAT(CTX_FIELD(context_base(pointer))));
    RESET(); RECORD_TYPE("type_conditional", CTX_LAZY(1, CTX_FIELD(context_base(pointer)), CTX_FIELD(context_base(pointer))));
    RESET(); RECORD_TYPE("type_comma", CTX_COMMA(0, CTX_FIELD(context_base(pointer))));
    RESET(); RECORD_TYPE("type_cast", CTX_TYPE(unsigned short, 65537u));
    RESET(); RECORD_TYPE("type_size", CTX_SIZE(CTX_FIELD(context_base(pointer))));
    return 0;
}
#endif
