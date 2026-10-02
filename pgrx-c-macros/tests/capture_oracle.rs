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
    AnalysisSession, EmissionStatus, MacroScanner, ParameterOrigin, emit_batch_with_bindings,
    emit_support_artifact_with_bindings, inspect,
};
use std::path::PathBuf;

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

#[test]
fn caller_scope_identifiers_become_explicit_hygienic_context_arguments() {
    let scanner = MacroScanner::new().expect("libclang required");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/capture_oracle.h");
    let mut args = vec!["-std=c17".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "SDK",
        );
        args.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend = inspect(&scanner, &header, &args, None).expect("inspect capture fixture");
    let names = ["CAP_READ", "CAP_SET", "CAP_COUNTER", "CAP_LAZY", "CAP_SIZE", "CAP_TWO"];
    let session =
        AnalysisSession::prepare(&scanner, &frontend, &names).expect("prepare capture fixture");
    let native = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Capture")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .unwrap()
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&native).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let artifact = emit_support_artifact_with_bindings(&session, &names, &catalog).unwrap();
    assert!(artifact.c_source.is_empty());
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={support:?}]pub mod __pgrx_c_macros;\n{native}\n{}",
        artifact.rust
    );
    for emission in emit_batch_with_bindings(&session, &names, &catalog) {
        let captures = emission
            .analysis
            .parameters
            .iter()
            .filter(|parameter| parameter.origin == ParameterOrigin::FreeIdentifier)
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            captures,
            match emission.analysis.name.as_str() {
                "CAP_COUNTER" => vec!["counter"],
                "CAP_TWO" => vec!["left", "right"],
                _ => vec!["scope"],
            }
        );
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("capture must emit: {emission:?}")
        };
        rust.push_str(&definition);
    }
    let original = r#"
#include <stdio.h>
int main(void) {
 struct Capture object={{3,5,7}}; struct Capture*scope=&object; int counter=11;
 int a=CAP_READ(1); int b=CAP_SET(2,19); int c=CAP_COUNTER(4);
 int left=23,right=9;
 printf("%d %d %d %d %d %zu %d\n",a,b,c,object.slots[2],CAP_LAZY(0),CAP_SIZE(),CAP_TWO());
}
"#;
    let generated = r#"
fn main() {
 let mut object=Capture { slots:[3,5,7] }; let scope=&raw mut object; let mut counter=11_i32;
 // SAFETY: The local record and counter remain live, initialized, aligned,
 // and exclusively available for every generated access and mutation.
 let (a,b,c)=unsafe {(CAP_READ!(1,scope).get(),CAP_SET!(2,19,scope).get(),CAP_COUNTER!(4,counter).get())};
 let mut evaluated=0;
 // SAFETY: The false branch performs no access, including the context input.
 let lazy=unsafe {CAP_LAZY!(0,{evaluated+=1;scope}).get()};
 assert_eq!(evaluated,0);
 let size=CAP_SIZE!({evaluated+=1;scope}).get(); assert_eq!(evaluated,0);
 let left=23_i32;let right=9_i32;
 println!("{} {} {} {} {} {} {}",a,b,c,object.slots[2],lazy,size,CAP_TWO!(left,right).get());
}
"#;
    let profile = frontend.profile();
    let args = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(
        rust_oracle::run_rust(&format!("{rust}\n{generated}")),
        oracle::run_c(&profile.compiler.executable, &header, original, &args, true)
    );
    let diagnostic =
        rust_oracle::reject_rust(&format!("{rust}\nfn main() {{ let _=CAP_READ!(0); }}"));
    assert!(diagnostic.contains("argument"), "{diagnostic}");
}
