//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::{detect_pg_config, env_tracked, is_for_release};
use bindgen::NonCopyUnionStyle;
use bindgen::callbacks::{DeriveTrait, EnumVariantValue, ImplementsTrait, MacroParsingBehavior};
use eyre::{WrapErr, eyre};
use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, BuildInputs, CompilationProfile, Diagnostic, EmissionStatus,
    IntegerConstant, MacroEmission, MacroScanner, PostgresConfig, generate_with_bindings,
    pg_sys_integer_bridges, postgres_function_macro_names,
};
use pgrx_pg_config::{PgConfig, PgMinorVersion, PgVersion, Pgrx, SUPPORTED_VERSIONS};
use quote::{ToTokens, quote};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{self, Path, PathBuf}; // disambiguate path::Path and syn::Type::Path
use std::process::{Command, Output};
use std::rc::Rc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use syn::{Item, ItemConst, spanned::Spanned};

const BLOCKLISTED_TYPES: [&str; 4] = ["Datum", "NullableDatum", "Oid", "TransactionId"];

// clang's safe wrapper permits one live Clang handle per process. Bindgen's
// Binding generation shares that inspection's runtime and verified arguments.
/// Serialize macro inspection and bindgen generation so both share one usable process-wide
/// Clang runtime.
static MACRO_SCANNER: Mutex<()> = Mutex::new(());
/// Allocate unique owned staging paths while formatting different PostgreSQL versions
/// concurrently.
static NEXT_MACRO_FORMATTING_DIRECTORY: AtomicU64 = AtomicU64::new(0);

// These postgres versions were effectively "yanked" by the community, even tho they still exist
// in the wild.  pgrx will refuse to compile against them
const YANKED_POSTGRES_VERSIONS: &[PgVersion] = &[
    // this set of releases introduced an ABI break in the [`pg_sys::ResultRelInfo`] struct
    // and was replaced by the community on 2024-11-21
    // https://www.postgresql.org/about/news/postgresql-172-166-1510-1415-1318-and-1222-released-2965/
    PgVersion::new(17, PgMinorVersion::Release(1), None),
    PgVersion::new(16, PgMinorVersion::Release(5), None),
    PgVersion::new(15, PgMinorVersion::Release(9), None),
];

/// Reuse the binding build's collector so fixture tests reconcile exactly the Rust facts used
/// in production generation.
mod binding_symbols;
/// Render provenance-derived header modules and same-scope support fragments for publication.
mod macro_files;
/// Compile generated native access helpers under the already verified C invocation profile.
mod macro_support;
use macro_files::MacroFiles;
use macro_support::{NativeBuild, compile_macro_support};
pub(super) mod clang;

#[derive(Debug)]
struct BindingOverride {
    ignore_macros: HashSet<&'static str>,
    enum_names: InnerMut<EnumMap>,
}

type InnerMut<T> = Rc<RefCell<T>>;
type EnumMap = BTreeMap<String, Vec<(String, EnumVariantValue)>>;

impl BindingOverride {
    fn new_from(enum_names: InnerMut<EnumMap>) -> Self {
        // these cause duplicate definition problems on linux
        // see: https://github.com/rust-lang/rust-bindgen/issues/687
        Self {
            ignore_macros: HashSet::from_iter([
                "FP_INFINITE",
                "FP_NAN",
                "FP_NORMAL",
                "FP_SUBNORMAL",
                "FP_ZERO",
                "IPPORT_RESERVED",
                // These are just annoying due to clippy
                "M_E",
                "M_LOG2E",
                "M_LOG10E",
                "M_LN2",
                "M_LN10",
                "M_PI",
                "M_PI_2",
                "M_PI_4",
                "M_1_PI",
                "M_2_PI",
                "M_SQRT2",
                "M_SQRT1_2",
                "M_2_SQRTPI",
            ]),
            enum_names,
        }
    }
}

impl bindgen::callbacks::ParseCallbacks for BindingOverride {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        if self.ignore_macros.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }

    fn blocklisted_type_implements_trait(
        &self,
        name: &str,
        derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        if !BLOCKLISTED_TYPES.contains(&name) {
            return None;
        }

        let implements_trait = match derive_trait {
            DeriveTrait::Copy => ImplementsTrait::Yes,
            DeriveTrait::Debug => ImplementsTrait::Yes,
            _ => ImplementsTrait::No,
        };
        Some(implements_trait)
    }

    // FIXME: alter types on some int macros to the actually-used types so we can stop as-casting them
    fn int_macro(&self, _name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        None
    }

    // FIXME: implement a... C compiler?
    fn func_macro(&self, _name: &str, _value: &[&[u8]]) {}

    /// Intentionally doesn't do anything, just updates internal state.
    fn enum_variant_behavior(
        &self,
        enum_name: Option<&str>,
        variant_name: &str,
        variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<bindgen::callbacks::EnumVariantCustomBehavior> {
        enum_name.inspect(|name| match name.strip_prefix("enum").unwrap_or(name).trim() {
            // specifically overridden enum
            "NodeTag" => (),
            name if name.contains("unnamed at") || name.contains("anonymous at") => (),
            // to prevent problems with BuiltinOid
            _ if variant_name.contains("OID") => (),
            name => self
                .enum_names
                .borrow_mut()
                .entry(name.to_string())
                .or_default()
                .push((variant_name.to_string(), variant_value)),
        });
        None
    }

    // FIXME: hide nodetag fields and default them to appropriate values
    fn field_visibility(
        &self,
        _info: bindgen::callbacks::FieldInfo<'_>,
    ) -> Option<bindgen::FieldVisibilityKind> {
        None
    }
}

/// Drive binding generation for configured PostgreSQL versions and register the generated-macro
/// configuration with Cargo before compiling consumers.
pub fn main() -> eyre::Result<()> {
    println!("cargo:rustc-check-cfg=cfg(docsrs)");
    println!("cargo:rustc-check-cfg=cfg(pgrx_c_macros)");
    println!("cargo:rerun-if-env-changed=DOCS_RS");

    if env_tracked("DOCS_RS").as_deref() == Some("1") {
        println!("cargo:rustc-cfg=docsrs");
        return Ok(());
    }

    // dump the environment for debugging if asked
    if env_tracked("PGRX_BUILD_VERBOSE").as_deref() == Some("true") {
        for (k, v) in std::env::vars() {
            eprintln!("{k}={v}");
        }
    }

    let compile_cshim = env_tracked("CARGO_FEATURE_CSHIM").as_deref() == Some("1");
    let build_paths = BuildPaths::from_env();

    eprintln!("build_paths={build_paths:?}");

    emit_rerun_if_changed();

    let pg_configs = detect_pg_config()?;

    // make sure we're not trying to build any of the yanked postgres versions
    for (_, pg_config) in &pg_configs {
        let version = pg_config.get_version()?;
        if YANKED_POSTGRES_VERSIONS.contains(&version) {
            panic!(
                "Postgres v{}{} is incompatible with \
                    other versions in this major series and is not supported by pgrx.  Please upgrade \
                    to the latest version in the v{} series.",
                version.major, version.minor, version.major
            );
        }
    }

    let integrated_cshims = std::thread::scope(|scope| {
        // This is pretty much either always 1 (normally) or 5 (for releases),
        // but in the future if we ever have way more, we should consider
        // chunking `pg_configs` based on `thread::available_parallelism()`.
        let threads = pg_configs
            .iter()
            .map(|(pg_major_ver, pg_config)| {
                scope.spawn(|| {
                    generate_bindings(
                        *pg_major_ver,
                        pg_config,
                        &build_paths,
                        is_for_release(),
                        compile_cshim,
                    )
                    .map(|integrated| (*pg_major_ver, integrated))
                })
            })
            .collect::<Vec<_>>();
        // Most of the rest of this is just for better error handling --
        // `thread::scope` already joins the threads for us before it returns.
        let results = threads
            .into_iter()
            .map(|thread| thread.join().expect("thread panicked while generating bindings"))
            .collect::<Vec<eyre::Result<_>>>();
        results.into_iter().collect::<eyre::Result<Vec<_>>>()
    })?;

    if compile_cshim {
        let active_major_version = active_pg_major_version()?;
        let pg_config = pg_configs
            .iter()
            .find(|(major_version, _)| *major_version == active_major_version)
            .map(|(_, pg_config)| pg_config)
            .ok_or_else(|| {
                eyre!("could not find pg_config for active feature pg{active_major_version}")
            })?;
        if !integrated_cshims
            .iter()
            .any(|(major, integrated)| *major == active_major_version && *integrated)
        {
            build_shim(&build_paths.shim_src, &build_paths.shim_dst, pg_config)?;
        }
    }

    Ok(())
}

fn active_pg_major_version() -> eyre::Result<u16> {
    let found = SUPPORTED_VERSIONS()
        .iter()
        .filter_map(|pgver| {
            env_tracked(&format!("CARGO_FEATURE_PG{}", pgver.major)).map(|_| pgver.major)
        })
        .collect::<Vec<_>>();

    match &found[..] {
        [major_version] => Ok(*major_version),
        [] => Err(eyre!("did not find a pg$VERSION feature while compiling the cshim")),
        versions => Err(eyre!(
            "multiple pg$VERSION features found while compiling the cshim: {}",
            versions.iter().map(|version| format!("pg{version}")).collect::<Vec<_>>().join(", ")
        )),
    }
}

fn cshim_static_wrapper_name(major_version: u16) -> String {
    // release builds generate bindings for every supported pg version in one OUT_DIR
    // bindgen writes the static wrapper as a side file, so a shared name lets the
    // last writer win and makes the cshim compile the wrong wrapper against this pg_config
    format!("pgrx-cshim-static-pg{major_version}")
}

/// Register binding inputs without tracking an absent configuration file perpetually;
/// macro-specific tracking additionally follows inspected compiler and header inputs.
fn emit_rerun_if_changed() {
    // `pgrx-pg-config` doesn't emit one for this.
    println!("cargo:rerun-if-env-changed=PGRX_PG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=PGRX_PG_CONFIG_AS_ENV");
    // Bindgen's behavior depends on these vars, but it doesn't emit them
    // directly because the output would cause issue with `bindgen-cli`. Do it
    // on bindgen's behalf.
    println!("cargo:rerun-if-env-changed=LLVM_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=LIBCLANG_PATH");
    println!("cargo:rerun-if-env-changed=LIBCLANG_STATIC_PATH");
    // Follows the logic bindgen uses here, more or less.
    // https://github.com/rust-lang/rust-bindgen/blob/e6dd2c636/bindgen/lib.rs#L2918
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    if let Some(target) = env_tracked("TARGET") {
        println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_{target}");
        println!(
            "cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_{}",
            target.replace('-', "_"),
        );
    }

    // don't want to get stuck always generating bindings
    println!("cargo:rerun-if-env-changed=PGRX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE");

    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=pgrx-cshim.c");

    if let Ok(pgrx_config) = Pgrx::config_toml()
        && pgrx_config.is_file()
    {
        println!("cargo:rerun-if-changed={}", pgrx_config.display());
    }
}

