/* Microsoft sized integer suffixes as MSVC's <stdint.h> and <limits.h> spell them. Clang
 * accepts them only with Microsoft extensions, which MSVC targets enable by default. */
#define MS_INT8_MAX(unused) 127i8
#define MS_UINT8_MAX(unused) 0xffui8
#define MS_INT16_MIN(unused) (-32767i16 - 1)
#define MS_UINT16_MAX(unused) 0xffffui16
#define MS_INT32_MIN(unused) (-2147483647i32 - 1)
#define MS_UINT32_MAX(unused) 0xffffffffui32
#define MS_INT64_MIN(unused) (-9223372036854775807i64 - 1)
#define MS_INT64_MAX(unused) 9223372036854775807i64
#define MS_UINT64_MAX(unused) 0xffffffffffffffffui64
#define MS_OCTAL_UPPER_CASE(unused) 0777UI64
#define MS_TRAILING_UNSIGNED(unused) 1i16u

/* Clang truncates these to the suffix's width without a diagnostic. */
#define MS_TRUNCATED_INT8(unused) 128i8
#define MS_TRUNCATED_INT32(unused) 0xffffffffi32

/* PostgreSQL 18 defines PG_INT32_MIN and PG_INT64_MIN as the <stdint.h> limits, which use
 * the spellings above on Windows. */
#define WIN_INT32_MIN (-2147483647i32 - 1)
#define WIN_INT64_MIN (-9223372036854775807i64 - 1)
#define PG_INT32_MIN WIN_INT32_MIN
#define PG_INT64_MIN WIN_INT64_MIN
typedef int DateADT;
typedef long long Timestamp;
#define DATEVAL_NOBEGIN ((DateADT) PG_INT32_MIN)
#define DATE_IS_NOBEGIN(j) ((j) == DATEVAL_NOBEGIN)
#define DT_NOBEGIN PG_INT64_MIN
#define TIMESTAMP_IS_NOBEGIN(j) ((j) == DT_NOBEGIN)
