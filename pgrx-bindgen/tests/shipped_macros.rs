//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![cfg(unix)]

use quote::ToTokens;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir()
            .join(format!("pgrx-shipped-macros-{}-{nonce}", std::process::id()));
        for directory in ["src/include/cmacros", "include", "server", "pgrx-home"] {
            fs::create_dir_all(root.join(directory)).unwrap();
        }
        Self(root)
    }

    fn command(&self) -> Command {
        let mut command = Command::new("cargo");
        for (name, _) in std::env::vars_os() {
            if name.to_str().is_some_and(|name| {
                name.starts_with("PGRX_") || name.starts_with("BINDGEN_EXTRA_CLANG_ARGS")
            }) {
                command.env_remove(name);
            }
        }
        command
            .args(["run", "--offline", "--manifest-path"])
            .arg(self.0.join("Cargo.toml"))
            .args(["--features", "pg18"])
            .env("CARGO_TARGET_DIR", self.0.join("target"))
            .env("PGRX_HOME", self.0.join("pgrx-home"))
            .env("PGRX_PG_CONFIG_PATH", self.0.join("pg_config"))
            .env_remove("DOCS_RS");
        command
    }

    fn run(&self, command: &mut Command, phase: &str) {
        assert_eq!(self.successful_output(command, phase), "12\n", "{phase} macro invocation");
    }

    fn successful_output(&self, command: &mut Command, phase: &str) -> String {
        let paths =
            [self.0.join(format!("{phase}.stdout")), self.0.join(format!("{phase}.stderr"))];
        let mut child = command
            .stdin(Stdio::null())
            .stdout(File::create(&paths[0]).unwrap())
            .stderr(File::create(&paths[1]).unwrap())
            .spawn()
            .expect("start isolated test process");
        let deadline = Instant::now() + Duration::from_secs(300);
        let status = loop {
            let oversized = paths.iter().any(|path| fs::metadata(path).unwrap().len() > 16 << 20);
            if oversized || Instant::now() >= deadline {
                let _ = child.kill();
                let _ = child.wait();
                panic!("{phase} exceeded its time or output limit");
            }
            if let Some(status) = child.try_wait().expect("wait for test process") {
                break status;
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        assert!(status.success(), "{phase} failed:\n{}", fs::read_to_string(&paths[1]).unwrap());
        fs::read_to_string(&paths[0]).unwrap()
    }

    fn formatted(&self, file: &syn::File, phase: &str) -> String {
        let path = self.0.join(format!("{phase}.rs"));
        fs::write(&path, file.to_token_stream().to_string()).unwrap();
        self.successful_output(
            Command::new("rustfmt")
                .args(["--edition", "2024", "--config", "skip_children=true"])
                .arg(&path),
            phase,
        );
        fs::read_to_string(path).unwrap()
    }

    fn output(&self) -> PathBuf {
        fs::read_dir(self.0.join("target/debug/build"))
            .unwrap()
            .map(|entry| entry.unwrap().path().join("out/cmacros/pg18/mod.rs"))
            .find(|path| path.exists())
            .expect("generated macros must exist")
    }

    fn out_dir(&self) -> PathBuf {
        self.output().ancestors().nth(3).unwrap().to_path_buf()
    }

    fn assert_mismatch_skips(&self, phase: &str) {
        let out_dir = self.out_dir();
        let report: serde_json::Value =
            serde_json::from_slice(&fs::read(out_dir.join("pg18_macro_report.json")).unwrap())
                .unwrap();
        assert_eq!(
            report["integer_bindings"]["integer_constants"]["SNAPSHOT_HUGE"]["value"]["value"],
            0
        );
        assert_eq!(
            report["integer_constants"]["SNAPSHOT_HUGE"]["value"]["value"],
            9223372036854775807_u64
        );
        let build_output = fs::read_to_string(out_dir.parent().unwrap().join("output")).unwrap();
        let stderr = fs::read_to_string(self.0.join(format!("{phase}.stderr"))).unwrap();
        for (name, code) in [
            ("SNAPSHOT_VALID", "binding_value_mismatch"),
            ("SNAPSHOT_VALID_WRAPPER", "dependency_skipped"),
        ] {
            let emission = report["macros"]
                .as_array()
                .unwrap()
                .iter()
                .find(|emission| emission["analysis"]["name"] == name)
                .expect("skipped macro must remain in the report");
            assert_eq!(emission["status"]["status"], "skipped");
            assert_eq!(emission["status"]["reason"]["code"], code);
            let reason = emission["status"]["reason"]["message"].as_str().unwrap();
            assert!(reason.contains("SNAPSHOT_HUGE"));
            assert!(reason.contains("bindgen=0"));
            assert!(reason.contains("clang=9223372036854775807"));
            assert!(reason.contains(&format!("skipping macro `{name}`")));
            if name == "SNAPSHOT_VALID" {
                assert_eq!(
                    reason,
                    "bindgen and clang do not agree on SNAPSHOT_HUGE's value; bindgen=0, clang=9223372036854775807; skipping macro `SNAPSHOT_VALID`"
                );
            } else {
                assert!(
                    reason.contains("SNAPSHOT_VALID"),
                    "dependency warning names the skipped callee"
                );
            }
            let directive = format!("cargo:warning={reason}");
            assert_eq!(
                build_output.lines().filter(|line| *line == directive).count(),
                1,
                "{phase} emits exactly one final warning for {name}"
            );
            assert!(stderr.contains(reason), "{phase} exposes the warning through Cargo");
        }
    }
}

fn rust_tree(directory: &Path) -> BTreeMap<PathBuf, String> {
    let mut result = BTreeMap::new();
    let mut directories = vec![directory.to_path_buf()];
    while let Some(current) = directories.pop() {
        for entry in fs::read_dir(current).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let kind = entry.file_type().unwrap();
            if kind.is_dir() {
                directories.push(path);
            } else if kind.is_file() && path.extension().is_some_and(|extension| extension == "rs")
            {
                let name = path.strip_prefix(directory).unwrap().to_path_buf();
                assert!(result.insert(name, fs::read_to_string(path).unwrap()).is_none());
            }
        }
    }
    result
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn docsrs_exports_generated_adapter_modules_and_accepts_support_free_snapshots() {
    let fixture = Fixture::new();
    let postgres_module = |ident: &syn::Ident| {
        ident
            .to_string()
            .strip_prefix("pg")
            .and_then(|suffix| suffix.split('_').next())
            .and_then(|major| major.parse::<u16>().ok())
    };
    let mut included = syn::parse_file(include_str!("../../pgrx-pg-sys/src/include.rs")).unwrap();
    // Keep the actual declarations, cfgs and reexports, without unrelated
    // PostgreSQL compatibility functions that would require complete bindings.
    included.items.retain(|item| match item {
        syn::Item::Mod(module) => {
            postgres_module(&module.ident).is_some() || module.ident == "cmacros"
        }
        syn::Item::Use(import) => {
            matches!(&import.tree, syn::UseTree::Path(path) if postgres_module(&path.ident).is_some() || path.ident == "cmacros")
        }
        _ => false,
    });
    let versions = included
        .items
        .iter()
        .filter_map(|item| {
            let syn::Item::Mod(module) = item else { return None };
            postgres_module(&module.ident)
        })
        .collect::<BTreeSet<_>>();
    assert!(!versions.is_empty(), "actual binding modules must be exercised");
    fs::write(fixture.0.join("src/include.rs"), included.into_token_stream().to_string()).unwrap();
    fs::write(
        fixture.0.join("src/include/cmacros/mod.rs"),
        include_str!("../../pgrx-pg-sys/src/include/cmacros/mod.rs"),
    )
    .unwrap();

    let mut library = syn::parse_file(include_str!("../../pgrx-pg-sys/src/lib.rs")).unwrap();
    library.attrs = vec![syn::parse_quote!(#![allow(unused_imports)])];
    library.items.retain(|item| match item {
        syn::Item::Mod(module) => module.ident == "include",
        syn::Item::Use(import) => {
            matches!(&import.tree, syn::UseTree::Path(path) if path.ident == "include")
        }
        _ => false,
    });
    assert_eq!(library.items.len(), 2, "use the defining crate's actual include/export path");
    let library_path = fixture.0.join("src/lib.rs");
    fs::write(&library_path, library.into_token_stream().to_string()).unwrap();

    for with_support in [false, true] {
        let kind = if with_support { "generated" } else { "support-free" };
        let support = if with_support {
            "#[doc(hidden)] pub mod __pgrx_c_generated { pub const ADAPTER: u32 = 12; }\n"
        } else {
            ""
        };
        let expression = if with_support {
            "$crate::__pgrx_c_generated::ADAPTER + $crate::BINDING"
        } else {
            "$crate::BINDING"
        };
        for major in &versions {
            fs::write(
                fixture.0.join(format!("src/include/pg{major}.rs")),
                format!("pub const BINDING: u32 = {major};\n"),
            )
            .unwrap();
            let macros = fixture.0.join(format!("src/include/cmacros/pg{major}"));
            fs::create_dir_all(&macros).unwrap();
            fs::write(macros.join("mod.rs"), "mod server;\npub use server::*;\n").unwrap();
            fs::write(
                macros.join("server.rs"),
                format!("{support}#[macro_export] macro_rules! SNAPSHOT_ADAPTER_VALUE {{ () => {{ {expression} }}; }}\npub use SNAPSHOT_ADAPTER_VALUE;\n"),
            )
            .unwrap();
            fs::write(fixture.0.join(format!("src/include/pg{major}_oids.rs")), "").unwrap();
        }
        for major in &versions {
            let phase = format!("docsrs-pg{major}-{kind}");
            let archive = fixture.0.join(format!("libdocs_snapshot_pg{major}_{kind}.rlib"));
            fixture.successful_output(
                Command::new("rustc")
                    .args(["--edition=2024", "--crate-name=docs_snapshot", "--crate-type=rlib"])
                    .args(["--cfg", "docsrs", "--cfg"])
                    .arg(format!("feature=\"pg{major}\""))
                    .arg(&library_path)
                    .arg("-o")
                    .arg(&archive),
                &format!("{phase}-library"),
            );
            let adapter_assertion = if with_support {
                "assert_eq!(renamed::__pgrx_c_generated::ADAPTER, 12);"
            } else {
                ""
            };
            let consumer = fixture.0.join(format!("{phase}.rs"));
            fs::write(
                &consumer,
                format!("extern crate docs_snapshot as renamed;\nfn main() {{ {adapter_assertion} println!(\"{{}}\", renamed::SNAPSHOT_ADAPTER_VALUE!()); }}\n"),
            )
            .unwrap();
            let executable = fixture.0.join(&phase);
            fixture.successful_output(
                Command::new("rustc")
                    .args(["--edition=2024", "--crate-name=docs_consumer", "--extern"])
                    .arg(format!("docs_snapshot={}", archive.display()))
                    .arg(&consumer)
                    .arg("-o")
                    .arg(&executable),
                &format!("{phase}-consumer"),
            );
            assert_eq!(
                fixture.successful_output(&mut Command::new(executable), &phase),
                format!("{}\n", u32::from(*major) + if with_support { 12 } else { 0 }),
                "actual include exports must resolve adapter paths under docsrs"
            );
        }
    }
}

#[test]
#[ignore = "compiles isolated ordinary, release and docs.rs builds; requires native Clang and cached dependencies"]
fn release_ships_macros_and_docsrs_uses_them_without_postgres_or_clang() {
    let fixture = Fixture::new();
    let bindgen = Path::new(env!("CARGO_MANIFEST_DIR"));
    let support = bindgen.parent().unwrap().join("pgrx-pg-sys/src/c_macros/support.rs");
    fs::write(
        fixture.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"shipped-macro-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n[features]\npg18 = []\n[build-dependencies]\npgrx-bindgen = {{ path = {bindgen:?} }}\n[profile.dev.package.sha2]\nopt-level = 3\n"
        ),
    )
    .unwrap();
    fs::write(fixture.0.join("build.rs"), "fn main() { pgrx_bindgen::build::main().unwrap(); }\n")
        .unwrap();
    fs::write(fixture.0.join("pgrx-cshim.c"), "").unwrap();
    fs::write(fixture.0.join("include/pg18.h"), "#include <server.h>\n").unwrap();
    fs::write(
        fixture.0.join("server/server.h"),
        r#"typedef unsigned int Oid;
typedef unsigned int TransactionId;
#define SNAPSHOT_ADD(left, right) (((left) + (right)) & 0xFFFFFFFF)
#define SNAPSHOT_HUGE (0xFFFFFFFFFFFFFFFFUL / 2)
#define SNAPSHOT_VALID(size) ((__SIZE_TYPE__) (size) <= SNAPSHOT_HUGE)
#define SNAPSHOT_VALID_WRAPPER(size) SNAPSHOT_VALID((size))
#define SNAPSHOT_SMALL 17
#define SNAPSHOT_SMALL_VALID(size) ((__SIZE_TYPE__) (size) <= SNAPSHOT_SMALL)
#define SNAPSHOT_SMALL_VALID_WRAPPER(size) SNAPSHOT_SMALL_VALID((size))
#define SNAPSHOT_ALIGNMENT 32
#define SNAPSHOT_ALIGN(align, len) (((__SIZE_TYPE__) (len) + ((align) - 1)) & ~((__SIZE_TYPE__) ((align) - 1)))
#define SNAPSHOT_BUFFERALIGN(len) SNAPSHOT_ALIGN(SNAPSHOT_ALIGNMENT, (len))
#define SNAPSHOT_SPLIT 1 + 2
#define SNAPSHOT_GROUPING(len) ((len) * SNAPSHOT_SPLIT)
"#,
    )
    .unwrap();
    let pg_config = fixture.0.join("pg_config");
    fs::write(
        &pg_config,
        r#"#!/bin/sh
root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
case "$1" in
    --version) printf '%s\n' 'PostgreSQL 18.4' ;;
    --includedir-server|--libdir) printf '%s\n' "$root/server" ;;
    --cflags) printf '%s\n' '-fwrapv -fsigned-char' ;;
    --cppflags|--configure) printf '\n' ;;
    *) exit 1 ;;
