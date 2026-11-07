//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef long (*ExpectCallback)(long, long);
typedef struct ExpectRecord {
    unsigned char lead;
    long value;
} ExpectRecord;
extern unsigned int expect_value_calls;
extern unsigned int expect_hint_calls;
extern unsigned int expect_trace;
extern volatile long expect_signal;
extern volatile long expect_hint_signal;
extern volatile int expect_interrupt_flag;
extern unsigned int expect_interrupt_calls;
extern int expect_interrupt_clear;
long expect_record_value(long value);
long expect_record_hint(long hint);
long expect_use_callback(ExpectCallback callback);
int *expect_take_pointer(int *pointer);
void expect_process_interrupts(void);
void expect_branch_yes(int tag);
void expect_branch_no(int tag);
long expect_branch_gate(void);

#ifdef EXPECT_SHADOW
#define __builtin_expect(value, hint) ((long) (value) + 3L)
#endif

#define EXPECT_RAW(value, hint) __builtin_expect(value, hint)
#define EXPECT_TYPE_CAST(type, value) __builtin_expect((value), (type)257)
#define EXPECT_TYPE_EFFECT(type, value) __builtin_expect((value), (sizeof(type *), (type)(expect_record_hint(19), 0)))
#define EXPECT_TYPE_DYNAMIC(type, value, hint) __builtin_expect((value), (sizeof(type *), (type)(hint)))
#define EXPECT_TYPE_SIZE(type, value) (sizeof(type *), __builtin_expect((value), sizeof(type)))
#define EXPECT_TYPE_ALIGN(type, value) __builtin_expect((value), _Alignof(type))
#define EXPECT_TYPE_POINTER_SIZE(type, value) __builtin_expect((value), sizeof(type *))
#define EXPECT_VALUE_SIZE(value) __builtin_expect((value), sizeof(value))
#define EXPECT_SOURCE_SIZE(value, hint_source) __builtin_expect((value), sizeof(hint_source))
#define EXPECT_OFFSET_TYPE(type, expression) __builtin_expect((expression), __builtin_offsetof(type, value))
#define EXPECT_OFFSET_FIELD(value, field) __builtin_expect((value), __builtin_offsetof(ExpectRecord, field))
#define EXPECT_OFFSET_BOTH(type, field, value) __builtin_expect((value), __builtin_offsetof(type, field))
#define EXPECT_GROUPED(value, hint) ((__builtin_expect))(value, hint)
#define EXPECT_NESTED(value) __builtin_expect(__builtin_expect(value, 1), 0)
#define EXPECT_NESTED_SEVEN(value) __builtin_expect(__builtin_expect(value, 7), 0)
#define EXPECT_RAW_SEVEN(value) __builtin_expect(value, 7)
#define EXPECT_DELEGATED_SEVEN(value) EXPECT_RAW(value, 7)
#define EXPECT_DELEGATED_NESTED(value) EXPECT_NESTED_SEVEN(value)
#define EXPECT_NESTED_LONG_LONG(value) __builtin_expect((long long) __builtin_expect(value, 7), 0)
#define EXPECT_NESTED_UNSIGNED_LONG(value) __builtin_expect((unsigned long) __builtin_expect(value, 7), 0)
#define EXPECT_NESTED_INT128(value) __builtin_expect((__int128) __builtin_expect(value, 7), 0)
#define EXPECT_NESTED_INT(value) __builtin_expect((int) __builtin_expect(value, 7), 0)
#define EXPECT_NESTED_BOOL(value) __builtin_expect((_Bool) __builtin_expect(value, 7), 0)
#define EXPECT_NESTED_DYNAMIC(value, hint) __builtin_expect((int) __builtin_expect(value, 7), hint)
#define EXPECT_EFFECT_ZERO(value) __builtin_expect(value, (expect_record_hint(19), 0))
#define EXPECT_LIKELY(value) __builtin_expect((value) != 0, 1)
#define EXPECT_UNLIKELY(value) __builtin_expect((value) != 0, 0)
#define EXPECT_INTERRUPTS() \
    do { \
        if (EXPECT_UNLIKELY(expect_interrupt_flag)) \
            expect_process_interrupts(); \
    } while (0)
#define EXPECT_RECORD(value, hint) __builtin_expect(expect_record_value(value), expect_record_hint(hint))
#define EXPECT_VOLATILE() __builtin_expect(expect_signal, expect_hint_signal)
#define EXPECT_SIZE(value, hint) sizeof(__builtin_expect(value, hint))
#define EXPECT_LAZY(condition, value, hint) ((condition) ? __builtin_expect(value, hint) : -7L)
#define EXPECT_AND(condition, value, hint) ((condition) && __builtin_expect(value, hint))
#define EXPECT_LAZY_ZERO(condition, value) ((condition) ? __builtin_expect(value, 0) : -7L)
#define EXPECT_AND_ONE(condition, value) ((condition) && __builtin_expect(value, 1))
#define EXPECT_REPEAT(value, hint) (__builtin_expect(value, hint) + __builtin_expect(value, hint))
#define EXPECT_UNUSED(unused) __builtin_expect(4, 0)
#ifdef __OPTIMIZE__
#define EXPECT_PROFILE(value) (__builtin_expect(value, 0) + 20L)
#else
#define EXPECT_PROFILE(value) (__builtin_expect(value, 0) + 10L)
#endif
#define EXPECT_ZERO() __builtin_expect(0, 1)
#define EXPECT_NULL_SELECT(condition, pointer) ((condition) ? EXPECT_ZERO() : (pointer))
#define EXPECT_NULL_CALL() expect_take_pointer(EXPECT_ZERO())
#define EXPECT_RUNTIME_NULL(value) expect_take_pointer(__builtin_expect(value, 1))
#define EXPECT_IMPURE_NULL() expect_take_pointer(__builtin_expect(0, expect_record_hint(1)))

#define EXPECT_WRONG0() __builtin_expect()
#define EXPECT_WRONG1(value) __builtin_expect(value)
#define EXPECT_WRONG3(value) __builtin_expect(value, 1, 2)
#define EXPECT_SYMBOL() __builtin_expect
#define EXPECT_ADDRESS() (&__builtin_expect)
#define EXPECT_DEREF() (*__builtin_expect)(7, 1)
#define EXPECT_CALLBACK() expect_use_callback(__builtin_expect)
