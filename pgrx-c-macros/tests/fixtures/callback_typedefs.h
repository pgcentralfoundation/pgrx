typedef int (*ExplicitCallback)(int);
typedef long (*ExplicitLongCallback)(long);
typedef long long (*ExplicitWideCallback)(long long);

#define EXPLICIT_APPLY(callback,value) (((ExplicitCallback)(callback))(value))
#define EXPLICIT_TWICE(callback,value) (EXPLICIT_APPLY(callback,value) + EXPLICIT_APPLY(callback,value))
#define EXPLICIT_LAZY(callback,value,condition) ((condition) ? EXPLICIT_APPLY(callback,value) : 0)
#define EXPLICIT_PRESENT(callback) ((ExplicitCallback)(callback) != 0)
#define EXPLICIT_LONG(callback,value) (((ExplicitLongCallback)(callback))(value))
#define EXPLICIT_WIDE(callback,value) (((ExplicitWideCallback)(callback))(value))
