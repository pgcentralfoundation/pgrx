/* Portions Copyright 2026 PgCentral Foundation, Inc. */
/* Use of this source code is governed by the MIT license. */
struct Capture { int slots[3]; };
#define CAP_READ(i) (scope->slots[(i)])
#define CAP_SET(i,v) (scope->slots[(i)] = (v))
#define CAP_COUNTER(d) ((counter) += (d))
#define CAP_LAZY(c) ((c) ? scope->slots[0] : 17)
#define CAP_SIZE() (sizeof(scope->slots))
#define CAP_TWO() ((left) - (right))
