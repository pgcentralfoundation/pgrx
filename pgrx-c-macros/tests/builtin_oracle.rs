//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Validate compiler builtins and isolate failed capability probes.
//!
//! Original C prototypes, native executions, and compiler witnesses establish
//! builtin types and effects. Conflicts or shadowed proof helpers must disable
//! only the affected capabilities; unrelated macro translations must survive.
//! Configured PostgreSQL checks additionally cover its actual Datum storage.
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
    AnalysisSession, BindingCatalog, EmissionStatus, PostgresConfig, generate_with_bindings,
};
use pgrx_c_macros::{
    BuiltinKind, FrontendOutput, IntegerKind, MacroScanner, TypeCategory, inspect,
};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
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
    "BUILTIN_SWAP16",
    "BUILTIN_SWAP32",
    "BUILTIN_SWAP64",
    "BUILTIN_GROUPED",
    "BUILTIN_NESTED32",
    "BUILTIN_PROFILE",
    "BUILTIN_HOST32",
    "BUILTIN_RECORD64",
    "BUILTIN_VOLATILE",
    "BUILTIN_SIZE",
    "BUILTIN_LAZY",
    "BUILTIN_ZERO32",
    "BUILTIN_ZERO64",
    "BUILTIN_NULL_SELECT",
    "BUILTIN_NULL_CALL",
    "BUILTIN_RUNTIME_NULL",
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
    "BUILTIN_WRONG0",
    "BUILTIN_WRONG2",
    "BUILTIN_SYMBOL",
    "BUILTIN_ADDRESS",
    "BUILTIN_DEREF",
    "BUILTIN_CALLBACK",
];
/// Fixture binding or native-support source paired with the unchanged C oracle.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
const NATIVE: &str = r#"
unsigned int builtin_evaluations;
volatile unsigned int builtin_signal;
unsigned int builtin_record32(unsigned int value) { builtin_evaluations++; return value; }
unsigned long long builtin_record64(unsigned long long value) { builtin_evaluations++; return value; }
unsigned int *builtin_address(unsigned int *value) { builtin_evaluations++; return value; }
int *builtin_take_pointer(int *value) { return value; }
unsigned int builtin_use_callback(BuiltinCallback callback) { return callback(17); }
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
#define C_RANK(value) _Generic((value), unsigned short: 2, unsigned int: 3, unsigned long: 4, unsigned long long: 5, default: 0)
_Static_assert(_Generic(BUILTIN_NULL_SELECT(1, (int *)0), int *: 1, default: 0), "closed builtin zero retains null-pointer-constant identity");
int main(void) {
    unsigned long long values[] = {0, 1, 0x0102030405060708ULL, 0xFFFFFFFFFFFFFFFFULL, 0x8000000000000000ULL, 0x100000001ULL, 0x0123456789ABCDEFULL};
    for (unsigned int index=0; index<7; index++) {
        unsigned long long value=values[index];
        printf("%04x %08x %016llx\n", (unsigned int) BUILTIN_SWAP16(value), (unsigned int) BUILTIN_SWAP32(value), (unsigned long long) BUILTIN_SWAP64(value));
    }
    int signed_values[]={-1, -2, -65537, 2147483647};
    for (unsigned int index=0; index<4; index++) {
        int value=signed_values[index];
        printf("%04x %08x %016llx\n", (unsigned int) BUILTIN_SWAP16(value), (unsigned int) BUILTIN_SWAP32(value), (unsigned long long) BUILTIN_SWAP64(value));
    }
    printf("%d %d %d %zu %zu %zu\n", C_RANK(BUILTIN_SWAP16(0)), C_RANK(BUILTIN_SWAP32(0)), C_RANK(BUILTIN_SWAP64(0)), sizeof(BUILTIN_SWAP16(0)), sizeof(BUILTIN_SWAP32(0)), sizeof(BUILTIN_SWAP64(0)));
    printf("%08x %016llx %08x\n", (unsigned int) BUILTIN_NESTED32(0x01020304U), (unsigned long long) BUILTIN_HOST32(0x01020304U), (unsigned int) BUILTIN_GROUPED(0x01020304U));
    printf("%016llx %d %zu\n", (unsigned long long) BUILTIN_PROFILE(0x1234U), C_RANK(BUILTIN_PROFILE(0x1234U)), sizeof(BUILTIN_PROFILE(0x1234U)));
    unsigned int once=BUILTIN_SWAP32(builtin_record32(0x01020304U));
    unsigned long long once64=BUILTIN_RECORD64(0x0102030405060708ULL);
    printf("%08x %016llx %u\n", once, once64, builtin_evaluations);
    builtin_signal=0x01234567U;
    printf("%08x %08x\n", (unsigned int) BUILTIN_VOLATILE(), (unsigned int) builtin_signal);
    builtin_evaluations=0;
    unsigned long size=BUILTIN_SIZE(builtin_record32(17));
    printf("%lu %u\n", size, builtin_evaluations);
    unsigned int absent=BUILTIN_LAZY(0, builtin_address((unsigned int *)0));
    printf("%08x %u\n", absent, builtin_evaluations);
    unsigned int object=0x89ABCDEFU;
    unsigned int present=BUILTIN_LAZY(1, builtin_address(&object));
    printf("%08x %u\n", present, builtin_evaluations);
    int pointee=31; int *pointer=&pointee;
    printf("%d %d %d\n", BUILTIN_NULL_SELECT(1,pointer)==0, BUILTIN_NULL_SELECT(0,pointer)==pointer, BUILTIN_NULL_CALL()==0);
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
fn main() {
    for value in [0_u64, 1, 0x0102030405060708, u64::MAX, 0x8000000000000000, 0x100000001, 0x0123456789ABCDEF] {
        let input=__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLongLong>::new(value);
        println!("{:04x} {:08x} {:016x}", BUILTIN_SWAP16!(input).get(), BUILTIN_SWAP32!(input).get(), BUILTIN_SWAP64!(input).get());
    }
    for value in [-1_i32, -2, -65537, i32::MAX] {
        println!("{:04x} {:08x} {:016x}", BUILTIN_SWAP16!(value).get(), BUILTIN_SWAP32!(value).get(), BUILTIN_SWAP64!(value).get());
    }
    let short=BUILTIN_SWAP16!(0); let int=BUILTIN_SWAP32!(0); let wide=BUILTIN_SWAP64!(0);
    println!("{} {} {} {} {} {}", rank(short.into_value()), rank(int.into_value()), rank(wide.into_value()), core::mem::size_of_val(&short.get()), core::mem::size_of_val(&int.get()), core::mem::size_of_val(&wide.get()));
    println!("{:08x} {:016x} {:08x}", BUILTIN_NESTED32!(0x01020304_u32).get(), BUILTIN_HOST32!(0x01020304_u32).get(), BUILTIN_GROUPED!(0x01020304_u32).get());
    let profiled=BUILTIN_PROFILE!(0x1234_u32);
    println!("{:016x} {} {}",profiled.get(),rank(profiled.into_value()),core::mem::size_of_val(&profiled.get()));
    // SAFETY: native functions mutate initialized process-local scalar counters
    // in this single-threaded executable. All accessed pointers name live,
    // aligned, initialized owned locals or the original volatile scalar global.
    // There are no references or concurrent accesses to these allocations.
    // The false branch never evaluates its callback or null-pointer dereference.
    unsafe {
        let once=BUILTIN_SWAP32!(builtin_record32(0x01020304)).get();
        let input=__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLongLong>::new(0x0102030405060708);
        let once64=BUILTIN_RECORD64!(input).get();
        let evaluations=builtin_evaluations;
        println!("{:08x} {:016x} {}", once, once64, evaluations);
        assert_eq!(evaluations,2,"each builtin argument is evaluated exactly once");
        core::ptr::write_volatile(core::ptr::addr_of_mut!(builtin_signal),0x01234567);
        let reversed=BUILTIN_VOLATILE!().get();
        let signal=core::ptr::read_volatile(core::ptr::addr_of!(builtin_signal));
        println!("{:08x} {:08x}", reversed,signal);
        builtin_evaluations=0;
        let size=BUILTIN_SIZE!(@__pgrx_c_expression; builtin_record32(17)).get();
        let evaluations=builtin_evaluations;
        println!("{} {}",size,evaluations);
        assert_eq!(evaluations,0,"sizeof suppresses builtin operand evaluation");
        let absent=BUILTIN_LAZY!(0,builtin_address(core::ptr::null_mut())).get();
        let evaluations=builtin_evaluations;
        println!("{:08x} {}",absent,evaluations);
        assert_eq!(evaluations,0,"an unselected operand is not evaluated");
        let mut object=0x89ABCDEF_u32; let pointer=&raw mut object;
        let present=BUILTIN_LAZY!(1,builtin_address(pointer)).get();
        let evaluations=builtin_evaluations;
        println!("{:08x} {}",present,evaluations);
        assert_eq!(evaluations,1);
        let mut pointee=31_i32; let pointer=&raw mut pointee;
        let absent: *mut i32=BUILTIN_NULL_SELECT!(1,pointer).get();
        let present: *mut i32=BUILTIN_NULL_SELECT!(0,pointer).get();
        let called: *mut i32=BUILTIN_NULL_CALL!().get();
        println!("{} {} {}",u8::from(absent.is_null()),u8::from(present==pointer),u8::from(called.is_null()));
    }
}
"#;

/// Inspect the fixture under an explicit compiler profile and retain the facts needed by
/// generation and native comparison.
fn profile(
    scanner: &MacroScanner,
    header: &Path,
    optimization: &str,
    shadow: bool,
) -> FrontendOutput {
    inspect(scanner, header, &arguments(optimization, shadow), None).unwrap()
}

/// Construct the C invocation used for both inspection and the native oracle, so
/// compiler-profile differences cannot explain a mismatch.
fn arguments(optimization: &str, shadow: bool) -> Vec<String> {
    let mut arguments = vec!["-std=c17".into(), optimization.into(), "-ffp-contract=off".into()];
    if shadow {
        arguments.push("-DBUILTIN_SHADOW=1".into());
    }
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "builtin oracle SDK",
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
        .allowlist_type("Builtin.*")
        .allowlist_function("builtin_.*")
        .allowlist_var("builtin_.*")
        .use_core()
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .unwrap()
        .to_string()
}

/// Assemble a standalone Rust producer with the real semantic support and generated adapter
/// definitions.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
fn base(directory: &Path, bindings: &str, support: &str) -> String {
    let runtime = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={runtime:?}] pub mod __pgrx_c_macros;\n{bindings}\n{support}"
    )
}