/// Write the selected version's bindings, OIDs, macro module tree, and skip report, publishing
/// documentation snapshots only during release generation. Return whether this
/// version's native artifact already includes the C shim so it is built only once.
fn generate_bindings(
    major_version: u16,
    pg_config: &PgConfig,
    build_paths: &BuildPaths,
    is_for_release: bool,
    enable_cshim: bool,
) -> eyre::Result<bool> {
    let mut include_h = build_paths.manifest_dir.clone();
    include_h.push("include");
    include_h.push(format!("pg{major_version}.h"));

    let active = env_tracked(&format!("CARGO_FEATURE_PG{major_version}")).is_some();
    let native = NativeBuild {
        out_dir: &build_paths.out_dir,
        active,
        cshim: (active && enable_cshim).then_some(build_paths.shim_src.as_path()),
    };
    let (bindgen_output, macros) =
        get_bindings(major_version, pg_config, &include_h, enable_cshim, native)
            .wrap_err_with(|| format!("bindgen failed for pg{major_version}"))?;

    let oids = extract_oids(&bindgen_output);
    let rewritten_items = rewrite_items(bindgen_output, &oids)
        .wrap_err_with(|| format!("failed to rewrite items for pg{major_version}"))?;
    let oids = format_builtin_oid_impl(oids);

    let dest_dirs = if is_for_release {
        vec![build_paths.out_dir.clone(), build_paths.src_dir.clone()]
    } else {
        vec![build_paths.out_dir.clone()]
    };
    for dest_dir in dest_dirs {
        let mut bindings_file = dest_dir.clone();
        bindings_file.push(format!("pg{major_version}.rs"));
        write_rs_file(
            rewritten_items.clone(),
            &bindings_file,
            quote! {
                use crate as pg_sys;
                use crate::{Datum, MultiXactId, Oid, PgNode, TransactionId};
            },
            is_for_release,
        )
        .wrap_err_with(|| {
            format!(
                "Unable to write bindings file for pg{} to `{}`",
                major_version,
                bindings_file.display()
            )
        })?;

        let mut oids_file = dest_dir.clone();
        oids_file.push(format!("pg{major_version}_oids.rs"));
        write_rs_file(oids.clone(), &oids_file, quote! {}, is_for_release).wrap_err_with(|| {
            format!(
                "Unable to write oids file for pg{} to `{}`",
                major_version,
                oids_file.display()
            )
        })?;
    }

    write_macro_files(
        &macros.files,
        &build_paths.out_dir.join(format!("cmacros/pg{major_version}")),
        false,
    )?;
    remove_legacy_macro_file(&build_paths.out_dir, major_version)?;
    if is_for_release {
        write_macro_files(
            &macros.files,
            &build_paths.src_dir.join(format!("cmacros/pg{major_version}")),
            true,
        )?;
        remove_legacy_macro_file(&build_paths.src_dir, major_version)?;
    }
    let report = build_paths.out_dir.join(format!("pg{major_version}_macro_report.json"));
    write_content_stable(&report, &macros.report)?;
    if macro_debug_enabled() {
        if macros.inspected {
            println!(
                "cargo:warning=pg{major_version} C macros: {} emitted, {} skipped; report {}",
                macros.emitted,
                macros.skipped,
                report.display()
            );
        } else {
            println!(
                "cargo:warning=pg{major_version} C macros unavailable; report {}",
                report.display()
            );
        }
    }
    if macros.emitted != 0 && env_tracked(&format!("CARGO_FEATURE_PG{major_version}")).is_some() {
        println!("cargo:rustc-cfg=pgrx_c_macros");
    }

    let lib_dir = pg_config.lib_dir()?;
    println!(
        "cargo:rustc-link-search={}",
        lib_dir.to_str().ok_or_else(|| eyre!("{lib_dir:?} is not valid UTF-8 string"))?
    );
    Ok(macros.integrated_cshim)
}

/// Keep normal binding builds quiet while allowing developers to inspect macro
/// skip diagnostics. Tracking the switch makes Cargo rerun generation when its
/// value changes; reports are written independently of this output policy.
fn macro_debug_enabled() -> bool {
    env_tracked("PGRX_MACRO_DEBUG").as_deref() == Some("1")
}

/// Keep a skip diagnostic within one Cargo warning line. Compiler rejection
/// messages can contain newlines; they must not become additional build-script
/// directives. The full unmodified reason remains in the JSON report.
fn macro_skip_warning(major_version: u16, name: &str, message: &str) -> String {
    format!("pg{major_version}: skipping macro `{name}`: {}", message.replace(['\r', '\n'], " "))
}

#[derive(Debug, Clone)]
struct BuildPaths {
    /// CARGO_MANIFEST_DIR
    manifest_dir: PathBuf,
    /// OUT_DIR
    out_dir: PathBuf,
    /// {manifest_dir}/src/include
    src_dir: PathBuf,
    /// {manifest_dir}/pgrx-cshim.c
    shim_src: PathBuf,
    /// {out_dir}/pgrx-cshim.c
    shim_dst: PathBuf,
}

impl BuildPaths {
    fn from_env() -> Self {
        // Cargo guarantees these are provided, so unwrap is fine.
        let manifest_dir = env_tracked("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
        let out_dir = env_tracked("OUT_DIR").map(PathBuf::from).unwrap();
        Self {
            src_dir: manifest_dir.join("src/include"),
            shim_src: manifest_dir.join("pgrx-cshim.c"),
            shim_dst: out_dir.join("pgrx-cshim.c"),
            out_dir,
            manifest_dir,
        }
    }
}

/// Add documentation-only guards to installation-specific target checks and native adapters
/// while preserving original source comments and macro templates.
fn macro_snapshot(source: &str) -> eyre::Result<String> {
    let file = syn::parse_file(source).wrap_err("could not parse generated C macros")?;
    let mut guards = BTreeMap::new();
    for item in &file.items {
        if let Item::Macro(item) = item
            && item.mac.path.is_ident("compile_error")
        {
            // Shipped bindings describe the generation installation, which may
            // differ from docs.rs's target. Keep the original target guard for
            // anyone including this snapshot outside documentation builds.
            guards.insert(
                item.mac.path.span().start().line,
                ("compile_error!", "C macro target guard"),
            );
        } else if let Item::Mod(item) = item
            && item.ident == "__pgrx_c_generated"
            && !item.attrs.iter().any(|attr| attr.meta == syn::parse_quote!(cfg(not(docsrs))))
        {
            // Native adapters describe the generation installation's types and
            // layout. Documentation builds use shipped bindings and no C shims.
            guards.insert(
                item.mod_token.span().start().line,
                ("pub mod __pgrx_c_generated", "C macro native support module"),
            );
        }
    }
    // Keep the original source: passing through a token stream would discard the
    // PGRX comments that explain why a named symbol or macro remains expanded.
    let mut snapshot = String::from(
        "/* Automatically generated by pgrx-c-macros. Do not hand-edit.\n\nThis code is generated for documentation purposes, so that it is easy to reference on docs.rs. C macros and their support code are regenerated for your build of pgrx, and your Postgres configuration may differ.\n*/\n",
    );
    for (index, line) in source.split_inclusive('\n').enumerate() {
        if let Some((prefix, item)) = guards.get(&(index + 1)) {
            if !line.trim_start().starts_with(*prefix) {
                return Err(eyre!("{item} does not start on its own line"));
            }
            snapshot.push_str("#[cfg(not(docsrs))]\n");
        }
        snapshot.push_str(line);
    }
    Ok(snapshot)
}

/// Stage and format the complete macro tree before stable writes, then remove only stale Rust
/// leaves owned by this version.
fn write_macro_files(
    files: &MacroFiles,
    directory: &Path,
    documentation: bool,
) -> eyre::Result<()> {
    fs::create_dir_all(directory)?;
    let staging = MacroFormattingDirectory::new(directory)?;
    let mut paths = Vec::with_capacity(files.sources.len());
    for (relative, source) in &files.sources {
        let path = staging.0.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        if documentation {
            fs::write(&path, macro_snapshot(source)?)?;
        } else {
            fs::write(&path, source)?;
        }
        paths.push(path);
    }
    let rustfmt = env_tracked("RUSTFMT").unwrap_or_else(|| "rustfmt".into());
    format_macro_files(&paths, Path::new(&rustfmt))?;
    for relative in files.sources.keys() {
        let path = directory.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        // Compare the final representation: formatting after the stable write
        // would rewrite identical artifacts on every regeneration.
        let content = fs::read(staging.0.join(relative))?;
        write_content_stable(&path, &content)?;
    }
    // This version directory contains generated Rust only. Retain other files
    // and versions, and remove empty directories after their stale leaves.
    for entry in walkdir::WalkDir::new(directory).contents_first(true) {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(directory)?;
        if entry.file_type().is_file()
            && path.extension().is_some_and(|extension| extension == "rs")
            && !files.sources.contains_key(relative)
        {
            fs::remove_file(path)?;
        } else if entry.file_type().is_dir()
            && path != directory
            && fs::read_dir(path)?.next().is_none()
        {
            fs::remove_dir(path)?;
        }
    }
    Ok(())
}

/// A sibling keeps the final directory's rustfmt configuration search while
/// keeping temporary files outside that version's stale-file pruning.
struct MacroFormattingDirectory(
    /// Owned fixture path used for isolated inputs and cleanup.
    PathBuf,
);

/// Own formatting staging beside the final tree so rustfmt sees its configuration
/// while stale-file pruning remains confined to generated version output.
impl MacroFormattingDirectory {
    /// Create a sibling formatting directory so rustfmt finds the destination configuration
    /// without stale-file pruning observing the temporary files.
    fn new(directory: &Path) -> eyre::Result<Self> {
        let parent = directory.parent().filter(|parent| !parent.as_os_str().is_empty());
        let parent = parent.unwrap_or_else(|| Path::new("."));
        for _ in 0..100 {
            let number = NEXT_MACRO_FORMATTING_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!(".pgrx-c-macros-{}-{number}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => {
                    return Err(error).wrap_err("could not stage C macros for formatting");
                }
            }
        }
        Err(eyre!("could not create a unique C macro formatting directory"))
    }
}

/// Remove only the staging files allocated by this generation attempt on either
/// successful publication or an earlier formatting error.
impl Drop for MacroFormattingDirectory {
    /// Remove only the staging directory owned by this formatter instance after success or
    /// failure.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Recognize rustup's missing-component diagnostic separately from Rust source formatting errors.
///
/// A rustup proxy can exist even when its selected toolchain has no rustfmt component. Only
/// that specific leading diagnostic is an availability failure; another error remains fatal.
fn missing_rustfmt_component(stderr: &[u8]) -> bool {
    let diagnostic = String::from_utf8_lossy(stderr);
    let mut lines = diagnostic.lines();
    let Some(toolchain) = lines.next().and_then(|line| {
        line.strip_prefix("error: 'rustfmt' is not installed for the toolchain '")
    }) else {
        return false;
    };
    let toolchain = toolchain.strip_suffix('.').unwrap_or(toolchain);
    toolchain.strip_suffix('\'').is_some_and(|name| !name.is_empty() && !name.contains('\''))
        && !lines.any(|line| line.starts_with("error:"))
}

/// Format generated leaves together while preventing child traversal; tolerate an absent
/// executable or rustup component, but propagate actual formatter failures.
fn format_macro_files(paths: &[PathBuf], rustfmt: &Path) -> eyre::Result<()> {
    if paths.is_empty() {
        return Ok(());
    }
    let mut command = Command::new(rustfmt);
    command.args(paths).args(["--edition", "2024", "--config", "skip_children=true"]);
    match run_command(&mut command, "C macro formatting") {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) if missing_rustfmt_component(&output.stderr) => {
            // The rustup proxy failed before invoking a formatter, so the staged
            // original sources remain suitable for optional unformatted output.
            Ok(())
        }
        Ok(output) => Err(eyre!(
            "could not format generated C macros: {}",
            String::from_utf8_lossy(&output.stderr)
        )),
        Err(error)
            if error
                .downcast_ref::<std::io::Error>()
                .is_some_and(|error| error.kind() == std::io::ErrorKind::NotFound) =>
        {
            // Rustfmt is optional for macro generation. The staged original
            // source remains the output when the executable is unavailable.
            Ok(())
        }
        Err(error) => Err(error).wrap_err("could not start the C macro formatter"),
    }
}

/// Remove the obsolete monolithic macro file after publishing the modular tree, tolerating an
/// already-absent predecessor.
fn remove_legacy_macro_file(directory: &Path, major_version: u16) -> eyre::Result<()> {
    let path = directory.join(format!("pg{major_version}_macros.rs"));
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).wrap_err_with(|| format!("could not remove {}", path.display())),
    }
}

/// Write formatted binding source and release documentation snapshots using the
/// repository edition; modular C macro publication uses its separate stable tree writer.
fn write_rs_file(
    code: proc_macro2::TokenStream,
    file_path: &Path,
    header: proc_macro2::TokenStream,
    is_for_release: bool,
) -> eyre::Result<()> {
    use std::io::Write;
    let mut contents = header;
    contents.extend(code);
    let mut file = fs::File::create(file_path)?;
    write!(file, "/* Automatically generated by bindgen. Do not hand-edit.")?;
    if is_for_release {
        write!(
            file,
            "\n
        This code is generated for documentation purposes, so that it is
        easy to reference on docs.rs. Bindings are regenerated for your
        build of pgrx, and the values of your Postgres version may differ.
        */"
        )
    } else {
        write!(file, " */")
    }?;
    write!(file, "{contents}")?;
    rust_fmt(file_path, "2021")
}

/// Given a token stream representing a file, apply a series of transformations to munge
/// the bindgen generated code with some postgres specific enhancements
fn rewrite_items(
    mut file: syn::File,
    oids: &BTreeMap<syn::Ident, Box<syn::Expr>>,
) -> eyre::Result<proc_macro2::TokenStream> {
    rewrite_c_abi_to_c_unwind(&mut file);
    let items_vec = rewrite_oid_consts(&file.items, oids);
    let mut items = apply_pg_guard(&items_vec)?;
    let pgnode_impls = impl_pg_node(&items_vec)?;

    // append the pgnodes to the set of items
    items.extend(pgnode_impls);

    Ok(items)
}

