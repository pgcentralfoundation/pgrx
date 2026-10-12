//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check independent object-constant proofs, diagnostic isolation and coherent inputs.

/// Run original C expressions independently of the API's typed proof sources.
#[path = "support/oracle.rs"]
mod oracle;
/// Resolve the native SDK through the same bounded helper used by other oracle tests.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    FrontendError, IntegerKind, IntegerValue, MacroScanner, TypeCategory, inspect,
    probe_integer_object_constants,
};
use std::fmt::Write;
use std::path::PathBuf;
use std::sync::Mutex;

/// Keep one safe libclang runtime owner in this test process.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Supply a native C17 profile, including the SDK required by macOS's compiler driver.
fn native_arguments() -> Vec<String> {
    let mut arguments = vec!["-std=c17".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "sdk",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    arguments
}

/// Own a mutable original header so the coherence check never modifies repository inputs.
struct OriginalHeader {
    /// Unique directory containing only this test's original compiler input.
    directory: PathBuf,
    /// Main-file path shared by inspection, proof and the independent C oracle.
    path: PathBuf,
}

impl OriginalHeader {
    /// Save a complete original fixture with process/time-isolated ownership.
    fn new(source: &str) -> Self {
        let nonce =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let directory = std::env::temp_dir()
            .join(format!("pgrx-object-constants-{}-{nonce}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();
        let path = directory.join("original.h");
        std::fs::write(&path, source).unwrap();
        Self { directory, path }
    }
}

impl Drop for OriginalHeader {
    /// Remove only the fixture directory allocated by this test.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

/// Ensure more malformed siblings than the old retry budget cannot hide later valid constants.
#[test]
fn bad_object_siblings_do_not_starve_verified_integer_constants() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut source = String::from(
        "volatile int object_runtime_value;\n\
         enum ObjectEnum { OBJECT_ENUM_VALUE = 37 };\n\
         typedef unsigned long ObjectWord;\n",
    );
    let mut rejected_names = Vec::new();
    // These sort before every valid name. Original header parsing succeeds,
    // but invoking their independent constant witnesses must fail separately.
    for index in 0..40 {
        let name = format!("A_MALFORMED_{index:03}");
        writeln!(source, "#define {name} (1 + )").unwrap();
        rejected_names.push(name);
        let name = format!("B_NONCONSTANT_{index:03}");
        writeln!(source, "#define {name} (object_runtime_value + {index})").unwrap();
        rejected_names.push(name);
    }
    let mut valid_names = Vec::new();
    for index in 0..96 {
        let name = format!("Z_VALID_{index:03}");
        writeln!(source, "#define {name} ((int){index} + 1000)").unwrap();
        valid_names.push(name);
    }
    source.push_str(
        "#define Z_SIGNED_MIN (-9223372036854775807LL - 1LL)\n\
         #define Z_UNSIGNED_MAX 18446744073709551615ULL\n\
         #define Z_ENUM OBJECT_ENUM_VALUE\n\
         #define Z_SIZE sizeof(unsigned long long)\n\
         #define Z_TYPEDEF ((ObjectWord)7)\n\
         #define Z_CHARACTER 'A'\n\
         #define Z_CHAR_CAST ((char)'A')\n\
         #define Z_SQLSTATE (((('2' - '0') & 0x3F)) | ((('3' - '0') & 0x3F) << 6) | ((('5' - '0') & 0x3F) << 12) | ((('0' - '0') & 0x3F) << 18) | ((('5' - '0') & 0x3F) << 24))\n\
         #define Z_FILE_SPELLING_SIZE sizeof(\"__FILE__\")\n\
         #define OBJECT_LONG(value) value##L\n\
         #define OBJECT_LONG_LONG(value) value##LL\n\
         #define OBJECT_UNSIGNED_LONG_LONG(value) value##ULL\n\
         #define Z_PASTED_LONG OBJECT_LONG(2147483647)\n\
         #define Z_PASTED_LONG_LONG OBJECT_LONG_LONG(9223372036854775807)\n\
         #define Z_PASTED_SIGNED_MIN (-OBJECT_LONG_LONG(9223372036854775807) - OBJECT_LONG_LONG(1))\n\
         #define Z_PASTED_UNSIGNED_MAX OBJECT_UNSIGNED_LONG_LONG(18446744073709551615)\n\
         #define LOCATION_LINE __LINE__\n\
         #define LOCATION_FILE __FILE__\n\
         #define C_LOCATION_LINE __LINE__\n\
         #define C_LOCATION_LINE_NESTED (LOCATION_LINE + 1)\n\
         #define C_LOCATION_FILE_SIZE sizeof(__FILE__)\n\
         #define C_LOCATION_FILE_NESTED sizeof(LOCATION_FILE)\n\
         #define C_COUNTER __COUNTER__\n\
         #define C_FUNCTION(value) (value)\n",
    );
    valid_names.extend(
        [
            "Z_SIGNED_MIN",
            "Z_UNSIGNED_MAX",
            "Z_ENUM",
            "Z_SIZE",
            "Z_TYPEDEF",
            "Z_CHARACTER",
            "Z_CHAR_CAST",
            "Z_SQLSTATE",
            "Z_FILE_SPELLING_SIZE",
            "Z_PASTED_LONG",
            "Z_PASTED_LONG_LONG",
            "Z_PASTED_SIGNED_MIN",
            "Z_PASTED_UNSIGNED_MAX",
        ]
        .map(str::to_owned),
    );
    rejected_names.extend(
        [
            "C_LOCATION_LINE",
            "C_LOCATION_LINE_NESTED",
            "C_LOCATION_FILE_SIZE",
            "C_LOCATION_FILE_NESTED",
            "C_COUNTER",
            "C_FUNCTION",
            "C_ABSENT",
        ]
        .map(str::to_owned),
    );
    let header = OriginalHeader::new(&source);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header.path, &native_arguments(), None).unwrap();
    let names = rejected_names.iter().chain(&valid_names).cloned().collect::<Vec<_>>();
    let proof = probe_integer_object_constants(&scanner, &frontend, &names).unwrap();
    assert_eq!(
        proof.constants.len(),
        valid_names.len(),
        "bad siblings must not consume valid proof coverage: {:?}",
        proof.rejected
    );
    assert_eq!(proof.rejected.len(), rejected_names.len());
    for name in &rejected_names {
        assert!(
            proof.rejected.get(name).is_some_and(|reason| !reason.is_empty()),
            "{name} must retain an explained refusal"
        );
        assert!(!proof.constants.contains_key(name), "{name} must never receive a guessed value");
    }
    for name in [
        "C_LOCATION_LINE",
        "C_LOCATION_LINE_NESTED",
        "C_LOCATION_FILE_SIZE",
        "C_LOCATION_FILE_NESTED",
    ] {
        assert!(
            proof.rejected[name].contains("invocation location"),
            "{name}: {}",
            proof.rejected[name]
        );
    }
    assert_eq!(proof.constants["Z_SIGNED_MIN"].value, IntegerValue::Signed(i64::MIN));
    assert_eq!(
        proof.constants["Z_SIGNED_MIN"].ty.category,
        TypeCategory::Integer(IntegerKind::LongLong)
    );
    assert_eq!(proof.constants["Z_UNSIGNED_MAX"].value, IntegerValue::Unsigned(u64::MAX));
    assert_eq!(
        proof.constants["Z_UNSIGNED_MAX"].ty.category,
        TypeCategory::Integer(IntegerKind::UnsignedLongLong)
    );
    assert_eq!(proof.constants["Z_ENUM"].ty.category, TypeCategory::Integer(IntegerKind::Int));
    assert_eq!(
        proof.constants["Z_SIZE"].ty.category,
        TypeCategory::Integer(frontend.profile().target.size_type)
    );
    assert_eq!(
        proof.constants["Z_CHAR_CAST"].ty.category,
        TypeCategory::Integer(IntegerKind::Char)
    );
    // Closed suffix pasting is proved by the original C compiler, preserving
    // rank even when long and long long happen to share the same storage width.
    for (name, kind) in [
        ("Z_PASTED_LONG", IntegerKind::Long),
        ("Z_PASTED_LONG_LONG", IntegerKind::LongLong),
        ("Z_PASTED_SIGNED_MIN", IntegerKind::LongLong),
        ("Z_PASTED_UNSIGNED_MAX", IntegerKind::UnsignedLongLong),
    ] {
        assert_eq!(proof.constants[name].ty.category, TypeCategory::Integer(kind));
    }
    assert_eq!(proof.constants["Z_PASTED_SIGNED_MIN"].value, IntegerValue::Signed(i64::MIN));
    assert_eq!(proof.constants["Z_PASTED_UNSIGNED_MAX"].value, IntegerValue::Unsigned(u64::MAX));

    let mut recording = String::from(
        "#include <limits.h>\n#include <stdio.h>\n\
         #define KIND(v) _Generic((v), _Bool: \"Bool\", char: \"Char\", signed char: \"SignedChar\", unsigned char: \"UnsignedChar\", short: \"Short\", unsigned short: \"UnsignedShort\", int: \"Int\", unsigned int: \"UnsignedInt\", long: \"Long\", unsigned long: \"UnsignedLong\", long long: \"LongLong\", unsigned long long: \"UnsignedLongLong\")\n\
         int main(void) {\n",
    );
    for name in &valid_names {
        writeln!(recording, "printf(\"{name}\\t%s\\t%u\\t%llu\\n\", KIND({name}), (unsigned)(sizeof({name}) * CHAR_BIT), (unsigned long long)({name}));").unwrap();
    }
    recording.push_str("return 0; }\n");
    let profile = frontend.profile();
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header.path,
        &recording,
        &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    assert_eq!(original.lines().count(), valid_names.len());
    for row in original.lines() {
        let fields = row.split('\t').collect::<Vec<_>>();
        let constant = &proof.constants[fields[0]];
        let TypeCategory::Integer(kind) = constant.ty.category else {
            panic!("integer proof must retain its canonical identity")
        };
        assert_eq!(fields[1], format!("{kind:?}"), "{} exact C kind", fields[0]);
        assert_eq!(
            fields[2].parse::<u64>().unwrap(),
            constant.ty.size.unwrap() * u64::from(profile.target.char_bits),
            "{} exact C width",
            fields[0]
        );
        let value = match constant.value {
            IntegerValue::Signed(value) => value as u64,
            IntegerValue::Unsigned(value) => value,
        };
        assert_eq!(fields[3].parse::<u64>().unwrap(), value, "{} original C value", fields[0]);
    }
    // This test owns the input, and changes it only after every coherent proof
    // and independent oracle observation above. A later proof must refuse it.
    std::fs::write(&header.path, format!("{source}\n#define CHANGED_AFTER_INSPECTION 1\n"))
        .unwrap();
    let changed = probe_integer_object_constants(&scanner, &frontend, &valid_names).unwrap_err();
    assert!(
        matches!(changed, FrontendError::Environment(message) if message.contains("changed after inspection")),
        "modified inputs must require fresh inspection"
    );
}