/// Checks that compiler builtins preserve prototypes evaluation null constants and macro
/// shadowing.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
#[test]
fn compiler_builtins_preserve_prototypes_evaluation_null_constants_and_macro_shadowing() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/builtin_oracle.h");
    let scanner = MacroScanner::new().expect("libclang required");
    for optimization in ["-O0", "-O2"] {
        let frontend = profile(&scanner, &header, optimization, false);
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
        let mut rust = base(&directory, &bindings, &generated.support.rust);
        for emission in &generated.macros {
            if REJECTED.contains(&emission.analysis.name.as_str()) {
                let EmissionStatus::Skipped { reason } = &emission.status else {
                    panic!("invalid builtin use must be rejected: {emission:?}");
                };
                assert!(!reason.message.is_empty());
                continue;
            }
            let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
                panic!("proved builtin macro must emit under {optimization}: {emission:?}");
            };
            if emission.analysis.name == "BUILTIN_VOLATILE" {
                assert!(
                    definition.contains("CVolatile"),
                    "volatile load must retain its compiler qualifier"
                );
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
        assert_eq!(
            actual, expected,
            "builtin semantics follow the exact {optimization} compiler profile"
        );
        assert_eq!(actual.lines().count(), 20);
        let diagnostic = rust_oracle::reject_rust(&format!(
            "{rust}\nfn main() {{ unsafe {{ let _=BUILTIN_RUNTIME_NULL!(builtin_record32(0)); }} }}"
        ));
        assert!(
            diagnostic.contains("ImplicitTo"),
            "runtime zero must not acquire ICE null identity: {diagnostic}"
        );
        let diagnostic = rust_oracle::reject_c_invocation(
            &profile.compiler.executable,
            &header,
            "void invalid(void) { (void) BUILTIN_RUNTIME_NULL(builtin_record32(0)); }",
            &arguments,
        );
        assert!(
            diagnostic.contains("integer to pointer"),
            "original C rejects runtime zero: {diagnostic}"
        );
        for invocation in [
            "BUILTIN_WRONG0()",
            "BUILTIN_WRONG2(7)",
            "BUILTIN_SYMBOL()",
            "BUILTIN_ADDRESS()",
            "BUILTIN_DEREF()",
            "BUILTIN_CALLBACK()",
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
    let frontend = profile(&scanner, &header, "-O2", true);
    let names = ["BUILTIN_SWAP32", "BUILTIN_NESTED32"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindings(&frontend);
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    let mut rust = base(&directory, &bindings, &generated.support.rust);
    for emission in &generated.macros {
        let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
            panic!("ordinary macro shadowing must use C preprocessing: {emission:?}");
        };
        rust.push_str(definition);
    }
    let original = "#include <stdio.h>\nint main(void) { unsigned int values[]={0,1,0x01020304U,0xFFFFFFFFU}; for(unsigned int i=0;i<4;i++) printf(\"%08x %08x\\n\",BUILTIN_SWAP32(values[i]),BUILTIN_NESTED32(values[i])); }";
    let consumer = "fn main() { for value in [0_u32,1,0x01020304,u32::MAX] { println!(\"{:08x} {:08x}\",BUILTIN_SWAP32!(value).get(),BUILTIN_NESTED32!(value).get()); } }";
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let expected = oracle::run_c(&profile.compiler.executable, &header, original, &arguments, true);
    let actual = rust_oracle::run_rust(&format!("{rust}\n{consumer}"));
    assert_eq!(actual, expected, "source macros shadow compiler builtins before lowering");
    assert_eq!(actual.lines().count(), 4);
}

/// Original C input used to demonstrate that one rejected capability does not remove unrelated
/// peers.
const ISOLATION_HEADER: &str = r#"
#define ISOLATED16(value) __builtin_bswap16(value)
#define ISOLATED32(value) __builtin_bswap32(value)
#define ISOLATED64(value) __builtin_bswap64(value)
#define ISOLATED_PLAIN(value) (value)
"#;
/// Builtin and ordinary peers selected together for probe-isolation assertions.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
const ISOLATION_NAMES: &[&str] = &["ISOLATED16", "ISOLATED32", "ISOLATED64", "ISOLATED_PLAIN"];
/// C input providing a branch-hint operation alongside independent builtin capabilities.
const EXPECT_HEADER: &str =
    "#define ISOLATED_EXPECT(value, expected) __builtin_expect(value, expected)";

/// Own the temporary header tree for this test and remove only its fixture files afterward.
struct Directory(
    /// Owned fixture path used for isolated inputs and cleanup.
    PathBuf,
);

/// Construct owned header trees for freshness and include-identity tests.
impl Directory {
    /// Create owned, uniquely named fixture storage so this test's headers and compiler outputs
    /// cannot collide with another invocation.
    fn new() -> Self {
        for attempt in 0..64 {
            let path = std::env::temp_dir()
                .join(format!("pgrx-builtin-errors-{}-{attempt}", std::process::id()));
            match std::fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("could not create builtin fixture directory: {error}"),
            }
        }
        panic!("could not reserve builtin fixture directory");
    }

    /// Return the original fixture header whose preprocessing and C definitions supply this
    /// test's semantics.
    fn header(&self, prefix: &str) -> PathBuf {
        let header = self.0.join("isolation.h");
        std::fs::write(&header, format!("{prefix}\n{ISOLATION_HEADER}")).unwrap();
        header
    }
}

/// Release only temporary artifacts owned by this fixture, including on failed compiler or
/// assertion paths.
impl Drop for Directory {
    /// Remove only this fixture's owned temporary storage after the test or oracle completes.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Require a failed capability witness to leave ordinary supported macros emitted.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
fn assert_isolated_emission(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    unavailable: &[&str],
    explanation: &str,
) -> String {
    let session = AnalysisSession::prepare(scanner, frontend, ISOLATION_NAMES).unwrap();
    let generated =
        generate_with_bindings(&session, ISOLATION_NAMES, &BindingCatalog::default()).unwrap();
    assert_eq!(generated.macros.len(), ISOLATION_NAMES.len());
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut rust = base(&directory, "", &generated.support.rust);
    for emission in generated.macros {
        match emission.status {
            EmissionStatus::Skipped { reason }
                if unavailable.contains(&emission.analysis.name.as_str()) =>
            {
                assert!(reason.message.contains(explanation), "{reason:?}");
            }
            EmissionStatus::Emitted { rust: definition, .. }
                if !unavailable.contains(&emission.analysis.name.as_str()) =>
            {
                rust.push_str(&definition);
            }
            status => panic!(
                "unexpected isolated macro outcome for {}: {status:?}",
                emission.analysis.name
            ),
        }
    }
    rust
}

/// Execute the surviving builtin peers against C after one capability is deliberately rejected.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
fn assert_supported_peers_run(rust: &str) {
    let output = rust_oracle::run_rust(&format!(
        r#"{rust}
fn main() {{
    assert_eq!(ISOLATED16!(0x1234_u32).get(), 0x3412);
    let value=__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLongLong>::new(0x0102030405060708);
    assert_eq!(ISOLATED64!(value).get(), 0x0807060504030201);
    assert_eq!(ISOLATED_PLAIN!(31_i32).get(), 31);
}}
"#
    ));
    assert!(output.is_empty());
}

/// Checks that original builtin declaration conflict does not remove other operations.
/// Catalog isolation is checked on every target; surviving emitted peers run only under LP64.
#[test]
fn original_builtin_declaration_conflict_does_not_remove_other_operations() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    // The compatible prototype keeps the original C header valid. Generation
    // still conservatively rejects an original declaration with this spelling.
    let header = directory.header("unsigned int __builtin_bswap32(unsigned int);");
    let scanner = MacroScanner::new().expect("libclang required");
    let frontend = profile(&scanner, &header, "-O2", false);
    let declarations = frontend.declarations();
    assert!(declarations.functions.contains_key("__builtin_bswap32"));
    assert!(!declarations.builtins.contains_key("__builtin_bswap32"));
    for name in ["__builtin_bswap16", "__builtin_bswap64"] {
        assert!(declarations.builtins.contains_key(name), "{name} must remain proven");
        assert!(!declarations.builtin_unavailable.contains_key(name));
    }
    let explanation = "an original C declaration conflicts with the compiler builtin";
    assert!(declarations.builtin_unavailable["__builtin_bswap32"].contains(explanation));
    #[cfg(all(
        target_pointer_width = "64",
        target_endian = "little",
        any(target_arch = "x86_64", target_arch = "aarch64"),
        any(target_os = "linux", target_os = "macos"),
    ))]
    {
        let rust = assert_isolated_emission(&scanner, &frontend, &["ISOLATED32"], explanation);
        assert_supported_peers_run(&rust);
    }
}

