//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef __UINTPTR_TYPE__ Datum;
typedef unsigned int Oid;
typedef unsigned int TransactionId;
typedef enum ExprDoneCond {
    ExprSingleResult,
    ExprMultipleResult,
    ExprEndResult
} ExprDoneCond;
typedef struct ReturnSetInfo { ExprDoneCond isDone; } ReturnSetInfo;
typedef struct FunctionCallInfoData {
    _Bool isnull;
    void *resultinfo;
} FunctionCallInfoData;
typedef FunctionCallInfoData *FunctionCallInfo;
typedef struct FuncCallContext { unsigned long call_cntr; } FuncCallContext;

extern unsigned int return_trace;
extern unsigned int return_evaluations;
Datum return_record(unsigned int digit, Datum value);
FunctionCallInfo return_fcinfo(FunctionCallInfo value);
FuncCallContext *return_context(FuncCallContext *value);
void return_end(FunctionCallInfo fcinfo, FuncCallContext *context);

#define RET_VALUE(value) return (value)
#define RET_ALIAS(value) RET_VALUE(value)
#define RET_BYTE(value) return ((unsigned char) (value))
#define RET_DATUM(value) return ((Datum) (value))
#define RET_ZERO() return 0
#define RET_UNUSED(value) return ((Datum) 0)
#define RET_SEQUENCE(value) return ((value), (value))
#define RET_LOCAL(value) do { unsigned int local = (value); return local; } while (0)
#define RET_LOCAL_WRAPPER(value) RET_LOCAL(value)
#define RET_LOCAL_ALIAS(value) \
    do { unsigned int local = (value); unsigned int *alias = &local; *alias += 1; return local; } while (0)
#define RET_SELF() do { void *slot = &slot; return slot == &slot; } while (0)
#define RET_NULL() do { fcinfo->isnull = 1; return (Datum) 0; } while (0)
#define RET_SRF_NEXT(context, result) \
    do { \
        ReturnSetInfo *rsi; \
        (context)->call_cntr++; \
        rsi = (ReturnSetInfo *) fcinfo->resultinfo; \
        rsi->isDone = ExprMultipleResult; \
        RET_DATUM(result); \
    } while (0)
#define RET_SRF_NULL(context) \
    do { \
        ReturnSetInfo *rsi; \
        (context)->call_cntr++; \
        rsi = (ReturnSetInfo *) fcinfo->resultinfo; \
        rsi->isDone = ExprMultipleResult; \
        RET_NULL(); \
    } while (0)
#define RET_SRF_DONE(context) \
    do { \
        ReturnSetInfo *rsi; \
        return_end(fcinfo, context); \
        rsi = (ReturnSetInfo *) fcinfo->resultinfo; \
        rsi->isDone = ExprEndResult; \
        RET_NULL(); \
    } while (0)
