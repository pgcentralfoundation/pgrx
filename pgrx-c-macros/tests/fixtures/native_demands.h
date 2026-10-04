//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#ifndef PGRX_TEST_NATIVE_DEMANDS_H
#define PGRX_TEST_NATIVE_DEMANDS_H

typedef struct DemandLeaf {
    unsigned int leaf;
    unsigned int sibling;
} DemandLeaf;
typedef union DemandChoice {
    DemandLeaf child;
    unsigned long long word;
} DemandChoice;
typedef struct DemandUnrelated {
    unsigned int selected;
    unsigned int promoted;
    unsigned int unrelated_only;
} DemandUnrelated;
typedef struct DemandOwner {
    unsigned char tag;
    unsigned int selected;
    DemandChoice choice;
    union {
        unsigned int promoted;
        DemandLeaf hidden;
    };
    const DemandLeaf frozen;
    volatile DemandLeaf observed;
    DemandUnrelated *outside;
} DemandOwner;
typedef DemandOwner DemandAlias;
typedef const DemandOwner DemandConstOwner;
typedef struct DemandRejected {
    long double unsupported;
    unsigned int selected;
} DemandRejected;
typedef struct DemandSigned { int selected; } DemandSigned;
typedef struct DemandFloat { double selected; } DemandFloat;
typedef struct DemandRecord { DemandLeaf selected; } DemandRecord;
typedef struct DemandArray { unsigned int selected[2]; } DemandArray;
typedef struct DemandPointer { unsigned int *selected; } DemandPointer;
typedef enum DemandIndex { DemandZero = 0, DemandOne = 1, DemandTwo = 2 } DemandIndex;
typedef int (*DemandCallback)(int);
typedef int (*DemandBoolCallback)(_Bool);
typedef double (*DemandFloatCallback)(double);
typedef int (*DemandDoubleIntCallback)(double);
typedef unsigned int (*DemandUnsignedCallback)(unsigned int);
typedef DemandLeaf (*DemandRecordCallback)(DemandLeaf);
typedef int (*DemandLeafCallback)(DemandLeaf *);
typedef int (*DemandConstLeafCallback)(const DemandLeaf *);
typedef int (*DemandOtherCallback)(DemandUnrelated *);
typedef int (*DemandVoidCallback)(void *);
typedef int (*DemandConstVoidCallback)(const void *);
typedef int (*DemandCallbackArgument)(DemandCallback);
typedef DemandLeaf *(*DemandLeafFactory)(int);
typedef void (*DemandVoidResultCallback)(int);
int demand_integer_function(int value);
extern DemandCallback demand_known_callback;
int demand_bool_function(_Bool value);
int demand_pointer_function(void *value);
DemandLeaf demand_record_function(DemandLeaf value);
int demand_leaf_pointer_function(const DemandLeaf *value);
int demand_mutable_leaf_function(DemandLeaf *value);
int demand_nested_pointer_function(DemandLeaf **value);
int demand_unsigned_pointer_function(unsigned int *value);
int demand_callback_function(DemandCallback value);
typedef struct DemandLeafPointer { DemandLeaf *selected; } DemandLeafPointer;
typedef struct DemandConstLeafPointer { const DemandLeaf *selected; } DemandConstLeafPointer;
typedef struct DemandVolatileLeafPointer { volatile DemandLeaf *selected; } DemandVolatileLeafPointer;
typedef struct DemandOtherPointer { DemandUnrelated *selected; } DemandOtherPointer;
typedef struct DemandVoidPointer { void *selected; } DemandVoidPointer;
typedef struct DemandCallbackPointer { DemandCallback selected; } DemandCallbackPointer;
typedef struct DemandLeafArray { DemandLeaf selected[2]; } DemandLeafArray;
typedef struct DemandNestedPointer { DemandLeaf **selected; } DemandNestedPointer;
typedef struct DemandNestedConstPointer { const DemandLeaf **selected; } DemandNestedConstPointer;
typedef struct DemandIndexPointer { DemandIndex *selected; } DemandIndexPointer;
typedef struct DemandRecordLeaf { DemandLeaf selected; } DemandRecordLeaf;
typedef struct DemandRecordOther { DemandUnrelated selected; } DemandRecordOther;
typedef struct DemandSelector {
    unsigned int first;
    int second;
    DemandIndex enumeration;
    double floating;
    DemandLeaf record;
    unsigned int *pointer;
    unsigned int array[2];
} DemandSelector;
typedef struct DemandListA { int length; int elements[4]; } DemandListA;
typedef struct DemandListB { unsigned int length; unsigned int elements[3]; } DemandListB;
typedef struct DemandNotList { int length; } DemandNotList;
typedef struct DemandStateA {
    DemandListA *first;
    DemandListA *second;
    int index;
    unsigned int *not_list;
    DemandNotList *missing_elements;
    DemandListA record_list;
    double wrong_index;
} DemandStateA;
typedef struct DemandStateB {
    DemandListB *list;
    DemandListB *empty;
    DemandListB array_list[1];
    unsigned int position;
    int *not_list;
    DemandNotList *missing_elements;
    DemandListB record_list;
    double wrong_index;
} DemandStateB;

