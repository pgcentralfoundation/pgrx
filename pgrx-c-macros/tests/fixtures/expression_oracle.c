/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

#include <limits.h>
#include <stdio.h>

#define KIND(value) _Generic((value), \
    int: "CInt", unsigned int: "CUnsignedInt", \
    long: "CLong", unsigned long: "CUnsignedLong", \
    long long: "CLongLong", unsigned long long: "CUnsignedLongLong")
static unsigned trace;
static unsigned calls;
static int tick(unsigned event, int value) {
    trace = trace * 10 + event;
    ++calls;
    return value;
}
#define RESET() do { trace = 0; calls = 0; } while (0)
#define RECORD(name, expression) do { \
    __typeof__(expression) value = (expression); \
    const unsigned __int128 bits = (unsigned __int128) value; \
    printf("%s\t%s\t%u\t%016llx%016llx\t%u\t%u\n", \
        name, KIND(value), (unsigned) (sizeof(value) * CHAR_BIT), \
        (unsigned long long) (bits >> 64), (unsigned long long) bits, trace, calls); \
} while (0)
#define RECORD_VOID(name, expression) do { \
    _Static_assert(__builtin_types_compatible_p(__typeof__(expression), void), "void result type"); \
    (expression); \
    printf("%s\tCVoid\t0\t00000000000000000000000000000000\t%u\t%u\n", name, trace, calls); \
} while (0)

int main(void) {
    RESET();
    EXPR_EMPTY(undeclared_argument, invalid_function(other_undeclared_argument));
    EXPR_EMPTY_ZERO();
    EXPR_EMPTY(tick(9, 3), tick(8, 2));
    RECORD("empty", 11);

    RESET();
    RECORD_VOID("void_constant", EXPR_DROP_CONSTANT(undeclared_argument));
    RESET();
    RECORD_VOID("void_once", EXPR_DROP(tick(1, -7)));
    RESET();
    RECORD("comma", EXPR_COMMA(tick(1, -7), tick(2, 9)));
    RESET();
    RECORD("three", EXPR_THREE(tick(1, -7), tick(2, 9), tick(3, -4)));
    RESET();
    RECORD("repeat", EXPR_REPEAT(tick(1, 9)));
    RESET();
    RECORD("drop_then", EXPR_DROP_THEN(tick(1, -7), tick(2, 9)));
    RESET();
    RECORD("comma_unsigned", EXPR_COMMA(tick(1, 5), (unsigned int) -1));
    RESET();
    RECORD("comma_long", EXPR_COMMA(tick(1, 5), -1L));
    RESET();
    RECORD("comma_ull", EXPR_COMMA(tick(1, 5), 0xFFFFFFFFFFFFFFFFULL));
    RESET();
    RECORD("lazy_yes", EXPR_LAZY(tick(1, 1), tick(2, -7), tick(3, 9)));
    RESET();
    RECORD("lazy_no", EXPR_LAZY(tick(1, 0), tick(2, -7), tick(3, 9)));
    RESET();
    RECORD_VOID("lazy_void_yes", EXPR_LAZY_VOID(tick(1, 1), tick(2, -7), tick(3, 9)));
    RESET();
    RECORD_VOID("lazy_void_no", EXPR_LAZY_VOID(tick(1, 0), tick(2, -7), tick(3, 9)));
    RESET();
    RECORD("nested", EXPR_NESTED(tick(1, -7), tick(2, 9), tick(3, -4)));
    RESET();
    int atomic_value = 16;
    RECORD("atomic_identifier", EXPR_ATOMIC_POW2(atomic_value));
    RECORD("atomic_literal", EXPR_ATOMIC_POW2(15));
    RECORD("atomic_parenthesized", EXPR_ATOMIC_POW2((1 + 3)));
    RECORD("atomic_signed_parenthesized", EXPR_ATOMIC_POW2((-1)));
    RECORD("atomic_unsigned", EXPR_ATOMIC_POW2(0x80000000U));
    return 0;
}
