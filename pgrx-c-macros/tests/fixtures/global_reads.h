/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

#ifndef PGRX_GLOBAL_READS_H
#define PGRX_GLOBAL_READS_H

extern void ExceptionalCondition(const char *conditionName, const char *fileName, int lineNumber)
	__attribute__((noreturn));

typedef enum GlobalMode { GLOBAL_IDLE, GLOBAL_BUSY } GlobalMode;
typedef void (*GlobalHook) (void);

extern int GlobalCount;
extern volatile int GlobalPending;
extern _Bool GlobalFlag;
extern int *GlobalPointer;
extern GlobalMode GlobalState;
extern GlobalHook GlobalHookValue;

/* Reads of integer, enum and data pointer globals. */
#define GLOBAL_COUNT() (GlobalCount)
#define GLOBAL_PENDING() (GlobalPending != 0)
#define GLOBAL_FLAG() (GlobalFlag)
#define GLOBAL_POINTER() (GlobalPointer)
#define GLOBAL_BUSY() (GlobalState == GLOBAL_BUSY)
/* A check whose condition reads a global, shaped like PostgreSQL's AssertMacro. */
#ifdef USE_ASSERT_CHECKING
#define GLOBAL_IN_RANGE(value) \
	((value) <= GlobalCount || (ExceptionalCondition("value <= GlobalCount", __FILE__, __LINE__), 0))
#else
#define GLOBAL_IN_RANGE(value) (1)
#endif
/* Several reads in one expansion share one thread check, which the first read to run makes.
 * An expansion that runs no read, as when `&&` skips them, makes none. */
#define GLOBAL_TOTAL() (GlobalCount + GlobalPending + GlobalFlag)
#define GLOBAL_ANY(skip) (!(skip) && GlobalCount > 0 && GlobalFlag)
/* A statement body shares one check too. */
#define GLOBAL_COPY(target) \
	do { \
		(target) = GlobalCount; \
		(target) += GlobalPending; \
	} while (0)
/* An operand evaluated for its effects expands the discard arm, which shares none. */
#define GLOBAL_AFTER(first) ((first), GlobalCount)
/* `sizeof` evaluates nothing, so none of these needs the caller's unsafe or checks the thread.
 * The operand of the first three holds no unsafe operation and gets no unsafe block. */
#define GLOBAL_SIZE() (sizeof(GlobalCount))
#define GLOBAL_SUM_SIZE() (sizeof(GlobalCount + GlobalPending))
#define GLOBAL_POINTEE_SIZE() (sizeof(*GlobalPointer))
/* A function pointer read is unsafe, and an operand of the caller's may be. */
#define GLOBAL_HOOK_SIZE() (sizeof(GlobalHookValue != 0))
#define GLOBAL_WIDER(value) (sizeof((value) + GlobalCount))
#define GLOBAL_POINTEE_WIDER() GLOBAL_WIDER(*GlobalPointer)
/* A dereference, a function pointer read and a write still need the caller's unsafe. */
#define GLOBAL_POINTEE() (*GlobalPointer)
#define GLOBAL_HOOKED() (GlobalHookValue != 0)
#define GLOBAL_SET(value) (GlobalCount = (value))

#endif
