//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

extern unsigned int cleanup_calls;
void cleanup_tick(void);
unsigned int cleanup_value(void);

#define CLEAN_EXPR_ZERO() 7
#define CLEAN_EXPR_BOUNDARY() 1 + 2
#define CLEAN_CAPTURE_ZERO() (free_value + 1)
#define CLEAN_STMT_ZERO() do { cleanup_tick(); } while (0)
#define CLEAN_STMT_BOUNDARY() if (0) cleanup_tick()
#define CLEAN_LOCAL_ZERO() do { int temporary = 3; (void) temporary; cleanup_tick(); } while (0)
#define CLEAN_DISCARD(value) do { value; } while (0)
#define CLEAN_LOCAL(value) do { int temporary = (value); (void) temporary; cleanup_tick(); } while (0)
#define CLEAN_TYPE(type) do { (void) ((type) (1)); cleanup_tick(); } while (0)
#define CLEAN_TYPE_PROVEN(type) do { (void) sizeof(type *); (void) ((type) (1)); cleanup_tick(); } while (0)
#define CLEAN_UNUSED(unused) do { cleanup_tick(); } while (0)
#define CLEAN_REPEAT(value) do { (void) (value); (void) (value); } while (0)
#define CLEAN_RETURN_ZERO() return 256
#define CLEAN_RETURN_ALIAS() CLEAN_RETURN_ZERO()
#define CLEAN_RETURN_IF(condition) do { if (condition) return 7; cleanup_tick(); } while (0)
