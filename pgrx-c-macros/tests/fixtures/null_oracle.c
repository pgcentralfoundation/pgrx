static int null_calls;
void null_reset(void) { null_calls = 0; }
int null_count(void) { return null_calls; }
int null_increment(int value) { return value + 1; }
NullCallback null_get(int present) {
    ++null_calls;
    return present ? null_increment : (NullCallback)0;
}
NullCallback null_identity_callback(NullCallback value) {
    ++null_calls;
    return value;
}
int null_callback_present(NullCallback value) {
    ++null_calls;
    return value != (NullCallback)0;
}
int *null_identity_object(int *value) {
    ++null_calls;
    return value;
}

#ifdef PGRX_NULL_ORACLE_MAIN
#include <stdio.h>
#include <limits.h>
#define RECORD(label, expression) do { \
    unsigned long result = (unsigned long)(expression); \
    printf("%s:%lu:%d\n", label, result, null_count()); \
} while (0)
#define RECORD_TYPE(label, expression) do { \
    __typeof__(expression) result = (expression); \
    const char *kind = _Generic(result, void *: "void_pointer", int *: "int_pointer", NullCallback: "callback"); \
    printf("%s:%s:%lu:%d\n", label, kind, (unsigned long)(sizeof(result) * CHAR_BIT), null_count()); \
} while (0)
int main(void) {
    int value = 7;
    int *pointer = &value;
    int zero = 0;
    null_reset(); RECORD("constant", NULL_CONST() == (void *)0);
    null_reset(); RECORD("integer_cast", NULL_CAST_CONST() == (void *)0);
    null_reset(); RECORD("enum_constant", NULL_ENUM_CONST() == (void *)0);
    null_reset(); RECORD("alias", NULL_ALIAS() == (void *)0);
    null_reset(); RECORD("inline", NULL_PASS_INLINE() == (NullCallback)0);
    null_reset(); RECORD("present_inline", NULL_PRESENT_INLINE());
    null_reset(); RECORD("typed_inline", NULL_TYPED_INLINE() == (NullCallback)0);
    null_reset(); RECORD("nested_function", NULL_PASS_FUN(NULL_CONST()) == (NullCallback)0);
    null_reset(); RECORD("nested_int_cast", NULL_PASS_FUN(NULL_INT_CONST()) == (NullCallback)0);
    null_reset(); RECORD("nested_alias", NULL_PASS_FUN(NULL_ALIAS()) == (NullCallback)0);
    null_reset(); RECORD("nested_object", NULL_PASS_OBJECT(NULL_CONST()) == (int *)0);
    null_reset(); RECORD("nested_enum", NULL_PASS_FUN(NULL_ENUM_CONST()) == (NullCallback)0);
    null_reset(); RECORD("function_equal", NULL_EQUAL(NULL_GET(0)));
    null_reset(); RECORD("function_equal_reverse", NULL_EQUAL_REVERSE(NULL_GET(0)));
    null_reset(); RECORD("function_not_equal", NULL_NOT_EQUAL(NULL_GET(1)));
    null_reset(); RECORD("object_equal", NULL_EQUAL((int *)0));
    null_reset(); RECORD("object_not_equal", NULL_NOT_EQUAL(pointer));
    null_reset(); RECORD("null_pair", NULL_COMPARE(NULL_CONST(), NULL_ALIAS()));
    null_reset(); RECORD("integer_null_pair", NULL_COMPARE(NULL_CONST(), NULL_INT_CONST()));
    null_reset(); RECORD("lazy_null", NULL_EQUAL(NULL_PICK(1, NULL_GET(1))));
    null_reset(); RECORD("lazy_function", NULL_NOT_EQUAL(NULL_PICK(0, NULL_GET(1))));
    null_reset(); RECORD("lazy_reverse", NULL_EQUAL(NULL_PICK_REVERSE(0, NULL_GET(1))));
    null_reset(); RECORD("object_selected", NULL_PICK(0, pointer) == pointer);
    null_reset(); RECORD("object_null", NULL_PICK(1, pointer) == (int *)0);
    null_reset(); RECORD("nested_size", NULL_SIZE(NULL_PASS_FUN(NULL_CONST())));
    null_reset(); RECORD("self_size", NULL_SELF_SIZE());
    null_reset(); NULL_DISCARD(NULL_PASS_FUN(NULL_CONST())); RECORD("discard", 0);
    null_reset(); RECORD_TYPE("constant_type", NULL_CONST());
    null_reset(); RECORD_TYPE("function_type", NULL_PICK(0, NULL_GET(1)));
    null_reset(); RECORD_TYPE("object_type", NULL_PICK(1, pointer));
    null_reset(); RECORD_TYPE("both_null_type", NULL_BOTH(1));
    null_reset(); RECORD("compound", NULL_PASS_FUN(NULL_COMPOUND_CONST()) == (NullCallback)0);
    null_reset(); RECORD("unsigned_cast", NULL_PASS_FUN(NULL_UNSIGNED_CAST()) == (NullCallback)0);
    null_reset(); RECORD("lazy_zero", NULL_PASS_FUN(NULL_LAZY_ZERO()) == (NullCallback)0);
    null_reset(); RECORD("logical_zero", NULL_PASS_FUN(NULL_LOGICAL_ZERO()) == (NullCallback)0);
    null_reset(); RECORD("compound_integer", NULL_PASS_FUN(NULL_COMPOUND_INT()) == (NullCallback)0);
    null_reset(); RECORD("signed_integer", NULL_PASS_FUN(NULL_SIGNED_INT()) == (NullCallback)0);
    null_reset(); RECORD("delegated_integer", NULL_PASS_FUN(NULL_DELEGATED_ZERO()) == (NullCallback)0);
    null_reset(); RECORD("literal_decimal", NULL_PASS_FUN(0) == (NullCallback)0);
    null_reset(); RECORD("literal_hex", NULL_PASS_FUN(0x000U) == (NullCallback)0);
    null_reset(); RECORD("literal_unsigned_short", NULL_PASS_FUN((unsigned short)0) == (NullCallback)0);
    null_reset(); RECORD("literal_negative_zero", NULL_PASS_FUN(-0) == (NullCallback)0);
    null_reset(); RECORD("literal_negative_suffixed_zero", NULL_PASS_FUN(-0) == (NullCallback)0);
    null_reset(); RECORD("literal_trailing_comma", NULL_PASS_FUN(-0) == (NullCallback)0);
    null_reset(); RECORD("literal_grouped_zero", NULL_PASS_FUN(((0))) == (NullCallback)0);
    null_reset(); RECORD("literal_bool_false", NULL_PASS_FUN((_Bool)0) == (NullCallback)0);
    null_reset(); RECORD("literal_object", NULL_PASS_OBJECT(0) == (int *)0);
    null_reset(); RECORD("literal_function_equal", NULL_COMPARE(NULL_GET(0), 0));
    null_reset(); RECORD("literal_lazy", NULL_LITERAL_PICK(1, 0, NULL_GET(1)) == (NullCallback)0);
    null_reset(); RECORD_TYPE("literal_function_type", NULL_LITERAL_PICK(0, 0, NULL_GET(1)));
    null_reset(); RECORD_TYPE("literal_void_type", NULL_LITERAL_PICK(1, NULL_CONST(), 0));
    null_reset(); RECORD("literal_int_size", NULL_SIZE(0));
    null_reset(); RECORD("literal_ull_size", NULL_SIZE(0ULL));
    null_reset(); RECORD("negative_native_expression", NULL_NEGATIVE_VALUE(-zero + 1));
    null_reset(); RECORD("negative_native_call", NULL_NEGATIVE_VALUE(-null_count() + 1));
    null_reset(); RECORD("negative_default_literal", NULL_NEGATIVE_VALUE(-1));
    null_reset(); RECORD("negative_suffixed_literal", NULL_NEGATIVE_VALUE(-7));
    null_reset(); RECORD("negative_suffixed_size", NULL_SIZE(-7));
    null_reset(); RECORD("tagged_long_long_size", NULL_SIZE(-7LL));
    null_reset(); RECORD_TYPE("cancelled_void_type", NULL_CANCEL_VOID());
    null_reset(); RECORD("cancelled_void_equal", NULL_COMPARE(NULL_CANCEL_VOID(), NULL_CONST()));
    return 0;
}
#endif