typedef struct DemandUpstreamLeafA {
    int payload;
    DemandCallback finish;
} DemandUpstreamLeafA;
typedef struct DemandUpstreamLeafB {
    unsigned int payload;
    unsigned int (*finish)(unsigned int);
} DemandUpstreamLeafB;
typedef struct DemandUpstreamUnreachable {
    unsigned int payload;
    int (*finish)(double);
} DemandUpstreamUnreachable;
typedef struct DemandUpstreamA { DemandUpstreamLeafA *upstream; } DemandUpstreamA;
typedef struct DemandUpstreamB { const DemandUpstreamLeafB *upstream; } DemandUpstreamB;
typedef struct DemandUpstreamArray { DemandUpstreamLeafA upstream[2]; } DemandUpstreamArray;
typedef struct DemandUpstreamSlotA { DemandUpstreamLeafA **slot; } DemandUpstreamSlotA;
typedef struct DemandUpstreamSlotB { const DemandUpstreamLeafB **slot; } DemandUpstreamSlotB;
typedef struct DemandUpstreamFetchA { DemandUpstreamLeafA *(*fetch)(int); } DemandUpstreamFetchA;
typedef struct DemandUpstreamFetchB { const DemandUpstreamLeafB *(*fetch)(unsigned int); } DemandUpstreamFetchB;
unsigned int demand_upstream_unsigned(unsigned int value);
DemandUpstreamLeafA *demand_upstream_fetch_a(int value);
const DemandUpstreamLeafB *demand_upstream_fetch_b(unsigned int value);

#define DEMAND_OWNER_READ(pointer) (((DemandOwner *) (pointer))->selected)
#define DEMAND_OWNER_READ_PROMOTED(pointer) (((DemandOwner *) (pointer))->promoted)
#define DEMAND_OWNER_SET_PROMOTED(pointer, value) (((DemandOwner *) (pointer))->promoted = (value))
#define DEMAND_OWNER_READ_NESTED(pointer) (((DemandOwner *) (pointer))->choice.child.leaf)
#define DEMAND_OWNER_SET_NESTED(pointer, value) (((DemandOwner *) (pointer))->choice.child.leaf = (value))
#define DEMAND_UNKNOWN_READ(pointer) ((pointer)->selected)
#define DEMAND_INTEGER_FIELD(pointer) (((pointer)->selected - 4U) >> 2)
#define DEMAND_FIXED_INTEGER_FIELD(pointer, member) \
    ((((DemandSelector *)(pointer))->member - 4U) >> 2)
#define DEMAND_UPSTREAM_READ(root) ((root)->upstream->payload)
#define DEMAND_UPSTREAM_MEMBER(root) ((*(root)->upstream).payload)
#define DEMAND_UPSTREAM_SET(root, value) ((root)->upstream->payload = (value))
#define DEMAND_UPSTREAM_DEREF(root) ((*(root)->slot)->payload)
#define DEMAND_UPSTREAM_INDEX(root, index) ((root)->upstream[(index)].payload)
#define DEMAND_UPSTREAM_REVERSE(root, index) ((index)[(root)->upstream].payload)
#define DEMAND_UPSTREAM_CALL(root, value) ((root)->upstream->finish(value))
#define DEMAND_UPSTREAM_RESULT(root, value) ((root)->fetch(value)->payload)
#define DEMAND_INDEX_RECORD(left, right) ((left)[(right)].selected >> 1)
#define DEMAND_INDEX_INTEGER(left, right) (((left)[(right)] - 4U) >> 2)
#define DEMAND_KNOWN_REVERSE_INDEX(index, pointer) \
    ((index)[(const DemandLeaf *)(pointer)].leaf)
