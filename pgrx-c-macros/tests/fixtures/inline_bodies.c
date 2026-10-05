/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */
#include <stdio.h>
int main(void)
{
    int value = 0;
    printf("macro\t%d\n", BODY_MACRO(3));
    printf("dropped\t%d\n", BODY_DROPPED(3));
    printf("branch\t%d\n", BODY_BRANCH(3));
    BODY_STORE(&value, 9);
    BODY_STORE(&value, -1);
    printf("store\t%d\n", value);
    BODY_EARLY(&value, 11);
    BODY_EARLY(&value, -1);
    printf("early\t%d\n", value);
    printf("color\t%d\t%d\n", (int) BODY_COLOR(0), (int) BODY_COLOR(5));
    printf("pair\t%d\n", BODY_PAIR(4).right);
    printf("caller\t%d\n", BODY_CALLER(1));
    printf("recursive\t%d\n", BODY_RECURSIVE(3));
    printf("late\t%d\n", BODY_LATE_USE(1));
    printf("redefined\t%d\n", BODY_REDEFINED_USE(1));
    printf("line\t%d\n", BODY_LINE());
    printf("argument\t%d\n", BODY_MACRO_ARG(13));
}
