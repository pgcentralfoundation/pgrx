//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#include <stdio.h>
#include <limits.h>

int main(void) {
    EnumRecord record;
    record.small = (EnumSmall)7;
    record.signed_value = (EnumSigned)-17;
    record.wide = (EnumWide)0x8000000000000001ul;
    printf("read\t%llu\n", (unsigned long long)ENUM_READ(&record));
    printf("set\t%llu\n", (unsigned long long)ENUM_SET(&record, 129));
    printf("add\t%llu\n", (unsigned long long)ENUM_ADD(&record, 2));
    printf("post\t%llu\n", (unsigned long long)ENUM_POST(&record));
    printf("prefix\t%llu\n", (unsigned long long)ENUM_PREFIX(&record));
    printf("cast\t%llu\n", (unsigned long long)ENUM_CAST(253));
    printf("not\t%llu\n", (unsigned long long)ENUM_NOT(&record));
    printf("size\t%llu\n", (unsigned long long)ENUM_SIZE(&record));
    printf("address\t%d\n", ENUM_ADDRESS(&record) == &record.small);
    printf("indirect\t%llu\n", (unsigned long long)ENUM_INDIRECT(&record));
    printf("volatile\t%llu\n", (unsigned long long)ENUM_VOLATILE(&record));
    printf("volatileset\t%llu\n", (unsigned long long)ENUM_VOLATILE_SET(&record, 251));
    printf("signed\t%lld\n", (long long)ENUM_SIGNED(&record));
    printf("signedset\t%lld\n", (long long)ENUM_SIGNED_SET(&record, -123));
    printf("wide\t%llu\n", (unsigned long long)ENUM_WIDE(&record));
    printf("wideset\t%llu\n", (unsigned long long)ENUM_WIDE_SET(&record, 0xFFFFFFFFFFFFFFFEul));
    PackedEnumRecord packed;
    packed.small = (EnumSmall)231;
    printf("packed\t%llu\n", (unsigned long long)ENUM_PACKED(&packed));
    printf("packedset\t%llu\n", (unsigned long long)ENUM_PACKED_SET(&packed, 229));
    printf("valid\t%llu\n", (unsigned long long)ENUM_IDENTITY(SmallOne));
    printf("inline\t%llu\n", (unsigned long long)ENUM_INLINE(254));
    EnumKeywordRecord keyword;
    keyword.type = (EnumSmall)17;
    keyword.u32 = 21;
    printf("keyword\t%llu\n", (unsigned long long)ENUM_KEYWORD(&keyword));
    printf("keywordset\t%llu\n", (unsigned long long)ENUM_KEYWORD_SET(&keyword, 19));
    printf("primitivename\t%u\n", ENUM_PRIMITIVE_NAME(&keyword));
    printf("constant\t%llu\n", (unsigned long long)ENUM_CONSTANT(2));
    printf("constantuse\t%lld\n", (long long)ENUM_CONSTANT_USE(2));
    printf("constantzero\t%d\n", ENUM_CONSTANT_ZERO((EnumSmall *)0));
    printf("compare\t%d\n", ENUM_COMPARE(&record, (EnumSmallStorage *)&record.small));
    for (unsigned int i = 0; i < 256; ++i) {
        printf("boundary%u\t%llu\n", i, (unsigned long long)ENUM_SET(&record, i));
    }
    return 0;
}
