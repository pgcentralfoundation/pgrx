//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise supported token-pasting macros through actual PostgreSQL exports.
//!
//! The public integration tests check symbolic results and caller evaluation while
//! using this build's bindings. The independent C oracle supplies the broader
//! semantic coverage for closed paste translation.

#![cfg(all(pgrx_c_macros, not(docsrs)))]
#![deny(unsafe_op_in_unsafe_fn)]

/// Run original C headers through the bounded independent oracle harness.
#[path = "../../pgrx-c-macros/tests/support/oracle.rs"]
mod oracle;

use core::cell::Cell;
use core::mem::MaybeUninit;
use core::ptr::addr_of_mut;
use pgrx_pg_sys as pg;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Read the selected build's macro audit report to configure an independent original-header C
/// oracle.
#[derive(serde::Deserialize)]
struct BuildReport {
    /// PostgreSQL version whose fresh macro report selects the original-header oracle.
    postgres_major_version: u16,
    /// Compiler invocation used by the actual binding build.
    profile: OriginalProfile,
}

/// Retain only the recorded invocation facts needed to recompile the original C definitions.
#[derive(serde::Deserialize)]
struct OriginalProfile {
    /// Original build wrapper whose macros must supply the native expectations.
    header: PathBuf,
    /// Compiler executable recorded during verified frontend inspection.
    compiler: OriginalCompiler,
    /// Exact flags recorded by this binding build for oracle compilation.
    arguments: Vec<String>,
}

/// Identify the exact compiler selected by frontend verification.
#[derive(serde::Deserialize)]
struct OriginalCompiler {
    /// Recorded Clang executable used to generate and validate this version's macros.
    executable: PathBuf,
}

/// Pair original-C input, result, and evaluation counts for comparison with public generated
/// macros.
struct Observation<I, R> {
    /// Input supplied to the original C macro and matching generated Rust invocation.
    input: I,
    /// Expected result observed independently from the unchanged installed header.
    result: R,
    /// Number of recorded operand evaluations in that original C invocation.
    evaluations: u32,
}

/// Cache independent original-C observations for the integration macro families so tests share
/// one native compilation.
#[derive(Default)]
struct OriginalResults {
    /// Original C observations for two-lane rotation, including operand evaluation counts.
    rotation: Vec<Observation<u64, u64>>,
    /// Original C observations for segment-count arithmetic, including operand evaluation
    /// counts.
    segments: Vec<Observation<u32, u64>>,
    /// Original C observations for timestamp range checks, including operand evaluation counts.
    timestamp: Vec<Observation<i64, bool>>,
    /// Original C observations for trigger context tags, including operand evaluation counts.
    trigger: Vec<Observation<i64, bool>>,
    /// Original C observations for event-trigger context tags, including operand evaluation
    /// counts.
    event_trigger: Vec<Observation<i64, bool>>,
    /// Original C observations for time-to-seconds conversion, including operand evaluation
    /// counts.
    time_double: Vec<Observation<i64, u64>>,
    /// Original C observations for time-to-milliseconds conversion, including operand
    /// evaluation counts.
    time_millisec: Vec<Observation<i64, u64>>,
    /// Original C observations for time-to-microseconds conversion, including operand
    /// evaluation counts.
    time_microsec: Vec<Observation<i64, u64>>,
    /// Original C observations for CTE target-list selection, including operand evaluation
    /// counts.
    cte: Vec<Observation<u32, i32>>,
    /// Original C observations for dummy-append short circuiting, including operand evaluation
    /// counts.
    dummy_append: Vec<Observation<u32, bool>>,
    /// Original C observations for memory-context tag predicates, including operand evaluation
    /// counts.
    memory_context: Vec<Observation<i64, bool>>,
    /// Original C observations for soft-error branch selection, including operand evaluation
    /// counts.
    soft_error: Vec<Observation<u32, bool>>,
}

