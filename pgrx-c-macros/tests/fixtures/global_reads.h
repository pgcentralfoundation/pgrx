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
/* A dereference, a function pointer read and a write still need the caller's unsafe. */
#define GLOBAL_POINTEE() (*GlobalPointer)
#define GLOBAL_HOOKED() (GlobalHookValue != 0)
#define GLOBAL_SET(value) (GlobalCount = (value))

#endif
