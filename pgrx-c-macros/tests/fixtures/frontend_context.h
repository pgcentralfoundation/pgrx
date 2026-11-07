#ifndef PGRX_FRONTEND_CONTEXT_H
#define PGRX_FRONTEND_CONTEXT_H

#define FRONT_EXTERNAL_STEP(value) ((value) + FRONT_EXTERNAL_VALUE)
#define FRONT_EXTERNAL_VALUE 4
typedef unsigned char FrontByte;
typedef unsigned short FrontWord;
typedef _Bool FrontBool;
enum FrontEnum { FRONT_ENUM_ZERO = 0, FRONT_ENUM_SEVEN = 7 };
struct FrontRecord { int value; };
extern volatile unsigned int front_volatile_counter;
extern const int front_const_variable;
extern int *front_pointer;

#endif
