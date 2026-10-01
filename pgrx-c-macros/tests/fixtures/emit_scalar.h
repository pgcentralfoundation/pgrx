typedef unsigned short EmitWord;
enum { EMIT_SEVEN = 7 };

#define EMIT_ID(value) (value)
#define EMIT_ADD(left, right) ((left) + (right))
#define EMIT_NEGATE(value) (-(value))
#define EMIT_PLUS(value) (+(value))
#define EMIT_NOT(value) (!(value))
#define EMIT_REPEAT(value) ((value) + (value))
#define EMIT_UNUSED(value) 17
#define EMIT_WORD(value) ((EmitWord) (value))
#define EMIT_BOOL(value) ((_Bool) (value))
#define EMIT_COMPARE(left, right) ((left) < (right))
#define EMIT_CONSTANT(value) EMIT_SEVEN
#define EMIT_LONG(value) 42L
#define EMIT_UNSIGNED_LONG_LONG(value) 18446744073709551615ULL
#define EMIT_KEYWORDS(type, match) ((type) + (match))
#define EMIT_DIVIDE(left, right) ((left) / (right))
#define EMIT_REMAINDER(left, right) ((left) % (right))
#define EMIT_SHIFT(value, count) ((value) << (count))
#define EMIT_SHIFT_RIGHT(value, count) ((value) >> (count))
#define EMIT_BITS(left, right) ((left) ^ (~(right)))
#define EMIT_LOGICAL_AND(left, right) ((left) && (right))
#define EMIT_LOGICAL_OR(left, right) ((left) || (right))
#define EMIT_CHOOSE(condition, yes, no) ((condition) ? (yes) : (no))
#define match(value) ((value) + 1)

#define EMIT_UNGROUPED(value) (value) + 1
#define EMIT_UNKNOWN(value) ((value) + EMIT_UNDECLARED)
#define EMIT_POINTER(value) (*(value))