/// Checks that shadowed typeof proof helper skips builtins and keeps ordinary macros.
/// Compiler proof refusals remain portable; the ordinary emitted peer requires the LP64 runtime.
#[test]
fn shadowed_typeof_proof_helper_skips_builtins_and_keeps_ordinary_macros() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.header("#define __typeof__(expression) unsigned int");
    let scanner = MacroScanner::new().expect("libclang required");
    let frontend = profile(&scanner, &header, "-O2", false);
    let explanation = "an active C macro shadows a builtin proof operation";
    for name in ["__builtin_bswap16", "__builtin_bswap32", "__builtin_bswap64"] {
        assert!(!frontend.declarations().builtins.contains_key(name));
        assert!(frontend.declarations().builtin_unavailable[name].contains(explanation));
    }
    #[cfg(all(
        target_pointer_width = "64",
        target_endian = "little",
        any(target_arch = "x86_64", target_arch = "aarch64"),
        any(target_os = "linux", target_os = "macos"),
    ))]
    {
        let rust = assert_isolated_emission(
            &scanner,
            &frontend,
            &["ISOLATED16", "ISOLATED32", "ISOLATED64"],
            explanation,
        );
        assert!(
            rust_oracle::run_rust(&format!(
                "{rust}\nfn main() {{ assert_eq!(ISOLATED_PLAIN!(31_i32).get(), 31); }}"
            ))
            .is_empty()
        );
    }
}