/// Find all the constants that represent Postgres type OID values.
///
/// These are constants of type `u32` whose name ends in the string "OID"
fn extract_oids(code: &syn::File) -> BTreeMap<syn::Ident, Box<syn::Expr>> {
    let mut oids = BTreeMap::new(); // we would like to have a nice sorted set
    for item in &code.items {
        let Item::Const(ItemConst { ident, ty, expr, .. }) = item else { continue };
        // Retype as strings for easy comparison
        let name = ident.to_string();
        let ty_str = ty.to_token_stream().to_string();

        // This heuristic identifies "OIDs"
        // We're going to warp the const declarations to be our newtype Oid
        if ty_str == "u32" && is_builtin_oid(&name) {
            oids.insert(ident.clone(), expr.clone());
        }
    }
    oids
}

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

fn rewrite_oid_consts(
    items: &[syn::Item],
    oids: &BTreeMap<syn::Ident, Box<syn::Expr>>,
) -> Vec<syn::Item> {
    items
        .iter()
        .map(|item| match item {
            Item::Const(ItemConst { ident, ty, expr, .. })
                if ty.to_token_stream().to_string() == "u32" && oids.get(ident) == Some(expr) =>
            {
                syn::parse2(quote! { pub const #ident : Oid = Oid(#expr); }).unwrap()
            }
            item => item.clone(),
        })
        .collect()
}

fn format_builtin_oid_impl(oids: BTreeMap<syn::Ident, Box<syn::Expr>>) -> proc_macro2::TokenStream {
    let enum_variants: proc_macro2::TokenStream;
    let from_impl: proc_macro2::TokenStream;
    (enum_variants, from_impl) = oids
        .iter()
        .map(|(ident, expr)| {
            (quote! { #ident = #expr, }, quote! { #expr => Ok(BuiltinOid::#ident), })
        })
        .unzip();

    quote! {
        use crate::{NotBuiltinOid};

        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
        pub enum BuiltinOid {
            #enum_variants
        }

        impl BuiltinOid {
            pub const fn from_u32(uint: u32) -> Result<BuiltinOid, NotBuiltinOid> {
                match uint {
                    0 => Err(NotBuiltinOid::Invalid),
                    #from_impl
                    _ => Err(NotBuiltinOid::Ambiguous),
                }
            }
        }
    }
}

/// Implement our `PgNode` marker trait for `pg_sys::Node` and its "subclasses"
fn impl_pg_node(items: &[syn::Item]) -> eyre::Result<proc_macro2::TokenStream> {
    let type_graph = TypeGraph::from(items);

    // Look through the entire file to produce a set of all variants of the Postgres `NodeTag` enum.
    // Also look at type aliases of structs/unions, as these could be node types, too.
    let mut node_tags: BTreeSet<String> = BTreeSet::new();
    let mut possible_alias_tags = HashMap::new();
    for item in items {
        match item {
            // the `NodeTag` enum
            syn::Item::Enum(item_enum) if item_enum.ident == "NodeTag" => {
                node_tags.extend(item_enum.variants.iter().map(|v| v.ident.to_string()))
            }
            // one type alias of a struct/union; e.g. `pub type DistinctExpr = OpExpr`
            syn::Item::Type(item_type)
                if let syn::Type::Path(p) = &*item_type.ty
                    && let Some(last) = p.path.segments.last()
                    && type_graph.name_tab.contains_key(&last.ident.to_string()) =>
            {
                let target_name = last.ident.to_string();
                let alias_name = item_type.ident.to_string();
                let tag_name = format!("T_{}", alias_name);
                possible_alias_tags
                    .entry(target_name)
                    .or_insert_with(BTreeSet::new)
                    .insert(tag_name);
            }
            _ => continue,
        }
    }

    // Identify the root nodes of the Postgres inheritance hierarchy and recursively resolve the
    // cast tags for them and their subclasses. The `BTreeMap` returns nodes in alphabetical order
    // when we emit the trait implementations.
    let mut identified_nodes = BTreeMap::new();
    for descriptor in type_graph.descriptors.iter() {
        let is_node = match descriptor.kind {
            // a node struct has a `NodeTag` for its first field
            TypeKind::Struct(struct_) => {
                let first_field = if let syn::Fields::Named(fields) = &struct_.fields {
                    fields.named.first()
                } else if let syn::Fields::Unnamed(fields) = &struct_.fields {
                    fields.unnamed.first()
                } else {
                    None
                };

                if let Some(first_field) = first_field
                    && let syn::Type::Path(p) = &first_field.ty
                    && let Some(last) = p.path.segments.last()
                {
                    last.ident == "NodeTag"
                } else {
                    false
                }
            }
            // a node union has one member that is a `Node`
            TypeKind::Union(union_) => union_.fields.named.iter().any(|field| {
                if let syn::Type::Path(p) = &field.ty
                    && let Some(last) = p.path.segments.last()
                {
                    last.ident == "Node"
                } else {
                    false
                }
            }),
        };

        if is_node {
            resolve_pg_node_tags(
                descriptor,
                &type_graph,
                &node_tags,
                &possible_alias_tags,
                &mut identified_nodes,
            );
        }
    }

    // Finally, emit `PgNode` implementations for every detected Node.
    let mut impls = proc_macro2::TokenStream::new();
    for (type_name, cast_tags) in identified_nodes {
        let ident_type_name = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
        let ident_cast_tags: Vec<syn::Ident> =
            cast_tags.iter().map(|t| syn::Ident::new(t, proc_macro2::Span::call_site())).collect();

        // Seal every Node.
        impls.extend(quote! {
            impl pg_sys::seal::Sealed for #ident_type_name {}
        });

        // Implement PgNode for every Node.
        impls.extend(match type_name.as_str() {
            // Override the default implementation of `try_cast` for Node.
            "Node" => quote! {
                impl pg_sys::PgNode for #ident_type_name {
                    const CAST_TAGS: &'static [pg_sys::NodeTag] = &[];

                    #[inline]
                    fn try_cast<T: pg_sys::PgNode>(node: &T) -> Option<&Self> {
                        Some(node.as_node())
                    }
                }
            },
            // Use the default implementation of `try_as` with populated CAST_TAGS.
            _ => quote! {
                impl pg_sys::PgNode for #ident_type_name {
                    const CAST_TAGS: &'static [pg_sys::NodeTag] = &[
                        #(pg_sys::NodeTag::#ident_cast_tags),*
                    ];
                }
            },
        });

        // Implement Display for every Node.
        impls.extend(quote! {
            impl ::core::fmt::Display for #ident_type_name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    self.display_node().fmt(f)
                }
            }
        });
    }

    Ok(impls)
}

/// Recursively traverse a Node's subclasses and return the union of cast node tags.
/// At the same time, collect results into `identified_nodes`.
fn resolve_pg_node_tags<'graph>(
    descriptor: &'graph TypeDescriptor<'graph>,
    type_graph: &'graph TypeGraph<'graph>,
    node_tags: &BTreeSet<String>,
    possible_alias_tags: &HashMap<String, BTreeSet<String>>,
    identified_nodes: &mut BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    let type_name = descriptor.ident.to_string();
    if let Some(tags) = identified_nodes.get(&type_name) {
        return tags.clone();
    }

    let mut cast_tags = BTreeSet::new();

    // Start with the type name. Any Node with this tag can cast to this type.
    let possible_tag_name = format!("T_{}", type_name);
    if node_tags.contains(&possible_tag_name) {
        cast_tags.insert(possible_tag_name);
    }

    // Any Node with the tag of a typedef alias can also cast to this type.
    if let Some(possible_tags) = possible_alias_tags.get(&type_name) {
        for possible_tag_name in possible_tags {
            if node_tags.contains(possible_tag_name) {
                cast_tags.insert(possible_tag_name.clone());
            }
        }
    }

    // Unions do not inherit their member's node tags because it is not always safe to cast in that
    // direction. The sizeof a union member may be smaller than the union, and casting from the
    // former to the latter leads to out-of-bounds reads and UB. The CAST_TAGS of a union should
    // contain, at most, its own node tag and aliases.
    if let TypeKind::Struct(_) = descriptor.kind {
        for child in descriptor.children(type_graph) {
            cast_tags.extend(resolve_pg_node_tags(
                child,
                type_graph,
                node_tags,
                possible_alias_tags,
                identified_nodes,
            ));
        }
    }

    // Register this Node and its resolved tags in the final result set.
    identified_nodes.insert(type_name, cast_tags.clone());

    // Return this Nodes' resolved tags.
    cast_tags
}

#[derive(Clone, Debug)]
enum TypeKind<'a> {
    Struct(&'a syn::ItemStruct),
    Union(&'a syn::ItemUnion),
}

/// A graph describing the inheritance relationships between different types
/// according to postgres' object system.
///
/// NOTE: the borrowed lifetime on a TypeGraph should also ensure that the offsets
///       it stores into the underlying items struct are always correct.
#[derive(Clone, Debug)]
struct TypeGraph<'a> {
    #[allow(dead_code)]
    /// A table mapping type names to their offset in the descriptor table
    name_tab: HashMap<String, usize>,
    #[allow(dead_code)]
    /// A table mapping offsets into the underlying items table to offsets in the descriptor table
    item_offset_tab: Vec<Option<usize>>,
    /// A table of type descriptors
    descriptors: Vec<TypeDescriptor<'a>>,
}

impl<'a> From<&'a [syn::Item]> for TypeGraph<'a> {
    fn from(items: &'a [syn::Item]) -> Self {
        let mut descriptors = Vec::new();

        // a table mapping type names to their offset in `descriptors`
        let mut name_tab: HashMap<String, usize> = HashMap::new();
        let mut item_offset_tab: Vec<Option<usize>> = vec![None; items.len()];
        for (i, item) in items.iter().enumerate() {
            let (kind, ident) = match item {
                syn::Item::Struct(struct_) => (TypeKind::Struct(struct_), struct_.ident.clone()),
                syn::Item::Union(union_) => (TypeKind::Union(union_), union_.ident.clone()),
                _ => continue,
            };

            let next_offset = descriptors.len();
            descriptors.push(TypeDescriptor {
                kind,
                ident: ident.clone(),
                items_offset: i,
                parent: None,
                children: Vec::new(),
            });
            name_tab.insert(ident.to_string(), next_offset);
            item_offset_tab[i] = Some(next_offset);
        }

        for item in items.iter() {
            match item {
                // Structs represent Postgres' single-inheritance hierarchy: when the first field of
                // a node struct is another node struct, the former "inherits" from the latter. The
                // first field of a struct type is its parent type.
                syn::Item::Struct(struct_) => {
                    let first_field = if let syn::Fields::Named(fields) = &struct_.fields {
                        fields.named.first()
                    } else if let syn::Fields::Unnamed(fields) = &struct_.fields {
                        fields.unnamed.first()
                    } else {
                        None
                    };

                    if let Some(first_field) = first_field
                        && let syn::Type::Path(p) = &first_field.ty
                        && let Some(last_segment) = p.path.segments.last()
                        && let Some(parent_offset) = name_tab.get(&last_segment.ident.to_string())
                    {
                        let child_offset = name_tab[&struct_.ident.to_string()];
                        descriptors[child_offset].parent = Some(*parent_offset);
                        descriptors[*parent_offset].children.push(child_offset);
                    }
                }
                // Unions represent a polymorphic container where each field is a subclass of the
                // union. The union is the (abstract) parent type of the field types.
                syn::Item::Union(union_) => {
                    let union_offset = name_tab[&union_.ident.to_string()];
                    for field in &union_.fields.named {
                        if let syn::Type::Path(p) = &field.ty
                            && let Some(last_segment) = p.path.segments.last()
                            && let Some(child_offset) =
                                name_tab.get(&last_segment.ident.to_string())
                        {
                            descriptors[*child_offset].parent = Some(union_offset);
                            descriptors[union_offset].children.push(*child_offset);
                        }
                    }
                }
                _ => continue,
            }
        }

        TypeGraph { name_tab, item_offset_tab, descriptors }
    }
}

impl<'a> TypeDescriptor<'a> {
    /// children returns an iterator over the children of this node in the graph
    fn children(&'a self, graph: &'a TypeGraph) -> TypeDescriptorChildren<'a> {
        TypeDescriptorChildren { offset: 0, descriptor: self, graph }
    }
}

/// An iterator over a TypeDescriptor's children
struct TypeDescriptorChildren<'a> {
    offset: usize,
    descriptor: &'a TypeDescriptor<'a>,
    graph: &'a TypeGraph<'a>,
}

