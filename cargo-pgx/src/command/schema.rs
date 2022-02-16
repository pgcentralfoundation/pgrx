use crate::{
    command::get::{find_control_file, get_property},
    CommandExecute,
};
use colored::Colorize;
use eyre::{eyre, WrapErr};
use pgx_utils::{
    pg_config::{PgConfig, Pgx},
    sql_entity_graph::{SqlGraphEntity, PgxSql},
    pgx_pg_sys_stub::PgxPgSysStub,
};
use std::{
    collections::HashSet,
    path::PathBuf,
    process::{Command, Stdio},
};
use symbolic::{
    common::{ByteView, DSymPathExt},
    debuginfo::{Archive, SymbolIterator},
};

/// Generate extension schema files
///
/// The SQL generation process requires configuring a few settings in the crate.
/// Normally `cargo pgx schema --force-default` can set these automatically.
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Schema {
    /// Skip building the `sql-generator`, use an existing build
    #[clap(long, short)]
    skip_build: bool,
    /// Do you want to run against Postgres `pg10`, `pg11`, `pg12`, `pg13`, `pg14`?
    pg_version: Option<String>,
    /// Compile for release mode (default is debug)
    #[clap(env = "PROFILE", long, short)]
    release: bool,
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c', parse(from_os_str))]
    pg_config: Option<PathBuf>,
    #[clap(flatten)]
    features: clap_cargo::Features,
    /// A path to output a produced SQL file (default is `sql/$EXTNAME-$VERSION.sql`)
    #[clap(long, short, parse(from_os_str))]
    out: Option<PathBuf>,
    /// A path to output a produced GraphViz DOT file
    #[clap(long, short, parse(from_os_str))]
    dot: Option<PathBuf>,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Schema {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let (_, extname) = crate::command::get::find_control_file()?;
        let metadata = crate::metadata::metadata(&Default::default())?;
        crate::metadata::validate(&metadata)?;
        let manifest = crate::manifest::manifest(&metadata)?;

        let out = match self.out {
            Some(out) => out,
            None => format!(
                "sql/{}-{}.sql",
                extname,
                crate::command::install::get_version()?,
            )
            .into(),
        };

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

        let (pg_config, pg_version) = match self.pg_config {
            None => match self.pg_version {
                None => {
                    let pg_version = match self.pg_version {
                        Some(s) => s,
                        None => crate::manifest::default_pg_version(&manifest)
                            .ok_or(eyre!("No provided `pg$VERSION` flag."))?,
                    };
                    (Pgx::from_config()?.get(&pg_version)?.clone(), pg_version)
                }
                Some(pgver) => (Pgx::from_config()?.get(&pgver)?.clone(), pgver),
            },
            Some(config) => {
                let pg_config = PgConfig::new(PathBuf::from(config));
                let pg_version = format!("pg{}", pg_config.major_version()?);
                (pg_config, pg_version)
            }
        };

        let features = crate::manifest::features_for_version(self.features, &manifest, &pg_version);

        generate_schema(
            &manifest,
            &pg_config,
            self.release,
            &features,
            &out,
            self.dot,
            log_level,
            self.skip_build,
        )
    }
}

