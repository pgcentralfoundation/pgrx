#ifndef PGRX_QUALIFIED_VOID_H
#define PGRX_QUALIFIED_VOID_H

#ifndef QVOID_NULL
#define QVOID_NULL ((void *)0)
/* System headers can define the same null replacement at distinct origins. */
#define QVOID_NULL ((void *)0)
#endif

#define QVOID_CONST(pointer) ((const void *)(pointer))
#define QVOID_VOLATILE(pointer) ((volatile void *)(pointer))
#define QVOID_BOTH(pointer) ((const volatile void *)(pointer))
#define QVOID_REORDER(pointer) ((void volatile const *)(pointer))
#define QVOID_VALID(pointer) (QVOID_CONST(pointer) != QVOID_NULL)
#define QVOID_VOLATILE_VALID(pointer) (QVOID_VOLATILE(pointer) != QVOID_NULL)
#define QVOID_BOTH_VALID(pointer) (QVOID_BOTH(pointer) != QVOID_NULL)

#endif
