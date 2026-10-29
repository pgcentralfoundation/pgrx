//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Keep availability-gated macro tests from disappearing without a failure, and explain a
//! generated macro that pgrx calls but this build did not emit.
//!
//! The classifier's availability arm and `if_c_macro!` expand to nothing when this build did
//! not emit the named macro. A generator regression would therefore delete the tests they wrap
//! while `cargo test` still passes. This test collects every gated name from the pg-sys
//! integration tests and the pgrx-unit-tests backend tests, then requires this build's macro
//! report to show it as emitted unless a listed profile difference explains its absence.
//! A gate that this build compiles out, through a file-level `#![cfg]` or a `#[cfg]` written
//! directly on the gate, wraps no test here and is not collected.
//!
//! pgrx and pgrx-macros' output templates call generated macros as `pg_sys::NAME!`. A macro the
//! generator skipped only fails their compilation with "cannot find macro", so this test also
//! requires each of those calls to be emitted and prints the generator's reason when it is not.

#![cfg(not(docsrs))]

/// Ask this build's original compiler profile whether assertions are enabled.
#[path = "support/postgres_oracle.rs"]
mod postgres_oracle;

use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use syn::ext::IdentExt;
use syn::visit_mut::{self, VisitMut};

/// Read only the availability facts from this build's macro report.
#[derive(serde::Deserialize)]
struct Report {
    /// Version whose installed headers produced the report.
    postgres_major_version: u16,
    /// Whether inspection ran; any other recorded status fails deserialization.
    status: ReportStatus,
    /// Every selected candidate and its emission result.
    macros: Vec<Candidate>,
}

/// Require a report from completed inspection.
#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum ReportStatus {
    /// Inspection ran and each candidate records its own result.
    Generated,
}

/// Identify one selected macro and its emission result.
#[derive(serde::Deserialize)]
struct Candidate {
    /// Analysis facts, of which only the name is needed here.
    analysis: Analysis,
    /// Structured emission or skip result.
    status: Emission,
}

/// Name a selected macro without reading its large analysis body.
#[derive(serde::Deserialize)]
struct Analysis {
    /// Original C macro name, which is also the generated Rust macro name.
    name: String,
}

/// Distinguish emitted macros from structured skips.
#[derive(serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum Emission {
    /// The generated Rust macro exists in this build.
    Emitted,
    /// The generator refused the macro for a recorded reason.
    Skipped {
        /// Structured refusal.
        reason: SkipReason,
    },
}

/// Skip cause recorded by the generator.
#[derive(serde::Deserialize)]
struct SkipReason {
    /// Stable snake_case skip code.
    code: String,
    /// The generator's explanation, printed so a failure on a remote runner names its cause.
    message: String,
}

/// A known profile difference under which a gated macro may be missing from this build.
struct Allowance {
    /// Gated macro name.
    name: &'static str,
    /// PostgreSQL major versions to which the difference applies.
    majors: RangeInclusive<u16>,
    /// Exact skip code, or `None` when the installed headers do not define the macro.
    skip: Option<&'static str>,
    /// Whether the difference exists only in assertion-enabled C profiles.
    assertions_only: bool,
}

/// Every allowed way for a gated macro to be missing. Anything else fails, including a new skip
/// code for a listed name, so each addition here must name the profile difference it accepts.
const ALLOWANCES: &[Allowance] = &[
    // macro_api.rs gates on an undefined name to check the unavailable branch.
    Allowance {
        name: "__PGRX_REVIEW_MACRO_THAT_DOES_NOT_EXIST",
        majors: 0..=u16::MAX,
        skip: None,
        assertions_only: false,
    },
    // PostgreSQL 16 introduced soft error contexts.
    Allowance { name: "SOFT_ERROR_OCCURRED", majors: 15..=15, skip: None, assertions_only: false },
    // PostgreSQL 15's timespec conversions use floating-point literals.
    Allowance {
        name: "INSTR_TIME_GET_DOUBLE",
        majors: 15..=15,
        skip: Some("unsupported_literal"),
        assertions_only: false,
    },
    Allowance {
        name: "INSTR_TIME_GET_MILLISEC",
        majors: 15..=15,
        skip: Some("unsupported_literal"),
        assertions_only: false,
    },
    // AssertMacro expands IsA's token pasting only when assertions are enabled.
    Allowance {
        name: "GetCTETargetList",
        majors: 0..=u16::MAX,
        skip: Some("dynamic_builtin"),
        assertions_only: true,
    },
];

