/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

/* A plain-char prototype needs a generated native-transport adapter, which only
   a static inline definition can receive. */
static int transport_widen(char value) { return value; }
#define TRANSPORT_WIDEN(value) transport_widen((value))

/* The same prototype on a static inline definition, which batch generation
   calls through its generated adapter. */
static inline int transport_inline_widen(char value) { return value; }
#define TRANSPORT_INLINE_WIDEN(value) transport_inline_widen((value))
