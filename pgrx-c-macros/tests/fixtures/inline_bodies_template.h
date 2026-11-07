/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

/* inline_bodies.h enters this file twice, with different macro and conditional state.
 * Clang treats the file as guarded by BODY_TEMPLATE_DISABLED, which is never defined. */
#ifndef BODY_TEMPLATE_DISABLED
static inline int BODY_TEMPLATE_VALUE_NAME(void) { return BODY_TEMPLATE_VALUE; }
static inline int BODY_TEMPLATE_BRANCH_NAME(int value)
{
	return (value
#ifdef BODY_TEMPLATE_PLUS
			+ 1
#endif
		);
}
#endif
