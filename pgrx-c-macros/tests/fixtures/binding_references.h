//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef __SIZE_TYPE__ RefSize;
typedef unsigned int Oid;
typedef unsigned int TransactionId;
enum RefEnum { REF_ENUM = 7 };
#define REF_HUGE (((RefSize) -1) / 2)
#define REF_HUGE_VALID(size) ((RefSize) (size) <= REF_HUGE)
#define REF_HUGE_WRAPPER(size) REF_HUGE_VALID(size)
#define REF_HUGE_OUTER(size) (REF_HUGE_WRAPPER(size) + 1)
#define REF_FIRST 17U
#define REF_ALIAS REF_FIRST
#define REF_ALIASES(value) ((value) + REF_ALIAS + REF_FIRST)
#define REF_UNGROUPED 1 + 2
#define REF_PRECEDENCE(value) ((value) * REF_UNGROUPED)
#define REF_ENUM_ADD(value) ((value) + REF_ENUM)
#define REF_ENUM_HIDDEN (REF_ENUM_ADD(0U))
#define REF_ENUM_HIDDEN_ADD(value) ((value) + REF_ENUM_HIDDEN)
#define REF_OID 42U
#define REF_OID_EQUAL(value) ((value) == REF_OID)
#define REF_BOOL ((_Bool) 1)
#define REF_BOOL_VALUE() REF_BOOL
#define REF_MIN (-9223372036854775807LL - 1LL)
#define REF_MIN_VALUE() REF_MIN
#define REF_HEX 0xFFFFFFFF
#define REF_HEX_VALUE() REF_HEX
#define REF_SIGNED_OVERFLOW (2147483647 + 1)
#define REF_SIGNED_OVERFLOW_VALUE() REF_SIGNED_OVERFLOW
#define REF_DIV_OVERFLOW ((-2147483647 - 1) / -1)
#define REF_DIV_OVERFLOW_VALUE() REF_DIV_OVERFLOW
#define REF_BAD_SHIFT (1U << 32)
#define REF_BAD_SHIFT_VALUE() REF_BAD_SHIFT
