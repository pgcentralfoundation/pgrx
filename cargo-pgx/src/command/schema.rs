/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use crate::command::get::{find_control_file, get_property};
use crate::command::install::format_display_path;
use crate::pgx_pg_sys_stub::PgxPgSysStub;
use crate::profile::CargoProfile;
use crate::CommandExecute;
use cargo_toml::Manifest;
use eyre::{eyre, WrapErr};
use object::Object;
use once_cell::sync::OnceCell;
use owo_colors::OwoColorize;
use pgx_pg_config::{get_target_dir, PgConfig, Pgx};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
// Since we support extensions with `#[no_std]`
extern crate alloc;
use crate::manifest::{get_package_manifest, pg_config_and_version};
use alloc::vec::Vec;

// An apparent bug in `glibc` 2.17 prevents us from safely dropping this
// otherwise users find issues such as https://github.com/tcdi/pgx/issues/572
static POSTMASTER_LIBRARY: OnceCell<libloading::os::unix::Library> = OnceCell::new();

// An apparent bug in `glibc` 2.17 prevents us from safely dropping this
// otherwise users find issues such as https://github.com/tcdi/pgx/issues/572
static EXTENSION_LIBRARY: OnceCell<libloading::os::unix::Library> = OnceCell::new();

/// Generate extension schema files
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Schema {
    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// Build in test mode (for `cargo pgx test`)
    #[clap(long)]
    test: bool,
    /// Do you want to run against Postgres `pg11`, `pg12`, `pg13`, `pg14`, `pg15`?
    pg_version: Option<String>,
    /// Compile for release mode (default is debug)
    #[clap(long, short)]
    release: bool,
    /// Specific profile to use (conflicts with `--release`)
    #[clap(long)]
    profile: Option<String>,
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c', value_parser)]
    pg_config: Option<PathBuf>,
    #[clap(flatten)]
    features: clap_cargo::Features,
    /// A path to output a produced SQL file (default is `stdout`)
    #[clap(long, short, value_parser)]
    out: Option<PathBuf>,
    /// A path to output a produced GraphViz DOT file
    #[clap(long, short, value_parser)]
    dot: Option<PathBuf>,
    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
    /// Skip building a fresh extension shared object.
    #[clap(long)]
    skip_build: bool,
}

impl CommandExecute for Schema {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(mut self) -> eyre::Result<()> {
        let log_level = if let Ok(log_level) = std::env::var("RUST_LOG") {
            Some(log_level)
        } else {
            match self.verbose {
                0 => Some("warn".into()),
                1 => Some("info".into()),
                2 => Some("debug".into()),
                _ => Some("trace".into()),
            }
        };

        let pgx = Pgx::from_config()?;
        let (package_manifest, package_manifest_path) = get_package_manifest(
            &self.features,
            self.package.as_ref(),
            self.manifest_path.as_ref(),
        )?;
        let (pg_config, _pg_version) = pg_config_and_version(
            &pgx,
            &package_manifest,
            self.pg_version.clone(),
            Some(&mut self.features),
            true,
        )?;

        let profile = CargoProfile::from_flags(
            self.profile.as_deref(),
            self.release.then_some(CargoProfile::Release).unwrap_or(CargoProfile::Dev),
        )?;

        generate_schema(
            &pg_config,
            self.manifest_path.as_ref(),
            self.package.as_ref(),
            package_manifest_path,
            &profile,
            self.test,
            &self.features,
            self.out.as_ref(),
            self.dot,
            log_level,
            self.skip_build,
        )
    }
}

// This is *mostly* a copy of the function in `build.rs`, except using
// `CARGO`/`cargo` rather than `RUSTC`/`rustc`. It seems too painful to try and
// share them, given how they're close-but-not-identical.
fn rust_minor_version() -> Option<u32> {
    // In order to support `cargo +whatever pgx`, use `CARGO` here (which
    // cargo sets for subcommands to allow this), if present.
    let rustc = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = std::process::Command::new(rustc).arg("--version").output().ok()?;
    let version = std::str::from_utf8(&output.stdout).ok()?;
    let mut iter = version.split('.');
    if iter.next() != Some("cargo 1") {
        None
    } else {
        iter.next()?.parse().ok()
    }
}

