//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check that capability pruning preserves the operations selected by macros.
//!
//! A synthetic header contains several callback and enum families. Emission may
//! omit unused adapters, but open operands and multiple call sites must retain
//! all compatible identities. Executed C and Rust observations check the result.

/// Reuse the binding build's collector so fixture tests reconcile exactly the Rust facts used
/// in production generation.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

/// Use the production scanner, analysis, and emission contracts so these checks exercise the
/// actual C macro pipeline.
use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, emit_batch_with_bindings,
    emit_support_artifact_with_bindings, inspect,
};
/// Read original fixtures and manage only the owned inputs and outputs used by generation
/// checks.
use std::fs;
/// Keep fixture and generated-output locations explicit so consumer builds remain independent
/// of the working directory.
use std::path::PathBuf;
/// Bound compiler processes and choose isolated temporary names without reusing prior oracle
/// artifacts.
use std::time::{SystemTime, UNIX_EPOCH};

/// Classify PostgreSQL OID constants so fixture bindgen uses the same checked-wrapper boundary
/// as the real binding build.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Synthetic C source inspected as an original header, keeping this test's semantic input
/// explicit.
const HEADER: &str = r#"
typedef enum DemandEnum { DemandZero = 0, DemandOne = 1 } DemandEnum;
typedef enum DemandOther { DemandOtherZero = 0, DemandOtherTwo = 2 } DemandOther;
typedef enum DemandHidden { DemandHiddenZero = 0, DemandHiddenOne = 1 } DemandHidden;
typedef enum DemandAddressEnum { DemandAddressZero = 0, DemandAddressOne = 1 } DemandAddressEnum;
typedef enum AliasDemandEnum { AliasDemandZero = 0, AliasDemandSeven = 7 } AliasDemandEnum;
typedef enum AliasDemandUnused { AliasDemandUnusedZero = 0 } AliasDemandUnused;
typedef struct DemandAliasRecord { AliasDemandEnum value; } DemandAliasRecord;
static inline DemandAddressEnum demand_address_only(DemandAddressEnum value) { return value; }
static inline AliasDemandEnum demand_alias_prototype(AliasDemandEnum value) { return value; }
typedef int (*DemandNoArgs)(void);
typedef int (*DemandUnary)(int);
typedef unsigned char (*DemandByte)(unsigned char);
typedef int (*DemandBinary)(int, int);
typedef void (*DemandVoid)(void);
typedef DemandUnary (*DemandFactory)(void);
typedef DemandFactory (*DemandOuter)(void);
typedef struct DemandVoidRecord { DemandVoid callback; } DemandVoidRecord;
extern DemandNoArgs demand_fixed;
extern DemandUnary demand_unary;
extern DemandByte demand_byte;
extern DemandOuter demand_outer;
extern unsigned int demand_void_count;
void demand_void_callback(void);
void demand_void_named(void);
#define DEMAND_FIXED() (demand_fixed())
#define DEMAND_CALL(fn,x) (fn(x))
#define DEMAND_UNARY_VALUE() (demand_unary)
#define DEMAND_BYTE_VALUE() (demand_byte)
#define DEMAND_READ(p) (*(p))
#define DEMAND_INDEX(p,i) ((p)[i])
#define DEMAND_HIDDEN_SIZE() (sizeof(DemandHidden *))
#define DEMAND_VOID(pointer, condition) ((condition) ? (pointer)->callback() : demand_void_named())
#define DEMAND_FACTORY() (demand_outer())
#define DEMAND_ADDRESS_ONLY() (&demand_address_only)
#define DEMAND_ADDRESS_SIZE() (sizeof(&demand_address_only))
#define DEMAND_ADDRESS_CALL() (demand_address_only(DemandAddressOne))
#define DEMAND_CALL_SIZE() (sizeof(demand_address_only(DemandAddressOne)))
#define DEMAND_OPEN_SCALAR(value) ((value) != 0)
#define DEMAND_NATIVE_SIZE(value) (sizeof(value))
#define DEMAND_TYPE_SIZE(type) (sizeof(type *))
#define DEMAND_TYPE_CAST(type, value) ((type)(value))
#define DEMAND_TYPE_CAST_PROVEN(type, value) ((void) _Alignof(type *), (type)(value))
#define DEMAND_ALIAS_CAST(value) ((AliasDemandEnum)(value))
#define DEMAND_ALIAS_FIELD(pointer) ((pointer)->value)
#define DEMAND_ALIAS_PROTO(value) (demand_alias_prototype(value))
"#;

