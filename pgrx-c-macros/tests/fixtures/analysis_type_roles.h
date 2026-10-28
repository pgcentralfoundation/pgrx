/* A sizeof or alignment type use never erases another use of the same formal. */
#define ROLE_VALUE_THEN_ALIGN(value) ((value) + _Alignof(value))
#define ROLE_ALIGN_THEN_VALUE(value) (_Alignof(value) + (value))
#define ROLE_MEMBER_THEN_SIZE(pointer, field) ((pointer)->field + sizeof(field *))
#define ROLE_SIZE_THEN_MEMBER(pointer, field) (sizeof(field *) + (pointer)->field)

typedef long RoleField;
struct RoleRecord {
    long RoleField;
};
