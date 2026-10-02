//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef unsigned int AclMode;
typedef AclMode AliasModeChain;
typedef const AclMode AliasConstMode;
typedef volatile AclMode AliasVolatileMode;
typedef unsigned long AliasLong;
typedef unsigned long long AliasLongLong;
typedef const unsigned long AliasConstWord;
typedef double AliasDouble;
typedef int *AliasPointer;
typedef const int *AliasConstPointer;
typedef AliasConstPointer AliasConstPointerChain;
typedef void *AliasVoidPointer;
typedef enum AliasEnum { AliasZero = 0, AliasOne = 1 } AliasEnum;
typedef AliasEnum AliasEnumChain;
typedef int (*AliasCallback)(int);
typedef struct AliasRecord { unsigned int count; int flag; } AliasRecord;
typedef AliasRecord *AliasRecordPointer;
typedef unsigned int AliasArray[3];
typedef AliasArray *AliasArrayPointer;

extern unsigned int alias_evaluations;
int alias_record(int value);
int alias_callback(int value);
int *alias_take_mut(int *value);
unsigned long *alias_take_ulong(unsigned long *value);
AliasCallback alias_take_callback(AliasCallback value);

#define ALIAS_MODE(privs) ((AclMode) (privs))
#define ALIAS_CHAIN(privs) ((AliasModeChain) (privs))
#define ALIAS_CONST_MODE(privs) ((AliasConstMode) (privs))
#define ALIAS_VOLATILE_MODE(privs) ((AliasVolatileMode) (privs))
#define ALIAS_LONG(value) ((AliasLong) (value))
#define ALIAS_LONG_LONG(value) ((AliasLongLong) (value))
#define ALIAS_RANK(value) (ALIAS_LONG(value) + ALIAS_LONG_LONG(0))
#define ALIAS_FLOAT(value) ((AliasDouble) (value))
#define ALIAS_POINTER(value) ((AliasPointer) (value))
#define ALIAS_CONST_POINTER(value) ((AliasConstPointer) (value))
#define ALIAS_POINTER_CHAIN(value) ((AliasConstPointerChain) (value))
#define ALIAS_RECORD_POINTER(value) ((AliasRecordPointer) (value))
#define ALIAS_RECORD_TAG_POINTER(value) ((struct AliasRecord *) (value))
#define ALIAS_ARRAY_POINTER(value) ((AliasArrayPointer) (value))
#define ALIAS_MODE_POINTER(value) ((AclMode *) (value))
#define ALIAS_CONST_MODE_POINTER(value) ((const AclMode *) (value))
#define ALIAS_VOLATILE_MODE_POINTER(value) ((volatile AclMode *) (value))
#define ALIAS_INTRINSIC_CONST_POINTER(value) ((volatile AliasConstWord *) (value))
#define ALIAS_ENUM(value) ((AliasEnum) (value))
#define ALIAS_ENUM_CHAIN(value) ((AliasEnumChain) (value))
#define ALIAS_CALLBACK(value) ((AliasCallback) (value))
#define ALIAS_NATIVE_CALLBACK() alias_callback
#define ALIAS_CALLBACK_ZERO() ((AliasCallback) 0)
#define ALIAS_ZERO() ((AliasVoidPointer) 0)
#define ALIAS_NULL_SELECT(condition, pointer) ((condition) ? ALIAS_ZERO() : (pointer))
#define ALIAS_NULL_CALLBACK() alias_take_callback(ALIAS_ZERO())
#define ALIAS_MUT_CALL(value) alias_take_mut((value))
#define ALIAS_LONG_MUT_CALL(value) alias_take_ulong((value))
