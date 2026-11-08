/* glibc's <stdint.h> undefines Clang's __INT64_C predefine and defines it again with the
 * same replacement. __INT32_C's replacement is the same on every target. */
#undef __INT32_C
#define __INT32_C(c) c

#define FRONT_PREDEFINE_USER(value) ((value) == __INT32_C(7))
