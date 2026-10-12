/* These replacement lists require C23 grammar rather than caller operands. */
typedef unsigned int C23InspectionAnchor;
#define KEYWORD_TRUE() true
#define KEYWORD_FALSE() false
#define KEYWORD_NULLPTR() nullptr
#define KEYWORD_BOOL(x) ((bool)(x))
#define KEYWORD_ALIGNOF() alignof(unsigned long)
