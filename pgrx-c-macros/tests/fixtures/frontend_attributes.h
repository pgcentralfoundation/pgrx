/* Canonical integers must not erase under-alignment on pointer/array aliases. */
typedef unsigned int __attribute__((aligned(1))) FrontUnaligned;
typedef FrontUnaligned FrontUnalignedAlias;
typedef FrontUnalignedAlias FrontUnalignedArray[2];
struct FrontAttributedPointers {
    unsigned int *plain;
    FrontUnalignedAlias *len;
    FrontUnalignedArray *array;
};
#define FRONT_ATTRIBUTED_LOAD(m) (*(m)->len)
#define FRONT_ATTRIBUTED_ARRAY(m) (*(m)->array)[0]