impl<'a> std::iter::Iterator for TypeDescriptorChildren<'a> {
    type Item = &'a TypeDescriptor<'a>;
    fn next(&mut self) -> Option<&'a TypeDescriptor<'a>> {
        if self.offset >= self.descriptor.children.len() {
            None
        } else {
            let ret = Some(&self.graph.descriptors[self.descriptor.children[self.offset]]);
            self.offset += 1;
            ret
        }
    }
}

/// A node in a TypeGraph
#[derive(Clone, Debug)]
struct TypeDescriptor<'a> {
    /// The kind of type (Struct or Union)
    kind: TypeKind<'a>,
    /// The identifier of the type
    ident: syn::Ident,
    #[allow(dead_code)]
    /// An offset into the items slice that was used to construct the struct graph that
    /// this TypeDescriptor is a part of
    items_offset: usize,
    /// The offset of the "parent" struct/union (if any).
    parent: Option<usize>,
    /// The offsets of the "children" structs/unions (if any).
    children: Vec<usize>,
}

/// Choose normal generation or an unavailable macro result when externally supplied bindings
/// lack the current C inspection facts.
fn get_bindings(
    major_version: u16,
    pg_config: &PgConfig,
    include_h: &path::Path,
    enable_cshim: bool,
    native: NativeBuild<'_>,
) -> eyre::Result<(syn::File, MacroOutput)> {
    let (bindings, macros) = if let Some(info_dir) =
        target_env_tracked(&format!("PGRX_TARGET_INFO_PATH_PG{major_version}"))
    {
        let bindings_file = format!("{info_dir}/pg{major_version}_raw_bindings.rs");
        cargo_input_path(Path::new(&bindings_file))?;
        println!("cargo:rerun-if-changed={bindings_file}");
        let bindings = std::fs::read_to_string(&bindings_file)
            .wrap_err_with(|| format!("failed to read raw bindings from {bindings_file}"))?;
        let reason = "precomputed target bindings do not include an inspected C macro profile; macro generation is unavailable without matching target headers and metadata";
        if macro_debug_enabled() {
            println!("cargo:warning=pg{major_version}: {reason}");
        }
        (bindings, MacroOutput::unavailable(major_version, reason)?)
    } else {
        let (bindings, macros) =
            run_bindgen(major_version, pg_config, include_h, enable_cshim, native)?;
        if let Some(path) = env_tracked("PGRX_PG_SYS_EXTRA_OUTPUT_PATH") {
            std::fs::write(path, &bindings)?;
        }
        (bindings, macros)
    };
    let bindings = syn::parse_file(bindings.as_str())
        .wrap_err_with(|| "failed to parse generated bindings")?;
    Ok((bindings, macros))
}

/// Given a specific postgres version, `run_bindgen` generates bindings for the given
/// postgres version and returns them as a token stream.
fn run_bindgen(
    major_version: u16,
    pg_config: &PgConfig,
    include_h: &path::Path,
    enable_cshim: bool,
    native: NativeBuild<'_>,
) -> eyre::Result<(String, MacroOutput)> {
    eprintln!("Generating bindings for pg{major_version}");
    let configure = pg_config.configure()?;
    let explicit_clang = env_tracked("CLANG_PATH").map(PathBuf::from);
    let configured_clang =
        configure.get("CLANG").filter(|value| !value.is_empty()).map(PathBuf::from);
    let preferred_clang = explicit_clang.as_deref().or(configured_clang.as_deref());
    eprintln!("Preferred Clang = {preferred_clang:?}");
    let pg_target_includes = pg_target_includes(major_version, pg_config)?;
    eprintln!("pg_target_includes = {pg_target_includes:?}");
    let (autodetect, includes) = clang::detect_include_paths_for(preferred_clang);
    let mut binder = bindgen::Builder::default();
    binder = add_blocklists(binder, major_version, enable_cshim);
    binder = add_allowlists(binder, pg_target_includes.iter().map(|x| x.as_str()));
    binder = add_derives(binder);
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let windows = env_tracked("CARGO_CFG_TARGET_OS").as_deref() == Some("windows");
    let mut arguments = Vec::new();
    if !windows {
        arguments.extend(postgres_cflags(pg_config)?);
    }
    if !autodetect {
        for include in includes {
            arguments.push(format!(
                "-I{}",
                include.to_str().ok_or_else(|| eyre!("Clang include directory is not UTF-8"))?
            ));
        }
    }
    arguments.extend(extra_bindgen_clang_args(pg_config)?);
    arguments.extend(pg_target_includes.iter().map(|include| format!("-I{include}")));
    let environment_arguments = bindgen_environment_arguments(env_tracked);
    // Make bindgen's otherwise implicit Cargo target explicit for both inspections.
    if !windows
        && !has_explicit_clang_target(arguments.iter().chain(&environment_arguments))
        && let Some(target) = env_tracked("TARGET")
    {
        arguments.insert(0, format!("--target={}", clang_target(&target)));
    }
    let enum_names = Rc::new(RefCell::new(BTreeMap::new()));
    let overrides = BindingOverride::new_from(Rc::clone(&enum_names));
    let generate = |arguments: Vec<String>| -> eyre::Result<String> {
        let bindings = binder
            .header(include_h.display().to_string())
            .clang_args(arguments)
            .detect_include_paths(windows && autodetect)
            .parse_callbacks(Box::new(overrides))
            .default_enum_style(bindgen::EnumVariation::ModuleConsts)
            // The NodeTag enum is closed: additions break existing values in the set, so it is not extensible
            .rustified_non_exhaustive_enum("NodeTag")
            .size_t_is_usize(true)
            .merge_extern_blocks(true)
            .wrap_unsafe_ops(true)
            .use_core()
            .generate_cstr(true)
            .disable_nested_struct_naming()
            .formatter(bindgen::Formatter::None)
            .layout_tests(false)
            .default_non_copy_union_style(NonCopyUnionStyle::ManuallyDrop)
            .wrap_static_fns(enable_cshim)
            .wrap_static_fns_path(out_path.join(cshim_static_wrapper_name(major_version)))
            .wrap_static_fns_suffix("__pgrx_cshim")
            .generate()
            .wrap_err_with(|| format!("Unable to generate bindings for pg{major_version}"))?;

        Ok(bindings.to_string())
    };
    if windows {
        let reason = "C macro generation is unavailable for Windows/MSVC profiles; existing binding generation is unchanged";
        if macro_debug_enabled() {
            println!("cargo:warning=pg{major_version}: {reason}");
        }
        Ok((generate(arguments)?, MacroOutput::unavailable(major_version, reason)?))
    } else {
        generate_macros(
            pg_config,
            include_h,
            arguments,
            &environment_arguments,
            explicit_clang.as_deref(),
            native,
            generate,
        )
    }
}

/// Carry the generated macro tree and audit report from inspection through binding publication.
struct MacroOutput {
    /// Versioned macro tree and adapters ready for formatted publication.
    files: MacroFiles,
    /// Serialized compiler facts and macro skip diagnostics written alongside the bindings.
    report: Vec<u8>,
    /// Successful public definitions counted for the Cargo summary.
    emitted: usize,
    /// Rejected candidates counted without treating unsupported syntax as build failure.
    skipped: usize,
    /// Whether authoritative C inspection succeeded, distinguishing unavailability from an
    /// empty selection.
    inspected: bool,
    /// Whether the active version's macro artifact owns the C shim as well,
    /// preventing a second object from defining the same header functions.
    integrated_cshim: bool,
}

/// Represent unavailable authoritative C inspection without publishing guessed
/// macro definitions or treating shipped documentation bindings as compiler evidence.
impl MacroOutput {
    /// Create an empty macro tree and explicit report when authoritative C inspection is
    /// unavailable, rather than deriving facts from shipped bindings.
    fn unavailable(major_version: u16, reason: &str) -> eyre::Result<Self> {
        let report = serde_json::to_vec_pretty(&serde_json::json!({
            "postgres_major_version": major_version,
            "status": "unavailable",
            "reason": reason,
        }))?;
        Ok(Self {
            files: MacroFiles::empty(),
            report,
            emitted: 0,
            skipped: 0,
            inspected: false,
            integrated_cshim: false,
        })
    }
}

/// Serialize the verified C invocation, fresh binding facts, and per-macro results for build
/// diagnostics and auditing.
#[derive(Serialize)]
struct MacroReport<'a> {
    /// Installation version associated with these compiler and binding facts.
    postgres_major_version: u16,
    /// Report status distinguishing generated output from unavailable inspection.
    status: &'static str,
    /// Verified compiler invocation, target, and semantic flags used for this generation.
    profile: &'a CompilationProfile,
    /// Header, compiler, search-root, and environment dependencies needed to invalidate output.
    inputs: &'a BuildInputs,
    /// Original frontend warnings retained for audit without repairing the source.
    diagnostics: &'a [Diagnostic],
    /// Every selected candidate, including provenance and structured emission or skip result.
    macros: &'a [MacroEmission],
    /// Compiler-owned constant facts used to verify symbolic binding references.
    integer_constants: &'a BTreeMap<String, IntegerConstant>,
    /// Fresh Rust binding paths, storage, and values reconciled with the C catalog.
    integer_bindings: &'a BindingCatalog,
    /// Explanation when checked pg_sys integer bridges could not be proved.
    integer_bridge_unavailable: Option<String>,
}