#[tracing::instrument(level = "error", skip_all, fields(
    pg_version = %pg_config.version()?,
    release = is_release,
    path,
    dot,
    features = ?features.features,
))]
pub(crate) fn generate_schema(
    manifest: &cargo_toml::Manifest,
    pg_config: &PgConfig,
    is_release: bool,
    features: &clap_cargo::Features,
    path: impl AsRef<std::path::Path>,
    dot: Option<impl AsRef<std::path::Path>>,
    log_level: Option<String>,
    skip_build: bool,
) -> eyre::Result<()> {
    let (control_file, _extname) = find_control_file()?;
    let package_name = &manifest.package
        .as_ref()
        .ok_or_else(|| eyre!("Could not find crate name in Cargo.toml."))?
        .name;

    if get_property("relocatable")? != Some("false".into()) {
        return Err(eyre!(
            "{}:  The `relocatable` property MUST be `false`.  Please update your .control file.",
            control_file.display()
        ));
    }

    let flags = std::env::var("PGX_BUILD_FLAGS").unwrap_or_default();

    
    let mut target_dir_with_profile = pgx_utils::get_target_dir()?;
    target_dir_with_profile.push(if is_release { "release" } else { "debug" });
    
    // Get the build plan so we can determine the correct `pgx_pg_sys` `OUT_DIR` to create a stub from.
    let mut command_build_plan = Command::new("cargo");
    command_build_plan.env("RUSTC_BOOTSTRAP", "1");
    command_build_plan.arg("build");
    command_build_plan.args(["-Z", "unstable-options", "--build-plan" ]);
    if is_release {
        command_build_plan.arg("--release");
    }
    let features_arg = features.features.join(" ");
    if !features_arg.trim().is_empty() {
        command_build_plan.arg("--features");
        command_build_plan.arg(&features_arg);
    }

    if features.no_default_features {
        command_build_plan.arg("--no-default-features");
    }
    if features.all_features {
        command_build_plan.arg("--all-features");
    }
    let build_plan_output = command_build_plan.output()?;

    let build_plan_bytes = build_plan_output.stdout;
    let build_plan: serde_json::Value = serde_json::from_slice(&build_plan_bytes)
        .wrap_err("Could not parse build plan.")?;
    let build_plan_invocations = build_plan.get("invocations")
        .ok_or_else(|| eyre!("Could not find `invocations` key in build plan."))?
        .as_array()
        .ok_or_else(|| eyre!("Could not parse `invocations` key in build plan as array."))?;
    let pgx_pg_sys_build_plan = build_plan_invocations.iter()
        .find(|invocation| {
            let package_name = invocation.get("package_name");
            let compile_mode = invocation.get("compile_mode");
            if let (Some(package_name), Some(compile_mode)) = (package_name, compile_mode) {
                package_name == "pgx-pg-sys" && compile_mode == "run-custom-build"
            } else {
                false
            }
        }).ok_or_else(|| eyre!("Could not find `pgx-pg-sys` in build plan."))?;
    let build_plan_out_dir_env = pgx_pg_sys_build_plan.get("env")
        .ok_or_else(|| eyre!("Could not find `env` key in `pgx_pg_sys` build plan."))?
        .as_object()
        .ok_or_else(|| eyre!("Could not parse `env` key in `pgx_pg_sys` build plan as object."))?;
    let pgx_pg_sys_out_dir = build_plan_out_dir_env.get("OUT_DIR")
        .ok_or_else(|| eyre!("Could not find `env` key in `pgx_pg_sys` build plan."))?
        .as_str()
        .ok_or_else(|| eyre!("Could not parse `env.OUT_DIR` key in `pgx_pg_sys` build plan as string."))?;
    let pgx_pg_sys_out_dir = PathBuf::from(pgx_pg_sys_out_dir);

    if !skip_build {
        // First, build the SQL generator so we can get a look at the symbol table
        let mut command = Command::new("cargo");
        command.arg("build");
        
        if is_release {
            command.arg("--release");
        }

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

        let command = command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        let command_str = format!("{:?}", command);
        println!(
            "{} for SQL generation with features `{}`\n{}",
            "    Building".bold().green(),
            features_arg,
            command_str
        );
        let status = command
            .status()
            .wrap_err_with(|| format!("failed to spawn cargo: {}", command_str))?;
        if !status.success() {
            return Err(eyre!("failed to build SQL generator"));
        }
    }

    let pg_version = pg_config.major_version()?;
    let pg_version_file_name = format!("pg{}.rs", pg_version);
    // Create stubbed `pgx_pg_sys` bindings for the generator to link with.
    let mut pgx_pg_sys_stub_file = target_dir_with_profile.clone();
    pgx_pg_sys_stub_file.push("pg_sys_stub");
    pgx_pg_sys_stub_file.push(&pg_version_file_name);
    
    let mut pgx_pg_sys_file = PathBuf::from(pgx_pg_sys_out_dir);
    pgx_pg_sys_file.push(&pg_version_file_name);

    PgxPgSysStub::from_file(&pgx_pg_sys_file)?
        .write_to_file(&pgx_pg_sys_stub_file)?;
    
    let mut pgx_pg_sys_stub_built = target_dir_with_profile.clone();
    pgx_pg_sys_stub_built.push("pg_sys_stub");
    pgx_pg_sys_stub_built.push("pgx_pg_sys_stub.so");
    
    Command::new("rustc")
        .args([
            "--crate-type", "cdylib",
            "-o", pgx_pg_sys_stub_built.to_str().unwrap(),
            pgx_pg_sys_stub_file.to_str().unwrap(),
        ]).output()
        .wrap_err_with(||
            eyre!("Could not invoke `rustc` on {}", &pgx_pg_sys_stub_file.display())
        )?;

    // Inspect the symbol table for a list of `__pgx_internals` we should have the generator call
    let mut lib_so = target_dir_with_profile.clone();
    #[cfg(target_os = "macos")]
    let so_extension = "dylib";
    #[cfg(not(target_os = "macos"))]
    let so_extension = "so";

    lib_so.push(&format!("lib{}.{}", package_name.replace("-", "_"), so_extension));

    println!("{} SQL entities", " Discovering".bold().green(),);
    let dsym_path = lib_so.resolve_dsym();
    let buffer = ByteView::open(dsym_path.as_deref().unwrap_or(&lib_so))
        .wrap_err_with(||
            eyre!("Could not get byte view into {}", &dsym_path.as_deref().unwrap_or(&lib_so).display())
        )?;
    let archive = Archive::parse(&buffer).expect("Could not parse archive");

    // Some users reported experiencing duplicate entries if we don't ensure `fns_to_call`
    // has unique entries.
    let mut fns_to_call = HashSet::new();
    for object in archive.objects() {
        match object {
            Ok(object) => match object.symbols() {
                SymbolIterator::Elf(iter) => {
                    for symbol in iter {
                        if let Some(name) = symbol.name {
                            if name.starts_with("__pgx_internals") || name.starts_with("__pgx_marker") {
                                fns_to_call.insert(name);
                            }
                        }
                    }
                }
                SymbolIterator::MachO(iter) => {
                    for symbol in iter {
                        if let Some(name) = symbol.name {
                            if name.starts_with("__pgx_internals") || name.starts_with("__pgx_marker") {
                                fns_to_call.insert(name);
                            }
                        }
                    }
                }
                _ => panic!("Unable to parse non-ELF or Mach0 symbols. (Please report this, we can  probably fix this!)"),
            },
            Err(e) => {
                panic!("Got error inspecting objects: {}", e);
            }
        }
    }
    let mut seen_schemas = Vec::new();
    let mut num_funcs = 0_usize;
    let mut num_types = 0_usize;
    let mut num_enums = 0_usize;
    let mut num_sqls = 0_usize;
    let mut num_ords = 0_usize;
    let mut num_hashes = 0_usize;
    let mut num_aggregates = 0_usize;
    for func in &fns_to_call {
        if func.starts_with("__pgx_internals_schema_") {
            let schema = func
                .split("_")
                .skip(5)
                .next()
                .expect("Schema extern name was not of expected format");
            seen_schemas.push(schema);
        } else if func.starts_with("__pgx_internals_fn_") {
            num_funcs += 1;
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

    println!(
        "{} {} SQL entities: {} schemas ({} unique), {} functions, {} types, {} enums, {} sqls, {} ords, {} hashes, {} aggregates",
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
    );

    let path = path.as_ref();
    let _ = path.parent().map(|p| std::fs::create_dir_all(&p).unwrap());

    tracing::info!(path = %path.display(), "Collecting {} SQL entities", fns_to_call.len());
    let mut entities = Vec::default();
    let typeid_sql_mapping;
    let source_only_sql_mapping;

    unsafe {
        let _pgx_pg_sys = libloading::os::unix::Library::open(
            Some(&pgx_pg_sys_stub_built),
            libloading::os::unix::RTLD_NOW | libloading::os::unix::RTLD_GLOBAL,
        ).expect(&format!("Couldn't libload {}", pgx_pg_sys_stub_built.display()));

        let lib = libloading::os::unix::Library::open(
            Some(&lib_so),
            libloading::os::unix::RTLD_LAZY,
        ).expect(&format!("Couldn't libload {}", lib_so.display()));

        let typeid_sql_mappings_symbol: libloading::os::unix::Symbol<
            unsafe extern fn() -> std::collections::HashSet<pgx_utils::sql_entity_graph::RustSqlMapping>
        > = lib.get("__pgx_typeid_sql_mappings".as_bytes()).expect(&format!("Couldn't call __pgx_typeid_sql_mappings"));
        typeid_sql_mapping = typeid_sql_mappings_symbol();
        let source_only_sql_mapping_symbol: libloading::os::unix::Symbol<
            unsafe extern fn() -> std::collections::HashSet<pgx_utils::sql_entity_graph::RustSourceOnlySqlMapping>
        > = lib.get("__pgx_source_only_sql_mappings".as_bytes()).expect(&format!("Couldn't call __pgx_source_only_sql_mappings"));
        source_only_sql_mapping = source_only_sql_mapping_symbol();

        let symbol: libloading::os::unix::Symbol<
            unsafe extern fn() -> SqlGraphEntity
        > = lib.get("__pgx_marker".as_bytes()).expect(&format!("Couldn't call __pgx_marker"));
        let control_file_entity = symbol();
        entities.push(
            control_file_entity
        );

        for symbol_to_call in fns_to_call {
            let symbol: libloading::os::unix::Symbol<
                unsafe extern fn() -> SqlGraphEntity
            > = lib.get(symbol_to_call.as_bytes()).expect(&format!("Couldn't call {:#?}", symbol_to_call));
            let entity = symbol();
            entities.push(entity);
        }
    };

    let pgx_sql = PgxSql::build(
        typeid_sql_mapping.clone().into_iter(),
        source_only_sql_mapping.clone().into_iter(),
        entities.into_iter()
    ).unwrap();

    tracing::info!(path = %path.display(), "Writing SQL");
    pgx_sql.to_file(path)
        .wrap_err_with(|| eyre!("Could not write SQL to {}", path.display()))?;

    if let Some(dot_path) = dot {
        let dot_path = dot_path.as_ref();
        tracing::info!(dot = %dot_path.display(), "Writing Graphviz DOT");
        pgx_sql.to_dot(dot_path)?;
    }
    Ok(())
}