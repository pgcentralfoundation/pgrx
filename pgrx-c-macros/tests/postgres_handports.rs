//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Use existing port names to choose C-oracle coverage, not expected behavior.
//!
//! The test discovers handwritten pgrx function names and compares every emittable
//! counterpart across configured PostgreSQL versions with the original C macro.
//! This catches mistakes in either implementation without blessing an old port.
//!
//! These generated consumers use the actual inspected C profile on each native host.
//! Cross-profile checks retain original compiler facts without executing foreign code.

/// Select installed PostgreSQL header oracles from configured metadata.
#[path = "support/postgres.rs"]
mod installed;
/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, MacroScanner, SignedOverflow,
    generate_with_bindings,
};
use std::collections::BTreeSet;
use std::path::PathBuf;

/// Representative macro families used alongside discovered handport names for
/// original-PostgreSQL oracle coverage.
const TRIGGERS: &[&str] = &[
    "TRIGGER_FIRED_BY_INSERT",
    "TRIGGER_FIRED_BY_DELETE",
    "TRIGGER_FIRED_BY_UPDATE",
    "TRIGGER_FIRED_BY_TRUNCATE",
    "TRIGGER_FIRED_FOR_ROW",
    "TRIGGER_FIRED_FOR_STATEMENT",
    "TRIGGER_FIRED_BEFORE",
    "TRIGGER_FIRED_AFTER",
    "TRIGGER_FIRED_INSTEAD",
];

// Names establish coverage only. No Rust body, type, or result is an oracle.
/// Discover the old port names as a coverage selection only; their implementations do not
/// determine expected results.
fn handwritten_function_names() -> BTreeSet<String> {
    [
        include_str!("../../pgrx-pg-sys/src/port.rs"),
        include_str!("../../pgrx/src/trigger_support/mod.rs"),
        include_str!("../../pgrx/src/varlena.rs"),
        include_str!("../../pgrx-pg-sys/src/submodules/errcodes.rs"),
    ]
    .into_iter()
    .flat_map(str::lines)
    .filter_map(|line| {
        let declaration = line.strip_prefix("pub ").unwrap_or(line);
        let declaration = declaration.strip_prefix("const ").unwrap_or(declaration);
        let declaration = declaration.strip_prefix("unsafe ").unwrap_or(declaration);
        let function = declaration.strip_prefix("fn ")?;
        function.split(['(', '<']).next().map(str::to_ascii_lowercase)
    })
    .collect()
}

