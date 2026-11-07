/* Original C fixture for native plain-char identity and conversion oracles. */
#define PLAIN_CHAR(x) ((char)(x))
#define PLAIN_PROMOTE(x) (+(char)(x))
#define PLAIN_WIDEN(x) ((unsigned long long)(char)(x))
