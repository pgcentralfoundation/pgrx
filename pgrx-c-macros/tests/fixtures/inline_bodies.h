/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

#define BODY_OFFSET 4
#define BODY_SCALE(value) ((value) * BODY_OFFSET)
static inline int BODY_MACRO(int value) { return BODY_SCALE(value) + BODY_OFFSET; }
#define BODY_DROP(value) 0
static inline int BODY_DROPPED(int value) { return value + BODY_DROP(BODY_SCALE(value)); }
static inline int BODY_BRANCH(int value)
{
#if BODY_OFFSET > 3
	return value + 1;
#else
	return value - 1;
#endif
}
static inline void BODY_STORE(int *where, int value)
{
	if (value >= 0)
		*where = value;
}
static inline void BODY_EARLY(int *where, int value)
{
	if (value < 0)
		return;
	*where = value;
}
typedef enum BodyColor { BODY_RED, BODY_BLUE } BodyColor;
static inline BodyColor BODY_COLOR(int value)
{
	if (value)
		return BODY_BLUE;
	return BODY_RED;
}
typedef struct BodyPair { int left; int right; } BodyPair;
static inline BodyPair BODY_PAIR(int value)
{
	BodyPair pair;

	pair.left = value;
	pair.right = value + 1;
	return pair;
}
static inline int BODY_LEAF(int value) { return value + 10; }
static inline int BODY_CALLER(int value) { return BODY_LEAF(value) * 2; }
static inline int BODY_RECURSIVE(int value) { return value <= 0 ? 0 : BODY_RECURSIVE(value - 1) + 1; }
enum { BODY_LATE = 5 };
static inline int BODY_LATE_USE(int value) { return value + BODY_LATE; }
#define BODY_LATE 7
#define BODY_REDEFINED 1
static inline int BODY_REDEFINED_USE(int value) { return value + BODY_REDEFINED; }
#undef BODY_REDEFINED
#define BODY_REDEFINED 2
static inline int BODY_LINE(void) { return __LINE__; }
#define BODY_ID(value) value
static inline int BODY_MACRO_ARG(int value) { return BODY_ID(value); }
/* The argument temporaries must still be alive when the body runs, as in a call. */
static inline void BODY_MARK(int *out, const int *witness)
{
	*out = 7;
}
/* A translated callee whose sizeof checks its argument's safety. */
static inline int BODY_SIZED(int value)
{
	int local = value;

	return (int) sizeof(local) + local;
}
static inline int BODY_SIZED_CALLER(int value) { return BODY_SIZED(value + 1); }
#define BODY_SIZED_MACRO(value) BODY_SIZED(value)
/* The comment continues the directive, which is `#if 2 - 1`. */
static inline int BODY_DIRECTIVE_COMMENT(int value)
{
	return (value
#if 2 /* the condition continues
	   on the next line */ - 1
#endif
		);
}
/* A spliced line continues the directive that closes a skipped group. Clang warns
 * about its extra tokens. */
static inline int BODY_DIRECTIVE_SPLICE(int value)
{
	return (value
#if 0
		* 100
#endif \
		- 1
		);
}
#define BODY_PUSHED_SCALE 1
#pragma push_macro("BODY_PUSHED_SCALE")
#undef BODY_PUSHED_SCALE
#define BODY_PUSHED_SCALE 1000
static inline int BODY_PUSHED(int value) { return value * BODY_PUSHED_SCALE; }
#pragma pop_macro("BODY_PUSHED_SCALE")