esac
"#,
    )
    .unwrap();
    fs::set_permissions(&pg_config, fs::Permissions::from_mode(0o755)).unwrap();
    fs::write(
        fixture.0.join("src/lib.rs"),
        format!(
            r#"
#[derive(Clone, Copy)] pub struct Oid(pub u32);
impl Oid {{ pub fn to_u32(self) -> u32 {{ self.0 }} pub fn from_u32(value:u32)->Self {{ Self(value) }} }}
#[derive(Clone, Copy)] pub struct TransactionId(pub u32);
impl TransactionId {{ pub fn into_inner(self) -> u32 {{ self.0 }} pub fn from_inner(value:u32)->Self {{ Self(value) }} }}
pub type MultiXactId = TransactionId;
pub struct Datum;
pub trait PgNode {{}}
#[path = {support:?}] pub mod __pgrx_c_macros;
#[cfg(not(docsrs))] mod pg18 {{
    include!(concat!(env!("OUT_DIR"), "/pg18.rs"));
}}
#[cfg(docsrs)] mod pg18 {{
    include!("include/pg18.rs");
}}
pub use pg18::*;
#[cfg(not(docsrs))] pub mod cmacros {{
    include!(concat!(env!("OUT_DIR"), "/cmacros/pg18/mod.rs"));
}}
#[cfg(docsrs)] #[path = "include/cmacros/pg18/mod.rs"] pub mod cmacros;
pub use cmacros::*;
"#
        ),
    )
    .unwrap();
    fs::write(
        fixture.0.join("src/main.rs"),
        r#"use shipped_macro_consumer as renamed;
fn main() {
    use renamed::__pgrx_c_macros::{CValue, CUnsignedLong};
    let untouched: u32 = renamed::SNAPSHOT_HUGE;
    assert_eq!(untouched, 0);
    assert_eq!(renamed::SNAPSHOT_SMALL_VALID!(CValue::<CUnsignedLong>::new(17)).get(), 1);
    assert_eq!(renamed::SNAPSHOT_SMALL_VALID_WRAPPER!(CValue::<CUnsignedLong>::new(18)).get(), 0);
    assert_eq!(renamed::SNAPSHOT_BUFFERALIGN!(33_i32).get(), 64);
    assert_eq!(renamed::SNAPSHOT_GROUPING!(5_i32).get(), 7);
    println!("{}", renamed::SNAPSHOT_ADD!(renamed::Oid(10), renamed::TransactionId(2)).get());
}
"#,
    )
    .unwrap();

    let snapshot = fixture.0.join("src/include/cmacros/pg18");
    fs::create_dir_all(&snapshot).unwrap();
    fs::write(snapshot.join("mod.rs"), "// ordinary builds must preserve this snapshot\n").unwrap();
    fixture.run(&mut fixture.command(), "ordinary");
    fixture.assert_mismatch_skips("ordinary");
    assert_eq!(
        fs::read_to_string(snapshot.join("mod.rs")).unwrap(),
        "// ordinary builds must preserve this snapshot\n"
    );
    let generated_root = fixture.output().parent().unwrap().to_path_buf();
    let generated = rust_tree(&generated_root);
    fixture.run(fixture.command().env("PGRX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE", "1"), "release");
    fixture.assert_mismatch_skips("release");
    assert_eq!(rust_tree(&generated_root), generated);
    let shipped = rust_tree(&snapshot);
    assert_eq!(shipped.keys().collect::<Vec<_>>(), generated.keys().collect::<Vec<_>>());
    let generated_source = generated.values().map(String::as_str).collect::<Vec<_>>().join("\n");
    let shipped_source = shipped.values().map(String::as_str).collect::<Vec<_>>().join("\n");
    assert!(
        shipped_source.contains("$crate::SNAPSHOT_SMALL"),
        "matching constants remain binding references"
    );
    assert!(
        shipped_source.contains("$crate::SNAPSHOT_SMALL_VALID!"),
        "matching wrappers remain macro calls"
    );
    for tree in [&generated, &shipped] {
        let mut names = BTreeSet::new();
        for source in tree.values() {
            let file = syn::parse_file(source).unwrap();
            names.extend(file.items.iter().filter_map(|item| match item {
                syn::Item::Macro(item) if item.mac.path.is_ident("macro_rules") => {
                    item.ident.as_ref().map(ToString::to_string)
                }
                _ => None,
            }));
        }
        assert!(!names.contains("SNAPSHOT_VALID"));
        assert!(!names.contains("SNAPSHOT_VALID_WRAPPER"));
        assert!(names.contains("SNAPSHOT_SMALL_VALID"));
        assert!(names.contains("SNAPSHOT_SMALL_VALID_WRAPPER"));
    }
    assert!(shipped_source.contains("$crate::SNAPSHOT_ALIGN!"), "preserve macro calls");
    assert!(
        shipped_source.contains("/* PGRX: SNAPSHOT_SPLIT"),
        "snapshot keeps fallback explanations"
    );
    assert!(shipped_source.contains("/// #define SNAPSHOT_ADD"), "keep original C doc comments");
    assert!(shipped_source.contains("0xFFFFFFFFu32"), "keep original hexadecimal literal notation");
    assert!(
        generated_source.contains("/// #define SNAPSHOT_ADD"),
        "normal build output keeps original C doc comments"
    );
    let documentation_guard: syn::Attribute = syn::parse_quote!(#[cfg(not(docsrs))]);
    let mut guards = 0;
    for (index, (name, shipped_source)) in shipped.iter().enumerate() {
        let generated_source = &generated[name];
        let comments = |source: &str| {
            source
                .lines()
                .map(str::trim)
                .filter(|line| line.starts_with("//"))
                .map(str::to_owned)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            comments(shipped_source),
            comments(generated_source),
            "snapshot preserves original C doc comments in {}",
            name.display()
        );
        let mut shipped_file = syn::parse_file(shipped_source).unwrap();
        let generated_file = syn::parse_file(generated_source).unwrap();
        assert_eq!(shipped_file.items.len(), generated_file.items.len());
        for (item, original) in shipped_file.items.iter_mut().zip(&generated_file.items) {
            let (attrs, expected_guard) = match item {
                syn::Item::Macro(item) => {
                    let target_guard = item.mac.path.is_ident("compile_error");
                    guards += usize::from(target_guard);
                    (&mut item.attrs, target_guard)
                }
                syn::Item::Mod(item) => {
                    let support = item.ident == "__pgrx_c_generated";
                    (&mut item.attrs, support)
                }
                syn::Item::Use(item) => {
                    let support =
                        item.tree.to_token_stream().to_string().contains("__pgrx_c_generated");
                    (&mut item.attrs, support)
                }
                _ => continue,
            };
            let original_attrs: &[syn::Attribute] = match original {
                syn::Item::Macro(item) => &item.attrs,
                syn::Item::Mod(item) => &item.attrs,
                syn::Item::Use(item) => &item.attrs,
                _ => &[],
            };
            let existing_guard = original_attrs.iter().any(|attr| {
                attr.to_token_stream().to_string()
                    == documentation_guard.to_token_stream().to_string()
            });
            let original_len = attrs.len();
            if !existing_guard {
                attrs.retain(|attr| {
                    attr.to_token_stream().to_string()
                        != documentation_guard.to_token_stream().to_string()
                });
            }
            if expected_guard && !existing_guard {
                assert_eq!(
                    attrs.len() + 1,
                    original_len,
                    "shipped guard excludes docs.rs in {}",
                    name.display()
                );
            } else {
                assert_eq!(
                    attrs.len(),
                    original_len,
                    "unexpected docs.rs guard in {}",
                    name.display()
                );
            }
        }
        // Rustfmt adds optional trailing commas inside opaque macro token bodies.
        // Normalize each file's syntax; doc comments were checked separately.
        assert_eq!(
            fixture.formatted(&shipped_file, &format!("format-shipped-{index}")),
            fixture.formatted(&generated_file, &format!("format-generated-{index}")),
            "release snapshot preserves macro tokens and integer bridges in {}",
            name.display()
        );
    }
    assert!(guards > 0, "generated macros must retain their target guards");

    // Force a target mismatch so docs.rs compilation proves the guard exclusion
    // instead of merely succeeding on the same target as the generator.
    for (name, source) in &shipped {
        let mut file = syn::parse_file(source).unwrap();
        let mut changed = false;
        for item in &mut file.items {
            if let syn::Item::Macro(item) = item
                && item.mac.path.is_ident("compile_error")
            {
                item.attrs = vec![syn::parse_quote!(#[cfg(all())]), documentation_guard.clone()];
                changed = true;
            }
        }
        if changed {
            fs::write(snapshot.join(name), file.into_token_stream().to_string()).unwrap();
        }
    }
    fs::remove_file(pg_config).unwrap();
    fs::remove_dir_all(fixture.0.join("server")).unwrap();
    fs::remove_dir_all(fixture.out_dir()).unwrap();
    fixture.run(
        fixture.command().env("DOCS_RS", "1").env("CLANG_PATH", "/deliberately/unavailable/clang"),
        "docsrs",
    );
}
