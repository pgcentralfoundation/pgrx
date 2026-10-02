//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, IntegerBinding, IntegerBindingRepresentation,
    IntegerValue, MacroScanner, SkipReasonCode, emit_batch_with_bindings, emit_with_bindings,
    inspect, pg_sys_integer_bridges,
};
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());
const NAMES: &[&str] = &[
    "REF_HUGE_VALID",
    "REF_ALIASES",
    "REF_PRECEDENCE",
    "REF_ENUM_ADD",
    "REF_OID_EQUAL",
    "REF_BOOL_VALUE",
    "REF_MIN_VALUE",
    "REF_HEX_VALUE",
    "REF_HUGE_WRAPPER",
    "REF_HUGE_OUTER",
    "REF_ENUM_HIDDEN_ADD",
];

#[test]
fn uncertain_constant_folding_preserves_domain_checks() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/binding_references.h");
    let names = ["REF_SIGNED_OVERFLOW_VALUE", "REF_DIV_OVERFLOW_VALUE", "REF_BAD_SHIFT_VALUE"];
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    for (flag, wrapping) in [("-fno-wrapv", false), ("-fwrapv", true)] {
        let frontend = inspect(&scanner, &header, &["-std=c11".into(), flag.into()], None).unwrap();
        let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
        assert!(!session.integer_constants().contains_key("REF_DIV_OVERFLOW"));
        assert!(!session.integer_constants().contains_key("REF_BAD_SHIFT"));
        assert_eq!(session.integer_constants().contains_key("REF_SIGNED_OVERFLOW"), wrapping);
        let bindings = BindingCatalog::default();
        let mut rust = format!("#[path = {support:?}] pub mod __pgrx_c_macros;\n");
        for name in &names {
            rust.push_str(&source(&session, name, &bindings));
        }
        rust.push_str("fn main() { assert!(std::panic::catch_unwind(|| REF_DIV_OVERFLOW_VALUE!()).is_err()); assert!(std::panic::catch_unwind(|| REF_BAD_SHIFT_VALUE!()).is_err());");
        if wrapping {
            rust.push_str("assert_eq!(REF_SIGNED_OVERFLOW_VALUE!().get(), i32::MIN);");
        } else {
            rust.push_str(
                "assert!(std::panic::catch_unwind(|| REF_SIGNED_OVERFLOW_VALUE!()).is_err());",
            );
        }
        rust.push('}');
        rust_oracle::run_rust(&rust);
    }
}

fn source(session: &AnalysisSession<'_>, name: &str, bindings: &BindingCatalog) -> String {
    match emit_with_bindings(session, name, bindings).status {
        EmissionStatus::Emitted { rust, .. } => rust,
        other => panic!("{name}: {other:?}"),
    }
}