/// Checks that expect uses exact C long rank and both parameter identities in each profile.
#[test]
fn expect_uses_exact_c_long_rank_and_both_parameter_identities_in_each_profile() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.header(EXPECT_HEADER);
    let scanner = MacroScanner::new().expect("libclang required");
    for optimization in ["-O0", "-O2"] {
        let frontend = profile(&scanner, &header, optimization, false);
        let info = frontend.declarations().builtins.get("__builtin_expect").unwrap_or_else(|| {
            panic!("{optimization}: {:?}", frontend.declarations().builtin_unavailable)
        });
        assert_eq!(info.kind, BuiltinKind::Expect);
        let result = &info.signature.result;
        assert_eq!(result.category, TypeCategory::Integer(IntegerKind::Long));
        let facts = &frontend.profile().target.integers[&IntegerKind::Long];
        assert!(facts.signed);
        assert_eq!(facts.rank, 4);
        assert_eq!(result.size, Some(u64::from(facts.bits / 8)));
        let parameters = info.signature.parameters.as_ref().unwrap();
        assert_eq!(parameters.len(), 2);
        assert!(parameters.iter().all(|parameter| parameter == result));
        assert!(!info.signature.variadic);
        assert_eq!(info.signature.calling_convention.as_deref(), Some("Cdecl"));
        for name in ["__builtin_bswap16", "__builtin_bswap32", "__builtin_bswap64"] {
            assert!(frontend.declarations().builtins.contains_key(name));
        }
    }
}

