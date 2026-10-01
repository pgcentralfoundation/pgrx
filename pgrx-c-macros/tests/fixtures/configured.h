#ifdef ENABLE_BRANCH
#define ENABLED_VALUE 11
#else
#define DISABLED_VALUE 22
#endif

#if 0
#define INACTIVE_VALUE 33
#endif

#ifdef COMMAND_LINE_VALUE
#define COMMAND_LINE_PRESENT COMMAND_LINE_VALUE
#endif

#ifdef ENABLE_BRANCH
#define ENABLED_FUNCTION(value) ((value) + 11)
#else
#define DISABLED_FUNCTION(value) ((value) + 22)
#endif

#ifdef COMMAND_LINE_VALUE
#define COMMAND_LINE_FUNCTION(value) ((value) + COMMAND_LINE_VALUE)
#endif
