/* Original macro definitions exercise target-owned C representations without system headers. */
typedef __SIZE_TYPE__ ProfileSize;
typedef __PTRDIFF_TYPE__ ProfileDiff;
typedef long long ProfileLongLongArray[2];
typedef long long ProfileLongLongNestedArray[2][2];
typedef double ProfileDoubleArray[2];
typedef double ProfileDoubleNestedArray[2][2];
typedef char (*ProfileCharCallback)(char value);
char profile_char_identity(char value);
ProfileCharCallback profile_char_callback(void);
typedef struct ProfileRecord {
    char character;
    long signed_long;
    ProfileSize count;
} ProfileRecord;
#define PROFILE_CHAR(value) ((char) (value))
#define PROFILE_CHAR_ADD(value) (((char) (value)) + 1)
#define PROFILE_CHAR_CALL(value) (profile_char_identity((char) (value)))
#define PROFILE_CHAR_CALLBACK_GET() (profile_char_callback())
#define PROFILE_CHAR_CALLBACK_CALL(callback, value) (((ProfileCharCallback) (callback))((char) (value)))
#define PROFILE_LONG_UINT(value) (((long) (value)) + 1U)
#define PROFILE_ULONG_LLONG(value) (((unsigned long) (value)) + -1LL)
#define PROFILE_SIZE(value) ((ProfileSize) (value))
#define PROFILE_SIZE_OF(value) (sizeof(value))
#define PROFILE_ALIGNMENT(type_name) (_Alignof(type_name))
#define PROFILE_ALIGNMENT_LONG_LONG() (_Alignof(long long))
#define PROFILE_ALIGNMENT_LONG_LONG_ARRAY() (_Alignof(ProfileLongLongArray))
#define PROFILE_ALIGNMENT_LONG_LONG_NESTED_ARRAY() (_Alignof(ProfileLongLongNestedArray))
#define PROFILE_ALIGNMENT_DOUBLE() (_Alignof(double))
#define PROFILE_ALIGNMENT_DOUBLE_ARRAY() (_Alignof(ProfileDoubleArray))
#define PROFILE_ALIGNMENT_DOUBLE_NESTED_ARRAY() (_Alignof(ProfileDoubleNestedArray))
#define PROFILE_POINTER_DIFF(left, right) ((left) - (right))
#define PROFILE_CHAR_STORE(pointer, value) (((char *) (pointer))[0] = (char) (value))
#define PROFILE_LONG_STORE(pointer, value) (((long *) (pointer))[0] = (long) (value))
#define PROFILE_SIZE_STORE(pointer, value) (((ProfileSize *) (pointer))[0] = (ProfileSize) (value))
#define PROFILE_RECORD_CHAR(pointer) ((pointer)->character)
#define PROFILE_RECORD_CHAR_STORE(pointer, value) ((pointer)->character = (char) (value))
#define PROFILE_RECORD_LONG(pointer) ((pointer)->signed_long)
#define PROFILE_RECORD_LONG_STORE(pointer, value) ((pointer)->signed_long = (long) (value))
#define PROFILE_RECORD_SIZE(pointer) ((pointer)->count)
#define PROFILE_RECORD_SIZE_STORE(pointer, value) ((pointer)->count = (ProfileSize) (value))
