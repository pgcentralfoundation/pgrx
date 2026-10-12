//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#include <stddef.h>

typedef struct OffsetNested { unsigned char head; unsigned int value; } OffsetNested;
typedef enum OffsetState { OffsetStateZero = 0, OffsetStateOne = 1 } OffsetState;
typedef struct OffsetRecord {
    unsigned char zero;
    unsigned long aligned;
    OffsetNested nested;
    int type;
    volatile unsigned int signal;
    unsigned int values[3];
    int (*callback)();
    OffsetState state;
} OffsetRecord;
typedef OffsetRecord OffsetAlias;
typedef struct __attribute__((packed)) OffsetPacked {
    unsigned char tag;
    unsigned int value;
    OffsetNested nested;
} OffsetPacked;
typedef struct OffsetFlexible {
    unsigned char tag;
    unsigned long length;
    unsigned char payload[];
} OffsetFlexible;
typedef struct OffsetOpaque { unsigned char tag; long double unsupported; } OffsetOpaque;
typedef union OffsetUnion { unsigned int word; OffsetNested nested; } OffsetUnion;
typedef struct OffsetAnonymous {
    unsigned char tag;
    union { unsigned int shared; OffsetNested inner; };
} OffsetAnonymous;
typedef struct OffsetBits { unsigned int bits : 3; unsigned int ordinary; } OffsetBits;
typedef struct OffsetIncomplete OffsetIncomplete;
typedef struct OffsetSimple { unsigned char before; unsigned int value; } OffsetSimple;
typedef const OffsetSimple OffsetConstSimple;
typedef volatile OffsetSimple OffsetVolatileSimple;
typedef struct OffsetQualified {
    OffsetConstSimple frozen;
    OffsetVolatileSimple changed;
} OffsetQualified;

extern unsigned int offset_evaluations;
int offset_record(int value);
int *offset_take_pointer(int *value);

#define OFFSET_ZERO() offsetof(OffsetRecord, zero)
#define OFFSET_TAG() __builtin_offsetof(struct OffsetRecord, aligned)
#define OFFSET_TYPEDEF() offsetof(OffsetAlias, aligned)
#define OFFSET_NESTED() __builtin_offsetof(OffsetRecord, nested.value)
#define OFFSET_RENAMED() offsetof(OffsetRecord, type)
#define OFFSET_PACKED() offsetof(OffsetPacked, value)
#define OFFSET_PACKED_NESTED() offsetof(OffsetPacked, nested.value)
#define OFFSET_VOLATILE() offsetof(OffsetRecord, signal)
#define OFFSET_FLEXIBLE() offsetof(OffsetFlexible, payload)
#define OFFSET_UNSUPPORTED() offsetof(OffsetOpaque, unsupported)
#define OFFSET_CALLBACK() offsetof(OffsetRecord, callback)
#define OFFSET_ENUM() offsetof(OffsetRecord, state)
#define OFFSET_PROMOTED() offsetof(OffsetAnonymous, shared)
#define OFFSET_ANON_NESTED() offsetof(OffsetAnonymous, inner.value)
#define OFFSET_UNION() offsetof(OffsetUnion, nested.value)
#define OFFSET_BITFIELD_SIBLING() offsetof(OffsetBits, ordinary)
#define OFFSET_SIMPLE() offsetof(OffsetSimple, value)
#define OFFSET_CONST_ROOT() offsetof(OffsetConstSimple, value)
#define OFFSET_VOLATILE_ROOT() offsetof(OffsetVolatileSimple, value)
#define OFFSET_CONST_NESTED() offsetof(OffsetQualified, frozen.value)
#define OFFSET_VOLATILE_NESTED() offsetof(OffsetQualified, changed.value)
#define OFFSET_FIELD(member) __builtin_offsetof(OffsetRecord, member)
#define OFFSET_GENERIC(type, member) __builtin_offsetof(type, member)
#define OFFSET_FORWARD(member) OFFSET_FIELD(member)
#define OFFSET_UNUSED(unused) offsetof(OffsetRecord, nested.value)
#define OFFSET_SIZE(unused) (sizeof(offsetof(OffsetRecord, nested.value)) + sizeof(unused))
#define OFFSET_LAZY(condition, value) ((condition) ? OFFSET_ZERO() : (size_t) (value))
#define OFFSET_NULL_SELECT(condition, pointer) ((condition) ? OFFSET_ZERO() : (pointer))
#define OFFSET_NULL_CALL() offset_take_pointer(OFFSET_ZERO())
#define OFFSET_RUNTIME_NULL(condition) offset_take_pointer((condition) ? OFFSET_ZERO() : OFFSET_NESTED())

#define OFFSET_BITFIELD() offsetof(OffsetBits, bits)
#define OFFSET_INCOMPLETE() offsetof(OffsetIncomplete, absent)
#define OFFSET_NONRECORD() offsetof(int, absent)
#define OFFSET_MISSING() offsetof(OffsetRecord, absent)
#define OFFSET_INDEX_CONSTANT() offsetof(OffsetRecord, values[1])
#define OFFSET_INDEX_DYNAMIC(index) offsetof(OffsetRecord, values[index])
#define OFFSET_CALL_PATH() offsetof(OffsetRecord, nested.value())
#define OFFSET_ARROW_PATH() offsetof(OffsetRecord, nested->value)
#define OFFSET_PATH_BUDGET() offsetof(OffsetRecord, nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.nested.value)
