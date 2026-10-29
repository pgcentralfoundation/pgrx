/* Synthetic original C definitions used by the independent compiler oracle. */
#ifndef PGRX_ORACLE_INTEGER_H
#define PGRX_ORACLE_INTEGER_H

#include <stdbool.h>
#include <stdint.h>

#define ORACLE_ADD(left, right) ((left) + (right))
#define ORACLE_NEGATE(value) (-(value))
#define ORACLE_COMPLEMENT(value) (~(value))
#define ORACLE_COMPARE(left, right) ((left) < (right))
#define ORACLE_CAST_U8(value) ((uint8_t) (value))
#define ORACLE_CAST_I16(value) ((int16_t) (value))
#define ORACLE_UNSIGNED_WRAP(value) ((value) + 1U)
#define ORACLE_UNGROUPED(value) value * 2
#define ORACLE_HEX 0xffffffff
#define ORACLE_DECIMAL 2147483648
#define ORACLE_REPEAT(value) ((value) + (value))
#define ORACLE_UNUSED(value) 17
#define ORACLE_AND(left, right) ((left) && (right))
#define ORACLE_OR(left, right) ((left) || (right))
#define ORACLE_CHOOSE(condition, yes, no) ((condition) ? (yes) : (no))
#define ORACLE_SEQUENCE(left, right) ((left), (right))
#define ORACLE_DIVIDE(left, right) ((left) / (right))
#define ORACLE_REMAINDER(left, right) ((left) % (right))
#define ORACLE_SHIFT(value, count) ((value) << (count))

#endif