/// Fixture binding or native-support source paired with the unchanged C oracle.
const NATIVE: &str = r#"
static int demand_answer(void) { return 7; }
static int demand_add(int value) { return value + 3; }
static unsigned char demand_add_byte(unsigned char value) { return value + 5; }
static DemandUnary demand_factory(void) { return demand_add; }
static DemandFactory demand_outer_factory(void) { return demand_factory; }
DemandNoArgs demand_fixed = demand_answer;
DemandUnary demand_unary = demand_add;
DemandByte demand_byte = demand_add_byte;
DemandOuter demand_outer = demand_outer_factory;
DemandMany demand_many = demand_many_target;
unsigned int demand_void_count;
void demand_void_callback(void) { demand_void_count += 1; }
void demand_void_named(void) { demand_void_count += 10; }
"#;

/// Own a synthetic header and its temporary directory so profile-sensitive generation has an
/// isolated source of C facts.
struct TemporaryHeader(
    /// Owned fixture path used for isolated inputs and cleanup.
    PathBuf,
);

/// Write an owned synthetic header whose source and compiler inputs can be varied
/// independently.
impl TemporaryHeader {
    /// Create owned, uniquely named fixture storage so this test's headers and compiler outputs
    /// cannot collide with another invocation.
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let directory = std::env::temp_dir()
            .join(format!("pgrx-capability-demands-{}-{nonce}", std::process::id()));
        fs::create_dir(&directory).expect("create isolated capability header directory");
        let header = Self(directory.join("demands.h"));
        // A valid signature beyond common fixed arity tables must retain support.
        let parameters = (0..40).map(|index| format!("int a{index}")).collect::<Vec<_>>();
        let values = (0..40).map(|index| index.to_string()).collect::<Vec<_>>();
        let sum = (0..40).map(|index| format!("a{index}")).collect::<Vec<_>>();
        let source = format!(
            "{HEADER}\ntypedef int (*DemandMany)({});\n\
             static inline int demand_many_target({}) {{ return {}; }}\n\
             extern DemandMany demand_many;\n\
             #define DEMAND_MANY() (demand_many({}))\n",
            vec!["int"; 40].join(", "),
            parameters.join(", "),
            sum.join(" + "),
            values.join(", "),
        );
        fs::write(&header.0, source).expect("write original capability macros");
        header
    }
}

/// Release only temporary artifacts owned by this fixture, including on failed compiler or
/// assertion paths.
impl Drop for TemporaryHeader {
    /// Remove only this fixture's owned temporary storage after the test or oracle completes.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(self.0.parent().expect("created header directory"));
    }
}

