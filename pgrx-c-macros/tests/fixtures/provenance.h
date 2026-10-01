#define SINGLE_LINE(value) ((value) + 1)
#define EMPTY_OBJECT_RANGE
#define EMPTY_FUNCTION_RANGE()
#define EMPTY_CONTINUED \
    \

#define TRAILING_COMMENT(value) value /* trailing comment
    occupies another physical line */
#define NEXT_VALUE 9
#define LAST_TOKEN(value) \
    value
/* comment outside macro */
#line 900 "audit_virtual.h"
#define AFTER_LINE_DIRECTIVE(value) value
#define MIDDLE_COMMENT(value) /* middle comment
    occupies another physical line */ value
