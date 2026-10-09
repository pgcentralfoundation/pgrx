//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef struct StatementRecord {
    unsigned char byte;
    _Bool flag;
    unsigned int total;
} StatementRecord;

extern unsigned int statement_trace;
extern unsigned int statement_calls;
extern volatile unsigned int statement_signal;
unsigned int statement_record(unsigned int digit, unsigned int value);
void statement_store(StatementRecord *pointer, unsigned int value);

#define STMT_ASSIGN(pointer, value) \
    do { \
        (pointer)->byte = (value); \
        (pointer)->flag = (value); \
        (pointer)->total = (value); \
    } while (0)
#define STMT_LOCAL(pointer, value) \
    do { \
        unsigned int temporary = (value); \
        unsigned int *alias = &temporary; \
        *alias += 1; \
        (pointer)->total = temporary; \
    } while (0)
#define STMT_NESTED(pointer, value) \
    do { \
        { STMT_LOCAL(pointer, value); } \
        do { (pointer)->byte = (value); } while (0); \
        { (pointer)->flag = 1; } \
    } while (0)
#define STMT_ALIAS(pointer, value) STMT_ASSIGN(pointer, value)
#define STMT_CAPTURE(value) do { scope->total = (value); } while (0)
#define STMT_EMPTY() do { } while (0)
#define STMT_EMPTY_BLOCK() { }
#define STMT_NOTHING(value)
#define STMT_VOLATILE() do { statement_signal++; statement_signal += 1; } while (0)
#define STMT_VOID(pointer, value) \
    do { statement_store((pointer), (value)); (pointer)->byte = (value); } while (0)
#define STMT_DISCARD(pointer) do { (void) ((pointer)->byte++); (void) statement_signal; } while (0)
#define STMT_USE(value) ((value) + 1)

#define STMT_UNINITIALIZED(pointer) \
    do { unsigned int temporary; (pointer)->total = temporary; } while (0)
#define STMT_UNINITIALIZED_COMPOUND(pointer) \
    do { unsigned int temporary; temporary += 1; (pointer)->total = temporary; } while (0)
#define STMT_BRANCH(pointer) do { if (pointer) (pointer)->total = 1; } while (0)
#define STMT_UNSUPPORTED_LOOP(pointer) do { while (pointer) (pointer)->total++; } while (0)
