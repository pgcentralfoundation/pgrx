//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare branch-prediction hint semantics and LLVM branch directions with C.
//!
//! Runtime observations check C long identity and both operand evaluations.
//! Separate code-generation witnesses inspect successor paths and branch weights,
//! including a cross-crate consumer, so successful values alone cannot hide a lost
//! or reversed hint.
//!
//! Generated consumers use the runtime's Linux/macOS host family and still validate
//! the inspected C ABI. Frontend, parser, and pre-emission rejection checks remain portable.

/// Reuse the binding build's collector so fixture tests reconcile exactly the Rust facts used
/// in production generation.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
mod rust_oracle;

#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, EvaluationRequirement, FrontendOutput, MacroScanner,
    generate_with_bindings, inspect,
};
use std::collections::{BTreeMap, BTreeSet};
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
use std::fs;
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
use std::path::{Path, PathBuf};
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
use std::process::Command;
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
use std::sync::Mutex;
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
use std::time::{SystemTime, UNIX_EPOCH};

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Classify PostgreSQL OID constants so fixture bindgen uses the same checked-wrapper boundary
/// as the real binding build.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Selected fixture macro names; explicit selection also exercises demand-driven adapter
/// generation.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
const NAMES: &[&str] = &[
    "EXPECT_RAW",
    "EXPECT_TYPE_CAST",
    "EXPECT_TYPE_EFFECT",
    "EXPECT_TYPE_DYNAMIC",
    "EXPECT_TYPE_SIZE",
    "EXPECT_TYPE_ALIGN",
    "EXPECT_TYPE_POINTER_SIZE",
    "EXPECT_VALUE_SIZE",
    "EXPECT_SOURCE_SIZE",
    "EXPECT_OFFSET_TYPE",
    "EXPECT_OFFSET_FIELD",
    "EXPECT_OFFSET_BOTH",
    "EXPECT_GROUPED",
    "EXPECT_NESTED",
    "EXPECT_NESTED_SEVEN",
    "EXPECT_RAW_SEVEN",
    "EXPECT_DELEGATED_SEVEN",
    "EXPECT_DELEGATED_NESTED",
    "EXPECT_NESTED_LONG_LONG",
    "EXPECT_NESTED_UNSIGNED_LONG",
    "EXPECT_NESTED_INT128",
    "EXPECT_NESTED_INT",
    "EXPECT_NESTED_BOOL",
    "EXPECT_NESTED_DYNAMIC",
    "EXPECT_EFFECT_ZERO",
    "EXPECT_LIKELY",
    "EXPECT_UNLIKELY",
    "EXPECT_INTERRUPTS",
    "EXPECT_RECORD",
    "EXPECT_VOLATILE",
    "EXPECT_SIZE",
    "EXPECT_LAZY",
    "EXPECT_AND",
    "EXPECT_LAZY_ZERO",
    "EXPECT_AND_ONE",
    "EXPECT_REPEAT",
    "EXPECT_UNUSED",
    "EXPECT_PROFILE",
    "EXPECT_ZERO",
    "EXPECT_NULL_SELECT",
    "EXPECT_NULL_CALL",
    "EXPECT_RUNTIME_NULL",
    "EXPECT_IMPURE_NULL",
];
/// Fixture candidates deliberately outside the supported contract; each must retain an
/// explained skip.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
const REJECTED: &[&str] = &[
    "EXPECT_WRONG0",
    "EXPECT_WRONG1",
    "EXPECT_WRONG3",
    "EXPECT_SYMBOL",
    "EXPECT_ADDRESS",
    "EXPECT_DEREF",
    "EXPECT_CALLBACK",
];
/// Fixture binding or native-support source paired with the unchanged C oracle.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
const NATIVE: &str = r#"
unsigned int expect_value_calls;
unsigned int expect_hint_calls;
unsigned int expect_trace;
volatile long expect_signal;
volatile long expect_hint_signal;
volatile int expect_interrupt_flag;
unsigned int expect_interrupt_calls;
int expect_interrupt_clear;
long expect_record_value(long value) { expect_value_calls++; expect_trace=expect_trace*10+1; return value; }
long expect_record_hint(long hint) { expect_hint_calls++; expect_trace=expect_trace*10+2; return hint; }
long expect_use_callback(ExpectCallback callback) { return callback(17,1); }
int *expect_take_pointer(int *pointer) { return pointer; }
void expect_process_interrupts(void) {
    expect_interrupt_calls++;
    if (expect_interrupt_clear) expect_interrupt_flag=0;
}
"#;
/// Original C recorder source whose header invocations establish expected semantic
/// observations.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
const ORIGINAL: &str = r#"
#include <stdio.h>
#include <limits.h>
#define C_RANK(value) _Generic((value), int: 3, long: 4, long long: 5, default: 0)
_Static_assert(_Generic(EXPECT_NULL_SELECT(1,(int *)0), int *: 1, default: 0), "closed expect zero is a null pointer constant");
static void observations(long result) {
    printf("%ld %u %u %u\n",result,expect_value_calls,expect_hint_calls,expect_trace);
    expect_value_calls=0; expect_hint_calls=0; expect_trace=0;
}
int main(void) {
    long values[]={-2147483647L,-2L,-1L,0L,1L,7L,2147483647L};
    for(unsigned int i=0;i<7;i++) {
        long value=values[i];
        printf("%ld %ld %ld %ld %ld %ld %ld %ld %ld %ld %ld\n",EXPECT_RAW(value,-9),EXPECT_GROUPED(value,1),EXPECT_NESTED(value),EXPECT_LIKELY(value),EXPECT_UNLIKELY(value),EXPECT_RAW_SEVEN(value),EXPECT_NESTED_SEVEN(value),(long)EXPECT_AND_ONE(1,value),EXPECT_LAZY_ZERO(1,value),EXPECT_DELEGATED_SEVEN(value),EXPECT_DELEGATED_NESTED(value));
    }
    long cast_values[]={0L,7L,-1L,0x100000000L,LONG_MIN,LONG_MAX};
    for(unsigned int i=0;i<6;i++) {
        long value=cast_values[i];
        printf("%ld %ld %ld %ld %ld %ld\n",EXPECT_NESTED_LONG_LONG(value),EXPECT_NESTED_UNSIGNED_LONG(value),EXPECT_NESTED_INT128(value),EXPECT_NESTED_INT(value),EXPECT_NESTED_BOOL(value),EXPECT_NESTED_DYNAMIC(value,value!=0?17L:-3L));
    }
    double floats[]={-123.75,-0.0,0.0,0.75,1.75,123.75};
    for(unsigned int i=0;i<6;i++) printf("%ld %ld\n",EXPECT_RAW(floats[i],-3.75),EXPECT_RAW((float)floats[i],2.5f));
    unsigned long wide[]={0UL,1UL,0x8000000000000000UL,0xFFFFFFFFFFFFFFFFUL};
    for(unsigned int i=0;i<4;i++) printf("%ld %ld\n",EXPECT_RAW(wide[i],0),EXPECT_RAW((unsigned long long)wide[i],0));
    printf("%d %zu %d %zu\n",C_RANK(EXPECT_RAW(0,1)),sizeof(EXPECT_RAW(0,1)),C_RANK(EXPECT_LIKELY(7)),sizeof(EXPECT_LIKELY(7)));
    observations(EXPECT_RECORD(7,-3));
    observations(EXPECT_EFFECT_ZERO(expect_record_value(7)));
    observations(EXPECT_NESTED_DYNAMIC(expect_record_value(0x100000000L),expect_record_hint(29)));
    (void)EXPECT_RAW(expect_record_value(11),expect_record_hint(29)); observations(0);
    observations(EXPECT_REPEAT(expect_record_value(3),expect_record_hint(1)));
    observations(EXPECT_TYPE_CAST(unsigned char,expect_record_value(7)));
    observations(EXPECT_TYPE_CAST(unsigned short,expect_record_value(7)));
    observations(EXPECT_TYPE_CAST(int,expect_record_value(7)));
    observations(EXPECT_TYPE_EFFECT(unsigned short,expect_record_value(7)));
    observations(EXPECT_TYPE_DYNAMIC(unsigned char,expect_record_value(7),expect_record_hint(257)));
    observations(EXPECT_TYPE_SIZE(ExpectRecord,expect_record_value(7)));
    observations(EXPECT_TYPE_ALIGN(ExpectRecord,expect_record_value(7)));
    observations(EXPECT_TYPE_POINTER_SIZE(ExpectRecord,expect_record_value(7)));
    observations(EXPECT_VALUE_SIZE(expect_record_value(7)));
    observations(EXPECT_SOURCE_SIZE(expect_record_value(7),expect_record_hint(19)));
    observations(EXPECT_OFFSET_TYPE(ExpectRecord,expect_record_value(7)));
    observations(EXPECT_OFFSET_FIELD(expect_record_value(7),lead));
    observations(EXPECT_OFFSET_BOTH(ExpectRecord,value,expect_record_value(7)));
    observations(EXPECT_OFFSET_BOTH(ExpectRecord,lead,expect_record_value(7)));
    long first=41, __pgrx_c_expect_result=73;
    printf("%ld %ld\n",EXPECT_RAW(first,__pgrx_c_expect_result),EXPECT_RAW(__pgrx_c_expect_result,first));
    expect_signal=-17; expect_hint_signal=2;
    printf("%ld %ld %ld\n",EXPECT_VOLATILE(),expect_signal,expect_hint_signal);
    unsigned long size=EXPECT_SIZE(expect_record_value(17),expect_record_hint(1));
    printf("%lu %u %u\n",size,expect_value_calls,expect_hint_calls);
    observations(EXPECT_UNUSED(expect_record_value(31)));
    observations(EXPECT_LAZY(0,expect_record_value(13),expect_record_hint(1)));
    observations(EXPECT_AND(0,expect_record_value(13),expect_record_hint(1)));
    observations(EXPECT_LAZY(1,expect_record_value(13),expect_record_hint(1)));
    observations(EXPECT_AND(1,expect_record_value(13),expect_record_hint(1)));
    printf("%ld %ld\n",EXPECT_PROFILE(7),EXPECT_ZERO());
    int object=31; int *pointer=&object;
    printf("%d %d %d\n",EXPECT_NULL_SELECT(1,pointer)==0,EXPECT_NULL_SELECT(0,pointer)==pointer,EXPECT_NULL_CALL()==0);
    int flags[]={0,1,-1,INT_MIN,INT_MAX};
    for(int clear=0;clear<2;clear++) {
        for(unsigned int i=0;i<5;i++) {
            expect_interrupt_flag=flags[i]; expect_interrupt_calls=0; expect_interrupt_clear=clear;
            for(unsigned int invocation=0;invocation<3;invocation++) {
                EXPECT_INTERRUPTS();
                printf("interrupt %d %u\n",expect_interrupt_flag,expect_interrupt_calls);
            }
        }
    }
    for(int condition=0;condition<2;condition++) {
        expect_interrupt_flag=1; expect_interrupt_calls=0; expect_interrupt_clear=0;
        int else_taken=0;
        if(condition)
            EXPECT_INTERRUPTS();
        else
            else_taken=1;
        printf("interrupt-scope %d %u %d\n",expect_interrupt_flag,expect_interrupt_calls,else_taken);
    }
}
"#;
/// Rust consumer source exercising actual generated macros and adapters.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
const CONSUMER: &str = r#"
fn rank<K: __pgrx_c_macros::CInteger>(_: __pgrx_c_macros::CValue<K>) -> u8 { K::RANK }
fn long(value:i64) -> __pgrx_c_macros::CValue<__pgrx_c_macros::CLong> { __pgrx_c_macros::CValue::new(value) }
fn observations(result:i64) {
    // SAFETY: these initialized scalar globals belong to this single-threaded
    // standalone C library. No references or concurrent accesses exist.
    unsafe {
        let value_calls=expect_value_calls; let hint_calls=expect_hint_calls; let trace=expect_trace;
        println!("{} {} {} {}",result,value_calls,hint_calls,trace);
        expect_value_calls=0; expect_hint_calls=0; expect_trace=0;
    }
}
fn main() {
    for value in [-2147483647_i64,-2,-1,0,1,7,2147483647] {
        let value=long(value);
        println!("{} {} {} {} {} {} {} {} {} {} {}",EXPECT_RAW!(value,-9).get(),EXPECT_GROUPED!(value,1).get(),EXPECT_NESTED!(value).get(),EXPECT_LIKELY!(value).get(),EXPECT_UNLIKELY!(value).get(),EXPECT_RAW_SEVEN!(value).get(),EXPECT_NESTED_SEVEN!(value).get(),EXPECT_AND_ONE!(1,value).get(),EXPECT_LAZY_ZERO!(1,value).get(),EXPECT_DELEGATED_SEVEN!(value).get(),EXPECT_DELEGATED_NESTED!(value).get());
    }
    for value in [0_i64,7,-1,0x100000000,i64::MIN,i64::MAX] {
        let value=long(value);
        println!("{} {} {} {} {} {}",EXPECT_NESTED_LONG_LONG!(value).get(),EXPECT_NESTED_UNSIGNED_LONG!(value).get(),EXPECT_NESTED_INT128!(value).get(),EXPECT_NESTED_INT!(value).get(),EXPECT_NESTED_BOOL!(value).get(),EXPECT_NESTED_DYNAMIC!(value,long(if value.get()!=0 {17} else {-3})).get());
    }
    for value in [-123.75_f64,-0.0,0.0,0.75,1.75,123.75] {
        println!("{} {}",EXPECT_RAW!(value,-3.75_f64).get(),EXPECT_RAW!(value as f32,2.5_f32).get());
    }
    for value in [0_u64,1,0x8000000000000000,u64::MAX] {
        let ulong=__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLong>::new(value);
        let ull=__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLongLong>::new(value);
        println!("{} {}",EXPECT_RAW!(ulong,0).get(),EXPECT_RAW!(ull,0).get());
    }
    let raw=EXPECT_RAW!(0,1); let likely=EXPECT_LIKELY!(7);
    println!("{} {} {} {}",rank(raw.into_value()),core::mem::size_of_val(&raw.get()),rank(likely.into_value()),core::mem::size_of_val(&likely.get()));
    // SAFETY: native callbacks access only initialized scalar state owned by
    // this single-threaded executable. Their complete C function executions
    // are sequenced, so tracing their order does not introduce unsequenced C
    // writes. Finite float inputs stay inside C long's representable range.
    // Pointer helpers only forward a live local address or a null pointer;
    // neither helper nor macro dereferences those pointers.
    unsafe {
        observations(EXPECT_RECORD!(long(7),long(-3)).get());
        observations(EXPECT_EFFECT_ZERO!(long(expect_record_value(7))).get());
        observations(EXPECT_NESTED_DYNAMIC!(long(expect_record_value(0x100000000)),long(expect_record_hint(29))).get());
        EXPECT_RAW!(@__pgrx_c_discard; long(expect_record_value(11)),long(expect_record_hint(29))); observations(0);
        observations(EXPECT_REPEAT!(long(expect_record_value(3)),long(expect_record_hint(1))).get());
        observations(EXPECT_TYPE_CAST!(u8,long(expect_record_value(7))).get());
        observations(EXPECT_TYPE_CAST!(u16,long(expect_record_value(7))).get());
        observations(EXPECT_TYPE_CAST!(i32,long(expect_record_value(7))).get());
        observations(EXPECT_TYPE_EFFECT!(u16,long(expect_record_value(7))).get());
        observations(EXPECT_TYPE_DYNAMIC!(u8,long(expect_record_value(7)),long(expect_record_hint(257))).get());
        observations(EXPECT_TYPE_SIZE!(ExpectRecord,long(expect_record_value(7))).get());
        observations(EXPECT_TYPE_ALIGN!(ExpectRecord,long(expect_record_value(7))).get());
        observations(EXPECT_TYPE_POINTER_SIZE!(ExpectRecord,long(expect_record_value(7))).get());
        observations(EXPECT_VALUE_SIZE!(long(expect_record_value(7))).get());
        observations(EXPECT_SOURCE_SIZE!(long(expect_record_value(7)),long(expect_record_hint(19))).get());
        observations(EXPECT_OFFSET_TYPE!(ExpectRecord,long(expect_record_value(7))).get());
        observations(EXPECT_OFFSET_FIELD!(long(expect_record_value(7)),lead).get());
        observations(EXPECT_OFFSET_BOTH!(ExpectRecord,value,long(expect_record_value(7))).get());
        observations(EXPECT_OFFSET_BOTH!(ExpectRecord,lead,long(expect_record_value(7))).get());
        let first=long(41); let __pgrx_c_expect_result=long(73);
        println!("{} {}",EXPECT_RAW!(first,__pgrx_c_expect_result).get(),EXPECT_RAW!(__pgrx_c_expect_result,first).get());
        core::ptr::write_volatile(core::ptr::addr_of_mut!(expect_signal),-17);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(expect_hint_signal),2);
        let value=EXPECT_VOLATILE!().get();
        let signal=core::ptr::read_volatile(core::ptr::addr_of!(expect_signal));
        let hint=core::ptr::read_volatile(core::ptr::addr_of!(expect_hint_signal));
        println!("{} {} {}",value,signal,hint);
        let size=EXPECT_SIZE!(@__pgrx_c_expression; long(expect_record_value(17)),long(expect_record_hint(1))).get();
        let value_calls=expect_value_calls; let hint_calls=expect_hint_calls;
        println!("{} {} {}",size,value_calls,hint_calls);
        assert_eq!((value_calls,hint_calls),(0,0),"sizeof must suppress both operands");
        observations(EXPECT_UNUSED!(expect_record_value(31)).get());
        observations(EXPECT_LAZY!(0,long(expect_record_value(13)),long(expect_record_hint(1))).get());
        observations(EXPECT_AND!(0,long(expect_record_value(13)),long(expect_record_hint(1))).get().into());
        observations(EXPECT_LAZY!(1,long(expect_record_value(13)),long(expect_record_hint(1))).get());
        observations(EXPECT_AND!(1,long(expect_record_value(13)),long(expect_record_hint(1))).get().into());
        println!("{} {}",EXPECT_PROFILE!(7).get(),EXPECT_ZERO!().get());
        let mut object=31_i32; let pointer=&raw mut object;
        let absent:*mut i32=EXPECT_NULL_SELECT!(1,pointer).get();
        let present:*mut i32=EXPECT_NULL_SELECT!(0,pointer).get();
        let called:*mut i32=EXPECT_NULL_CALL!().get();
        println!("{} {} {}",u8::from(absent.is_null()),u8::from(present==pointer),u8::from(called.is_null()));
    }
    // SAFETY: These C-defined scalar globals are initialized, correctly aligned,
    // and accessed only by this single-threaded executable, without Rust
    // references. The volatile flag uses raw volatile reads and writes. The
    // void stub only increments its bounded counter and optionally clears that
    // flag; it cannot raise a PostgreSQL error, unwind, or retain any pointers.
    unsafe {
        for clear in [0_i32,1] {
            for initial_flag in [0_i32,1,-1,i32::MIN,i32::MAX] {
                core::ptr::write_volatile(core::ptr::addr_of_mut!(expect_interrupt_flag),initial_flag);
                expect_interrupt_calls=0; expect_interrupt_clear=clear;
                for invocation in 0..3_u32 {
                    let _: () = EXPECT_INTERRUPTS!();
                    let flag=core::ptr::read_volatile(core::ptr::addr_of!(expect_interrupt_flag));
                    let calls=expect_interrupt_calls;
                    let expected_calls=if initial_flag==0 { 0 } else if clear!=0 { 1 } else { invocation+1 };
                    assert_eq!(calls,expected_calls,"one void call per nonzero invocation, even while the flag stays set");
                    assert_eq!(flag,if clear!=0 {0} else {initial_flag});
                    println!("interrupt {} {}",flag,calls);
                }
            }
        }
        for condition in [0_i32,1] {
            core::ptr::write_volatile(core::ptr::addr_of_mut!(expect_interrupt_flag),1);
            expect_interrupt_calls=0; expect_interrupt_clear=0;
            let mut else_taken=0;
            if condition!=0 {
                EXPECT_INTERRUPTS!();
            } else {
                else_taken=1;
            }
            let flag=core::ptr::read_volatile(core::ptr::addr_of!(expect_interrupt_flag));
            let calls=expect_interrupt_calls;
            assert_eq!(calls,condition as u32,"an unexecuted outer branch must not call the stub");
            assert_eq!(else_taken,1-condition,"the outer else keeps its own scope");
            println!("interrupt-scope {} {} {}",flag,calls,else_taken);
        }
    }
}
"#;

