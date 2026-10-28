/* Only qualifiers may follow a star in a concrete type operand. */
#define DECLARATOR_RESTRICT_SIZE(n) ((n) * sizeof(char *restrict))
#define DECLARATOR_GNU_RESTRICT_SIZE(n) ((n) * sizeof(char *__restrict const))
#define DECLARATOR_RESTRICT_CAST(p) (*(char *restrict) (p))
/* An array of pointers is not one pointer. */
#define DECLARATOR_POINTER_ARRAY_SIZE(n) ((n) * sizeof(char *[4]))
/* A named element type does not make the array spelling a value operand. */
typedef int DeclaratorInt;
typedef struct DeclaratorPair { int x; int y; } DeclaratorPair;
#define DECLARATOR_FOUR 4
#define DECLARATOR_TYPEDEF_ARRAY_SIZE(n) ((n) * sizeof(DeclaratorInt[4]))
#define DECLARATOR_RECORD_ARRAY_SIZE(n) ((n) * sizeof(DeclaratorPair[2]))
#define DECLARATOR_DEPENDENT_ARRAY_SIZE(n) ((n) * sizeof(DeclaratorInt[DECLARATOR_FOUR]))
#define DECLARATOR_TYPEDEF_POINTER_ARRAY_SIZE(n) ((n) * sizeof(DeclaratorInt *[4]))
/* Only a caller's local that shadows the typedef could make this valid C. */
#define DECLARATOR_TYPE_AS_VALUE(n) ((n) + DeclaratorInt)