/// Original C recorder source whose header invocations establish expected semantic
/// observations.
const ORIGINAL: &str = r#"
#include <limits.h>
#include <stdio.h>
#include <string.h>
#undef printf
/* Use libc only for oracle output; none of the original macro bodies change. */
static unsigned int pgrx_paste_oracle_calls;
static unsigned long long record_rotation(unsigned long long value) { pgrx_paste_oracle_calls++; return value; }
static unsigned int record_size(unsigned int value) { pgrx_paste_oracle_calls++; return value; }
static long long record_timestamp(long long value) { pgrx_paste_oracle_calls++; return value; }
static FunctionCallInfo record_info(FunctionCallInfo value) { pgrx_paste_oracle_calls++; return value; }
static CommonTableExpr *record_cte(CommonTableExpr *value) { pgrx_paste_oracle_calls++; return value; }
static AppendPath *record_append(AppendPath *value) { pgrx_paste_oracle_calls++; return value; }
static MemoryContext record_context(MemoryContext value) { pgrx_paste_oracle_calls++; return value; }
#ifdef SOFT_ERROR_OCCURRED
static ErrorSaveContext *record_error(ErrorSaveContext *value) { pgrx_paste_oracle_calls++; return value; }
#endif
#if PG_VERSION_NUM < 190000 && !defined(WIN32)
static const instr_time *record_time(const instr_time *value) { pgrx_paste_oracle_calls++; return value; }
static unsigned long long double_bits(double value) {
    unsigned long long bits;
    _Static_assert(sizeof bits == sizeof value, "double and oracle bit storage have the same width");
    memcpy(&bits, &value, sizeof bits);
    return bits;
}
#endif
_Static_assert(MIN_TIMESTAMP > LLONG_MIN && END_TIMESTAMP < LLONG_MAX, "timestamp boundary neighbors fit in long long");
int main(void) {
    unsigned long long rotations[]={0,1,ULLONG_MAX,0x8000000080000000ULL,0x0123456789ABCDEFULL,0x0000000100000000ULL};
    unsigned int sizes[]={1,8192,1024*1024,16*1024*1024,1024*1024*1024,UINT_MAX};
    long long timestamps[]={LLONG_MIN,MIN_TIMESTAMP-1,MIN_TIMESTAMP,MIN_TIMESTAMP+1,0,END_TIMESTAMP-1,END_TIMESTAMP,LLONG_MAX};
    NodeTag tags[]={T_Invalid,T_TriggerData,T_EventTriggerData};
    FunctionCallInfoBaseData info;
    Node node;
    CommonTableExpr cte;
    Query query;
    List target, returning, subpath;
    List *selected;
    CmdType commands[]={CMD_SELECT,CMD_UPDATE,CMD_INSERT};
    AppendPath append;
    MemoryContextData context;
    NodeTag context_tags[]={T_Invalid,T_AllocSetContext,T_SlabContext,T_GenerationContext
#if PG_VERSION_NUM >= 170000
        ,T_BumpContext
#endif
    };
#ifdef SOFT_ERROR_OCCURRED
    ErrorSaveContext error;
#endif
#if PG_VERSION_NUM < 190000 && !defined(WIN32)
    long long ticks[]={0,1,-1,999,1000,-1000,1123456789LL,LLONG_MIN,LLONG_MAX};
    instr_time timer;
#endif
    unsigned int index;
    unsigned long long numeric;
    int result;
    for(index=0;index<sizeof rotations/sizeof rotations[0];index++) {
        pgrx_paste_oracle_calls=0;
        numeric=ROTATE_HIGH_AND_LOW_32BITS(record_rotation(rotations[index]));
        printf("rotation %llu %llu %u\n",rotations[index],numeric,pgrx_paste_oracle_calls);
    }
    for(index=0;index<sizeof sizes/sizeof sizes[0];index++) {
        pgrx_paste_oracle_calls=0;
        numeric=XLogSegmentsPerXLogId(record_size(sizes[index]));
        printf("segments %u %llu %u\n",sizes[index],numeric,pgrx_paste_oracle_calls);
    }
    for(index=0;index<sizeof timestamps/sizeof timestamps[0];index++) {
        pgrx_paste_oracle_calls=0;
        result=IS_VALID_TIMESTAMP(record_timestamp(timestamps[index]));
        printf("timestamp %lld %d %u\n",timestamps[index],result,pgrx_paste_oracle_calls);
    }
    info.context=NULL;
    pgrx_paste_oracle_calls=0;
    result=CALLED_AS_TRIGGER(record_info(&info));
    printf("trigger -1 %d %u\n",result,pgrx_paste_oracle_calls);
    pgrx_paste_oracle_calls=0;
    result=CALLED_AS_EVENT_TRIGGER(record_info(&info));
    printf("event_trigger -1 %d %u\n",result,pgrx_paste_oracle_calls);
    for(index=0;index<sizeof tags/sizeof tags[0];index++) {
        node.type=tags[index];
        info.context=&node;
        pgrx_paste_oracle_calls=0;
        result=CALLED_AS_TRIGGER(record_info(&info));
        printf("trigger %u %d %u\n",(unsigned int)tags[index],result,pgrx_paste_oracle_calls);
        pgrx_paste_oracle_calls=0;
        result=CALLED_AS_EVENT_TRIGGER(record_info(&info));
        printf("event_trigger %u %d %u\n",(unsigned int)tags[index],result,pgrx_paste_oracle_calls);
    }
    query.type=T_Query;
    cte.ctequery=(Node *)&query;
    for(index=0;index<6;index++) {
        query.commandType=commands[index/2];
        query.targetList=(index%2) ? NULL : &target;
        query.returningList=(index%2) ? NULL : &returning;
        pgrx_paste_oracle_calls=0;
        selected=GetCTETargetList(record_cte(&cte));
        result=selected == NULL ? -1 : (selected == &target ? 0 : (selected == &returning ? 1 : 2));
        printf("cte %u %d %u\n",index,result,pgrx_paste_oracle_calls);
    }
    for(index=0;index<3;index++) {
        append.path.type=index == 0 ? T_Invalid : T_AppendPath;
        /* The wrong tag must skip even an uninitialized subpaths field. */
        if(index != 0) append.subpaths=index == 1 ? NULL : &subpath;
        pgrx_paste_oracle_calls=0;
        result=IS_DUMMY_APPEND(record_append(&append));
        printf("dummy_append %u %d %u\n",index,result,pgrx_paste_oracle_calls);
    }
    pgrx_paste_oracle_calls=0;
    result=MemoryContextIsValid(record_context(NULL));
    printf("memory_context -1 %d %u\n",result,pgrx_paste_oracle_calls);
    for(index=0;index<sizeof context_tags/sizeof context_tags[0];index++) {
        context.type=context_tags[index];
        pgrx_paste_oracle_calls=0;
        result=MemoryContextIsValid(record_context(&context));
        printf("memory_context %u %d %u\n",(unsigned int)context_tags[index],result,pgrx_paste_oracle_calls);
    }
#ifdef SOFT_ERROR_OCCURRED
    for(index=0;index<4;index++) {
        error.type=index < 2 ? T_Invalid : T_ErrorSaveContext;
        /* The null and wrong-tag branches never inspect this uninitialized flag. */
        if(index >= 2) error.error_occurred=index == 3;
        pgrx_paste_oracle_calls=0;
        result=SOFT_ERROR_OCCURRED(record_error(index == 0 ? NULL : &error));
        printf("soft_error %u %d %u\n",index,result,pgrx_paste_oracle_calls);
    }
#endif
#if PG_VERSION_NUM < 190000 && !defined(WIN32)
    for(index=0;index<sizeof ticks/sizeof ticks[0];index++) {
#if PG_VERSION_NUM >= 160000
        timer.ticks=ticks[index];
#elif defined(HAVE_CLOCK_GETTIME)
        timer.tv_sec=ticks[index]/1000000000LL;
        timer.tv_nsec=ticks[index]%1000000000LL;
#else
        timer.tv_sec=ticks[index]/1000000000LL;
        timer.tv_usec=(ticks[index]%1000000000LL)/1000;
#endif
        pgrx_paste_oracle_calls=0;
        numeric=double_bits(INSTR_TIME_GET_DOUBLE(*record_time(&timer)));
        printf("time_double %lld %llu %u\n",ticks[index],numeric,pgrx_paste_oracle_calls);
        pgrx_paste_oracle_calls=0;
        numeric=double_bits(INSTR_TIME_GET_MILLISEC(*record_time(&timer)));
        printf("time_millisec %lld %llu %u\n",ticks[index],numeric,pgrx_paste_oracle_calls);
        pgrx_paste_oracle_calls=0;
        numeric=(unsigned long long)INSTR_TIME_GET_MICROSEC(*record_time(&timer));
        printf("time_microsec %lld %llu %u\n",ticks[index],numeric,pgrx_paste_oracle_calls);
    }
#endif
}
"#;