// Different sink tags keep LLVM's function-merging pass from combining wrappers
// whose runtime behavior agrees but whose expectation directions are opposite.
/// Original C functions used as compiler witnesses for branch-prediction direction.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
const C_CODEGEN: &str = r#"
void expect_codegen_likely(int value) {
    if(EXPECT_LIKELY(value)) expect_branch_yes(1); else expect_branch_no(1);
}
void expect_codegen_unlikely(int value) {
    if(EXPECT_UNLIKELY(value)) expect_branch_yes(2); else expect_branch_no(2);
}
void expect_codegen_raw(long value) {
    if(EXPECT_RAW_SEVEN(value)==7) expect_branch_yes(3); else expect_branch_no(3);
}
void expect_codegen_negated(int value) {
    if(!EXPECT_LIKELY(value)) expect_branch_yes(4); else expect_branch_no(4);
}
void expect_codegen_interrupts(void) {
    EXPECT_INTERRUPTS();
}
void expect_codegen_and(int guard) {
    if(EXPECT_AND_ONE(guard,expect_branch_gate())) expect_branch_yes(6); else expect_branch_no(6);
}
void expect_codegen_conditional(int guard) {
    if(EXPECT_LAZY_ZERO(guard,expect_branch_gate())) expect_branch_yes(7); else expect_branch_no(7);
}
void expect_codegen_nested(long value) {
    if(EXPECT_NESTED(value)==0) expect_branch_yes(8); else expect_branch_no(8);
}
void expect_codegen_nested_seven(long value) {
    if(EXPECT_NESTED_SEVEN(value)==0) expect_branch_yes(9); else expect_branch_no(9);
}
void expect_codegen_dynamic(long value, long hint) {
    if(EXPECT_RAW(value,hint)!=hint) expect_branch_yes(10); else expect_branch_no(10);
}
void expect_codegen_delegated(long value) {
    if(EXPECT_DELEGATED_SEVEN(value)==7) expect_branch_yes(11); else expect_branch_no(11);
}
void expect_codegen_delegated_nested(long value) {
    if(EXPECT_DELEGATED_NESTED(value)==0) expect_branch_yes(12); else expect_branch_no(12);
}
void expect_codegen_nested_long_long(long value) {
    if(EXPECT_NESTED_LONG_LONG(value)==0) expect_branch_yes(13); else expect_branch_no(13);
}
void expect_codegen_nested_unsigned_long(long value) {
    if(EXPECT_NESTED_UNSIGNED_LONG(value)==0) expect_branch_yes(14); else expect_branch_no(14);
}
void expect_codegen_nested_int128(long value) {
    if(EXPECT_NESTED_INT128(value)==0) expect_branch_yes(15); else expect_branch_no(15);
}
void expect_codegen_nested_int(long value) {
    if(EXPECT_NESTED_INT(value)==0) expect_branch_yes(16); else expect_branch_no(16);
}
void expect_codegen_effect(long value) {
    if(EXPECT_EFFECT_ZERO(value)==0) expect_branch_yes(17); else expect_branch_no(17);
}
void expect_codegen_nested_bool(long value) {
    if(EXPECT_NESTED_BOOL(value)==0) expect_branch_yes(18); else expect_branch_no(18);
}
void expect_codegen_nested_dynamic(long value, long hint) {
    if(EXPECT_NESTED_DYNAMIC(value,hint)==0) expect_branch_yes(19); else expect_branch_no(19);
}
void expect_codegen_typed_cast(long value) {
    if(EXPECT_TYPE_CAST(unsigned char,value)==1) expect_branch_yes(20); else expect_branch_no(20);
}
void expect_codegen_typed_effect(long value) {
    if(EXPECT_TYPE_EFFECT(unsigned short,value)==0) expect_branch_yes(21); else expect_branch_no(21);
}
void expect_codegen_typed_size(long value) {
    if(EXPECT_TYPE_SIZE(ExpectRecord,value)==sizeof(ExpectRecord)) expect_branch_yes(22); else expect_branch_no(22);
}
void expect_codegen_typed_align(long value) {
    if(EXPECT_TYPE_ALIGN(ExpectRecord,value)==_Alignof(ExpectRecord)) expect_branch_yes(23); else expect_branch_no(23);
}
void expect_codegen_typed_pointer_size(long value) {
    if(EXPECT_TYPE_POINTER_SIZE(ExpectRecord,value)==sizeof(ExpectRecord *)) expect_branch_yes(24); else expect_branch_no(24);
}
void expect_codegen_offset_type(long value) {
    if(EXPECT_OFFSET_TYPE(ExpectRecord,value)==__builtin_offsetof(ExpectRecord,value)) expect_branch_yes(25); else expect_branch_no(25);
}
void expect_codegen_offset_field(long value) {
    if(EXPECT_OFFSET_FIELD(value,lead)==0) expect_branch_yes(26); else expect_branch_no(26);
}
void expect_codegen_offset_both(long value) {
    if(EXPECT_OFFSET_BOTH(ExpectRecord,value,value)==__builtin_offsetof(ExpectRecord,value)) expect_branch_yes(27); else expect_branch_no(27);
}
void expect_codegen_typed_dynamic(long value, long hint) {
    if(EXPECT_TYPE_DYNAMIC(unsigned char,value,hint)!=(unsigned char)hint) expect_branch_yes(28); else expect_branch_no(28);
}
void expect_codegen_value_size(long value) {
    if(EXPECT_VALUE_SIZE(value)==sizeof(value)) expect_branch_yes(29); else expect_branch_no(29);
}
void expect_codegen_source_size(long value) {
    if(EXPECT_SOURCE_SIZE(value,expect_record_hint(19))==sizeof(long)) expect_branch_yes(30); else expect_branch_no(30);
}
"#;

