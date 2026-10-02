//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef enum EnumSmall { SmallZero = 0, SmallOne = 1 } EnumSmall;
typedef enum EnumOther { OtherZero = 0, OtherOne = 1 } EnumOther;
typedef enum EnumSigned { SignedNegative = -1, SignedPositive = 2 } EnumSigned;
typedef enum EnumWide { WideZero = 0, WideMaximum = 0xFFFFFFFFFFFFFFFFul } EnumWide;
typedef struct EnumRecord {
    EnumSmall small;
    EnumSigned signed_value;
    EnumWide wide;
    _Bool unread;
} EnumRecord;
typedef struct __attribute__((packed)) PackedEnumRecord {
    unsigned char prefix;
    EnumSmall small;
} PackedEnumRecord;
typedef struct EnumKeywordRecord {
    EnumSmall type;
    unsigned int u32;
    _Bool unread;
} EnumKeywordRecord;

#define ENUM_READ(p) ((p)->small)
#define ENUM_SET(p, v) ((p)->small = (v))
#define ENUM_ADD(p, v) ((p)->small += (v))
#define ENUM_POST(p) ((p)->small++)
#define ENUM_PREFIX(p) (++(p)->small)
#define ENUM_CAST(v) ((EnumSmall)(v))
#define ENUM_NOT(p) (~(p)->small)
#define ENUM_SIZE(p) (sizeof((p)->small))
#define ENUM_ADDRESS(p) (&(p)->small)
#define ENUM_INDIRECT(p) (*(&(p)->small))
#define ENUM_VOLATILE(p) (((volatile EnumRecord *)(p))->small)
#define ENUM_VOLATILE_SET(p, v) (((volatile EnumRecord *)(p))->small = (v))
#define ENUM_SIGNED(p) ((p)->signed_value)
#define ENUM_SIGNED_SET(p, v) ((p)->signed_value = (v))
#define ENUM_WIDE(p) ((p)->wide)
#define ENUM_WIDE_SET(p, v) ((p)->wide = (v))
#define ENUM_PACKED(p) ((p)->small)
#define ENUM_PACKED_SET(p, v) ((p)->small = (v))
#define ENUM_IDENTITY(v) (v)
#define ENUM_COMPARE(p, integer_pointer) (&(p)->small == (integer_pointer))
#define ENUM_DISTINCT(p, other_pointer) (&(p)->small == (other_pointer))
#define ENUM_KEYWORD(p) ((p)->type)
#define ENUM_KEYWORD_SET(p, v) ((p)->type = (v))
#define ENUM_PRIMITIVE_NAME(p) ((p)->u32)
#define ENUM_CONSTANT(v) ((v) + SmallOne)
#define ENUM_CONSTANT_USE(v) ((EnumSigned)((v) + SignedNegative))
#define ENUM_CONSTANT_ZERO(p) ((p) == SmallZero)

static inline EnumSmall enum_inline_identity(EnumSmall value) { return value; }
#define ENUM_INLINE(v) enum_inline_identity((EnumSmall)(v))
