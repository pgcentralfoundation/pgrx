#if __INCLUDE_LEVEL__ == 0
#define EXP_CONTEXT_VALUE 1
typedef char ExpansionContextType;
#else
#define EXP_CONTEXT_VALUE 2
typedef int ExpansionContextType;
#endif
#define EXP_CONTEXT(x) ((x) + EXP_CONTEXT_VALUE)
#define EXP_CONTEXT_TYPE(x) ((ExpansionContextType)(x))