/// Checks that every emittable handport matches each original PostgreSQL version.
#[test]
fn every_emittable_handport_matches_each_original_postgres_version() {
    let installations = installed::configured();
    if installations.is_empty() {
        return;
    }
    let scanner = MacroScanner::new().expect("libclang must be available");
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let handwritten = handwritten_function_names();
    for postgres in installations {
        let major = postgres.pg_config().major_version().unwrap();
        let inspection_arguments = vec!["-O2".into()];
        // ELF link-time garbage collection requires a section for each unused
        // backend routine; these code-generation flags stay in the shared profile.
        #[cfg(any(target_os = "linux", target_os = "windows"))]
        let inspection_arguments = {
            let mut arguments = inspection_arguments;
            arguments.extend(["-ffunction-sections".into(), "-fdata-sections".into()]);
            arguments
        };
        let frontend = installed::inspect(&scanner, &postgres, &inspection_arguments);
        if !installed::supports_rust_comparisons(&scanner, &frontend) {
            continue;
        }
        let names = frontend
            .inventory()
            .macros
            .iter()
            .filter(|definition| handwritten.contains(&definition.name.to_ascii_lowercase()))
            .map(|definition| definition.name.as_str())
            .collect::<Vec<_>>();
        let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
        let mut rust =
            format!("#[path = {:?}]\npub mod __pgrx_c_macros;\n", support.to_str().unwrap());
        let generation =
            generate_with_bindings(&session, &names, &BindingCatalog::default()).unwrap();
        let artifact = generation.support;
        rust.push_str(&artifact.rust);
        let mut emitted = BTreeSet::new();
        for emission in generation.macros {
            if let EmissionStatus::Emitted { rust: generated, .. } = emission.status {
                let name = emission.analysis.name;
                assert!(emitted.insert(name), "a handport must have one primary definition");
                rust.push_str(&generated);
            }
        }
        let mut expected = BTreeSet::from([
            "TYPEALIGN",
            "MAXALIGN",
            "BufferIsLocal",
            "TransactionIdIsNormal",
            "VARATT_NOT_PAD_BYTE",
            "type_is_array",
            "PGSIXBIT",
            "MAKE_SQLSTATE",
        ]);
        if major == 15 {
            expected.insert("PageSizeIsValid");
            expected.insert("PageIsValid");
        }
        expected.extend(TRIGGERS.iter().copied());
        if major < 19 {
            expected.insert("VARTAG_IS_EXPANDED");
        }
        assert_eq!(
            emitted.iter().map(String::as_str).collect::<BTreeSet<_>>(),
            expected,
            "PG{major}: every emittable handport needs an original-C case"
        );

        let mut prefix = String::new();
        let wrapping = frontend.profile().signed_overflow == SignedOverflow::Wrapping;
        if wrapping {
            prefix.push_str("#define ORACLE_WRAP_ALIGN 1\n");
        }
        if major == 15 {
            prefix.push_str("#define ORACLE_PAGE_SIZE_MACRO 1\n");
        }
        if major < 19 {
            prefix.push_str("#define ORACLE_VARTAG_MACRO 1\n");
        }
        let profile = frontend.profile();
        let original = oracle::run_c(
            &profile.compiler.executable,
            &profile.header,
            &format!("{prefix}{}", include_str!("fixtures/handports_postgres.c")),
            &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            true,
        );
        let block_size = original
            .lines()
            .next()
            .and_then(|row| row.strip_prefix("BLOCK_SIZE\t"))
            .and_then(|value| value.parse::<u32>().ok())
            .expect("original C must report the configured block-size input");
        assert!(block_size > 0 && block_size < u32::MAX);
        let mut triggers = String::from("first.set(0); second.set(0);\n");
        for (kind, events) in [
            ("LOW", "(0_u32..256).collect::<Vec<_>>()"),
            (
                "HIGH",
                "[0x80000000_u32, 0xFFFFFF00, 0xFFFFFFEF, 0xFFFFFFFF].into_iter().flat_map(|mask| (0_u32..32).map(move |low| mask | low)).collect::<Vec<_>>()",
            ),
        ] {
            triggers
                .push_str(&format!("for (index, event) in {events}.into_iter().enumerate() {{\n"));
            for name in TRIGGERS {
                triggers.push_str(&format!(
                    "record(\"{name}_{kind}\", index as u32, {name}!(event), 0, 0);\n"
                ));
            }
            triggers.push_str("}\n");
        }
        triggers.push_str("let event_argument = || { first.set(first.get() + 1); 0x1C_u32 };\n");
        for name in TRIGGERS {
            triggers.push_str(&format!("first.set(0); second.set(0);\nlet value = {name}!(event_argument());\nrecord(\"{name}_EVAL\", 0, value, first.get(), second.get());\n"));
        }
        let (vartag_metadata, vartag_cases) = if major < 19 {
            let kind = original
                .lines()
                .find_map(|row| row.strip_prefix("VARTAG_KIND\t"))
                .expect("original C must identify the enum's compatible integer kind");
            let repr = match kind {
                "CChar" => {
                    if profile.target.char_is_signed {
                        "i8"
                    } else {
                        "u8"
                    }
                }
                "CSignedChar" => "i8",
                "CUnsignedChar" => "u8",
                "CShort" => "i16",
                "CUnsignedShort" => "u16",
                "CInt" => "i32",
                "CUnsignedInt" => "u32",
                "CLong" => {
                    if profile.target.integers[&pgrx_c_macros::IntegerKind::Long].bits == 32 {
                        "i32"
                    } else {
                        "i64"
                    }
                }
                "CUnsignedLong" => {
                    if profile.target.integers[&pgrx_c_macros::IntegerKind::UnsignedLong].bits == 32
                    {
                        "u32"
                    } else {
                        "u64"
                    }
                }
                "CLongLong" => "i64",
                "CUnsignedLongLong" => "u64",
                _ => panic!(
                    "original vartag enum kind {kind:?} is outside the tested integer input domain"
                ),
            };
            let cases = format!(
                r#"
first.set(0); second.set(0);
for tag in 0_u32..256 {{
    record("VARTAG_IS_EXPANDED_ENUM", tag, VARTAG_IS_EXPANDED!(CValue::<__pgrx_c_macros::{kind}>::new(tag as {repr})), 0, 0);
}}
for (index, tag) in [0x80000000_u32, 0xFFFFFFFE, 0xFFFFFFFF, 0x7FFFFFFF].into_iter().enumerate() {{
    record("VARTAG_IS_EXPANDED_HIGH", index as u32, VARTAG_IS_EXPANDED!(CValue::<__pgrx_c_macros::{kind}>::new(tag as {repr})), 0, 0);
}}
record("VARTAG_IS_EXPANDED_SIGNED", 0, VARTAG_IS_EXPANDED!(-1_i32), 0, 0);
record("VARTAG_IS_EXPANDED_BYTE", 0, VARTAG_IS_EXPANDED!(3_u8), 0, 0);
let vartag_argument = || {{ first.set(first.get() + 1); CValue::<__pgrx_c_macros::{kind}>::new(2 as {repr}) }};
let value = VARTAG_IS_EXPANDED!(vartag_argument());
record("VARTAG_IS_EXPANDED_EVAL", 0, value, first.get(), second.get());
"#
            );
            (format!("println!(\"VARTAG_KIND\\t{kind}\");"), cases)
        } else {
            (String::new(), String::new())
        };
        let wrapping_case = if wrapping {
            "record(\"TYPEALIGN_MINIMUM_ALIGNMENT\", 0, TYPEALIGN!(i32::MIN, 13_i32), 0, 0);"
        } else {
            ""
        };
        let page_cases = if major == 15 {
            r#"
first.set(0); second.set(0);
record("PageSizeIsValid_BELOW", 0, PageSizeIsValid!(CValue::<CUnsignedLong>::new(u64::from(block_size) - 1)), 0, 0);
record("PageSizeIsValid_EXACT", 0, PageSizeIsValid!(CValue::<CUnsignedLong>::new(u64::from(block_size))), 0, 0);
record("PageSizeIsValid_ABOVE", 0, PageSizeIsValid!(CValue::<CUnsignedLong>::new(u64::from(block_size) + 1)), 0, 0);
record("PageSizeIsValid_UNSIGNED_LITERAL", 0, PageSizeIsValid!(block_size), 0, 0);
let page_size_argument = || { first.set(first.get() + 1); CValue::<CUnsignedLong>::new(u64::from(block_size)) };
let value = PageSizeIsValid!(page_size_argument());
record("PageSizeIsValid_EVAL", 0, value, first.get(), second.get());
"#
        } else {
            ""
        };
        // PostgreSQL replaced PageIsValid's macro with a static inline function
        // after PG15, so it is no longer part of macro discovery there.
        let page_valid_cases = if major == 15 {
            r#"
record("PageIsValid_NULL_MUTABLE", 0, PageIsValid!(core::ptr::null_mut::<u8>()), 0, 0);
record("PageIsValid_NULL_CONST", 0, PageIsValid!(core::ptr::null::<u8>()), 0, 0);
record("PageIsValid_MUTABLE", 0, PageIsValid!(pointer.cast_mut()), 0, 0);
record("PageIsValid_CONST", 0, PageIsValid!(pointer), 0, 0);
let page_argument = || { first.set(first.get() + 1); pointer.cast_mut() };
let value = PageIsValid!(page_argument());
record("PageIsValid_EVAL", 0, value, first.get(), second.get());
"#
        } else {
            ""
        };
        rust.push_str(
            &include_str!("fixtures/handports_postgres.rs")
                .replace("/* @BLOCK_SIZE@ */", &format!("{block_size}_u32"))
                .replace("// @WRAPPING_CASES@", wrapping_case)
                .replace("// @PAGE_SIZE_CASES@", page_cases)
                .replace("// @PAGE_VALID_CASES@", page_valid_cases)
                .replace("// @TRIGGER_CASES@", &triggers)
                .replace("// @VARTAG_METADATA@", &vartag_metadata)
                .replace("// @VARTAG_CASES@", &vartag_cases),
        );
        let generated = rust_oracle::run_rust_linked_with_cfg(
            &rust,
            &profile.compiler.executable,
            &profile.header,
            &format!(
                "{}\n#define PGRX_HANDPORT_NO_MAIN 1\n{}",
                artifact.c_source,
                include_str!("fixtures/handports_postgres.c")
            ),
            &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            &pgrx_c_macros::support_rust_cfg(profile).unwrap(),
        );
        let expected_rows = 1
            + 20
            + 2 * 4096
            + 8 * 1024
            + 18
            + usize::from(wrapping)
            + 4
            + if major == 15 { 5 } else { 0 }
            + 9 * (256 + 4 * 32 + 1)
            + if major < 19 { 264 } else { 0 }
            + 2 * 128
            + 7
            + 4096
            + 5
            + 3
            + if major == 15 { 5 } else { 0 }
            + 258
            + 11;
        assert_eq!(original.lines().count(), expected_rows, "PG{major}: C corpus completeness");
        assert_eq!(generated.lines().count(), expected_rows, "PG{major}: Rust corpus completeness");
        for (index, (c, rust)) in original.lines().zip(generated.lines()).enumerate() {
            assert_eq!(rust, c, "PG{major}: original C vs generated Rust record {index}");
        }
        eprintln!("PG{major}: {} handports, {expected_rows} C/Rust records", emitted.len());
    }
}
