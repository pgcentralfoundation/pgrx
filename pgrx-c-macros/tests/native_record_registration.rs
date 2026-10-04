//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check private native record registration across crate boundaries.
//!
//! A producer exposes generated record input behavior to a consumer while sealing
//! its registration trait. Valid use must compile, but downstream crates must not
//! forge registrations that bypass the compiler-owned layout contract.

/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

/// Read original fixtures and manage only the owned inputs and outputs used by generation
/// checks.
use std::fs;
/// Keep fixture and generated-output locations explicit so consumer builds remain independent
/// of the working directory.
use std::path::PathBuf;
/// Invoke independent compilers and consumers and inspect their actual exit status rather than
/// trusting generated source alone.
use std::process::Command;
/// Bound compiler processes and choose isolated temporary names without reusing prior oracle
/// artifacts.
use std::time::{SystemTime, UNIX_EPOCH};

/// Consumer registration declarations used to exercise native record sealing and valid
/// downstream inputs.
const REGISTRATIONS: &str = r#"
use __pgrx_c_macros::expression::{self, CRecord, COpaque, IntoExpression, NativeType, RecordValue};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyRecord(pub u32);
impl expression::NativeRecord for CopyRecord {}

#[repr(C)]
pub struct NonCopyRecord(pub u32);
impl expression::NativeRecord for NonCopyRecord {}

#[derive(Clone, Copy)]
pub struct Opaque;
impl __pgrx_c_macros::sealed::Sealed for Opaque {}
impl NativeType for Opaque { type Marker = COpaque<Self>; }

// Existing hand-written bridges remain usable without record registration.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct LegacyRecord(pub u32);
impl __pgrx_c_macros::sealed::Sealed for LegacyRecord {}
impl NativeType for LegacyRecord { type Marker = CRecord<Self>; }
impl IntoExpression for LegacyRecord {
    type Value = RecordValue<Self>;
    fn into_expression(self) -> Self::Value { RecordValue::new(self) }
}
"#;

/// Consumer imports required to test registration from a separate crate.
const IMPORTS: &str = r#"
extern crate pgrx_record_registration_runtime as renamed;
use renamed::{CopyRecord, LegacyRecord, NonCopyRecord, Opaque};
use renamed::__pgrx_c_macros::{self, expression::*};
"#;

/// Checks that record registration is private but native inputs work across crates.
#[test]
fn record_registration_is_private_but_native_inputs_work_across_crates() {
    let directory = TemporaryDirectory::new();
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .expect("locate the full production runtime");
    let runtime = directory.0.join("runtime.rs");
    fs::write(
        &runtime,
        format!(
            "#![deny(warnings)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{REGISTRATIONS}"
        ),
    )
    .expect("write crate-owned record registrations");
    let library = directory.0.join("libpgrx_record_registration_runtime.rlib");
    rust_oracle::run_tool(
        Command::new("rustc")
            .args(["--edition=2024", "--crate-type=rlib"])
            .args(["--crate-name", "pgrx_record_registration_runtime"])
            .arg(&runtime)
            .arg("-o")
            .arg(&library),
        "compile_record_runtime",
    );
    let dependency = format!("pgrx_record_registration_runtime={}", library.display());
    let consumer = directory.0.join("consumer.rs");
    fs::write(
        &consumer,
        format!(
            r#"{IMPORTS}
fn main() {{
    let value: RecordValue<CopyRecord> = input(CopyRecord(11));
    assert_eq!(value.get().0, 11);
    let mut record = NonCopyRecord(17);
    let mutable: Pointer<CRecord<NonCopyRecord>> = input(&raw mut record);
    let readonly: Pointer<CRecord<NonCopyRecord>, ReadOnly> = input(&raw const record);
    assert_eq!(mutable.get().cast_const(), readonly.get());
    let raw: RawRecordValue<NonCopyRecord> = input(core::mem::MaybeUninit::uninit());
    assert_eq!(core::mem::size_of_val(&raw.get()), core::mem::size_of::<NonCopyRecord>());
    let opaque: Pointer<COpaque<Opaque>> = input(core::ptr::null_mut::<Opaque>());
    assert!(opaque.get().is_null());
    let legacy: RecordValue<LegacyRecord> = input(LegacyRecord(23));
    assert_eq!(legacy.get().0, 23);
    let integer: __pgrx_c_macros::CValue<__pgrx_c_macros::CInt> = input(
        __pgrx_c_macros::CValue::<CNullConstant<__pgrx_c_macros::CInt>>::new(0)
    );
    assert_eq!(integer.get(), 0);
}}
"#
        ),
    )
    .expect("write renamed downstream record consumer");
    let executable = directory.0.join("consumer");
    rust_oracle::run_tool(
        Command::new("rustc")
            .args(["--edition=2024", "--extern"])
            .arg(&dependency)
            .arg(&consumer)
            .arg("-o")
            .arg(&executable),
        "compile_record_consumer",
    );
    rust_oracle::run_tool(&mut Command::new(&executable), "execute_record_consumer");

    for (name, body, code, context) in [
        (
            "register_local",
            "struct Local(u32); impl renamed::__pgrx_c_macros::expression::NativeRecord for Local {}",
            "E0603",
            "NativeRecord",
        ),
        (
            "register_foreign",
            "impl renamed::__pgrx_c_macros::expression::NativeRecord for CopyRecord {}",
            "E0603",
            "NativeRecord",
        ),
        (
            "manual_native_bypass",
            "struct Local(u32); impl NativeType for Local { type Marker = CRecord<Self>; }",
            "E0277",
            "Sealed",
        ),
        ("non_copy_input", "fn rejected() { let _ = input(NonCopyRecord(7)); }", "E0277", "Copy"),
        ("opaque_input", "fn rejected() { let _ = input(Opaque); }", "E0277", "Opaque"),
    ] {
        let source = directory.0.join(format!("{name}.rs"));
        fs::write(&source, format!("{IMPORTS}\n{body}\nfn main() {{}}"))
            .expect("write rejected downstream registration or input");
        let diagnostic = rust_oracle::reject_tool(
            Command::new("rustc")
                .args(["--edition=2024", "--emit=metadata", "--extern"])
                .arg(&dependency)
                .arg(&source)
                .arg("-o")
                .arg(directory.0.join(format!("{name}.rmeta"))),
            name,
        );
        assert!(
            diagnostic.contains(code) && diagnostic.contains(context),
            "{name} must reject the specific registration/input contract: {diagnostic}"
        );
    }
}

/// Own isolated compiler inputs and outputs so oracle runs cannot reuse stale artifacts or
/// leave a growing target tree.
struct TemporaryDirectory(
    /// Owned fixture path used for isolated inputs and cleanup.
    PathBuf,
);

/// Allocate isolated compiler artifacts with process-local uniqueness and deterministic cleanup
/// ownership.
impl TemporaryDirectory {
    /// Create owned, uniquely named fixture storage so this test's headers and compiler outputs
    /// cannot collide with another invocation.
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir()
            .join(format!("pgrx-record-registration-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).expect("create isolated compiler fixture directory");
        Self(path)
    }
}

/// Release only temporary artifacts owned by this fixture, including on failed compiler or
/// assertion paths.
impl Drop for TemporaryDirectory {
    /// Remove only this fixture's owned temporary storage after the test or oracle completes.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
