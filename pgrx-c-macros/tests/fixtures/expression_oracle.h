/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

#define EXPR_EMPTY(first, second)
#define EXPR_EMPTY_ZERO()
#define EXPR_DROP(value) ((void) (value))
#define EXPR_DROP_CONSTANT(unused) ((void) 0)
#define EXPR_COMMA(left, right) ((left), (right))
#define EXPR_THREE(first, second, third) ((first), (second), (third))
#define EXPR_REPEAT(value) ((value), (value), (value))
#define EXPR_DROP_THEN(value, result) ((void) (value), (result))
#define EXPR_LAZY(flag, yes, no) \
    ((flag) ? ((void) (yes), 1U) : ((void) (no), 2L))
#define EXPR_LAZY_VOID(flag, yes, no) \
    ((flag) ? (void) (yes) : (void) (no))
#define EXPR_NESTED(left, right, result) \
    (((void) (left), (right)), (result))
#define EXPR_ATOMIC_POW2(value) (value > 0 && ((value) & ((value) - 1)) == 0)
#define EXPR_POINTER_CAST(value) ((char *) (value))
#define EXPR_STATEMENT(value) do { (void) (value); } while (0)
#define EXPR_BAD_LOOP(value) do { while (value) (value)--; } while (0)
#define EXPR_BAD_PASTE(value) value ## suffix
#define EXPR_BAD_STRINGIFY(value) #value
#define EXPR_BAD_VARIADIC(...) (__VA_ARGS__)