/// Inspect the selected installation, reconcile fresh bindings, emit supported macros and
/// native adapters, verify unchanged C inputs, and serialize structured skips.
fn generate_macros(
    pg_config: &PgConfig,
    header: &Path,
    binder_arguments: Vec<String>,
    environment_arguments: &[String],
    preferred_clang: Option<&Path>,
    native: NativeBuild<'_>,
    generate_bindings: impl FnOnce(Vec<String>) -> eyre::Result<String>,
) -> eyre::Result<(String, MacroOutput)> {
    let _lock = MACRO_SCANNER.lock().map_err(|_| eyre!("C macro scanner lock was poisoned"))?;
    let scanner = MacroScanner::new().wrap_err("could not initialize C macro discovery")?;
    let postgres = PostgresConfig::from_pg_config(pg_config.clone())?;
    let major_version = pg_config.major_version()?;
    let mut effective_arguments = binder_arguments.clone();
    effective_arguments.extend_from_slice(environment_arguments);
    let mut frontend = postgres
        .inspect_with_arguments(&scanner, header, &effective_arguments, preferred_clang)
        .wrap_err("could not establish the binding generator's C macro compilation profile")?;
    // Inspection pins the matching compiler's resource directory. Bindgen still
    // appends its environment tail, so normalize that resource option before the
    // tail and inspect again only when this changes the complete argument order.
    let (binder_arguments, normalized_arguments) = normalize_bindgen_arguments(
        &binder_arguments,
        environment_arguments,
        &frontend.profile().arguments,
    )?;
    if frontend.profile().arguments != normalized_arguments {
        frontend = postgres
            .inspect_with_arguments(&scanner, header, &normalized_arguments, preferred_clang)
            .wrap_err("could not verify normalized Clang resource and environment arguments")?;
        if frontend.profile().arguments != normalized_arguments {
            return Err(eyre!("C macro inspection changed an already resolved argument vector"));
        }
    }
    let names = postgres_function_macro_names(&frontend, postgres.server_include_dir())?;
    let session = AnalysisSession::prepare(&scanner, &frontend, &names)?;
    emit_macro_rerun_inputs(session.inputs(), native.out_dir)?;
    let mut source = String::new();
    let integer_bridge_unavailable = match pg_sys_integer_bridges(&frontend) {
        Ok(bridges) => {
            source.push_str(&bridges);
            None
        }
        Err(error) => Some(error.to_string()),
    };
    let bindings = generate_bindings(binder_arguments)?;
    session.verify_inputs().wrap_err("C inputs changed during binding generation")?;
    let mut parsed_bindings =
        syn::parse_file(&bindings).wrap_err("could not parse bindings for C symbol references")?;
    // Callback storage facts must reflect the same ABI rewrite as the final
    // bindings, while foreign function guards are still generated afterward.
    rewrite_c_abi_to_c_unwind(&mut parsed_bindings);
    let mut symbols = binding_symbols::collect_bindings(
        &parsed_bindings,
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    symbols.ffi_boundary = Some(vec!["ffi".into(), "pg_guard_ffi_boundary".into()]);
    if integer_bridge_unavailable.is_none() {
        symbols.integer_storage.extend([
            ("Oid".into(), pgrx_c_macros::IntegerKind::UnsignedInt),
            ("TransactionId".into(), pgrx_c_macros::IntegerKind::UnsignedInt),
            ("MultiXactId".into(), pgrx_c_macros::IntegerKind::UnsignedInt),
        ]);
        if let Some(pgrx_c_macros::TypeInfo {
            category: pgrx_c_macros::TypeCategory::Integer(kind),
            ..
        }) = frontend.declarations().types.get("Datum")
        {
            symbols.integer_storage.insert("Datum".into(), *kind);
        }
    }
    let pgrx_c_macros::MacroGeneration { macros: emissions, support } =
        generate_with_bindings(&session, &names, &symbols).map_err(|message| eyre!(message))?;
    source.push_str(&support.rust);
    let integrated_cshim = if native.active && !support.c_source.is_empty() {
        compile_macro_support(major_version, frontend.profile(), &support.c_source, &native)?
    } else {
        false
    };
    // Native access primitives reread the original headers. Do not publish Rust
    // facts from one snapshot alongside C object code compiled from another.
    session.verify_inputs().wrap_err("C inputs changed during macro support generation")?;
    let mut emitted = 0;
    let mut skipped = 0;
    let macro_debug = macro_debug_enabled();
    for emission in &emissions {
        match &emission.status {
            EmissionStatus::Emitted { .. } => {
                emitted += 1;
                symbols.macros.insert(emission.analysis.name.clone());
            }
            EmissionStatus::Skipped { reason } => {
                skipped += 1;
                if macro_debug {
                    println!(
                        "cargo:warning={}",
                        macro_skip_warning(major_version, &emission.analysis.name, &reason.message)
                    );
                }
            }
        }
    }
    let files = MacroFiles::new(source, &emissions, postgres.server_include_dir())?;
    let report = serde_json::to_vec_pretty(&MacroReport {
        postgres_major_version: major_version,
        status: "generated",
        profile: frontend.profile(),
        inputs: session.inputs(),
        diagnostics: &frontend.inventory().diagnostics,
        macros: &emissions,
        integer_constants: session.integer_constants(),
        integer_bindings: &symbols,
        integer_bridge_unavailable,
    })?;
    Ok((
        bindings,
        MacroOutput { files, report, emitted, skipped, inspected: true, integrated_cshim },
    ))
}

/// Place inspected resource options before bindgen's environment tail and require inspection to
/// retain the original supplied prefix.
fn normalize_bindgen_arguments(
    base: &[String],
    environment: &[String],
    inspected: &[String],
) -> eyre::Result<(Vec<String>, Vec<String>)> {
    let supplied_length = base.len() + environment.len();
    let supplied = inspected
        .get(..supplied_length)
        .filter(|supplied| supplied.starts_with(base) && &supplied[base.len()..] == environment)
        .ok_or_else(|| eyre!("C macro inspection did not preserve the supplied argument prefix"))?;
    let mut binder = base.to_vec();
    binder.extend_from_slice(&inspected[supplied.len()..]);
    let mut effective = binder.clone();
    effective.extend_from_slice(environment);
    Ok((binder, effective))
}

/// Match bindgen 0.72's target-specific lookup and malformed-quoting fallback.
fn bindgen_environment_arguments(mut lookup: impl FnMut(&str) -> Option<String>) -> Vec<String> {
    let target = lookup("TARGET");
    let value = target
        .as_ref()
        .and_then(|target| {
            lookup(&format!("BINDGEN_EXTRA_CLANG_ARGS_{target}")).or_else(|| {
                lookup(&format!("BINDGEN_EXTRA_CLANG_ARGS_{}", target.replace('-', "_")))
            })
        })
        .or_else(|| lookup("BINDGEN_EXTRA_CLANG_ARGS"));
    value.map_or_else(Vec::new, |value| shlex::split(&value).unwrap_or_else(|| vec![value]))
}

/// Recognize user target flags so Cargo's default target does not override an explicitly
/// selected Clang ABI.
fn has_explicit_clang_target<'a>(arguments: impl Iterator<Item = &'a String>) -> bool {
    let mut arguments = arguments;
    while let Some(argument) = arguments.next() {
        if argument.starts_with("--target=")
            || (argument == "-target" && arguments.next().is_some())
        {
            return true;
        }
    }
    false
}

// Bindgen applies these target spelling conversions before giving Cargo's target
// to Clang. Pin the same spelling explicitly so macro inspection sees that target.
/// Apply bindgen's target spelling conversions before inspection so Clang and binding
/// generation select the same ABI.
fn clang_target(target: &str) -> String {
    let mut parts = target.split_terminator('-').collect::<Vec<_>>();
    parts.resize(4, "");
    if parts[0].starts_with("riscv32") {
        parts[0] = "riscv32";
    } else if parts[0].starts_with("riscv64") {
        parts[0] = "riscv64";
    }
    if parts[1] == "apple" {
        if parts[0] == "aarch64" {
            parts[0] = "arm64";
        }
        if parts[3] == "sim" {
            parts[3] = "simulator";
        }
    }
    if parts[2] == "espidf" {
        parts[2] = "elf";
    }
    parts.into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join("-")
}

/// Publish the verified file, directory, and environment dependencies that must invalidate
/// generated C macros.
fn emit_macro_rerun_inputs(inputs: &BuildInputs, out_dir: &Path) -> eyre::Result<()> {
    for path in macro_rerun_paths(inputs, out_dir)? {
        cargo_input_path(&path)?;
        println!("cargo:rerun-if-changed={}", path.display());
    }
    for name in inputs.environment.keys() {
        if name.contains(['\n', '\r']) {
            return Err(eyre!("invalid Cargo environment dependency name"));
        }
        println!("cargo:rerun-if-env-changed={name}");
    }
    Ok(())
}

/// Combine present inspected files with safe directory watches for optional inputs and compiler
/// search roots.
fn macro_rerun_paths(inputs: &BuildInputs, out_dir: &Path) -> eyre::Result<BTreeSet<PathBuf>> {
    let mut paths = macro_watch_directories(inputs, out_dir)?;
    paths.extend(
        inputs
            .files
            .iter()
            .filter(|file| !matches!(inputs.fingerprints.get(*file), Some(None)))
            .cloned(),
    );
    Ok(paths)
}

/// Watch optional input creation and search-path changes without recursively watching this
/// build's own outputs.
fn macro_watch_directories(
    inputs: &BuildInputs,
    out_dir: &Path,
) -> eyre::Result<BTreeSet<PathBuf>> {
    let mut out_dirs = vec![out_dir.to_owned()];
    if let Ok(identity) = out_dir.canonicalize() {
        out_dirs.push(identity);
    }
    let mut watched = BTreeSet::new();
    for directory in &inputs.directories {
        watch_input_directory(&mut watched, directory, &out_dirs)?;
    }
    for file in &inputs.files {
        if matches!(inputs.fingerprints.get(file), Some(None)) {
            let parent = file
                .parent()
                .ok_or_else(|| eyre!("absent C macro input {} has no parent", file.display()))?;
            // Cargo marks a missing file perpetually dirty. Watch its nearest
            // existing parent so creation still invalidates the inspected profile.
            watch_input_directory(&mut watched, parent, &out_dirs)?;
        }
    }
    for directory in &inputs.executable_search_directories {
        watch_input_directory(&mut watched, directory, &out_dirs).wrap_err_with(|| {
            format!(
                "cannot track compiler PATH search root {} safely; set CLANG_PATH to an absolute compiler path to avoid PATH discovery",
                directory.display()
            )
        })?;
    }
    Ok(watched)
}

/// Track both supplied and canonical directory identities and reject overlap with OUT_DIR to
/// prevent self-invalidating builds.
fn watch_input_directory(
    watched: &mut BTreeSet<PathBuf>,
    requested: &Path,
    out_dirs: &[PathBuf],
) -> eyre::Result<()> {
    let directory = existing_directory_ancestor(requested)?;
    let identity = directory.canonicalize().wrap_err_with(|| {
        format!("could not resolve C macro input directory {}", directory.display())
    })?;
    for path in [&directory, &identity] {
        if out_dirs.iter().any(|out_dir| out_dir.starts_with(path) || path.starts_with(out_dir)) {
            return Err(eyre!(
                "C macro input directory {} overlaps this build's OUT_DIR; recursive Cargo tracking would invalidate its own generated output",
                requested.display()
            ));
        }
        watched.insert(path.to_owned());
    }
    Ok(())
}

/// Find an existing parent to watch when an optional include or compiler-search directory has
/// not yet been created.
fn existing_directory_ancestor(directory: &Path) -> eyre::Result<PathBuf> {
    directory.ancestors().find(|ancestor| ancestor.is_dir()).map(Path::to_owned).ok_or_else(|| {
        eyre!("no existing ancestor for C macro input directory {}", directory.display())
    })
}

/// Require a UTF-8 single-line path before emitting a Cargo dependency directive.
fn cargo_input_path(path: &Path) -> eyre::Result<()> {
    let value = path.to_str().ok_or_else(|| eyre!("Cargo input path is not UTF-8: {path:?}"))?;
    if value.contains(['\n', '\r']) {
        return Err(eyre!("Cargo input path contains a line break: {path:?}"));
    }
    Ok(())
}

/// Preserve a generated artifact's modification time when its final bytes match, avoiding
/// unnecessary downstream rebuilds.
fn write_content_stable(path: &Path, content: &[u8]) -> eyre::Result<()> {
    match fs::read(path) {
        Ok(existing) if existing == content => return Ok(()),
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error).wrap_err_with(|| format!("could not read {}", path.display()));
        }
    }
    fs::write(path, content).wrap_err_with(|| format!("could not write {}", path.display()))
}

fn add_blocklists(
    bind: bindgen::Builder,
    major_version: u16,
    enable_cshim: bool,
) -> bindgen::Builder {
    let bind = if major_version >= 19 {
        // Postgres 19 turned these into `static inline` functions, so without the cshim there's
        // no symbol to link against.  We implement them ourselves, in Rust, in `port.rs`
        bind.blocklist_function("TransactionId(Precedes|PrecedesOrEquals|Follows|FollowsOrEquals)")
    } else {
        bind
    };
    let bind = if major_version < 16 || !enable_cshim {
        // Before Postgres 16 these are macros. Without cshim, Postgres 16+ static inline
        // functions have no symbol to link against. Use the Rust fallback in both cases.
        bind.blocklist_function("BufferGetBlock").blocklist_function("BufferGetPage")
    } else {
        bind
    };
    bind.blocklist_type("Datum") // manually wrapping datum for correctness
        .blocklist_type("Oid") // "Oid" is not just any u32
        .blocklist_type("TransactionId") // "TransactionId" is not just any u32
        .blocklist_type("MultiXactId") // it's an alias of "TransactionId"
        .blocklist_var("CONFIGURE_ARGS") // configuration during build is hopefully irrelevant
        .blocklist_var("_*(?:HAVE|have)_.*") // header tracking metadata
        .blocklist_var("_[A-Z_]+_H") // more header metadata
        // It's used by explict `extern "C-unwind"`
        .blocklist_function("pg_re_throw")
        .blocklist_function("err(start|code|msg|detail|context_msg|hint|finish)")
        // These functions are already ported in Rust
        .blocklist_function("heap_getattr")
        .blocklist_function("BufferIsLocal")
        .blocklist_function("GetMemoryChunkContext")
        .blocklist_function("GETSTRUCT")
        .blocklist_function("MAXALIGN")
        .blocklist_function("MemoryContextIsValid")
        .blocklist_function("MemoryContextSwitchTo")
        .blocklist_function("TYPEALIGN")
        .blocklist_function("TransactionIdIsNormal")
        .blocklist_function("expression_tree_walker")
        .blocklist_function("get_pg_major_minor_version_string")
        .blocklist_function("get_pg_major_version_num")
        .blocklist_function("get_pg_major_version_string")
        .blocklist_function("get_pg_version_string")
        .blocklist_function("heap_tuple_get_struct")
        .blocklist_function("planstate_tree_walker")
        .blocklist_function("query_or_expression_tree_walker")
        .blocklist_function("query_tree_walker")
        .blocklist_function("range_table_entry_walker")
        .blocklist_function("range_table_walker")
        .blocklist_function("raw_expression_tree_walker")
        .blocklist_function("type_is_array")
        .blocklist_function("varsize_any")
        // we define these ourselves b/c Postgres is schizophrenic about them across versions
        .blocklist_function("PageValidateSpecialPointer")
        .blocklist_function("PageIsValid")
        // it's defined twice on Windows, so use PGERROR instead
        .blocklist_item("ERROR")
        // Keep these inline helpers blocklisted for compatibility with Windows linking.
        .blocklist_function("IsQueryIdEnabled")
        .blocklist_function("am_tablesync_worker")
        .blocklist_function("am_sequencesync_worker")
        .blocklist_function("am_leader_apply_worker")
        .blocklist_function("am_parallel_apply_worker")
        .blocklist_function("get_logical_worker_type")
}

