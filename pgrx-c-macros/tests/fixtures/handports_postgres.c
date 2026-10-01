/* Every invocation below expands the original configured PostgreSQL header. */
#include <limits.h>
#include <stdint.h>
#include <stdio.h>

/* The standalone runner uses libc rather than PostgreSQL's printf replacement. */
#undef printf

#define KIND(value) _Generic((value), \
    _Bool: "CBool", char: "CChar", signed char: "CSignedChar", \
    unsigned char: "CUnsignedChar", short: "CShort", unsigned short: "CUnsignedShort", \
    int: "CInt", unsigned int: "CUnsignedInt", long: "CLong", unsigned long: "CUnsignedLong", \
    long long: "CLongLong", unsigned long long: "CUnsignedLongLong", \
    __int128: "CInt128", unsigned __int128: "CUnsignedInt128")
#define RANK(value) _Generic((value), \
    _Bool: 0, char: 1, signed char: 1, unsigned char: 1, short: 2, unsigned short: 2, \
    int: 3, unsigned int: 3, long: 4, unsigned long: 4, long long: 5, unsigned long long: 5, \
    __int128: 6, unsigned __int128: 6)
#define IS_SIGNED(value) _Generic((value), \
    _Bool: 0, char: CHAR_MIN < 0, signed char: 1, unsigned char: 0, short: 1, unsigned short: 0, \
    int: 1, unsigned int: 0, long: 1, unsigned long: 0, long long: 1, unsigned long long: 0, \
    __int128: 1, unsigned __int128: 0)

static unsigned first_evaluations;
static unsigned second_evaluations;
static int alignment_argument(void) { ++first_evaluations; return 8; }
static uintptr_t length_argument(void) { ++second_evaluations; return 13; }
static Buffer buffer_argument(void) { ++first_evaluations; return -7; }
static TransactionId transaction_argument(void) { ++first_evaluations; return 3; }
static uint32 event_argument(void) { ++first_evaluations; return 0x1CU; }
static int sixbit_argument(void) { ++first_evaluations; return '0'; }
static unsigned sqlstate_evaluations[5];
static int sqlstate_argument(unsigned index) { ++sqlstate_evaluations[index]; return '0' + index; }
#ifdef ORACLE_PAGE_SIZE_MACRO
static Size page_size_argument(void) { ++first_evaluations; return BLCKSZ; }
#endif
#ifdef ORACLE_VARTAG_MACRO
static vartag_external vartag_argument(void) { ++first_evaluations; return (vartag_external)2; }
#endif

#define RECORD(name, index, expression) do { \
    __typeof__(expression) value = (expression); \
    const unsigned __int128 bits = (unsigned __int128)value; \
    printf("%s\t%u\t%s\t%u\t%d\t%d\t%016llx%016llx\t%u\t%u\n", \
        name, (unsigned)(index), KIND(value), (unsigned)(sizeof(value) * CHAR_BIT), \
        IS_SIGNED(value), RANK(value), (unsigned long long)(bits >> 64), \
        (unsigned long long)bits, first_evaluations, second_evaluations); \
} while (0)