/// Returns an error if the Rust minor version at build time doesn't match the
/// one at runtime.
///
/// This is an error because we `dlopen` rust code that we build, and call
/// `extern "Rust"` functions on `#[repr(Rust)]` types. This may be relaxed in
/// the future, but for now is a requirement.
///
/// To waive this, you may set `PGX_IGNORE_RUST_VERSIONS` in the environment (to
/// any value other than `"0"`). Also, note that this check is best-effort only,
/// and is expected to error only if there is a definite mismatch.
///
/// It also cannot detect versions of `cargo-pgx` and `pgx` differing, which
/// could cause similar issues (in the future this may be detected).
fn check_rust_version() -> eyre::Result<()> {
    const COMPILE_TIME_MINOR_VERSION: Option<&str> = option_env!("MINOR_RUST_VERSION");
    if matches!(std::env::var("PGX_IGNORE_RUST_VERSIONS"), Ok(s) if s != "0") {
        return Ok(());
    }
    let parsed = COMPILE_TIME_MINOR_VERSION.and_then(|s| s.trim().parse::<u32>().ok());
    if let (Some(from_env), Some(run_locally)) = (parsed, rust_minor_version()) {
        if from_env != run_locally {
            eyre::bail!(
                "Mismatched rust versions: `cargo-pgx` was built with Rust `1.{from_env}`, but \
                Rust `1.{run_locally}` is currently in use.",
            );
        }
    }
    Ok(())
}

