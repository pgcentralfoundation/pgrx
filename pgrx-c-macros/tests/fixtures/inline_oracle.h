/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

static unsigned int inline_calls;
void inline_oracle_reset(void);
unsigned int inline_oracle_count(void);
int inline_existing(int value);

static inline unsigned char inline_byte(unsigned char value)
{
    ++inline_calls;
    return value;
}
static inline int inline_signed(int value)
{
    ++inline_calls;
    return -value;
}
static inline _Bool inline_bool(_Bool value)
{
    ++inline_calls;
    return !value;
}
static inline double inline_double(double value)
{
    ++inline_calls;
    return value * 0.5;
}
static inline unsigned short inline_write(int *where, unsigned short value)
{
    ++inline_calls;
    *where = value;
    return value;
}
static inline void inline_void(int *where)
{
    ++inline_calls;
    *where += 3;
}
static inline int *inline_pointer(int *where)
{
    ++inline_calls;
    return where;
}
static inline const int *inline_const_pointer(const int *where)
{
    ++inline_calls;
    return where;
}
static inline void *inline_void_pointer(void *where)
{
    ++inline_calls;
    return where;
}
static inline unsigned char inline_nested(unsigned short value)
{
    ++inline_calls;
    return inline_byte(value);
}
typedef __UINTPTR_TYPE__ NativeWord;
static inline NativeWord inline_word(NativeWord value)
{
    ++inline_calls;
    return value;
}
static inline int inline_variadic(int value, ...) { return value; }
static inline int inline_unprototyped() { return 1; }
static inline int inline_undefined(int value);
typedef struct InlineRecord { int member; _Bool validity; } InlineRecord;
static inline InlineRecord inline_record(int value)
{
    InlineRecord result;
    result.member = value;
    return result;
}

#define INLINE_BYTE(value) inline_byte((value))
#define INLINE_SIGNED(value) (inline_signed)((value))
#define INLINE_BOOL(value) inline_bool((value))
#define INLINE_DOUBLE(value) inline_double((value))
#define INLINE_WRITE(where, value) inline_write((where), (value))
#define INLINE_VOID(where) inline_void((where))
#define INLINE_POINTER(where) inline_pointer((where))
#define INLINE_CONST_POINTER(where) inline_const_pointer((where))
#define INLINE_VOID_POINTER(where) inline_void_pointer((where))
#define INLINE_NESTED(value) inline_nested((value))
#define INLINE_REPEAT(value) (inline_byte((value)), inline_byte((value)))
#define INLINE_LAZY(flag, value) ((flag) ? inline_byte((value)) : 123)
#define INLINE_EXISTING(value) inline_existing((value))
#define INLINE_WORD(value) inline_word((value))
#define INLINE_VARIADIC(value) inline_variadic((value))
#define INLINE_UNPROTOTYPED(value) inline_unprototyped((value))
#define INLINE_RECORD(value) inline_record((value))
#define INLINE_RECORD_MEMBER(value) (inline_record((value)).member)
#define INLINE_UNDEFINED(value) inline_undefined((value))
/* The protected function call and generated primitive must bypass this macro. */
#define inline_signed(value) ((value) + 41)