fn add_allowlists<'a>(
    mut bind: bindgen::Builder,
    pg_target_includes: impl Iterator<Item = &'a str>,
) -> bindgen::Builder {
    for pg_target_include in pg_target_includes {
        bind = bind.allowlist_file(format!("{}.*", regex::escape(pg_target_include)))
    }
    bind.allowlist_item("PGERROR").allowlist_item("SIG.*")
}

fn add_derives(bind: bindgen::Builder) -> bindgen::Builder {
    bind.derive_debug(true)
        .derive_copy(true)
        .derive_default(true)
        .derive_eq(false)
        .derive_partialeq(false)
        .derive_hash(false)
        .derive_ord(false)
        .derive_partialord(false)
}

fn target_env_tracked(s: &str) -> Option<String> {
    let target = env_tracked("TARGET").unwrap();
    env_tracked(&format!("{s}_{target}")).or_else(|| env_tracked(s))
}

fn find_include(
    pg_version: u16,
    var: &str,
    default: impl Fn() -> eyre::Result<PathBuf>,
) -> eyre::Result<String> {
    let value =
        target_env_tracked(&format!("{var}_PG{pg_version}")).or_else(|| target_env_tracked(var));
    let path = match value {
        // No configured value: ask `pg_config`.
        None => default()?,
        // Configured to non-empty string: pass to bindgen
        Some(overridden) => Path::new(&overridden).to_path_buf(),
    };
    let path = std::fs::canonicalize(&path)
        .wrap_err(format!("cannot find {path:?} for C header files"))?
        .join("") // returning a `/`-ending path
        .display()
        .to_string();
    if let Some(path) = path.strip_prefix("\\\\?\\") { Ok(path.to_string()) } else { Ok(path) }
}

fn pg_target_includes(pg_version: u16, pg_config: &PgConfig) -> eyre::Result<Vec<String>> {
    let mut result =
        vec![find_include(pg_version, "PGRX_INCLUDEDIR_SERVER", || pg_config.includedir_server())?];
    if let Some("msvc") = env_tracked("CARGO_CFG_TARGET_ENV").as_deref() {
        result.push(find_include(pg_version, "PGRX_PKGINCLUDEDIR", || pg_config.pkgincludedir())?);
        result.push(find_include(pg_version, "PGRX_INCLUDEDIR_SERVER_PORT_WIN32", || {
            pg_config.includedir_server_port_win32()
        })?);
        result.push(find_include(pg_version, "PGRX_INCLUDEDIR_SERVER_PORT_WIN32_MSVC", || {
            pg_config.includedir_server_port_win32_msvc()
        })?);
    }
    Ok(result)
}

/// Compile PostgreSQL shims against the selected installation and its recorded C
/// flags, preserving the same arithmetic and ABI profile used by macro inspection.
fn build_shim(
    shim_src: &path::Path,
    shim_dst: &path::Path,
    pg_config: &PgConfig,
) -> eyre::Result<()> {
    let major_version = pg_config.major_version()?;
    let generated_wrapper = format!("\"{}.c\"", cshim_static_wrapper_name(major_version));

    std::fs::copy(shim_src, shim_dst).unwrap();

    let mut build = cc::Build::new();
    // pgrx-cshim.c includes the generated bindgen wrapper through this macro so
    // each cshim build picks the wrapper that matches its postgres headers
    build.define("PGRX_CSHIM_STATIC", Some(generated_wrapper.as_str()));
    let compiler = build.get_compiler();
    if compiler.is_like_gnu() || compiler.is_like_clang() {
        build.flag("-ffunction-sections");
        build.flag("-fdata-sections");
    }
    if compiler.is_like_msvc() {
        build.flag("/Gy");
        build.flag("/Gw");
    }
    if env_tracked("CARGO_CFG_TARGET_OS").as_deref() != Some("windows")
        && (compiler.is_like_gnu() || compiler.is_like_clang())
    {
        for flag in postgres_cflags(pg_config)? {
            build.flag(flag);
        }
    }
    for flag in extra_bindgen_clang_args(pg_config)? {
        build.flag(&flag);
    }
    for pg_target_include in pg_target_includes(major_version, pg_config)?.iter() {
        build.flag(format!("-I{pg_target_include}"));
    }
    build.file(shim_dst);
    build.compile("pgrx-cshim");
    Ok(())
}

/// Decode the installation's recorded compiler flags without losing shell quoting;
/// reject malformed or non-UTF-8 profiles before bindgen and native macros can diverge.
fn postgres_cflags(pg_config: &PgConfig) -> eyre::Result<Vec<String>> {
    let flags = pg_config.cflags()?;
    let flags = flags.to_str().ok_or_else(|| eyre!("PostgreSQL CFLAGS are not UTF-8"))?;
    shlex::split(flags).ok_or_else(|| eyre!("invalid PostgreSQL CFLAGS quoting"))
}

fn extra_bindgen_clang_args(pg_config: &PgConfig) -> eyre::Result<Vec<String>> {
    let mut out = vec![];
    let flags = shlex::split(&pg_config.cppflags()?.to_string_lossy()).unwrap_or_default();
    if env_tracked("CARGO_CFG_TARGET_OS").as_deref() != Some("windows") {
        // Just give clang the full flag set, since presumably that's what we're
        // getting when we build the C shim anyway.
        // Skip it on Windows, since clang is used to generate cshim but MSVC is
        // used to compile PostgreSQL.
        out.extend(flags.iter().cloned());
    }
    if env_tracked("CARGO_CFG_TARGET_OS").as_deref() == Some("macos") {
        // Find the `-isysroot` flags so we can warn about them, so something
        // reasonable shows up if/when the build fails.
        //
        // TODO(thom): Could probably fix some brew/xcode issues here in the
        // Find the `-isysroot` flags so we can warn about them, so something
        // reasonable shows up if/when the build fails.
        //
        // - Handle homebrew packages initially linked against as keg-only, but
        //   which have had their version bumped.
        for pair in flags.windows(2) {
            if pair[0] == "-isysroot" {
                if !std::path::Path::new(&pair[1]).exists() {
                    // The SDK path doesn't exist. Emit a warning, which they'll
                    // see if the build ends up failing (it may not fail in all
                    // cases, so we don't panic here).
                    //
                    // There's a bunch of smarter things we can try here, but
                    // most of them either break things that currently work, or
                    // are very difficult to get right. If you try to fix this,
                    // be sure to consider cases like:
                    //
                    // - User may have CommandLineTools and not Xcode, vice
                    //   versa, or both installed.
                    // - User may using a newer SDK than their OS, or vice
                    //   versa.
                    // - User may be using a newer SDK than their XCode (updated
                    //   Command line tools, not OS), or vice versa.
                    // - And so on.
                    //
                    // These are all actually fairly common. Note that the code
                    // as-is is *not* broken in these cases (except on OS/SDK
                    // updates), so care should be taken to avoid changing that
                    // if possible.
                    //
                    // The logic we'd like ideally is for `cargo pgrx init` to
                    // choose a good SDK in the first place, and force postgres
                    // to use it. Then, the logic in this build script would
                    // Just Work without changes (since we are using its
                    // sysroot verbatim).
                    //
                    // The value of "Good" here is tricky, but the logic should
                    // probably:
                    //
                    // - prefer SDKs from the CLI tools to ones from XCode
                    //   (since they're guaranteed compatible with the user's OS
                    //   version)
                    //
                    // - prefer SDKs that specify only the major SDK version
                    //   (e.g. MacOSX12.sdk and not MacOSX12.4.sdk or
                    //   MacOSX.sdk), to avoid breaking too frequently (if we
                    //   have a minor version) or being totally unable to detect
                    //   what version of the SDK was used to build postgres (if
                    //   we have neither).
                    //
                    // - Avoid choosing an SDK newer than the user's OS version,
                    //   since postgres fails to detect that they are missing if
                    //   you do.
                    //
                    // This is surprisingly hard to implement, as the
                    // information is scattered across a dozen ini files.
                    // Presumably Apple assumes you'll use
                    // `MACOSX_DEPLOYMENT_TARGET`, rather than basing it off the
                    // SDK version, but it's not an option for postgres.
                    let major_version = pg_config.major_version()?;
                    println!(
                        "cargo:warning=postgres v{major_version} was compiled against an \
                         SDK Root which does not seem to exist on this machine ({}). You may \
                         need to re-run `cargo pgrx init` and/or update your command line tools.",
                        pair[1],
                    );
                };
                // Either way, we stop here.
                break;
            }
        }
    }
    Ok(out)
}

fn run_command(mut command: &mut Command, version: &str) -> eyre::Result<Output> {
    let mut dbg = String::new();

    command = command
        .env_remove("DEBUG")
        .env_remove("MAKEFLAGS")
        .env_remove("MAKELEVEL")
        .env_remove("MFLAGS")
        .env_remove("DYLD_FALLBACK_LIBRARY_PATH")
        .env_remove("OPT_LEVEL")
        .env_remove("PROFILE")
        .env_remove("OUT_DIR")
        .env_remove("NUM_JOBS");

    eprintln!("[{version}] {command:?}");
    dbg.push_str(&format!("[{version}] -------- {command:?} -------- \n"));

    let output = command.output()?;
    let rc = output.clone();

    if !output.stdout.is_empty() {
        for line in String::from_utf8(output.stdout).unwrap().lines() {
            if line.starts_with("cargo:") {
                dbg.push_str(&format!("{line}\n"));
            } else {
                dbg.push_str(&format!("[{version}] [stdout] {line}\n"));
            }
        }
    }

    if !output.stderr.is_empty() {
        for line in String::from_utf8(output.stderr).unwrap().lines() {
            dbg.push_str(&format!("[{version}] [stderr] {line}\n"));
        }
    }
    dbg.push_str(&format!("[{version}] /----------------------------------------\n"));

    eprintln!("{dbg}");
    Ok(rc)
}

fn apply_pg_guard(items: &Vec<syn::Item>) -> eyre::Result<proc_macro2::TokenStream> {
    let mut out = proc_macro2::TokenStream::new();
    for item in items {
        match item {
            Item::ForeignMod(block) => {
                out.extend(quote! {
                    #[pgrx_macros::pg_guard]
                    #block
                });
            }
            _ => {
                out.extend(item.into_token_stream());
            }
        }
    }

    Ok(out)
}

/// Normalize callback and foreign ABI declarations to the same C-unwind storage used by guarded
/// pgrx bindings.
fn rewrite_c_abi_to_c_unwind(file: &mut syn::File) {
    use proc_macro2::Span;
    use syn::LitStr;
    use syn::visit_mut::VisitMut;
    /// Normalize nested ABI syntax before catalog collection and final pg_guard rewriting share
    /// callback storage facts.
    pub struct Visitor {}
    /// Traverse the parsed syntax so nested declarations participate in the same ABI or storage
    /// checks.
    impl VisitMut for Visitor {
        /// Rewrite explicit C ABIs throughout the parsed bindings while leaving other calling
        /// conventions intact.
        fn visit_abi_mut(&mut self, abi: &mut syn::Abi) {
            if let Some(name) = &mut abi.name
                && name.value() == "C"
            {
                *name = LitStr::new("C-unwind", Span::call_site());
            }
        }
    }
    Visitor {}.visit_file_mut(file);
}

