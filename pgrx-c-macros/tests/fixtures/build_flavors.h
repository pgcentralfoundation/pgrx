/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

/* The ways PostgreSQL's headers depend on USE_ASSERT_CHECKING, in miniature. */

#ifndef PGRX_BUILD_FLAVORS_H
#define PGRX_BUILD_FLAVORS_H

extern void ExceptionalCondition(const char *conditionName, const char *fileName, int lineNumber)
	__attribute__((noreturn));
extern void abort(void) __attribute__((noreturn));

#ifndef USE_ASSERT_CHECKING
#define Assert(condition) ((void)1)
#define AssertMacro(condition) ((void)1)
#define TrapMacro(condition) (1)
#define flavor_unreachable() __builtin_unreachable()
#else
#define Assert(condition) \
	do { \
		if (!(condition)) \
			ExceptionalCondition(#condition, __FILE__, __LINE__); \
	} while (0)
#define AssertMacro(condition) \
	((void) ((condition) || \
			 (ExceptionalCondition(#condition, __FILE__, __LINE__), 0)))
#define TrapMacro(condition) \
	((_Bool) (! (condition) || \
			  (ExceptionalCondition(#condition, __FILE__, __LINE__), 0)))
#define flavor_unreachable() abort()
#endif

/* Defined once, in terms of a macro that differs. */
#define AssertPositive(value) Assert((value) > 0)
/* A value behind a check. */
#define CHECKED_HALF(value) (AssertMacro((value) % 2 == 0), (value) / 2)

/* Functions with assertions, and macros that do nothing without them. */
#ifdef USE_ASSERT_CHECKING
extern void AssertTransactionIdInAllowableRange(unsigned int xid);
extern void SlruPagePrecedesUnitTests(const int *ctl, int per_page);
#else
#define AssertTransactionIdInAllowableRange(xid) ((void)1)
#define SlruPagePrecedesUnitTests(ctl, per_page) do {} while (0)
#endif

/* A function with assertions, and a static inline function that does nothing without them. */
#ifdef USE_ASSERT_CHECKING
extern void AssertCouldGetRelation(void);
#else
static inline void
AssertCouldGetRelation(void)
{
}
#endif

/*
 * Names only one flavor declares. A macro whose assert-enabled definition calls a static
 * inline function among them, as castNode calls castNodeImpl, and one whose definition
 * calls an external function that a release build has no way to provide.
 */
#ifdef USE_ASSERT_CHECKING
extern int AssertOnlyFunction(int value);
extern int AssertOnlyVariable;
#define ASSERT_ONLY_MACRO(value) ((value) + 1)
static inline int
CheckedImpl(int tag, int value)
{
	Assert(tag == 1);
	return value;
}
#define CHECKED(value) (CheckedImpl(1, value))
#define CHECKED_TAG(tag, value) (CheckedImpl(tag, value))
#define NEEDS_ASSERT_ONLY(value) (AssertOnlyFunction(value))
#else
extern int ReleaseOnlyFunction(int value);
#define RELEASE_ONLY_MACRO(value) ((value) - 1)
#define CHECKED(value) (value)
#define CHECKED_TAG(tag, value) (value)
#define NEEDS_ASSERT_ONLY(value) (value)
#endif
#define USES_CHECKED(value) (CHECKED(value) + 1)
#define USES_NEEDS_ASSERT_ONLY(value) (NEEDS_ASSERT_ONLY(value) + 1)

/* A check that reads through its operand. */
static inline void
ValidateObject(const int *object)
{
	Assert(object);
	Assert(*object >= 0);
}

/* A definition that differs between the flavors. */
static inline int
ConditionalBody(int value)
{
#ifdef USE_ASSERT_CHECKING
	value += AssertOnlyFunction(value);
#endif
	return value + 1;
}

/*
 * A `sizeof` whose operand reads through a pointer only where assertions are enabled, and
 * one whose operand does not in either flavor.
 */
extern int *PointerVariable;
#define ASSERTED_SIZE() (sizeof(TrapMacro(*PointerVariable)))
#define PLAIN_SIZE() (sizeof(PointerVariable))

/* A stop that is no assertion keeps its condition's effects in both flavors. */
#define COUNT_OR_ABORT(counter) do { if (++(*(counter)) > 5) abort(); } while (0)

#endif
