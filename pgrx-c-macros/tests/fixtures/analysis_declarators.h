/* Only qualifiers may follow a star in a concrete type operand. */
#define DECLARATOR_RESTRICT_SIZE(n) ((n) * sizeof(char *restrict))
#define DECLARATOR_GNU_RESTRICT_SIZE(n) ((n) * sizeof(char *__restrict const))
#define DECLARATOR_RESTRICT_CAST(p) (*(char *restrict) (p))
/* An array of pointers is not one pointer. */
#define DECLARATOR_POINTER_ARRAY_SIZE(n) ((n) * sizeof(char *[4]))
