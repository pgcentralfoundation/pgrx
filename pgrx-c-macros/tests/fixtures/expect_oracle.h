//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef long (*ExpectCallback)(long, long);
extern unsigned int expect_value_calls;
extern unsigned int expect_hint_calls;
extern unsigned int expect_trace;
extern volatile long expect_signal;
extern volatile long expect_hint_signal;
long expect_record_value(long value);
long expect_record_hint(long hint);
long expect_use_callback(ExpectCallback callback);
int *expect_take_pointer(int *pointer);

#ifdef EXPECT_SHADOW
#define __builtin_expect(value, hint) ((long) (value) + 3L)
#endif

#define EXPECT_RAW(value, hint) __builtin_expect(value, hint)
#define EXPECT_GROUPED(value, hint) ((__builtin_expect))(value, hint)
#define EXPECT_NESTED(value) __builtin_expect(__builtin_expect(value, 1), 0)
#define EXPECT_LIKELY(value) __builtin_expect((value) != 0, 1)
#define EXPECT_UNLIKELY(value) __builtin_expect((value) != 0, 0)
#define EXPECT_RECORD(value, hint) __builtin_expect(expect_record_value(value), expect_record_hint(hint))
#define EXPECT_VOLATILE() __builtin_expect(expect_signal, expect_hint_signal)
#define EXPECT_SIZE(value, hint) sizeof(__builtin_expect(value, hint))
#define EXPECT_LAZY(condition, value, hint) ((condition) ? __builtin_expect(value, hint) : -7L)
#define EXPECT_AND(condition, value, hint) ((condition) && __builtin_expect(value, hint))
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