/// Format generated binding source under the requested edition and report formatter
/// diagnostics through Cargo; macro leaves use their stricter staged writer instead.
fn rust_fmt(path: &Path, edition: &str) -> eyre::Result<()> {
    // We shouldn't hit this path in a case where we care about it, but... just
    // in case we probably should respect RUSTFMT.
    let rustfmt = env_tracked("RUSTFMT").unwrap_or_else(|| "rustfmt".into());
    let mut command = Command::new(rustfmt);
    command.arg(path).args(["--edition", edition]).current_dir(".");

    let out = run_command(&mut command, "[bindings_diff]");
    match out {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => {
            let rustfmt_output = format!(
                r#"Problems running rustfmt: {command:?}:
                {}
                {}"#,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );

            for line in rustfmt_output.lines() {
                println!("cargo:warning={line}");
            }

            // we won't fail the build because rustfmt failed
            Ok(())
        }
        Err(e)
            if e.downcast_ref::<std::io::Error>()
                .ok_or_else(|| eyre!("Couldn't downcast error ref"))?
                .kind()
                == std::io::ErrorKind::NotFound =>
        {
            Err(e).wrap_err("Failed to run `rustfmt`, is it installed?")
        }
        Err(e) => Err(e),
    }
}

/// Check catalog, publication, and invalidation invariants directly against the private
/// binding-build implementation.
#[cfg(test)]
mod macro_build_tests {
    //! Check catalog, publication, and invalidation invariants directly against the private
    //! binding-build implementation.
    //!
    //! The tests inspect real parsed output, generated leaf maps, and compiler dependency
    //! directives. Temporary input trees make success and rejection conditions explicit without
    //! relying on a configured server.

    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    /// Allocate process-local unique directory suffixes for concurrent isolated oracle runs.
    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    /// Prove multiline compiler diagnostics remain one warning rather than
    /// injecting Cargo directives, without changing the original report text.
    #[test]
    fn macro_skip_warnings_keep_compiler_diagnostics_within_one_directive() {
        let message = "rejected C witness\r\ncargo:rustc-cfg=forged\nsee source:12";
        let warning = macro_skip_warning(18, "REJECTED", message);
        assert_eq!(warning.lines().count(), 1);
        assert!(!warning.contains('\r'));
        assert_eq!(
            warning,
            "pg18: skipping macro `REJECTED`: rejected C witness  cargo:rustc-cfg=forged see source:12"
        );
    }

    /// Checks that documentation snapshots guard native support and preserve macro templates.
    #[test]
    fn documentation_snapshots_guard_native_support_and_preserve_macro_templates() {
        let source = r#"#[cfg(not(target_pointer_width = "64"))]
compile_error!("C macro target does not match the generation installation");
pub const PROFILE: usize = 64;
#[doc(hidden)]
#[allow(non_snake_case, non_camel_case_types)]
pub mod __pgrx_c_generated {
    const _: () = assert!(::core::mem::size_of::<usize>() == 8);
    pub struct Field;
}
pub mod documentation_helpers {}
/// C macro EXAMPLE from example.h:1
///
/// ```text
/// #define EXAMPLE(x) NESTED(x)
/// ```
#[macro_export]
macro_rules! EXAMPLE {
    ($argument:expr) => {
        /* PGRX: NESTED remains expanded because its binding is unavailable. */
        $crate::__pgrx_c_generated::Field
    };
}
"#;
        let snapshot = macro_snapshot(source).unwrap();
        assert!(snapshot.contains("This code is generated for documentation purposes"));
        assert!(
            snapshot.contains(
                "/* PGRX: NESTED remains expanded because its binding is unavailable. */"
            )
        );
        let mut expected = syn::parse_file(source).unwrap();
        let documentation_guard: syn::Attribute = syn::parse_quote!(#[cfg(not(docsrs))]);
        let Item::Macro(target_guard) = &mut expected.items[0] else {
            panic!("fixture target guard must be a macro item");
        };
        target_guard.attrs.push(documentation_guard.clone());
        let Item::Mod(native_support) = &mut expected.items[2] else {
            panic!("fixture native support must be a module item");
        };
        native_support.attrs.push(documentation_guard);
        assert_eq!(syn::parse_file(&snapshot).unwrap(), expected);
    }

    /// Checks that documentation snapshots require guarded items on their own lines.
    #[test]
    fn documentation_snapshots_require_guarded_items_on_their_own_lines() {
        for (source, description) in [
            ("const PROFILE: () = (); compile_error!(\"wrong target\");", "C macro target guard"),
            (
                "const PROFILE: () = (); pub mod __pgrx_c_generated {}",
                "C macro native support module",
            ),
        ] {
            let error = macro_snapshot(source).unwrap_err().to_string();
            assert_eq!(error, format!("{description} does not start on its own line"));
        }
    }

    /// Checks that environment arguments match bindgen priority and shell quoting.
    #[test]
    fn environment_arguments_match_bindgen_priority_and_shell_quoting() {
        let mut values = BTreeMap::from([
            ("TARGET", "x86_64-unknown-linux-gnu"),
            ("BINDGEN_EXTRA_CLANG_ARGS", "-DGLOBAL=1"),
            ("BINDGEN_EXTRA_CLANG_ARGS_x86_64_unknown_linux_gnu", "-DNORMALIZED=1"),
            (
                "BINDGEN_EXTRA_CLANG_ARGS_x86_64-unknown-linux-gnu",
                "-I'/headers with spaces' -DVALUE=2",
            ),
        ]);
        let resolve = |values: &BTreeMap<&str, &str>| {
            bindgen_environment_arguments(|name| values.get(name).map(|value| (*value).to_owned()))
        };
        assert_eq!(resolve(&values), ["-I/headers with spaces", "-DVALUE=2"]);
        values.remove("BINDGEN_EXTRA_CLANG_ARGS_x86_64-unknown-linux-gnu");
        assert_eq!(resolve(&values), ["-DNORMALIZED=1"]);
        values.remove("BINDGEN_EXTRA_CLANG_ARGS_x86_64_unknown_linux_gnu");
        assert_eq!(resolve(&values), ["-DGLOBAL=1"]);
        values.insert("BINDGEN_EXTRA_CLANG_ARGS_x86_64-unknown-linux-gnu", "");
        assert!(resolve(&values).is_empty(), "an empty override still takes precedence");
        values.insert("BINDGEN_EXTRA_CLANG_ARGS_x86_64-unknown-linux-gnu", "-I'unterminated");
        assert_eq!(
            resolve(&values),
            ["-I'unterminated"],
            "bindgen preserves malformed quoting as one argument"
        );
    }

    /// Checks that explicit targets and cargo target spellings are preserved.
    #[test]
    fn explicit_targets_and_cargo_target_spellings_are_preserved() {
        assert_eq!(clang_target("aarch64-apple-darwin"), "arm64-apple-darwin");
        assert_eq!(clang_target("aarch64-apple-ios-sim"), "arm64-apple-ios-simulator");
        assert_eq!(clang_target("riscv64gc-unknown-linux-gnu"), "riscv64-unknown-linux-gnu");
        assert_eq!(clang_target("xtensa-esp32-espidf"), "xtensa-esp32-elf");
        for arguments in
            [vec!["--target=x86_64-unknown-linux-gnu"], vec!["-target", "arm64-apple-darwin"]]
        {
            let arguments = arguments.into_iter().map(String::from).collect::<Vec<_>>();
            assert!(has_explicit_clang_target(arguments.iter()));
        }
        let missing = ["-target".to_owned()];
        assert!(!has_explicit_clang_target(missing.iter()));
    }

    /// Checks that inspected resource options and environment tail are used exactly once.
    #[test]
    fn inspected_resource_options_and_environment_tail_are_used_exactly_once() {
        let base = ["-fwrapv", "-I/server", "--target=arm64-apple-darwin"].map(String::from);
        let environment = ["-include", "/override/header.h", "-DVALUE=9"].map(String::from);
        let inspected = base
            .iter()
            .chain(&environment)
            .cloned()
            .chain(std::iter::once("-resource-dir=/clang/resource".to_owned()))
            .collect::<Vec<_>>();
        let (binder, effective) =
            normalize_bindgen_arguments(&base, &environment, &inspected).unwrap();
        assert_eq!(
            binder,
            [
                "-fwrapv",
                "-I/server",
                "--target=arm64-apple-darwin",
                "-resource-dir=/clang/resource"
            ]
        );
        assert_eq!(effective, binder.iter().chain(&environment).cloned().collect::<Vec<_>>());
        assert_eq!(effective.iter().filter(|arg| arg.as_str() == "-include").count(), 1);
        let (same_binder, same_effective) =
            normalize_bindgen_arguments(&binder, &environment, &effective).unwrap();
        assert_eq!(same_binder, binder);
        assert_eq!(same_effective, effective, "an already resolved profile must be stable");
        let no_environment = ["-fwrapv".to_owned(), "-resource-dir=/clang/resource".to_owned()];
        assert_eq!(
            normalize_bindgen_arguments(&no_environment, &[], &no_environment).unwrap().1,
            no_environment
        );
        assert!(normalize_bindgen_arguments(&base, &environment, &base).is_err());
    }

    /// Checks that unchanged generated artifacts keep their modification time.
    #[test]
    fn unchanged_generated_artifacts_keep_their_modification_time() {
        let directory = TemporaryDirectory::new();
        let path = directory.0.join("pg18_macros.rs");
        write_content_stable(&path, b"original").unwrap();
        let old = UNIX_EPOCH + Duration::from_secs(1);
        fs::File::options().write(true).open(&path).unwrap().set_modified(old).unwrap();
        let before = fs::metadata(&path).unwrap().modified().unwrap();
        write_content_stable(&path, b"original").unwrap();
        assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), before);
        write_content_stable(&path, b"changed").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"changed");
        assert_ne!(fs::metadata(&path).unwrap().modified().unwrap(), before);
    }

    /// Checks that macro tree updates preserve unchanged leaves and remove stale Rust only.
    #[test]
    fn macro_tree_updates_preserve_unchanged_leaves_and_remove_stale_rust_only() {
        let directory = TemporaryDirectory::new();
        let version = directory.0.join("cmacros/pg18");
        let files = MacroFiles {
            sources: BTreeMap::from([
                (PathBuf::from("mod.rs"), "mod c;\n".into()),
                (PathBuf::from("c.rs"), "pub const PROFILE: u32 = 18;\n".into()),
            ]),
        };
        write_macro_files(&files, &version, false).unwrap();
        let leaf = version.join("c.rs");
        let old = UNIX_EPOCH + Duration::from_secs(1);
        fs::File::options().write(true).open(&leaf).unwrap().set_modified(old).unwrap();
        fs::create_dir_all(version.join("obsolete")).unwrap();
        fs::write(version.join("obsolete/header.rs"), "old generated source").unwrap();
        fs::write(version.join("notes.txt"), "keep non-Rust files").unwrap();
        let other_version = directory.0.join("cmacros/pg17/mod.rs");
        fs::create_dir_all(other_version.parent().unwrap()).unwrap();
        fs::write(&other_version, "other version").unwrap();
        write_macro_files(&files, &version, false).unwrap();
        assert_eq!(fs::metadata(&leaf).unwrap().modified().unwrap(), old);
        assert!(!version.join("obsolete").exists());
        assert_eq!(fs::read_to_string(version.join("notes.txt")).unwrap(), "keep non-Rust files");
        assert_eq!(fs::read_to_string(other_version).unwrap(), "other version");
        write_macro_files(&MacroFiles::empty(), &version, false).unwrap();
        assert!(!leaf.exists(), "unavailable generation removes previous macros");
        assert!(version.join("mod.rs").exists(), "an empty module still loads");
    }

    /// Checks that macro trees are formatted before stable normal and snapshot writes.
    #[test]
    fn macro_trees_are_formatted_before_stable_normal_and_snapshot_writes() {
        let directory = TemporaryDirectory::new();
        let original = "/// C macro EXAMPLE from c.h:1\n/// ```text\n/// #define EXAMPLE(x) (x)\n/// ```\npub fn example( value:u32 )->u32{ /* PGRX: preserve this comment */ value+1 }\n";
        let files = MacroFiles {
            sources: BTreeMap::from([
                (PathBuf::from("mod.rs"), "mod c;\n".into()),
                (PathBuf::from("c.rs"), original.into()),
            ]),
        };
        for documentation in [false, true] {
            let version = directory.0.join(if documentation { "snapshot" } else { "normal" });
            write_macro_files(&files, &version, documentation).unwrap();
            let leaf = fs::read_to_string(version.join("c.rs")).unwrap();
            assert!(leaf.contains("pub fn example(value: u32) -> u32 {"));
            assert!(leaf.contains("/// #define EXAMPLE(x) (x)"));
            assert!(leaf.contains("/* PGRX: preserve this comment */"));
            let expected =
                if documentation { macro_snapshot(original).unwrap() } else { original.to_owned() };
            assert_eq!(syn::parse_file(&leaf).unwrap(), syn::parse_file(&expected).unwrap());
            let old = UNIX_EPOCH + Duration::from_secs(1);
            for relative in files.sources.keys() {
                fs::File::options()
                    .write(true)
                    .open(version.join(relative))
                    .unwrap()
                    .set_modified(old)
                    .unwrap();
            }
            write_macro_files(&files, &version, documentation).unwrap();
            for relative in files.sources.keys() {
                assert_eq!(fs::metadata(version.join(relative)).unwrap().modified().unwrap(), old);
            }
        }
    }

    /// Checks that unavailable macro formatter preserves original sources.
    #[test]
    fn unavailable_macro_formatter_preserves_original_sources() {
        let directory = TemporaryDirectory::new();
        let path = directory.0.join("original.rs");
        let original = "pub const VALUE:u32=18;\n";
        fs::write(&path, original).unwrap();
        format_macro_files(std::slice::from_ref(&path), &directory.0.join("missing-rustfmt"))
            .unwrap();
        assert_eq!(fs::read_to_string(path).unwrap(), original);
    }

    /// Check that only rustup's specific missing-component error permits optional formatting.
    #[test]
    fn missing_rustfmt_component_diagnostic_excludes_formatting_errors() {
        for diagnostic in [
            "error: 'rustfmt' is not installed for the toolchain '1.96.0-x86_64-unknown-linux-gnu'\n",
            "error: 'rustfmt' is not installed for the toolchain 'nightly-aarch64-apple-darwin'.\nhelp: run `rustup component add rustfmt`\n",
        ] {
            assert!(missing_rustfmt_component(diagnostic.as_bytes()), "{diagnostic}");
        }
        for diagnostic in [
            "error: expected expression, found `;`\n",
            "error: unable to find rustfmt configuration\n",
            "error: 'clippy' is not installed for the toolchain '1.96.0-x86_64-unknown-linux-gnu'\n",
            "warning: 'rustfmt' is not installed for the toolchain '1.96.0-x86_64-unknown-linux-gnu'\n",
            "error: 'rustfmt' is not installed for the toolchain ''\n",
            "error: 'rustfmt' is not installed for the toolchain 'fixture'\nerror: unrelated failure\n",
        ] {
            assert!(!missing_rustfmt_component(diagnostic.as_bytes()), "{diagnostic}");
        }
    }

    /// Build an isolated failing formatter proxy whose stderr is independent of source content.
    #[cfg(unix)]
    fn failing_macro_formatter(directory: &Path, diagnostic: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;

        let formatter = directory.join("fixture-rustfmt");
        fs::write(&formatter, "#!/bin/sh\ncat \"$0.stderr\" >&2\nexit 1\n").unwrap();
        fs::write(directory.join("fixture-rustfmt.stderr"), diagnostic).unwrap();
        fs::set_permissions(&formatter, fs::Permissions::from_mode(0o700)).unwrap();
        formatter
    }

    /// Check that an executable rustup proxy with a missing component preserves staged sources.
    #[cfg(unix)]
    #[test]
    fn unavailable_rustfmt_component_preserves_original_sources() {
        let directory = TemporaryDirectory::new();
        let path = directory.0.join("original.rs");
        let original = "pub const VALUE:u32=18;\n";
        fs::write(&path, original).unwrap();
        let formatter = failing_macro_formatter(
            &directory.0,
            "error: 'rustfmt' is not installed for the toolchain '1.96.0-x86_64-unknown-linux-gnu'\nhelp: run `rustup component add rustfmt`\n",
        );
        format_macro_files(std::slice::from_ref(&path), &formatter).unwrap();
        assert_eq!(fs::read_to_string(path).unwrap(), original);
    }

    /// Check that real formatter failures remain fatal rather than publishing unformatted sources.
    #[cfg(unix)]
    #[test]
    fn macro_formatter_failure_propagates_its_diagnostic() {
        let directory = TemporaryDirectory::new();
        let path = directory.0.join("original.rs");
        let original = "pub const VALUE:u32=18;\n";
        fs::write(&path, original).unwrap();
        let diagnostic = "error: expected expression, found `;`\n";
        let formatter = failing_macro_formatter(&directory.0, diagnostic);
        let error =
            format_macro_files(std::slice::from_ref(&path), &formatter).unwrap_err().to_string();
        assert!(error.contains("could not format generated C macros"), "{error}");
        assert!(error.contains(diagnostic.trim()), "{error}");
        assert_eq!(fs::read_to_string(path).unwrap(), original);
    }

    /// Checks that macro formatting directories are removed after use.
    #[test]
    fn macro_formatting_directories_are_removed_after_use() {
        let directory = TemporaryDirectory::new();
        let final_directory = directory.0.join("pg18");
        let staging = MacroFormattingDirectory::new(&final_directory).unwrap();
        let staging_path = staging.0.clone();
        assert_eq!(staging_path.parent(), final_directory.parent());
        fs::write(staging_path.join("fragment.rs"), "pub const VALUE: u32 = 18;\n").unwrap();
        drop(staging);
        assert!(!staging_path.exists());
        assert!(directory.0.exists(), "cleanup only removes its own directory");
    }

    /// Checks that macro snapshots guard each leaf and do not duplicate native guards.
    #[test]
    fn macro_snapshots_guard_each_leaf_and_do_not_duplicate_native_guards() {
        let directory = TemporaryDirectory::new();
        let files = MacroFiles {
            sources: BTreeMap::from([
                (PathBuf::from("mod.rs"), "mod c;\n".into()),
                (
                    PathBuf::from("c.rs"),
                    "#[cfg(not(target_pointer_width = \"64\"))]\ncompile_error!(\"wrong target\");\n/// C macro EXAMPLE from c.h:1\n/// ```text\n/// #define EXAMPLE(x) (x)\n/// ```\n#[macro_export]\nmacro_rules! EXAMPLE { ($x:expr) => { $x }; }\n".into(),
                ),
            ]),
        };
        write_macro_files(&files, &directory.0, true).unwrap();
        let leaf = fs::read_to_string(directory.0.join("c.rs")).unwrap();
        assert!(leaf.contains("#[cfg(not(docsrs))]"));
        assert!(leaf.contains("/// #define EXAMPLE(x) (x)"));
        assert!(!leaf.contains("#[doc ="));
        let guarded = "#[cfg(not(docsrs))]\n#[doc(hidden)]\npub mod __pgrx_c_generated {}\n";
        let snapshot = macro_snapshot(guarded).unwrap();
        assert_eq!(snapshot.matches("#[cfg(not(docsrs))]").count(), 1);
    }

    /// Checks that absent files watch existing parents without perpetually dirty file
    /// directives.
    #[test]
    fn absent_files_watch_existing_parents_without_perpetually_dirty_file_directives() {
        let directory = TemporaryDirectory::new();
        let out_dir = directory.0.join("target/debug/build/pg-sys/out");
        fs::create_dir_all(&out_dir).unwrap();
        let isolated_home = directory.0.join("isolated-pgrx-home");
        fs::create_dir(&isolated_home).unwrap();
        let present = isolated_home.join("pg_config");
        fs::write(&present, b"recorded config").unwrap();
        let missing = isolated_home.join("nested/config.toml");
        let inputs = BuildInputs {
            files: vec![present.clone(), missing.clone()],
            fingerprints: BTreeMap::from([
                (present.clone(), Some("recorded content identity".to_owned())),
                (missing.clone(), None),
            ]),
            ..BuildInputs::default()
        };
        let paths = macro_rerun_paths(&inputs, &out_dir).unwrap();
        assert!(paths.contains(&present));
        assert!(!paths.contains(&missing), "Cargo must not receive a missing file directive");
        assert!(paths.contains(&isolated_home));
        assert!(paths.contains(&isolated_home.canonicalize().unwrap()));
        let nested = isolated_home.join("nested");
        fs::create_dir(&nested).unwrap();
        let paths = macro_rerun_paths(&inputs, &out_dir).unwrap();
        assert!(paths.contains(&nested), "newly available parents narrow the tracked root");
        assert!(!paths.contains(&missing));
        let unsafe_missing = out_dir.join("missing.toml");
        let unsafe_inputs = BuildInputs {
            files: vec![unsafe_missing.clone()],
            fingerprints: BTreeMap::from([(unsafe_missing, None)]),
            ..BuildInputs::default()
        };
        assert!(macro_rerun_paths(&unsafe_inputs, &out_dir).is_err());
    }

    /// Checks that missing search directories watch creation without tracking cargo outputs.
    #[test]
    fn missing_search_directories_watch_creation_without_tracking_cargo_outputs() {
        let directory = TemporaryDirectory::new();
        let out_dir = directory.0.join("target/debug/build/pg-sys/out");
        fs::create_dir_all(&out_dir).unwrap();
        let headers = directory.0.join("headers");
        fs::create_dir(&headers).unwrap();
        let external_bin = directory.0.join("toolchain/bin");
        fs::create_dir_all(&external_bin).unwrap();
        let custom_bin = directory.0.join("target/debug/custom-clang/bin");
        fs::create_dir_all(&custom_bin).unwrap();
        let inputs = BuildInputs {
            directories: vec![headers.join("optional/missing")],
            executable_search_directories: vec![external_bin.clone(), custom_bin.clone()],
            ..BuildInputs::default()
        };
        let watched = macro_watch_directories(&inputs, &out_dir).unwrap();
        assert!(watched.contains(&headers));
        assert!(watched.contains(&headers.canonicalize().unwrap()));
        assert!(watched.contains(&external_bin));
        assert!(watched.contains(&custom_bin), "custom Cargo-target descendants are still inputs");
        let unsafe_inputs =
            BuildInputs { directories: vec![out_dir.clone()], ..BuildInputs::default() };
        let error = macro_watch_directories(&unsafe_inputs, &out_dir).unwrap_err().to_string();
        assert!(error.contains("overlaps this build's OUT_DIR"));
        let unsafe_search = BuildInputs {
            executable_search_directories: vec![directory.0.join("missing-toolchain/bin")],
            ..BuildInputs::default()
        };
        let error = macro_watch_directories(&unsafe_search, &out_dir).unwrap_err().to_string();
        assert!(error.contains("set CLANG_PATH to an absolute compiler path"));
        let cargo_lookup = BuildInputs {
            executable_search_directories: vec![directory.0.join("target/debug")],
            ..BuildInputs::default()
        };
        let error = macro_watch_directories(&cargo_lookup, &out_dir).unwrap_err().to_string();
        assert!(error.contains("set CLANG_PATH to an absolute compiler path"));
    }

    /// Checks that directory aliases remain watched and cannot hide output overlap.
    #[cfg(unix)]
    #[test]
    fn directory_aliases_remain_watched_and_cannot_hide_output_overlap() {
        let directory = TemporaryDirectory::new();
        let out_dir = directory.0.join("target/debug/build/pg-sys/out");
        fs::create_dir_all(&out_dir).unwrap();
        let headers = directory.0.join("headers");
        fs::create_dir(&headers).unwrap();
        let alias = directory.0.join("header-alias");
        std::os::unix::fs::symlink(&headers, &alias).unwrap();
        let inputs = BuildInputs { directories: vec![alias.clone()], ..BuildInputs::default() };
        let watched = macro_watch_directories(&inputs, &out_dir).unwrap();
        assert!(watched.contains(&alias), "retargeting the alias must be observable");
        assert!(watched.contains(&headers.canonicalize().unwrap()));
        fs::remove_file(&alias).unwrap();
        std::os::unix::fs::symlink(&out_dir, &alias).unwrap();
        assert!(macro_watch_directories(&inputs, &out_dir).is_err());
    }

    /// Checks that unavailable target metadata retains availability queries and an explicit
    /// report without inventing an inspected compiler profile or publishing C definitions.
    #[test]
    fn unavailable_target_metadata_retains_availability_and_explicit_report() {
        let output = MacroOutput::unavailable(18, "no matching macro target metadata").unwrap();
        assert_eq!(output.files.sources.len(), 1);
        assert_eq!(output.files.sources, MacroFiles::empty().sources);
        assert!(!output.inspected);
        assert_eq!(output.emitted, 0);
        let report: serde_json::Value = serde_json::from_slice(&output.report).unwrap();
        assert_eq!(report["postgres_major_version"], 18);
        assert_eq!(report["status"], "unavailable");
        assert_eq!(report["reason"], "no matching macro target metadata");
        assert!(report.get("profile").is_none(), "host semantics must not be invented");
    }

    /// Own isolated compiler inputs and outputs so oracle runs cannot reuse stale artifacts or
    /// leave a growing target tree.
    struct TemporaryDirectory(
        /// Owned fixture path used for isolated inputs and cleanup.
        PathBuf,
    );

    /// Allocate isolated compiler artifacts with process-local uniqueness and deterministic
    /// cleanup ownership.
    impl TemporaryDirectory {
        /// Create an isolated header and output directory owned by this build-publication test.
        fn new() -> Self {
            let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            let number = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("pgrx-macro-build-{}-{nonce}-{number}", std::process::id()));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    /// Release only temporary artifacts owned by this fixture, including on failed compiler or
    /// assertion paths.
    impl Drop for TemporaryDirectory {
        /// Remove only the temporary files owned by this test fixture after its assertions.
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
}
