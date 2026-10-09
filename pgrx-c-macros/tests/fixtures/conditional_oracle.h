//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef struct ConditionalRecord {
    unsigned char byte;
    unsigned int total;
} ConditionalRecord;
typedef struct ConditionalHooks {
    void (*callback)(ConditionalRecord *pointer, unsigned int value);
} ConditionalHooks;

extern unsigned int conditional_trace;
extern unsigned int conditional_calls;
extern unsigned int conditional_float_left;
extern unsigned int conditional_float_right;
extern unsigned int conditional_float_places;
unsigned int conditional_record(unsigned int digit, unsigned int value);
void conditional_store(ConditionalRecord *pointer, unsigned int digit, unsigned int value);
void conditional_hook(ConditionalRecord *pointer, unsigned int value);
double conditional_float_record(unsigned int operand, double value);
double *conditional_float_place(double *pointer);

#define COND_CHOOSE(pointer, condition, yes, no) \
    do { if (condition) (pointer)->byte = (yes); else (pointer)->byte = (no); } while (0)
#define COND_REPEAT(pointer, condition, value) \
    do { \
        if (condition) { (pointer)->byte = (value); (pointer)->total = (value); } \
        else (pointer)->total = (value); \
    } while (0)
#define COND_DANGLING(pointer, outer, inner) \
    if (outer) if (inner) (pointer)->total = 11; else (pointer)->total = 22
#define COND_BLOCKS(pointer, condition) \
    do { \
        if (condition) { do { ; } while (0); } else { } \
        (pointer)->total += 1; \
    } while (0)
#define COND_EMPTY(condition) do { if (condition) ; else ; } while (0)
#define COND_MUTATE(pointer) \
    do { if ((pointer)->total++) (pointer)->byte = 1; else (pointer)->byte = 2; } while (0)
#define COND_NATIVE(pointer) \
    do { \
        if (conditional_record(1, (pointer)->total)) conditional_store((pointer), 2, 20); \
        else conditional_store((pointer), 3, 30); \
        conditional_store((pointer), 4, (pointer)->total + 1); \
    } while (0)
#define COND_HOOK(table, pointer, value) \
    do { if ((table)->callback) (table)->callback((pointer), (value)); } while (0)
#define COND_SCOPES(pointer, condition, value) \
    do { \
        if (condition) { unsigned int then_local = (value); (pointer)->byte = then_local; } \
        else { unsigned int else_local = (value) + 1; (pointer)->byte = else_local; } \
    } while (0)
#define COND_INIT_BOTH(pointer, condition, yes, no) \
    do { \
        unsigned int merged; \
        if (condition) merged = (yes); else merged = (no); \
        (pointer)->total = merged; \
    } while (0)
#define COND_INIT_TEST(pointer, value) \
    do { \
        unsigned int condition_local; \
        if ((condition_local = (value))) (pointer)->byte = 1; else (pointer)->byte = 2; \
        (pointer)->total = condition_local; \
    } while (0)
#define COND_CAPTURE(condition, value) \
    do { if (condition) context->total = (value); else context->total = 0; } while (0)
#define COND_RETURN_ALL(condition, yes, no) \
    do { if (condition) return (yes); else return (no); } while (0)
#define COND_RAW_RETURN(condition) if (condition) return -1; else return 256
#define COND_RETURN_IF(pointer, condition, value) \
    do { \
        if (condition) return (value); \
        (pointer)->total += 5; \
    } while (0)
#define COND_INIT_SURVIVOR(pointer, condition, value) \
    do { \
        unsigned int surviving; \
        if (condition) return (value); else surviving = (value) + 1; \
        (pointer)->total = surviving; \
        return surviving; \
    } while (0)
#define COND_RETURN_ALIAS(condition, yes, no) \
    do { if (condition) { COND_RETURN_ALL(1, yes, no); } else { return (no); } } while (0)
#define COND_USE(value) ((value) + 1)
#define COND_FLOAT_ADD(pointer, condition, left, right) \
    do { if (condition) *(pointer) += (left) * (right); } while (0)
#define COND_FLOAT_MUL(pointer, condition, left, right) \
    do { if (condition) *(pointer) *= (left) + (right); } while (0)

#define COND_INIT_ONE(pointer, condition) \
    do { unsigned int unwritten; if (condition) unwritten = 1; (pointer)->total = unwritten; } while (0)
#define COND_INIT_COMPOUND(pointer, condition) \
    do { unsigned int unwritten; if (condition) unwritten = 1; else unwritten += 1; (pointer)->total = unwritten; } while (0)
#define COND_INIT_CONDITION(pointer) \
    do { unsigned int unwritten; if (unwritten) unwritten = 1; else unwritten = 2; (pointer)->total = unwritten; } while (0)
