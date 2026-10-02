#include "frontend_context.h"
extern int analysis_function(int value);

#define ANALYSIS_ID(value) (value)
#define ANALYSIS_ADD(left, right) ((left) + (right))
#define ANALYSIS_UNARY(value) (~(-(value)))
#define ANALYSIS_PRECEDENCE(value) (((value) + 2 * 3) << 1)
#define ANALYSIS_CAST(value) ((FrontByte) (value))
#define ANALYSIS_LITERAL(value) ((value) + 0xffUL)
#define ANALYSIS_ENUM(value) ((value) + FRONT_ENUM_SEVEN)
#define ANALYSIS_BOOL_CAST(value) ((_Bool) ((value) < 0))
#define ANALYSIS_UNUSED(value) 17
#define ANALYSIS_REPEAT(value) ((value) + (value))
#define ANALYSIS_LAZY(left, right) ((left) && (right))
#define ANALYSIS_CHOOSE(condition, yes, no) ((condition) ? (yes) : (no))
#define ANALYSIS_COMMENT(value) ((value) /* retained */ + 1)
#define ANALYSIS_UNSIGNED_LITERAL(value) ((value) + 18446744073709551615ULL)

#define ANALYSIS_UNGROUPED(value) value * 2
#define ANALYSIS_ROOT_UNGROUPED(value) (value) + 1
#define ANALYSIS_POINTER(value) (*(value))
#define ANALYSIS_MEMBER(value) ((value)->field)
#define ANALYSIS_SUBSCRIPT(value) ((value)[0])
#define ANALYSIS_MUTATION(value) (++(value))
#define ANALYSIS_ASSIGN(value) ((value) = 1)
#define ANALYSIS_CALL(value) analysis_function((value))
#define ANALYSIS_STATEMENT(value) do { (value)++; } while (0)
#define ANALYSIS_CONTROL_FLOW(value) do { while (value) (value)--; } while (0)
#define ANALYSIS_SIZEOF(value) sizeof(value)
#define ANALYSIS_TYPE_PARAMETER(type, value) ((type) (value))
#define ANALYSIS_COMMA(left, right) ((left), (right))
#define ANALYSIS_FLOAT(value) ((value) + 1.5)
#define ANALYSIS_CHAR(value) ((value) + 'a')
#define ANALYSIS_UNKNOWN(value) ((value) + ANALYSIS_MISSING_IDENTIFIER)
#define ANALYSIS_VARIABLE(value) ((value) + front_const_variable)
#define ANALYSIS_EMPTY(value)
#define ANALYSIS_STRINGIFY(value) #value
#define ANALYSIS_PASTE(left, right) left ## right
#define ANALYSIS_VARIADIC(first, ...) ((first) + __VA_ARGS__)

#define ANALYSIS_LATE_USER(value) ((value) + ANALYSIS_LATE_VALUE)
#define ANALYSIS_LATE_VALUE 1
#undef ANALYSIS_LATE_VALUE
#define ANALYSIS_LATE_VALUE 9
#define ANALYSIS_NESTED(value) ANALYSIS_ADD(value, 3)
#define ANALYSIS_RECURSIVE(value) ANALYSIS_RECURSIVE(value)
#define ANALYSIS_CYCLE_A ANALYSIS_CYCLE_B
#define ANALYSIS_CYCLE_B ANALYSIS_CYCLE_A
#define ANALYSIS_CYCLE(value) ((value) + ANALYSIS_CYCLE_A)

#define ANALYSIS_AMBIGUOUS(value) ((value) + 5)
#undef ANALYSIS_AMBIGUOUS
#define ANALYSIS_AMBIGUOUS(value) ((value) + 5)
#define ANALYSIS_OBJECT 41
#define ANALYSIS_LITERAL_TOO_LARGE(value) ((value) + 18446744073709551616ULL)
