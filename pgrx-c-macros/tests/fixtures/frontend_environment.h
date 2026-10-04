#include "frontend_context.h"

#define FRONT_DELETED(value) ((value) + 1)
#undef FRONT_DELETED
#define FRONT_REDEFINED(value) ((value) + 1)
#undef FRONT_REDEFINED
#define FRONT_REDEFINED(value) ((value) + 2)

#define FRONT_RESTORED(value) ((value) + 5)
#pragma push_macro("FRONT_RESTORED")
#undef FRONT_RESTORED
#define FRONT_RESTORED(value) ((value) + 99)
#pragma pop_macro("FRONT_RESTORED")

#define FRONT_IDENTICAL(value) ((value) + 3)
#undef FRONT_IDENTICAL
#define FRONT_IDENTICAL(value) ((value) + 3)

#define FRONT_LATE_USER(value) ((value) + FRONT_LATE_VALUE)
#define FRONT_LATE_VALUE 6
#undef FRONT_LATE_VALUE
#define FRONT_LATE_VALUE 9
#undef FRONT_EXTERNAL_VALUE
#define FRONT_EXTERNAL_VALUE 8

#if defined(FRONT_COMMAND_LINE)
#define FRONT_COMMAND_PRESENT(value) FRONT_COMMAND_LINE(value)
#else
#define FRONT_COMMAND_ABSENT(value) (value)
#endif
#if 0
#define FRONT_INACTIVE(value) ((value) + 100)
#endif

#if __has_include("frontend_optional_missing.h")
#define FRONT_OPTIONAL_PRESENT(value) (value)
#else
#define FRONT_OPTIONAL_ABSENT(value) (value)
#endif
