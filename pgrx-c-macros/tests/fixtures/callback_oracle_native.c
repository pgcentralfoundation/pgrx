/* Portions Copyright 2026 PgCentral Foundation, Inc. */
/* Use of this source code is governed by the MIT license. */
static unsigned int calls;
static unsigned char cb_byte(unsigned char value) { ++calls; return value + 1; }
static long cb_long(long value) { ++calls; return value + 1; }
static long long cb_wide(long long value) { ++calls; return value + 2; }
static int cb_int(int value) { ++calls; return value + 3; }
static int cb_read(const int *value) { ++calls; return *value; }
static int cb_write(int *slot, int value) { ++calls; return *slot = value; }
static void cb_clear(int *slot) { ++calls; *slot = 0; }
static CallbackInt cb_factory(int choose) { ++calls; return choose ? cb_int : 0; }
static int cb_noargs(void) { ++calls; return 19; }
static bool cb_predicate(unsigned int value) { ++calls; return value % 2 == 0; }
static struct CallbackRecord cb_record(struct CallbackRecord value) { ++calls; value.value += 5; return value; }
static CallbackPartialRecord cb_partial(int value) {
    ++calls;
    CallbackPartialRecord result;
    result.value = value + 7;
    return result;
}
static CallbackPartialRecord cb_partial_identity(CallbackPartialRecord value) { ++calls; return value; }
static int cb_partial_take(CallbackPartialRecord value) { ++calls; return value.value; }
static CallbackState cb_state(CallbackState value) { ++calls; return (CallbackState)((unsigned int)value + 5U); }
static struct CallbackTable table = {
    cb_byte, cb_long, cb_wide, cb_int, cb_read, cb_write, cb_clear, cb_factory, cb_noargs, 0, 0,
    cb_record, cb_partial, cb_partial_identity, cb_partial_take, cb_state
};
CallbackInt callback_global = cb_int;
CallbackPredicate callback_predicate = cb_predicate;
struct CallbackTable *callback_table(void) { return &table; }
void callback_reset(void) { calls = 0; }
unsigned int callback_count(void) { return calls; }
CallbackInt callback_get(int choose) { return choose ? cb_int : 0; }
int callback_original(short value) { ++calls; return value + 23; }
void callback_set_predicate(int enabled) { callback_predicate = enabled ? cb_predicate : 0; }
