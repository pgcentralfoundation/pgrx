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
#define BODY_INNER 1
#define BODY_OUTER BODY_INNER
static inline int BODY_TRANSITIVE_USE(int value) { return value + BODY_OUTER; }
#undef BODY_INNER
#define BODY_INNER 2
#define BODY_DROPPED_NAME 1
static inline int BODY_DROPPED_LATER(int value) { return value + BODY_DROP(BODY_DROPPED_NAME); }
#undef BODY_DROPPED_NAME
#define BODY_DROPPED_NAME 2
enum { BODY_ARGUMENT = 5 };
static inline int BODY_ARGUMENT_LATE(int value) { return BODY_ID(BODY_ARGUMENT) + value; }
#define BODY_ARGUMENT 7
#define BODY_TEMPLATE_VALUE_NAME BODY_FIRST_VALUE
#define BODY_TEMPLATE_BRANCH_NAME BODY_FIRST_BRANCH
#define BODY_TEMPLATE_VALUE 1
#define BODY_TEMPLATE_PLUS
#include "inline_bodies_template.h"
#undef BODY_TEMPLATE_VALUE_NAME
#undef BODY_TEMPLATE_BRANCH_NAME
#undef BODY_TEMPLATE_VALUE
#undef BODY_TEMPLATE_PLUS
#define BODY_TEMPLATE_VALUE_NAME BODY_SECOND_VALUE
#define BODY_TEMPLATE_BRANCH_NAME BODY_SECOND_BRANCH
#define BODY_TEMPLATE_VALUE 2
#include "inline_bodies_template.h"
/* PostgreSQL 18 continues a signature onto the line of its opening brace, whose
 * spelling then begins with the splice. */
static inline int BODY_BRACE_SPLICE(int value) \
{
	return value + 3;
}
/* The directive that closes a skipped group begins with a line splice. */
static inline int BODY_SPLICED_HASH(int value)
{
	return (value
#if 0
		* 100
\
#endif \
		- 1
		);
}
/* The same directive is spelled with a digraph that a splice divides. */
static inline int BODY_SPLICED_DIGRAPH(int value)
{
	return (value
#if 0
		* 100
%\
:endif \
		- 1
		);
}
/* With -std=c17, trigraphs spell the splice or the directive's `#`. */
static inline int BODY_TRIGRAPH_SPLICE(int value)
{
	return (value
#if 0
		* 100
#endif ??/
		- 1
		);
}
static inline int BODY_TRIGRAPH_HASH(int value)
{
	return (value
#if 0
		* 100
??=endif \
		- 1
		);
}
/* Its body's lines end with a bare carriage return, as on classic Mac OS. */
static inline void BODY_CR_STORE(int *where, int value)
{	if (value)		*where = 1;#if 0		*where = 3;#endif	else		*where = 2;}
