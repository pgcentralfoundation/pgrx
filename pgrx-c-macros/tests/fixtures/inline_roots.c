/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */
#include <stdio.h>
#define KIND(value) _Generic((value), int: 1, _Bool: 2, unsigned int: 3, unsigned long: 4, unsigned long long: 5)
static unsigned int evaluations;
/* Function executions are sequenced relative to each other, unlike two ++ operands. */
static unsigned int next_value(void) { return ++evaluations; }
int main(void)
{
    int value = 7;
    printf("int\t%d\n", ROOT_INT(-2));
    printf("truth\t%d\t%d\t%d\n", !!ROOT_PRED(-2), !!ROOT_PRED(0), KIND(ROOT_PRED(0)));
    printf("word\t%llu\t%d\n", (unsigned long long)ROOT_WORD(7u), KIND(ROOT_WORD(7u)));
    printf("pointer\t%d\t%d\n", ROOT_POINTER(&value) == &value, ROOT_CONST_POINTER(&value) == &value);
    printf("enum\t%llu\n", (unsigned long long)ROOT_ENUM(17));
    ROOT_VOID(&value, 23);
    printf("void\t%d\n", value);
    unsigned int repeated = ROOT_REPEAT(next_value());
    printf("repeat\t%u\t%u\n", repeated, evaluations);
    evaluations = 0;
    RootSize size = ROOT_SIZE(next_value());
    printf("size\t%llu\t%u\n", (unsigned long long)size, evaluations);
    evaluations = 0;
    unsigned int lazy = ROOT_LAZY(0, next_value());
    printf("lazy\t%u\t%u\n", lazy, evaluations);
    printf("capture\t%d\t%d\n", ROOT_CAPTURE(31), __pgrx_inline_parameter_0(37));
    printf("collision\t%d\n", ROOT_COLLISION(1));
    printf("zero\t%d\n", ROOT_ZERO());
}
