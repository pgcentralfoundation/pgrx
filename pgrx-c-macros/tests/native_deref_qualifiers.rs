//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check native dereference owner types and nested C qualifiers.
//!
//! A temporary header isolates const and volatile pointer shapes. Original C and
//! Rust observations, plus rejected consumers, ensure alias resolution and cast
//! precedence do not strip qualification at a deeper pointer level.
//!
//! These generated consumers use the runtime's Linux/macOS host family. Emission
//! still validates the inspected C ABI and flags; unsupported-profile checks remain portable.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]

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

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, generate_with_bindings, inspect,
};
use std::fs;
use std::path::PathBuf;
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
typedef struct DerefState { int scalar; int values[2]; } DerefState;
typedef struct DerefConstState { int scalar; const int values[2]; } DerefConstState;
#define DEREF_READ(state) ((state).scalar)
#define DEREF_ARRAY(state) ((state).values)
#define DEREF_ADD(value) ((value) + 3)
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
        let directory =
            std::env::temp_dir().join(format!("pgrx-native-deref-{}-{nonce}", std::process::id()));
        fs::create_dir(&directory).expect("create isolated native dereference header directory");
        let header = Self(directory.join("dereference.h"));
        fs::write(&header.0, HEADER).expect("write original native dereference macros");
        header
    }
}

/// Release only temporary artifacts owned by this fixture, including on failed compiler or
/// assertion paths.
impl Drop for TemporaryHeader {
    /// Remove only this fixture's owned temporary storage after the test or oracle completes.
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
        let _ = fs::remove_dir(self.0.parent().expect("created header directory"));
    }
}

/// Checks that native raw dereferences preserve qualification owners and precedence.
#[test]
fn native_raw_dereferences_preserve_qualification_owners_and_precedence() {
    let header = TemporaryHeader::new();
    let scanner = MacroScanner::new().expect("libclang must be available");
    let mut arguments = vec!["-std=c17".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "native dereference SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend = inspect(&scanner, &header.0, &arguments, None).unwrap();
    let names = ["DEREF_READ", "DEREF_ARRAY", "DEREF_ADD"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.0.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Deref.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate fresh native dereference bindings")
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    assert!(generated.support.c_source.is_empty(), "fixture requires no C access shims");
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
        generated.support.rust
    );
    for emission in &generated.macros {
        let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
            panic!("native dereference fixture must emit: {emission:?}");
        };
        rust.push_str(definition);
    }
    let c = r#"
    DerefState state = { 17, { 23, 31 } };
    DerefState *pointer = &state;
    const DerefState *constant = &state;
    int *mutable = DEREF_ARRAY(*pointer);
    mutable[1] = 37;
    const int *readonly = DEREF_ARRAY(*constant);
    printf("%d\n%d\n%d\n", DEREF_READ(*pointer), mutable[1], readonly[1]);
    DerefConstState const_field = { 41, { 43, 47 } };
    const int *field = DEREF_ARRAY(const_field);
    printf("%d\n", field[1]);
    DerefState owned = { 53, { 59, 61 } };
    printf("%d\n%d\n", DEREF_READ(owned), DEREF_READ(owned));
    DerefState temporary = { 67, { 71, 73 } };
    printf("%d\n", DEREF_READ(temporary));
    int number = 79;
    int *scalar = &number;
    printf("%d\n%d\n", DEREF_ADD(*scalar + 5), DEREF_ADD((*scalar)));
    "#;
    let body = r#"
    struct ObservedOwner<'a> {
        value: DerefState,
        events: &'a std::cell::RefCell<Vec<u8>>,
    }
    impl core::ops::Deref for ObservedOwner<'_> {
        type Target = DerefState;
        fn deref(&self) -> &DerefState {
            self.events.borrow_mut().push(1);
            &self.value
        }
    }
    impl Drop for ObservedOwner<'_> {
        fn drop(&mut self) { self.events.borrow_mut().push(3); }
    }
    fn observe(value: i32, events: &std::cell::RefCell<Vec<u8>>) {
        events.borrow_mut().push(2);
        assert_eq!(value, 83);
    }
    fn main() {
        let mut state = DerefState { scalar: 17, values: [23, 31] };
        let pointer = &raw mut state;
        let constant = &raw const state;
        // SAFETY: The initialized records remain live; mutable array storage is
        // accessed only through the raw mutable pointer during these operations.
        unsafe {
            let mutable: *mut i32 = DEREF_ARRAY!(((*pointer))).get();
            *mutable.add(1) = 37;
            let readonly: *const i32 = DEREF_ARRAY!(*(constant)).get();
            println!("{}\n{}\n{}", DEREF_READ!(*pointer).get(), *mutable.add(1), *readonly.add(1));
            let const_field = DerefConstState { scalar: 41, values: [43, 47] };
            let field: *const i32 = DEREF_ARRAY!(const_field).get();
            println!("{}", *field.add(1));
            let mut owned = Box::new(DerefState { scalar: 53, values: [59, 61] });
            println!("{}", DEREF_READ!(*owned).get());
            let reference = &mut *owned;
            println!("{}", DEREF_READ!(*reference).get());
            println!("{}", DEREF_READ!(*Box::new(DerefState { scalar: 67, values: [71, 73] })).get());
            assert_eq!(owned.scalar, 53, "the named Box was borrowed rather than moved");
            let mut number = 79_i32;
            let scalar = &raw mut number;
            println!("{}\n{}", DEREF_ADD!(*scalar + 5).get(), DEREF_ADD!((*scalar)).get());
            let events = std::cell::RefCell::new(Vec::new());
            observe(
                DEREF_READ!(*ObservedOwner {
                    value: DerefState { scalar: 83, values: [89, 97] },
                    events: &events,
                }).get(),
                &events,
            );
            assert_eq!(*events.borrow(), [1, 2, 3], "the temporary owner survives until the full expression completes");
        }
    }
    "#;
    let arguments = frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let original = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header.0,
        &format!("#include <stdio.h>\nint main(void) {{ {c} return 0; }}"),
        &arguments,
        true,
    );
    let actual = rust_oracle::run_rust(&format!("{rust}\n{body}"));
    assert_eq!(original.lines().count(), 9, "complete native dereference comparison");
    assert_eq!(actual, original, "native descriptors must preserve C value and qualification");

    for (label, body) in [
        (
            "raw const record",
            "let state = DerefState { scalar: 1, values: [2,3] }; let constant=&raw const state; unsafe { let _: *mut i32 = DEREF_ARRAY!(*constant).get(); }",
        ),
        (
            "const array member",
            "let mut state = DerefConstState { scalar: 1, values: [2,3] }; let pointer=&raw mut state; unsafe { let _: *mut i32 = DEREF_ARRAY!(*pointer).get(); }",
        ),
        (
            "named Rust reference",
            "let mut state = DerefState { scalar: 1, values: [2,3] }; let reference=&mut state; unsafe { let _: *mut i32 = DEREF_ARRAY!(*reference).get(); }",
        ),
    ] {
        let error = rust_oracle::reject_rust(&format!("{rust}\nfn main() {{ {body} }}"));
        assert!(error.contains("E0308"), "{label} must retain const qualification: {error}");
    }
}
