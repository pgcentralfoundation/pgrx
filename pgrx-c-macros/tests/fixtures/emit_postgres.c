/* The runner includes the original configured PostgreSQL wrapper before this source. */
#include <limits.h>
#include <stdint.h>
#include <stdio.h>

/* c.h routes printf through PostgreSQL; this standalone oracle uses libc. */
#undef printf

#define KIND(value) _Generic((value), \
    _Bool: "CBool", char: "CChar", signed char: "CSignedChar", \
    unsigned char: "CUnsignedChar", short: "CShort", unsigned short: "CUnsignedShort", \
    int: "CInt", unsigned int: "CUnsignedInt", long: "CLong", unsigned long: "CUnsignedLong", \
    long long: "CLongLong", unsigned long long: "CUnsignedLongLong")
#define RANK(value) _Generic((value), \
    _Bool: 0, char: 1, signed char: 1, unsigned char: 1, short: 2, unsigned short: 2, \
    int: 3, unsigned int: 3, long: 4, unsigned long: 4, long long: 5, unsigned long long: 5)
#define IS_SIGNED(value) _Generic((value), \
    _Bool: 0, char: CHAR_MIN < 0, signed char: 1, unsigned char: 0, short: 1, unsigned short: 0, \
    int: 1, unsigned int: 0, long: 1, unsigned long: 0, long long: 1, unsigned long long: 0)

static unsigned first_evaluations;
static unsigned second_evaluations;
static int alignment_argument(void) { ++first_evaluations; return 8; }
static uintptr_t length_argument(void) { ++second_evaluations; return 13; }
static OffsetNumber offset_argument(void) { ++first_evaluations; return 41; }
static TransactionId first_transaction(void) { ++first_evaluations; return 42; }
static TransactionId second_transaction(void) { ++second_evaluations; return 42; }

#define RECORD(name, alignment, length, value) do { \
    const unsigned long long observed = (unsigned long long) (value); \
    printf("%s\t%u\t%u\t%s\t%u\t%d\t%d\t%llu\t%u\t%u\n", \
        name, (unsigned) (alignment), (unsigned) (length), KIND(value), \
        (unsigned) (sizeof(value) * CHAR_BIT), IS_SIGNED(value), RANK(value), observed, \
        first_evaluations, second_evaluations); \
} while (0)

int main(void) {
    int alignment;
    unsigned length;

    /* Bounded power-of-two alignments keep all these calls in the defined C domain. */
    for (alignment = 1; alignment <= 64; alignment *= 2) {
        for (length = 0; length < 4096; ++length) {
            RECORD("TYPEALIGN", alignment, length, TYPEALIGN(alignment, length));
            RECORD("TYPEALIGN_DOWN", alignment, length, TYPEALIGN_DOWN(alignment, length));
            RECORD("TYPEALIGN64", alignment, length, TYPEALIGN64(alignment, length));
        }
    }
    for (length = 0; length < 4096; ++length) {
        RECORD("MAXALIGN", 0, length, MAXALIGN(length));
        RECORD("MAXALIGN_DOWN", 0, length, MAXALIGN_DOWN(length));
        RECORD("MAXALIGN64", 0, length, MAXALIGN64(length));
    }
    /* uintptr_t conversions and unsigned additions wrap with defined semantics. */
    RECORD("TYPEALIGN_MAX", 8, 0, TYPEALIGN(8, UINTPTR_MAX));
    RECORD("TYPEALIGN_NEGATIVE", 8, 0, TYPEALIGN(8, -1));
    RECORD("TYPEALIGN_DOWN_MAX", 8, 0, TYPEALIGN_DOWN(8, UINTPTR_MAX));
    RECORD("TYPEALIGN64_MAX", 8, 0, TYPEALIGN64(8, UINT64_MAX));
    RECORD("MAXALIGN_MAX", 0, 0, MAXALIGN(UINTPTR_MAX));
    RECORD("MAXALIGN64_MAX", 0, 0, MAXALIGN64(UINT64_MAX));

    for (length = 0; length <= 65535; length += 257) {
        RECORD("OffsetNumberNext", 0, length, OffsetNumberNext((OffsetNumber) length));
        RECORD("OffsetNumberPrev", 0, length, OffsetNumberPrev((OffsetNumber) length));
    }
    for (length = 0; length < 256; ++length) {
        RECORD("IS_HIGHBIT_SET", 0, length, IS_HIGHBIT_SET((unsigned char) length));
    }
    for (length = 0; length <= 255; ++length) {
        const unsigned protocol = (length << 16) | (255 - length);
        RECORD("PG_PROTOCOL_MAJOR", 0, length, PG_PROTOCOL_MAJOR(protocol));
        RECORD("PG_PROTOCOL_MINOR", 0, length, PG_PROTOCOL_MINOR(protocol));
    }

    first_evaluations = second_evaluations = 0;
    RECORD("TYPEALIGN_EVAL", 8, 13, TYPEALIGN(alignment_argument(), length_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TYPEALIGN_DOWN_EVAL", 8, 13, TYPEALIGN_DOWN(alignment_argument(), length_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TYPEALIGN64_EVAL", 8, 13, TYPEALIGN64(alignment_argument(), length_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("MAXALIGN_EVAL", 0, 13, MAXALIGN(length_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("MAXALIGN_DOWN_EVAL", 0, 13, MAXALIGN_DOWN(length_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("MAXALIGN64_EVAL", 0, 13, MAXALIGN64(length_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("OffsetNumberNext_EVAL", 0, 41, OffsetNumberNext(offset_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("OffsetNumberPrev_EVAL", 0, 41, OffsetNumberPrev(offset_argument()));
    first_evaluations = second_evaluations = 0;

#ifdef ORACLE_TRANSACTION_EQUALS
    RECORD("TransactionIdEquals", 0, 42, TransactionIdEquals(first_transaction(), second_transaction()));
#endif
#ifdef ORACLE_TRANSACTION_VALID
    first_evaluations = second_evaluations = 0;
    RECORD("TransactionIdIsValid", 0, 42, TransactionIdIsValid(first_transaction()));
#endif
#ifdef ORACLE_TRANSACTION_NORMAL
    first_evaluations = second_evaluations = 0;
    RECORD("TransactionIdIsNormal", 0, 42, TransactionIdIsNormal(first_transaction()));
#endif
    return 0;
}