/// Compile the installed original C headers under this build's reported profile once and retain
/// the independent observations for public-macro comparisons.
fn original_results() -> &'static OriginalResults {
    /// Cache independently compiled C observations for this process so integration tests share
    /// the same verified profile.
    static RESULTS: OnceLock<OriginalResults> = OnceLock::new();
    RESULTS.get_or_init(|| {
        let major = if cfg!(feature = "pg15") {
            15
        } else if cfg!(feature = "pg16") {
            16
        } else if cfg!(feature = "pg17") {
            17
        } else if cfg!(feature = "pg18") {
            18
        } else {
            assert!(cfg!(feature = "pg19"), "one PostgreSQL major feature is required");
            19
        };
        let path = PathBuf::from(env!("OUT_DIR")).join(format!("pg{major}_macro_report.json"));
        // Stream and ignore the large macro bodies; only the current build's
        // original compiler profile is needed, never checked-in bindings.
        let report: BuildReport = serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open(&path).expect("open this build's C macro report"),
        ))
        .expect("read this build's original compiler profile");
        assert_eq!(report.postgres_major_version, major);
        let arguments = report.profile.arguments;
        // Remove unused linker sections from the all-header wrapper on Linux.
        // These codegen flags do not change its preprocessing or target ABI.
        #[cfg(target_os = "linux")]
        let arguments = {
            let mut arguments = arguments;
            arguments.extend(["-ffunction-sections".into(), "-fdata-sections".into()]);
            arguments
        };
        let arguments = arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let output = oracle::run_c(
            &report.profile.compiler.executable,
            &report.profile.header,
            ORIGINAL,
            &arguments,
            true,
        );
        let mut results = OriginalResults::default();
        for line in output.lines() {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            assert_eq!(fields.len(), 4, "unexpected original C oracle row: {line}");
            let evaluations = number(fields[3]);
            match fields[0] {
                "rotation" => results.rotation.push(Observation {
                    input: number(fields[1]),
                    result: number(fields[2]),
                    evaluations,
                }),
                "segments" => results.segments.push(Observation {
                    input: number(fields[1]),
                    result: number(fields[2]),
                    evaluations,
                }),
                "timestamp" => results.timestamp.push(Observation {
                    input: number(fields[1]),
                    result: c_truth(fields[2]),
                    evaluations,
                }),
                "trigger" => results.trigger.push(Observation {
                    input: number(fields[1]),
                    result: c_truth(fields[2]),
                    evaluations,
                }),
                "event_trigger" => results.event_trigger.push(Observation {
                    input: number(fields[1]),
                    result: c_truth(fields[2]),
                    evaluations,
                }),
                "time_double" => results.time_double.push(Observation {
                    input: number(fields[1]),
                    result: number(fields[2]),
                    evaluations,
                }),
                "time_millisec" => results.time_millisec.push(Observation {
                    input: number(fields[1]),
                    result: number(fields[2]),
                    evaluations,
                }),
                "time_microsec" => results.time_microsec.push(Observation {
                    input: number(fields[1]),
                    result: number(fields[2]),
                    evaluations,
                }),
                "cte" => results.cte.push(Observation {
                    input: number(fields[1]),
                    result: number(fields[2]),
                    evaluations,
                }),
                "dummy_append" => results.dummy_append.push(Observation {
                    input: number(fields[1]),
                    result: c_truth(fields[2]),
                    evaluations,
                }),
                "memory_context" => results.memory_context.push(Observation {
                    input: number(fields[1]),
                    result: c_truth(fields[2]),
                    evaluations,
                }),
                "soft_error" => results.soft_error.push(Observation {
                    input: number(fields[1]),
                    result: c_truth(fields[2]),
                    evaluations,
                }),
                _ => panic!("unknown original C oracle row: {line}"),
            }
        }
        assert_eq!(
            (
                results.rotation.len(),
                results.segments.len(),
                results.timestamp.len(),
                results.trigger.len(),
                results.event_trigger.len()
            ),
            (6, 6, 8, 4, 4)
        );
        assert_eq!(results.cte.len(), 6);
        assert_eq!(results.dummy_append.len(), 3);
        assert_eq!(results.memory_context.len(), if major >= 17 { 6 } else { 5 });
        assert_eq!(results.soft_error.len(), if major >= 16 { 4 } else { 0 });
        let time_cases = if major < 19 && !cfg!(target_os = "windows") { 9 } else { 0 };
        assert_eq!(
            (results.time_double.len(), results.time_millisec.len(), results.time_microsec.len()),
            (time_cases, time_cases, time_cases)
        );
        results
    })
}

