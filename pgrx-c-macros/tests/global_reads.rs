//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! When the catalog names a thread check, a macro reads an integer, enum or data pointer
//! global after that check, so the read needs no `unsafe` from its caller, and neither does
//! an assertion whose condition reads one. Writes, dereferences and function pointer reads
//! still need the caller's `unsafe`. A macro that reads several such globals checks once,
//! before the first read it runs.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos")
))]

/// Reconcile fixture bindings using production's actual storage collector.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Locate the native SDK arguments.
#[path = "support/oracle.rs"]
#[allow(dead_code)]
mod oracle;
/// Compile generated Rust consumers with bounded tool execution.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, FrontendOutput, MacroGeneration, MacroScanner,
    generate_with_bindings, inspect,
};
use std::path::PathBuf;

/// Preserve the production OID classification boundary in the shared fixture collector.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Macros that read a global and need no `unsafe` once the catalog names a thread check.
const CHECKED: &[&str] =
    &["GLOBAL_COUNT", "GLOBAL_PENDING", "GLOBAL_FLAG", "GLOBAL_POINTER", "GLOBAL_BUSY"];
/// Macros in which several reads share one thread check.
const SHARED: &[&str] = &["GLOBAL_TOTAL", "GLOBAL_ANY"];
/// A statement body whose reads share one check. It writes its operand, which needs `unsafe`.
const COPY: &str = "GLOBAL_COPY";
/// A macro that reads one global after an operand it evaluates only for its effects.
const AFTER: &str = "GLOBAL_AFTER";
/// Macros that need the caller's `unsafe` either way.
const UNSAFE: &[&str] = &["GLOBAL_POINTEE", "GLOBAL_HOOKED", "GLOBAL_SET"];
/// The check whose condition reads a global.
const CHECK: &str = "GLOBAL_IN_RANGE";

/// The fixture header.
fn header() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/global_reads.h")
}

/// Bind the fixture's globals and enum as production does.
fn bind(frontend: &FrontendOutput) -> String {
    bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header().to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Global.*")
        .allowlist_var("Global.*")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate bindings")
        .to_string()
}

/// Each macro's emitted Rust and whether it needs the caller's `unsafe`.
fn emitted<'a>(generated: &'a MacroGeneration, name: &str) -> (&'a str, bool) {
    let emission = generated.macros.iter().find(|emission| emission.analysis.name == name);
    match emission.map(|emission| &emission.status) {
        Some(EmissionStatus::Emitted { rust, caller_unsafe, .. }) => (rust, *caller_unsafe),
        status => panic!("{name}: {status:?}"),
    }
}

#[test]
fn checked_global_reads_need_no_unsafe() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let mut arguments = oracle::native_arguments();
    // The check runs only against headers that enable assertions.
    arguments.extend(["-std=c17".into(), "-DUSE_ASSERT_CHECKING=1".into()]);
    let frontend = inspect(&scanner, &header(), &arguments, None).expect("inspect definitions");
    let names = CHECKED
        .iter()
        .chain(SHARED)
        .chain(UNSAFE)
        .chain([&COPY, &AFTER, &CHECK])
        .copied()
        .collect::<Vec<_>>();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).expect("prepare roots");
    let bindings = bind(&frontend);
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );

    // Without a thread check every read needs `unsafe`. The check stays in its macro, which
    // then needs `unsafe` too.
    let unchecked = generate_with_bindings(&session, &names, &catalog).unwrap();
    for name in CHECKED.iter().chain(UNSAFE).chain([&CHECK]) {
        assert!(emitted(&unchecked, name).1, "{name} needs unsafe without a thread check");
    }
    assert!(emitted(&unchecked, CHECK).0.contains("assertion_failure"));

    catalog.thread_check = Some(vec!["thread_check".into()]);
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    for name in CHECKED.iter().chain(SHARED).chain([&AFTER]) {
        let (rust, caller_unsafe) = emitted(&generated, name);
        assert!(!caller_unsafe && rust.contains("load_global"), "{name}: {rust}");
        // A macro with one read checks there. One with several declares the check they
        // share once, in the arm that yields its value.
        let shared = usize::from(SHARED.contains(name));
        assert_eq!(rust.matches("ThreadCheck::new").count(), shared, "{name}: {rust}");
    }
    let (rust, caller_unsafe) = emitted(&generated, COPY);
    assert!(caller_unsafe && rust.matches("ThreadCheck::new").count() == 1, "{rust}");
    for name in UNSAFE {
        assert!(emitted(&generated, name).1, "{name} still needs unsafe");
    }
    let (rust, caller_unsafe) = emitted(&generated, CHECK);
    assert!(!caller_unsafe && rust.contains("assertion_failure"), "{rust}");

    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn, unused_unsafe)]\n#![allow(non_snake_case,non_camel_case_types,non_upper_case_globals,unused_parens,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
        generated.support.rust
    );
    for name in &names {
        rust.push_str(emitted(&generated, name).0);
    }
    rust.push_str(include_str!("fixtures/global_reads.rs"));
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let output = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header(),
        &format!(
            "{}\n#include <stdlib.h>\nint GlobalCount = 5;\nvolatile int GlobalPending = 1;\n_Bool GlobalFlag = 1;\nstatic int GlobalTarget = 9;\nint *GlobalPointer = &GlobalTarget;\nGlobalMode GlobalState = GLOBAL_BUSY;\nGlobalHook GlobalHookValue = 0;\nvoid ExceptionalCondition(const char *c, const char *f, int l) {{ abort(); }}\n",
            generated.support.c_source
        ),
        &arguments,
    );
    // The last column counts thread checks so far. A macro with one checked read checks
    // once, including the pointer read inside the dereference. The function pointer read
    // and the write do not. The three reads of `GLOBAL_TOTAL` check once. `GLOBAL_ANY`
    // checks only when it runs a read, and `GLOBAL_COPY` once for its two statements. The
    // discard arm of `GLOBAL_TOTAL` checks for each of its three reads, before the read
    // of `GLOBAL_AFTER` itself.
    assert_eq!(
        output,
        "count\t5\t1\npending\t1\t2\nflag\ttrue\t3\npointer\t4\nbusy\t1\t5\nin range\t1\t6\npointee\t9\nhooked\t0\ncount\t3\t8\ntotal\t5\t9\nany\t0\t9\nany\t1\t10\ncopy\t4\t11\nafter\t3\t15\n"
    );
    session.verify_inputs().expect("original profile stayed unchanged");
}
