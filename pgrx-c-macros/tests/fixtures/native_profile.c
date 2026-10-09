/* The runner includes the unchanged profile header before this original-C observation. */
#include <limits.h>
#include <stdio.h>
#define KIND(value) _Generic((value), \
    char: "CChar", int: "CInt", unsigned int: "CUnsignedInt", \
    long: "CLong", unsigned long: "CUnsignedLong", \
    long long: "CLongLong", unsigned long long: "CUnsignedLongLong")
#define SIGNED(value) _Generic((value), \
    char: CHAR_MIN < 0, int: 1, unsigned int: 0, \
    long: 1, unsigned long: 0, long long: 1, unsigned long long: 0)
#define RANK(value) _Generic((value), \
    char: 1, int: 3, unsigned int: 3, \
    long: 4, unsigned long: 4, long long: 5, unsigned long long: 5)
#define RECORD(name, expression) do { \
    __typeof__(expression) observed = (expression); \
    unsigned long long high = SIGNED(observed) && (long long)observed < 0 ? ~0ULL : 0; \
    printf("%s\t%s\t%u\t%d\t%d\t%016llx%016llx\n", name, KIND(observed), \
        (unsigned)(sizeof(observed) * CHAR_BIT), SIGNED(observed), RANK(observed), high, \
        (unsigned long long)observed); \
} while (0)
int main(void) {
    for (unsigned value = 0; value < 256; ++value) {
        RECORD("char", PROFILE_CHAR(value));
        RECORD("char_add", PROFILE_CHAR_ADD(value));
    }
    ProfileCharCallback callback = PROFILE_CHAR_CALLBACK_GET();
    for (unsigned value = 0; value < 256; ++value) {
        RECORD("char_call", PROFILE_CHAR_CALL(value));
        RECORD("char_callback_call", PROFILE_CHAR_CALLBACK_CALL(callback, value));
    }
    RECORD("long_unsigned_int", PROFILE_LONG_UINT(-2));
    RECORD("unsigned_long_long_long", PROFILE_ULONG_LLONG(-1));
    RECORD("size", PROFILE_SIZE(-1));
    RECORD("sizeof", PROFILE_SIZE_OF(PROFILE_CHAR(255)));
    RECORD("alignment_long_long", PROFILE_ALIGNMENT(long long));
    RECORD("alignment_long_long_array", PROFILE_ALIGNMENT(ProfileLongLongArray));
    RECORD("alignment_long_long_nested_array", PROFILE_ALIGNMENT(ProfileLongLongNestedArray));
    RECORD("alignment_double", PROFILE_ALIGNMENT(double));
    RECORD("alignment_double_array", PROFILE_ALIGNMENT(ProfileDoubleArray));
    RECORD("alignment_double_nested_array", PROFILE_ALIGNMENT(ProfileDoubleNestedArray));
    RECORD("alignment_typed_long_long", PROFILE_ALIGNMENT_LONG_LONG());
    RECORD("alignment_typed_long_long_array", PROFILE_ALIGNMENT_LONG_LONG_ARRAY());
    RECORD("alignment_typed_long_long_nested_array", PROFILE_ALIGNMENT_LONG_LONG_NESTED_ARRAY());
    RECORD("alignment_typed_double", PROFILE_ALIGNMENT_DOUBLE());
    RECORD("alignment_typed_double_array", PROFILE_ALIGNMENT_DOUBLE_ARRAY());
    RECORD("alignment_typed_double_nested_array", PROFILE_ALIGNMENT_DOUBLE_NESTED_ARRAY());
    int values[4] = {1, 2, 3, 4};
    RECORD("pointer_difference", PROFILE_POINTER_DIFF(&values[3], &values[0]));
    char character = 0;
    long signed_long = 0;
    ProfileSize size = 0;
    RECORD("char_store", PROFILE_CHAR_STORE(&character, 255));
    RECORD("char_storage", character);
    RECORD("long_store", PROFILE_LONG_STORE(&signed_long, -7));
    RECORD("long_storage", signed_long);
    RECORD("size_store", PROFILE_SIZE_STORE(&size, -1));
    RECORD("size_storage", size);
    ProfileRecord item = {0, 0, 0};
    for (unsigned value = 0; value < 256; ++value) {
        RECORD("record_char_store", PROFILE_RECORD_CHAR_STORE(&item, value));
        RECORD("record_char_storage", PROFILE_RECORD_CHAR(&item));
    }
    RECORD("record_long_store", PROFILE_RECORD_LONG_STORE(&item, -7));
    RECORD("record_long_storage", PROFILE_RECORD_LONG(&item));
    RECORD("record_size_store", PROFILE_RECORD_SIZE_STORE(&item, -1));
    RECORD("record_size_storage", PROFILE_RECORD_SIZE(&item));
    return 0;
}