/// Parse a numeric oracle column with its expected storage type, rejecting malformed
/// observations.
fn number<T: std::str::FromStr>(value: &str) -> T {
    value.parse().unwrap_or_else(|_| panic!("invalid original C oracle integer: {value}"))
}

/// Decode the original C predicate result without confusing integer truth with Rust-only bool
/// storage.
fn c_truth(value: &str) -> bool {
    match value {
        "0" => false,
        "1" => true,
        _ => panic!("invalid original C oracle truth value: {value}"),
    }
}

pg::__pgrx_c_classify! { @if_available ROTATE_HIGH_AND_LOW_32BITS {
/// Checks that generated lane rotation matches the original C two lane operation.
#[test]
fn generated_lane_rotation_matches_the_original_c_two_lane_operation() {
    for case in &original_results().rotation {
        let input=case.input;
        let calls=Cell::new(0_u32);
        // The C expression is generic; distinguish long long from equally wide long.
        let value=pg::__pgrx_c_macros::CValue::<pg::__pgrx_c_macros::CUnsignedLongLong>::new(input);
        let actual=pg::ROTATE_HIGH_AND_LOW_32BITS!({calls.set(calls.get()+1);value}).get();
        assert_eq!(actual,case.result,"input={input:#018X}");
        assert_eq!(calls.get(),case.evaluations,"input={input:#018X}");
    }
}
} }

