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
    int mark = 0;
    BODY_MARK(&mark, &(int){0});
    printf("mark\t%d\n", mark);
    printf("sized\t%d\t%d\t%d\n", BODY_SIZED(1), BODY_SIZED_CALLER(1), BODY_SIZED_MACRO(3));
    printf("directive\t%d\t%d\n", BODY_DIRECTIVE_COMMENT(10), BODY_DIRECTIVE_SPLICE(10));
    printf("pushed\t%d\t%d\t%d\n", BODY_PUSHED(2), BODY_POPPED(1), BODY_HIDDEN(2));
    printf("transitive\t%d\n", BODY_TRANSITIVE_USE(1));
    printf("dropped later\t%d\n", BODY_DROPPED_LATER(1));
    printf("argument late\t%d\n", BODY_ARGUMENT_LATE(1));
    printf("rescan\t%d\t%d\t%d\n", BODY_RESCAN(3), BODY_RESCAN_ARGUMENT(3), BODY_RESCAN_TARGET(3));
    printf("template\t%d\t%d\t%d\t%d\n", BODY_FIRST_VALUE(), BODY_SECOND_VALUE(),
           BODY_FIRST_BRANCH(10), BODY_SECOND_BRANCH(10));
    printf("guarded\t%d\t%d\n", BODY_GUARDED_FIRST(), BODY_GUARDED_SECOND());
    int carriage = 0;
    BODY_CR_STORE(&carriage, 0);
    printf("spliced\t%d\t%d\t%d\t%d\t%d\t%d\n", BODY_BRACE_SPLICE(1), BODY_SPLICED_HASH(10),
           BODY_SPLICED_DIGRAPH(10), BODY_TRIGRAPH_SPLICE(10), BODY_TRIGRAPH_HASH(10), carriage);
}
