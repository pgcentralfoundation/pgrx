//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#include <stdio.h>
unsigned int record_oracle_packed_load(struct PackedArrayRecord *p, int n)
{
    return RECORD_PACKED_ARRAY(p, n);
}
void record_oracle_packed_store(struct PackedArrayRecord *p, int n, unsigned int value)
{
    RECORD_SET_PACKED_ARRAY(p, n, value);
}
unsigned int record_oracle_packed_flex_load(struct PackedFlexibleRecord *p, int n)
{
    return RECORD_PACKED_FLEX(p, n);
}
int main(void)
{
    CastOnlyRecord cast_only;
    cast_only.first=61;
    CastOnlyRecord copy=RECORD_CAST_ONLY((void *)&cast_only);
    printf("castonly\t%u\n",copy.first);
    printf("volatilecastfield\t%u\n",RECORD_VOLATILE_CAST_FIELD((void *)&cast_only));
    printf("volatilewholesize\t%zu\n",RECORD_VOLATILE_WHOLE_SIZE((void *)&cast_only));
    printf("volatileassignmentsize\t%zu\n",RECORD_VOLATILE_ASSIGNMENT_SIZE((void *)0,cast_only));
    printf("volatilewholeaddress\t%d\n",RECORD_VOLATILE_WHOLE_ADDRESS((void *)&cast_only)==&cast_only);
    union { long double align; unsigned char bytes[64]; } allocation;
    AnonymousRecord *anonymous = (AnonymousRecord *)&allocation;
    anonymous->expanded.header = 17;
    anonymous->expanded.bytes[1] = 65;
    printf("header\t%u\n", RECORD_HEADER(anonymous));
    printf("byte\t%d\n", RECORD_BYTE(anonymous, 1));
    printf("setbyte\t%d\n", RECORD_SET_BYTE(anonymous, 1, 258));
    printf("address\t%d\n", RECORD_HEADER_ADDRESS(anonymous) == &anonymous->expanded.header);
    printf("size\t%zu\n", RECORD_CHILD_SIZE(anonymous));
    printf("anonymous\t%d\n", RECORD_ANON_INLINE(anonymous) == anonymous);
    PromotedRecord promoted;
    promoted.promoted = 31;
    printf("promoted\t%u\n", RECORD_PROMOTED(&promoted));
    printf("setpromoted\t%u\n", RECORD_SET_PROMOTED(&promoted, 65539));
    FlexibleRecord *flexible = (FlexibleRecord *)&allocation;
    flexible->items[2] = 53;
    printf("flex\t%u\n", RECORD_FLEX(flexible, 2));
    printf("setflex\t%u\n", RECORD_SET_FLEX(flexible, 2, 99));
    printf("flexaddress\t%d\n", RECORD_FLEX_ADDRESS(flexible) == flexible->items);
    unsigned int (*array_pointer)[3] = (unsigned int (*)[3])flexible->items;
    printf("flexarrayaddress\t%d\n", RECORD_FLEX_ARRAY_ADDRESS(flexible) == &flexible->items);
    printf("flexarrayequal\t%d\n", RECORD_FLEX_ARRAY_EQUAL(flexible, array_pointer));
    printf("flexarrayvoid\t%d\n", RECORD_FLEX_ARRAY_VOID(flexible) == flexible->items);
    unsigned int (*incomplete_array)[] = array_pointer;
    unsigned int (*complete_array)[3] = incomplete_array;
    printf("flexarrayconvert\t%d\n", complete_array == array_pointer);
    OpaqueRecord *opaque = (OpaqueRecord *)&allocation;
    printf("opaque\t%d\n", RECORD_OPAQUE(opaque) == opaque);
    printf("opaqueinline\t%d\n", RECORD_OPAQUE_INLINE(opaque) == opaque);
    printf("compare\t%d\n", RECORD_OPAQUE_COMPARE(opaque, opaque));
    printf("void\t%d\n", RECORD_OPAQUE_VOID(opaque) == &allocation);
    unsigned int array[3] = { 7, 11, 19 };
    printf("array\t%u\n", RECORD_ARRAY(&array, 2));
    VolatilePromotedRecord signal;
    signal.signal = 29;
    printf("volatile\t%u\n", RECORD_VOLATILE_READ(&signal));
    printf("volatileindirect\t%u\n", RECORD_VOLATILE_INDIRECT(&signal));
    printf("volatileaddress\t%d\n", RECORD_VOLATILE_ADDRESS(&signal) == &signal.signal);
    struct PackedArrayRecord *packed = (struct PackedArrayRecord *)&allocation;
    packed->array[1] = 43;
    printf("packedarray\t%u\n", RECORD_PACKED_ARRAY(packed, 1));
    printf("setpackedarray\t%u\n", RECORD_SET_PACKED_ARRAY(packed, 1, 101));
    struct PackedFlexibleRecord *packed_flex = (struct PackedFlexibleRecord *)&allocation;
    packed_flex->array[2] = 47;
    printf("packedflex\t%u\n", RECORD_PACKED_FLEX(packed_flex, 2));
    return 0;
}
