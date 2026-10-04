//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)] // This oracle requires no native support library.
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, ParameterRole, generate_with_bindings, inspect,
};
use std::path::PathBuf;

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

#[test]
fn field_tokens_select_only_compiler_verified_record_capabilities() {
    let scanner = MacroScanner::new().expect("libclang required");
    let header =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/field_parameter_oracle.h");
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
        "FIELD_READ",
        "FIELD_WRITE",
        "FIELD_NEXT",
        "FIELD_OFFSET",
        "FIELD_SIZE",
        "FIELD_ADDRESS",
        "FIELD_BAD_ROLE",
    ];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let native = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Field.*")
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
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    assert!(generated.support.c_source.is_empty());
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={support:?}]pub mod __pgrx_c_macros;\n{native}\n{}",
        generated.support.rust
    );
    for emission in generated.macros {
        if emission.analysis.name == "FIELD_BAD_ROLE" {
            let EmissionStatus::Skipped { reason } = emission.status else {
                panic!("mixed identifier/value role must skip")
            };
            assert!(reason.message.contains("field identifier"), "{reason:?}");
            continue;
        }
        assert!(
            emission
                .analysis
                .parameters
                .iter()
                .any(|parameter| parameter.roles == [ParameterRole::Identifier])
        );
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("field parameter must emit: {emission:?}")
        };
        rust.push_str(&definition);
    }
    let original = r#"
#include <stdio.h>
int main(void) {
 FieldItem item={3,{5,7,11,13}}; FieldState state; int *cell=0;
 state.left=&item; state.right=0; state.index=1; state.offset=0;
 int a=FIELD_READ(&state,index); int b=FIELD_WRITE(&state,index,2);
 int c=*FIELD_NEXT(cell,state,left,index);
 int d=FIELD_NEXT(cell,state,right,index)==0;
 int e=FIELD_OFFSET(&state,offset)==0;
 FIELD_WRITE(&state,offset,4u);
 int f=FIELD_OFFSET(&state,offset)==(void *)((char *)&state+4);
 printf("%d %d %d %d %d %d %zu %d\n",a,b,c,d,e,f,FIELD_SIZE(&item,elements),FIELD_ADDRESS(&state,index)==&state.index);
}
"#;
    let consumer = r#"
fn main() {
 let mut item=FieldItem {length:3,elements:[5,7,11,13]};
 let mut state=core::mem::MaybeUninit::<FieldState>::uninit();
 let p=state.as_mut_ptr(); let mut cell=core::ptr::null_mut::<i32>();
 // SAFETY: Only accessed fields are initialized. Raw projections never read
 // unread. The record and array are live, aligned and exclusively accessible.
 unsafe {
  core::ptr::addr_of_mut!((*p).left).write(&raw mut item);
  core::ptr::addr_of_mut!((*p).right).write(core::ptr::null_mut());
  core::ptr::addr_of_mut!((*p).index).write(1);
  core::ptr::addr_of_mut!((*p).offset).write(0);
  let a=FIELD_READ!(p,index).get(); let b=FIELD_WRITE!(p,index,2).get();
  let c=*FIELD_NEXT!(cell,(*p),left,index).get();
  let d=FIELD_NEXT!(cell,(*p),right,index).get().is_null();
  let e=FIELD_OFFSET!(p,offset).get().is_null();
  FIELD_WRITE!(p,offset,4u32);
  let f=FIELD_OFFSET!(p,offset).get()==p.cast::<u8>().add(4).cast();
  let mut effects=0;
  let size=FIELD_SIZE!({effects+=1;&raw mut item},elements).get(); assert_eq!(effects,0);
  let address=FIELD_ADDRESS!(p,index).get()==core::ptr::addr_of_mut!((*p).index);
  println!("{} {} {} {} {} {} {} {}",a,b,c,u8::from(d),u8::from(e),u8::from(f),size,u8::from(address));
 }
}
"#;
    let profile = frontend.profile();
    let args = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(
        rust_oracle::run_rust(&format!("{rust}\n{consumer}")),
        oracle::run_c(&profile.compiler.executable, &header, original, &args, true)
    );
    for consumer in [
        "fn main(){let _=unsafe{FIELD_READ!(core::ptr::null_mut::<FieldState>(),missing)};}",
        "fn main(){let _=unsafe{FIELD_READ!(core::ptr::null_mut::<FieldState>(),index+1)};}",
        "fn main(){let _=unsafe{FIELD_READ!(core::ptr::null_mut::<FieldItem>(),index)};}",
        "fn main(){let _=unsafe{FIELD_WRITE!(core::ptr::null::<FieldState>(),index,1)};}",
    ] {
        let diagnostic = rust_oracle::reject_rust(&format!("{rust}\n{consumer}"));
        assert!(
            diagnostic.contains("field")
                || diagnostic.contains("argument")
                || diagnostic.contains("trait bound"),
            "{diagnostic}"
        );
    }
}
