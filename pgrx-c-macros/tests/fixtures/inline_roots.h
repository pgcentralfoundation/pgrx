/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */

typedef __SIZE_TYPE__ RootSize;
typedef const void RootConstVoid;
typedef enum RootEnum { RootZero = 0, RootOne = 1 } RootEnum;
static inline int ROOT_COLLISION(int value) { return value + 100; }
#define ROOT_COLLISION(value) ((value) + 50)

#ifdef ROOT_NATIVE
static inline int ROOT_INT(int value) { return value + 3; }
static inline _Bool ROOT_PRED(int value) { return value != 0; }
static inline RootSize ROOT_WORD(RootSize value) { return value; }
static inline void *ROOT_POINTER(void *where) { return where; }
static inline const void *ROOT_CONST_POINTER(const void *where) { return where; }
static inline RootEnum ROOT_ENUM(int value) { return (RootEnum)value; }
static inline void ROOT_VOID(int *where, int value) { *where = value; }
static inline unsigned int ROOT_REPEAT(unsigned int value) { return value + value; }
static inline RootSize ROOT_SIZE(unsigned int value) { return sizeof(value); }
static inline unsigned int ROOT_LAZY(int condition, unsigned int value) { return condition ? value : 0; }
static inline int ROOT_CAPTURE(int ROOT_CAPTURE) { return ROOT_CAPTURE; }
static inline int __pgrx_inline_parameter_0(int value) { return value; }
static inline int ROOT_ZERO(void) { return 19; }
static inline int ROOT_VARIADIC(int value, ...) { return value; }
static inline int ROOT_UNPROTOTYPED() { return 1; }
static inline int ROOT_UNDEFINED(int value);
static inline int __attribute__((preserve_most)) ROOT_ABI(int value) { return value; }
#else
#define ROOT_INT(value) ((value) + 3)
#define ROOT_PRED(value) ((value) != 0)
#define ROOT_WORD(value) ((unsigned int)(value))
#define ROOT_POINTER(where) ((void *)(where))
#define ROOT_CONST_POINTER(where) ((RootConstVoid *)(where))
#define ROOT_ENUM(value) ((RootEnum)(value))
#define ROOT_VOID(where, value) ((void)(*(where) = (value)))
#define ROOT_REPEAT(value) ((value) + (value))
#define ROOT_SIZE(value) (sizeof(value))
#define ROOT_LAZY(condition, value) ((condition) ? (value) : 0u)
#define ROOT_CAPTURE(value) (value)
#define __pgrx_inline_parameter_0(value) (value)
#define ROOT_ZERO() 19
#endif
