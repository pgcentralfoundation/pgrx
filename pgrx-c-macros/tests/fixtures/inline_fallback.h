/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

typedef enum FallbackShade { FALLBACK_LIGHT = 1, FALLBACK_DARK = 2 } FallbackShade;
/* No generated adapter calls a function with this calling convention. It returns no value
 * because Clang 14 and 15 restore %rax on return from an unoptimized x86_64 preserve_most
 * function. */
static inline void __attribute__((preserve_most)) FALLBACK_HELPER(int *value) { *value *= 10; }
/* The body analyzes but cannot be rendered, so the macro calls the original function. Its
 * enum parameter needs a generated C thunk. */
static inline int FALLBACK_RANK(FallbackShade shade)
{
	int rank = (int) shade;

	FALLBACK_HELPER(&rank);
	return rank;
}
/* Refused while rendering, after demand planning saw its call to FALLBACK_RANK. */
#define FALLBACK_REFUSED(shade, rank) (FALLBACK_HELPER(rank), FALLBACK_RANK(shade))
typedef int (*FallbackRankFunction)(FallbackShade);
/* Uses only FALLBACK_RANK's address. */
#define FALLBACK_ADDRESS() ((FallbackRankFunction) &FALLBACK_RANK)
typedef enum FallbackColor { FALLBACK_RED = 1, FALLBACK_BLUE = 2 } FallbackColor;
static inline int FALLBACK_COLOR_USE(int value) { return value + FALLBACK_BLUE; }
/* An external function whose C type only it and the thunks for native calls below
 * witness, with different Rust ABIs. */
int FallbackDouble(int value) { return value * 2; }
#define FALLBACK_DOUBLE_ADDRESS() (&FallbackDouble)
/* Its translation falls back to the native call. */
static inline int FALLBACK_NEXT(int value)
{
	FALLBACK_HELPER(&value);
	return value + 1;
}
/* A macro calls this function natively. */
static inline int FallbackPlain(int value) { return value + 1; }
#define FALLBACK_PLAIN_CALL(value) FallbackPlain(value)