/// Require every gated test's macro, and every generated macro pgrx calls, to exist in this build
/// unless a listed difference applies.
#[test]
fn every_availability_gated_macro_is_emitted() {
    let major = postgres_oracle::postgres_major_version();
    let path = PathBuf::from(env!("OUT_DIR")).join(format!("pg{major}_macro_report.json"));
    let report: Report = serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(&path).expect("open this build's C macro report"),
    ))
    .expect("read this build's C macro report");
    assert_eq!(report.postgres_major_version, major);
    let mut emissions = BTreeMap::new();
    for candidate in report.macros {
        let name = candidate.analysis.name;
        assert!(
            emissions.insert(name.clone(), candidate.status).is_none(),
            "PG{major} report lists {name} more than once"
        );
    }
    let build = Build { major, target_os: std::env::consts::OS };
    let mut gated = gated_names(&build);
    for (name, files) in called_names(&build) {
        gated.entry(name).or_default().extend(files);
    }
    let mut assertions = None;
    let mut missing = Vec::new();
    for (name, files) in &gated {
        let reason = match emissions.get(name) {
            Some(Emission::Emitted) => continue,
            Some(Emission::Skipped { reason }) => Some(reason),
            None => None,
        };
        let skip = reason.map(|reason| reason.code.as_str());
        let allowed = ALLOWANCES.iter().any(|allowance| {
            allowance.name == name
                && allowance.majors.contains(&major)
                && allowance.skip == skip
                && (!allowance.assertions_only
                    || *assertions.get_or_insert_with(assertions_enabled))
        });
        if !allowed {
            let cause = reason.map_or("not in report".into(), |reason| {
                format!("{}: {}", reason.code, reason.message)
            });
            missing.push(format!("{name} ({cause}) in {files:?}"));
        }
    }
    assert!(
        missing.is_empty(),
        "PG{major} did not emit macros that gated tests or pgrx code need:\n{}",
        missing.join("\n")
    );
}

/// Windows builds of PostgreSQL 15-18 compile out every instr_time gate, because their tests
/// compare Unix timing representations. Collecting them there would fail the guard although no
/// test disappears: PostgreSQL 15's WIN32 MILLISEC and MICROSEC use floating-point literals
/// that the generator refuses, and Windows CI tests PostgreSQL 15 and 17. Windows PostgreSQL 19
/// and every non-Windows build keep them.
#[test]
fn gates_compiled_out_by_cfg_are_not_collected() {
    let time = ["INSTR_TIME_GET_DOUBLE", "INSTR_TIME_GET_MILLISEC", "INSTR_TIME_GET_MICROSEC"];
    for major in 15..=19 {
        for target_os in ["linux", "macos", "windows"] {
            let gated = gated_names(&Build { major, target_os });
            for name in time {
                let expected = target_os != "windows" || major == 19;
                assert_eq!(gated.contains_key(name), expected, "{name} on {target_os} PG{major}");
            }
            // generated_c_macro_tests.rs is compiled out on Windows by its file-level cfg.
            assert_eq!(
                gated.contains_key("pgstat_count_heap_scan"),
                target_os != "windows",
                "{target_os} PG{major}"
            );
        }
    }
}

/// A predicate the guard does not model fails instead of silently keeping or dropping a gate.
#[test]
#[should_panic(expected = "cannot evaluate cfg predicate")]
fn unknown_gate_cfgs_fail() {
    cfg_holds("feature = \"cshim\"", &Build { major: 18, target_os: "linux" });
}

/// A file without gates hides no test, so its cfgs need not be ones the guard understands.
#[test]
fn files_without_gates_need_no_understood_cfgs() {
    let build = Build { major: 18, target_os: "linux" };
    assert!(gates("#![cfg(unix)]\nfn helper() {}\n", &build).is_empty());
    assert!(gates("#![cfg(target_family = \"unix\")]\n", &build).is_empty());
}

