/* Portions Copyright 2026 PgCentral Foundation, Inc. */
/* Use of this source code is governed by the MIT license. */
#include <stdio.h>
_Static_assert(_Generic(CALLBACK_LONG((struct CallbackTable *)0, 1), long: 1, default: 0), "long callback result");
_Static_assert(_Generic(CALLBACK_WIDE((struct CallbackTable *)0, 1), long long: 1, default: 0), "long long callback result");
_Static_assert(__builtin_types_compatible_p(CallbackStateValue, unsigned int (*)(unsigned int)), "the compiler's enum-compatible type also preserves the function prototype");
int main(void) {
    struct CallbackTable *p = callback_table();
    callback_reset();
    for (int i = -2; i < 260; i += 13) {
        unsigned int byte = CALLBACK_BYTE(p, i);
        long lng = CALLBACK_LONG(p, i);
        long long wide = CALLBACK_WIDE(p, i);
        int integer = CALLBACK_INT(p, i);
        printf("values %d %u %ld %lld %d\n", i, byte, lng, wide, integer);
    }
    int slot = 17;
    int read = CALLBACK_READ(p, &slot);
    int written = CALLBACK_WRITE(p, &slot, 41);
    CALLBACK_CLEAR(p, &slot);
    printf("place %d %d %d\n", read, written, slot);
    int direct = CALLBACK_DIRECT(CALLBACK_MEMBER(p), 9);
    int factory = CALLBACK_FACTORY(p, 1, 9);
    int global = CALLBACK_GLOBAL(9);
    int noargs = CALLBACK_NOARGS(p);
    int returned = CALLBACK_GET_CALL(1, 9);
    printf("calls %d %d %d %d %d\n", direct, factory, global, noargs, returned);
    int generic = CALLBACK_TWO(CALLBACK_WRITE_MEMBER(p), &slot, 7);
    printf("generic %d %d\n", generic, slot);
    int dereferenced = CALLBACK_DEREF(CALLBACK_MEMBER(p), 11);
    int addressed = CALLBACK_DIRECT(CALLBACK_ADDRESS(CALLBACK_MEMBER(p)), 11);
    int original = CALLBACK_DIRECT(CALLBACK_ORIGINAL_ADDRESS(), 11);
    int designator = CALLBACK_DIRECT(CALLBACK_ORIGINAL_VALUE(), 11);
    printf("designators %d %d %d %d %d\n", dereferenced, addressed, original, designator, CALLBACK_ADDRESS(CALLBACK_GET(0)) == 0);
    struct CallbackRecord full = CALLBACK_RECORD(p, ((struct CallbackRecord){13}));
    int full_value = CALLBACK_RECORD_VALUE(full);
    struct CallbackRecord again = CALLBACK_RECORD(p, full);
    printf("record %d %d\n", full_value, CALLBACK_RECORD_VALUE(again));
    CallbackPartialRecord partial = CALLBACK_PARTIAL(p, 17);
    int partial_value = CALLBACK_RECORD_VALUE(partial);
    CallbackPartialRecord copied = CALLBACK_PARTIAL_IDENTITY(p, partial);
    int taken = CALLBACK_PARTIAL_TAKE(p, copied);
    int member = CALLBACK_PARTIAL_MEMBER(p, 17);
    printf("partial %d %d %d\n", partial_value, taken, member);
    unsigned int states[] = {0U, 1U, 2U, 77U, 0xFFFFFFFFU};
    for (unsigned int index = 0; index < sizeof(states)/sizeof(states[0]); ++index) {
        unsigned int value = states[index];
        unsigned int returned = CALLBACK_STATE(p, value);
        unsigned int cast = CALLBACK_STATE_VALUE(p, value);
        printf("enum %u %u %u\n", value, returned, cast);
    }
    unsigned int named = CALLBACK_STATE(p, CallbackOne);
    printf("enum_named %u\n", named);
    int even = CALLBACK_NEEDED(14U);
    int odd = CALLBACK_NEEDED(15U);
    callback_set_predicate(0);
    int skipped = 0;
    int absent = CALLBACK_NEEDED((++skipped, 14U));
    printf("predicate %d %d %d %d\n", even, odd, absent, skipped);
    printf("void_address %d\n", CALLBACK_ADDRESS((void*)0) == 0);
    struct CallbackTable empty = {0};
    int untouched = 0;
    int lazy = CALLBACK_LAZY(&empty, 0, ++untouched);
    printf("lazy %d %d %d %u\n", lazy, untouched, CALLBACK_GET(0)==0, callback_count());
    return 0;
}
