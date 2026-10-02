/* Portions Copyright 2026 PgCentral Foundation, Inc. */
/* Use of this source code is governed by the MIT license. */
#include <stdbool.h>
#include <stdint.h>
typedef unsigned char (*CallbackByte)(unsigned char);
typedef long (*CallbackLong)(long);
typedef long long (*CallbackLongLong)(long long);
typedef int (*CallbackInt)(int);
typedef int (*CallbackRead)(const int *);
typedef int (*CallbackWrite)(int *, int);
typedef void (*CallbackVoid)(int *);
typedef CallbackInt (*CallbackFactory)(int);
typedef int (*CallbackNoArgs)(void);
typedef unsigned int CallbackOid;
typedef bool (*CallbackPredicate)(CallbackOid);
typedef int (*CallbackVariadic)(int, ...);
typedef int (*CallbackUnprototyped)();
struct CallbackRecord { int value; };
typedef struct CallbackRecord (*CallbackRecordValue)(struct CallbackRecord);
typedef enum CallbackState { CallbackOne = 1, CallbackTwo = 2 } CallbackState;
typedef CallbackState (*CallbackStateValue)(CallbackState);
typedef struct CallbackPartialRecord {
    int value;
    _Bool untouched;
    CallbackState state;
} CallbackPartialRecord;
typedef CallbackPartialRecord (*CallbackPartial)(int);
typedef CallbackPartialRecord (*CallbackPartialIdentity)(CallbackPartialRecord);
typedef int (*CallbackPartialTake)(CallbackPartialRecord);
struct CallbackTable {
    CallbackByte byte;
    CallbackLong lng;
    CallbackLongLong wide;
    CallbackInt integer;
    CallbackRead read;
    CallbackWrite write;
    CallbackVoid clear;
    CallbackFactory factory;
    CallbackNoArgs noargs;
    CallbackVariadic variadic;
    CallbackUnprototyped unprototyped;
    CallbackRecordValue record;
    CallbackPartial partial;
    CallbackPartialIdentity partial_identity;
    CallbackPartialTake partial_take;
    CallbackStateValue state;
};
extern CallbackInt callback_global;
extern CallbackPredicate callback_predicate;
struct CallbackTable *callback_table(void);
void callback_reset(void);
unsigned int callback_count(void);
CallbackInt callback_get(int);
int callback_original(short);
void callback_set_predicate(int);

#define CALLBACK_DIRECT(fn,x) (((fn))(x))
#define CALLBACK_TWO(fn,p,x) (((fn))((p),(x)))
#define CALLBACK_BYTE(p,x) ((p)->byte(x))
#define CALLBACK_LONG(p,x) ((p)->lng(x))
#define CALLBACK_WIDE(p,x) ((p)->wide(x))
#define CALLBACK_INT(p,x) ((p)->integer(x))
#define CALLBACK_READ(p,x) ((p)->read(x))
#define CALLBACK_WRITE(p,slot,x) ((p)->write((slot),(x)))
#define CALLBACK_CLEAR(p,slot) ((p)->clear(slot))
#define CALLBACK_NOARGS(p) ((p)->noargs())
#define CALLBACK_FACTORY(p,choose,x) ((p)->factory(choose)(x))
#define CALLBACK_GLOBAL(x) (callback_global(x))
#define CALLBACK_MEMBER(p) ((p)->integer)
#define CALLBACK_WRITE_MEMBER(p) ((p)->write)
#define CALLBACK_LAZY(p,c,x) ((c) ? (p)->integer(x) : 0)
#define CALLBACK_GET(choose) (callback_get(choose))
#define CALLBACK_GET_CALL(choose,x) (callback_get(choose)(x))
#define CALLBACK_GET_ADDRESS() (&callback_get)
#define CALLBACK_DEREF(fn,x) ((*((fn)))(x))
#define CALLBACK_ADDRESS(fn) (&*((fn)))
#define CALLBACK_ORIGINAL_ADDRESS() (&callback_original)
#define CALLBACK_ORIGINAL_VALUE() (callback_original)
#define CALLBACK_VARIADIC(p,x) ((p)->variadic(x))
#define CALLBACK_UNPROTOTYPED(p,x) ((p)->unprototyped(x))
#define CALLBACK_RECORD(p,x) ((p)->record(x))
#define CALLBACK_RECORD_VALUE(x) ((x).value)
#define CALLBACK_PARTIAL(p,x) ((p)->partial(x))
#define CALLBACK_PARTIAL_IDENTITY(p,x) ((p)->partial_identity(x))
#define CALLBACK_PARTIAL_TAKE(p,x) ((p)->partial_take(x))
#define CALLBACK_PARTIAL_MEMBER(p,x) ((p)->partial(x).value)
#define CALLBACK_NEEDED(x) (!callback_predicate ? false : (*callback_predicate)(x))
#define CALLBACK_STATE(p,x) ((p)->state(x))
#define CALLBACK_STATE_VALUE(p,x) ((unsigned int)(p)->state(x))