/// Generated Rust consumer functions used to inspect retained LLVM branch weights.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
const RUST_CODEGEN: &str = r#"
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes and must
/// not unwind or perform PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_likely(value:core::ffi::c_int) {
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_LIKELY!(value).get()!=0 { expect_branch_yes(1); } else { expect_branch_no(1); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_unlikely(value:core::ffi::c_int) {
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_UNLIKELY!(value).get()!=0 { expect_branch_yes(2); } else { expect_branch_no(2); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_raw(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_RAW_SEVEN!(value).get()==7 { expect_branch_yes(3); } else { expect_branch_no(3); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_negated(value:core::ffi::c_int) {
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_LIKELY!(value).get()==0 { expect_branch_yes(4); } else { expect_branch_no(4); }
    }
}
/// # Safety
/// The linked volatile flag must be an initialized C int with no concurrent
/// writes; the void stub must not unwind or raise PostgreSQL errors. No Rust
/// references are formed. This IR witness is not linked or executed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_interrupts() {
    // SAFETY: the caller establishes initialized volatile storage and the stub's
    // scalar-only C contract. The generated macro uses a raw volatile load.
    unsafe { EXPECT_INTERRUPTS!(); }
}
/// # Safety
/// Linked sinks and gate must implement the fixture's scalar-only C prototypes
/// without unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_and(guard:core::ffi::c_int) {
    // SAFETY: the caller guarantees all opaque callees' scalar-only C contracts.
    unsafe {
        if EXPECT_AND_ONE!(guard,__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(expect_branch_gate())).get()!=0 {
            expect_branch_yes(6);
        } else {
            expect_branch_no(6);
        }
    }
}
/// # Safety
/// Linked sinks and gate must implement the fixture's scalar-only C prototypes
/// without unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_conditional(guard:core::ffi::c_int) {
    // SAFETY: the caller guarantees all opaque callees' scalar-only C contracts.
    unsafe {
        if EXPECT_LAZY_ZERO!(guard,__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(expect_branch_gate())).get()!=0 {
            expect_branch_yes(7);
        } else {
            expect_branch_no(7);
        }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_nested(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_NESTED!(value).get()==0 { expect_branch_yes(8); } else { expect_branch_no(8); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_nested_seven(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_NESTED_SEVEN!(value).get()==0 { expect_branch_yes(9); } else { expect_branch_no(9); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_dynamic(value:core::ffi::c_long,hint:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    let expected=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(hint);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_RAW!(value,expected).get()!=hint { expect_branch_yes(10); } else { expect_branch_no(10); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_delegated(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_DELEGATED_SEVEN!(value).get()==7 { expect_branch_yes(11); } else { expect_branch_no(11); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_delegated_nested(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_DELEGATED_NESTED!(value).get()==0 { expect_branch_yes(12); } else { expect_branch_no(12); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_nested_long_long(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_NESTED_LONG_LONG!(value).get()==0 { expect_branch_yes(13); } else { expect_branch_no(13); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_nested_unsigned_long(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_NESTED_UNSIGNED_LONG!(value).get()==0 { expect_branch_yes(14); } else { expect_branch_no(14); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_nested_int128(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_NESTED_INT128!(value).get()==0 { expect_branch_yes(15); } else { expect_branch_no(15); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_nested_int(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_NESTED_INT!(value).get()==0 { expect_branch_yes(16); } else { expect_branch_no(16); }
    }
}
/// # Safety
/// Linked sinks and the hint recorder must implement the fixture's scalar-only
/// C prototypes without unwinding or PostgreSQL nonlocal jumps. This witness
/// is not linked or executed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_effect(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the sinks' and hint recorder's C contracts.
    unsafe {
        if EXPECT_EFFECT_ZERO!(value).get()==0 { expect_branch_yes(17); } else { expect_branch_no(17); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_nested_bool(value:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_NESTED_BOOL!(value).get()==0 { expect_branch_yes(18); } else { expect_branch_no(18); }
    }
}
/// # Safety
/// Linked sinks must implement the fixture's C scalar-only prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_nested_dynamic(value:core::ffi::c_long,hint:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    let hint=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(hint);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_NESTED_DYNAMIC!(value,hint).get()==0 { expect_branch_yes(19); } else { expect_branch_no(19); }
    }
}
macro_rules! fixed_type_witness {
    ($name:ident, $value:ident, $expression:expr, $expected:expr, $tag:literal) => {
        /// # Safety
        /// Linked sinks and the hint recorder must implement the fixture's
        /// scalar-only C prototypes without unwinding or PostgreSQL jumps.
        /// This exported IR witness is not linked or called by the test.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(raw:core::ffi::c_long) {
            let $value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(raw);
            // SAFETY: the caller guarantees these scalar-only native contracts;
            // no pointer, allocation or PostgreSQL resource is involved.
            unsafe {
                if $expression.get()==$expected { expect_branch_yes($tag); } else { expect_branch_no($tag); }
            }
        }
    };
}
fixed_type_witness!(expect_codegen_typed_cast,value,EXPECT_TYPE_CAST!(u8,value),1,20);
fixed_type_witness!(expect_codegen_typed_effect,value,EXPECT_TYPE_EFFECT!(u16,value),0,21);
fixed_type_witness!(expect_codegen_typed_size,value,EXPECT_TYPE_SIZE!(ExpectRecord,value),core::mem::size_of::<ExpectRecord>() as core::ffi::c_long,22);
fixed_type_witness!(expect_codegen_typed_align,value,EXPECT_TYPE_ALIGN!(ExpectRecord,value),core::mem::align_of::<ExpectRecord>() as core::ffi::c_long,23);
fixed_type_witness!(expect_codegen_typed_pointer_size,value,EXPECT_TYPE_POINTER_SIZE!(ExpectRecord,value),core::mem::size_of::<*mut ExpectRecord>() as core::ffi::c_long,24);
fixed_type_witness!(expect_codegen_offset_type,value,EXPECT_OFFSET_TYPE!(ExpectRecord,value),core::mem::offset_of!(ExpectRecord,value) as core::ffi::c_long,25);
fixed_type_witness!(expect_codegen_offset_field,value,EXPECT_OFFSET_FIELD!(value,lead),0,26);
fixed_type_witness!(expect_codegen_offset_both,value,EXPECT_OFFSET_BOTH!(ExpectRecord,value,value),core::mem::offset_of!(ExpectRecord,value) as core::ffi::c_long,27);
fixed_type_witness!(expect_codegen_value_size,value,EXPECT_VALUE_SIZE!(value),core::mem::size_of::<core::ffi::c_long>() as core::ffi::c_long,29);
fixed_type_witness!(expect_codegen_source_size,value,EXPECT_SOURCE_SIZE!(value,__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(expect_record_hint(19))),core::mem::size_of::<core::ffi::c_long>() as core::ffi::c_long,30);
/// # Safety
/// Linked sinks must implement the fixture's scalar-only C prototypes without
/// unwinding or PostgreSQL nonlocal jumps. This IR witness is not linked.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expect_codegen_typed_dynamic(value:core::ffi::c_long,hint:core::ffi::c_long) {
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(value);
    let expected=__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(hint);
    // SAFETY: the caller guarantees the opaque sinks' scalar-only C contracts.
    unsafe {
        if EXPECT_TYPE_DYNAMIC!(u8,value,expected).get()!=hint as u8 as core::ffi::c_long { expect_branch_yes(28); } else { expect_branch_no(28); }
    }
}
"#;

/// Construct the C invocation used for both inspection and the native oracle, so
/// compiler-profile differences cannot explain a mismatch.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
fn arguments(optimization: &str, shadow: bool) -> Vec<String> {
    let mut arguments = vec!["-std=c17".into(), optimization.into(), "-ffp-contract=off".into()];
    if shadow {
        arguments.push("-DEXPECT_SHADOW=1".into());
    }
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "expect oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    arguments
}

/// Build the fixture binding catalog used to validate symbolic references and native adapters
/// against compiler facts.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
fn bindings(frontend: &FrontendOutput) -> String {
    bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(frontend.profile().header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Expect.*")
        .allowlist_function("expect_.*")
        .allowlist_var("expect_.*")
        .use_core()
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .unwrap()
        .to_string()
}

/// Assemble the Rust oracle prelude with real support and this fixture's generated bindings and
/// adapters.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
fn rust_base(directory: &Path, bindings: &str, support: &str) -> String {
    let runtime = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={runtime:?}] pub mod __pgrx_c_macros;\n{bindings}\n{support}"
    )
}

/// Checks that expectation builtins preserve long identity and both operand evaluations.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
#[test]
fn expectation_builtins_preserve_long_identity_and_both_operand_evaluations() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/expect_oracle.h");
    let scanner = MacroScanner::new().expect("libclang required");
    for optimization in ["-O0", "-O2"] {
        let frontend = inspect(&scanner, &header, &arguments(optimization, false), None).unwrap();
        let names = NAMES.iter().chain(REJECTED).copied().collect::<Vec<_>>();
        let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
        let bindings = bindings(&frontend);
        let catalog = binding_symbols::collect_bindings(
            &syn::parse_file(&bindings).unwrap(),
            session.integer_constants(),
            frontend.declarations(),
            &frontend.profile().target,
        );
        let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
        let mut rust = rust_base(&directory, &bindings, &generated.support.rust);
        assert_eq!(generated.macros.len(), NAMES.len() + REJECTED.len());
        for emission in &generated.macros {
            if REJECTED.contains(&emission.analysis.name.as_str()) {
                let EmissionStatus::Skipped { reason } = &emission.status else {
                    panic!("invalid expectation builtin use must be rejected: {emission:?}");
                };
                assert!(!reason.message.is_empty());
                continue;
            }
            let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
                panic!("proved expectation builtin must emit under {optimization}: {emission:?}");
            };
            if emission.analysis.name == "EXPECT_RAW" {
                assert!(
                    emission
                        .analysis
                        .evaluation
                        .requirements
                        .contains(&EvaluationRequirement::UnspecifiedOperandOrder)
                );
            }
            if matches!(emission.analysis.name.as_str(), "EXPECT_VOLATILE" | "EXPECT_INTERRUPTS") {
                assert!(definition.contains("CVolatile"));
            }
            if matches!(
                emission.analysis.name.as_str(),
                "EXPECT_TYPE_CAST"
                    | "EXPECT_TYPE_EFFECT"
                    | "EXPECT_TYPE_SIZE"
                    | "EXPECT_TYPE_ALIGN"
                    | "EXPECT_TYPE_POINTER_SIZE"
                    | "EXPECT_VALUE_SIZE"
                    | "EXPECT_SOURCE_SIZE"
                    | "EXPECT_OFFSET_TYPE"
                    | "EXPECT_OFFSET_FIELD"
                    | "EXPECT_OFFSET_BOTH"
            ) {
                assert!(definition.contains("::expression::expect("), "{definition}");
                assert!(!definition.contains("expected value is not established"), "{definition}");
            }
            if emission.analysis.name == "EXPECT_TYPE_DYNAMIC" {
                assert!(definition.contains("expected value is not established"), "{definition}");
                assert!(!definition.contains("::expression::expect("), "{definition}");
            }
            rust.push_str(definition);
        }
        let profile = frontend.profile();
        let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let native = format!("{NATIVE}\n{}", generated.support.c_source);
        let expected = oracle::run_c(
            &profile.compiler.executable,
            &header,
            &format!("{native}\n{ORIGINAL}"),
            &arguments,
            true,
        );
        let actual = rust_oracle::run_rust_linked(
            &format!("{rust}\n{CONSUMER}"),
            &profile.compiler.executable,
            &header,
            &native,
            &arguments,
        );
        assert_eq!(actual, expected, "expect semantics follow the exact {optimization} profile");
        assert_eq!(actual.lines().count(), 85);
        // C permits variably modified typedefs here, but the generated Rust
        // capability deliberately admits only statically sized native storage.
        // A VLA has no NativeType implementation and cannot acquire a false hint.
        assert!(
            oracle::run_c(
                &profile.compiler.executable,
                &header,
                "void expect_vla(int count,long value) { typedef int VLA[count]; (void)EXPECT_TYPE_SIZE(VLA,value); }",
                &arguments,
                false,
            )
            .is_empty()
        );
        let unsized_diagnostic = rust_oracle::reject_rust(&format!(
            "{rust}\nfn main() {{ let _=EXPECT_TYPE_SIZE!([i32],7); }}"
        ));
        assert!(
            unsized_diagnostic.contains("Sized"),
            "a variable-size Rust type cannot be admitted: {unsized_diagnostic}"
        );
        for invocation in [
            "EXPECT_RUNTIME_NULL!(__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(expect_record_value(0)))",
            "EXPECT_IMPURE_NULL!()",
            "EXPECT_RAW!(core::ptr::null_mut::<i32>(),1)",
            "EXPECT_RAW!(1,core::ptr::null_mut::<i32>())",
        ] {
            let diagnostic = rust_oracle::reject_rust(&format!(
                "{rust}\nfn main() {{ unsafe {{ let _={invocation}; }} }}"
            ));
            assert!(diagnostic.contains("ImplicitTo"), "invalid implicit conversion: {diagnostic}");
        }
        // The impure hint's unusual Clang constant-folding rules are deliberately
        // not used to establish Rust null identity. Compare C's definite runtime
        // and pointer-argument rejections, independent of that conservative case.
        for invocation in [
            "EXPECT_RUNTIME_NULL(expect_record_value(0))",
            "EXPECT_RAW((int *)0,1)",
            "EXPECT_RAW(1,(int *)0)",
            "EXPECT_WRONG0()",
            "EXPECT_WRONG1(7)",
            "EXPECT_WRONG3(7)",
            "EXPECT_SYMBOL()",
            "EXPECT_ADDRESS()",
            "EXPECT_DEREF()",
            "EXPECT_CALLBACK()",
        ] {
            let diagnostic = rust_oracle::reject_c_invocation(
                &profile.compiler.executable,
                &header,
                &format!("void invalid(void) {{ (void) {invocation}; }}"),
                &arguments,
            );
            assert!(!diagnostic.is_empty());
        }
    }
    let frontend = inspect(&scanner, &header, &arguments("-O2", true), None).unwrap();
    let names = ["EXPECT_RAW", "EXPECT_NESTED"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindings(&frontend);
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    let mut rust = rust_base(&directory, &bindings, &generated.support.rust);
    for emission in &generated.macros {
        let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
            panic!("source macro shadowing must follow C preprocessing: {emission:?}");
        };
        rust.push_str(definition);
    }
    let original = "#include <stdio.h>\nint main(void) { int values[]={-7,0,1,17}; for(unsigned int i=0;i<4;i++) printf(\"%ld %ld\\n\",EXPECT_RAW(values[i],0),EXPECT_NESTED(values[i])); }";
    let consumer = "fn main() { for value in [-7_i32,0,1,17] { println!(\"{} {}\",EXPECT_RAW!(value,0).get(),EXPECT_NESTED!(value).get()); } }";
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let expected = oracle::run_c(&profile.compiler.executable, &header, original, &arguments, true);
    let actual = rust_oracle::run_rust(&format!("{rust}\n{consumer}"));
    assert_eq!(actual, expected, "source macros shadow compiler builtins before lowering");
}

/// Checks that expectation builtins preserve native branch weight directions.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
#[test]
fn expectation_builtins_preserve_native_branch_weight_directions() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/expect_oracle.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let frontend = inspect(&scanner, &header, &arguments("-O2", false), None).unwrap();
    let names = [
        "EXPECT_RAW",
        "EXPECT_TYPE_CAST",
        "EXPECT_TYPE_EFFECT",
        "EXPECT_TYPE_DYNAMIC",
        "EXPECT_TYPE_SIZE",
        "EXPECT_TYPE_ALIGN",
        "EXPECT_TYPE_POINTER_SIZE",
        "EXPECT_VALUE_SIZE",
        "EXPECT_SOURCE_SIZE",
        "EXPECT_OFFSET_TYPE",
        "EXPECT_OFFSET_FIELD",
        "EXPECT_OFFSET_BOTH",
        "EXPECT_RAW_SEVEN",
        "EXPECT_DELEGATED_SEVEN",
        "EXPECT_DELEGATED_NESTED",
        "EXPECT_NESTED_LONG_LONG",
        "EXPECT_NESTED_UNSIGNED_LONG",
        "EXPECT_NESTED_INT128",
        "EXPECT_NESTED_INT",
        "EXPECT_NESTED_BOOL",
        "EXPECT_NESTED_DYNAMIC",
        "EXPECT_EFFECT_ZERO",
        "EXPECT_LIKELY",
        "EXPECT_UNLIKELY",
        "EXPECT_INTERRUPTS",
        "EXPECT_AND_ONE",
        "EXPECT_LAZY_ZERO",
        "EXPECT_NESTED",
        "EXPECT_NESTED_SEVEN",
    ];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindings(&frontend);
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    assert_eq!(generated.macros.len(), names.len());
    let scratch = TemporaryDirectory::new();
    let runtime_source = scratch.0.join("runtime.rs");
    let runtime_library = scratch.0.join("libpgrx_expect_oracle_runtime.rlib");
    // Generated adapters implement sealed runtime capabilities and belong with
    // the bindings and exported macros in their defining crate, as in pg-sys.
    let mut runtime = rust_base(&directory, &bindings, &generated.support.rust);
    for emission in &generated.macros {
        let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
            panic!("expectation codegen fixture must transpile: {emission:?}");
        };
        runtime.push_str(definition);
    }
    fs::write(&runtime_source, runtime).expect("write actual runtime crate's generated macros");
    rust_oracle::run_tool(
        Command::new("rustc")
            .args([
                "--edition=2024",
                "--crate-type=rlib",
                "--crate-name=pgrx_expect_oracle_runtime",
                "-O",
                "-C",
                "lto=off",
            ])
            .arg(&runtime_source)
            .arg("-o")
            .arg(&runtime_library),
        "compile_expect_runtime",
    );
    // Exercise the extension/runtime boundary without LTO. Exported macro $crate
    // references must retain the runtime's own bindings and inline capabilities.
    let rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n\
         pub use pgrx_expect_oracle_runtime::*;\n{RUST_CODEGEN}"
    );

    let original = scratch.0.join("expect.c");
    let emitted = scratch.0.join("expect.rs");
    fs::write(&original, C_CODEGEN).expect("write original expectation codegen witnesses");
    fs::write(&emitted, rust).expect("write generated expectation codegen witnesses");
    let profile = frontend.profile();
    let original_llvm = rust_oracle::run_tool(
        Command::new(&profile.compiler.executable)
            .args(&profile.arguments)
            .args(["-x", "c", "-S", "-emit-llvm", "-o", "-", "-include"])
            .arg(&header)
            .arg(&original),
        "original_expect_llvm",
    );
    let emitted_llvm = rust_oracle::run_tool(
        Command::new("rustc")
            .args(["--edition=2024", "--crate-type=lib", "-O", "--emit=llvm-ir", "-o", "-"])
            .args(["-C", "lto=off", "--extern"])
            .arg(format!("pgrx_expect_oracle_runtime={}", runtime_library.display()))
            .arg(&emitted),
        "emitted_expect_llvm",
    );
    for (function, callee, expected_hot) in [
        ("expect_codegen_likely", "expect_branch_yes", true),
        ("expect_codegen_unlikely", "expect_branch_yes", false),
        ("expect_codegen_raw", "expect_branch_yes", true),
        ("expect_codegen_negated", "expect_branch_yes", false),
        ("expect_codegen_interrupts", "expect_process_interrupts", false),
        ("expect_codegen_and", "expect_branch_yes", true),
        ("expect_codegen_conditional", "expect_branch_no", true),
        ("expect_codegen_nested", "expect_branch_yes", true),
        ("expect_codegen_nested_seven", "expect_branch_yes", true),
        ("expect_codegen_delegated", "expect_branch_yes", true),
        ("expect_codegen_delegated_nested", "expect_branch_yes", true),
        ("expect_codegen_nested_long_long", "expect_branch_yes", true),
        ("expect_codegen_nested_unsigned_long", "expect_branch_yes", true),
        ("expect_codegen_nested_int128", "expect_branch_yes", true),
        ("expect_codegen_nested_int", "expect_branch_yes", true),
        ("expect_codegen_effect", "expect_branch_yes", true),
        ("expect_codegen_nested_bool", "expect_branch_yes", true),
        ("expect_codegen_typed_cast", "expect_branch_yes", true),
        ("expect_codegen_typed_effect", "expect_branch_yes", true),
        ("expect_codegen_typed_size", "expect_branch_yes", true),
        ("expect_codegen_typed_align", "expect_branch_yes", true),
        ("expect_codegen_typed_pointer_size", "expect_branch_yes", true),
        ("expect_codegen_offset_type", "expect_branch_yes", true),
        ("expect_codegen_offset_field", "expect_branch_yes", true),
        ("expect_codegen_offset_both", "expect_branch_yes", true),
        ("expect_codegen_value_size", "expect_branch_yes", true),
        ("expect_codegen_source_size", "expect_branch_yes", true),
    ] {
        let original_hot = weighted_call_direction(&original_llvm, function, callee);
        assert_eq!(original_hot, expected_hot, "the original C witness's expectation direction");
        assert_eq!(
            weighted_call_direction(&emitted_llvm, function, callee),
            original_hot,
            "{function} must favor the same successor as native C, independent of weight magnitude"
        );
    }
    for llvm in [&original_llvm, &emitted_llvm] {
        for function in ["expect_codegen_effect", "expect_codegen_typed_effect"] {
            assert_eq!(
                function_body(llvm, function).matches("@expect_record_hint(").count(),
                1,
                "a statically known expected value must still evaluate its side effect exactly once"
            );
        }
        assert!(
            !function_body(llvm, "expect_codegen_source_size").contains("@expect_record_hint("),
            "a fixed sizeof expectation must not evaluate its hint-source operand"
        );
        for function in [
            "expect_codegen_dynamic",
            "expect_codegen_nested_dynamic",
            "expect_codegen_typed_dynamic",
        ] {
            let dynamic = function_body(llvm, function);
            assert!(
                dynamic.contains("br i1 "),
                "dynamic witness must retain its conditional branch: {dynamic}"
            );
            assert!(
                !dynamic.contains("!prof "),
                "a genuinely runtime expected operand must not invent branch weights that Clang omits: {dynamic}"
            );
        }
        assert_eq!(
            function_body(llvm, "expect_codegen_interrupts").matches("load volatile").count(),
            1,
            "the interrupt condition must retain exactly one volatile flag load"
        );
    }
}

/// Extract a named LLVM function's body for control-flow and branch-weight inspection.
fn function_body<'a>(llvm: &'a str, name: &str) -> &'a str {
    let definition = llvm
        .lines()
        .find(|line| line.starts_with("define ") && line.contains(&format!("@{name}(")))
        .unwrap_or_else(|| panic!("LLVM did not preserve the exported expectation witness {name}"));
    let start = llvm.find(definition).unwrap();
    let tail = &llvm[start..];
    &tail[..tail.find("\n}").expect("LLVM function must terminate") + 2]
}

/// Read a basic block's outgoing branch labels instead of inferring direction from condition
/// spelling.
fn successors(line: &str) -> Vec<&str> {
    line.split("label %")
        .skip(1)
        .map(|successor| {
            successor
                .split(|character: char| character == ',' || character.is_whitespace())
                .next()
                .unwrap()
        })
        .collect()
}

/// Follow successor blocks to determine whether a branch reaches the observable call used by
/// the hint witness.
fn reaches_call(blocks: &BTreeMap<&str, Vec<&str>>, first: &str, callee: &str) -> bool {
    let mut pending = vec![first];
    let mut seen = BTreeSet::new();
    let call = format!("@{callee}(");
    while let Some(label) = pending.pop() {
        if !seen.insert(label) {
            continue;
        }
        let lines = blocks.get(label).unwrap_or_else(|| panic!("unknown LLVM successor {label}"));
        if lines.iter().any(|line| line.contains(&call)) {
            return true;
        }
        for line in lines {
            if line.trim_start().starts_with("br ") {
                pending.extend(successors(line));
            }
        }
    }
    false
}

/// Extract the integer weights attached to a branch metadata node.
fn branch_weights(metadata: &str) -> Option<[u32; 2]> {
    let payload = metadata.split_once("!{").expect("LLVM metadata must contain a node").1;
    let payload = payload.rsplit_once('}').expect("LLVM metadata node must terminate").0;
    let mut fields = payload.split(',').map(str::trim);
    if fields.next() != Some("!\"branch_weights\"") {
        return None;
    }
    let weights = fields
        .filter_map(|field| field.strip_prefix("i32 "))
        .map(|weight| {
            let value = weight.parse::<i64>().unwrap_or_else(|error| {
                panic!("invalid LLVM branch weight {weight:?}: {error}; {metadata}")
            });
            // LLVM prints an i32 bit pattern with either signed or unsigned
            // spelling. Profile weights are unsigned, including after scaling.
            if value < 0 {
                i32::try_from(value).expect("LLVM signed i32 weight must fit") as u32
            } else {
                u32::try_from(value).expect("LLVM unsigned i32 weight must fit")
            }
        })
        .collect::<Vec<_>>();
    Some(weights.try_into().unwrap_or_else(|weights: Vec<u32>| {
        panic!("a conditional branch must have two weights, got {weights:?}: {metadata}")
    }))
}

/// Resolve which weighted successor reaches the witness call, distinguishing a preserved hint
/// from a reversed one.
fn weighted_call_direction(llvm: &str, function: &str, callee: &str) -> bool {
    let body = function_body(llvm, function);
    let mut blocks = BTreeMap::<&str, Vec<&str>>::new();
    let mut label = "entry";
    for line in body.lines().skip(1) {
        if let Some((candidate, _)) = line.split_once(':')
            && !candidate.is_empty()
            && candidate
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || "._-".contains(character))
        {
            label = candidate;
        } else {
            blocks.entry(label).or_default().push(line);
        }
    }
    let mut directions = Vec::new();
    for branch in body.lines().filter(|line| line.trim_start().starts_with("br i1 ")) {
        let Some((_, metadata)) = branch.split_once("!prof !") else {
            continue;
        };
        let id = metadata.split(|character: char| !character.is_ascii_digit()).next().unwrap();
        let prefix = format!("!{id} = ");
        let weights = llvm
            .lines()
            .find(|line| line.starts_with(&prefix))
            .expect("referenced branch metadata must exist");
        let Some(weights) = branch_weights(weights) else {
            continue;
        };
        let successors = successors(branch);
        assert_eq!(successors.len(), 2);
        let reaches = [
            reaches_call(&blocks, successors[0], callee),
            reaches_call(&blocks, successors[1], callee),
        ];
        let index = match reaches {
            [true, false] => 0,
            [false, true] => 1,
            _ => continue,
        };
        assert_ne!(weights[0], weights[1], "expectation must distinguish branch weights: {branch}");
        directions.push(weights[index] > weights[1 - index]);
    }
    assert_eq!(
        directions.len(),
        1,
        "{function} must have one weighted branch that distinguishes reaching {callee}:\n{body}"
    );
    directions[0]
}

/// Checks that branch direction parser resolves successors instead of condition spelling.
#[test]
fn branch_direction_parser_resolves_successors_instead_of_condition_spelling() {
    let llvm = r#"
define void @witness(i1 %condition) {
entry:
  br i1 %condition, label %without_call, label %through_trampoline, !prof !0
without_call:
  br label %exit
through_trampoline:
  br label %with_call
with_call:
  call void @sink()
  br label %exit
exit:
  ret void
}
!0 = !{!"branch_weights", !"expected", i32 1, i32 2000}
"#;
    assert!(weighted_call_direction(llvm, "witness", "sink"));
    assert!(weighted_call_direction(&llvm.replace("!\"expected\", ", ""), "witness", "sink"));
    assert!(weighted_call_direction(
        &llvm.replace("i32 2000", "i32 -2147483648"),
        "witness",
        "sink"
    ));
    assert!(weighted_call_direction(
        &llvm.replace("i32 2000", "i32 4294967295"),
        "witness",
        "sink"
    ));
    assert!(!weighted_call_direction(
        &llvm.replace("i32 1, i32 2000", "i32 2000, i32 1"),
        "witness",
        "sink"
    ));
    assert!(weighted_call_direction(
        &llvm
            .replace(
                "label %without_call, label %through_trampoline",
                "label %through_trampoline, label %without_call"
            )
            .replace("i32 1, i32 2000", "i32 2000, i32 1"),
        "witness",
        "sink"
    ));
}

/// Own isolated compiler inputs and outputs so oracle runs cannot reuse stale artifacts or
/// leave a growing target tree.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
struct TemporaryDirectory(
    /// Owned fixture path used for isolated inputs and cleanup.
    PathBuf,
);

/// Allocate isolated compiler artifacts with process-local uniqueness and deterministic cleanup
/// ownership.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
impl TemporaryDirectory {
    /// Create owned, uniquely named fixture storage so this test's headers and compiler outputs
    /// cannot collide with another invocation.
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir()
            .join(format!("pgrx-expect-codegen-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).expect("create expectation codegen scratch directory");
        Self(path)
    }
}

/// Release only temporary artifacts owned by this fixture, including on failed compiler or
/// assertion paths.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
impl Drop for TemporaryDirectory {
    /// Remove only this fixture's owned temporary storage after the test or oracle completes.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
