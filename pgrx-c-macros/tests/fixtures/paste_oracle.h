//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef unsigned long PasteWord;
typedef enum PasteTag { T_Alpha=7, T_Beta=19, BadEnum=23 } PasteTag;
typedef struct PasteRecord { unsigned int field; int type; } PasteRecord;
typedef PasteRecord *PastePointer;
extern unsigned int paste_int_calls;
extern unsigned int paste_long_calls;
int paste_int(int value);
long paste_record(long value);
int *paste_take_pointer(int *pointer);

#define PASTE_CAT(left, right) left ## right
#define PASTE_CAT_INDIRECT(left, right) PASTE_CAT(left, right)
#define PASTE_CAT3(left, middle, right) left ## middle ## right
#define PASTE_ADD(value) ((value) + 7)
#define PASTE_SUFFIX ULL
#define PASTE_CONSTANT 0xA5U
#define PASTE_VALUE (PASTE_CAT(Bad, Enum))

#define PASTE_UL() PASTE_CAT(0xFFFFFFFFFFFFFFFF, UL)
#define PASTE_ULL() PASTE_CAT(0xFFFFFFFFFFFFFFFF, ULL)
#define PASTE_L() PASTE_CAT(0x1234, L)
#define PASTE_LL() PASTE_CAT(0x1234, LL)
#ifdef __OPTIMIZE__
#define PASTE_PROFILE() PASTE_CAT(0x1234, LL)
#else
#define PASTE_PROFILE() PASTE_CAT(0x1234, L)
#endif
#define PASTE_NESTED() PASTE_CAT_INDIRECT(0x0123456789ABCDEF, PASTE_SUFFIX)
#define PASTE_ENUM(value) ((value) == PASTE_CAT(T_, Alpha))
#define PASTE_OBJECT(value) ((value) + PASTE_CAT(PASTE_, CONSTANT))
#define PASTE_TYPE(value) ((PASTE_CAT(Paste, Word)) (value))
#define PASTE_POINTER(value) ((PASTE_CAT(Paste, Pointer)) (value))
#define PASTE_TAG_POINTER(value) ((struct PASTE_CAT(Paste, Record) *) (value))
#define PASTE_MEMBER(pointer) ((pointer)->PASTE_CAT(fi, eld))
#define PASTE_RENAMED(pointer) ((pointer)->PASTE_CAT(ty, pe))
#define PASTE_SET_MEMBER(pointer, value) ((pointer)->PASTE_CAT(fi, eld) = (value))
#define PASTE_CALL(value) PASTE_CAT(paste_, record)(value)
#define PASTE_HELPER(value) PASTE_CAT(PASTE_, ADD)(value)
#define PASTE_PLACE_LEFT(value) (PASTE_CAT(, value))
#define PASTE_PLACE_RIGHT(value) (PASTE_CAT(value, ))
#define PASTE_REPEAT(value) (PASTE_HELPER(value) + PASTE_HELPER(value))
#define PASTE_UNUSED(unused) PASTE_UL()
#define PASTE_SIZE(value) (sizeof(PASTE_CALL(value)))
#define PASTE_LAZY(condition, value) ((condition) ? PASTE_CALL(value) : -7L)
#define PASTE_ZERO() PASTE_CAT(0, U)
#define PASTE_NULL_SELECT(condition, pointer) ((condition) ? PASTE_ZERO() : (pointer))
#define PASTE_NULL_CALL() paste_take_pointer(PASTE_ZERO())
#define PASTE_RUNTIME_NULL(value) paste_take_pointer(PASTE_PLACE_LEFT(value))
#define PASTE_VALUE_BAD(value) ((value) + PASTE_CAT(Bad, Enum))
#define PASTE_VALUE_ADD(value) ((value) + PASTE_VALUE)
#define PASTE_VALUE_TOP(value) PASTE_VALUE_ADD(value)

#define PASTE_DROP(value) 0
#define PASTE_FORWARD(value) PASTE_DROP(value)
#define PASTE_STRING(value) #value
#define PASTE_FORMAL_PREFIX(value) PASTE_CAT(T_, value)
#define PASTE_FORMAL_SUFFIX(value) PASTE_CAT(value, _suffix)
#define PASTE_FORMAL_MIDDLE(value) PASTE_CAT3(pre_, value, _suffix)
#define PASTE_FORMAL_DIGRAPH(value) value %:%: _suffix
#define PASTE_ERASED(value) PASTE_FORWARD(PASTE_CAT(value, _suffix))
#define PASTE_SYNTH_STRING(value) PASTE_CAT(PASTE_, STRING)(value)
