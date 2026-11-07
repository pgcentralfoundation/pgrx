/* Compiled with the original selected PostgreSQL wrapper included by the runner. */
#include <limits.h>
#include <stdint.h>

#define EXPECT_TYPE(value, type) _Static_assert(_Generic((value), type: 1, default: 0), #value)
EXPECT_TYPE(OffsetNumberNext((OffsetNumber) 0), OffsetNumber);
EXPECT_TYPE(OffsetNumberPrev((OffsetNumber) 0), OffsetNumber);
EXPECT_TYPE(IS_HIGHBIT_SET((unsigned char) 0), int);
EXPECT_TYPE(TYPEALIGN(8, 13), uintptr_t);
_Static_assert(sizeof(OffsetNumber) * CHAR_BIT == 16, "OffsetNumber uses the C uint16 contract");

static unsigned offset_evaluations;
static unsigned alignment_evaluations;
static unsigned length_evaluations;

static OffsetNumber offset_argument(void) { ++offset_evaluations; return 41; }
static int alignment_argument(void) { ++alignment_evaluations; return 8; }
static uintptr_t length_argument(void) { ++length_evaluations; return 13; }

int main(void) {
    unsigned value;
    uintptr_t alignment;
    uintptr_t length;
    uintptr_t aligned;
    if (OffsetNumberNext((OffsetNumber) 41) != 42 ||
        OffsetNumberPrev((OffsetNumber) 41) != 40 ||
        OffsetNumberNext((OffsetNumber) 65535) != 0 ||
        OffsetNumberPrev((OffsetNumber) 0) != 65535)
        return 1;
    /* Conversion to uintptr_t and unsigned addition have defined wrapping behavior. */
    if (TYPEALIGN(8, UINTPTR_MAX) != 0 || TYPEALIGN(8, -1) != 0)
        return 6;
    for (value = 0; value <= 255; ++value) {
        if (IS_HIGHBIT_SET((unsigned char) value) != (value >= 128 ? 128 : 0))
            return 2;
    }
    /* Power-of-two alignment and bounded lengths avoid every unsigned-wrap edge. */
    for (alignment = 1; alignment <= 64; alignment *= 2) {
        for (length = 0; length < 4096; ++length) {
            aligned = TYPEALIGN(alignment, length);
            if (aligned < length || aligned - length >= alignment || aligned % alignment != 0)
                return 3;
        }
    }
    if (OffsetNumberNext(offset_argument()) != 42 || offset_evaluations != 1)
        return 4;
    if (TYPEALIGN(alignment_argument(), length_argument()) != 16 ||
        alignment_evaluations != 2 || length_evaluations != 1)
        return 5;
    return 0;
}