#[tracing::instrument(level = "error", skip_all, fields(
    pg_version = %pg_config.version()?,
    profile = ?profile,
    test = is_test,
    path = path.as_ref().map(|path| tracing::field::display(path.as_ref().display())),
    dot,
    features = ?features.features,
))]
pub(crate) fn generate_schema(
    pg_config: &PgConfig,
    user_manifest_path: Option<impl AsRef<Path>>,
    user_package: Option<&String>,
    package_manifest_path: impl AsRef<Path>,
    profile: &CargoProfile,
    is_test: bool,
    features: &clap_cargo::Features,
    path: Option<impl AsRef<std::path::Path>>,
    dot: Option<impl AsRef<std::path::Path>>,
    log_level: Option<String>,
    skip_build: bool,
) -> eyre::Result<()> {
    check_rust_version()?;
    let manifest = Manifest::from_path(&package_manifest_path)?;
    let (control_file, _extname) = find_control_file(&package_manifest_path)?;
    let package_name = &manifest
        .package
        .as_ref()
        .ok_or_else(|| eyre!("Could not find crate name in Cargo.toml."))?
        .name;

    if get_property(&package_manifest_path, "relocatable")? != Some("false".into()) {
        return Err(eyre!(
            "{}:  The `relocatable` property MUST be `false`.  Please update your .control file.",
            control_file.display()
        ));
    }

    let versioned_so = get_property(&package_manifest_path, "module_pathname")?.is_none();

    let flags = std::env::var("PGX_BUILD_FLAGS").unwrap_or_default();

    let mut target_dir_with_profile = get_target_dir()?;
    target_dir_with_profile.push(profile.target_subdir());

    // First, build the SQL generator so we can get a look at the symbol table
    if !skip_build {
        let mut command = Command::new("cargo");
        command.stderr(Stdio::inherit());
        command.stdout(Stdio::inherit());
        if is_test {
            command.arg("test");
            command.arg("--no-run");
        } else {
            command.arg("build");
        }

        if let Some(user_package) = user_package {
            command.arg("--package");
            command.arg(user_package);
        }

        if let Some(user_manifest_path) = user_manifest_path {
            command.arg("--manifest-path");
            command.arg(user_manifest_path.as_ref());
        }

        command.args(profile.cargo_args());

        if let Some(log_level) = &log_level {
            command.env("RUST_LOG", log_level);
        }

        let features_arg = features.features.join(" ");
        if !features_arg.trim().is_empty() {
            command.arg("--features");
            command.arg(&features_arg);
        }

        if features.no_default_features {
            command.arg("--no-default-features");
        }

        if features.all_features {
            command.arg("--all-features");
        }

        for arg in flags.split_ascii_whitespace() {
            command.arg(arg);
        }

        let command = command.stderr(Stdio::inherit());
        let command_str = format!("{:?}", command);
        eprintln!(
            "{} for SQL generation with features `{}`",
            "    Building".bold().green(),
            features_arg,
        );

        tracing::debug!(command = %command_str, "Running");
        let cargo_output =
            command.output().wrap_err_with(|| format!("failed to spawn cargo: {}", command_str))?;
        tracing::trace!(status_code = %cargo_output.status, command = %command_str, "Finished");

        if !cargo_output.status.success() {
            // We explicitly do not want to return a spantraced error here.
            std::process::exit(1)
        }
    };

    // Create stubbed `pgx_pg_sys` bindings for the generator to link with.
    let mut postmaster_stub_dir =
        Pgx::postmaster_stub_dir().wrap_err("couldn't get postmaster stub dir env")?;

    postmaster_stub_dir.push(
        pg_config
            .postmaster_path()
            .wrap_err("couldn't get postmaster path")?
            .strip_prefix("/")
            .wrap_err("couldn't make postmaster path relative")?
            .parent()
            .ok_or(eyre!("couldn't get postmaster parent dir"))?,
    );

    let postmaster_path = pg_config.postmaster_path().wrap_err("could not get postmaster path")?;

    // The next action may take a few seconds, we'd like the user to know we're thinking.
    eprintln!("{} SQL entities", " Discovering".bold().green(),);

    let postmaster_stub_built = create_stub(&postmaster_path, &postmaster_stub_dir)?;

    // Inspect the symbol table for a list of `__pgx_internals` we should have the generator call
    let mut lib_so = target_dir_with_profile.clone();

    let so_extension = if cfg!(target_os = "macos") { ".dylib" } else { ".so" };

    lib_so.push(&format!("lib{}{}", package_name.replace('-', "_"), so_extension));

    let lib_so_data = std::fs::read(&lib_so).wrap_err("couldn't read extension shared object")?;
    let lib_so_obj_file =
        object::File::parse(&*lib_so_data).wrap_err("couldn't parse extension shared object")?;
    let lib_so_exports =
        lib_so_obj_file.exports().wrap_err("couldn't get exports from extension shared object")?;

    // Some users reported experiencing duplicate entries if we don't ensure `fns_to_call`
    // has unique entries.
    let mut fns_to_call = HashSet::new();
    for export in lib_so_exports {
        let name = std::str::from_utf8(export.name())?.to_string();
        #[cfg(target_os = "macos")]
        let name = {
            // Mac will prefix symbols with `_` automatically, so we remove it to avoid getting
            // two.
            let mut name = name;
            let rename = name.split_off(1);
            assert_eq!(name, "_");
            rename
        };

        if name.starts_with("__pgx_internals") {
            fns_to_call.insert(name);
        }
    }
    let mut seen_schemas = Vec::new();
    let mut num_funcs = 0_usize;
    let mut num_triggers = 0_usize;
    let mut num_types = 0_usize;
    let mut num_enums = 0_usize;
    let mut num_sqls = 0_usize;
    let mut num_ords = 0_usize;
    let mut num_hashes = 0_usize;
    let mut num_aggregates = 0_usize;
    for func in &fns_to_call {
        if func.starts_with("__pgx_internals_schema_") {
            let schema = func
                .split('_')
                .skip(5)
                .next()
                .expect("Schema extern name was not of expected format");
            seen_schemas.push(schema);
        } else if func.starts_with("__pgx_internals_fn_") {
            num_funcs += 1;
        } else if func.starts_with("__pgx_internals_trigger_") {
            num_triggers += 1;
        } else if func.starts_with("__pgx_internals_type_") {
            num_types += 1;
        } else if func.starts_with("__pgx_internals_enum_") {
            num_enums += 1;
        } else if func.starts_with("__pgx_internals_sql_") {
            num_sqls += 1;
        } else if func.starts_with("__pgx_internals_ord_") {
            num_ords += 1;
        } else if func.starts_with("__pgx_internals_hash_") {
            num_hashes += 1;
        } else if func.starts_with("__pgx_internals_aggregate_") {
            num_aggregates += 1;
        }
    }

    eprintln!(
        "{} {} SQL entities: {} schemas ({} unique), {} functions, {} types, {} enums, {} sqls, {} ords, {} hashes, {} aggregates, {} triggers",
        "  Discovered".bold().green(),
        fns_to_call.len().to_string().bold().cyan(),
        seen_schemas.iter().count().to_string().bold().cyan(),
        seen_schemas.iter().collect::<std::collections::HashSet<_>>().iter().count().to_string().bold().cyan(),
        num_funcs.to_string().bold().cyan(),
        num_types.to_string().bold().cyan(),
        num_enums.to_string().bold().cyan(),
        num_sqls.to_string().bold().cyan(),
        num_ords.to_string().bold().cyan(),
        num_hashes.to_string().bold().cyan(),
        num_aggregates.to_string().bold().cyan(),
        num_triggers.to_string().bold().cyan(),
    );

    tracing::debug!("Collecting {} SQL entities", fns_to_call.len());
    let mut entities = Vec::default();
    let sql_mapping;

    #[rustfmt::skip] // explicit extern "Rust" is more clear here
    unsafe {
        // SAFETY: Calls foreign functions with the correct type signatures.
        // Assumes that repr(Rust) enums are represented the same in this crate as in the external
        // binary, which is the case in practice when the same compiler is used to compile the
        // external crate.

        POSTMASTER_LIBRARY
            .get_or_try_init(|| {
                libloading::os::unix::Library::open(
                    Some(&postmaster_stub_built),
                    libloading::os::unix::RTLD_NOW | libloading::os::unix::RTLD_GLOBAL,
                )
            })
            .wrap_err_with(|| format!("Couldn't libload {}", postmaster_stub_built.display()))?;

        let lib = EXTENSION_LIBRARY
            .get_or_try_init(|| {
                libloading::os::unix::Library::open(Some(&lib_so), libloading::os::unix::RTLD_LAZY)
            })
            .wrap_err_with(|| format!("Couldn't libload {}", lib_so.display()))?;

        let sql_mappings_symbol: libloading::os::unix::Symbol<
            unsafe extern "Rust" fn() -> pgx_sql_entity_graph::RustToSqlMapping,
        > = lib
            .get("__pgx_sql_mappings".as_bytes())
            .expect("Couldn't call __pgx_sql_mappings");
        sql_mapping = sql_mappings_symbol();

        let symbol: libloading::os::unix::Symbol<
            unsafe extern "Rust" fn() -> eyre::Result<pgx_sql_entity_graph::ControlFile>,
        > = lib
            .get("__pgx_marker".as_bytes())
            .expect("Couldn't call __pgx_marker");
        let control_file_entity = pgx_sql_entity_graph::SqlGraphEntity::ExtensionRoot(
            symbol().expect("Failed to get control file information"),
        );
        entities.push(control_file_entity);

        for symbol_to_call in fns_to_call {
            let symbol: libloading::os::unix::Symbol<unsafe extern "Rust" fn() -> pgx_sql_entity_graph::SqlGraphEntity> =
                lib.get(symbol_to_call.as_bytes()).unwrap_or_else(|_|
                    panic!("Couldn't call {:#?}", symbol_to_call));
            let entity = symbol();
            entities.push(entity);
        }
    };

    let pgx_sql = pgx_sql_entity_graph::PgxSql::build(
        sql_mapping,
        entities.into_iter(),
        package_name.to_string(),
        versioned_so,
    )
    .wrap_err("SQL generation error")?;

    if let Some(out_path) = path {
        let out_path = out_path.as_ref();

        eprintln!(
            "{} SQL entities to {}",
            "     Writing".bold().green(),
            format_display_path(out_path)?.cyan()
        );

        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent).wrap_err("Could not create parent directory")?
        }
        pgx_sql
            .to_file(out_path)
            .wrap_err_with(|| eyre!("Could not write SQL to {}", out_path.display()))?;
    } else {
        eprintln!("{} SQL entities to {}", "     Writing".bold().green(), "/dev/stdout".cyan(),);
        pgx_sql
            .write(&mut std::io::stdout())
            .wrap_err_with(|| eyre!("Could not write SQL to stdout"))?;
    }

    if let Some(dot_path) = dot {
        let dot_path = dot_path.as_ref();
        tracing::info!(dot = %dot_path.display(), "Writing Graphviz DOT");
        pgx_sql.to_dot(dot_path)?;
    }
    Ok(())
}

