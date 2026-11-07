/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

#include <stddef.h>

/* Each macro needs a capability that only batch generation creates: a callback
   signature, an original function address getter, or a field or offset adapter. */
typedef int (*capability_callback)(int);
struct capability_record { int value; capability_callback call; };
extern capability_callback capability_global;
int capability_original(int);

#define CAPABILITY_ADDRESS() (&capability_original)
#define CAPABILITY_GLOBAL_CALL(x) (capability_global(x))
#define CAPABILITY_FIELD(p) ((p)->value)
#define CAPABILITY_OFFSET() offsetof(struct capability_record, value)
