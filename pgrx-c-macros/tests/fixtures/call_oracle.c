/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

static unsigned trace;
static unsigned calls;
static void observed(unsigned event) {
    trace = trace * 10U + event;
    ++calls;
}
void call_reset(void) { trace = 0; calls = 0; }
unsigned call_trace(void) { return trace; }
unsigned call_count(void) { return calls; }
unsigned char call_byte(unsigned char value) { observed(1); return value; }
unsigned short call_short(unsigned short value) { observed(2); return value; }
int call_int(int value) { observed(3); return value; }
long call_long(long value) { observed(4); return value; }
unsigned long call_ulong(unsigned long value) { observed(5); return value; }
long long call_llong(long long value) { observed(6); return value; }
unsigned long long call_ull(unsigned long long value) { observed(7); return value; }
_Bool call_bool(_Bool value) { observed(8); return value; }
int call_read(const int *value) { observed(9); return *value; }
int call_write(int *value, int replacement) { observed(1); *value = replacement; return *value; }
void call_void(int value) { observed(2); (void) value; }
int *call_pointer(int *value) { observed(3); return value; }
const int *call_const_pointer(const int *value) { observed(4); return value; }
CallRecord call_record(unsigned short small, int wide) {
    observed(5);
    CallRecord value = { small, wide };
    return value;
}
int call_record_sum(CallRecord value) { observed(6); return (int) value.small + value.wide; }

#ifdef PGRX_CALL_ORACLE_MAIN
#include <limits.h>
#include <stdio.h>
#define KIND(value) _Generic((value), \
    _Bool: "CBool", unsigned char: "CUnsignedChar", unsigned short: "CUnsignedShort", \
    int: "CInt", unsigned int: "CUnsignedInt", \
    long: "CLong", unsigned long: "CUnsignedLong", \
    long long: "CLongLong", unsigned long long: "CUnsignedLongLong")
