//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef unsigned int (*BuiltinCallback)(unsigned int);
extern unsigned int builtin_evaluations;
extern volatile unsigned int builtin_signal;
unsigned int builtin_record32(unsigned int value);
unsigned long long builtin_record64(unsigned long long value);
unsigned int *builtin_address(unsigned int *value);
int *builtin_take_pointer(int *value);
unsigned int builtin_use_callback(BuiltinCallback callback);

#ifdef BUILTIN_SHADOW
#define __builtin_bswap32(value) ((unsigned int) (value) + 3U)
#endif

#define BUILTIN_SWAP16(value) __builtin_bswap16(value)
#define BUILTIN_SWAP32(value) __builtin_bswap32(value)
#define BUILTIN_SWAP64(value) __builtin_bswap64(value)
#define BUILTIN_GROUPED(value) ((__builtin_bswap32))(value)
#define BUILTIN_NESTED32(value) BUILTIN_SWAP32(BUILTIN_SWAP32(value))
#ifdef __OPTIMIZE__
#define BUILTIN_PROFILE(value) __builtin_bswap16(value)
#else
#define BUILTIN_PROFILE(value) __builtin_bswap64(value)
#endif
#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
#define BUILTIN_HOST32(value) (value)
#else
#define BUILTIN_HOST32(value) BUILTIN_SWAP32(value)
#endif
#define BUILTIN_RECORD64(value) __builtin_bswap64(builtin_record64(value))
#define BUILTIN_VOLATILE() __builtin_bswap32(builtin_signal)
#define BUILTIN_SIZE(value) sizeof(__builtin_bswap32(value))
#define BUILTIN_LAZY(condition, pointer) ((condition) ? __builtin_bswap32(*(pointer)) : 0U)
#define BUILTIN_ZERO32() __builtin_bswap32(0)
#define BUILTIN_ZERO64() __builtin_bswap64(0)
#define BUILTIN_NULL_SELECT(condition, pointer) ((condition) ? BUILTIN_ZERO64() : (pointer))
#define BUILTIN_NULL_CALL() builtin_take_pointer(BUILTIN_ZERO32())
#define BUILTIN_RUNTIME_NULL(value) builtin_take_pointer(__builtin_bswap32(value))
#define BUILTIN_WRONG0() __builtin_bswap32()
#define BUILTIN_WRONG2(value) __builtin_bswap32(value, value)
#define BUILTIN_SYMBOL() __builtin_bswap32
#define BUILTIN_ADDRESS() (&__builtin_bswap32)
#define BUILTIN_DEREF() (*__builtin_bswap32)(7)
#define BUILTIN_CALLBACK() builtin_use_callback(__builtin_bswap32)