/// Only the attribute lines directly above a gate apply to it. A cfg on an earlier one-line
/// item belongs to that item, so it must not compile the gate out.
#[test]
fn cfgs_of_preceding_items_do_not_apply_to_a_gate() {
    let unrelated = format!(
        "#[cfg(feature = \"pg19\")]\n#[allow(dead_code)] fn helper() {{}}\n{}",
        gate("FOO")
    );
    let direct =
        format!("#[cfg(feature = \"pg19\")]\n// Only PostgreSQL 19 has FOO.\n{}", gate("FOO"));
    for major in [18, 19] {
        let build = Build { major, target_os: "linux" };
        assert_eq!(gates(&unrelated, &build), [("FOO", true)], "PG{major}");
        assert_eq!(gates(&direct, &build), [("FOO", major == 19)], "PG{major}");
    }
}

/// Qualified calls count in compiled code, including inside other macros' tokens such as
/// `quote!` templates. Comparisons with pg-sys constants, and items, statements, match arms and
/// struct fields that the build compiles out, do not; unmodeled `cfg`s keep their code.
#[test]
fn pgrx_calls_follow_the_code_a_build_compiles() {
    let source = r#"
fn plain() { unsafe { pg_sys::PLAIN!(x).get() }; }
fn nested() { assert!(::pgrx::pg_sys::NESTED!(pgrx_pg_sys::INNER!(y)) != 0); }
fn template() -> TokenStream { quote! { ::pgrx::pg_sys::TEMPLATE!(#value) } }
fn compare(event: u32) -> bool { event & pg_sys::CONSTANT != 0 }
#[cfg(feature = "pg19")]
fn added() { pg_sys::ONLY_19!(); }
impl Lock {
    #[cfg(not(feature = "pg19"))]
    #[inline]
    fn removed(&self) { pg_sys::BEFORE_19!(); }
}
fn statements(value: i32) {
    #[cfg(target_os = "windows")]
    pg_sys::WINDOWS!();
    let chosen = {
        #[cfg(any(feature = "pg15", feature = "pg16"))]
        { pg_sys::EARLY!() }
        #[cfg(not(any(feature = "pg15", feature = "pg16")))]
        { pg_sys::LATE!() }
    };
    match value {
        #[cfg(feature = "pg15")]
        0 => pg_sys::ARM_15!(),
        _ => Fields { #[cfg(feature = "pg16")] field: pg_sys::FIELD_16!(), other: chosen },
    }
}
#[cfg(test)]
mod tests { fn unmodeled() { pg_sys::UNMODELED!(); } }
"#;
    let names = |major, target_os| {
        let mut names = file_calls(source, PathBuf::new(), &Build { major, target_os }).names;
        names.sort();
        names
    };
    let common = ["INNER", "NESTED", "PLAIN", "TEMPLATE", "UNMODELED"];
    let expected = |extra: &[&'static str]| {
        let mut expected = common.iter().chain(extra).copied().collect::<Vec<_>>();
        expected.sort();
        expected
    };
    assert_eq!(names(15, "linux"), expected(&["ARM_15", "BEFORE_19", "EARLY"]));
    assert_eq!(names(16, "macos"), expected(&["BEFORE_19", "EARLY", "FIELD_16"]));
    assert_eq!(names(19, "windows"), expected(&["LATE", "ONLY_19", "WINDOWS"]));
}

/// Module declarations lead to their files unless the build compiles them out, including those
/// declared inside an inline module.
#[test]
fn pgrx_call_scan_follows_compiled_module_files() {
    let directory =
        std::env::temp_dir().join(format!("pgrx-macro-availability-{}", std::process::id()));
    std::fs::create_dir_all(directory.join("inline/nested")).unwrap();
    std::fs::write(directory.join("file.rs"), "").unwrap();
    std::fs::write(directory.join("inline/nested/mod.rs"), "").unwrap();
    let source =
        "mod file;\n#[cfg(target_os = \"windows\")]\nmod missing;\nmod inline { mod nested; }\n";
    let modules =
        file_calls(source, directory.clone(), &Build { major: 18, target_os: "linux" }).modules;
    let _ = std::fs::remove_dir_all(&directory);
    assert_eq!(
        modules,
        [
            (directory.join("file.rs"), directory.join("file")),
            (directory.join("inline/nested/mod.rs"), directory.join("inline/nested")),
        ]
    );
}

/// Spell an availability gate for `name` as test input, without writing one in this file that
/// the guard's own scan would collect.
fn gate(name: &str) -> String {
    format!("pg::__pgrx_c_classify! {{ {} {name} {{\n}} }}\n", "@if_available")
}

/// The configuration that decides which gates compile: the `pg<major>` feature and the target
/// operating system, spelled as Rust's `target_os` cfg spells it.
struct Build {
    /// Selected PostgreSQL major version.
    major: u16,
    /// Target operating system.
    target_os: &'static str,
}

/// Map each macro name used by an availability gate that `build` compiles to the test files
/// that use it.
fn gated_names(build: &Build) -> BTreeMap<String, Vec<PathBuf>> {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut gated = BTreeMap::<String, Vec<PathBuf>>::new();
    for root in ["pgrx-pg-sys/tests", "pgrx-unit-tests/src/tests"] {
        let mut files = Vec::new();
        collect_sources(&workspace.join(root), &mut files);
        let mut gates_found = 0;
        for file in files {
            let source = std::fs::read_to_string(&file).expect("read gated test source");
            let relative = file.strip_prefix(workspace).unwrap().to_owned();
            for (name, compiled) in gates(&source, build) {
                gates_found += 1;
                if !compiled {
                    continue;
                }
                let users = gated.entry(name.to_owned()).or_default();
                if !users.contains(&relative) {
                    users.push(relative.clone());
                }
            }
        }
        // An empty scan would make this guard pass vacuously after tests move.
        assert_ne!(gates_found, 0, "found no availability gates below {root}");
    }
    gated
}

/// Map each generated macro that pgrx, or pgrx-macros' output templates, calls as
/// `pg_sys::NAME!` or `pgrx_pg_sys::NAME!` in code `build` compiles to the files that call it.
/// pgrx-pg-sys's own exported macros, such as `error!`, are not generated and are left out.
fn called_names(build: &Build) -> BTreeMap<String, Vec<PathBuf>> {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let handwritten = exported_macros(&workspace.join("pgrx-pg-sys/src"));
    let mut called = BTreeMap::<String, Vec<PathBuf>>::new();
    for root in ["pgrx/src", "pgrx-macros/src"] {
        let directory = workspace.join(root);
        let mut files = vec![(directory.join("lib.rs"), directory)];
        let mut calls_found = 0;
        while let Some((file, directory)) = files.pop() {
            let source = std::fs::read_to_string(&file).expect("read crate source");
            let found = file_calls(&source, directory, build);
            calls_found += found.names.len();
            let relative = file.strip_prefix(workspace).unwrap().to_owned();
            for name in found.names.into_iter().filter(|name| !handwritten.contains(name)) {
                let users = called.entry(name).or_default();
                if !users.contains(&relative) {
                    users.push(relative.clone());
                }
            }
            files.extend(found.modules);
        }
        // An empty scan would make this guard pass vacuously after the code changes shape.
        assert_ne!(calls_found, 0, "found no pg_sys macro calls below {root}");
    }
    called
}

/// Names of the macros pgrx-pg-sys exports from its own source, outside the generated binding
/// snapshot in `src/include`.
fn exported_macros(source: &Path) -> BTreeSet<String> {
    let mut files = Vec::new();
    collect_sources(source, &mut files);
    let mut names = BTreeSet::new();
    for file in files.iter().filter(|file| !file.starts_with(source.join("include"))) {
        let text = std::fs::read_to_string(file).expect("read pg-sys source");
        let mut exported = false;
        for line in text.lines().map(str::trim) {
            if let Some(rest) = line.strip_prefix("macro_rules!")
                && exported
            {
                let rest = rest.trim_start();
                let end = rest
                    .find(|character: char| {
                        !(character.is_ascii_alphanumeric() || character == '_')
                    })
                    .unwrap_or(rest.len());
                names.insert(rest[..end].to_owned());
            }
            // The export attribute may share its item with other attributes and comments.
            exported = line == "#[macro_export]"
                || (exported && (line.starts_with("#[") || line.starts_with("//")));
        }
    }
    assert!(names.contains("error"), "found no exported pg-sys macros below {}", source.display());
    names
}

/// The qualified pg-sys macro calls in one source file, and the module files it declares.
struct FileCalls<'a> {
    /// The build whose `cfg`s decide which code exists.
    build: &'a Build,
    /// Directory holding the files of modules declared in the current module.
    directory: PathBuf,
    /// Called macro names, in source order with repeats.
    names: Vec<String>,
    /// Declared module files paired with the directories holding their own module files.
    modules: Vec<(PathBuf, PathBuf)>,
}

/// Parse `source`, drop the items, statements, arms and fields `build` compiles out, and collect
/// the qualified pg-sys macro calls in what remains, including those inside other macros' tokens
/// such as `quote!` templates. A `cfg` the guard cannot evaluate keeps its code, so the guard can
/// only become stricter. `directory` holds the files of modules this file declares.
fn file_calls<'a>(source: &str, directory: PathBuf, build: &'a Build) -> FileCalls<'a> {
    let mut file = syn::parse_file(source).expect("parse crate source");
    let mut calls = FileCalls { build, directory, names: Vec::new(), modules: Vec::new() };
    let compiled = file.attrs.iter().all(|attribute| match &attribute.meta {
        syn::Meta::List(list) if list.path.is_ident("cfg") => {
            cfg_value(&list.tokens.to_string(), build) != Some(false)
        }
        _ => true,
    });
    if compiled {
        calls.visit_file_mut(&mut file);
    }
    calls
}

impl FileCalls<'_> {
    /// Whether `build` may compile a node, judged by the `cfg`s among its outer attributes.
    fn compiled(&self, node: &impl ToTokens) -> bool {
        let mut tokens = node.to_token_stream().into_iter();
        while let (Some(TokenTree::Punct(hash)), Some(TokenTree::Group(attribute))) =
            (tokens.next(), tokens.next())
        {
            if hash.as_char() != '#' {
                break;
            }
            let mut attribute = attribute.stream().into_iter();
            if let (Some(TokenTree::Ident(name)), Some(TokenTree::Group(predicate)), None) =
                (attribute.next(), attribute.next(), attribute.next())
                && name == "cfg"
                && cfg_value(&predicate.stream().to_string(), self.build) == Some(false)
            {
                return false;
            }
        }
        true
    }
}

