/* A sizeof or alignment type use never erases another use of the same formal. */
#define ROLE_VALUE_THEN_ALIGN(value) ((value) + _Alignof(value))
#define ROLE_ALIGN_THEN_VALUE(value) (_Alignof(value) + (value))
#define ROLE_MEMBER_THEN_SIZE(pointer, field) ((pointer)->field + sizeof(field *))
#define ROLE_SIZE_THEN_MEMBER(pointer, field) (sizeof(field *) + (pointer)->field)

/* Type evidence anywhere in the replacement makes (T) before an operator a cast. */
#define ROLE_CAST_THEN_SIZE(T) ((T) - 1 + sizeof(T *))
#define ROLE_SIZE_THEN_CAST(T, value) (sizeof(T *) + (T) + (value))
/* GNU alignment also accepts a value, so it cannot decide that (value) - 1 is a cast. */
#define ROLE_ALIGN_SUBTRACT(value, other) ((value) - 1 + _Alignof(value) + sizeof(other))

typedef long RoleField;
struct RoleRecord {
    long RoleField;
};
