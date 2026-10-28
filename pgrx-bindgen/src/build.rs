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
    IntegerConstant, MacroEmission, MacroScanner, PostgresConfig, PostgresError,
    SystemIncludeDirectory, SystemIncludes, generate_with_bindings, lower_msvc_runtime_flags,
    pg_sys_integer_bridges, postgres_function_macro_names, postgres_inline_function_names,
    postgres_object_macro_names, probe_integer_object_constants,
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
use syn::{Item, ItemConst};

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
/// Merge every version's macro documentation into the docs.rs snapshot.
mod macro_snapshot;
/// Compile generated native access helpers under the already verified C invocation profile.
mod macro_support;
/// Recover module and function ABI records by invoking PostgreSQL's original declaration macros.
mod metadata_support;
/// Carry matching fresh bindings, macro adapters and native code across target builds.
mod target_artifacts;
use macro_files::MacroFiles;
use macro_snapshot::SnapshotVersion;
use macro_support::{NativeBuild, compile_macro_support, link_macro_support};

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
    println!("cargo:rustc-check-cfg=cfg(pgrx_c_char_signed)");
    println!("cargo:rustc-check-cfg=cfg(pgrx_c_char_unsigned)");
    println!("cargo:rustc-check-cfg=cfg(pgrx_c_alignment)");
    println!("cargo:rustc-check-cfg=cfg(pgrx_c_int128_unavailable)");
    println!(
        "cargo:rustc-check-cfg=cfg(pgrx_c_size_type, values(\"unsigned_int\", \"unsigned_long\", \"unsigned_long_long\"))"
    );
    println!(
        "cargo:rustc-check-cfg=cfg(pgrx_c_ptrdiff_type, values(\"int\", \"long\", \"long_long\"))"
    );
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
    if is_for_release() {
        let target = env_tracked("TARGET").ok_or_else(|| eyre!("Cargo did not set TARGET"))?;
        let manifest = fs::read_to_string(build_paths.manifest_dir.join("Cargo.toml"))?;
        check_documented_target(&manifest, &target)?;
    }

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

    let releases = std::thread::scope(|scope| {
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
    if is_for_release() {
        publish_release(releases.into_iter().flatten().collect(), &build_paths.src_dir)?;
    }
    Ok(())
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

/// One version's release artifacts, held until the whole release verifies.
struct ReleaseVersion {
    /// PostgreSQL major version.
    major: u16,
    /// Source of `pgNN.rs`, including its generation id.
    bindings: String,
    /// Source of `pgNN_oids.rs`.
    oids: String,
    /// The version's documentation.
    snapshot: SnapshotVersion,
}

/// Write the selected version's bindings, OIDs, macro module tree, and skip report to
/// OUT_DIR. Release generation also returns what it publishes to `src/include`, which
/// [`publish_release`] writes only after every version's snapshot verifies.
fn generate_bindings(
    major_version: u16,
    pg_config: &PgConfig,
    build_paths: &BuildPaths,
    is_for_release: bool,
    enable_cshim: bool,
) -> eyre::Result<Option<ReleaseVersion>> {
    let mut include_h = build_paths.manifest_dir.clone();
    include_h.push("include");
    include_h.push(format!("pg{major_version}.h"));

    let active = env_tracked(&format!("CARGO_FEATURE_PG{major_version}")).is_some();
    let native = NativeBuild {
        out_dir: &build_paths.out_dir,
        active,
        cshim: (active && enable_cshim).then_some(build_paths.shim_src.as_path()),
    };
    let (bindgen_output, macros) = get_bindings(major_version, pg_config, &include_h, native)
        .wrap_err_with(|| {
            format!("generating bindings and C macros for pg{major_version} failed")
        })?;

    let oids = extract_oids(&bindgen_output);
    let rewritten_items = rewrite_items(bindgen_output, &oids)
        .wrap_err_with(|| format!("failed to rewrite items for pg{major_version}"))?;
    // The documentation snapshot asserts this id, so docs builds reject bindings and
    // a snapshot from different release generations. Equal bindings give equal ids.
    let generation = target_artifacts::sha256(rewritten_items.to_string().as_bytes());
    let bindings = quote! {
        use crate as pg_sys;
        use crate::{Datum, MultiXactId, Oid, PgNode, TransactionId};
        #rewritten_items
        /// The release generation of these bindings, checked by the documentation snapshot.
        #[doc(hidden)]
        pub const __PGRX_GENERATION: &str = #generation;
    }
    .to_string();
    let oids = format_builtin_oid_impl(oids).to_string();

    let bindings_file = build_paths.out_dir.join(format!("pg{major_version}.rs"));
    write_rs_file(&bindings, &bindings_file, is_for_release).wrap_err_with(|| {
        format!(
            "Unable to write bindings file for pg{major_version} to `{}`",
            bindings_file.display()
        )
    })?;
    let oids_file = build_paths.out_dir.join(format!("pg{major_version}_oids.rs"));
    write_rs_file(&oids, &oids_file, is_for_release).wrap_err_with(|| {
        format!("Unable to write oids file for pg{major_version} to `{}`", oids_file.display())
    })?;

    write_macro_files(
        &macros.files,
        &build_paths.out_dir.join(format!("cmacros/pg{major_version}")),
        false,
    )?;
    remove_legacy_macro_file(&build_paths.out_dir, major_version)?;
    let report = build_paths.out_dir.join(format!("pg{major_version}_macro_report.json"));
    write_content_stable(&report, &macros.report)?;
    if macro_debug_enabled() {
        // Debug formatting escapes line breaks that recorded flags may contain.
        let dropped = if macros.cflags_dropped.is_empty() {
            String::new()
        } else {
            format!("; dropped recorded CFLAGS {:?}", macros.cflags_dropped)
        };
        println!(
            "cargo:warning=pg{major_version} C macros: {} emitted, {} skipped{dropped}; report {}",
            macros.emitted,
            macros.skipped,
            report.display()
        );
    }
    if active {
        for cfg in &macros.runtime_cfg {
            // Only the active installation configures the defining runtime;
            // release generation may inspect other target profiles.
            println!("cargo:rustc-cfg={cfg}");
        }
    }

    let lib_dir = pg_config.lib_dir()?;
    println!(
        "cargo:rustc-link-search={}",
        lib_dir.to_str().ok_or_else(|| eyre!("{lib_dir:?} is not valid UTF-8 string"))?
    );
    if !is_for_release {
        return Ok(None);
    }
    // Only an imported target bundle lacks the documentation; it carries no shells.
    let mut snapshot = macros.snapshot.ok_or_else(|| {
        eyre!(
            "release generation documents the C macros of pg{major_version}, but PGRX_TARGET_INFO_PATH_PG{major_version} imports them without their documentation"
        )
    })?;
    snapshot.generated_by(generation);
    Ok(Some(ReleaseVersion { major: major_version, bindings, oids, snapshot }))
}

/// Require release generation to run on the single target docs.rs documents.
///
/// The committed bindings and the snapshot's support describe one C target profile,
/// and the snapshot's target guard rejects docs builds for any other target.
fn check_documented_target(manifest: &str, target: &str) -> eyre::Result<()> {
    let manifest = manifest.parse::<toml::Table>().wrap_err("could not parse Cargo.toml")?;
    let targets = manifest
        .get("package")
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.get("docs"))
        .and_then(|docs| docs.get("rs"))
        .and_then(|docsrs| docsrs.get("targets"))
        .and_then(toml::Value::as_array)
        .ok_or_else(|| {
            eyre!("release generation requires [package.metadata.docs.rs] targets in Cargo.toml")
        })?;
    let [documented] = targets.as_slice() else {
        return Err(eyre!(
            "[package.metadata.docs.rs] targets must name exactly one target for release generation"
        ));
    };
    let documented = documented
        .as_str()
        .ok_or_else(|| eyre!("[package.metadata.docs.rs] targets must be strings"))?;
    if documented != target {
        return Err(eyre!(
            "release bindings and the C macro snapshot describe {documented}, the docs.rs target in Cargo.toml; run release generation on that target, not {target}"
        ));
    }
    Ok(())
}

/// Publish every version's bindings and the docs.rs macro snapshot to `src_dir`.
///
/// The snapshot is merged and verified before anything is written, so a failure
/// leaves `src_dir` unchanged. Supported versions that the run does not configure are
/// left out of the snapshot, which then fails docs builds of those versions.
fn publish_release(releases: Vec<ReleaseVersion>, src_dir: &Path) -> eyre::Result<()> {
    if releases.is_empty() {
        return Err(eyre!(
            "release generation configured no supported PostgreSQL version; nothing was published"
        ));
    }
    let (snapshots, releases): (Vec<_>, Vec<_>) = releases
        .into_iter()
        .map(|ReleaseVersion { major, bindings, oids, snapshot }| {
            (snapshot, (major, bindings, oids))
        })
        .unzip();
    let supported = SUPPORTED_VERSIONS().iter().map(|version| version.major).collect::<Vec<_>>();
    let files = macro_snapshot::snapshot_files(&snapshots, &supported)?;
    let snapshot_dir = src_dir.join("cmacros/snapshot");
    write_macro_files(&files, &snapshot_dir, true)?;
    for (major, bindings, oids) in &releases {
        write_rs_file(bindings, &src_dir.join(format!("pg{major}.rs")), true)
            .wrap_err_with(|| format!("Unable to publish bindings for pg{major}"))?;
        write_rs_file(oids, &src_dir.join(format!("pg{major}_oids.rs")), true)
            .wrap_err_with(|| format!("Unable to publish oids for pg{major}"))?;
        remove_legacy_macro_file(src_dir, *major)?;
    }
    let covered = snapshots.iter().map(SnapshotVersion::major).collect::<BTreeSet<_>>();
    let missing = supported
        .iter()
        .filter(|major| !covered.contains(major))
        .map(|major| format!("pg{major}"))
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        println!(
            "cargo:warning=C macro documentation snapshot covers only {}; it omits {}, so do not commit it",
            covered.iter().map(|major| format!("pg{major}")).collect::<Vec<_>>().join(", "),
            missing.join(", ")
        );
    }
    let (mut bytes, mut count) = (0, 0);
    for entry in walkdir::WalkDir::new(&snapshot_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            bytes += entry.metadata()?.len();
            count += 1;
        }
    }
    println!("cargo:warning=C macro documentation snapshot: {bytes} bytes in {count} files");
    for snapshot in &snapshots {
        println!("cargo:warning={}", snapshot.report());
    }
    Ok(())
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
}

