/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

/* inline_bodies.h enters this file twice, with different macro and conditional state. */
static inline int BODY_TEMPLATE_VALUE_NAME(void) { return BODY_TEMPLATE_VALUE; }
static inline int BODY_TEMPLATE_BRANCH_NAME(int value)
{
	return (value
#ifdef BODY_TEMPLATE_PLUS
			+ 1
#endif
		);
}
