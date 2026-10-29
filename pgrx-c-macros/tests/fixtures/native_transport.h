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

/* An enum parameter, and an enum result through a typedef, also need native
   transport, even when binding storage matches the compatible integer type. */
enum transport_color { TRANSPORT_RED, TRANSPORT_GREEN };
typedef enum transport_color transport_color_t;
static inline int transport_color_value(enum transport_color color) { return (int)color; }
#define TRANSPORT_COLOR_VALUE(color) transport_color_value((color))
static inline transport_color_t transport_color_pick(int green) {
    return green ? TRANSPORT_GREEN : TRANSPORT_RED;
}
#define TRANSPORT_COLOR_PICK(green) transport_color_pick((green))

/* So does a record passed by value. */
struct transport_pair { int first; int second; };
static inline int transport_pair_sum(struct transport_pair pair) { return pair.first + pair.second; }
#define TRANSPORT_PAIR_SUM(pair) transport_pair_sum((pair))
