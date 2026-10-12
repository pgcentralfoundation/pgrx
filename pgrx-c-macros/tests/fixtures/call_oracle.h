/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

typedef struct CallRecord {
    unsigned short small;
    int wide;
} CallRecord;

void call_reset(void);
unsigned int call_trace(void);
unsigned int call_count(void);
unsigned char call_byte(unsigned char value);
unsigned short call_short(unsigned short value);
int call_int(int value);
long call_long(long value);
unsigned long call_ulong(unsigned long value);
long long call_llong(long long value);
unsigned long long call_ull(unsigned long long value);
_Bool call_bool(_Bool value);
int call_read(const int *value);
int call_write(int *value, int replacement);
void call_void(int value);
int *call_pointer(int *value);
const int *call_const_pointer(const int *value);
CallRecord call_record(unsigned short small, int wide);
int call_record_sum(CallRecord value);
int call_unprototyped();
int call_variadic(int value, ...);

#define CALL_BYTE(value) call_byte((value))
#define CALL_SHORT(value) call_short((value))
#define CALL_INT(value) call_int((value))
#define CALL_LONG(value) call_long((value))
#define CALL_ULONG(value) call_ulong((value))
#define CALL_LLONG(value) call_llong((value))
#define CALL_ULL(value) call_ull((value))
#define CALL_BOOL(value) call_bool((value))
#define CALL_READ(value) call_read((value))
#define CALL_WRITE(value, replacement) call_write((value), (replacement))
#define CALL_VOID(value) call_void((value))
#define CALL_LAZY(flag, yes, no) ((flag) ? call_int((yes)) : call_int((no)))
#define CALL_LAZY_VOID(flag, yes, no) ((flag) ? call_void((yes)) : call_void((no)))
#define CALL_COMMA(left, right) (call_int((left)), call_int((right)))
#define CALL_REPEAT(value) (call_int((value)), call_int((value)), call_int((value)))
#define CALL_NESTED(value) call_byte(call_short((value)))
#define CALL_SEQUENCE(value) (call_void((value)), call_int((value)))
#define CALL_VOID_DISCARD(value) ((void) call_int((value)))
#define CALL_POINTER(value) call_pointer((value))
#define CALL_CONST_POINTER(value) call_const_pointer((value))
#define CALL_RECORD(small, wide) call_record((small), (wide))
#define CALL_RECORD_SUM(value) call_record_sum((value))
#define CALL_RECORD_NESTED(small, wide) call_record_sum(call_record((small), (wide)))
#define CALL_RECORD_FIELD(small_value, wide_value) (call_record((small_value), (wide_value)).wide)
#define CALL_UNPROTOTYPED(value) call_unprototyped((value))
#define CALL_VARIADIC(value) call_variadic((value))
#define CALL_WRONG_ARITY(value) call_write((value))
#define CALL_MISSING(value) call_missing((value))
