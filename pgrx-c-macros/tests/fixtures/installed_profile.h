/* PostgreSQL 18 c.h's TYPEALIGN definition, with no target system-header dependency. */
typedef __UINTPTR_TYPE__ uintptr_t;
#define TYPEALIGN(ALIGNVAL,LEN) \
    (((uintptr_t) (LEN) + ((ALIGNVAL) - 1)) & ~((uintptr_t) ((ALIGNVAL) - 1)))