/// Prune compiled-out code before visiting it, follow module declarations, and record calls.
impl VisitMut for FileCalls<'_> {
    fn visit_file_mut(&mut self, file: &mut syn::File) {
        file.items.retain(|item| self.compiled(item));
        visit_mut::visit_file_mut(self, file);
    }

    fn visit_item_mod_mut(&mut self, module: &mut syn::ItemMod) {
        let name = module.ident.unraw().to_string();
        let directory = self.directory.join(&name);
        let Some((_, items)) = &mut module.content else {
            assert!(
                !module.attrs.iter().any(|attribute| attribute.path().is_ident("path")),
                "module {name} in {} has a #[path] the guard does not follow",
                self.directory.display()
            );
            let file = [self.directory.join(format!("{name}.rs")), directory.join("mod.rs")]
                .into_iter()
                .find(|file| file.is_file())
                .unwrap_or_else(|| {
                    panic!("no file for module {name} in {}", self.directory.display())
                });
            self.modules.push((file, directory));
            return;
        };
        items.retain(|item| self.compiled(item));
        let parent = std::mem::replace(&mut self.directory, directory);
        visit_mut::visit_item_mod_mut(self, module);
        self.directory = parent;
    }

    fn visit_item_impl_mut(&mut self, item: &mut syn::ItemImpl) {
        item.items.retain(|item| self.compiled(item));
        visit_mut::visit_item_impl_mut(self, item);
    }

    fn visit_item_trait_mut(&mut self, item: &mut syn::ItemTrait) {
        item.items.retain(|item| self.compiled(item));
        visit_mut::visit_item_trait_mut(self, item);
    }

    fn visit_block_mut(&mut self, block: &mut syn::Block) {
        block.stmts.retain(|statement| self.compiled(statement));
        visit_mut::visit_block_mut(self, block);
    }

    fn visit_expr_match_mut(&mut self, expression: &mut syn::ExprMatch) {
        expression.arms.retain(|arm| self.compiled(arm));
        visit_mut::visit_expr_match_mut(self, expression);
    }

    fn visit_expr_struct_mut(&mut self, expression: &mut syn::ExprStruct) {
        expression.fields = std::mem::take(&mut expression.fields)
            .into_pairs()
            .filter(|field| self.compiled(field.value()))
            .collect();
        visit_mut::visit_expr_struct_mut(self, expression);
    }

    fn visit_macro_mut(&mut self, invocation: &mut syn::Macro) {
        let segments = invocation.path.segments.iter().collect::<Vec<_>>();
        if let [.., module, name] = segments.as_slice()
            && (module.ident == "pg_sys" || module.ident == "pgrx_pg_sys")
        {
            self.names.push(name.ident.to_string());
        }
        token_calls(invocation.tokens.clone(), &mut self.names);
    }
}