int main(void) {
    /* Configuration is input selection for the paired runner, never a pgrx expectation. */
    printf("BLOCK_SIZE\t%u\n", (unsigned)BLCKSZ);
#ifdef ORACLE_VARTAG_MACRO
    printf("VARTAG_KIND\t%s\n", KIND((vartag_external)0));
#endif

    const Buffer buffers[] = { INT_MIN, INT_MIN + 1, -4096, -2, -1, 0, 1, 2, 4096, INT_MAX - 1, INT_MAX };
    for (unsigned index = 0; index < sizeof(buffers) / sizeof(buffers[0]); ++index)
        RECORD("BufferIsLocal_BOUNDARY", index, BufferIsLocal(buffers[index]));
    const TransactionId transactions[] = { 0, 1, 2, 3, 4, 0x7FFFFFFFU, 0x80000000U, 0xFFFFFFFEU, 0xFFFFFFFFU };
    for (unsigned index = 0; index < sizeof(transactions) / sizeof(transactions[0]); ++index)
        RECORD("TransactionIdIsNormal_BOUNDARY", index, TransactionIdIsNormal(transactions[index]));

    uint32 seed = 0xDECAFBADU;
    for (unsigned index = 0; index < 4096; ++index) {
        seed = seed * 1664525U + 1013904223U;
        RECORD("BufferIsLocal_SAMPLE", index, BufferIsLocal((Buffer)seed));
        RECORD("TransactionIdIsNormal_SAMPLE", index, TransactionIdIsNormal((TransactionId)seed));
    }

    const int alignments[] = { 1, 2, 4, 8, 16, 32, 64 };
    for (unsigned index = 0; index < sizeof(alignments) / sizeof(alignments[0]); ++index)
        for (unsigned length = 0; length < 1024; ++length)
            RECORD("TYPEALIGN_SAMPLE", index * 1024 + length, TYPEALIGN(alignments[index], length));
    for (unsigned length = 0; length < 1024; ++length)
        RECORD("MAXALIGN_SAMPLE", length, MAXALIGN(length));

    RECORD("TYPEALIGN_MAX", 0, TYPEALIGN(8, UINTPTR_MAX));
    RECORD("TYPEALIGN_NEAR_MAX", 0, TYPEALIGN(8, UINTPTR_MAX - 1));
    RECORD("TYPEALIGN_NEGATIVE_LENGTH", 0, TYPEALIGN(8, -1));
    RECORD("TYPEALIGN_MINIMUM_LENGTH", 0, TYPEALIGN(8, INT_MIN));
    RECORD("TYPEALIGN_ZERO_ALIGNMENT", 0, TYPEALIGN(0, 13));
    RECORD("TYPEALIGN_NEGATIVE_ALIGNMENT", 0, TYPEALIGN(-1, 13));
    RECORD("TYPEALIGN_SIGNED_CHAR", 0, TYPEALIGN((signed char)8, (unsigned char)255));
    RECORD("TYPEALIGN_UNSIGNED_ALIGNMENT", 0, TYPEALIGN(8U, UINTPTR_MAX));
    RECORD("TYPEALIGN_LONG_ALIGNMENT", 0, TYPEALIGN(8L, UINTPTR_MAX));
    RECORD("TYPEALIGN_LONG_LONG_ALIGNMENT", 0, TYPEALIGN(8LL, UINTPTR_MAX));
    RECORD("TYPEALIGN_UNSIGNED_LONG_LONG_ALIGNMENT", 0, TYPEALIGN(8ULL, UINTPTR_MAX));
    RECORD("TYPEALIGN_UNSIGNED_BOUNDARY_ALIGNMENT", 0, TYPEALIGN(UINT_MAX, 13));
    RECORD("MAXALIGN_MAX", 0, MAXALIGN(UINTPTR_MAX));
    RECORD("MAXALIGN_NEGATIVE", 0, MAXALIGN(-1));
    RECORD("BufferIsLocal_UNSIGNED_LITERAL", 0, BufferIsLocal(0x80000000U));
    RECORD("TransactionIdIsNormal_SIGNED_LITERAL", 0, TransactionIdIsNormal(-1));
    RECORD("TransactionIdIsNormal_LONG_LITERAL", 0, TransactionIdIsNormal(-1L));
    RECORD("TransactionIdIsNormal_LONG_LONG_LITERAL", 0, TransactionIdIsNormal(-1LL));
#ifdef ORACLE_WRAP_ALIGN
    RECORD("TYPEALIGN_MINIMUM_ALIGNMENT", 0, TYPEALIGN(INT_MIN, 13));
#endif

    first_evaluations = second_evaluations = 0;
    RECORD("TYPEALIGN_EVAL", 0, TYPEALIGN(alignment_argument(), length_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("MAXALIGN_EVAL", 0, MAXALIGN(length_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("BufferIsLocal_EVAL", 0, BufferIsLocal(buffer_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TransactionIdIsNormal_EVAL", 0, TransactionIdIsNormal(transaction_argument()));
#ifdef ORACLE_PAGE_SIZE_MACRO
    first_evaluations = second_evaluations = 0;
    RECORD("PageSizeIsValid_BELOW", 0, PageSizeIsValid((Size)BLCKSZ - 1));
    RECORD("PageSizeIsValid_EXACT", 0, PageSizeIsValid((Size)BLCKSZ));
    RECORD("PageSizeIsValid_ABOVE", 0, PageSizeIsValid((Size)BLCKSZ + 1));
    RECORD("PageSizeIsValid_UNSIGNED_LITERAL", 0, PageSizeIsValid((unsigned)BLCKSZ));
    RECORD("PageSizeIsValid_EVAL", 0, PageSizeIsValid(page_size_argument()));
#endif
    first_evaluations = second_evaluations = 0;
    for (uint32 event = 0; event < 256; ++event) {
        RECORD("TRIGGER_FIRED_BY_INSERT_LOW", event, TRIGGER_FIRED_BY_INSERT(event));
        RECORD("TRIGGER_FIRED_BY_DELETE_LOW", event, TRIGGER_FIRED_BY_DELETE(event));
        RECORD("TRIGGER_FIRED_BY_UPDATE_LOW", event, TRIGGER_FIRED_BY_UPDATE(event));
        RECORD("TRIGGER_FIRED_BY_TRUNCATE_LOW", event, TRIGGER_FIRED_BY_TRUNCATE(event));
        RECORD("TRIGGER_FIRED_FOR_ROW_LOW", event, TRIGGER_FIRED_FOR_ROW(event));
        RECORD("TRIGGER_FIRED_FOR_STATEMENT_LOW", event, TRIGGER_FIRED_FOR_STATEMENT(event));
        RECORD("TRIGGER_FIRED_BEFORE_LOW", event, TRIGGER_FIRED_BEFORE(event));
        RECORD("TRIGGER_FIRED_AFTER_LOW", event, TRIGGER_FIRED_AFTER(event));
        RECORD("TRIGGER_FIRED_INSTEAD_LOW", event, TRIGGER_FIRED_INSTEAD(event));
    }
    const uint32 masks[] = { 0x80000000U, 0xFFFFFF00U, 0xFFFFFFEFU, 0xFFFFFFFFU };
    for (unsigned mask = 0; mask < 4; ++mask) for (unsigned low = 0; low < 32; ++low) {
        const uint32 event = masks[mask] | low;
        const unsigned index = mask * 32 + low;
        RECORD("TRIGGER_FIRED_BY_INSERT_HIGH", index, TRIGGER_FIRED_BY_INSERT(event));
        RECORD("TRIGGER_FIRED_BY_DELETE_HIGH", index, TRIGGER_FIRED_BY_DELETE(event));
        RECORD("TRIGGER_FIRED_BY_UPDATE_HIGH", index, TRIGGER_FIRED_BY_UPDATE(event));
        RECORD("TRIGGER_FIRED_BY_TRUNCATE_HIGH", index, TRIGGER_FIRED_BY_TRUNCATE(event));
        RECORD("TRIGGER_FIRED_FOR_ROW_HIGH", index, TRIGGER_FIRED_FOR_ROW(event));
        RECORD("TRIGGER_FIRED_FOR_STATEMENT_HIGH", index, TRIGGER_FIRED_FOR_STATEMENT(event));
        RECORD("TRIGGER_FIRED_BEFORE_HIGH", index, TRIGGER_FIRED_BEFORE(event));
        RECORD("TRIGGER_FIRED_AFTER_HIGH", index, TRIGGER_FIRED_AFTER(event));
        RECORD("TRIGGER_FIRED_INSTEAD_HIGH", index, TRIGGER_FIRED_INSTEAD(event));
    }
    first_evaluations = second_evaluations = 0;
    RECORD("TRIGGER_FIRED_BY_INSERT_EVAL", 0, TRIGGER_FIRED_BY_INSERT(event_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TRIGGER_FIRED_BY_DELETE_EVAL", 0, TRIGGER_FIRED_BY_DELETE(event_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TRIGGER_FIRED_BY_UPDATE_EVAL", 0, TRIGGER_FIRED_BY_UPDATE(event_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TRIGGER_FIRED_BY_TRUNCATE_EVAL", 0, TRIGGER_FIRED_BY_TRUNCATE(event_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TRIGGER_FIRED_FOR_ROW_EVAL", 0, TRIGGER_FIRED_FOR_ROW(event_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TRIGGER_FIRED_FOR_STATEMENT_EVAL", 0, TRIGGER_FIRED_FOR_STATEMENT(event_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TRIGGER_FIRED_BEFORE_EVAL", 0, TRIGGER_FIRED_BEFORE(event_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TRIGGER_FIRED_AFTER_EVAL", 0, TRIGGER_FIRED_AFTER(event_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("TRIGGER_FIRED_INSTEAD_EVAL", 0, TRIGGER_FIRED_INSTEAD(event_argument()));
#ifdef ORACLE_VARTAG_MACRO
    first_evaluations = second_evaluations = 0;
    for (uint32 tag = 0; tag < 256; ++tag)
        RECORD("VARTAG_IS_EXPANDED_ENUM", tag, VARTAG_IS_EXPANDED((vartag_external)tag));
    const uint32 tags[] = { 0x80000000U, 0xFFFFFFFEU, 0xFFFFFFFFU, 0x7FFFFFFFU };
    for (unsigned index = 0; index < 4; ++index)
        RECORD("VARTAG_IS_EXPANDED_HIGH", index, VARTAG_IS_EXPANDED((vartag_external)tags[index]));
    RECORD("VARTAG_IS_EXPANDED_SIGNED", 0, VARTAG_IS_EXPANDED(-1));
    RECORD("VARTAG_IS_EXPANDED_BYTE", 0, VARTAG_IS_EXPANDED((unsigned char)3));
    RECORD("VARTAG_IS_EXPANDED_EVAL", 0, VARTAG_IS_EXPANDED(vartag_argument()));
#endif
    first_evaluations = second_evaluations = 0;
    for (int ch = 0; ch < 128; ++ch)
        RECORD("PGSIXBIT_ASCII", ch, PGSIXBIT(ch));
    for (unsigned char ch = 0; ch < 128; ++ch)
        RECORD("PGSIXBIT_BYTE_ASCII", ch, PGSIXBIT(ch));
    RECORD("PGSIXBIT_NEGATIVE_INT", 0, PGSIXBIT(-1));
    RECORD("PGSIXBIT_NEGATIVE_BYTE", 0, PGSIXBIT((signed char)-128));
    RECORD("PGSIXBIT_UNSIGNED_BYTE", 0, PGSIXBIT((unsigned char)255));
    RECORD("PGSIXBIT_UNSIGNED_ZERO", 0, PGSIXBIT(0U));
    RECORD("PGSIXBIT_UNSIGNED_MAX", 0, PGSIXBIT(UINT_MAX));
    RECORD("PGSIXBIT_CHARACTER_LITERAL", 0, PGSIXBIT('0'));
    RECORD("PGSIXBIT_LONG_LITERAL", 0, PGSIXBIT(-1L));
    seed = 0xA5E130C7U;
    for (unsigned index = 0; index < 4096; ++index) {
        int characters[5];
        for (unsigned character = 0; character < 5; ++character) {
            seed = seed * 1664525U + 1013904223U;
            characters[character] = (seed >> 24) & 0x7F;
        }
        RECORD("MAKE_SQLSTATE_SAMPLE", index,
            MAKE_SQLSTATE(characters[0], characters[1], characters[2], characters[3], characters[4]));
    }
    RECORD("MAKE_SQLSTATE_BYTES", 0,
        MAKE_SQLSTATE((unsigned char)0, (unsigned char)48, (unsigned char)65, (unsigned char)90, (unsigned char)127));
    RECORD("MAKE_SQLSTATE_SIGNED", 0, MAKE_SQLSTATE(-1, -128, -48, 48, 127));
    RECORD("MAKE_SQLSTATE_UNSIGNED", 0, MAKE_SQLSTATE(0U, UINT_MAX, 0x80000000U, 48U, 127U));
    RECORD("MAKE_SQLSTATE_MIXED", 0, MAKE_SQLSTATE(-1, (unsigned char)48, 65L, 90, UINT_MAX));
    RECORD("MAKE_SQLSTATE_CHARACTER_LITERALS", 0, MAKE_SQLSTATE('2', '2', '0', '1', '2'));
    RECORD("PGSIXBIT_EVAL", 0, PGSIXBIT(sixbit_argument()));
    first_evaluations = second_evaluations = 0;
    RECORD("MAKE_SQLSTATE_EVAL", 0,
        MAKE_SQLSTATE(sqlstate_argument(0), sqlstate_argument(1), sqlstate_argument(2), sqlstate_argument(3), sqlstate_argument(4)));
    printf("MAKE_SQLSTATE_ARGUMENT_EVAL\t%u\t%u\t%u\t%u\t%u\n",
        sqlstate_evaluations[0], sqlstate_evaluations[1], sqlstate_evaluations[2], sqlstate_evaluations[3], sqlstate_evaluations[4]);
    return 0;
}