/// Checks that expect macro and declaration shadowing are isolated from byte swaps.
#[test]
fn expect_macro_and_declaration_shadowing_are_isolated_from_byte_swaps() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang required");
    for (prefix, explanation) in [
        (
            "#define __builtin_expect(value, expected) ((long)(value))",
            "an active C macro shadows the compiler builtin",
        ),
        (
            "long __builtin_expect(long, long);",
            "an original C declaration conflicts with the compiler builtin",
        ),
        (
            "#define __typeof__(expression) unsigned int",
            "an active C macro shadows a builtin proof operation",
        ),
    ] {
        let directory = Directory::new();
        let header = directory.header(&format!("{prefix}\n{EXPECT_HEADER}"));
        let frontend = profile(&scanner, &header, "-O2", false);
        assert!(!frontend.declarations().builtins.contains_key("__builtin_expect"));
        assert!(
            frontend.declarations().builtin_unavailable["__builtin_expect"].contains(explanation)
        );
        if !prefix.contains("__typeof__") {
            for name in ["__builtin_bswap16", "__builtin_bswap32", "__builtin_bswap64"] {
                assert!(frontend.declarations().builtins.contains_key(name));
            }
        } else {
            for name in ["__builtin_bswap16", "__builtin_bswap32", "__builtin_bswap64"] {
                assert!(frontend.declarations().builtin_unavailable[name].contains(explanation));
            }
        }
    }
}