#define RECORD(name, expression) do { \
    __typeof__(expression) __pgrx_call_oracle_result = (expression); \
    unsigned __int128 __pgrx_call_oracle_bits = (unsigned __int128) __pgrx_call_oracle_result; \
    printf("%s\t%s\t%u\t%016llx%016llx\t%u\t%u\n", name, KIND(__pgrx_call_oracle_result), \
        (unsigned) (sizeof(__pgrx_call_oracle_result) * CHAR_BIT), (unsigned long long) (__pgrx_call_oracle_bits >> 64), \
        (unsigned long long) __pgrx_call_oracle_bits, call_trace(), call_count()); \
} while (0)
#define RECORD_VOID(name, expression) do { \
    _Static_assert(__builtin_types_compatible_p(__typeof__(expression), void), "void result type"); \
    (expression); \
    printf("%s\tCVoid\t0\t00000000000000000000000000000000\t%u\t%u\n", name, call_trace(), call_count()); \
} while (0)
#define RECORD_POINTER(name, expression, expected) do { \
    __typeof__(expression) __pgrx_call_oracle_result = (expression); \
    unsigned __int128 __pgrx_call_oracle_bits = (__pgrx_call_oracle_result == (expected)); \
    printf("%s\tCPointer\t%u\t%016llx%016llx\t%u\t%u\n", name, \
        (unsigned) (sizeof(__pgrx_call_oracle_result) * CHAR_BIT), (unsigned long long) (__pgrx_call_oracle_bits >> 64), \
        (unsigned long long) __pgrx_call_oracle_bits, call_trace(), call_count()); \
} while (0)
#define RECORD_STRUCT(name, expression) do { \
    _Static_assert(__builtin_types_compatible_p(__typeof__(expression), CallRecord), "record result type"); \
    CallRecord __pgrx_call_oracle_result = (expression); \
    unsigned __int128 __pgrx_call_oracle_bits = ((unsigned __int128) __pgrx_call_oracle_result.small << 64) | (unsigned int) __pgrx_call_oracle_result.wide; \
    printf("%s\tCRecord\t%u\t%016llx%016llx\t%u\t%u\n", name, \
        (unsigned) (sizeof(__pgrx_call_oracle_result) * CHAR_BIT), (unsigned long long) (__pgrx_call_oracle_bits >> 64), \
        (unsigned long long) __pgrx_call_oracle_bits, call_trace(), call_count()); \
} while (0)
int main(void) {
    call_reset(); RECORD("byte_negative", CALL_BYTE(-1));
    call_reset(); RECORD("byte_narrow", CALL_BYTE(0x1234));
    call_reset(); RECORD("byte_wrap", CALL_BYTE(256U));
    call_reset(); RECORD("byte_identity", CALL_BYTE((unsigned char) 255));
    call_reset(); RECORD("short_negative", CALL_SHORT(-1));
    call_reset(); RECORD("short_narrow", CALL_SHORT(0x12345U));
    call_reset(); RECORD("int_from_byte", CALL_INT((unsigned char) 255));
    call_reset(); RECORD("int_negative", CALL_INT(-19));
    call_reset(); RECORD("long_from_int", CALL_LONG(-1));
    call_reset(); RECORD("long_identity", CALL_LONG(-31L));
    call_reset(); RECORD("ulong_negative", CALL_ULONG(-1));
    call_reset(); RECORD("ulong_identity", CALL_ULONG(0xFEDCBAUL));
    call_reset(); RECORD("llong_identity", CALL_LLONG(-41LL));
    call_reset(); RECORD("llong_from_uint", CALL_LLONG(0xFFFFFFFFU));
    call_reset(); RECORD("ull_negative", CALL_ULL(-1));
    call_reset(); RECORD("ull_identity", CALL_ULL(0xFEDCBA9876543210ULL));
    call_reset(); RECORD("bool_zero", CALL_BOOL(0));
    call_reset(); RECORD("bool_negative", CALL_BOOL(-3));
    call_reset(); RECORD("bool_before_narrow", CALL_BOOL(256U));
    int value = 37;
    call_reset(); RECORD("bool_pointer", CALL_BOOL(&value));
    call_reset(); RECORD("bool_null_pointer", CALL_BOOL((const int *) 0));
    call_reset(); RECORD("read_const", CALL_READ((const int *) &value));
    call_reset(); RECORD("read_mutable", CALL_READ(&value));
    call_reset(); RECORD("write", CALL_WRITE(&value, -7));
    RECORD("write_observed", value);
    call_reset(); RECORD_VOID("void", CALL_VOID(-8));
    call_reset(); RECORD("lazy_yes", CALL_LAZY(1, -1, 99));
    call_reset(); RECORD("lazy_no", CALL_LAZY(0, -1, 99));
    call_reset(); RECORD_VOID("lazy_void_yes", CALL_LAZY_VOID(1, -1, 99));
    call_reset(); RECORD_VOID("lazy_void_no", CALL_LAZY_VOID(0, -1, 99));
    call_reset(); RECORD("comma", CALL_COMMA(-2, 5));
    call_reset(); RECORD("repeat", CALL_REPEAT(-3));
    call_reset(); RECORD("repeat_argument", CALL_REPEAT(call_int(-3)));
    call_reset(); RECORD("comma_argument", CALL_COMMA(call_int(-2), call_int(5)));
    call_reset(); RECORD("lazy_argument", CALL_LAZY(call_int(1), call_int(-2), call_int(5)));
    call_reset(); RECORD("nested", CALL_NESTED(0x12345));
    call_reset(); RECORD("sequence", CALL_SEQUENCE(-9));
    call_reset(); RECORD_VOID("void_discard", CALL_VOID_DISCARD(-2));
    call_reset(); RECORD_POINTER("pointer_mutable", CALL_POINTER(&value), &value);
    call_reset(); RECORD_POINTER("pointer_const", CALL_CONST_POINTER((const int *) &value), &value);
    call_reset(); RECORD_STRUCT("record", CALL_RECORD(0x12345U, -71));
    CallRecord record = { 13, -7 };
    call_reset(); RECORD("record_by_value", CALL_RECORD_SUM(record));
    call_reset(); RECORD("record_nested", CALL_RECORD_NESTED(0x12345U, -71));
    call_reset(); RECORD("record_field", CALL_RECORD_FIELD(0x12345U, -71));
    return 0;
}
#endif
