/* Header-only target fixture: inspection must preserve LLP64 without needing a Windows SDK. */
#define FRONT_LLP64_LONG(value) ((long) (value))
#define FRONT_LLP64_SIZE(value) (sizeof(value))
