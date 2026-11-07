/* Compiler-owned expansion fixtures; expected behavior comes from C preprocessing. */
#ifndef PGRX_EXPANSION_FIXTURE_H
#define PGRX_EXPANSION_FIXTURE_H

#define EXP_ID(x) ((x))
#define EXP_ADD_HELPER(a, b) ((a) + (b))
#define EXP_NESTED(x) EXP_ADD_HELPER(EXP_ADD_HELPER(x, 1), 2)
#define EXP_LATE_VALUE 1
#define EXP_LATE(x) ((x) + EXP_LATE_VALUE)
#undef EXP_LATE_VALUE
#define EXP_LATE_VALUE 9
#define EXP_DUPLICATE(x) ((x) + (x))
#define EXP_PRESCAN(x) EXP_DUPLICATE(EXP_DUPLICATE(x))
#define EXP_FUNCTION_ALIAS EXP_ID
#define EXP_RESCAN(x) EXP_FUNCTION_ALIAS(x)
#define EXP_RECURSIVE(x) ((x) + EXP_RECURSIVE(x))
#define EXP_CYCLE_A(x) EXP_CYCLE_B(x)
#define EXP_CYCLE_B(x) EXP_CYCLE_A(x)
#define EXP_DROP_HELPER(x) (7)
#define EXP_UNUSED(x) EXP_DROP_HELPER(x)
#define EXP_RAW_HELPER(x) x + 1
#define EXP_RAW(x) EXP_RAW_HELPER(x)
#define EXP_OUTER_GROUP(x) (EXP_RAW_HELPER(x))

#define EXP_STRING_HELPER(x) #x
#define EXP_STRING(x) EXP_STRING_HELPER(x)
#define EXP_PASTE_HELPER(x) x ## suffix
#define EXP_PASTE(x) EXP_PASTE_HELPER(x)
#define EXP_DYNAMIC_HELPER(x) ((x) + __LINE__)
#define EXP_DYNAMIC(x) EXP_DYNAMIC_HELPER(x)
#define EXP_COUNTER(x) ((x) + __COUNTER__)
#define EXP_DATE(x) __DATE__
#define EXP_PRAGMA(x) _Pragma("GCC diagnostic push") (x)
#define EXP_HAS_INCLUDE(x) __has_include(x)
#define EXP_TARGET_QUERY(x) __is_target_arch(x)
#define EXP_MODULE_QUERY(x) __building_module(x)
#define EXP_UNKNOWN_QUERY(x) __unknown_preprocessor_query(x)
#define EXP_VARIADIC_HELPER(x, ...) (x)
#define EXP_VARIADIC(x) EXP_VARIADIC_HELPER(x)
#define EXP_AMBIG_HELPER(x) ((x) + 1)
#define EXP_AMBIG_HELPER(x) ((x) + 1)
#define EXP_AMBIG(x) EXP_AMBIG_HELPER(x)
#define EXP_BAD_ARITY(x) EXP_ADD_HELPER(x)

#define EXP_TEN_HELPER(x) ((x)+(x)+(x)+(x)+(x)+(x)+(x)+(x)+(x)+(x))
#define EXP_BIG(x) EXP_TEN_HELPER(x)

/* A hostile existing namespace must never turn a probe argument into the value 77. */
#define __pgrx_c_expand_0_parameter_0_0 77
#define EXP_HOSTILE(__pgrx_c_expand_1_parameter_0_0) ((__pgrx_c_expand_1_parameter_0_0))

enum { captured = 7 };
#define EXP_CAPTURE_BASE captured
#define EXP_CAPTURE(x, captured) ((x) + EXP_CAPTURE_BASE)
#define EXP_SNAPSHOT_COMMENT(x) ((x) /*__pgrx_c_snapshot_end_0__*/ + 1)
#define EXP_SNAPSHOT_STRING(x) ("/*__pgrx_c_snapshot_end_0__*/ \\\\")

#endif