/// Collect the qualified pg-sys macro calls inside a macro's tokens. `pg_sys::NAME != 0` is a
/// comparison, so a call requires the delimited arguments after `!`.
fn token_calls(tokens: TokenStream, names: &mut Vec<String>) {
    let tokens = tokens.into_iter().collect::<Vec<_>>();
    for (index, token) in tokens.iter().enumerate() {
        if let TokenTree::Group(group) = token {
            token_calls(group.stream(), names);
        }
        if let Some(
            [
                TokenTree::Ident(module),
                TokenTree::Punct(first),
                TokenTree::Punct(second),
                TokenTree::Ident(name),
                TokenTree::Punct(bang),
                TokenTree::Group(_),
            ],
        ) = tokens.get(index..index + 6)
            && (module == "pg_sys" || module == "pgrx_pg_sys")
            && [first, second].iter().all(|punct| punct.as_char() == ':')
            && bang.as_char() == '!'
        {
            names.push(name.to_string());
        }
    }
}

/// Recursively collect Rust sources below a test root.
fn collect_sources(directory: &Path, files: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).expect("read test source directory") {
        let path = entry.expect("read test source entry").path();
        if path.is_dir() {
            collect_sources(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

/// Return each availability gate in `source` with whether `build` compiles it.
fn gates<'a>(source: &'a str, build: &Build) -> Vec<(&'a str, bool)> {
    let names = gate_names(source);
    // A file without gates hides no test, so its cfgs are not evaluated.
    let file = !names.is_empty() && file_cfgs(source).all(|predicate| cfg_holds(predicate, build));
    names
        .into_iter()
        .map(|(name, line_start)| {
            let gate =
                gate_cfgs(&source[..line_start]).all(|predicate| cfg_holds(predicate, build));
            (name, file && gate)
        })
        .collect()
}

/// Return each macro name gated by a classifier availability arm or an `if_c_macro!`
/// invocation, with the offset of the line that starts the gate.
fn gate_names(source: &str) -> Vec<(&str, usize)> {
    let mut names = Vec::new();
    for (marker, delimited) in [("@if_available", false), ("if_c_macro!", true)] {
        for (index, _) in source.match_indices(marker) {
            let rest = source[index + marker.len()..].trim_start();
            let rest = if delimited {
                let Some(rest) = rest.strip_prefix(['{', '(', '[']) else { continue };
                rest.trim_start()
            } else {
                rest
            };
            let end = rest
                .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
                .unwrap_or(rest.len());
            if end != 0 && rest[end..].trim_start().starts_with('{') {
                let line_start = source[..index].rfind('\n').map_or(0, |newline| newline + 1);
                names.push((&rest[..end], line_start));
            }
        }
    }
    names
}

/// Return the predicates of the `#![cfg(...)]` attributes among the comments and inner
/// attributes that open a source file.
fn file_cfgs(source: &str) -> impl Iterator<Item = &str> {
    source
        .lines()
        .map(str::trim)
        .take_while(|line| line.is_empty() || line.starts_with("//") || line.starts_with("#!["))
        .filter_map(|line| line.strip_prefix("#![cfg(")?.strip_suffix(")]"))
}

/// Return the predicates of the single-line `#[cfg(...)]` attributes directly above a gate,
/// given the source before the gate's line. The walk crosses only comments and lines that end
/// with their attribute, so a line that also holds an item ends it and that item's cfg is not
/// taken for the gate's. A cfg spelled any other way is not seen, so the gate stays collected
/// and the guard can only become stricter.
fn gate_cfgs(before: &str) -> impl Iterator<Item = &str> {
    before
        .lines()
        .rev()
        .map(str::trim)
        .take_while(|line| {
            (line.starts_with("#[") && line.ends_with(']')) || line.starts_with("//")
        })
        .filter_map(|line| line.strip_prefix("#[cfg(")?.strip_suffix(")]"))
}

/// Evaluate a `cfg` predicate for `build`. Only the predicates that these tests use are
/// understood; any other fails the test, so a new kind of exclusion cannot drop a gate unseen.
fn cfg_holds(predicate: &str, build: &Build) -> bool {
    cfg_value(predicate, build)
        .unwrap_or_else(|| panic!("availability gates cannot evaluate cfg predicate {predicate:?}"))
}

/// Evaluate a `cfg` predicate for `build`, or return `None` when its value depends on a
/// configuration the guard does not model, such as another feature.
fn cfg_value(predicate: &str, build: &Build) -> Option<bool> {
    let predicate = predicate.trim();
    // A false `all` member, or a true `any` member, decides the result whatever the others are.
    for (name, decisive) in [("all", false), ("any", true)] {
        if let Some(predicates) = cfg_arguments(predicate, name) {
            let values =
                predicates.iter().map(|predicate| cfg_value(predicate, build)).collect::<Vec<_>>();
            return if values.contains(&Some(decisive)) {
                Some(decisive)
            } else if values.contains(&None) {
                None
            } else {
                Some(!decisive)
            };
        }
    }
    if let Some([predicate]) = cfg_arguments(predicate, "not").as_deref() {
        return cfg_value(predicate, build).map(|value| !value);
    }
    let value = |key: &str| {
        let (name, value) = predicate.split_once('=')?;
        (name.trim() == key).then(|| value.trim().strip_prefix('"')?.strip_suffix('"')).flatten()
    };
    if let Some(target_os) = value("target_os") {
        return Some(target_os == build.target_os);
    }
    if let Some(major) =
        value("feature").and_then(|feature| feature.strip_prefix("pg")?.parse::<u16>().ok())
    {
        return Some(build.major == major);
    }
    // This guard compiles only without docsrs.
    (predicate == "docsrs").then_some(false)
}

/// Split `name(first, second, ...)` into its top-level predicates.
fn cfg_arguments<'a>(predicate: &'a str, name: &str) -> Option<Vec<&'a str>> {
    let inner = predicate.strip_prefix(name)?.trim_start().strip_prefix('(')?.strip_suffix(')')?;
    let mut arguments = Vec::new();
    let (mut depth, mut start) = (0_usize, 0);
    for (index, character) in inner.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                arguments.push(inner[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    arguments.push(inner[start..].trim());
    arguments.retain(|argument| !argument.is_empty());
    Some(arguments)
}

/// Report whether this build's original C profile defines USE_ASSERT_CHECKING.
fn assertions_enabled() -> bool {
    let output = postgres_oracle::run(
        "#include <stdio.h>\n#undef printf\nint main(void) {\n#ifdef USE_ASSERT_CHECKING\n    printf(\"1\\n\");\n#else\n    printf(\"0\\n\");\n#endif\n    return 0;\n}\n",
    );
    match output.trim() {
        "1" => true,
        "0" => false,
        other => panic!("unexpected assertion probe output: {other:?}"),
    }
}
