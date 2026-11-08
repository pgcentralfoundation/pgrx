/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

#ifndef PGRX_INLINE_FUNCTIONS_H
#define PGRX_INLINE_FUNCTIONS_H

typedef enum WrapShade { WRAP_LIGHT = 1, WRAP_DARK = 2 } WrapShade;
/* Translated bodies whose prototypes the binding stores directly. */
static inline int WrapAdd(int left, int right) { return left + right; }
static inline int *WrapNext(int *value) { return value + 1; }
static inline void WrapStore(int *target, int value) { *target = value; }
/* A parameter named after a Rust keyword. */
static inline unsigned int WrapKeyword(unsigned int type) { return type * 2u; }
/* Translations that write a global, directly and through the translation they call. */
extern int WrapCounter;
static inline int WrapBump(int amount)
{
	WrapCounter += amount;
	return WrapCounter;
}
static inline int WrapBumpTwice(int amount)
{
	WrapBump(amount);
	return WrapBump(amount);
}
/* Translated, but its enum parameter needs native transport. */
static inline int WrapShadeRank(WrapShade shade) { return (int) shade * 10; }
/* No generated adapter calls a function with this calling convention, so the next body
 * keeps the native call. It returns no value because Clang 14 and 15 restore %rax on
 * return from an unoptimized x86_64 preserve_most function. */
static inline void __attribute__((preserve_most)) WRAP_HELPER(int *value) { *value *= 10; }
static inline int WrapNative(int value)
{
	WRAP_HELPER(&value);
	return value;
}
/* PostgreSQL's MSVC atomics pass a pointer to unsigned storage where `Interlocked*` takes
 * a pointer to `long` or `long long`. C converts neither pointer without a cast, so these
 * two bodies keep the native call. The third passes a compatible pointer. */
typedef struct WrapAtomic32 { volatile unsigned int value; } WrapAtomic32;
typedef struct WrapAtomic64 { volatile unsigned long long value; } WrapAtomic64;
extern long WrapExchange32(volatile long *target, long value);
extern long long WrapExchange64(volatile long long *target, long long value);
extern unsigned int WrapLoad32(volatile unsigned int *source);
static inline unsigned int WrapAtomicExchange32(volatile WrapAtomic32 *ptr, unsigned int value)
{
	return WrapExchange32(&ptr->value, value);
}
static inline unsigned long long WrapAtomicExchange64(volatile WrapAtomic64 *ptr,
													  unsigned long long value)
{
	return WrapExchange64(&ptr->value, value);
}
static inline unsigned int WrapAtomicRead32(volatile WrapAtomic32 *ptr)
{
	return WrapLoad32(&ptr->value);
}

#endif
