//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

typedef struct ParameterRecord { unsigned int state; unsigned char byte; } ParameterRecord;
extern unsigned int parameter_evaluations;
unsigned int parameter_record(unsigned int value);

#define NAME_PRIVS(privs) ((privs) & 0x7U)
#define NAME_PAIR(first, second) ((first) + (second))
#define NAME_REPEAT(privs) ((privs), (privs))
#define NAME_UNUSED(privs, unused) ((privs) + 1)
#define NAME_CAST(type, privs) ((type) (privs))
#define NAME_FIELD(pointer, member) ((pointer)->member)
#define NAME_KEYWORDS(type, match) ((type) + (match))
#define NAME_RUST_SPECIAL(self, Self, super, _) ((self) + (Self) + (super) + (_))
#define NAME_SCAFFOLD(state, raw, done, mode, __pgrx_c_return) \
    ((state) + (raw) + (done) + (mode) + (__pgrx_c_return))
#define NAME_RETURN(state, raw, done, mode, __pgrx_c_return) \
    do { return (state) + (raw) + (done) + (mode) + (__pgrx_c_return); } while (0)
#define NAME_RETURN_CHAIN(__pgrx_c_return, __pgrx_c_return_, __pgrx_c_return__, __pgrx_c_return___) \
    do { return (__pgrx_c_return) + (__pgrx_c_return_) + (__pgrx_c_return__) + (__pgrx_c_return___); } while (0)
#define NAME_CALL(privs) NAME_PRIVS((privs))
#define NAME_CAPTURE(privs) ((privs) + context)
#define NAME_INNER() fcinfo
#define NAME_FORMAL_CAPTURE(fcinfo) (NAME_INNER() + (fcinfo))
#define NAME_INNER_CRATE() crate
#define NAME_CRATE_CAPTURE(privs) (NAME_INNER_CRATE() + (privs))
#define NAME_RESERVED(crate) ((crate) + 1)