#[test]
fn references_and_fallbacks_preserve_original_c_values_types_and_precedence() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/binding_references.h");
    let frontend =
        inspect(&scanner, &header, &["-std=c11".into(), "-fwrapv".into()], None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, NAMES).unwrap();
    let mut bindings = BindingCatalog::default();
    for name in
        ["REF_HUGE", "REF_FIRST", "REF_ALIAS", "REF_OID", "REF_BOOL", "REF_MIN", "REF_ENUM_HIDDEN"]
    {
        let constant = &session.integer_constants()[name];
        bindings.integer_constants.insert(
            name.into(),
            IntegerBinding {
                path: vec![name.into()],
                value: constant.value,
                representation: if name == "REF_OID" {
                    IntegerBindingRepresentation::Oid
                } else {
                    IntegerBindingRepresentation::Primitive
                },
            },
        );
    }
    bindings.integer_constants.insert(
        "REF_ENUM".into(),
        IntegerBinding {
            path: vec!["RefEnum".into(), "REF_ENUM".into()],
            value: IntegerValue::Unsigned(7),
            representation: IntegerBindingRepresentation::Primitive,
        },
    );
    bindings.integer_constants.insert(
        "REF_UNGROUPED".into(),
        IntegerBinding {
            path: vec!["REF_UNGROUPED".into()],
            value: IntegerValue::Unsigned(3),
            representation: IntegerBindingRepresentation::Primitive,
        },
    );
    let huge = source(&session, "REF_HUGE_VALID", &bindings);
    assert!(huge.split_once("=> {").unwrap().1.contains("$crate::REF_HUGE"));
    let aliases = source(&session, "REF_ALIASES", &bindings);
    let body = aliases.split_once("=> {").unwrap().1;
    assert!(body.contains("$crate::REF_ALIAS") && body.contains("$crate::REF_FIRST"));
    let precedence = source(&session, "REF_PRECEDENCE", &bindings);
    let body = precedence.split_once("=> {").unwrap().1;
    assert!(!body.contains("$crate::REF_UNGROUPED"));
    assert!(body.contains("/* PGRX: REF_UNGROUPED") && body.contains("grouping"));

    let mut wrong = bindings.clone();
    wrong.integer_constants.get_mut("REF_HUGE").unwrap().value = IntegerValue::Unsigned(0);
    let emissions = emit_batch_with_bindings(&session, NAMES, &wrong);
    for (name, expected_code) in [
        ("REF_HUGE_VALID", SkipReasonCode::BindingValueMismatch),
        ("REF_HUGE_WRAPPER", SkipReasonCode::DependencySkipped),
        ("REF_HUGE_OUTER", SkipReasonCode::DependencySkipped),
    ] {
        let emission = emissions.iter().find(|emission| emission.analysis.name == name).unwrap();
        let EmissionStatus::Skipped { reason } = &emission.status else {
            panic!("a binding mismatch must skip {name}, not expand it");
        };
        assert_eq!(reason.code, expected_code);
        assert!(reason.message.contains("REF_HUGE's value; bindgen=0, clang=9223372036854775807"));
        assert!(reason.message.contains(&format!("skipping macro `{name}`")));
        if expected_code == SkipReasonCode::DependencySkipped {
            assert!(reason.message.contains("depends on skipped macro"));
        }
    }
    assert!(
        emissions
            .iter()
            .find(|emission| emission.analysis.name == "REF_ALIASES")
            .is_some_and(|emission| matches!(emission.status, EmissionStatus::Emitted { .. }))
    );
    // A matching object binding can hide the bad enum from the analyzed tree.
    // The shared graph must still propagate the skipped function through it.
    let mut wrong_enum = bindings.clone();
    wrong_enum.integer_constants.get_mut("REF_ENUM").unwrap().value = IntegerValue::Unsigned(0);
    assert!(matches!(
        emit_with_bindings(&session, "REF_ENUM_HIDDEN_ADD", &wrong_enum).status,
        EmissionStatus::Emitted { .. }
    ));
    let emissions = emit_batch_with_bindings(&session, NAMES, &wrong_enum);
    let hidden =
        emissions.iter().find(|emission| emission.analysis.name == "REF_ENUM_HIDDEN_ADD").unwrap();
    let EmissionStatus::Skipped { reason } = &hidden.status else {
        panic!("the hidden dependency must skip");
    };
    assert_eq!(reason.code, SkipReasonCode::DependencySkipped);
    assert!(reason.message.contains("REF_ENUM_HIDDEN") && reason.message.contains("REF_ENUM_ADD"));
    assert!(reason.message.contains("bindgen=0, clang=7"));
    // Only request the final caller: its unrequested callee must still seed the
    // same failure from the full active environment.
    let subset = AnalysisSession::prepare(&scanner, &frontend, &["REF_ENUM_HIDDEN_ADD"]).unwrap();
    let emitted = emit_batch_with_bindings(&subset, &["REF_ENUM_HIDDEN_ADD"], &wrong_enum);
    assert!(matches!(&emitted[0].status, EmissionStatus::Skipped { reason }
        if reason.code == SkipReasonCode::DependencySkipped
            && reason.message.contains("REF_ENUM_ADD")
            && reason.message.contains("bindgen=0, clang=7")));
    let mut ungrouped = bindings.clone();
    ungrouped.integer_constants.get_mut("REF_UNGROUPED").unwrap().value = IntegerValue::Unsigned(0);
    assert!(matches!(emit_with_bindings(&session, "REF_PRECEDENCE", &ungrouped).status,
        EmissionStatus::Skipped { reason } if reason.code == SkipReasonCode::BindingValueMismatch));
    let missing = source(&session, "REF_ENUM_ADD", &BindingCatalog::default());
    assert!(
        missing.contains("/* PGRX: REF_ENUM") && missing.contains("no integer constant binding")
    );

    let literal = source(&session, "REF_HEX_VALUE", &BindingCatalog::default());
    assert!(literal.contains("0xFFFFFFFFu32"), "fallback preserves original literal spelling");
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!("#[path = {support:?}] pub mod __pgrx_c_macros;\n");
    rust.push_str("#[derive(Clone, Copy)] pub struct Oid(u32); impl Oid { pub fn to_u32(self) -> u32 { self.0 } }\n#[derive(Clone, Copy)] pub struct TransactionId(u32); impl TransactionId { pub fn into_inner(self) -> u32 { self.0 } }\n");
    rust.push_str(&pg_sys_integer_bridges(&frontend).unwrap());
    rust.push_str("pub const REF_HUGE: u64 = 9223372036854775807; pub const REF_FIRST: u32 = 17; pub const REF_ALIAS: u32 = 17; pub const REF_OID: Oid = Oid(42); pub const REF_BOOL: bool = true; pub const REF_MIN: i64 = i64::MIN; pub const REF_UNGROUPED: i32 = 3; pub const REF_ENUM_HIDDEN: i32 = 7; pub mod RefEnum { pub const REF_ENUM: u32 = 7; }\n");
    for name in NAMES {
        rust.push_str(&source(&session, name, &bindings));
    }
    rust.push_str("fn main() { assert_eq!(REF_HUGE_VALID!(__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLong>::new(9223372036854775807)).get(), 1); assert_eq!(REF_HUGE_VALID!(__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLong>::new(9223372036854775808)).get(), 0); assert_eq!(REF_ALIASES!(1_u32).get(), 35); assert_eq!(REF_PRECEDENCE!(5_i32).get(), 7); assert_eq!(REF_ENUM_ADD!(1_i32).get(), 8); assert_eq!(REF_OID_EQUAL!(42_u32).get(), 1); assert!(REF_BOOL_VALUE!().get()); assert_eq!(REF_MIN_VALUE!().get(), i64::MIN); }");
    rust_oracle::run_rust(&rust);
    // Compile the independently resolved fallback, with deliberately absent Rust
    // symbols, to prove that an unavailable binding cannot affect its value.
    let missing_huge = source(&session, "REF_HUGE_VALID", &BindingCatalog::default());
    let rust = format!(
        "#[path = {support:?}] pub mod __pgrx_c_macros;\n{missing_huge}\n{missing}\nfn main() {{ assert_eq!(REF_HUGE_VALID!(__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLong>::new(9223372036854775807)).get(), 1); assert_eq!(REF_HUGE_VALID!(__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLong>::new(9223372036854775808)).get(), 0); assert_eq!(REF_ENUM_ADD!(1_i32).get(), 8); }}"
    );
    rust_oracle::run_rust(&rust);
    oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        r#"
