/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

/* A trigraph that spells a backslash and a bare carriage return can splice lines in ways
 * the search for pop pragmas does not model, so no body in this unit that names a macro
 * is translated. */
#define BODY_PHASE_OFFSET 1
static inline int BODY_PHASE_MACRO(int value) { return value + BODY_PHASE_OFFSET; }
static inline int BODY_PHASE_PLAIN(int value) { return value + 1; }
/* With -std=c17, a trigraph spells the splice. */
static inline int BODY_TRIGRAPH_SPLICE(int value)
{
	return (value
#if 0
		* 100
#endif ??/
		- 1
		);
}
/* Its body's lines end with a bare carriage return, as on classic Mac OS. */
static inline void BODY_CR_STORE(int *where, int value)
{	if (value)		*where = 1;#if 0		*where = 3;#endif	else		*where = 2;}
