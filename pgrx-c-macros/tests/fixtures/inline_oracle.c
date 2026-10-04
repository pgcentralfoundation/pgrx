/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */
#include <stdio.h>
#include <string.h>
/* Separate function executions retain defined C sequencing for repeated operands. */
static unsigned int inline_evaluations;
static unsigned int inline_next(void) { return ++inline_evaluations; }
int main(void)
{
    int value = 9;
    inline_oracle_reset();
    unsigned int byte = INLINE_BYTE(-1);
    printf("byte\t%u\t%u\n", byte, inline_oracle_count());
    int signed_value = INLINE_SIGNED((short)12);
    printf("signed\t%d\t%u\n", signed_value, inline_oracle_count());
    int boolean = INLINE_BOOL(7);
    printf("bool\t%d\t%u\n", boolean, inline_oracle_count());
    double floating = INLINE_DOUBLE(1.5f);
    unsigned long long bits = 0;
    memcpy(&bits, &floating, sizeof(floating));
    printf("double\t%016llx\t%u\n", bits, inline_oracle_count());
    unsigned int write = INLINE_WRITE(&value, 65537u);
    printf("write\t%u\t%u\n", write, inline_oracle_count());
    INLINE_VOID(&value);
    printf("void\t%d\t%u\n", value, inline_oracle_count());
    int pointer = INLINE_POINTER(&value) == &value;
    printf("pointer\t%d\t%u\n", pointer, inline_oracle_count());
    int const_pointer = INLINE_CONST_POINTER(&value) == &value;
    printf("const_pointer\t%d\t%u\n", const_pointer, inline_oracle_count());
    int void_pointer = INLINE_VOID_POINTER(&value) == &value;
    printf("void_pointer\t%d\t%u\n", void_pointer, inline_oracle_count());
    unsigned int nested = INLINE_NESTED(65537u);
    printf("nested\t%u\t%u\n", nested, inline_oracle_count());
    unsigned int repeat = INLINE_REPEAT(inline_next());
    printf("repeat\t%u\t%u\t%u\n", repeat, inline_evaluations, inline_oracle_count());
    int lazy = INLINE_LAZY(0, inline_next());
    printf("lazy\t%d\t%u\n", lazy, inline_oracle_count());
    NativeWord word = (NativeWord)(void *)&value;
    NativeWord returned = INLINE_WORD(word);
    printf("word\t%d\t%u\n", returned == word, inline_oracle_count());
    int existing = INLINE_EXISTING(9);
    printf("existing\t%d\t%u\n", existing, inline_oracle_count());
    InlineRecord partial = INLINE_RECORD(23);
    printf("partial_record\t%d\t%u\n", partial.member, inline_oracle_count());
    int member = INLINE_RECORD_MEMBER(31);
    printf("partial_member\t%d\t%u\n", member, inline_oracle_count());
}