pg::__pgrx_c_classify! { @if_available XLogSegmentsPerXLogId {
/// Checks that generated segment count uses the pasted 64 bit boundary without truncation.
#[test]
fn generated_segment_count_uses_the_pasted_64_bit_boundary_without_truncation() {
    for case in &original_results().segments {
        let size=case.input;
        let calls=Cell::new(0_u32);
        let actual=pg::XLogSegmentsPerXLogId!({calls.set(calls.get()+1);size}).get();
        assert_eq!(actual,case.result,"size={size}");
        assert_eq!(calls.get(),case.evaluations,"size={size}");
    }
}
} }

pg::__pgrx_c_classify! { @if_available IS_VALID_TIMESTAMP {
/// Checks that generated timestamp range check preserves both boundaries and lazy evaluation.
#[test]
fn generated_timestamp_range_check_preserves_both_boundaries_and_lazy_evaluation() {
    for case in &original_results().timestamp {
        let input=case.input;
        let calls=Cell::new(0_u32);
        let value=pg::__pgrx_c_macros::CValue::<pg::__pgrx_c_macros::CLongLong>::new(input);
        let result=pg::IS_VALID_TIMESTAMP!({calls.set(calls.get()+1);value});
        assert_eq!(pg::__pgrx_c_macros::expression::truth(result.into_value()),case.result,"input={input}");
        assert_eq!(calls.get(),case.evaluations,"input={input}");
    }
}
} }

/// Build only the call-frame and node fields accessed on each selected branch, then compare
/// predicate results and evaluation counts with C.
fn exercise_context(
    mut classify: impl FnMut(pg::FunctionCallInfo, &Cell<u32>) -> bool,
    expected: &[Observation<i64, bool>],
) {
    let mut storage = MaybeUninit::<pg::FunctionCallInfoBaseData>::uninit();
    let info = storage.as_mut_ptr();
    let calls = Cell::new(0_u32);
    // SAFETY: info names live owned aligned record storage. Only its context
    // field is initialized and accessed; no whole-record reference/value is
    // formed. The null branch never dereferences context. Non-null contexts
    // point to live aligned initialized owned Nodes with valid enum values.
    // IsA reads only Node.type; neither classifier reads any trigger-specific
    // fields, accesses backend globals, nor calls a PostgreSQL function. All
    // record access is exclusive and every pointer stays within this function.
    unsafe {
        for case in expected {
            let mut node = pg::Node { type_: pg::NodeTag::T_Invalid };
            let context = if case.input == -1 {
                core::ptr::null_mut()
            } else {
                // Select a known valid enum value without constructing a Rust
                // discriminant from arbitrary C integer bits.
                node.type_ = [
                    pg::NodeTag::T_Invalid,
                    pg::NodeTag::T_TriggerData,
                    pg::NodeTag::T_EventTriggerData,
                ]
                .into_iter()
                .find(|tag| (*tag as i64) == case.input)
                .expect("original C tag has a valid corresponding fresh Rust binding");
                &raw mut node
            };
            addr_of_mut!((*info).context).write(context);
            assert_eq!(classify(info, &calls), case.result, "tag={}", case.input);
            assert_eq!(calls.replace(0), case.evaluations, "tag={}", case.input);
        }
    }
}

