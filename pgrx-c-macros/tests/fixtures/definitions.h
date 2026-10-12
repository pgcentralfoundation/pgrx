#include "included.h"

#define OBJECT_PARENS (17)
#define OBJECT_SPACED (value) (value)
#define FUNCTION(value) ((value) + 1)
#define EMPTY_OBJECT
#define EMPTY_FUNCTION()
#define MULTILINE(value) \
    ((value) + \
     2)
#define COMMENTED(value) /* retained comment */ ((value) + 3)
#define STRINGIFY(value) #value
#define TOKEN_PASTE(left, right) left ## right
#define STRING_LITERAL "quoted \\ path"
#define VARIADIC(first, ...) first, __VA_ARGS__
#define GNU_VARIADIC(first, rest...) first, rest
#define REDEFINED 1
#undef REDEFINED
#define REDEFINED 2

static int scanner_body_fixture(void)
{
#define IN_BODY(value) ((value) * 2)
    return IN_BODY(3);
}

#define REDEFINED_FUNCTION(value) ((value) + 1)
#undef REDEFINED_FUNCTION
#define REDEFINED_FUNCTION(value) ((value) + 2)
