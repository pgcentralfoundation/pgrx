struct BitFields {
    unsigned int left : 3;
    unsigned int narrow : 7;
    unsigned int right : 22;
    signed int signed_bits : 5;
    unsigned int sentinel : 27;
    unsigned int full : 32;
    unsigned long long wide : 40;
    unsigned long long small_wide : 7;
    _Bool truth : 1;
    unsigned int unused;
};
struct __attribute__((packed)) PackedBits {
    unsigned int narrow : 7;
    unsigned int neighbor : 1;
};
struct __attribute__((packed)) PackedContainer {
    char lead;
    struct BitFields child;
};
struct VolatileBits {
    volatile unsigned int narrow : 7;
    volatile _Bool truth : 1;
};

#define BIT_READ(p) ((p)->narrow)
#define BIT_SET(p, x) ((p)->narrow = (x))
#define BIT_ADD(p, x) ((p)->narrow += (x))
#define BIT_PRE(p) (++(p)->narrow)
#define BIT_POST(p) ((p)->narrow++)
#define BIT_NOT(p) (~(p)->narrow)
#define BIT_POST_NOT(p) (~((p)->narrow++))
#define BIT_SIGNED(p) ((p)->signed_bits)
#define BIT_SIGNED_SET(p, x) ((p)->signed_bits = (x))
#define BIT_FULL(p) ((p)->full)
#define BIT_FULL_SET(p, x) ((p)->full = (x))
#define BIT_WIDE(p) ((p)->wide)
#define BIT_WIDE_SET(p, x) ((p)->wide = (x))
#define BIT_TRUTH(p) ((p)->truth)
#define BIT_TRUTH_SET(p, x) ((p)->truth = (x))
#define BIT_LEFT(p) ((p)->left)
#define BIT_LEFT_SET(p, x) ((p)->left = (x))
#define BIT_RIGHT(p) ((p)->right)
#define BIT_RIGHT_SET(p, x) ((p)->right = (x))
#define BIT_SENTINEL(p) ((p)->sentinel)
#define BIT_SENTINEL_SET(p, x) ((p)->sentinel = (x))
#define BIT_NESTED(p) ((p)->child.narrow)
#define BIT_NESTED_SET(p, x) ((p)->child.narrow = (x))
#define BIT_SIZE_INVALID(p) (sizeof((p)->narrow))
#define BIT_ADDRESS_INVALID(p) (&(p)->narrow)

#define BIT_SMALL(p) ((p)->small_wide)
#define BIT_SMALL_SET(p,x) ((p)->small_wide = (x))
#define BIT_SMALL_COMMA_SIZE(p) (sizeof((0, (p)->small_wide)))
#define BIT_SMALL_ASSIGN_SIZE(p) (sizeof((p)->small_wide = 127))
#define BIT_SMALL_PRE_SIZE(p) (sizeof(++(p)->small_wide))
#define BIT_SMALL_POST_SIZE(p) (sizeof((p)->small_wide++))
#define BIT_SMALL_COMMA_NOT(p) (~(0, (p)->small_wide))
#define BIT_SMALL_ASSIGN_NOT(p) (~((p)->small_wide = 127))
#define BIT_SMALL_PRE_NOT(p) (~++(p)->small_wide)
#define BIT_SMALL_POST_NOT(p) (~(p)->small_wide++)
