/* Portions Copyright 2026 PgCentral Foundation, Inc. */
/* Use of this source code is governed by the MIT license. */
#include <stdio.h>
int main(void) {
    for (int i = -100; i < 100; i += 7) {
        PartialRecord value = PARTIAL_RECORD(i);
        PartialRecord other = PARTIAL_IDENTITY(value);
        PartialRecord assigned;
        PartialRecord result = PARTIAL_ASSIGN(assigned, other);
        int changed = PARTIAL_MUTATE(assigned, i + 17);
        int first = PARTIAL_FIRST(result);
        int inner = PARTIAL_INNER(result);
        int taken = PARTIAL_TAKE(PARTIAL_CHOOSE(i & 1, result, assigned));
        printf("record %d %d %d %d %d %zu %d\n", i, first, inner, changed, taken, PARTIAL_SIZE(PARTIAL_RECORD(++i)), PARTIAL_TEMP_INNER(i));
    }
    return 0;
}
