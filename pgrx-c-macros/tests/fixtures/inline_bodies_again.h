/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

/* inline_bodies.h enters this file twice, and each entry defines the same macros again, as
 * Clang's <stddef.h> defines NULL each time a glibc header asks for it. */
#undef BODY_AGAIN
#ifdef BODY_AGAIN_DISABLED
#define BODY_AGAIN 0
#else
#define BODY_AGAIN 4
#endif
#undef BODY_AGAIN_GAP
#define BODY_AGAIN_GAP 5
/* A declaration is read while this one is undefined. */
#undef BODY_AGAIN_SEEN
struct BodyAgainSeen;
#define BODY_AGAIN_SEEN 6
