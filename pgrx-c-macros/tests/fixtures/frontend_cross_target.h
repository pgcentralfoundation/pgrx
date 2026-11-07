/* Header-only target fixture: a cross compiler profile must not need target system headers. */
#define FRONT_CROSS_LONG(value) ((long) (value))
#define FRONT_CROSS_SIZE(value) (sizeof(value))