/// Construct a compiler wrapper that rejects a selected witness so capability isolation and
/// driver bounds can be tested.
#[cfg(unix)]
fn reject_llvm_witness(
    directory: &Directory,
    compiler: &Path,
    rejected: &str,
) -> (PathBuf, PathBuf) {
    use std::os::unix::fs::PermissionsExt;

    let wrapper = directory.0.join("clang-wrapper");
    let log = directory.0.join("llvm-runs");
    std::fs::write(&log, "").unwrap();
    let shell_quote = |text: &str| format!("'{}'", text.replace('\'', "'\\''"));
    let script = format!(
        r#"#!/bin/sh
llvm=0
overlay=
next_overlay=0
for argument do
    if [ "$next_overlay" -eq 1 ]; then
        overlay=$argument
        next_overlay=0
        continue
    fi
    case "$argument" in
        -emit-llvm) llvm=1 ;;
        -ivfsoverlay) next_overlay=1 ;;
    esac
done
if [ "$llvm" -eq 1 ] && [ -n "$overlay" ]; then
    source=$(sed -n 's/.*"external-contents":"\([^"]*\)".*/\1/p' "$overlay")
    if [ ! -r "$source" ]; then
        printf >&2 '%s\n' 'could not read the builtin witness overlay'
        exit 2
    fi
    operations=
    for operation in __builtin_bswap16 __builtin_bswap32 __builtin_bswap64 __builtin_expect; do
        if LC_ALL=C grep -q "return $operation(" "$source"; then
            operations="$operations $operation"
        fi
    done
    printf '%s\n' "$operations" >> {log}
    case " $operations " in
        *" {rejected} "*) printf >&2 '%s\n' {diagnostic}; exit 1 ;;
    esac
fi
exec {compiler} "$@"
"#,
        log = shell_quote(log.to_str().unwrap()),
        compiler = shell_quote(compiler.to_str().unwrap()),
        diagnostic = shell_quote(&format!("deliberate {rejected} LLVM witness rejection")),
    );
    // Only dynamic witnesses fail. Version/resource queries, preprocessing and
    // typed probes retain the authentic compiler and original profile.
    std::fs::write(&wrapper, script).unwrap();
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o700)).unwrap();
    (wrapper, log)
}

/// Checks that rejected LLVM witness is isolated within four driver runs.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
#[test]
fn rejected_llvm_witness_is_isolated_within_four_driver_runs() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.header("");
    let scanner = MacroScanner::new().expect("libclang required");
    let baseline = profile(&scanner, &header, "-O2", false);
    for name in ["__builtin_bswap16", "__builtin_bswap32", "__builtin_bswap64"] {
        assert!(baseline.declarations().builtins.contains_key(name));
    }
    let (wrapper, log) = reject_llvm_witness(
        &directory,
        &baseline.profile().compiler.executable,
        "__builtin_bswap32",
    );
    let frontend = inspect(&scanner, &header, &arguments("-O2", false), Some(&wrapper)).unwrap();
    let batches = std::fs::read_to_string(&log)
        .unwrap()
        .lines()
        .map(|line| line.split_ascii_whitespace().map(str::to_owned).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert!(!batches.is_empty(), "the wrapper must observe real LLVM witnesses");
    assert!(batches.len() <= 4, "driver isolation must respect its four-run budget: {batches:?}");
    assert_eq!(
        batches[0],
        ["__builtin_bswap16", "__builtin_bswap32", "__builtin_bswap64"],
        "the original proof must batch all three operations"
    );
    assert!(batches.iter().any(|batch| batch.as_slice() == ["__builtin_bswap16"]));
    assert!(batches.iter().any(|batch| batch.as_slice() == ["__builtin_bswap64"]));
    let explanation = "deliberate __builtin_bswap32 LLVM witness rejection";
    assert!(!frontend.declarations().builtins.contains_key("__builtin_bswap32"));
    assert!(frontend.declarations().builtin_unavailable["__builtin_bswap32"].contains(explanation));
    for name in ["__builtin_bswap16", "__builtin_bswap64"] {
        assert!(frontend.declarations().builtins.contains_key(name), "{name} must remain proven");
        assert!(!frontend.declarations().builtin_unavailable.contains_key(name));
    }
    let rust = assert_isolated_emission(&scanner, &frontend, &["ISOLATED32"], explanation);
    assert_supported_peers_run(&rust);
}

