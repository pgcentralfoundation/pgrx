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

/*
 * A function with assertions whose name a release build gives to a macro that does work.
 * A release build's call to the function does nothing, so no call can stand for the macro.
 */
#ifdef USE_ASSERT_CHECKING
extern void AssertOrClear(int *value);
#else
#define AssertOrClear(value) (*(value) = 0)
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

/*
 * Checks that only an assert-enabled build declares, and a macro that makes one only
 * there. The macro holds both of its definitions, so a release build makes no call.
 */
#ifdef USE_ASSERT_CHECKING
extern void VerifyElement(const int *elements, int index);
extern void VerifyValue(int value);
#define VERIFIED_ELEMENT(elements, index) (VerifyElement(elements, index), (elements)[index])
#else
#define VERIFIED_ELEMENT(elements, index) ((elements)[index])
#endif

/* A definition with a statement that only one flavor has, as TupleDescCompactAttr has. */
static inline const int *
CheckedElement(const int *elements, int index)
{
	const int *element = &elements[index];
#ifdef USE_ASSERT_CHECKING
	/* Check that the element is populated. */
	VerifyElement(elements, index);
#endif
	return element;
}

/* A definition with a statement for each flavor. */
static inline int
FlavoredValue(int value)
{
	Assert(value >= 0);
#ifdef USE_ASSERT_CHECKING
	return value + 100;
#else
	return value + 200;
#endif
}

/* A definition whose statement of one flavor uses a value of a function that flavor alone has. */
static inline int
ConditionalBody(int value)
{
#ifdef USE_ASSERT_CHECKING
	value += AssertOnlyFunction(value);
#endif
	return value + 1;
}

/* One whose statement uses a macro that flavor alone has. */
static inline int
ConditionalMacro(int value)
{
#ifdef USE_ASSERT_CHECKING
	value = ASSERT_ONLY_MACRO(value);
#endif
	return value + 1;
}

/* Tokens of one flavor that close one block and open another. */
static inline int
ConditionalBraces(int value)
{
	if (value > 0)
	{
		value += 1;
#ifdef USE_ASSERT_CHECKING
	}
	else
	{
#endif
		value += 2;
	}
	return value;
}

/* Tokens of one flavor inside an expression. */
static inline int
ConditionalOperand(int value)
{
	return value
#ifdef USE_ASSERT_CHECKING
		+ 10
#endif
		;
}

/* A statement of one flavor that is the branch of a conditional. */
static inline int
ConditionalBranch(int value)
{
	if (value > 0)
#ifdef USE_ASSERT_CHECKING
		VerifyValue(value);
#endif
	value += 5;
	return value;
}

/* A local that each flavor declares and both use afterwards. */
static inline int
ConditionalLocal(int value)
{
#ifdef USE_ASSERT_CHECKING
	int step = 1;
#else
	int step = 2;
#endif
	return value + step;
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

/*
 * Macros whose two definitions do different work. Each build must do what its own does.
 */
#define SLOT_LIMIT 8
extern int WorkCounterVariable;
#ifdef USE_ASSERT_CHECKING
/* A release build wraps where an assert-enabled build checks. */
#define NEXT_SLOT(slot) (AssertMacro((slot) + 1 < SLOT_LIMIT), (slot) + 1)
/* Work that only an assert-enabled build does, as an expression and as a statement. */
#define COUNT_CHECKED(value) (WorkCounterVariable++, AssertMacro((value) >= 0))
#define COUNT_STATEMENT(value) do { WorkCounterVariable += 2; Assert((value) >= 0); } while (0)
/* Parameters that the release definition names otherwise. */
#define RENAMED(first, second) ((first) - (second))
/* Definitions that have one type for a `long` operand and two for an `int`. */
#define WIDENED(value) ((value) + 1L)
#else
#define NEXT_SLOT(slot) (((slot) + 1) % SLOT_LIMIT)
#define COUNT_CHECKED(value) ((void)1)
#define COUNT_STATEMENT(value) ((void)1)
#define RENAMED(second, first) ((second) - (first) + 100)
#define WIDENED(value) ((value) + 0)
#endif
/*
 * An assertion with assertions and an assumption without, as PostgreSQL 19's pg_assume.
 * A release build evaluates the operand, which the assertion it replaces would not.
 */
#ifdef USE_ASSERT_CHECKING
#define flavor_assume(holds) Assert(holds)
#else
#define flavor_assume(holds) do { if (!(holds)) __builtin_unreachable(); } while (0)
#endif
/* Uses of them inside other macros, one as the unbraced branch of an `if`. */
#define SLOT_AFTER(slot) (NEXT_SLOT(slot) + 1)
#define COUNT_IF(flag, value) \
	do { if (flag) COUNT_STATEMENT(value); else WorkCounterVariable += 10; } while (0)
/* A check as the unbraced branch of an `if`, and checks inside a comma expression. */
#define CHECK_IF(flag, value) \
	do { if (flag) Assert((value) > 0); else WorkCounterVariable += 100; } while (0)
#define CHECK_THEN(value) (AssertMacro((value) > 0), AssertMacro((value) < 100), (value) + 1)
/* Unreachable code behind a condition. */
#define UNREACHABLE_UNLESS(flag) do { if (!(flag)) flavor_unreachable(); } while (0)

/*
 * Pairs of definitions that no one definition can hold: a statement beside a value, two
 * types, a replacement list that an operator of its use could bind, and a name that only
 * a release build declares. A macro that uses one follows it.
 */
#ifdef USE_ASSERT_CHECKING
#define MIXED_FORMS(value) do { WorkCounterVariable += (value); } while (0)
#define MIXED_TYPES(value) ((long) (value))
#define OPEN_SUM(value) (value) + 1
#define NEEDS_RELEASE_ONLY(value) (value)
#else
#define MIXED_FORMS(value) (value)
#define MIXED_TYPES(value) ((unsigned int) (value))
#define OPEN_SUM(value) (value) + 2
#define NEEDS_RELEASE_ONLY(value) (ReleaseOnlyFunction(value))
#endif
#define USES_MIXED_FORMS(value) (MIXED_FORMS(value) + 1)
#define USES_MIXED_TYPES(value) (MIXED_TYPES(value) < 0)
#define USES_OPEN_SUM(value) (2 * OPEN_SUM(value))

#endif
