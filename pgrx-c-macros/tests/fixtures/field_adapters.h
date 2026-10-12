struct Child
{
    unsigned int value;
    const unsigned int frozen;
};

struct Outer
{
    struct Child child;
    struct Child *next;
    unsigned int count;
    _Bool other;
};

struct __attribute__((packed)) Packed
{
    char lead;
    unsigned int count;
    volatile unsigned char signal;
};

struct Volatile
{
    volatile unsigned int signal;
};

union Value
{
    unsigned int count;
    int signed_value;
};

struct Limitations
{
    unsigned int bits : 3;
    unsigned int array[2];
};

struct __attribute__((packed)) VolatilePacked
{
    char lead;
    volatile unsigned int signal;
};

#define READ_COUNT(p) ((p)->count)
#define SET_COUNT(p, n) ((p)->count = (n))
#define READ_CHILD(p) ((p)->child.value)
#define READ_NEXT(p) ((p)->next->value)
#define READ_SIGNAL(p) ((p)->signal)
