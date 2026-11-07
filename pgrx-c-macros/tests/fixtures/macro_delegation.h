//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef __SIZE_TYPE__ DelegateSize;

#define DELEGATE_ALIGNMENT 32
#define DELEGATE_TYPEALIGN(ALIGNVAL, LEN) \
    (((DelegateSize) (LEN) + ((ALIGNVAL) - 1)) & ~((DelegateSize) ((ALIGNVAL) - 1)))
#define DELEGATE_BUFFERALIGN(LEN) DELEGATE_TYPEALIGN(DELEGATE_ALIGNMENT, (LEN))
#define DELEGATE_GROUPED_ALIGN(LEN) ((DELEGATE_TYPEALIGN(DELEGATE_ALIGNMENT, (LEN))))
#define DELEGATE_ALIGN_ALIAS(LEN) DELEGATE_BUFFERALIGN((LEN))

#define DELEGATE_ADD(left, right) ((left) + (right))
#define DELEGATE_SWAP(left, right) DELEGATE_ADD((right), (left))
#define DELEGATE_REPEAT(value) ((value) + (value))
#define DELEGATE_TWICE(value) DELEGATE_REPEAT((value))
#define DELEGATE_NESTED(value) DELEGATE_REPEAT(DELEGATE_ADD((value), 1))
#define DELEGATE_CHOOSE(condition, yes, no) ((condition) ? (yes) : (no))
#define DELEGATE_SELECT(condition, yes, no) DELEGATE_CHOOSE((condition), (yes), (no))
#define DELEGATE_ONE_ARGUMENT(value) DELEGATE_CHOOSE((value), (value), 17U)
#define DELEGATE_VALUE() (0xFFFFFFFFU)
#define DELEGATE_ZERO_ARGUMENT() DELEGATE_VALUE()
#define match(value) ((value))
#define DELEGATE_KEYWORD(value) match((value))

#define DELEGATE_UNUSED(value) (7)
#define DELEGATE_UNUSED_WRAPPER(value) DELEGATE_UNUSED((value))
#define DELEGATE_UNGROUPED(value) (value * 2)
#define DELEGATE_GROUPING_FIXED(value) DELEGATE_UNGROUPED((value))
