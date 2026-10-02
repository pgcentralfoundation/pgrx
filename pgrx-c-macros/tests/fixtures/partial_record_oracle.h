/* Portions Copyright 2026 PgCentral Foundation, Inc. */
/* Use of this source code is governed by the MIT license. */
typedef enum PartialState { PartialOne = 1, PartialTwo = 2 } PartialState;
typedef struct PartialInner { int value; _Bool untouched; } PartialInner;
typedef struct PartialRecord {
    int first;
    _Bool untouched;
    PartialState state;
    PartialInner inner;
} PartialRecord;
PartialRecord partial_record(int value);
PartialRecord partial_identity(PartialRecord value);
int partial_take(PartialRecord value);

#define PARTIAL_RECORD(x) partial_record((x))
#define PARTIAL_IDENTITY(x) partial_identity((x))
#define PARTIAL_FIRST(x) ((x).first)
#define PARTIAL_INNER(x) ((x).inner.value)
#define PARTIAL_TAKE(x) partial_take((x))
#define PARTIAL_ASSIGN(dst,src) ((dst) = (src))
#define PARTIAL_MUTATE(dst,x) ((dst).first = (x))
#define PARTIAL_CHOOSE(c,x,y) ((c) ? (x) : (y))
#define PARTIAL_SIZE(x) (sizeof(x))
#define PARTIAL_TEMP_INNER(x) (partial_record((x)).inner.value)