#define IS_TYPE(value, type) _Static_assert(_Generic((value), type: 1, default: 0), #value)
IS_TYPE(REF_HUGE, RefSize);
IS_TYPE(REF_ENUM, int);
IS_TYPE(REF_ALIASES(1U), unsigned int);
IS_TYPE(REF_PRECEDENCE(5), int);
IS_TYPE(REF_BOOL_VALUE(), _Bool);
IS_TYPE(REF_MIN_VALUE(), long long);
_Static_assert(REF_HUGE == 9223372036854775807ULL, "unsigned size limit");
_Static_assert(REF_HUGE_VALID(9223372036854775807ULL) == 1, "valid limit");
_Static_assert(REF_HUGE_VALID(9223372036854775808ULL) == 0, "above limit");
_Static_assert(REF_ALIASES(1U) == 35U, "distinct aliases");
_Static_assert(REF_PRECEDENCE(5) == 7, "textual object precedence");
_Static_assert(REF_ENUM_ADD(1) == 8, "enumerator");
_Static_assert(REF_OID_EQUAL(42U) == 1, "OID constant");
_Static_assert(REF_BOOL_VALUE() == 1, "boolean");
_Static_assert(REF_MIN_VALUE() == (-9223372036854775807LL - 1LL), "minimum signed long long");
"#,
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );
}
