/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

typedef enum FallbackShade { FALLBACK_LIGHT = 1, FALLBACK_DARK = 2 } FallbackShade;
/* No generated adapter calls a function with this calling convention. */
static inline int __attribute__((preserve_most)) FALLBACK_HELPER(int value) { return value * 10; }
/* The body analyzes but cannot be rendered, so the macro calls the original function. Its
 * enum parameter needs a generated C thunk. */
static inline int FALLBACK_RANK(FallbackShade shade) { return FALLBACK_HELPER((int) shade); }
/* Refused while rendering, after demand planning saw its call to FALLBACK_RANK. */
#define FALLBACK_REFUSED(shade) (FALLBACK_RANK(shade) + FALLBACK_HELPER(1))
typedef int (*FallbackRankFunction)(FallbackShade);
/* Uses only FALLBACK_RANK's address. */
#define FALLBACK_ADDRESS() ((FallbackRankFunction) &FALLBACK_RANK)
typedef enum FallbackColor { FALLBACK_RED = 1, FALLBACK_BLUE = 2 } FallbackColor;
static inline int FALLBACK_COLOR_USE(int value) { return value + FALLBACK_BLUE; }
