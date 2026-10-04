#include "field_adapters.h"

#define ADD_COUNT(p, n) ((p)->count += (n))
#define PRE_COUNT(p) (++(p)->count)
#define POST_COUNT(p) ((p)->count++)
#define DOT_COUNT(object) ((object).count)
#define DEREF_COUNT(p) ((*(p)).count)
#define SET_CHILD(p, n) ((p)->child.value = (n))
#define SET_ARRAY(p, n) ((p)->array[1] = (n))
#define READ_ARRAY(p) ((p)->array[1])
#define SIZE_ARRAY(p) (sizeof((p)->array))
#define ADDRESS_COUNT(p) (&(p)->count)
