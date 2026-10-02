typedef int (*NullCallback)(int);
enum { NULL_ZERO = 0 };
void null_reset(void);
int null_count(void);
int null_increment(int value);
NullCallback null_get(int present);
NullCallback null_identity_callback(NullCallback value);
int null_callback_present(NullCallback value);
int *null_identity_object(int *value);

#define NULL_CONST() ((void *)0)
#define NULL_CAST_CONST() ((void *)(int)0)
#define NULL_ENUM_CONST() ((void *)NULL_ZERO)
#define NULL_INT_CONST() ((int)0)
#define NULL_ALIAS() NULL_CONST()
#define NULL_PICK(condition, pointer) ((condition) ? NULL_CONST() : (pointer))
#define NULL_PICK_REVERSE(condition, pointer) ((condition) ? (pointer) : NULL_CONST())
#define NULL_BOTH(condition) ((condition) ? NULL_CONST() : NULL_CONST())
#define NULL_EQUAL(pointer) ((pointer) == NULL_CONST())
#define NULL_EQUAL_REVERSE(pointer) (NULL_CONST() == (pointer))
#define NULL_NOT_EQUAL(pointer) ((pointer) != NULL_CONST())
#define NULL_COMPARE(left, right) ((left) == (right))
#define NULL_PASS_FUN(value) null_identity_callback((value))
#define NULL_PASS_OBJECT(value) null_identity_object((value))
#define NULL_PASS_INLINE() null_identity_callback(NULL_CONST())
#define NULL_PRESENT_INLINE() null_callback_present(NULL_CONST())
#define NULL_TYPED_INLINE() null_identity_callback((NullCallback)NULL_CONST())
#define NULL_GET(value) null_get((value))
#define NULL_SIZE(value) (sizeof(value))
#define NULL_SELF_SIZE() (sizeof(NULL_CONST()))
#define NULL_DISCARD(value) ((void)(value))
#define NULL_CONST_QUALIFIED() ((const void *)0)
#define NULL_VOLATILE_QUALIFIED() ((volatile void *)0)
#define NULL_POINTER_CAST() ((void *)(void *)0)
#define NULL_RUNTIME_CAST(value) ((void *)(value))
#define NULL_COMPOUND_CONST() ((void *)(1 - 1))
#define NULL_UNSIGNED_CAST() ((void *)(unsigned char)256)
#define NULL_LAZY_ZERO() ((void *)(1 ? 0 : (1 / 0)))
#define NULL_LOGICAL_ZERO() ((void *)(0 && (1 / 0)))
#define NULL_COMPOUND_INT() ((5 * 7) - 35)
#define NULL_SIGNED_INT() ((-1) + 1)
#define NULL_RUNTIME_SUM(value) ((void *)((value) + 0))
#define NULL_COMMA_ZERO() ((void *)(1, 0))
#define NULL_SUB_INT(value) ((value) - 1)
#define NULL_DELEGATED_ZERO() NULL_SUB_INT(1)
#define NULL_LITERAL_PICK(condition, left, right) ((condition) ? (left) : (right))
#define NULL_NEGATIVE_VALUE(value) ((value) + 0)
#define NULL_CANCEL_VOID() (&*NULL_CONST())