/// Checks that selected callback operations and open enum operands match original c.
#[test]
fn selected_callback_operations_and_open_enum_operands_match_original_c() {
    let header = TemporaryHeader::new();
    let scanner = MacroScanner::new().expect("libclang must be available");
    let mut arguments = vec!["-std=c17".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "capability SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend =
        inspect(&scanner, &header.0, &arguments, None).expect("inspect C capability macros");
    let all_names = [
        "DEMAND_FIXED",
        "DEMAND_CALL",
        "DEMAND_UNARY_VALUE",
        "DEMAND_BYTE_VALUE",
        "DEMAND_READ",
        "DEMAND_INDEX",
        "DEMAND_HIDDEN_SIZE",
        "DEMAND_VOID",
        "DEMAND_FACTORY",
        "DEMAND_ADDRESS_ONLY",
        "DEMAND_ADDRESS_SIZE",
        "DEMAND_ADDRESS_CALL",
        "DEMAND_CALL_SIZE",
        "DEMAND_OPEN_SCALAR",
        "DEMAND_NATIVE_SIZE",
        "DEMAND_TYPE_SIZE",
        "DEMAND_TYPE_CAST",
        "DEMAND_TYPE_CAST_PROVEN",
        "DEMAND_ALIAS_CAST",
        "DEMAND_ALIAS_FIELD",
        "DEMAND_ALIAS_PROTO",
        "DEMAND_MANY",
    ];
    let session = AnalysisSession::prepare(&scanner, &frontend, &all_names)
        .expect("analyze original capability macros");
    assert!(matches!(
        session.analyze("DEMAND_TYPE_CAST").status,
        pgrx_c_macros::AnalysisStatus::Skipped { reason }
            if reason.code == pgrx_c_macros::SkipReasonCode::TypeParameter
    ));
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.0.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Demand.*")
        .allowlist_type("AliasDemand.*")
        .allowlist_var("demand_.*")
        .allowlist_function("demand_.*")
        .rustified_enum("Demand.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate fresh capability bindings")
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    assert_eq!(catalog.enums.len(), 4, "only the Demand-prefixed enums are Rust objects");
    assert!(matches!(
        catalog.types["AliasDemandEnum"].target,
        pgrx_c_macros::RustBindingType::Integer { .. }
    ));
    let hidden = &frontend.declarations().types["DemandHidden"];
    assert!(
        !frontend
            .declarations()
            .type_shapes
            .contains_key(&format!("{} *", hidden.canonical_spelling)),
        "the macro-only pointer must exercise nominal dependency synthesis"
    );
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    for (names, calls, signatures, rust_body, c_body) in [
        (
            vec!["DEMAND_FIXED"],
            1,
            Some(1),
            "unsafe { println!(\"{}\", DEMAND_FIXED!().get()); }",
            "printf(\"%d\\n\", DEMAND_FIXED());",
        ),
        (
            vec!["DEMAND_MANY"],
            1,
            Some(1),
            "unsafe { println!(\"{}\", DEMAND_MANY!().get()); }",
            "printf(\"%d\\n\", DEMAND_MANY());",
        ),
        (
            vec!["DEMAND_FACTORY"],
            1,
            Some(2),
            "unsafe { println!(\"{}\", usize::from(DEMAND_FACTORY!().get().is_some())); }",
            "printf(\"%d\\n\", DEMAND_FACTORY() != 0);",
        ),
        (
            vec!["DEMAND_ADDRESS_ONLY"],
            0,
            Some(1),
            "println!(\"{}\", usize::from(DEMAND_ADDRESS_ONLY!().get().is_some()));",
            "printf(\"%d\\n\", DEMAND_ADDRESS_ONLY() != 0);",
        ),
        (
            vec!["DEMAND_ADDRESS_SIZE"],
            0,
            Some(1),
            "println!(\"{}\", DEMAND_ADDRESS_SIZE!().get());",
            "printf(\"%zu\\n\", DEMAND_ADDRESS_SIZE());",
        ),
        (
            vec!["DEMAND_ADDRESS_CALL"],
            0,
            Some(0),
            "unsafe { println!(\"{}\", DEMAND_ADDRESS_CALL!().get()); }",
            "printf(\"%u\\n\", (unsigned) DEMAND_ADDRESS_CALL());",
        ),
        (
            vec!["DEMAND_CALL_SIZE"],
            0,
            Some(0),
            "println!(\"{}\", DEMAND_CALL_SIZE!().get());",
            "printf(\"%zu\\n\", DEMAND_CALL_SIZE());",
        ),
        (
            vec!["DEMAND_CALL", "DEMAND_UNARY_VALUE", "DEMAND_BYTE_VALUE"],
            2,
            None,
            "unsafe { println!(\"{}\", DEMAND_CALL!(DEMAND_UNARY_VALUE!(), 5_i32).get()); println!(\"{}\", DEMAND_CALL!(DEMAND_BYTE_VALUE!(), 5_i32).get()); }",
            "printf(\"%d\\n%d\\n\", DEMAND_CALL(DEMAND_UNARY_VALUE(), 5), DEMAND_CALL(DEMAND_BYTE_VALUE(), 5));",
        ),
        (
            vec!["DEMAND_UNARY_VALUE"],
            0,
            Some(1),
            "unsafe { println!(\"{}\", usize::from(DEMAND_UNARY_VALUE!().get().is_some())); }",
            "printf(\"%d\\n\", DEMAND_UNARY_VALUE() != 0);",
        ),
        (
            vec!["DEMAND_READ", "DEMAND_UNARY_VALUE"],
            0,
            None,
            "unsafe { println!(\"{}\", usize::from(DEMAND_READ!(DEMAND_READ!(DEMAND_UNARY_VALUE!())).get().is_some())); }",
            "printf(\"%d\\n\", DEMAND_READ(DEMAND_READ(DEMAND_UNARY_VALUE())) != 0);",
        ),
        (
            vec!["DEMAND_READ"],
            0,
            None,
            "let mut value = DemandEnum::DemandOne; unsafe { println!(\"{}\", DEMAND_READ!(&raw mut value).get()); }",
            "DemandEnum value = DemandOne; printf(\"%u\\n\", (unsigned) DEMAND_READ(&value));",
        ),
        (
            vec!["DEMAND_INDEX"],
            0,
            None,
            "let mut values = [DemandOther::DemandOtherZero, DemandOther::DemandOtherTwo]; unsafe { println!(\"{}\", DEMAND_INDEX!(values.as_mut_ptr(), 1_i32).get()); }",
            "DemandOther values[] = { DemandOtherZero, DemandOtherTwo }; printf(\"%u\\n\", (unsigned) DEMAND_INDEX(values, 1));",
        ),
        (
            vec!["DEMAND_HIDDEN_SIZE"],
            0,
            Some(0),
            "println!(\"{}\", DEMAND_HIDDEN_SIZE!().get());",
            "printf(\"%zu\\n\", DEMAND_HIDDEN_SIZE());",
        ),
        (
            vec!["DEMAND_VOID"],
            1,
            None,
            "unsafe { demand_void_count = 0; let mut value = DemandVoidRecord { callback: Some(demand_void_callback) }; DEMAND_VOID!(&raw mut value, 1_i32); DEMAND_VOID!(&raw mut value, 0_i32); let count = demand_void_count; println!(\"{count}\"); }",
            "demand_void_count = 0; DemandVoidRecord value = { demand_void_callback }; DEMAND_VOID(&value, 1); DEMAND_VOID(&value, 0); printf(\"%u\\n\", demand_void_count);",
        ),
        (
            vec!["DEMAND_OPEN_SCALAR"],
            0,
            None,
            "let alias: AliasDemandEnum = 7; println!(\"{}\\n{}\", DEMAND_OPEN_SCALAR!(alias).get(), DEMAND_OPEN_SCALAR!(DemandEnum::DemandOne).get());",
            "AliasDemandEnum alias = AliasDemandSeven; printf(\"%d\\n%d\\n\", DEMAND_OPEN_SCALAR(alias), DEMAND_OPEN_SCALAR(DemandOne));",
        ),
        (
            vec!["DEMAND_READ"],
            0,
            None,
            "let mut alias: AliasDemandEnum = 7; unsafe { println!(\"{}\\n{}\", DEMAND_READ!(&raw mut alias).get(), DEMAND_READ!(&raw const alias).get()); }",
            "AliasDemandEnum alias = AliasDemandSeven; const AliasDemandEnum *readonly = &alias; printf(\"%u\\n%u\\n\", (unsigned) DEMAND_READ(&alias), (unsigned) DEMAND_READ(readonly));",
        ),
        (
            vec!["DEMAND_NATIVE_SIZE"],
            0,
            None,
            "let alias: AliasDemandEnum = 7; println!(\"{}\\n{}\", DEMAND_NATIVE_SIZE!(alias).get(), DEMAND_NATIVE_SIZE!(DemandEnum::DemandOne).get());",
            "AliasDemandEnum alias = AliasDemandSeven; DemandEnum native = DemandOne; printf(\"%zu\\n%zu\\n\", DEMAND_NATIVE_SIZE(alias), DEMAND_NATIVE_SIZE(native));",
        ),
        (
            vec!["DEMAND_TYPE_SIZE"],
            0,
            None,
            "println!(\"{}\\n{}\", DEMAND_TYPE_SIZE!(AliasDemandEnum).get(), DEMAND_TYPE_SIZE!(DemandEnum).get());",
            "printf(\"%zu\\n%zu\\n\", DEMAND_TYPE_SIZE(AliasDemandEnum), DEMAND_TYPE_SIZE(DemandEnum));",
        ),
        (
            vec!["DEMAND_TYPE_CAST_PROVEN"],
            0,
            None,
            "println!(\"{}\\n{}\", DEMAND_TYPE_CAST_PROVEN!(AliasDemandEnum, 7_i32).get(), DEMAND_TYPE_CAST_PROVEN!(DemandEnum, 7_i32).get());",
            "printf(\"%u\\n%u\\n\", (unsigned) DEMAND_TYPE_CAST_PROVEN(AliasDemandEnum, 7), (unsigned) DEMAND_TYPE_CAST_PROVEN(DemandEnum, 7));",
        ),
        (
            vec!["DEMAND_ALIAS_CAST"],
            0,
            None,
            "println!(\"{}\", DEMAND_ALIAS_CAST!(7_i32).get());",
            "printf(\"%u\\n\", (unsigned) DEMAND_ALIAS_CAST(7));",
        ),
        (
            vec!["DEMAND_ALIAS_FIELD"],
            0,
            None,
            "let mut record = DemandAliasRecord { value: 7 }; unsafe { println!(\"{}\\n{}\", DEMAND_ALIAS_FIELD!(&raw mut record).get(), DEMAND_ALIAS_FIELD!(&raw const record).get()); }",
            "DemandAliasRecord record = { AliasDemandSeven }; const DemandAliasRecord *readonly = &record; printf(\"%u\\n%u\\n\", (unsigned) DEMAND_ALIAS_FIELD(&record), (unsigned) DEMAND_ALIAS_FIELD(readonly));",
        ),
        (
            vec!["DEMAND_ALIAS_PROTO"],
            0,
            None,
            "unsafe { let alias: AliasDemandEnum = 7; println!(\"{}\", DEMAND_ALIAS_PROTO!(alias).get()); }",
            "AliasDemandEnum alias = AliasDemandSeven; printf(\"%u\\n\", (unsigned) DEMAND_ALIAS_PROTO(alias));",
        ),
    ] {
        let artifact = emit_support_artifact_with_bindings(&session, &names, &catalog)
            .expect("generate operation-specific native support");
        assert_eq!(artifact.rust.matches("::Call<").count(), calls, "{names:?}");
        if let Some(signatures) = signatures {
            assert_eq!(
                artifact.rust.matches("::NativeFunctionSignature for").count(),
                signatures,
                "{names:?}"
            );
        }
        if names == ["DEMAND_ADDRESS_ONLY"] || names == ["DEMAND_ADDRESS_SIZE"] {
            assert!(!artifact.rust.contains("pub unsafe fn Inline_"), "unused callable wrapper");
            assert!(!artifact.rust.contains("fn raw_inline_"), "unused native call declaration");
            assert!(
                !artifact
                    .c_source
                    .lines()
                    .any(|line| line.contains("__pgrx_inline_") && line.contains(") {")),
                "unused original-C call wrapper"
            );
            assert!(artifact.rust.contains("needs_drop"), "native storage validation remains");
            assert!(artifact.c_source.contains("_Static_assert"), "C layout validation remains");
            assert!(
                !artifact.rust.contains("NativeType for crate::DemandAddressEnum"),
                "address storage does not require enum operand bridges"
            );
        }
        if names == ["DEMAND_ADDRESS_CALL"] || names == ["DEMAND_CALL_SIZE"] {
            assert!(
                artifact.rust.contains("pub unsafe fn Inline_"),
                "calls retain callable support"
            );
        }
        let enum_identities = match names.as_slice() {
            [
                "DEMAND_OPEN_SCALAR"
                | "DEMAND_READ"
                | "DEMAND_NATIVE_SIZE"
                | "DEMAND_TYPE_SIZE"
                | "DEMAND_TYPE_CAST_PROVEN",
            ] => Some(4),
            ["DEMAND_ALIAS_CAST" | "DEMAND_ALIAS_PROTO"] => Some(5),
            ["DEMAND_ALIAS_FIELD"] => Some(1),
            _ => None,
        };
        if let Some(expected) = enum_identities {
            assert_eq!(
                artifact.rust.matches("::EnumIdentity for").count(),
                expected,
                "native enum objects and explicit nominal dependencies: {names:?}"
            );
        }
        let mut rust = format!(
            "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
            artifact.rust
        );
        for emission in emit_batch_with_bindings(&session, &names, &catalog).unwrap() {
            let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
                panic!("capability macro must emit: {emission:?}");
            };
            rust.push_str(&definition);
        }
        let native = format!("{NATIVE}\n{}", artifact.c_source);
        let generated = rust_oracle::run_rust_linked(
            &format!("{rust}\nfn main() {{ {rust_body} }}"),
            &profile.compiler.executable,
            &header.0,
            &native,
            &arguments,
        );
        let original = oracle::run_c(
            &profile.compiler.executable,
            &header.0,
            &format!("#include <stdio.h>\n{native}\nint main(void) {{ {c_body} return 0; }}"),
            &arguments,
            true,
        );
        assert_eq!(generated, original, "demand-selected C macro semantics: {names:?}");
    }
}
