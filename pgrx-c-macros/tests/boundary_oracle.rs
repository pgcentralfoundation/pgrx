//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)] // This oracle requires no native support library.
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, InvocationContract, MacroScanner,
    generate_with_bindings, inspect,
};
use std::path::PathBuf;

#[test]
fn non_atomic_replacements_require_an_explicit_c_invocation_boundary() {
    let scanner = MacroScanner::new().expect("libclang required");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/boundary_oracle.h");
    let mut args = vec!["-std=c17".into(), "-ffp-contract=off".into(), "-O2".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "SDK",
        );
        args.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend = inspect(&scanner, &header, &args, None).unwrap();
    let names = [
        "BOUND_SUM",
        "BOUND_NEG",
        "BOUND_CAST",
        "BOUND_SET",
        "BOUND_SELECT",
        "BOUND_ATOMIC",
        "BOUND_ALIAS",
        "BOUND_ALIAS_GROUP",
        "BOUND_LVALUE",
        "BOUND_SIZE",
        "BOUND_USE",
    ];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let generated = generate_with_bindings(&session, &names, &BindingCatalog::default()).unwrap();
    assert!(generated.support.c_source.is_empty());
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={support:?}]pub mod __pgrx_c_macros;\n{}",
        generated.support.rust
    );
    for emission in generated.macros {
        if !["BOUND_USE", "BOUND_ALIAS_GROUP", "BOUND_SIZE"]
            .contains(&emission.analysis.name.as_str())
        {
            assert_eq!(
                emission.analysis.invocation,
                InvocationContract::ExplicitExpressionBoundary
            );
        }
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("explicit invocation must emit: {emission:?}")
        };
        rust.push_str(&definition);
    }
    let original = r#"
#include <stdio.h>
_Static_assert(BOUND_SUM(2) * 3 == 5, "unparenthesized body changes caller precedence");
_Static_assert((BOUND_SUM(2)) * 3 == 9, "parenthesized invocation is a different contract");
int main(void) {
 int value=12; int result=0; int effects=0;
 int a=(BOUND_SUM(2))*3; int b=(BOUND_NEG(7));
 int c=(BOUND_CAST(&value))==&value;
 int d=(BOUND_SET(result,(++effects,12)));
 int e=(BOUND_SELECT(0)); int f=(BOUND_ATOMIC((1?7:2)));
 int g=(BOUND_ALIAS(4)); int h=BOUND_USE((BOUND_SUM(2)));
 int alias=BOUND_ALIAS_GROUP(4);
 unsigned long size=BOUND_SIZE((BOUND_SUM(++effects)));
 int nested=(BOUND_SET((BOUND_LVALUE(&result)),21));
 printf("%d %d %d %d %d %d %d %d %d %d %d %lu %d\n",a,b,c,d,e,f,g,h,result,effects,alias,size,nested);
}
"#;
    let consumer = r#"
fn main() {
 let mut value=12i32; let mut result=0i32; let mut effects=0;
 let a=BOUND_SUM!(@__pgrx_c_expression;2).get()*3;
 let b=BOUND_NEG!(@__pgrx_c_expression;7).get();
 let c=BOUND_CAST!(@__pgrx_c_expression;&raw mut value).get()==&raw mut value;
 // SAFETY: The initialized local result is live, aligned, and exclusively
 // writable. The argument is evaluated once before its assignment.
 let d=unsafe{BOUND_SET!(@__pgrx_c_expression;result,{effects+=1;12}).get()};
 let e=BOUND_SELECT!(@__pgrx_c_expression;0).get();
 let f=BOUND_ATOMIC!(@__pgrx_c_expression;(if true {7} else {2})).get();
 let g=BOUND_ALIAS!(@__pgrx_c_expression;4).get();
 let h=BOUND_USE!(BOUND_SUM!(@__pgrx_c_expression;2)).get();
 let alias=BOUND_ALIAS_GROUP!(4).get();
 let size=BOUND_SIZE!(BOUND_SUM!(@__pgrx_c_expression;{effects+=1;2})).get();
 // SAFETY: The nested macro preserves result's same exclusive live place.
 let nested=unsafe{BOUND_SET!(@__pgrx_c_expression;BOUND_LVALUE!(@__pgrx_c_expression;&raw mut result),21).get()};
 println!("{} {} {} {} {} {} {} {} {} {} {} {} {}",a,b,u8::from(c),d,e,f,g,h,result,effects,alias,size,nested);
}
"#;
    let profile = frontend.profile();
    let args = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(
        rust_oracle::run_rust(&format!("{rust}\n{consumer}")),
        oracle::run_c(&profile.compiler.executable, &header, original, &args, true)
    );
    let diagnostic =
        rust_oracle::reject_rust(&format!("{rust}\nfn main(){{let _=BOUND_SUM!(2);}}"));
    assert!(diagnostic.contains("explicit parenthesized invocation"), "{diagnostic}");
    let diagnostic = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main(){{let mut x=0i32;let _=BOUND_SET!(@__pgrx_c_expression;x,3);}}"
    ));
    assert!(diagnostic.contains("unsafe"), "{diagnostic}");
    let diagnostic = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main(){{let _=BOUND_ATOMIC!(@__pgrx_c_expression;1+2);}}"
    ));
    assert!(diagnostic.contains("argument"), "{diagnostic}");
}
