typedef unsigned int (*WitnessCallback)(unsigned int);

typedef struct WitnessPlain
{
    unsigned int scalar;
    unsigned int *pointer;
    unsigned int array[2];
    WitnessCallback callback;
    const unsigned int frozen;
} WitnessPlain;

typedef struct WitnessPromoted
{
    struct
    {
        const unsigned int promoted;
    };
} WitnessPromoted;

typedef struct WitnessPayload
{
    unsigned int value;
} WitnessPayload;

typedef struct WitnessAncestor
{
    const WitnessPayload const_payload;
} WitnessAncestor;

typedef union WitnessWrapped
{
    WitnessPayload payload;
} WitnessWrapped;

#define WITNESS_SCALAR(p) (((WitnessPlain *)(p))->scalar)
#define WITNESS_POINTER(p) (((WitnessPlain *)(p))->pointer)
#define WITNESS_ARRAY(p) (((WitnessPlain *)(p))->array)
#define WITNESS_CALLBACK(p) (((WitnessPlain *)(p))->callback)
#define WITNESS_FROZEN(p) (((WitnessPlain *)(p))->frozen)
#define WITNESS_PROMOTED(p) (((WitnessPromoted *)(p))->promoted)
#define WITNESS_WRAPPED(p) (((WitnessWrapped *)(p))->payload.value)
#define WITNESS_ANCESTOR(p) (((WitnessAncestor *)(p))->const_payload.value)
