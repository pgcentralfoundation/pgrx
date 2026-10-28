/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

typedef unsigned long long FrontWide;
typedef struct FrontNode FrontNode;
typedef int FrontCallback(const FrontNode *, unsigned long);
typedef FrontCallback *FrontCallbackPointer;
typedef int FrontArray[4];
typedef int FrontIncompleteArray[];
typedef const volatile FrontNode *const FrontConstPointer;

struct FrontNode {
    unsigned int flags : 7;
    signed int code : 5;
    unsigned int : 0;
    FrontNode *next;
    const FrontNode *parent;
    volatile unsigned int tick;
    int array[3];
    int matrix[2][3];
    FrontCallbackPointer callback;
    union {
        double metric;
        FrontNode *peer;
    };
    unsigned char payload[];
};

union FrontChoice {
    unsigned long word;
    FrontNode *node;
};

struct FrontIncomplete;
extern struct FrontIncomplete *front_incomplete;
extern FrontConstPointer front_const_pointer;
extern FrontCallbackPointer front_callback;
extern FrontArray front_array;
extern FrontIncompleteArray front_incomplete_array;

/* Named members of unnamed record types, as in PostgreSQL's varattrib_4b, are ordinary fields. */
union FrontNamedMembers {
    struct { unsigned int header; unsigned int info; } four_byte;
    struct { unsigned int header; char data[1]; } compressed;
};

struct FrontMixedMembers {
    int tag;
    union { struct { int inner; } named; int other; };
    struct { int a; } *pointer_member;
    struct { int b; } array_member[2];
};

extern union FrontNamedMembers front_named_members;
extern struct FrontMixedMembers front_mixed_members;

struct __attribute__((packed)) FrontPacked {
    unsigned char tag;
    unsigned int value;
};

static inline FrontWide front_static(FrontNode *restrict node, FrontWide value) {
    (void) node;
    return value;
}

int front_external(const FrontNode *, unsigned long);
void front_void(void);
int front_unprototyped();
int front_variadic(const char *, ...);