pg::__pgrx_c_classify! { @if_available CALLED_AS_TRIGGER {
/// Checks that generated trigger context check uses the closed pasted node tag.
#[test]
fn generated_trigger_context_check_uses_the_closed_pasted_node_tag() {
    exercise_context(|info,calls| {
        // SAFETY: exercise_context establishes owned context and tag read contracts.
        unsafe {
            pg::__pgrx_c_macros::expression::truth(
                pg::CALLED_AS_TRIGGER!({calls.set(calls.get()+1);info}).into_value()
            )
        }
    },&original_results().trigger);
}
} }

pg::__pgrx_c_classify! { @if_available CALLED_AS_EVENT_TRIGGER {
/// Checks that generated event trigger context check uses the closed pasted node tag.
#[test]
fn generated_event_trigger_context_check_uses_the_closed_pasted_node_tag() {
    exercise_context(|info,calls| {
        // SAFETY: exercise_context establishes owned context and tag read contracts.
        unsafe {
            pg::__pgrx_c_macros::expression::truth(
                pg::CALLED_AS_EVENT_TRIGGER!({calls.set(calls.get()+1);info}).into_value()
            )
        }
    },&original_results().event_trigger);
}
} }

/// Build the selected PostgreSQL version's instr_time storage for a signed tick observation.
#[cfg(all(not(feature = "pg19"), not(target_os = "windows")))]
fn time_value(ticks: i64) -> pg::instr_time {
    #[cfg(feature = "pg15")]
    {
        pg::instr_time { tv_sec: ticks / 1_000_000_000, tv_nsec: ticks % 1_000_000_000 }
    }
    #[cfg(not(feature = "pg15"))]
    {
        pg::instr_time { ticks }
    }
}

pg::__pgrx_c_classify! { @if_available INSTR_TIME_GET_DOUBLE {
/// Checks that generated time seconds matches original C floating bits.
#[cfg(all(not(feature="pg19"),not(target_os="windows")))]
#[test]
fn generated_time_seconds_matches_original_c_floating_bits() {
    for case in &original_results().time_double {
        let time=time_value(case.input);
        let calls=Cell::new(0_u32);
        // SAFETY: time is a fully initialized aligned owned instr_time record.
        // These PG15–18 Unix macros read integer fields and perform arithmetic;
        // they access neither a clock, backend state, nor a native function.
        let actual=unsafe {pg::INSTR_TIME_GET_DOUBLE!(*{calls.set(calls.get()+1);&raw const time}).get()};
        assert_eq!(actual.to_bits(),case.result,"ticks={}",case.input);
        assert_eq!(calls.get(),case.evaluations,"ticks={}",case.input);
    }
}

/// # Safety
/// The caller is in the permitted PostgreSQL backend thread, with timing
/// initialized for its selected clock source. time is fully initialized and
/// contains a valid tick representation for that clock, with conversion
/// arithmetic valid under the original pg_ticks_to_ns contract. The PG19
/// expansion invokes that helper through its guarded native adapter.
#[cfg(feature="pg19")]
#[allow(dead_code)]
unsafe fn time_seconds_in_backend(time:pg::instr_time)->f64 {
    // SAFETY: the caller establishes backend, timing and record preconditions.
    unsafe {pg::INSTR_TIME_GET_DOUBLE!(time).get()}
}
} }

pg::__pgrx_c_classify! { @if_available INSTR_TIME_GET_MILLISEC {
/// Checks that generated time milliseconds matches original C floating bits.
#[cfg(all(not(feature="pg19"),not(target_os="windows")))]
#[test]
fn generated_time_milliseconds_matches_original_c_floating_bits() {
    for case in &original_results().time_millisec {
        let time=time_value(case.input);
        let calls=Cell::new(0_u32);
        // SAFETY: time_value initializes the entire aligned owned record; the selected
        // PG15–18 Unix expansion only reads its scalar fields and does arithmetic.
        let actual=unsafe {pg::INSTR_TIME_GET_MILLISEC!(*{calls.set(calls.get()+1);&raw const time}).get()};
        assert_eq!(actual.to_bits(),case.result,"ticks={}",case.input);
        assert_eq!(calls.get(),case.evaluations,"ticks={}",case.input);
    }
}

/// # Safety
/// The caller is in the permitted PostgreSQL backend thread with timing
/// initialized for its selected clock source. time is initialized, contains a
/// valid tick representation for that clock, and permits the conversion
/// arithmetic required by the original pg_ticks_to_ns contract.
#[cfg(feature="pg19")]
#[allow(dead_code)]
unsafe fn time_milliseconds_in_backend(time:pg::instr_time)->f64 {
    // SAFETY: the caller establishes the guarded timing adapter's preconditions.
    unsafe {pg::INSTR_TIME_GET_MILLISEC!(time).get()}
}
} }

