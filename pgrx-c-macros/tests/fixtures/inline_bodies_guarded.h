/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

/* inline_bodies.h undefines the guard and enters this file again. */
#ifndef INLINE_BODIES_GUARDED_H
#define INLINE_BODIES_GUARDED_H
static inline int BODY_GUARDED_NAME(void) { return BODY_GUARDED_VALUE; }
#endif
