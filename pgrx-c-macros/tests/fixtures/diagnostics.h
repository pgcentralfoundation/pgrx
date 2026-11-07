#if defined(TEST_MISSING_INCLUDE)
#include "pgrx_macro_fixture_missing_header.h"
#endif

#if defined(TEST_ERROR)
#error expected macro scanner failure
#endif

#if defined(TEST_WARNING)
#warning expected macro scanner warning
#endif

#define AFTER_DIAGNOSTIC 7
