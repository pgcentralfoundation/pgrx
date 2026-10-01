//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use clap::{Args, Parser, Subcommand, ValueEnum};
use pgrx_c_macros::{Diagnostic, Error, MacroKind, MacroScanner, PostgresConfig, PostgresError};
use std::collections::HashSet;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(version, about = "Discover C macros for a pgrx-managed PostgreSQL installation")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List the macro definitions encountered while processing a C header or source file.
    List(ListArgs),
}

#[derive(Args)]
struct ListArgs {
    /// Configured PostgreSQL major version, such as 18 or pg18.
    pg_version: String,

    /// Wrapper header or source file; defaults to the selected server's postgres.h.
    header: Option<PathBuf>,

    /// Output names, normalized C definitions, or a JSON inventory.
    #[arg(long, value_enum, default_value_t = Format::Definitions)]
    format: Format,

    /// Select an exact macro name; may be repeated.
    #[arg(long)]
    name: Vec<String>,

    /// List only function-like macros.
    #[arg(long, conflicts_with = "object_like")]
    function_like: bool,

    /// List only object-like macros.
    #[arg(long, conflicts_with = "function_like")]
    object_like: bool,

    /// Exclude definitions from included files and macros without a source file.
    #[arg(long)]
    main_file_only: bool,

    /// Include compiler-provided macro definitions.
    #[arg(long)]
    include_builtins: bool,

    /// Additional Clang arguments after --, such as -Iinclude, -DNAME, or --target=....
    #[arg(last = true, allow_hyphen_values = true)]
    clang_args: Vec<String>,
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Names,
    Definitions,
    Json,
}

#[derive(Debug, thiserror::Error)]
enum CliError {
    #[error(transparent)]
    Postgres(#[from] PostgresError),
    #[error(transparent)]
    Discovery(#[from] Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(CliError::Io(error)) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(CliError::Json(error)) if error.io_error_kind() == Some(io::ErrorKind::BrokenPipe) => {
            ExitCode::SUCCESS
        }
        Err(CliError::Discovery(Error::Diagnostics(diagnostics)))
        | Err(CliError::Postgres(PostgresError::Discovery(Error::Diagnostics(diagnostics)))) => {
            for diagnostic in &diagnostics {
                print_diagnostic(diagnostic);
            }
            ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<(), CliError> {
    let Command::List(args) = cli.command;
    let postgres = PostgresConfig::resolve(&args.pg_version)?;
    let scanner = MacroScanner::new()?;
    let mut inventory = postgres.scan(&scanner, args.header.as_deref(), &args.clang_args)?;
    for diagnostic in &inventory.diagnostics {
        print_diagnostic(diagnostic);
    }

    let names: HashSet<_> = args.name.iter().map(String::as_str).collect();
    inventory.macros.retain(|definition| {
        (args.include_builtins || !definition.builtin)
            && (names.is_empty() || names.contains(definition.name.as_str()))
            && (!args.function_like || definition.kind == MacroKind::FunctionLike)
            && (!args.object_like || definition.kind == MacroKind::ObjectLike)
            && (!args.main_file_only || definition.main_file)
    });
    // Stable sorting retains preprocessing order for repeated definitions of the same name.
    inventory.macros.sort_by(|left, right| left.name.cmp(&right.name));

    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    match args.format {
        Format::Names => {
            for definition in &inventory.macros {
                writeln!(output, "{}", definition.name)?;
            }
        }
        Format::Definitions => {
            for definition in &inventory.macros {
                writeln!(output, "{definition}")?;
            }
        }
        Format::Json => {
            serde_json::to_writer_pretty(&mut output, &inventory)?;
            writeln!(output)?;
        }
    }
    output.flush()?;
    Ok(())
}

fn print_diagnostic(diagnostic: &Diagnostic) {
    if let Some(location) = &diagnostic.location {
        eprintln!(
            "{}:{}:{}: {:?}: {}",
            location.file.display(),
            location.line,
            location.column,
            diagnostic.severity,
            diagnostic.message,
        );
    } else {
        eprintln!("{:?}: {}", diagnostic.severity, diagnostic.message);
    }
}
