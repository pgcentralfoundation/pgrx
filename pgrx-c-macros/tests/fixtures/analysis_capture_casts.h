/* Keep unresolved caller names out of the inspected declaration catalog. */
#define CAPTURE_UNKNOWN_SUBTRACT(x) ((x) == ((UnresolvedId) -1))
#define CAPTURE_UNKNOWN_ADD(x) ((UnresolvedId) + (x))
#define CAPTURE_UNKNOWN_MULTIPLY(x) ((UnresolvedId) * (x))
#define CAPTURE_UNKNOWN_AND(x) ((UnresolvedId) & (x))
#define CAPTURE_UNKNOWN_CALL(x) ((UnresolvedId)(x))
#define CAPTURE_DIRECT_CALL(x) (callee(x))
#define CAPTURE_NESTED_CALL(x) (((callee))(x))
#define CAPTURE_EMPTY_CALL() ((callee)())
#define CAPTURE_POSTFIX() ((counter)++)

extern unsigned int bound_value;
extern int bound_function(int);
typedef unsigned int KnownId;
#define CAPTURE_KNOWN_VALUE(x) ((bound_value) - (x))
#define CAPTURE_KNOWN_FUNCTION(x) ((bound_function)(x))
#define CAPTURE_KNOWN_TYPE(x) ((x) == ((KnownId) -1))