#[tracing::instrument(level = "error", skip_all, fields(
    postmaster_path = %format_display_path(postmaster_path.as_ref())?,
    postmaster_stub_dir = %format_display_path(postmaster_stub_dir.as_ref())?,
))]
fn create_stub(
    postmaster_path: impl AsRef<Path>,
    postmaster_stub_dir: impl AsRef<Path>,
) -> eyre::Result<PathBuf> {
    let postmaster_path = postmaster_path.as_ref();
    let postmaster_stub_dir = postmaster_stub_dir.as_ref();

    let mut postmaster_stub_file = postmaster_stub_dir.to_path_buf();
    postmaster_stub_file.push("postmaster_stub.rs");

    let mut postmaster_hash_file = postmaster_stub_dir.to_path_buf();
    postmaster_hash_file.push("postmaster.hash");

    let mut postmaster_stub_built = postmaster_stub_dir.to_path_buf();
    postmaster_stub_built.push("postmaster_stub.so");

    let postmaster_bin_data =
        std::fs::read(postmaster_path).wrap_err("couldn't read postmaster")?;

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    postmaster_bin_data.hash(&mut hasher);
    let postmaster_bin_hash = hasher.finish().to_string().into_bytes();

    let postmaster_hash_data = std::fs::read(&postmaster_hash_file).ok();

    // Determine if we already built this stub.
    if let Some(postmaster_hash_data) = postmaster_hash_data {
        if postmaster_hash_data == postmaster_bin_hash && postmaster_stub_built.exists() {
            // We already built this and it's up to date.
            tracing::debug!(stub = %postmaster_stub_built.display(), "Existing stub for postmaster");
            return Ok(postmaster_stub_built);
        }
    }

    let postmaster_obj_file =
        object::File::parse(&*postmaster_bin_data).wrap_err("couldn't parse postmaster")?;
    let postmaster_exports = postmaster_obj_file
        .exports()
        .wrap_err("couldn't get exports from extension shared object")?;

    let mut symbols_to_stub = HashSet::new();
    for export in postmaster_exports {
        let name = std::str::from_utf8(export.name())?.to_string();
        #[cfg(target_os = "macos")]
        let name = {
            // Mac will prefix symbols with `_` automatically, so we remove it to avoid getting
            // two.
            let mut name = name;
            let rename = name.split_off(1);
            assert_eq!(name, "_");
            rename
        };
        symbols_to_stub.insert(name);
    }

    tracing::debug!("Creating stub of appropriate PostgreSQL symbols");
    PgxPgSysStub::from_symbols(&symbols_to_stub)?.write_to_file(&postmaster_stub_file)?;

    let mut so_rustc_invocation = Command::new("rustc");
    so_rustc_invocation.stderr(Stdio::inherit());

    if let Some(rustc_flags_str) = std::env::var("RUSTFLAGS").ok() {
        let rustc_flags = rustc_flags_str.split(' ').collect::<Vec<_>>();
        so_rustc_invocation.args(rustc_flags);
    }

    so_rustc_invocation.args([
        "--crate-type",
        "cdylib",
        "-o",
        postmaster_stub_built
            .to_str()
            .ok_or(eyre!("could not call postmaster_stub_built.to_str()"))?,
        postmaster_stub_file
            .to_str()
            .ok_or(eyre!("could not call postmaster_stub_file.to_str()"))?,
    ]);

    let so_rustc_invocation_str = format!("{:?}", so_rustc_invocation);
    tracing::debug!(command = %so_rustc_invocation_str, "Running");
    let output = so_rustc_invocation.output().wrap_err_with(|| {
        eyre!("could not invoke `rustc` on {}", &postmaster_stub_file.display())
    })?;

    let code = output.status.code().ok_or(eyre!("could not get status code of build"))?;
    tracing::trace!(status_code = %code, command = %so_rustc_invocation_str, "Finished");
    if code != 0 {
        return Err(eyre!("rustc exited with code {}", code));
    }

    std::fs::write(&postmaster_hash_file, postmaster_bin_hash)
        .wrap_err("could not write postmaster stub hash")?;

    Ok(postmaster_stub_built)
}