#define DEMAND_DIRECT_INTEGER(value) demand_integer_function(value)
#define DEMAND_FUNCTION_VALUE() (demand_integer_function)
#define DEMAND_KNOWN_CALLBACK(value) demand_known_callback(value)
#define DEMAND_OPEN_CALLBACK(callback, value) ((callback)(value))
#define DEMAND_OPEN_INTEGER(callback) ((callback)(47))
#define DEMAND_OPEN_NULL(callback) ((callback)(0))
#define DEMAND_OPEN_LEAF(callback, value) ((callback)((DemandLeaf *)(value)))
#define DEMAND_OPEN_CONST_LEAF(callback, value) ((callback)((const DemandLeaf *)(value)))
#define DEMAND_OPEN_VOID(callback, value) ((callback)((void *)(value)))
#define DEMAND_OPEN_RECORD(callback, value) ((callback)(*(DemandLeaf *)(value)))
#define DEMAND_OPEN_STORED_INTEGER(callback, value) \
    ((callback)(((DemandSelector *)(value))->second))
#define DEMAND_OPEN_INTEGER_RESULT(callback, value) (((callback)(value)) >> 1)
#define DEMAND_OPEN_TWO_SITES(first, second, value) \
    ((first)(47), (second)((DemandLeaf *)(value)))
#define DEMAND_OPEN_UNCONSTRAINED_SITE(callback, value) ((callback)(47), (callback)(value))

/* Double grouping establishes a call independently of caller argument types. */
#define DEMAND_OPEN_CALLBACK_CALL(callback, value) (((callback))(value))
#define DEMAND_OPEN_INTEGER_CALL(callback) (((callback))(47))
#define DEMAND_OPEN_NULL_CALL(callback) (((callback))(0))
#define DEMAND_OPEN_LEAF_CALL(callback, value) (((callback))((DemandLeaf *)(value)))
#define DEMAND_OPEN_CONST_LEAF_CALL(callback, value) (((callback))((const DemandLeaf *)(value)))
#define DEMAND_OPEN_VOID_CALL(callback, value) (((callback))((void *)(value)))
#define DEMAND_OPEN_RECORD_CALL(callback, value) (((callback))(*(DemandLeaf *)(value)))
#define DEMAND_OPEN_STORED_INTEGER_CALL(callback, value) \
    (((callback))(((DemandSelector *)(value))->second))
#define DEMAND_OPEN_INTEGER_RESULT_CALL(callback, value) ((((callback))(value)) >> 1)
#define DEMAND_OPEN_TWO_SITES_CALL(first, second, value) \
    (((first))(47), ((second))((DemandLeaf *)(value)))
#define DEMAND_OPEN_UNCONSTRAINED_SITE_CALL(callback, value) (((callback))(47), ((callback))(value))
#define DEMAND_CALLBACK_IDENTITY(value) (value)
#define DEMAND_CALLBACK_SIZE(value) sizeof((value))
#define DEMAND_CALLBACK_DISCARD(value) ((void)(value))
#define DEMAND_DIRECT_BOOL(value) demand_bool_function(value)
#define DEMAND_DIRECT_POINTER(value) demand_pointer_function(value)
#define DEMAND_CAST_POINTER(value) demand_pointer_function((void *)(value))
#define DEMAND_DIRECT_RECORD(value) demand_record_function(value)
#define DEMAND_LEAF_POINTER(pointer) demand_leaf_pointer_function((pointer)->selected)
#define DEMAND_MUTABLE_LEAF(pointer) demand_mutable_leaf_function((pointer)->selected)
#define DEMAND_NESTED_POINTER(pointer) demand_nested_pointer_function((pointer)->selected)
#define DEMAND_UNSIGNED_POINTER(pointer) demand_unsigned_pointer_function((pointer)->selected)
#define DEMAND_CALLBACK_POINTER(pointer) demand_callback_function((pointer)->selected)
#define DEMAND_RECORD_FIELD(pointer) demand_record_function((pointer)->selected)
#define DEMAND_ASSIGN_LEAF(pointer, destination) \
    (*(DemandLeaf *)(destination) = (pointer)->selected)
#define DEMAND_DYNAMIC_OFFSET(pointer, member) \
    ((pointer)->member == 0 ? (void *)0 : (void *)((char *)(pointer) + (pointer)->member))
#define DEMAND_NEXT(cell, state, list, index) \
    ((cell) = ((state).list != (void *)0 && (state).index < (state).list->length) \
        ? &(state).list->elements[(state).index] : (void *)0)
#define DEMAND_OWNER_OFFSET(member) __builtin_offsetof(DemandOwner, member)
#define DEMAND_NAMED_OFFSET() __builtin_offsetof(DemandAlias, choice.child.leaf)
#define DEMAND_GENERIC_OFFSET(type, member) __builtin_offsetof(type, member)
#define DEMAND_REJECTED(pointer) (((DemandRejected *) (pointer))->unsupported)
#define DEMAND_LITERAL() 7

#endif