pg::__pgrx_c_classify! { @if_available INSTR_TIME_GET_MICROSEC {
/// Checks that generated time microseconds matches original C integer conversion.
#[cfg(all(not(feature="pg19"),not(target_os="windows")))]
#[test]
fn generated_time_microseconds_matches_original_c_integer_conversion() {
    for case in &original_results().time_microsec {
        let time=time_value(case.input);
        let calls=Cell::new(0_u32);
        // SAFETY: this fully initialized aligned owned record permits every selected
        // scalar-field read; the Unix PG15–18 expansion never calls a backend.
        let actual=unsafe {pg::INSTR_TIME_GET_MICROSEC!(*{calls.set(calls.get()+1);&raw const time})};
        // Normalize the original C macro's version-dependent signed/unsigned
        // result through the same defined unsigned-long-long conversion in C.
        let actual=pg::__pgrx_c_macros::expression::cast::<pg::__pgrx_c_macros::CUnsignedLongLong,_>(actual.into_value()).get();
        assert_eq!(actual,case.result,"ticks={}",case.input);
        assert_eq!(calls.get(),case.evaluations,"ticks={}",case.input);
    }
}

/// # Safety
/// The caller is in the permitted PostgreSQL backend thread with timing
/// initialized for its selected clock source. time is initialized, contains a
/// valid tick representation for that clock, and permits the conversion
/// arithmetic required by the original pg_ticks_to_ns contract.
#[cfg(feature="pg19")]
#[allow(dead_code)]
unsafe fn time_microseconds_in_backend(time:pg::instr_time)->i64 {
    // SAFETY: the caller establishes the guarded timing adapter's preconditions.
    unsafe {pg::INSTR_TIME_GET_MICROSEC!(time).get()}
}
} }

pg::__pgrx_c_classify! { @if_available GetCTETargetList {
/// Checks that generated cte target list selects owned pointer fields.
#[test]
fn generated_cte_target_list_selects_owned_pointer_fields() {
    let mut cte_storage=MaybeUninit::<pg::CommonTableExpr>::uninit();
    let cte=cte_storage.as_mut_ptr();
    let mut query_storage=MaybeUninit::<pg::Query>::uninit();
    let query=query_storage.as_mut_ptr();
    let mut target_storage=MaybeUninit::<pg::List>::uninit();
    let target=target_storage.as_mut_ptr();
    let mut returning_storage=MaybeUninit::<pg::List>::uninit();
    let returning=returning_storage.as_mut_ptr();
    let commands=[pg::CmdType::CMD_SELECT,pg::CmdType::CMD_UPDATE,pg::CmdType::CMD_INSERT];
    let calls=Cell::new(0_u32);
    // SAFETY: each raw pointer identifies live owned aligned storage with
    // exclusive access. Only ctequery, Query.type/commandType and the two list
    // pointer fields are initialized or read; no whole-record value/reference
    // is formed. The assertion sees a valid Query tag. Lists are used only as
    // pointer identities, never dereferenced. No backend function is called.
    unsafe {
        addr_of_mut!((*cte).ctequery).write(query.cast::<pg::Node>());
        addr_of_mut!((*query).type_).write(pg::NodeTag::T_Query);
        for case in &original_results().cte {
            addr_of_mut!((*query).commandType).write(commands[(case.input/2) as usize]);
            let absent=case.input%2 != 0;
            addr_of_mut!((*query).targetList).write(if absent {core::ptr::null_mut()} else {target});
            addr_of_mut!((*query).returningList).write(if absent {core::ptr::null_mut()} else {returning});
            let actual=pg::GetCTETargetList!({calls.set(calls.get()+1);cte}).get();
            let expected=match case.result {
                -1=>core::ptr::null_mut(),
                0=>target,
                1=>returning,
                _=>panic!("original C returned an unknown list pointer"),
            };
            assert_eq!(actual,expected,"case={}",case.input);
            assert_eq!(calls.replace(0),case.evaluations,"case={}",case.input);
        }
    }
}
} }