/// Checks that rejected expect witness keeps all byte swaps within five driver runs.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
#[test]
fn rejected_expect_witness_keeps_all_byte_swaps_within_five_driver_runs() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.header(EXPECT_HEADER);
    let scanner = MacroScanner::new().expect("libclang required");
    let baseline = profile(&scanner, &header, "-O2", false);
    assert!(baseline.declarations().builtins.contains_key("__builtin_expect"));
    let (wrapper, log) = reject_llvm_witness(
        &directory,
        &baseline.profile().compiler.executable,
        "__builtin_expect",
    );
    let frontend = inspect(&scanner, &header, &arguments("-O2", false), Some(&wrapper)).unwrap();
    let batches = std::fs::read_to_string(&log)
        .unwrap()
        .lines()
        .map(|line| line.split_ascii_whitespace().map(str::to_owned).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert!(!batches.is_empty(), "the wrapper must observe real LLVM witnesses");
    assert!(batches.len() <= 5, "driver isolation must respect its five-run budget: {batches:?}");
    assert_eq!(
        batches[0],
        ["__builtin_bswap16", "__builtin_bswap32", "__builtin_bswap64", "__builtin_expect"],
        "the original proof must batch all four operations"
    );
    let explanation = "deliberate __builtin_expect LLVM witness rejection";
    assert!(!frontend.declarations().builtins.contains_key("__builtin_expect"));
    assert!(frontend.declarations().builtin_unavailable["__builtin_expect"].contains(explanation));
    for name in ["__builtin_bswap16", "__builtin_bswap32", "__builtin_bswap64"] {
        assert!(batches.iter().any(|batch| batch.as_slice() == [name]));
        assert!(frontend.declarations().builtins.contains_key(name), "{name} must remain proven");
        assert!(!frontend.declarations().builtin_unavailable.contains_key(name));
    }
    let session = AnalysisSession::prepare(&scanner, &frontend, &["ISOLATED_EXPECT"]).unwrap();
    let generated =
        generate_with_bindings(&session, &["ISOLATED_EXPECT"], &BindingCatalog::default()).unwrap();
    let emission = generated.macros.into_iter().next().unwrap();
    let EmissionStatus::Skipped { reason } = emission.status else {
        panic!("rejected expect witness must skip its dependent macro");
    };
    assert!(reason.message.contains(explanation));
    let rust = assert_isolated_emission(&scanner, &frontend, &[], "");
    assert_supported_peers_run(&rust);
}

/// Find the configured PG19 installation for a real-header oracle, allowing environments
/// without it to omit that optional case.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
fn configured_pg19() -> Option<PostgresConfig> {
    use pgrx_pg_config::Pgrx;

    let explicit_pg_config = match std::env::var("PGRX_PG_CONFIG_PATH") {
        Ok(_) => true,
        Err(std::env::VarError::NotPresent) => false,
        Err(std::env::VarError::NotUnicode(_)) => {
            panic!("PGRX_PG_CONFIG_PATH must be UTF-8 for the inspected compilation profile");
        }
    };
    if !explicit_pg_config {
        let configuration = match Pgrx::config_toml() {
            Ok(path) => path,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("PG19 native builtin oracle omitted: pgrx home is not configured");
                return None;
            }
            Err(error) => panic!("could not locate pgrx configuration: {error}"),
        };
        if !configuration.try_exists().expect("inspect pgrx configuration path") {
            eprintln!("PG19 native builtin oracle omitted: pgrx configuration does not exist");
            return None;
        }
    }
    let configurations = Pgrx::from_config().expect("read pgrx configuration");
    let selected = match configurations.get("pg19") {
        Ok(configuration) => configuration,
        Err(error) if error.to_string() == "Postgres `pg19` is not managed by pgrx" => {
            eprintln!("PG19 native builtin oracle omitted: PG19 is not configured");
            return None;
        }
        Err(error) => panic!("could not resolve configured PG19: {error}"),
    };
    Some(PostgresConfig::from_pg_config(selected).expect("resolve installed PG19 headers"))
}

