use crate::commands::get::find_control_file;
use crate::commands::get::get_property;
use crate::CommandExecute;
use colored::Colorize;
use pgx_utils::pg_config::PgConfig;
use pgx_utils::pg_config::Pgx;
use pgx_utils::{exit_with_error, handle_result};
use std::collections::HashSet;
use std::fs::File;
use std::os::unix::prelude::MetadataExt;
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;
use std::{
    io::{Read, Write},
    path::Path,
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
    /// Additional cargo features to activate (default is `--no-default-features`)
    #[clap(long)]
    features: Vec<String>,
    /// A path to output a produced SQL file (default is `sql/$EXTNAME-$VERSION.sql`)
    #[clap(long, short, parse(from_os_str))]
    out: Option<PathBuf>,
    /// A path to output a produced GraphViz DOT file
    #[clap(long, short, parse(from_os_str))]
    dot: Option<PathBuf>,
    /// Enable debug logging (`-vv` for trace)
    #[clap(long, short = 'v', parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Schema {
    fn execute(self) -> std::result::Result<(), std::io::Error> {
        let (_, extname) = crate::commands::get::find_control_file();
        let out = self.out.unwrap_or_else(|| {
            format!(
                "sql/{}-{}.sql",
                extname,
                crate::commands::install::get_version()
            )
            .into()
        });

        let log_level = if let Ok(log_level) = std::env::var("RUST_LOG") {
            Some(log_level)
        } else {
            match self.verbose {
                0 => None,
                1 => Some("debug".to_string()),
                _ => Some("trace".to_string()),
            }
        };

        let pg_config = match std::env::var("PGX_TEST_MODE_VERSION") {
            // for test mode, we want the pg_config specified in PGX_TEST_MODE_VERSION
            Ok(pgver) => match Pgx::from_config()?.get(&pgver) {
                Ok(pg_config) => pg_config.clone(),
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "PGX_TEST_MODE_VERSION does not contain a valid postgres version number",
                    ));
                }
            },

            // otherwise, the user just ran "cargo pgx install", and we use whatever "pg_config" is configured
            Err(_) => match self.pg_config {
                None => match self.pg_version {
                    None => PgConfig::from_path(),
                    Some(pgver) => Pgx::from_config()?.get(&pgver)?.clone(),
                },
                Some(config) => PgConfig::new(PathBuf::from(config)),
            },
        };

        generate_schema(
            &pg_config,
            self.release,
            &self.features,
            &out,
            self.dot,
            log_level,
            self.force_default,
            self.manual,
            self.skip_build,
        )
    }
}

pub(crate) fn generate_schema(
    pg_config: &PgConfig,
    is_release: bool,
    additional_features: &Vec<impl AsRef<str>>,
    path: impl AsRef<std::path::Path>,
    dot: Option<impl AsRef<std::path::Path>>,
    log_level: Option<String>,
    force_default: bool,
    manual: bool,
    skip_build: bool,
) -> Result<(), std::io::Error> {
    let additional_features = additional_features
        .iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>();
    let (control_file, _extname) = find_control_file();
    let major_version = pg_config.major_version()?;

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

        let expected_bin_source_content = format!(
            "\
            /* Auto-generated by pgx. You may edit this, or delete it to have a new one created. */\n\
            pgx::pg_binary_magic!({});\n\
        ",
            crate_name
        );
        check_templated_file(
            "src/bin/sql-generator.rs",
            expected_bin_source_content,
            force_default,
        )?;

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

    if get_property("relocatable") != Some("false".into()) {
        exit_with_error!(
            "{}:  The `relocatable` property MUST be `false`.  Please update your .control file.",
            control_file.display()
        )
    }

    let mut features =
        std::env::var("PGX_BUILD_FEATURES").unwrap_or(format!("pg{}", major_version));
    let flags = std::env::var("PGX_BUILD_FLAGS").unwrap_or_default();
    if !additional_features.is_empty() {
        use std::fmt::Write;
        let mut additional_features = additional_features.join(" ");
        let _ = write!(&mut additional_features, " {}", features);
        features = additional_features
    }

    if !skip_build {
        // First, build the SQL generator so we can get a look at the symbol table
        let mut command = Command::new("cargo");
        command.args(&["build", "--bin", "sql-generator"]);
        if is_release {
            command.arg("--release");
        }

        if let Some(log_level) = &log_level {
            command.env("RUST_LOG", log_level);
        }

        if !features.trim().is_empty() {
            command.arg("--features");
            command.arg(&features);
            command.arg("--no-default-features");
        }

        for arg in flags.split_ascii_whitespace() {
            command.arg(arg);
        }

        let command = command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        let command_str = format!("{:?}", command);
        println!(
            "{} SQL generator with features `{}`\n{}",
            "    Building".bold().green(),
            features,
            command_str
        );
        let status = handle_result!(
            command.status(),
            format!("failed to spawn cargo: {}", command_str)
        );
        if !status.success() {
            exit_with_error!("failed to build SQL generator");
        }
    }

    // Inspect the symbol table for a list of `__pgx_internals` we should have the generator call
    let mut sql_gen_path = pgx_utils::get_target_dir();
    sql_gen_path.push(if is_release { "release" } else { "debug" });
    sql_gen_path.push("sql-generator");
    println!("{} SQL entities", " Discovering".bold().green(),);
    let dsym_path = sql_gen_path.resolve_dsym();
    let buffer = ByteView::open(dsym_path.as_deref().unwrap_or(&sql_gen_path))?;
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
                            if name.starts_with("__pgx_internals") {
                                fns_to_call.insert(name);
                            }
                        }
                    }
                }
                SymbolIterator::MachO(iter) => {
                    for symbol in iter {
                        if let Some(name) = symbol.name {
                            if name.starts_with("__pgx_internals") {
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
        }
    }

    println!(
        "{} {} SQL entities: {} schemas ({} unique), {} functions, {} types, {} enums, {} sqls, {} ords, {} hashes",
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
    );

    // Now run the generator with the correct symbol table
    let mut command = Command::new(sql_gen_path);

    let path = path.as_ref();
    let _ = path.parent().map(|p| std::fs::create_dir_all(&p).unwrap());

    command.arg("--sql");
    command.arg(path);
    if let Some(dot) = dot {
        command.arg("--dot");
        command.arg(dot.as_ref());
    }
    command.env(
        "PGX_SQL_ENTITY_SYMBOLS",
        fns_to_call
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(" "),
    );

    let command = command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let command_str = format!("{:?}", command);
    println!("running SQL generator\n{}", command_str);
    let status = handle_result!(
        command.status(),
        format!("failed to spawn sql-generator: {}", command_str)
    );
    if !status.success() {
        exit_with_error!("failed to run SQL generator");
    }
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
