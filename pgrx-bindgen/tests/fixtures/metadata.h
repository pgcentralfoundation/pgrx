/* Synthetic declaration metadata for isolated binding-generator tests.
 * Deliberately distinct values prove that C owns initialization. */
typedef struct { int len; int version; } Pg_magic_struct;
typedef struct { int api_version; } Pg_finfo_record;
#define PG_MODULE_MAGIC_DATA(...) { sizeof(Pg_magic_struct), 37 }
#define PG_FUNCTION_INFO_V1(function) \
    const Pg_finfo_record *pg_finfo_##function(void) { \
        static const Pg_finfo_record info = { 23 }; \
        return &info; \
    }
