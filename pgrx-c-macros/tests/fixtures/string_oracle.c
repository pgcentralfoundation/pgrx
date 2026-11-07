//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#include <stdio.h>
#include <string.h>

static const char *condition;
static const char *filename;
static int line_number;
int string_global_values[3] = { 10, 20, 30 };
void string_failure(const char *failed, const char *file, int line)
{
    condition = failed;
    filename = file;
    line_number = line;
}
void string_diagnostics(const char *expected_file, int expected_line)
{
    printf("diagnostic\t%s\t%d\t%d\n", condition,
           strcmp(filename, expected_file) == 0, line_number == expected_line);
    fflush(stdout);
}

#ifdef PGRX_STRING_ORACLE_MAIN
static unsigned evaluations;
static int argument(int value) { ++evaluations; return value; }
static void record(const char *name, const char *value, unsigned length)
{
    printf("%s\t", name);
    for (unsigned index = 0; index < length; ++index)
        printf("%02x", (unsigned char)value[index]);
    printf("\t%u\n", evaluations);
}
int main(void)
{
    record("closed", STRING_CLOSED(), sizeof(STRING_CLOSED()));
    record("escapes", STRING_ESCAPES(), sizeof(STRING_ESCAPES()));
    record("embedded", STRING_EMBEDDED(), sizeof(STRING_EMBEDDED()));
    printf("size\t%lu\t%lu\t%d\n", (unsigned long)STRING_SIZE(),
           (unsigned long)STRING_DEREF_SIZE(STRING_POINTER()), STRING_CHAR(1));
    for (int value = 0; value <= 1; ++value)
    {
        evaluations = 0;
        const char *selected = STRING_SELECT(argument(value));
        record("select", selected, (unsigned)strlen(selected) + 1);
        evaluations = 0;
        selected = STRING_REPEAT(argument(value));
        record("repeat", selected, (unsigned)strlen(selected) + 1);
    }
    evaluations = 0;
    int expected_line = __LINE__ + 1;
    int checked = STRING_CHECK(argument(0));
    printf("check\t%d\t%u\n", checked, evaluations);
    string_diagnostics(__FILE__, expected_line);
    evaluations = 0;
    checked = STRING_CHECK(argument(1));
    printf("check\t%d\t%u\n", checked, evaluations);
    expected_line = __LINE__ + 1;
    STRING_LOCATION_NESTED();
    string_diagnostics(__FILE__, expected_line);
    expected_line = __LINE__ + 1;
    checked = STRING_LINE();
    printf("line\t%d\n", checked == expected_line);
    printf("file_size\t%d\n", STRING_FILE_SIZE() == sizeof(__FILE__));
    expected_line = __LINE__ + 1;
    STRING_DIAGNOSTIC(identifier);
    string_diagnostics(__FILE__, expected_line);
    expected_line = __LINE__ + 1;
    STRING_DIAGNOSTIC_FORWARD(* pointer);
    string_diagnostics(__FILE__, expected_line);
    expected_line = __LINE__ + 1;
    STRING_DIAGNOSTIC(((left + right), third));
    string_diagnostics(__FILE__, expected_line);
    expected_line = __LINE__ + 1;
    STRING_DIAGNOSTIC(left   +   right);
    string_diagnostics(__FILE__, expected_line);
    expected_line = __LINE__ + 1;
    STRING_DIAGNOSTIC("a\\\"b");
    string_diagnostics(__FILE__, expected_line);
    expected_line = __LINE__ + 1;
    STRING_DIAGNOSTIC(-17);
    string_diagnostics(__FILE__, expected_line);
    expected_line = __LINE__ + 1;
    STRING_DIAGNOSTIC(- value);
    string_diagnostics(__FILE__, expected_line);
    expected_line = __LINE__ + 1;
    STRING_DIAGNOSTIC_NESTED(identifier);
    string_diagnostics(__FILE__, expected_line);
    expected_line = __LINE__ + 1;
    STRING_DIAGNOSTIC_MULTI(first, (* second));
    string_diagnostics(__FILE__, expected_line);
    expected_line = __LINE__ + 1;
    STRING_TEXT_FORMAL(undefined);
    string_diagnostics(__FILE__, expected_line);
    evaluations = 0;
    expected_line = __LINE__ + 1;
    STRING_OPERAND_ASSERT(argument(0));
    printf("operand_assert\t%u\n", evaluations);
    string_diagnostics(__FILE__, expected_line);
    evaluations = 0;
    STRING_OPERAND_ASSERT(argument(1));
    printf("operand_assert\t%u\n", evaluations);
    return 0;
}
#endif