/// Checks that configured pg19 datum conversion matches original C without entering backend
/// guards.
#[cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]
#[test]
fn configured_pg19_datum_conversion_matches_original_c_without_entering_backend_guards() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(postgres) = configured_pg19() else {
        return;
    };
    assert_eq!(postgres.pg_config().major_version().unwrap(), 19);
    let directory = Directory::new();
    let header = directory.0.join("postgres_builtin.h");
    std::fs::write(&header, "#include \"postgres.h\"\n#include \"port/pg_bswap.h\"\n").unwrap();
    let scanner = MacroScanner::new().expect("libclang required");
    let mut extra = Vec::new();
    #[cfg(target_os = "linux")]
    extra.extend(["-ffunction-sections".into(), "-fdata-sections".into()]);
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "PG19 builtin oracle SDK",
        );
        extra.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend = postgres
        .inspect(&scanner, Some(&header), &extra, None)
        .expect("inspect original PG19 builtin profile");
    let expected_calls = if frontend.environment().active.contains_key("WORDS_BIGENDIAN") {
        0
    } else if frontend.environment().active.contains_key("pg_bswap64") {
        2
    } else {
        3
    };
    let names = ["DatumBigEndianToNative"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("^(Datum|uint64)$")
        .allowlist_function("^(DatumGetUInt64|UInt64GetDatum)$")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate fresh installed PG19 scalar bindings")
        .to_string();
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    catalog.ffi_boundary = Some(vec!["ffi".into(), "boundary".into()]);
    let TypeCategory::Integer(datum_kind) = frontend.declarations().types["Datum"].category else {
        panic!("PG19 Datum must have its compiler-established integer identity");
    };
    let datum_marker = match datum_kind {
        IntegerKind::UnsignedLong => "CUnsignedLong",
        IntegerKind::UnsignedLongLong => "CUnsignedLongLong",
        other => panic!("unexpected native 64-bit PG19 Datum identity: {other:?}"),
    };
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    let project = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut rust = base(&project, &bindings, &generated.support.rust);
    for emission in generated.macros {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("actual PG19 datum endian macro must emit: {emission:?}");
        };
        rust.push_str(&definition);
    }
    // The installed postgres.h helpers only cast unsigned integer payloads;
    // pg_bswap.h's possible static-inline fallback only rearranges those bits.
    // Generated C adapters call those original definitions, with no backend
    // globals, allocation, pointer dereference, callback, or error path. This
    // standalone boundary counts calls and invokes the native adapter directly;
    // it does not substitute pgrx's backend guard in the production crate.
    rust.push_str(
        r#"
pub mod ffi {
    pub static CALLS: core::sync::atomic::AtomicU32=core::sync::atomic::AtomicU32::new(0);
    /// # Safety
    /// Only the generated pure integer conversion and byte-swap adapters enter
    /// this standalone boundary. The closure must neither access backend state nor
    /// invoke callbacks, allocate, unwind, or perform a nonlocal jump.
    pub unsafe fn boundary<T,F:FnOnce()->T>(call:F)->T {
        CALLS.fetch_add(1,core::sync::atomic::Ordering::Relaxed);
        call()
    }
}
fn rank<K:__pgrx_c_macros::CInteger>(_:__pgrx_c_macros::CValue<K>)->u8 { K::RANK }
"#,
    );
    rust.push_str(&format!(
        r#"
fn main() {{
    for bits in [0_u64,1,u64::MAX,0x0102030405060708,0x8000000000000000,0x0123456789ABCDEF] {{
        let input=__pgrx_c_macros::CValue::<__pgrx_c_macros::{datum_marker}>::new(bits);
        let evaluations=core::cell::Cell::new(0_u32);
        ffi::CALLS.store(0,core::sync::atomic::Ordering::Relaxed);
        // SAFETY: This standalone macro only calls the original C scalar cast
        // or byte-swap helpers through generated adapters and the direct boundary.
        // Its initialized unsigned payload is never interpreted as a pointer.
        let result=unsafe {{ DatumBigEndianToNative!({{evaluations.set(evaluations.get()+1); input}}) }};
        let value=result.get();
        println!("{{:016x}} {{}} {{}} {{}}",value,rank(result.into_value()),core::mem::size_of_val(&value),evaluations.get());
        assert_eq!(evaluations.get(),1);
        assert_eq!(ffi::CALLS.load(core::sync::atomic::Ordering::Relaxed),{expected_calls});
    }}
}}
"#
    ));
    let original = r#"
#include <stdio.h>
#undef printf
#define C_RANK(value) _Generic((value), unsigned long: 4, unsigned long long: 5, default: 0)
int main(void) {
    unsigned long long values[]={0,1,0xFFFFFFFFFFFFFFFFULL,0x0102030405060708ULL,0x8000000000000000ULL,0x0123456789ABCDEFULL};
    for(unsigned int index=0;index<6;index++) {
        unsigned int evaluations=0;
        Datum value=DatumBigEndianToNative((evaluations++, (Datum)values[index]));
        printf("%016llx %d %zu %u\n",(unsigned long long)value,C_RANK(value),sizeof(value),evaluations);
    }
}
"#;
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let expected = oracle::run_c(&profile.compiler.executable, &header, original, &arguments, true);
    let actual = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header,
        &generated.support.c_source,
        &arguments,
    );
    assert_eq!(actual, expected, "actual PG19 macros and scalar helpers must preserve C semantics");
    assert_eq!(actual.lines().count(), 6);
}
