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
/* A translation copies a record value, so an array member of one would decay into a
 * pointer into that copy. The type of a call's result through a pointer, and so of its
 * members, is not established. */
typedef struct BodyArray { int values[2]; int count; } BodyArray;
static inline BodyArray BODY_ARRAY(int value)
{
	BodyArray array;

	array.values[0] = value;
	array.values[1] = value + 1;
	array.count = 2;
	return array;
}
static inline int BODY_ARRAY_COUNT(int value) { return value + BODY_ARRAY(value).count; }
static inline int BODY_ARRAY_INDEX(int value) { return value + BODY_ARRAY(value).values[1]; }
static inline int BODY_ARRAY_DECAY(int value) { return value + *BODY_ARRAY(value).values; }
static inline int BODY_ARRAY_SELECT(const BodyArray *left, const BodyArray *right, int value) { return value + (value ? *left : *right).values[1]; }
typedef struct BodyArrayMaker { BodyArray (*make)(int); } BodyArrayMaker;
static inline int BODY_MADE_COUNT(const BodyArrayMaker *maker, int value) { return value + maker->make(value).count; }
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
/* The name a macro's replacement uses is undefined at the definition, then popped. */
#define BODY_POPPED_VALUE 1000
#pragma push_macro("BODY_POPPED_VALUE")
#undef BODY_POPPED_VALUE
typedef enum BodyPoppedEnum { BODY_POPPED_VALUE = 5 } BodyPoppedEnum;
#define BODY_POPPED_SCALE BODY_POPPED_VALUE
static inline int BODY_POPPED(int value) { return value + BODY_POPPED_SCALE; }
#pragma pop_macro("BODY_POPPED_VALUE")
/* The same, with a pop pragma whose name a line splice divides. */
#define BODY_SPLICED_POP_VALUE 1000
#pragma push_macro("BODY_SPLICED_POP_VALUE")
#undef BODY_SPLICED_POP_VALUE
typedef enum BodySplicedPopEnum { BODY_SPLICED_POP_VALUE = 5 } BodySplicedPopEnum;
#define BODY_SPLICED_POP_SCALE BODY_SPLICED_POP_VALUE
static inline int BODY_SPLICED_POP(int value) { return value + BODY_SPLICED_POP_SCALE; }
#pragma pop_mac\
ro("BODY_SPLICED_POP_VALUE")
/* Pasting forms the names of these pop pragmas, which their text therefore lacks. The
 * body uses a pushed definition that is neither the last one nor active at the end. */
#define BODY_PRAGMA(text) _Pragma(#text)
#define BODY_PRAGMA_EXPANDED(text) BODY_PRAGMA(text)
#define BODY_JOIN(left, right) left##right
#define BODY_HIDDEN_SCALE 1
#pragma push_macro("BODY_HIDDEN_SCALE")
#define BODY_HIDDEN_SCALE 1000
#pragma push_macro("BODY_HIDDEN_SCALE")
#define BODY_HIDDEN_SCALE 1
BODY_PRAGMA_EXPANDED(BODY_JOIN(pop_, macro)("BODY_HIDDEN_SCALE"))
static inline int BODY_HIDDEN(int value) { return value * BODY_HIDDEN_SCALE; }
BODY_PRAGMA_EXPANDED(BODY_JOIN(pop_, macro)("BODY_HIDDEN_SCALE"))
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
/* A function-like macro that only rescanning a macro's result expands has no expansion
 * record. It is undefined afterwards, and a function has its name. */
static inline int BODY_RESCAN_TARGET(int value) { return value + 1; }
#define BODY_RESCAN_TARGET(value) ((value) * 100)
#define BODY_RESCAN_SELECT(name) name
#define BODY_RESCAN_APPLY(name, value) name(value)
static inline int BODY_RESCAN(int value) { return BODY_RESCAN_SELECT(BODY_RESCAN_TARGET)(value); }
static inline int BODY_RESCAN_ARGUMENT(int value) { return BODY_RESCAN_APPLY(BODY_RESCAN_TARGET, value); }
#undef BODY_RESCAN_TARGET
/* Without -ffp-contract=off, a body that multiplies and adds has no floating profile, so
 * its own floating values keep the native call. Arguments still convert to their
 * integer parameters as in a call. */
static inline double BODY_FMA(double a, double b, double c) { return a * b + c; }
static inline int BODY_INT_FMA(int a, double unused) { return a * 2 + 1; }
static inline int BODY_ORDER(int a, int b, int c) { return a * 100 + b * 10 + c; }
/* Floating objects reached through a member, an index or a dereference, one written
 * from integer arithmetic, and a call's result also need that profile. A floating
 * result converted from integer arithmetic does not, as the conversion is not part of
 * the arithmetic. */
typedef struct BodyScaled { double scale; int count; } BodyScaled;
static inline int BODY_FIELD_FMA(const BodyScaled *record, int value) { return value * 2 + (int) record->scale; }
static inline int BODY_INDEX_FMA(const double *values, int value) { return value * 2 + (int) values[1]; }
static inline int BODY_DEREF_FMA(const double *value, int count) { return count * 2 + (int) *value; }
static inline void BODY_STORE_FMA(BodyScaled *record, int value) { record->scale = value * 2 + 1; }
/* Analysis does not establish the result type of a call through a member. */
typedef struct BodyScaler { double (*scale)(void); } BodyScaler;
static inline int BODY_CALL_FMA(const BodyScaler *scaler, int value) { return value * 2 + (int) scaler->scale(); }
static inline double BODY_RETURN_FMA(int value) { return value * 2 + 1; }
/* Stringification spells its argument with the body's original spacing. */
#define BODY_STRING(text) #text
#define BODY_STRING_OF(text) BODY_STRING(text)
static inline int BODY_STRING_SIZE(int value) { return value + (int) sizeof(BODY_STRING(1+2)); }
static inline int BODY_STRING_CHAR(int value) { return value + BODY_STRING_OF(1+2)[1]; }
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
#define BODY_GUARDED_NAME BODY_GUARDED_FIRST
#define BODY_GUARDED_VALUE 1
#include "inline_bodies_guarded.h"
#undef INLINE_BODIES_GUARDED_H
#undef BODY_GUARDED_NAME
#undef BODY_GUARDED_VALUE
#define BODY_GUARDED_NAME BODY_GUARDED_SECOND
#define BODY_GUARDED_VALUE 2
#include "inline_bodies_guarded.h"
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
/* With -std=c17, a trigraph spells the directive's `#`. */
static inline int BODY_TRIGRAPH_HASH(int value)
{
	return (value
#if 0
		* 100
??=endif \
		- 1
		);
}
