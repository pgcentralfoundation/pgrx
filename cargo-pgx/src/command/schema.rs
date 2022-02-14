use crate::{
    command::get::{find_control_file, get_property},
    CommandExecute,
};
use colored::Colorize;
use eyre::{eyre, WrapErr};
use pgx_utils::{
    pg_config::{PgConfig, Pgx},
    sql_entity_graph::{SqlGraphEntity},
};
use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    os::unix::prelude::{MetadataExt, PermissionsExt},
    path::Path,
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
    /// Skip checking for required files
    #[clap(long, short)]
    manual: bool,
    /// Skip building the `sql-generator`, use an existing build
    #[clap(long, short)]
    skip_build: bool,
    /// Force the generation of default required files
    #[clap(long, short)]
    force_default: bool,
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
            &pg_config,
            self.release,
            &features,
            &out,
            self.dot,
            log_level,
            self.force_default,
            self.manual,
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
    pg_config: &PgConfig,
    is_release: bool,
    features: &clap_cargo::Features,
    path: impl AsRef<std::path::Path>,
    dot: Option<impl AsRef<std::path::Path>>,
    log_level: Option<String>,
    force_default: bool,
    manual: bool,
    skip_build: bool,
) -> eyre::Result<()> {
    let (control_file, _extname) = find_control_file()?;

    // If not manual, we should ensure a few files exist and are what is expected.
    if !manual {
        let cargo_toml = {
            let mut buf = String::default();
            let mut cargo_file =
                std::fs::File::open("Cargo.toml").expect(&format!("Could not open Cargo.toml"));
            cargo_file
                .read_to_string(&mut buf)
                .expect(&format!("Could not read Cargo.toml"));
            buf
        };
        let crate_name = cargo_toml
            .lines()
            .find(|line| line.starts_with("name"))
            .and_then(|line| line.split(" = ").last())
            .map(|line| line.trim_matches('\"').to_string())
            .map(|item| item.replace("-", "_"))
            .expect("Expected crate name");

        let expected_linker_script = include_str!("../templates/pgx-linker-script.sh");
        check_templated_file(
            ".cargo/pgx-linker-script.sh",
            expected_linker_script.to_string(),
            force_default,
        )?;
        check_permissions(".cargo/pgx-linker-script.sh", 0o755, force_default)?;
        let expected_cargo_config = include_str!("../templates/cargo_config");
        check_templated_file(
            ".cargo/config",
            expected_cargo_config.to_string(),
            force_default,
        )?;
    }

    if get_property("relocatable")? != Some("false".into()) {
        return Err(eyre!(
            "{}:  The `relocatable` property MUST be `false`.  Please update your .control file.",
            control_file.display()
        ));
    }

    let flags = std::env::var("PGX_BUILD_FLAGS").unwrap_or_default();

    if !skip_build {
        // First, build the SQL generator so we can get a look at the symbol table
        let mut command = Command::new("cargo");
        command.args(&["build"]);
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
            "{} SQL generator with features `{}`\n{}",
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

    // Inspect the symbol table for a list of `__pgx_internals` we should have the generator call
    let mut lib_so = pgx_utils::get_target_dir()?;
    lib_so.push(if is_release { "release" } else { "debug" });
    lib_so.push("libarrays.so");
    println!("{} SQL entities", " Discovering".bold().green(),);
    let dsym_path = lib_so.resolve_dsym();
    let buffer = ByteView::open(dsym_path.as_deref().unwrap_or(&lib_so))?;
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
    let mut num_funcs = 0;
    let mut num_types = 0;
    let mut num_enums = 0;
    let mut num_sqls = 0;
    let mut num_ords = 0;
    let mut num_hashes = 0;
    let mut num_aggregates = 0;
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

    unsafe {
        let lib = libloading::os::unix::Library::open(
            Some(&lib_so),
            libloading::os::unix::RTLD_LAZY,
        ).expect(&format!("Couldn't libload {}", lib_so.display()));

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

    // let pgx_sql = PgxSql::build(
    //     pgx::DEFAULT_TYPEID_SQL_MAPPING.clone().into_iter(),
    //     pgx::DEFAULT_SOURCE_ONLY_SQL_MAPPING.clone().into_iter(),
    //     entities.into_iter()).unwrap();

    // tracing::info!(path = %path.display(), "Writing SQL");
    // pgx_sql.to_file(path)?;
    // if let Some(dot_path) = dot {
    //     tracing::info!(dot = %dot_path.display(), "Writing Graphviz DOT");
    //     pgx_sql.to_dot(dot_path)?;
    // }
    Ok(())
}

/// Returns Ok(true) if something was created.
fn check_templated_file(
    path: impl AsRef<Path>,
    expected_content: String,
    overwrite: bool,
) -> Result<bool, std::io::Error> {
    let path = path.as_ref();
    let existing_contents = match File::open(&path) {
        Ok(mut file) => Some({
            let mut buf = String::default();
            file.read_to_string(&mut buf)?;
            Some(buf)
        }),
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                None
            } else {
                return Err(err);
            }
        }
    };

    match existing_contents {
        Some(contents) if contents == Some(expected_content.clone()) => Ok(false),
        Some(_content) => {
            if overwrite {
                println!(
                    "{} custom `{}` file due to `--force-default`",
                    " Overwriting".bold().yellow(),
                    path.display().to_string().bold().cyan()
                );
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                };
                let mut fd = File::create(path)?;
                fd.write_all(expected_content.as_bytes())?;
                Ok(true)
            } else {
                // Extension has a customized file, we shouldn't touch it or fail, but we should notify.
                println!(
                    "{} custom `{}` file (having trouble? `cargo pgx schema --help` details settings needed)",
                    "   Detecting".bold().green(),
                    path.display().to_string().bold().cyan()
                );
                Ok(false)
            }
        }
        None => {
            // The extension doesn't have the file! We'll create it with the expected content.
            println!(
                "{} required file `{}` for SQL bindings",
                "    Creating".bold().green(),
                path.display().to_string().bold().cyan()
            );
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            };
            let mut fd = File::create(path)?;
            fd.write_all(expected_content.as_bytes())?;
            Ok(true)
        }
    }
}

/// Returns Ok(true) if permissions where changed.
fn check_permissions(
    path: impl AsRef<Path>,
    expected_mode: u32,
    overwrite: bool,
) -> Result<bool, std::io::Error> {
    let file = File::open(&path)?;
    let metadata = file.metadata()?;
    if metadata.mode() == expected_mode {
        Ok(false)
    } else if overwrite {
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(expected_mode))?;
        Ok(true)
    } else {
        Ok(false)
    }
}