//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, IntegerKind, MacroScanner, emit_batch_with_bindings,
    emit_support_with_bindings, inspect, pg_sys_integer_bridges,
};
use std::path::PathBuf;

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

const FUNCTIONS: &str = "\
Datum datum_identity(Datum value) { return value; }\n\
Datum datum_next(Datum value) { return value + sizeof(unsigned int); }\n\
unsigned int datum_read(Datum value) { return *(unsigned int *) value; }\n";

#[test]
fn actual_datum_storage_preserves_native_integer_abi_and_pointer_roundtrips() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/datum_oracle.h");
    let mut arguments = vec!["-std=c17".into(), "-O2".into()];
    if cfg!(target_os = "macos") {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "sdk",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let names = ["DATUM_IDENTITY", "DATUM_NEXT", "DATUM_READ", "DATUM_POINTER", "DATUM_INTEGER"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .blocklist_type("Datum|Oid|TransactionId")
        .allowlist_function("datum_.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .unwrap()
        .to_string();
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let pgrx_c_macros::TypeCategory::Integer(kind) =
        frontend.declarations().types["Datum"].category
    else {
        panic!("native Datum integer");
    };
    assert_eq!(kind, IntegerKind::UnsignedLong, "supported LP64 target");
    catalog.integer_storage.insert("Datum".into(), kind);
    let support = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let datum = directory.join("../pgrx-pg-sys/src/submodules/datum.rs").canonicalize().unwrap();
    let mut rust = format!(
        "\
#![allow(non_snake_case, non_camel_case_types, dead_code)]\n\
#[path = {support:?}] pub mod __pgrx_c_macros;\n\
#[path = {datum:?}] mod datum; pub use datum::Datum;\n\
pub struct NullableDatum {{ pub value: Datum, pub isnull: bool }}\n\
pub unsafe fn palloc(_: usize) -> *mut core::ffi::c_void {{ panic!(\"the LP64 oracle never allocates a by-value Datum\") }}\n\
#[derive(Clone, Copy)] pub struct Oid(u32); impl Oid {{ pub fn to_u32(self)->u32 {{self.0}} pub fn from_u32(value:u32)->Self {{Self(value)}} }}\n\
#[derive(Clone, Copy)] pub struct TransactionId(u32); impl TransactionId {{ pub fn into_inner(self)->u32 {{self.0}} pub fn from_inner(value:u32)->Self {{Self(value)}} }}\n\
{bindings}\n"
    );
    rust.push_str(&pg_sys_integer_bridges(&frontend).unwrap());
    rust.push_str(&emit_support_with_bindings(&session, &names, &catalog).unwrap());
    for emission in emit_batch_with_bindings(&session, &names, &catalog).unwrap() {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("{}: {:?}", emission.analysis.name, emission.status);
        };
        rust.push_str(&definition);
    }
    rust.push_str(
        r#"
fn main() {
    let mut values = [17_u32, 99_u32];
    let pointer = values.as_mut_ptr();
    let datum = Datum::from(pointer);
    // SAFETY: The array stays live and every C read uses one initialized element.
    unsafe {
        println!("{}", DATUM_READ!(datum).get());
        println!("{}", DATUM_READ!(DATUM_NEXT!(datum_identity(datum))).get());
        println!("{}", *(DATUM_POINTER!(DATUM_IDENTITY!(datum)).get() as *mut u32));
        println!("{}", DATUM_READ!(DATUM_INTEGER!(pointer)).get());
        let next = DATUM_POINTER!(DATUM_NEXT!(datum)).get() as *mut u32;
        println!("{}", next.offset_from(pointer));
        *next = 123;
        println!("{}", values[1]);
        for bits in [0_usize, 1, 0xFFFFFFFF, usize::MAX] {
            println!("{}", DATUM_IDENTITY!(Datum::from(bits)).get());
        }
    }
}
"#,
    );
    let original = format!(
        "{FUNCTIONS}\n{}",
        r#"
#include <stdio.h>
int main(void) {
    unsigned int values[] = {17,99};
    unsigned int *pointer = values;
    Datum value = (Datum) pointer;
    printf("%u\n", DATUM_READ(value));
    printf("%u\n", DATUM_READ(DATUM_NEXT(datum_identity(value))));
    printf("%u\n", *(unsigned int *) DATUM_POINTER(DATUM_IDENTITY(value)));
    printf("%u\n", DATUM_READ(DATUM_INTEGER(pointer)));
    unsigned int *next = DATUM_POINTER(DATUM_NEXT(value));
    printf("%ld\n", (long) (next-pointer));
    *next = 123;
    printf("%u\n", values[1]);
    Datum bits[] = {0,1,0xFFFFFFFF,~(Datum)0};
    for(unsigned int i=0;i<4;i++) printf("%llu\n", (unsigned long long) DATUM_IDENTITY(bits[i]));
}
"#
    );
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let original =
        oracle::run_c(&profile.compiler.executable, &header, &original, &arguments, true);
    let generated = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header,
        FUNCTIONS,
        &arguments,
    );
    assert_eq!(original.lines().count(), 10);
    assert_eq!(generated, original);
}
