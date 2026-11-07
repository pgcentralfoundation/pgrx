//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.


typedef struct OpaqueRecord OpaqueRecord;
OpaqueRecord *opaque_identity(OpaqueRecord *pointer);
typedef struct CastOnlyRecord { unsigned int first; _Bool unread; } CastOnlyRecord;

typedef union
{
    struct { unsigned int header; char bytes[]; } expanded;
    struct { unsigned int header; unsigned int info; char bytes[]; } compressed;
} AnonymousRecord;

typedef struct
{
    unsigned int prefix;
    union { struct { unsigned int promoted; _Bool unread; }; unsigned int other; };
} PromotedRecord;

typedef struct
{
    union { unsigned int signal; unsigned int other; };
} VolatilePromotedRecord;

typedef struct
{
    unsigned int count;
    _Bool unread;
    unsigned int items[];
} FlexibleRecord;

struct __attribute__((packed)) PackedArrayRecord
{
    char lead;
    unsigned int array[3];
};
struct __attribute__((packed)) PackedFlexibleRecord
{
    char lead;
    unsigned int array[];
};

static inline unsigned int array_element(const unsigned int (*array)[3], int index)
{
    return (*array)[index];
}
static inline AnonymousRecord *anonymous_identity(AnonymousRecord *pointer) { return pointer; }
static inline OpaqueRecord *opaque_inline_identity(OpaqueRecord *pointer) { return pointer; }

#define RECORD_HEADER(p) ((p)->expanded.header)
#define RECORD_BYTE(p, n) ((p)->expanded.bytes[n])
#define RECORD_SET_BYTE(p, n, value) ((p)->expanded.bytes[n] = (value))
#define RECORD_HEADER_ADDRESS(p) (&(p)->expanded.header)
#define RECORD_CHILD_SIZE(p) (sizeof((p)->expanded))
#define RECORD_PROMOTED(p) ((p)->promoted)
#define RECORD_SET_PROMOTED(p, value) ((p)->promoted = (value))
#define RECORD_FLEX(p, n) ((p)->items[n])
#define RECORD_FLEX_ADDRESS(p) ((p)->items)
#define RECORD_SET_FLEX(p, n, value) ((p)->items[n] = (value))
#define RECORD_OPAQUE(p) (opaque_identity(p))
#define RECORD_OPAQUE_INLINE(p) (opaque_inline_identity(p))
#define RECORD_OPAQUE_COMPARE(p, q) ((p) == (q))
#define RECORD_OPAQUE_VOID(p) ((void *)(p))
#define RECORD_ANON_INLINE(p) (anonymous_identity(p))
#define RECORD_ARRAY(p, n) (array_element(p, n))
#define RECORD_VOLATILE_READ(p) (((volatile VolatilePromotedRecord *)(p))->signal)
#define RECORD_VOLATILE_ADDRESS(p) (&((volatile VolatilePromotedRecord *)(p))->signal)
#define RECORD_VOLATILE_INDIRECT(p) (*(&((volatile VolatilePromotedRecord *)(p))->signal))
#define RECORD_PACKED_ARRAY(p, n) ((p)->array[n])
#define RECORD_SET_PACKED_ARRAY(p, n, value) ((p)->array[n] = (value))
#define RECORD_PACKED_FLEX(p, n) ((p)->array[n])
#define RECORD_FLEX_ARRAY_ADDRESS(p) (&((p)->items))
#define RECORD_FLEX_ARRAY_EQUAL(p, array) (&((p)->items) == (array))
#define RECORD_FLEX_ARRAY_VOID(p) ((void *)&((p)->items))
#define RECORD_FLEX_ARRAY_STEP(p, n) (&((p)->items) + (n))
#define RECORD_OPAQUE_SIZE(p) (sizeof(*(p)))
#define RECORD_OPAQUE_OFFSET(p, n) ((p) + (n))
#define RECORD_OPAQUE_READ(p) (*(p))
#define RECORD_FLEX_SIZE(p) (sizeof((p)->items))
#define RECORD_CAST_ONLY(p) (*(CastOnlyRecord *)(p))
#define RECORD_VOLATILE_WHOLE(p) (*(volatile CastOnlyRecord *)(p))
#define RECORD_VOLATILE_WHOLE_SET(p, value) (*(volatile CastOnlyRecord *)(p) = (value))
#define RECORD_VOLATILE_WHOLE_ROUNDTRIP(p) (*(&(*(volatile CastOnlyRecord *)(p))))
#define RECORD_VOLATILE_WHOLE_SELECT(p) (*(1 ? (volatile CastOnlyRecord *)(p) : (volatile CastOnlyRecord *)(p)))
#define RECORD_VOLATILE_WHOLE_SELECT_MIXED(p) (*(1 ? (volatile CastOnlyRecord *)(p) : (CastOnlyRecord *)(p)))
#define RECORD_VOLATILE_WHOLE_ADDRESS(p) (&(*(volatile CastOnlyRecord *)(p)))
#define RECORD_VOLATILE_WHOLE_SIZE(p) (sizeof(*(volatile CastOnlyRecord *)(p)))
#define RECORD_VOLATILE_CAST_FIELD(p) (((volatile CastOnlyRecord *)(p))->first)
#define RECORD_VOLATILE_ASSIGNMENT_SIZE(p, value) (sizeof(*(volatile CastOnlyRecord *)(p) = (value)))
