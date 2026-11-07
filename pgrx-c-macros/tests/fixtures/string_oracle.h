//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#ifndef PGRX_STRING_ORACLE_H
#define PGRX_STRING_ORACLE_H

void string_failure(const char *condition, const char *file, int line);
void string_diagnostics(const char *expected_file, int expected_line);
int string_result(const char *condition);
extern int string_global_values[3];

#define STRINGIFY_OPERAND(value) #value
#define STRINGIFY_EXPANDED(value) STRINGIFY_OPERAND(value)
#define STRING_CLOSED() STRINGIFY_OPERAND(closed "quoted" tokens)
#define STRING_ESCAPES() "a\n\t\x7f\101\\\"\?"
#define STRING_EMBEDDED() "a\0b" "c"
#define STRING_SIZE() sizeof(STRING_EMBEDDED())
#define STRING_POINTER() (&"ab")
#define STRING_DEREF_SIZE(pointer) sizeof(*(pointer))
#define STRING_CHAR(index) ("ab"[(index)])
#define STRING_SELECT(value) ((value) ? "yes" : "no")
#define STRING_REPEAT(value) ((value) ? ((value), "yes") : "no")
#define STRING_ASSERT(condition) \
    ((void)((condition) || (string_failure(#condition, __FILE__, __LINE__), 0)))
#define STRING_CHECK(value) ((value) > 0 ? 7 : (STRING_ASSERT(0), (value)))
#define STRING_LOCATION() (string_failure("location", __FILE__, __LINE__))
#define STRING_LOCATION_NESTED() STRING_LOCATION()
#define STRING_DIAGNOSTIC(value) (string_failure(STRINGIFY_OPERAND(value), __FILE__, __LINE__))
#define STRING_DIAGNOSTIC_FORWARD(value) STRING_DIAGNOSTIC(value)
#define STRING_DIAGNOSTIC_NESTED(value) STRING_DIAGNOSTIC(((value) + 1))
#define STRING_DIAGNOSTIC_MULTI(first, second) STRING_DIAGNOSTIC((first, second))
#define STRING_OPERAND_ASSERT(value) STRING_ASSERT(value)
#define STRING_TEXT_FORMAL(value) (string_failure("value", __FILE__, __LINE__))
#define STRING_OPEN_SIZE(value) sizeof(STRINGIFY_OPERAND(value))
#define STRING_OPEN_ADDRESS(value) (&STRINGIFY_OPERAND(value))
#define STRING_OPEN_CONCAT(value) (string_failure(STRINGIFY_OPERAND(value) " suffix", __FILE__, __LINE__))
#define STRING_OPEN_RESULT(value) (string_result(STRINGIFY_OPERAND(value)))
#define STRING_LINE() __LINE__
#define STRING_FILE_SIZE() sizeof(__FILE__)
#define STRING_WRITE() ("ab"[0] = 1)

#define STRING_OBJECT_OFFSET 1
#define STRING_OBJECT_NUMBER (STRING_OBJECT_OFFSET + 41)
#define STRING_OBJECT_ELEMENT (&string_global_values[STRING_OBJECT_OFFSET])
#define STRING_OBJECT_LITERAL "object"
#define STRING_OBJECT_COUNTER __COUNTER__

#define STRING_OPEN(value) STRINGIFY_OPERAND(value)
#define STRING_CONTEXT() STRINGIFY_EXPANDED(__FILE__)
#define STRING_FUNCTION() __func__
#define STRING_WIDE() L"wide"
#define STRING_UTF8() u8"utf8"
#define STRING_HIGH() "\x80"
#define STRING_NONASCII() "é"
#define STRING_UNIVERSAL() "\u00e9"

#endif