pg::__pgrx_c_classify! { @if_available IS_DUMMY_APPEND {
/// Checks that generated dummy append short circuits before uninitialized subpaths.
#[test]
fn generated_dummy_append_short_circuits_before_uninitialized_subpaths() {
    let mut storage=MaybeUninit::<pg::AppendPath>::uninit();
    let append=storage.as_mut_ptr();
    let mut list_storage=MaybeUninit::<pg::List>::uninit();
    let list=list_storage.as_mut_ptr();
    let calls=Cell::new(0_u32);
    // SAFETY: append and list point to owned aligned live storage. Only the
    // path.type field is read for the invalid tag, so subpaths can remain
    // uninitialized in that first case. Matching-tag cases initialize subpaths
    // before its read. The list is compared as an address, never dereferenced.
    // No whole-record value/reference or backend call is made.
    unsafe {
        for case in &original_results().dummy_append {
            addr_of_mut!((*append).path.type_).write(if case.input == 0 {pg::NodeTag::T_Invalid} else {pg::NodeTag::T_AppendPath});
            if case.input != 0 {
                addr_of_mut!((*append).subpaths).write(if case.input == 1 {core::ptr::null_mut()} else {list});
            }
            let actual=pg::__pgrx_c_macros::expression::truth(pg::IS_DUMMY_APPEND!({calls.set(calls.get()+1);append}).into_value());
            assert_eq!(actual,case.result,"case={}",case.input);
            assert_eq!(calls.replace(0),case.evaluations,"case={}",case.input);
        }
    }
}
} }

pg::__pgrx_c_classify! { @if_available MemoryContextIsValid {
/// Checks that generated memory context tags match original C lazy disjunction.
#[test]
fn generated_memory_context_tags_match_original_c_lazy_disjunction() {
    let mut storage=MaybeUninit::<pg::MemoryContextData>::uninit();
    let context=storage.as_mut_ptr();
    let tags=[pg::NodeTag::T_Invalid,pg::NodeTag::T_AllocSetContext,pg::NodeTag::T_SlabContext,pg::NodeTag::T_GenerationContext,
        #[cfg(any(feature="pg17",feature="pg18",feature="pg19"))]
        pg::NodeTag::T_BumpContext];
    let calls=Cell::new(0_u32);
    // SAFETY: context is an owned aligned live record; the macro reads only
    // its initialized valid NodeTag through the common Node prefix. The null
    // branch never dereferences it, and no other context fields or allocator
    // methods are accessed. Access is exclusive; no backend call is made.
    unsafe {
        for case in &original_results().memory_context {
            let input=if case.input == -1 {core::ptr::null_mut()} else {
                let tag=tags.into_iter().find(|tag|(*tag as i64)==case.input).expect("original C context tag has a valid fresh Rust binding");
                addr_of_mut!((*context).type_).write(tag);
                context
            };
            let actual=pg::__pgrx_c_macros::expression::truth(pg::MemoryContextIsValid!({calls.set(calls.get()+1);input}).into_value());
            assert_eq!(actual,case.result,"tag={}",case.input);
            assert_eq!(calls.replace(0),case.evaluations,"tag={}",case.input);
        }
    }
}
} }

pg::__pgrx_c_classify! { @if_available SOFT_ERROR_OCCURRED {
/// Checks that generated soft error checks only initialized fields on the selected path.
#[cfg(not(feature="pg15"))]
#[test]
fn generated_soft_error_checks_only_initialized_fields_on_the_selected_path() {
    let mut storage=MaybeUninit::<pg::ErrorSaveContext>::uninit();
    let context=storage.as_mut_ptr();
    let calls=Cell::new(0_u32);
    // SAFETY: this owned aligned record is accessed exclusively through raw
    // fields. Null and wrong-tag cases skip the uninitialized error flag.
    // Matching-tag cases initialize the flag before reading it. All tags and
    // bools are valid Rust representations; no backend error machinery runs.
    unsafe {
        for case in &original_results().soft_error {
            addr_of_mut!((*context).type_).write(if case.input < 2 {pg::NodeTag::T_Invalid} else {pg::NodeTag::T_ErrorSaveContext});
            if case.input >= 2 {
                addr_of_mut!((*context).error_occurred).write(case.input == 3);
            }
            let input=if case.input == 0 {core::ptr::null_mut()} else {context};
            let actual=pg::__pgrx_c_macros::expression::truth(pg::SOFT_ERROR_OCCURRED!({calls.set(calls.get()+1);input}).into_value());
            assert_eq!(actual,case.result,"case={}",case.input);
            assert_eq!(calls.replace(0),case.evaluations,"case={}",case.input);
        }
    }
}
} }
