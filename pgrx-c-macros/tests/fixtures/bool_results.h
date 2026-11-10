/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

/*
 * Macros and inline functions whose C result is a truth value, which the generated macro
 * returns as a Rust bool, beside ones whose result is a number.
 */

#ifndef PGRX_BOOL_RESULTS_H
#define PGRX_BOOL_RESULTS_H

extern _Bool truth_native(int value);
extern int count_native(int value);
extern _Bool truth_flag;

static inline _Bool
truth_inline(int value)
{
	return value > 2;
}

/* The declared type decides, not the expression a function returns. */
static inline int
number_inline(int value)
{
	return value > 2;
}

/* Relational, equality and logical operators yield the int zero or one. */
#define B_EQUAL(a, b) ((a) == (b))
#define B_UNEQUAL(a, b) ((a) != (b))
#define B_LESS(a, b) ((a) < (b))
#define B_AT_LEAST(a, b) ((a) >= (b))
#define B_AND(a, b) ((a) && (b))
#define B_OR(a, b) ((a) || (b))
#define B_NOT(a) (!(a))
#define B_GROUPED(a, b) ((((a) > (b))))
/* The _Bool type: a cast, an object, a native call and an inline function. */
#define B_CAST(a) ((_Bool) (a))
#define B_FLAG() (truth_flag)
#define B_NATIVE(a) (truth_native(a))
#define B_INLINE(a) (truth_inline(a))
/* A comma expression and a statement expression take the type of their value. */
#define B_COMMA(a, b) ((void) (a), (b) != 0)
#define B_STATEMENT(a) ({ int seen = (a); seen != 0; })
/* A conditional of two truth values, or of one beside the constant zero or one. */
#define B_CONDITIONAL(a, b) ((a) ? (b) > 1 : (b) < -1)
#define B_CONDITIONAL_ZERO(a, b) ((a) ? (b) > 1 : 0)
#define B_CONDITIONAL_ONE(a, b) ((a) ? 1 : (b) > 1)
/* A wrapper keeps its callee's result. */
#define B_WRAPPER(a, b) B_EQUAL((a), (b))

/* Numbers: constants, arithmetic, other casts, and a caller's operand. */
#define N_CONSTANT(a) ((void) (a), 1)
#define N_BOTH_CONSTANTS(a) ((a) ? 1 : 0)
#define N_CONDITIONAL(a, b) ((a) ? (b) > 1 : 2)
#define N_SUM(a, b) ((a) + (b))
#define N_MASK(a) ((a) & 1)
#define N_INT_CAST(a, b) ((int) ((a) == (b)))
#define N_OPERAND(a) (a)
#define N_NATIVE(a) (count_native(a))
#define N_INLINE(a) (number_inline(a))
/* A truth-valued macro inside another macro is still the C int it was. */
#define N_COUNT(a, b) (B_EQUAL(a, b) + B_LESS(a, b))
#define N_SIZE(a) (sizeof(a))

#endif