impl BuildPaths {
    fn from_env() -> Self {
        // Cargo guarantees these are provided, so unwrap is fine.
        let manifest_dir = env_tracked("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
        let out_dir = env_tracked("OUT_DIR").map(PathBuf::from).unwrap();
        Self {
            src_dir: manifest_dir.join("src/include"),
            shim_src: manifest_dir.join("pgrx-cshim.c"),
            out_dir,
            manifest_dir,
        }
    }
}

/// Publish the macro tree with stable writes, then remove only stale Rust leaves owned by this
/// version. The documentation snapshot is staged and formatted first because it is checked in.
fn write_macro_files(
    files: &MacroFiles,
    directory: &Path,
    documentation: bool,
) -> eyre::Result<()> {
    fs::create_dir_all(directory)?;
    let publish = |relative: &Path, content: &[u8]| -> eyre::Result<()> {
        let path = directory.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        write_content_stable(&path, content)
    };
    if documentation {
        let staging = MacroFormattingDirectory::new(directory)?;
        let mut paths = Vec::with_capacity(files.sources.len());
        for (relative, source) in &files.sources {
            let path = staging.0.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, source)?;
            paths.push(path);
        }
        let rustfmt = env_tracked("RUSTFMT").unwrap_or_else(|| "rustfmt".into());
        format_macro_files(&paths, Path::new(&rustfmt))?;
        for relative in files.sources.keys() {
            // Compare the final representation: formatting after the stable write
            // would rewrite identical artifacts on every regeneration.
            publish(relative, &fs::read(staging.0.join(relative))?)?;
        }
    } else {
        // OUT_DIR trees are compiler input that nobody edits, and the generator
        // already lays out macro bodies. Rustfmt over their tens of megabytes
        // cost seconds per build-script run and made the bytes depend on any
        // rustfmt.toml above the target directory.
        for (relative, source) in &files.sources {
            publish(relative, source.as_bytes())?;
        }
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

/// Bound escaped filenames per process, leaving headroom below Windows' 32,767 UTF-16-unit
/// command-line limit for the formatter executable, options, and terminating null.
const MACRO_FORMAT_PATH_BUDGET: usize = 16 * 1024;

/// Borrow the largest prefix within the formatter's filename budget without allocating commands.
/// Double the encoded byte length for Windows quoting/backslash escaping; it also bounds UTF-16
/// length. Always admit one path so an individually oversized filename yields an OS error.
fn macro_format_batch_len(paths: &[PathBuf]) -> usize {
    let mut size = 0usize;
    for (index, path) in paths.iter().enumerate() {
        let cost = path.as_os_str().as_encoded_bytes().len().saturating_mul(2).saturating_add(3);
        if index > 0 && size.saturating_add(cost) > MACRO_FORMAT_PATH_BUDGET {
            return index;
        }
        size = size.saturating_add(cost);
    }
    paths.len()
}

/// Format generated leaves in bounded batches without child traversal; tolerate an absent
/// executable or rustup component, but propagate actual formatter failures before publication.
fn format_macro_files(mut paths: &[PathBuf], rustfmt: &Path) -> eyre::Result<()> {
    while !paths.is_empty() {
        let (batch, remaining) = paths.split_at(macro_format_batch_len(paths));
        let mut command = Command::new(rustfmt);
        command.args(batch).args(["--edition", "2024", "--config", "skip_children=true"]);
        match run_command(&mut command, "C macro formatting") {
            Ok(output) if output.status.success() => {}
            Ok(output) if missing_rustfmt_component(&output.stderr) => {
                // The proxy failed before formatting this batch. Original staged
                // sources remain suitable for optional unformatted output.
                return Ok(());
            }
            Ok(output) => {
                return Err(eyre!(
                    "could not format generated C macros: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            Err(error)
                if error
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|error| error.kind() == std::io::ErrorKind::NotFound) =>
            {
                // Rustfmt is optional; untouched staged sources remain usable.
                return Ok(());
            }
            Err(error) => return Err(error).wrap_err("could not start the C macro formatter"),
        }
        paths = remaining;
    }
    Ok(())
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
fn write_rs_file(contents: &str, file_path: &Path, is_for_release: bool) -> eyre::Result<()> {
    use std::io::Write;
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
    rust_fmt(file_path, "2024")
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

/// Generate from original C inputs or import their complete verified target bundle.
/// Raw bindings alone cannot implement the migrated macro consumers.
fn get_bindings(
    major_version: u16,
    pg_config: &PgConfig,
    include_h: &path::Path,
    native: NativeBuild<'_>,
) -> eyre::Result<(syn::File, MacroOutput)> {
    let (bindings, macros) = if let Some(info_dir) =
        target_env_tracked(&format!("PGRX_TARGET_INFO_PATH_PG{major_version}"))
    {
        target_artifacts::import(
            Path::new(&info_dir),
            target_artifact_domain(major_version)?,
            native.out_dir,
            native.active,
        )?
    } else {
        let target = env_tracked("TARGET").ok_or_else(|| eyre!("Cargo did not set TARGET"))?;
        let metadata = pg_config
            .path()
            .map_or_else(|| "PGRX_PG_CONFIG_AS_ENV".to_owned(), |path| path.display().to_string());
        let pg_target_includes =
            pg_target_includes(major_version, pg_config).wrap_err_with(|| {
                format!(
                    "pg{major_version}: could not select PostgreSQL server headers for {target} (PostgreSQL metadata {metadata})"
                )
            })?;
        eprintln!("pg_target_includes = {pg_target_includes:?}");
        let (bindings, macros) =
            run_bindgen(major_version, pg_config, include_h, &target, &pg_target_includes, native)
                .wrap_err_with(|| {
                    format!(
                        "pg{major_version}: pgrx-pg-sys requires generated C macros for {target} (server headers {}, PostgreSQL metadata {metadata})",
                        pg_target_includes[0]
                    )
                })?;
        if let Some(path) = env_tracked("PGRX_PG_SYS_EXTRA_OUTPUT_PATH") {
            std::fs::write(path, &bindings)?;
        }
        if let Some(path) = env_tracked("PGRX_PG_SYS_EXTRA_TARGET_INFO_PATH") {
            if !native.active {
                return Err(eyre!(
                    "target artifact export requires exactly the active PostgreSQL version"
                ));
            }
            target_artifacts::export(
                Path::new(&path),
                target_artifact_domain(major_version)?,
                &bindings,
                &macros,
                native.out_dir,
            )?;
        }
        (bindings, macros)
    };
    let bindings = syn::parse_file(bindings.as_str())
        .wrap_err_with(|| "failed to parse generated bindings")?;
    Ok((bindings, macros))
}

/// Bind imported native archives to Cargo's exact target and selected shim feature.
fn target_artifact_domain(major: u16) -> eyre::Result<target_artifacts::ArtifactDomain> {
    Ok(target_artifacts::ArtifactDomain {
        major,
        rust_target: env_tracked("TARGET")
            .ok_or_else(|| eyre!("target artifact needs Cargo TARGET"))?,
        cshim: env_tracked("CARGO_FEATURE_CSHIM").as_deref() == Some("1"),
        generator_version: env!("CARGO_PKG_VERSION").to_owned(),
    })
}

/// Given a specific postgres version, `run_bindgen` generates bindings for the given
/// postgres version and returns them as a token stream.
fn run_bindgen(
    major_version: u16,
    pg_config: &PgConfig,
    include_h: &path::Path,
    target: &str,
    pg_target_includes: &[String],
    native: NativeBuild<'_>,
) -> eyre::Result<(String, MacroOutput)> {
    eprintln!("Generating bindings for pg{major_version}");
    let _lock = MACRO_SCANNER.lock().map_err(|_| eyre!("C macro scanner lock was poisoned"))?;
    let scanner = MacroScanner::new().wrap_err("could not initialize C macro discovery")?;
    let postgres = macro_postgres_config(pg_config, &pg_target_includes[0])?;
    let explicit_clang = env_tracked("CLANG_PATH").map(PathBuf::from);
    let environment_arguments = bindgen_environment_arguments(env_tracked);
    let cflags = macro_cflags(pg_config, env_tracked)?;
    let mut macro_arguments = cflags.applied.clone();
    // Ordinary bindings deliberately omit MSVC CPPFLAGS. The separate
    // macro profile still needs the recorded preprocessing controls.
    if env_tracked("CARGO_CFG_TARGET_OS").as_deref() == Some("windows") {
        let flags = pg_config.cppflags()?;
        let flags = flags.to_str().ok_or_else(|| eyre!("PostgreSQL CPPFLAGS are not UTF-8"))?;
        macro_arguments.extend(recorded_macro_flags(flags, env_tracked)?.unwrap_or_default());
    }
    let cppflags = extra_bindgen_clang_args(pg_config)?;
    // Bindgen's own include detection asks whichever clang clang-sys finds, which need not
    // match the loaded libclang; PostgreSQL 16's `#include <emmintrin.h>` then reaches builtin
    // headers of another Clang release. Ask the driver macro inspection selects instead, with
    // the profile's own target, sysroot and toolchain arguments, such as the `-isysroot` that
    // macOS records in CPPFLAGS, and give its search list to bindgen and inspection alike.
    let include_detection = if target_env_tracked("PGRX_BINDGEN_NO_DETECT_INCLUDES").is_some() {
        None
    } else {
        let profile = [&macro_arguments[..], &cppflags, &environment_arguments].concat();
        let arguments = include_search_arguments(target, &profile);
        let includes = postgres
            .system_includes(&scanner, &arguments, explicit_clang.as_deref())
            .wrap_err("could not detect the Clang driver's include directories for the target")?;
        eprintln!(
            "Clang include directories from {}: {:?}",
            includes.compiler.display(),
            includes.directories
        );
        Some(IncludeDetection {
            include_arguments: system_include_arguments(&includes)?,
            compiler: includes.compiler,
            arguments,
        })
    };
    let mut binder = bindgen::Builder::default();
    binder = add_blocklists(binder, major_version);
    binder = add_allowlists(binder, pg_target_includes.iter().map(|x| x.as_str()));
    binder = add_derives(binder);
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut arguments = include_detection
        .as_ref()
        .map(|detection| detection.include_arguments.clone())
        .unwrap_or_default();
    arguments.extend(cppflags);
    arguments.extend(pg_target_includes.iter().map(|include| format!("-I{include}")));
    // Make bindgen's otherwise implicit Cargo target explicit for both inspections.
    if !has_explicit_clang_target(arguments.iter().chain(&environment_arguments)) {
        arguments.insert(0, format!("--target={}", clang_target(target)));
    }
    macro_arguments.extend_from_slice(&arguments);
    let enum_names = Rc::new(RefCell::new(BTreeMap::new()));
    // The bindgen invocation is independent of macro CFLAGS, compiler-driver
    // discovery and native support. Inspection still verifies the fresh Rust
    // storage against the full recorded PostgreSQL C profile.
    let generate = || -> eyre::Result<String> {
        let overrides = BindingOverride::new_from(Rc::clone(&enum_names));
        let bindings = binder
            .header(include_h.display().to_string())
            .clang_args(arguments)
            .detect_include_paths(false)
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
            // PostgreSQL routinely replaces macros with inline functions. Keep
            // their generated native bindings available with either cshim API
            // configuration instead of maintaining Rust translations.
            .wrap_static_fns(true)
            .wrap_static_fns_path(out_path.join(cshim_static_wrapper_name(major_version)))
            .wrap_static_fns_suffix("__pgrx_cshim")
            .generate()
            .wrap_err_with(|| format!("Unable to generate bindings for pg{major_version}"))?;

        Ok(bindings.to_string())
    };
    generate_macros(
        &scanner,
        &postgres,
        include_h,
        macro_arguments,
        &environment_arguments,
        explicit_clang.as_deref(),
        include_detection.as_ref(),
        native,
        &cflags,
        target,
        generate,
    )
}

/// Give macro ownership to the server headers that bindgen and inspection include, the first
/// of `pg_target_includes`. A `PGRX_INCLUDEDIR_SERVER` override replaces pg_config's directory
/// as that root, so the macros it defines are generated and the installation's are not.
fn macro_postgres_config(pg_config: &PgConfig, server_root: &str) -> eyre::Result<PostgresConfig> {
    Ok(PostgresConfig::from_pg_config(pg_config.clone())?
        .with_server_include_dir(server_root.into()))
}

/// The Clang driver query that chose the system include directories of bindgen and inspection.
#[derive(Debug, Serialize)]
struct IncludeDetection {
    /// The queried driver, which macro inspection must also select.
    compiler: PathBuf,
    /// The target, sysroot and toolchain arguments of the query: the profile without its
    /// include directories.
    arguments: Vec<String>,
    /// The driver's search list as bindgen and inspection receive it.
    include_arguments: Vec<String>,
}

/// Pass the driver's system directories as system directories, in the driver's order and with
/// each directory's kind, so libclang finds the same headers. As for the driver, `-I`
/// directories such as the server's are searched first, and libclang ignores a repeated
/// directory it already searches.
fn system_include_arguments(includes: &SystemIncludes) -> eyre::Result<Vec<String>> {
    includes
        .directories
        .iter()
        .map(|SystemIncludeDirectory { path, framework }| {
            let path = path
                .to_str()
                .ok_or_else(|| eyre!("Clang include directory {} is not UTF-8", path.display()))?;
            Ok(if *framework { format!("-iframework{path}") } else { format!("-isystem{path}") })
        })
        .collect()
}

/// Require macro inspection to have used the driver whose search list the arguments contain.
/// Both select through the same lookup, so they differ only if the lookup's inputs changed in
/// between.
fn require_include_driver(include_driver: Option<&Path>, inspected: &Path) -> eyre::Result<()> {
    match include_driver {
        Some(driver) if driver != inspected => Err(eyre!(
            "include directories came from {} but C macro inspection selected {}",
            driver.display(),
            inspected.display()
        )),
        _ => Ok(()),
    }
}

/// Name the supported ways to supply target headers when the selected headers were configured
/// for another machine, ABI or major version. Other inspection failures keep their general
/// context.
fn inspection_error(error: PostgresError, major_version: u16, target: &str) -> eyre::Report {
    let context = if matches!(error, PostgresError::TargetConfiguration(_)) {
        format!(
            "these server headers do not describe {target}; set PGRX_INCLUDEDIR_SERVER_PG{major_version}_{target} (or PGRX_INCLUDEDIR_SERVER_PG{major_version}_{}) to PostgreSQL {major_version} server headers configured for {target}, describe the target with PGRX_PG_CONFIG_AS_ENV, or import a target bundle with PGRX_TARGET_INFO_PATH_PG{major_version}",
            target.replace('-', "_")
        )
    } else {
        "could not establish the binding generator's C macro compilation profile".into()
    };
    eyre::Report::new(error).wrap_err(context)
}

/// Carry the generated macro tree and audit report from inspection through binding publication.
struct MacroOutput {
    /// Versioned macro tree and adapters ready for stable publication.
    files: MacroFiles,
    /// Serialized compiler facts and macro skip diagnostics written alongside the bindings.
    report: Vec<u8>,
    /// Successful public definitions counted for the Cargo summary.
    emitted: usize,
    /// Rejected candidates counted without treating unsupported syntax as build failure.
    skipped: usize,
    /// Recorded CFLAGS this build left out of the macro profile, for the Cargo summary.
    cflags_dropped: Vec<String>,
    /// Verified scalar identity choices for the active defining runtime.
    runtime_cfg: Vec<String>,
    /// Documentation for the docs.rs snapshot, collected only during release generation.
    snapshot: Option<SnapshotVersion>,
}

/// Serialize the verified C invocation, fresh binding facts, and per-macro results for build
/// diagnostics and auditing.
#[derive(Serialize)]
struct MacroReport<'a> {
    /// Installation version associated with these compiler and binding facts.
    postgres_major_version: u16,
    /// Always `generated`; bundle import requires it as evidence of original-C inspection.
    status: &'static str,
    /// Whether the installation recorded historical CFLAGS. When false the
    /// verified header, target and caller invocation define macro semantics;
    /// the report makes no claim about unrecorded server build options.
    cflags_recorded: bool,
    /// Recorded CFLAGS left out of the profile: a host `pg_config`'s flags that do not
    /// describe a cross target, and `-arch` slice selections that Cargo's target replaces.
    cflags_dropped: &'a [String],
    /// The driver query behind the profile's system include directories; absent when
    /// `PGRX_BINDGEN_NO_DETECT_INCLUDES` skips it.
    include_detection: Option<&'a IncludeDetection>,
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
    /// Absent bindgen constants independently resolved from the original C definitions.
    supplemental_constants: &'a pgrx_c_macros::ObjectIntegerConstants,
    /// Fresh Rust binding paths, storage, and values reconciled with the C catalog.
    integer_bindings: &'a BindingCatalog,
}

/// Inspect the selected installation, generate and reconcile fresh bindings, emit supported
/// macros and native adapters, verify unchanged C inputs, and serialize structured skips.
#[allow(clippy::too_many_arguments)] // One inspection owns compiler inputs, binding storage and native publication.
fn generate_macros(
    scanner: &MacroScanner,
    postgres: &PostgresConfig,
    header: &Path,
    binder_arguments: Vec<String>,
    environment_arguments: &[String],
    preferred_clang: Option<&Path>,
    include_detection: Option<&IncludeDetection>,
    native: NativeBuild<'_>,
    cflags: &MacroCflags,
    target: &str,
    generate_bindings: impl FnOnce() -> eyre::Result<String>,
) -> eyre::Result<(String, MacroOutput)> {
    let major_version = postgres.pg_config().major_version()?;
    let msvc = env_tracked("CARGO_CFG_TARGET_ENV").as_deref() == Some("msvc");
    // Bindgen appends its original environment tail itself. A CRT selection
    // there cannot be lowered without changing the tail seen by libclang;
    // refuse instead of inspecting different flags from the binding parser.
    if msvc && lower_msvc_runtime_flags(environment_arguments, true)? != environment_arguments {
        return Err(eyre!(
            "MSVC runtime selections in BINDGEN_EXTRA_CLANG_ARGS are unsupported; \
             supply them in the recorded PostgreSQL CFLAGS/CPPFLAGS instead"
        ));
    }
    let mut effective_arguments = binder_arguments.clone();
    effective_arguments.extend_from_slice(environment_arguments);
    // A failed build script always reruns, so only a successful inspection
    // needs its inputs tracked.
    let frontend = postgres
        .inspect_with_arguments(scanner, header, &effective_arguments, preferred_clang)
        .map_err(|error| inspection_error(error, major_version, target))?;
    require_include_driver(
        include_detection.map(|detection| detection.compiler.as_path()),
        &frontend.profile().compiler.executable,
    )?;
    emit_macro_rerun_inputs(&frontend.profile().inputs, native.out_dir)?;
    let mut names = postgres_function_macro_names(&frontend, postgres.server_include_dir())?;
    let owned_objects = postgres_object_macro_names(&frontend, postgres.server_include_dir())?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut global_roots = frontend
        .environment()
        .active
        .iter()
        .filter_map(|(name, active)| {
            active
                .definition
                .tokens
                .iter()
                .any(|token| frontend.declarations().variables.contains_key(&token.spelling))
                .then_some(name.clone())
        })
        .collect::<BTreeSet<_>>();
    // Propagate declaration references once through caller edges. Integer
    // constants belong in bindings; only context-bearing object expressions
    // require a zero-argument macro rather than duplicated constant exports.
    global_roots.extend(
        frontend
            .dependencies()
            .impacts(&global_roots.iter().collect::<Vec<_>>())
            .into_iter()
            .map(|impact| impact.name),
    );
    let object_names = frontend
        .environment()
        .active
        .iter()
        .filter_map(|(name, active)| {
            let definition = &active.definition;
            (definition.kind == pgrx_c_macros::MacroKind::ObjectLike
                && definition.tokens.len() > 1
                && global_roots.contains(name)
                && owned_objects.contains(name))
            .then_some(name.clone())
        })
        .collect::<Vec<_>>();
    let inline_names = postgres_inline_function_names(&frontend, postgres.server_include_dir())?;
    let session = AnalysisSession::prepare_with_inline_functions(
        scanner,
        &frontend,
        &names,
        &object_names,
        &inline_names,
    )?;
    names.extend(object_names);
    names.extend(inline_names);
    let mut source = pg_sys_integer_bridges(&frontend)?;
    // Macros require an unchanged input window around binding generation.
    let mut bindings = generate_bindings()?;
    session.verify_inputs().wrap_err("C inputs changed during binding generation")?;
    let mut parsed_bindings =
        syn::parse_file(&bindings).wrap_err("could not parse bindings for C symbol references")?;
    let available =
        parsed_bindings
            .items
            .iter()
            .filter_map(|item| {
                if let Item::Const(item) = item { Some(item.ident.to_string()) } else { None }
            })
            .collect::<BTreeSet<_>>();
    let missing_objects = frontend
        .environment()
        .active
        .iter()
        .filter_map(|(name, active)| {
            let definition = &active.definition;
            if definition.kind != pgrx_c_macros::MacroKind::ObjectLike
                || available.contains(name)
                || !owned_objects.contains(name)
            {
                return None;
            }
            let body = &definition.tokens[1..];
            // Empty header guards, strings and declaration fragments are not
            // numeric binding candidates. The compiler owns the final type proof.
            let numeric = body.iter().any(|token| {
                (token.kind == pgrx_c_macros::TokenKind::Literal
                    && !token.spelling.starts_with('"'))
                    || matches!(
                        token.spelling.as_str(),
                        "sizeof" | "offsetof" | "__builtin_offsetof"
                    )
                    || frontend.declarations().integer_constants.contains_key(&token.spelling)
                    || frontend.environment().active.get(&token.spelling).is_some_and(|active| {
                        active.definition.kind == pgrx_c_macros::MacroKind::ObjectLike
                            && active.definition.tokens.len() > 1
                    })
            });
            (numeric
                && !body.iter().any(|token| matches!(token.spelling.as_str(), "{" | "}" | ";")))
            .then_some(name.clone())
        })
        .collect::<Vec<_>>();
    let objects = probe_integer_object_constants(scanner, &frontend, &missing_objects)?;
    for (name, constant) in &objects.constants {
        let pgrx_c_macros::TypeCategory::Integer(kind) = constant.ty.category else { continue };
        let layout = &frontend.profile().target.integers[&kind];
        let ty = if kind == pgrx_c_macros::IntegerKind::Bool {
            "bool".to_owned()
        } else if kind == frontend.profile().target.size_type {
            "usize".to_owned()
        } else {
            format!("{}{}", if layout.signed { "i" } else { "u" }, layout.bits)
        };
        let value = match constant.value {
            pgrx_c_macros::IntegerValue::Signed(value) => value.to_string(),
            pgrx_c_macros::IntegerValue::Unsigned(value) => value.to_string(),
        };
        let value = if kind == pgrx_c_macros::IntegerKind::Bool {
            (value != "0").to_string()
        } else {
            value
        };
        let item = syn::parse_str::<syn::ItemConst>(&format!(
            "/// Compiler-verified C object macro `{name}`, omitted by bindgen.\npub const {name}: {ty} = {value};"
        ))?;
        bindings.push_str(&item.to_token_stream().to_string());
        parsed_bindings.items.push(Item::Const(item));
    }
    // Callback storage facts must reflect the same ABI rewrite as the final
    // bindings, while foreign function guards are still generated afterward.
    rewrite_c_abi_to_c_unwind(&mut parsed_bindings);
    let mut symbols = binding_symbols::collect_bindings_at(
        &parsed_bindings,
        &session
            .integer_constants()
            .iter()
            .chain(objects.constants.iter())
            .map(|(name, constant)| (name.clone(), constant.clone()))
            .collect(),
        frontend.declarations(),
        &frontend.profile().target,
        &["__pgrx_c_bindings".into()],
    );
    symbols.ffi_boundary = Some(vec!["ffi".into(), "pg_guard_ffi_boundary".into()]);
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
    let pgrx_c_macros::MacroGeneration { macros: emissions, support } =
        generate_with_bindings(&session, &names, &symbols).map_err(|message| eyre!(message))?;
    let metadata = metadata_support::generate(&frontend, &symbols)?;
    source.push_str(&metadata.rust);
    source.push_str(&support.rust);
    // Metadata support always contains C, so the active version always builds
    // its archive. It also owns bindgen's static wrappers and, when enabled,
    // the C shim.
    if native.active {
        compile_macro_support(
            major_version,
            frontend.profile(),
            &format!("{}\n{}", support.c_source, metadata.c_source),
            &native,
        )?;
    }
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
    // The snapshot ships exactly the support this build includes with its macros.
    let snapshot = if is_for_release() {
        Some(SnapshotVersion::new(
            major_version,
            &emissions,
            postgres.server_include_dir(),
            source.clone(),
        )?)
    } else {
        None
    };
    let files = MacroFiles::new(source, &emissions, postgres.server_include_dir())?;
    let report = serde_json::to_vec_pretty(&MacroReport {
        postgres_major_version: major_version,
        status: "generated",
        cflags_recorded: cflags.recorded,
        cflags_dropped: &cflags.dropped,
        include_detection,
        profile: frontend.profile(),
        inputs: session.inputs(),
        diagnostics: &frontend.inventory().diagnostics,
        macros: &emissions,
        integer_constants: session.integer_constants(),
        supplemental_constants: &objects,
        integer_bindings: &symbols,
    })?;
    let runtime_cfg = pgrx_c_macros::support_rust_cfg(frontend.profile())?;
    if native.active {
        // Publish linkage only after every proof, fingerprint and serialization succeeds.
        link_macro_support(major_version, native.out_dir);
    }
    let cflags_dropped = cflags.dropped.clone();
    Ok((
        bindings,
        MacroOutput { files, report, emitted, skipped, cflags_dropped, runtime_cfg, snapshot },
    ))
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

/// Select the arguments that make Clang report the profile's system search paths: Cargo's
/// target unless one is explicit, plus the profile's recorded flags, CPPFLAGS and environment
/// arguments, whose sysroot and toolchain options choose the list. Include directories are
/// removed with bindgen 0.72's own detection filter, so they are not reported back as system
/// paths.
fn include_search_arguments(target: &str, profile_arguments: &[String]) -> Vec<String> {
    let mut arguments = Vec::with_capacity(profile_arguments.len() + 1);
    let mut include_value = false;
    for argument in profile_arguments {
        if std::mem::take(&mut include_value) {
            continue;
        }
        if argument == "-I" || argument == "--include-directory" {
            include_value = true;
        } else if !argument.starts_with("-I") && !argument.starts_with("--include-directory=") {
            arguments.push(argument.clone());
        }
    }
    if !has_explicit_clang_target(arguments.iter()) {
        arguments.insert(0, format!("--target={}", clang_target(target)));
    }
    arguments
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
    for name in inputs.environment.keys() {
        if name.contains(['\n', '\r']) {
            return Err(eyre!("invalid Cargo environment dependency name"));
        }
        println!("cargo:rerun-if-env-changed={name}");
    }
    for path in macro_rerun_paths(inputs, out_dir)? {
        cargo_input_path(&path)?;
        println!("cargo:rerun-if-changed={}", path.display());
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
        if directory.is_dir() {
            watch_input_directory(&mut watched, directory, &out_dirs)?;
        } else {
            watch_absent_input(&mut watched, directory, &out_dirs)?;
        }
    }
    for file in &inputs.files {
        if matches!(inputs.fingerprints.get(file), Some(None)) {
            watch_absent_input(&mut watched, file, &out_dirs)?;
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

/// Watch the parent of an absent file or search directory so that creating it invalidates the
/// inspected profile; Cargo would mark the missing path itself perpetually dirty.
///
/// Do not climb past a missing parent: the nearest existing ancestor of an absent `PATH` or `-I`
/// root can be `$HOME`, `/usr/lib` or `/`, which Cargo would scan recursively. Do not watch a
/// parent that contains OUT_DIR either, as a project below `$HOME` would then invalidate its own
/// output. The environment values and arguments that named the input remain tracked.
///
/// An absent input inside OUT_DIR is refused at any depth, even before its parent exists: this
/// build owns that directory, so it cannot also be an inspected C input.
fn watch_absent_input(
    watched: &mut BTreeSet<PathBuf>,
    absent: &Path,
    out_dirs: &[PathBuf],
) -> eyre::Result<()> {
    // The missing tail has no canonical identity. Resolve its existing prefix so an alias of
    // OUT_DIR is recognized as well as its spelling.
    let resolved = absent.ancestors().find(|ancestor| ancestor.is_dir()).and_then(|existing| {
        Some(existing.canonicalize().ok()?.join(absent.strip_prefix(existing).ok()?))
    });
    if [Some(absent), resolved.as_deref()]
        .into_iter()
        .flatten()
        .any(|path| out_dirs.iter().any(|out_dir| path.starts_with(out_dir)))
    {
        return Err(eyre!(
            "absent C macro input {} overlaps this build's OUT_DIR; the build's own output cannot be an inspected C input",
            absent.display()
        ));
    }
    let Some(parent) = absent.parent().filter(|parent| parent.is_dir()) else {
        return Ok(());
    };
    let identity = parent.canonicalize().wrap_err_with(|| {
        format!("could not resolve C macro input directory {}", parent.display())
    })?;
    let contains_output = |path: &Path| {
        out_dirs.iter().any(|out_dir| out_dir.starts_with(path) && out_dir.as_path() != path)
    };
    if contains_output(parent) || contains_output(&identity) {
        return Ok(());
    }
    watch_input_directory(watched, parent, out_dirs)
}

/// Find an existing parent to watch when a legacy compiler-search directory has not yet been
/// created.
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

fn add_blocklists(bind: bindgen::Builder, major_version: u16) -> bindgen::Builder {
    let bind = if major_version < 16 {
        // PG15's unprototyped tree callbacks have explicit guarded ABI
        // declarations. On later versions these names are generated C macros.
        bind.blocklist_function("expression_tree_walker")
            .blocklist_function("planstate_tree_walker")
            .blocklist_function("query_or_expression_tree_walker")
            .blocklist_function("query_tree_walker")
            .blocklist_function("range_table_entry_walker")
            .blocklist_function("range_table_walker")
            .blocklist_function("raw_expression_tree_walker")
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
        // Rust version utilities are independent of PostgreSQL declarations.
        .blocklist_function("get_pg_major_minor_version_string")
        .blocklist_function("get_pg_major_version_num")
        .blocklist_function("get_pg_major_version_string")
        .blocklist_function("get_pg_version_string")
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
    target_env_lookup(s, env_tracked).map(|(_, value)| value)
}

/// Prefer the variable suffixed with Cargo's TARGET, then the underscore spelling that shells
/// can export, then the unsuffixed variable. Returns the name that was set with its value.
fn target_env_lookup(s: &str, lookup: impl Fn(&str) -> Option<String>) -> Option<(String, String)> {
    let target = lookup("TARGET").expect("Cargo sets TARGET for every build script");
    let underscored = target.replace('-', "_");
    let mut names = vec![format!("{s}_{target}")];
    if underscored != target {
        names.push(format!("{s}_{underscored}"));
    }
    names.push(s.to_owned());
    names.into_iter().find_map(|name| lookup(&name).map(|value| (name, value)))
}

/// Resolve an include directory from the first set spelling of `{var}_PG{N}` or `{var}`, or
/// from `default`, the PostgreSQL metadata. A missing directory names the variable that set it.
fn find_include(
    pg_version: u16,
    var: &str,
    default: impl Fn() -> eyre::Result<PathBuf>,
) -> eyre::Result<String> {
    let value = target_env_lookup(&format!("{var}_PG{pg_version}"), env_tracked)
        .or_else(|| target_env_lookup(var, env_tracked));
    let (path, source) = match value {
        None => (default()?, "the PostgreSQL metadata".to_owned()),
        Some((name, overridden)) => (PathBuf::from(overridden), name),
    };
    let path = std::fs::canonicalize(&path)
        .wrap_err_with(|| format!("cannot find {path:?} for C header files, from {source}"))?
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

/// PostgreSQL's machine-independent semantic flags (meson's `common_functional_flags` and the
/// configure.ac equivalents). They are the only host CFLAGS that also describe a cross target.
const POSTGRES_FUNCTIONAL_CFLAGS: [&str; 3] =
    ["-fno-strict-aliasing", "-fwrapv", "-fexcess-precision=standard"];

/// The installation's recorded CFLAGS, split by whether they apply to the macro profile.
#[derive(Debug, Default, PartialEq)]
struct MacroCflags {
    /// Flags applied to macro inspection and native compilation, in recorded order.
    applied: Vec<String>,
    /// Flags left out of the profile, in recorded order.
    dropped: Vec<String>,
    /// Whether the installation recorded CFLAGS at all.
    recorded: bool,
}

/// Decode the installation's recorded compiler flags without losing shell quoting;
/// reject malformed or non-UTF-8 profiles before bindgen and native macros can diverge.
fn macro_cflags(
    pg_config: &PgConfig,
    lookup: impl Fn(&str) -> Option<String>,
) -> eyre::Result<MacroCflags> {
    let flags = pg_config.optional_cflags()?;
    let flags = flags
        .as_deref()
        .map(|flags| flags.to_str().ok_or_else(|| eyre!("PostgreSQL CFLAGS are not UTF-8")))
        .transpose()?;
    scope_macro_cflags(flags, pg_config.is_real(), lookup)
}

/// A `pg_config` program describes the machine it runs on, so in a cross build its codegen and
/// ISA flags describe the host server rather than the target. Keep only PostgreSQL's functional
/// flags there. AS_ENV metadata and native builds describe the target and apply in full, except
/// for `-arch` slice selections: Cargo's target selects the slice of a universal macOS build.
fn scope_macro_cflags(
    flags: Option<&str>,
    host_program: bool,
    lookup: impl Fn(&str) -> Option<String>,
) -> eyre::Result<MacroCflags> {
    let Some(flags) =
        flags.map(|flags| recorded_macro_flags(flags, &lookup)).transpose()?.flatten()
    else {
        return Ok(MacroCflags::default());
    };
    let (applied, dropped) = if host_program && lookup("HOST") != lookup("TARGET") {
        flags.into_iter().partition(|flag| POSTGRES_FUNCTIONAL_CFLAGS.contains(&flag.as_str()))
    } else {
        pgrx_c_macros::separate_architecture_flags(flags)
    };
    Ok(MacroCflags { applied, dropped, recorded: true })
}

/// Decode one recorded CFLAGS or CPPFLAGS fragment for the macro profile; `None` means the
/// installation did not record flags. MSVC fragments contain CL switches such as meson's
/// `/wd4018`. Translate them with the reviewed table the CLI also uses, so both inspections see
/// the same GNU-style Clang arguments.
fn recorded_macro_flags(
    flags: &str,
    lookup: impl Fn(&str) -> Option<String>,
) -> eyre::Result<Option<Vec<String>>> {
    let windows = lookup("CARGO_CFG_TARGET_OS").as_deref() == Some("windows");
    let Some(arguments) = pgrx_c_macros::split_recorded_cflags(flags, windows)? else {
        return Ok(None);
    };
    if lookup("CARGO_CFG_TARGET_ENV").as_deref() == Some("msvc") {
        return Ok(Some(pgrx_c_macros::postgres_clang_flags(flags, true)?));
    }
    Ok(Some(arguments))
}

/// Preserve the established ordinary binding arguments independently of macro inspection;
/// PostgreSQL's historical CFLAGS belong to the separate verified macro profile.
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
///
/// The tests inspect real parsed output, generated leaf maps, and compiler dependency
/// directives. Temporary input trees make success and rejection conditions explicit without
/// relying on a configured server.
#[cfg(test)]
mod macro_build_tests {

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

    /// Target-scoped PGRX variables accept the hyphenated Cargo spelling first, then the
    /// underscore spelling a shell can export, then the unsuffixed name.
    #[test]
    fn target_env_tracked_accepts_underscore_target_spelling() {
        let mut values = BTreeMap::from([
            ("TARGET", "aarch64-unknown-linux-gnu"),
            ("PGRX_INCLUDEDIR_SERVER", "/host/server"),
            ("PGRX_INCLUDEDIR_SERVER_aarch64_unknown_linux_gnu", "/underscored/server"),
            ("PGRX_INCLUDEDIR_SERVER_aarch64-unknown-linux-gnu", "/hyphenated/server"),
        ]);
        let resolve = |values: &BTreeMap<&str, &str>| {
            target_env_lookup("PGRX_INCLUDEDIR_SERVER", |name| {
                values.get(name).map(|value| (*value).to_owned())
            })
        };
        let found = |name: &str, value: &str| Some((name.to_owned(), value.to_owned()));
        assert_eq!(
            resolve(&values),
            found("PGRX_INCLUDEDIR_SERVER_aarch64-unknown-linux-gnu", "/hyphenated/server")
        );
        values.remove("PGRX_INCLUDEDIR_SERVER_aarch64-unknown-linux-gnu");
        assert_eq!(
            resolve(&values),
            found("PGRX_INCLUDEDIR_SERVER_aarch64_unknown_linux_gnu", "/underscored/server")
        );
        values.remove("PGRX_INCLUDEDIR_SERVER_aarch64_unknown_linux_gnu");
        assert_eq!(resolve(&values), found("PGRX_INCLUDEDIR_SERVER", "/host/server"));
        values.remove("PGRX_INCLUDEDIR_SERVER");
        assert_eq!(resolve(&values), None);
    }

    /// Meson-built MSVC PostgreSQL records CL warning switches in CFLAGS. The build inspection
    /// decodes them with the CLI's reviewed translation instead of passing raw CL switches on.
    #[test]
    fn recorded_msvc_macro_flags_use_the_reviewed_cl_translation() {
        let target = |os: &'static str, env: &'static str| {
            move |name: &str| match name {
                "CARGO_CFG_TARGET_OS" => Some(os.to_owned()),
                "CARGO_CFG_TARGET_ENV" => Some(env.to_owned()),
                _ => None,
            }
        };
        let recorded = "-O2 /W3 /wd4018 /wd4244 /DWIN32";
        let decoded = recorded_macro_flags(recorded, target("windows", "msvc")).unwrap().unwrap();
        assert_eq!(decoded, ["-O3", "-fomit-frame-pointer", "-ffunction-sections", "-DWIN32"]);
        assert_eq!(decoded, pgrx_c_macros::postgres_clang_flags(recorded, true).unwrap());
        // PostgreSQL 19's meson.build adds C11 and level and error switches for single warnings.
        let pg19 = "/std:c11 -Wall -O2 /external:anglebrackets /external:W0 /wd4244 /w24062 \
                    /w24102 /w24334 /we4013 /wd4090 /wd4018 /wd4101 /wd4267 /w24255";
        assert_eq!(
            recorded_macro_flags(pg19, target("windows", "msvc")).unwrap().unwrap(),
            ["-std=c11", "-O3", "-fomit-frame-pointer", "-ffunction-sections"]
        );
        assert_eq!(recorded_macro_flags("/W3", target("windows", "msvc")).unwrap(), Some(vec![]));
        assert_eq!(recorded_macro_flags("not recorded", target("windows", "msvc")).unwrap(), None);
        assert!(recorded_macro_flags("/unknown", target("windows", "msvc")).is_err());
        assert_eq!(
            recorded_macro_flags("-I'/a b' -O2", target("linux", "gnu")).unwrap().unwrap(),
            ["-I/a b", "-O2"]
        );
    }

    /// A `pg_config` program reports the host server's flags. A cross build keeps only
    /// PostgreSQL's functional flags from it, in recorded order, so host ISA options such as
    /// `-fcf-protection` never reach the target. Native builds and AS_ENV keep every flag but
    /// Darwin `-arch` selections, which the next test covers.
    #[test]
    fn macro_cflags_scope_host_metadata_in_cross_builds() {
        let recorded = "-O2 -fno-strict-aliasing -fcf-protection -fwrapv \
                        -mno-omit-leaf-frame-pointer -fexcess-precision=standard";
        let environment = |target: &'static str| {
            move |name: &str| match name {
                "HOST" => Some("x86_64-unknown-linux-gnu".to_owned()),
                "TARGET" => Some(target.to_owned()),
                "CARGO_CFG_TARGET_OS" => Some("linux".to_owned()),
                "CARGO_CFG_TARGET_ENV" => Some("gnu".to_owned()),
                _ => None,
            }
        };
        let native =
            scope_macro_cflags(Some(recorded), true, environment("x86_64-unknown-linux-gnu"))
                .unwrap();
        assert_eq!(
            native,
            MacroCflags {
                applied: recorded.split_whitespace().map(String::from).collect(),
                dropped: Vec::new(),
                recorded: true,
            }
        );
        let cross = environment("aarch64-unknown-linux-gnu");
        assert_eq!(
            scope_macro_cflags(Some(recorded), false, cross).unwrap(),
            native,
            "AS_ENV metadata describes the target"
        );
        let scoped = scope_macro_cflags(Some(recorded), true, cross).unwrap();
        assert_eq!(
            scoped.applied,
            ["-fno-strict-aliasing", "-fwrapv", "-fexcess-precision=standard"]
        );
        assert_eq!(scoped.dropped, ["-O2", "-fcf-protection", "-mno-omit-leaf-frame-pointer"]);
        assert!(scoped.recorded);
        for absent in [None, Some(""), Some("not recorded")] {
            assert_eq!(scope_macro_cflags(absent, true, cross).unwrap(), MacroCflags::default());
        }
    }

    /// Postgres.app records `-arch arm64 -arch x86_64`, which Clang cannot preprocess. Cargo's
    /// target selects the slice instead, natively and in cross builds alike.
    #[test]
    fn macro_cflags_leave_universal_slices_to_the_target() {
        let recorded = "-fwrapv -Os -mmacosx-version-min=10.15 -arch arm64 -arch x86_64";
        let environment = |target: &'static str| {
            move |name: &str| match name {
                "HOST" => Some("aarch64-apple-darwin".to_owned()),
                "TARGET" => Some(target.to_owned()),
                "CARGO_CFG_TARGET_OS" => Some("macos".to_owned()),
                _ => None,
            }
        };
        let slices = ["-arch", "arm64", "-arch", "x86_64"];
        for host_program in [true, false] {
            let native = scope_macro_cflags(
                Some(recorded),
                host_program,
                environment("aarch64-apple-darwin"),
            )
            .unwrap();
            assert_eq!(native.applied, ["-fwrapv", "-Os", "-mmacosx-version-min=10.15"]);
            assert_eq!(native.dropped, slices);
        }
        let cross =
            scope_macro_cflags(Some(recorded), true, environment("x86_64-apple-darwin")).unwrap();
        assert_eq!(cross.applied, ["-fwrapv"]);
        assert_eq!(
            cross.dropped,
            ["-Os", "-mmacosx-version-min=10.15", "-arch", "arm64", "-arch", "x86_64"]
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

    /// Include detection receives the target, sysroot and toolchain arguments, but no include
    /// directory, in either spelling, that Clang would echo back as a system search path.
    #[test]
    fn include_search_arguments_match_bindgen_filter() {
        let arguments = |values: &[&str]| -> Vec<String> {
            values.iter().map(|&value| value.to_owned()).collect()
        };
        let environment = arguments(&[
            "--sysroot=/sysroots/aarch64",
            "-I",
            "/separate/include",
            "-I/joined/include",
            "--include-directory",
            "/long/include",
            "--include-directory=/long/joined",
            "-isystem",
            "/target/include",
            "-ccc-gcc-name",
            "aarch64-linux-gnu-gcc",
        ]);
        assert_eq!(
            include_search_arguments("aarch64-unknown-linux-gnu", &environment),
            arguments(&[
                "--target=aarch64-unknown-linux-gnu",
                "--sysroot=/sysroots/aarch64",
                "-isystem",
                "/target/include",
                "-ccc-gcc-name",
                "aarch64-linux-gnu-gcc",
            ])
        );
        assert_eq!(
            include_search_arguments("aarch64-apple-darwin", &[]),
            ["--target=arm64-apple-darwin"],
            "Cargo's target uses bindgen's Clang spelling"
        );
        for explicit in [&["--target=armv7-unknown-linux-gnueabihf"][..], &["-target", "armv7"]] {
            let explicit = arguments(explicit);
            assert_eq!(include_search_arguments("aarch64-unknown-linux-gnu", &explicit), explicit);
        }
    }

    /// The driver's search list stays a list of system directories in its own order, searched
    /// after the server's `-I` directories as the driver searches them, and framework
    /// directories stay framework directories.
    #[test]
    fn system_includes_become_ordered_system_include_arguments() {
        let includes = SystemIncludes {
            compiler: "/usr/bin/clang-14".into(),
            directories: vec![
                SystemIncludeDirectory {
                    path: "/usr/lib/llvm-14/lib/clang/14/include".into(),
                    framework: false,
                },
                SystemIncludeDirectory {
                    path: "/usr/include/aarch64-linux-gnu".into(),
                    framework: false,
                },
                SystemIncludeDirectory {
                    path: "/System/Library/Frameworks".into(),
                    framework: true,
                },
            ],
        };
        assert_eq!(
            system_include_arguments(&includes).unwrap(),
            [
                "-isystem/usr/lib/llvm-14/lib/clang/14/include",
                "-isystem/usr/include/aarch64-linux-gnu",
                "-iframework/System/Library/Frameworks",
            ]
        );
    }

    /// Inspection must have used the driver that reported the include directories.
    #[test]
    fn inspection_must_use_the_include_driver() {
        let driver = Path::new("/usr/bin/clang-14");
        require_include_driver(None, driver).unwrap();
        require_include_driver(Some(driver), driver).unwrap();
        let error = require_include_driver(Some(driver), Path::new("/usr/bin/clang")).unwrap_err();
        assert_eq!(
            error.to_string(),
            "include directories came from /usr/bin/clang-14 but C macro inspection selected /usr/bin/clang"
        );
    }

    /// Headers configured for another machine name the variables that supply target headers,
    /// in both spellings, above the configuration error itself. Other inspection failures keep
    /// the general context.
    #[test]
    fn foreign_headers_name_the_target_header_remedies() {
        let foreign =
            PostgresError::TargetConfiguration(pgrx_c_macros::ConfigurationError::Machine {
                recorded: "x86_64-pc-linux-gnu".into(),
                target: "aarch64-unknown-linux-gnu".into(),
            });
        let chain = inspection_error(foreign, 17, "aarch64-unknown-linux-gnu")
            .chain()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        assert_eq!(chain.len(), 2, "{chain:?}");
        for remedy in [
            "PGRX_INCLUDEDIR_SERVER_PG17_aarch64-unknown-linux-gnu",
            "PGRX_INCLUDEDIR_SERVER_PG17_aarch64_unknown_linux_gnu",
            "PGRX_PG_CONFIG_AS_ENV",
            "PGRX_TARGET_INFO_PATH_PG17",
        ] {
            assert!(chain[0].contains(remedy), "{remedy} missing from {chain:?}");
        }
        assert!(chain[1].contains("configured for x86_64-pc-linux-gnu"), "{chain:?}");
        let other = inspection_error(PostgresError::InvalidCFlags, 17, "aarch64-unknown-linux-gnu");
        assert_eq!(
            other.to_string(),
            "could not establish the binding generator's C macro compilation profile"
        );
        assert!(!format!("{other:?}").contains("PGRX_INCLUDEDIR_SERVER"));
    }

    /// A PGRX_INCLUDEDIR_SERVER override, the first selected include directory, owns the
    /// generated macros rather than pg_config's server directory.
    #[cfg(unix)]
    #[test]
    fn server_include_overrides_own_generated_macros() {
        let directory = TemporaryDirectory::new();
        let pg_config = directory.0.join("pg_config");
        crate::test_script::write_script(
            &pg_config,
            "#!/bin/sh\ncase \"$1\" in\n--includedir-server) echo /installation/server ;;\n--version) echo 'PostgreSQL 18.4' ;;\n--cppflags) echo ;;\n*) exit 1 ;;\nesac\n",
        );
        let postgres =
            macro_postgres_config(&PgConfig::new_with_defaults(pg_config), "/override/server/")
                .unwrap();
        assert_eq!(postgres.server_include_dir(), Path::new("/override/server/"));
    }

    /// Release generation runs only on the single target docs.rs documents.
    #[test]
    fn release_requires_the_single_documented_target() {
        let manifest = |targets: &str| {
            format!("[package]\nname = \"x\"\n[package.metadata.docs.rs]\ntargets = {targets}\n")
        };
        let linux = "x86_64-unknown-linux-gnu";
        check_documented_target(&manifest(r#"["x86_64-unknown-linux-gnu"]"#), linux).unwrap();
        assert_eq!(
            check_documented_target(
                &manifest(r#"["x86_64-unknown-linux-gnu"]"#),
                "aarch64-apple-darwin"
            )
            .unwrap_err()
            .to_string(),
            "release bindings and the C macro snapshot describe x86_64-unknown-linux-gnu, the docs.rs target in Cargo.toml; run release generation on that target, not aarch64-apple-darwin"
        );
        for targets in ["[]", r#"["x86_64-unknown-linux-gnu", "aarch64-apple-darwin"]"#, "[1]"] {
            assert!(check_documented_target(&manifest(targets), linux).is_err(), "{targets}");
        }
        assert!(check_documented_target("[package]\nname = \"x\"\n", linux).is_err());
        check_documented_target(include_str!("../../pgrx-pg-sys/Cargo.toml"), linux).unwrap();
    }

    /// A release is merged and verified before anything is written to `src/include`.
    #[test]
    fn releases_publish_only_after_the_snapshot_verifies() {
        let directory = TemporaryDirectory::new();
        let src = directory.0.join("src/include");
        fs::create_dir_all(&src).unwrap();
        let release = |major: u16, support: &str| {
            let mut snapshot =
                SnapshotVersion::new(major, &[], Path::new("/not/used"), support.into()).unwrap();
            snapshot.generated_by(format!("{major:064x}"));
            ReleaseVersion {
                major,
                bindings: "pub const BINDING: u32 = 1;".into(),
                oids: String::new(),
                snapshot,
            }
        };
        let error = publish_release(
            vec![release(15, "pub mod m {}\n"), release(16, "#[doc(hidden)]\npub mod m {}\n")],
            &src,
        )
        .unwrap_err()
        .to_string();
        assert_eq!(error, "pg15 and pg16 declare support module m differently");
        assert_eq!(fs::read_dir(&src).unwrap().count(), 0, "a failed release publishes nothing");
        assert!(publish_release(Vec::new(), &src).is_err());
        assert_eq!(fs::read_dir(&src).unwrap().count(), 0);

        // A run that configures only pg15 publishes a snapshot that rejects pg16 docs builds.
        publish_release(vec![release(15, "pub mod m {}\n")], &src).unwrap();
        for file in ["pg15.rs", "pg15_oids.rs"] {
            assert!(src.join(file).is_file(), "{file}");
        }
        assert!(!src.join("pg16.rs").exists());
        let index = fs::read_to_string(src.join("cmacros/snapshot/mod.rs")).unwrap();
        assert!(index.contains(&format!("b\"{:064x}\"", 15)), "{index}");
        assert!(index.contains("does not include pg16"), "{index}");
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
        let index_only =
            MacroFiles { sources: BTreeMap::from([(PathBuf::from("mod.rs"), String::new())]) };
        write_macro_files(&index_only, &version, false).unwrap();
        assert!(!leaf.exists(), "a smaller generation removes previous macros");
        assert!(version.join("mod.rs").exists(), "an empty module still loads");
    }

    /// The checked-in documentation snapshot is formatted before its stable writes. OUT_DIR
    /// trees are compiler input that nobody edits: formatting tens of megabytes there cost
    /// seconds per build-script run for cosmetic changes, so they keep the generator's bytes.
    #[test]
    fn only_documentation_snapshots_are_formatted_before_stable_writes() {
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
            if documentation {
                assert!(leaf.contains("pub fn example(value: u32) -> u32 {"));
                assert!(leaf.contains("/// #define EXAMPLE(x) (x)"));
                assert!(leaf.contains("/* PGRX: preserve this comment */"));
                assert_eq!(syn::parse_file(&leaf).unwrap(), syn::parse_file(original).unwrap());
            } else {
                assert_eq!(leaf, original, "OUT_DIR trees are published as generated");
            }
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

    /// Format every leaf in a long-path tree that exceeds Windows' aggregate command-line limit,
    /// preserving source meaning and comments without traversing undeclared child modules.
    #[test]
    fn large_macro_trees_are_formatted_without_oversized_commands() {
        let directory = TemporaryDirectory::new();
        let leaves = directory
            .0
            .join("generated headers with spaces and Unicode λ")
            .join("a longer directory for the generated macro formatting regression");
        fs::create_dir_all(&leaves).unwrap();
        let original = "mod absent;\npub fn value( arg:u32 )->u32{ /* PGRX: keep */ arg+1 }\n";
        let expected = syn::parse_file(original).unwrap();
        let paths = (0..256)
            .map(|index| {
                let path = leaves.join(format!("header_{index:03}.rs"));
                fs::write(&path, original).unwrap();
                path
            })
            .collect::<Vec<_>>();
        assert!(
            paths.iter().map(|path| path.to_str().unwrap().encode_utf16().count()).sum::<usize>()
                > 32_767,
            "the unbatched command must exceed Windows' process limit"
        );
        #[cfg(not(unix))]
        let formatter = PathBuf::from("rustfmt");
        // A Unix proxy reproduces the process limit locally while still executing the real
        // formatter. Windows exercises its own command-line limit with rustfmt directly.
        #[cfg(unix)]
        let formatter = {
            let proxy = directory.0.join("bounded-rustfmt");
            crate::test_script::write_script(
                &proxy,
                "#!/bin/sh\nsize=0\nfor arg; do size=$((size + ${#arg} + 1)); done\nif [ \"$size\" -gt 32767 ]; then echo 'formatter command exceeds process limit' >&2; exit 1; fi\nexec rustfmt \"$@\"\n",
            );
            proxy
        };
        format_macro_files(&paths, &formatter).unwrap();
        for path in paths {
            let source = fs::read_to_string(path).unwrap();
            assert!(source.contains("pub fn value(arg: u32) -> u32 {"));
            assert!(source.contains("/* PGRX: keep */"));
            assert_eq!(syn::parse_file(&source).unwrap(), expected);
        }
    }

    /// Bound quoting and Unicode costs without losing progress for empty or individually
    /// oversized input; a subsequent path must begin a new batch once the budget is exhausted.
    #[test]
    fn macro_formatter_batches_account_for_escaped_paths_and_always_progress() {
        let paths = (0..256)
            .map(|index| {
                PathBuf::from(format!("{}λ😀\"\\{index:03}.rs", "path with spaces/".repeat(12)))
            })
            .collect::<Vec<_>>();
        let count = macro_format_batch_len(&paths);
        assert!(count > 0 && count < paths.len());
        let size = paths[..count]
            .iter()
            .map(|path| 2 * path.as_os_str().as_encoded_bytes().len() + 3)
            .sum::<usize>();
        assert!(size <= MACRO_FORMAT_PATH_BUDGET);
        assert!(
            size + 2 * paths[count].as_os_str().as_encoded_bytes().len() + 3
                > MACRO_FORMAT_PATH_BUDGET
        );
        assert_eq!(macro_format_batch_len(&[]), 0);
        assert_eq!(macro_format_batch_len(&paths[..1]), 1);
        let oversized = [PathBuf::from("x".repeat(MACRO_FORMAT_PATH_BUDGET)), paths[0].clone()];
        assert_eq!(macro_format_batch_len(&oversized), 1);
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
        let formatter = directory.join("fixture-rustfmt");
        crate::test_script::write_script(&formatter, "#!/bin/sh\ncat \"$0.stderr\" >&2\nexit 1\n");
        fs::write(directory.join("fixture-rustfmt.stderr"), diagnostic).unwrap();
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
        assert!(!paths.contains(&isolated_home), "a missing parent is not escalated");
        let nested = isolated_home.join("nested");
        fs::create_dir(&nested).unwrap();
        let paths = macro_rerun_paths(&inputs, &out_dir).unwrap();
        assert!(paths.contains(&nested), "an existing parent tracks the file's creation");
        assert!(paths.contains(&nested.canonicalize().unwrap()));
        assert!(!paths.contains(&missing));
        let unsafe_missing = out_dir.join("missing.toml");
        let unsafe_inputs = BuildInputs {
            files: vec![unsafe_missing.clone()],
            fingerprints: BTreeMap::from([(unsafe_missing, None)]),
            ..BuildInputs::default()
        };
        assert!(macro_rerun_paths(&unsafe_inputs, &out_dir).is_err());
    }

    /// Compiler lookup records every absent PATH and libclang-adjacent candidate. A candidate
    /// under a missing directory must not escalate to a broad existing ancestor: that would
    /// refuse a project below `$HOME` because the ancestor contains OUT_DIR, or watch `$HOME`
    /// or `/usr/lib` recursively on every build. An existing parent that contains OUT_DIR is
    /// not watched either.
    #[test]
    fn absent_candidates_under_missing_directories_do_not_watch_ancestors() {
        let directory = TemporaryDirectory::new();
        let home = directory.0.join("home");
        let out_dir = home.join("extension/target/debug/build/pg-sys/out");
        fs::create_dir_all(&out_dir).unwrap();
        let library_dir = directory.0.join("usr/lib");
        fs::create_dir_all(&library_dir).unwrap();
        let path_candidate = home.join("go/bin/clang-18");
        let adjacent_candidate = library_dir.join("bin/clang");
        // A PATH root such as `.` names the project, whose existing directory contains OUT_DIR.
        let project_candidate = home.join("extension/clang-18");
        let inputs = BuildInputs {
            files: vec![
                path_candidate.clone(),
                adjacent_candidate.clone(),
                project_candidate.clone(),
            ],
            fingerprints: BTreeMap::from([
                (path_candidate.clone(), None),
                (adjacent_candidate.clone(), None),
                (project_candidate, None),
            ]),
            ..BuildInputs::default()
        };
        let watched = macro_watch_directories(&inputs, &out_dir).unwrap();
        assert!(!watched.contains(&home), "an ancestor containing OUT_DIR is not an input");
        assert!(!watched.contains(&library_dir), "a missing bin directory cannot watch lib");
        assert!(watched.is_empty(), "{watched:?}");

        let path_root = path_candidate.parent().unwrap();
        fs::create_dir_all(path_root).unwrap();
        let watched = macro_watch_directories(&inputs, &out_dir).unwrap();
        assert!(watched.contains(path_root), "an existing parent detects candidate creation");
        assert!(!watched.contains(&home));
    }

    /// A missing search directory is watched through its parent when that parent exists, so its
    /// creation invalidates the profile. Like an absent file, it never escalates to a broader
    /// ancestor: a missing `-I$HOME/...` root would otherwise refuse a project below `$HOME`,
    /// whose OUT_DIR that ancestor contains, or make Cargo scan `$HOME` on every build.
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
        let optional = headers.join("optional");
        let inputs = BuildInputs {
            directories: vec![
                headers.join("missing"),
                optional.join("missing"),
                directory.0.join("include"),
                directory.0.join("removed-prefix/include"),
            ],
            executable_search_directories: vec![external_bin.clone(), custom_bin.clone()],
            ..BuildInputs::default()
        };
        let watched = macro_watch_directories(&inputs, &out_dir).unwrap();
        assert!(watched.contains(&headers));
        assert!(watched.contains(&headers.canonicalize().unwrap()));
        assert!(watched.contains(&external_bin));
        assert!(watched.contains(&custom_bin), "custom Cargo-target descendants are still inputs");
        assert!(
            watched.iter().all(|path| !out_dir.starts_with(path)),
            "no ancestor of OUT_DIR is an input: {watched:?}"
        );
        fs::create_dir(&optional).unwrap();
        let watched = macro_watch_directories(&inputs, &out_dir).unwrap();
        assert!(watched.contains(&optional), "an existing parent detects the root's creation");
        let unsafe_inputs =
            BuildInputs { directories: vec![out_dir.clone()], ..BuildInputs::default() };
        let error = macro_watch_directories(&unsafe_inputs, &out_dir).unwrap_err().to_string();
        assert!(error.contains("overlaps this build's OUT_DIR"));
        // A missing input inside OUT_DIR is refused whether or not its parent exists yet.
        for missing in [out_dir.join("include"), out_dir.join("generated/include")] {
            let missing_output =
                BuildInputs { directories: vec![missing.clone()], ..BuildInputs::default() };
            let error = macro_watch_directories(&missing_output, &out_dir).unwrap_err().to_string();
            assert!(error.contains("overlaps this build's OUT_DIR"), "{missing:?}: {error}");
        }
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
        let missing = BuildInputs {
            directories: vec![alias.join("generated/include")],
            ..BuildInputs::default()
        };
        let error = macro_watch_directories(&missing, &out_dir).unwrap_err().to_string();
        assert!(error.contains("overlaps this build's OUT_DIR"), "{error}");
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
