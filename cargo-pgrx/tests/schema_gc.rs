//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![cfg(target_os = "linux")]

use object::{Object, ObjectSection, ObjectSymbol};
use pgrx_sql_entity_graph::section::{
    ENTITY_SCHEMA, EntryWriter, decode_entities, str_len, u8_len, u32_len,
};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output};

fn schema_command(root: &Path, gc: &str) -> Command {
    let mut flags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_else(|_| {
        std::env::var("RUSTFLAGS")
            .unwrap_or_default()
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .join("\x1f")
    });
    if !flags.is_empty() {
        flags.push('\x1f');
    }
    flags.push_str(&format!("-C\x1flink-arg=-Wl,{gc}"));
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-pgrx"));
    command
        .current_dir(root)
        .args(["pgrx", "schema", "pg18", "--release", "--cargo=--offline"])
        .env("PGRX_HOME", root.join("pgrx-home"))
        .env("PGRX_PG_CONFIG_PATH", root.join("pg_config"))
        .env("CARGO_TARGET_DIR", root.join("target with spaces,comma"))
        .env("CARGO_ENCODED_RUSTFLAGS", flags)
        .env_remove("CARGO_PGRX_DISABLE_GC_SECTIONS_WORKAROUND")
        .env_remove("PGRX_BUILD_FLAGS");
    command
}

fn run(command: &mut Command) -> Output {
    let output = command.output().expect("cargo-pgrx should launch");
    assert!(
        output.status.success(),
        "{command:?} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    output
}

fn has_dead_import(file: &object::File<'_>) -> bool {
    file.dynamic_symbols().any(|symbol| {
        symbol.is_undefined() && symbol.name().unwrap() == "pgrx_gc_test_missing_symbol"
    })
}

#[test]
fn schema_retention_does_not_retain_unreachable_code() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    fs::create_dir(root.join("src")).unwrap();
    fs::create_dir(root.join("pgrx-home")).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        r#"
[package]
name = "schema_gc_fixture"
version = "0.1.0"
edition = "2024"
[workspace]
[lib]
crate-type = ["cdylib"]
[features]
pg18 = []
[profile.release]
lto = false
codegen-units = 16
"#,
    )
    .unwrap();
    fs::write(
        root.join("schema_gc_fixture.control"),
        "comment = 'schema retention regression'\ndefault_version = '@CARGO_VERSION@'\nmodule_pathname = '$libdir/schema_gc_fixture'\nrelocatable = false\nsuperuser = false\n",
    )
    .unwrap();
    // Schema extraction needs a PostgreSQL version, but never starts or loads a backend.
    fs::write(
        root.join("pg_config"),
        "#!/bin/sh\n[ \"$1\" = --version ] || exit 1\nprintf 'PostgreSQL 18.7\\n'\n",
    )
    .unwrap();
    fs::set_permissions(root.join("pg_config"), fs::Permissions::from_mode(0o755)).unwrap();

    fs::write(
        root.join("src/lib.rs"),
        r#"
mod first;
mod other;

unsafe extern "C" {
    fn pgrx_gc_test_missing_symbol();
}

// This function is never executed: the test only inspects the linked ELF file.
unsafe extern "C" fn unused_wrapper() {
    // SAFETY: this fixture is never loaded; the unresolved call tests linker GC.
    unsafe { pgrx_gc_test_missing_symbol() }
}

// A sym operand keeps the wrapper through compilation without SHF_GNU_RETAIN.
core::arch::global_asm!(
    ".pushsection .pgrx_dead,\"aw\"",
    ".quad {wrapper}",
    ".popsection",
    wrapper = sym unused_wrapper,
);
"#,
    )
    .unwrap();
    const PAYLOAD_LEN: usize =
        u8_len() + str_len("first") + str_len("first") + str_len("fixture.rs") + u32_len();
    for name in ["first", "other"] {
        let entry = EntryWriter::<{ u32_len() + PAYLOAD_LEN }>::new()
            .u32(PAYLOAD_LEN as u32)
            .u8(ENTITY_SCHEMA)
            .str(name)
            .str(name)
            .str("fixture.rs")
            .u32(1)
            .finish();
        // Model toolchains that emit pgrx's private #[used] bytes without SHF_GNU_RETAIN.
        // Current rustc may set that flag, which would hide a broken KEEP rule.
        let bytes = entry.iter().map(u8::to_string).collect::<Vec<_>>().join(",");
        fs::write(
            root.join(format!("src/{name}.rs")),
            format!(
                "core::arch::global_asm!(r#\".pushsection .pgrxsc,\"a\"\n.byte {bytes}\n.popsection\"#);\n",
            ),
        )
        .unwrap();
    }

    let library = root.join("target with spaces,comma/release/libschema_gc_fixture.so");
    run(schema_command(root, "--no-gc-sections")
        .env("CARGO_PGRX_DISABLE_GC_SECTIONS_WORKAROUND", "1"));
    let uncollected = fs::read(&library).unwrap();
    let file = object::File::parse(uncollected.as_slice()).unwrap();
    assert!(file.section_by_name(".pgrx_dead").is_some());
    assert!(has_dead_import(&file), "fixture must retain the unused import without GC");

    let output = schema_command(root, "--gc-sections")
        .env("CARGO_PGRX_DISABLE_GC_SECTIONS_WORKAROUND", "1")
        .output()
        .unwrap();
    assert!(!output.status.success(), "unrooted schema metadata must be collected");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("no embedded pgrx schema section found"),
        "{}",
        String::from_utf8_lossy(&output.stderr),
    );

    let output = run(&mut schema_command(root, "--gc-sections"));
    let sql = String::from_utf8(output.stdout).unwrap();
    for name in ["first", "other"] {
        assert!(sql.contains(&format!("CREATE SCHEMA IF NOT EXISTS {name};")), "{sql}");
    }
    let collected = fs::read(&library).unwrap();
    let file = object::File::parse(collected.as_slice()).unwrap();
    let section = file.section_by_name(".pgrxsc").expect("retained schema section");
    assert_eq!(decode_entities(section.data().unwrap()).unwrap().len(), 2);
    assert!(file.section_by_name(".pgrx_dead").is_none());
    assert!(!has_dead_import(&file), "retaining metadata must not retain dead imports");
    eprintln!("schema GC fixture: {} -> {} bytes", uncollected.len(), collected.len());

    let modified = fs::metadata(&library).unwrap().modified().unwrap();
    run(&mut schema_command(root, "--gc-sections"));
    assert_eq!(
        fs::metadata(&library).unwrap().modified().unwrap(),
        modified,
        "the retention script must not force a relink on every invocation",
    );
}
