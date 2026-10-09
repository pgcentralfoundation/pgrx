#ifndef PGRX_CONTEXT_ORACLE_H
#define PGRX_CONTEXT_ORACLE_H

typedef struct ContextRecord {
    unsigned short field;
    int array[3];
    unsigned int bit : 3;
    volatile int volatile_field;
} ContextRecord;

ContextRecord *context_base(ContextRecord *record);
void context_reset(void);
int context_count(void);

#define CTX_FIELD(p) ((p)->field)
#define CTX_ARRAY(p) ((p)->array)
#define CTX_BIT(p) ((p)->bit)
#define CTX_VOLATILE(p) ((p)->volatile_field)
#define CTX_IDENTITY(value) ((value))
#define CTX_SET(destination, value) ((destination) = (value))
#define CTX_MOD(destination, value) ((destination) += (value))
#define CTX_POST(destination) ((destination)++)
#define CTX_PRE(destination) (++(destination))
#define CTX_ADDRESS(value) (&(value))
#define CTX_SIZE(value) (sizeof(value))
#define CTX_VOID(value) ((void)(value))
#define CTX_COMMA(left, right) ((left), (right))
#define CTX_LAZY(condition, left, right) ((condition) ? (left) : (right))
#define CTX_REPEAT(value) ((value) + (value))
#define CTX_ATOMIC(value) (value + 1)
#define CTX_TYPE(type, value) ((type)(value))
#define CTX_TYPE_PROVEN(type, value) ((void) sizeof(type *), ((type)(value)))
#define CTX_IGNORE(unused)

#endif
