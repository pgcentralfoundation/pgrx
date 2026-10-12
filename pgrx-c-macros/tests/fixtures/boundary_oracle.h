//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#define BOUND_SUM(x) (x) + 1
#define BOUND_NEG(x) -(x)
#define BOUND_CAST(p) (int *)(p)
#define BOUND_SET(result, x) (result) = (x) / 3
#define BOUND_SELECT(c) (c) ? 2 : 3
#define BOUND_ATOMIC(x) x + 1
#define BOUND_ALIAS(x) BOUND_SUM(x)
#define BOUND_ALIAS_GROUP(x) (BOUND_SUM(x))
#define BOUND_LVALUE(p) *(p)
#define BOUND_SIZE(x) (sizeof(x))
#define BOUND_USE(x) ((x) * 3)
