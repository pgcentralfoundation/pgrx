//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#include <stddef.h>
#include <stdint.h>

typedef unsigned int Oid;
typedef unsigned int TransactionId;
typedef unsigned int DocWord;
typedef size_t DocSize;

typedef enum DocState { DOC_IDLE, DOC_BUSY } DocState;
typedef int (*DocCallback)(int);

typedef struct DocInner {
    short count;
    DocState state;
} DocInner;

typedef struct DocRecord {
    int value;
    DocInner inner;
    DocCallback callback;
} DocRecord;

typedef struct DocVarlena {
    int length;
    char data[];
} DocVarlena;

typedef struct DocName {
    char data[64];
} DocName;

typedef struct DocHolder {
    DocName name;
    DocRecord *record;
} DocHolder;

typedef struct DocCounter {
    int v;
} DocCounter;

typedef struct DocBits {
    unsigned int low : 4;
    unsigned int high : 12;
} DocBits;

typedef struct DocKeyword {
    int type;
    int plain;
} DocKeyword;

int doc_native(int value);
void doc_void(int value);
void doc_report(const char *expression, const char *file, int line);
extern int doc_global;

#define DOC_CAST(x) ((unsigned short) (x))
#define DOC_TYPEDEF(x) ((DocWord) (x))
#define DOC_SIZE_TYPEDEF(x) ((DocSize) (x))
#define DOC_NULL() ((void *) 0)
#define DOC_ZERO() ((DocRecord *) 0)
#define DOC_VOID(x) ((void) (x))
#define DOC_FIELD(record) ((record)->value)
#define DOC_NESTED(record) ((record)->inner.count)
#define DOC_STATE(record) ((record)->inner.state)
#define DOC_IS_BUSY(record) ((record)->inner.state == DOC_BUSY)
#define DOC_MEMBER(record, field) ((record)->field)
#define DOC_DATA(pointer) (((DocVarlena *) (pointer))->data)
#define DOC_MAX(left, right) ((left) > (right) ? (left) : (right))
#define DOC_AND(left, right) ((left) && (right))
#define DOC_NOT(value) (!(value))
#define DOC_ADD(left, right) ((left) + (right))
#define DOC_ALIGN(alignment, length) \
    (((uintptr_t) (length) + ((alignment) - 1)) & ~((uintptr_t) ((alignment) - 1)))
#define DOC_ALIGN8(length) DOC_ALIGN(8, (length))
#define DOC_CALLBACK(record, value) ((record)->callback(value))
#define DOC_NATIVE(value) doc_native(value)
#define DOC_VOID_CALL(value) doc_void(value)
#define DOC_GLOBAL() (doc_global + 1)
#define DOC_OFFSET(field) offsetof(DocRecord, field)
#define DOC_OFFSET_INNER() offsetof(DocRecord, inner.count)
#define DOC_OFFSET_KEYWORD(field) offsetof(DocKeyword, field)
#define DOC_SIZEOF(value) (sizeof(value))
#define DOC_SIZEOF_TYPE() (sizeof(DocRecord))
#define DOC_SIZEOF_PARAMETER(type) (sizeof(type *))
#define DOC_TYPED(type, pointer) ((type *) (pointer))
#define DOC_SWAP16(value) __builtin_bswap16(value)
#define DOC_LIKELY(value) __builtin_expect((value) != 0, 1)
#define DOC_FILE() __FILE__
#define DOC_LINE() __LINE__
#define DOC_STRING() "documented"
#define DOC_REPORT_AT(expression, file, line) doc_report(#expression, file, line)
#define DOC_REPORT(expression) DOC_REPORT_AT(expression, __FILE__, __LINE__)
#define DOC_ASSIGN(pointer, value) (*(pointer) = (value))
#define DOC_INCREMENT(pointer) (++*(pointer))
#define DOC_COMMA(left, right) ((void) (left), (right))
#define DOC_ADDRESS(record) (&(record)->inner)
#define DOC_INDEX(pointer, index) ((pointer)[index])
#define DOC_EMPTY()
#define DOC_NAME(name) ((name).data)
#define DOC_NAME_OF(holder) ((holder)->name)
#define DOC_SPELL_NAME(name) (DOC_REPORT_AT(name, "", 0), (name).data)
#define DOC_PICK(flag, left, right) ((flag) ? (left).data : (right).data)
#define DOC_SET(place, value) ((place) = (value))
#define DOC_POSTINC(place) ((place)++)
#define DOC_ACCUM(place, counter) ((place).v += (counter).v)
#define DOC_CLEAR(place) ((place).v = 0)
#define DOC_ADDRESS_OF(place) (&(place))
#define DOC_BITS(bits) ((bits)->high)
#define DOC_OID(x) ((Oid) (x))
#define DOC_NOTHING() 0
#define DOC_IGNORE(ignored) ((void) 0)
#define DOC_IGNORE_FIRST(ignored, value) (value)
#define DOC_SWAP(left, right) \
    do { int doc_swap_tmp = *(left); *(left) = *(right); *(right) = doc_swap_tmp; } while (0)
#define DOC_MAYBE_RETURN(condition) \
    do { if (condition) return 0; } while (0)
#define DOC_RETURN(condition) if (condition) return -1; else return 1
