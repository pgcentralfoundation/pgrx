//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use clap::{Args, Parser, Subcommand, ValueEnum};
use pgrx_c_macros::{
    AnalysisSession, AnalysisStatus, BindingCatalog, CompilationProfile, Diagnostic,
    EmissionStatus, Error, FrontendError, FrontendOutput, MacroAnalysis, MacroEmission,
    MacroScanner, PostgresConfig, PostgresError, emit_batch_with_bindings,
    postgres_function_macro_names,
};
use serde::Serialize;
use std::collections::HashSet;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    version,
    about = "Discover, analyze, and emit C macros for a pgrx-managed PostgreSQL installation"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List function-like macros from the selected PostgreSQL version's pgrx headers.
    List(ListArgs),
    /// Analyze final active PostgreSQL macros without claiming validated Rust support.
    Analyze(AnalyzeArgs),
    /// Emit Rust macros for the supported C integer family; log skipped definitions.
    Emit(EmitArgs),
}

#[derive(Args)]
struct AnalyzeArgs {
    /// Configured PostgreSQL major version, such as 18 or pg18.
    pg_version: String,

    /// Override pgrx's version-specific wrapper header with a header or source file.
    header: Option<PathBuf>,

    /// Output a human-readable report or JSON.
    #[arg(long, value_enum, default_value_t = AnalysisFormat::Human)]
    format: AnalysisFormat,

    /// Select an exact macro name; may be repeated.
    #[arg(long)]
    name: Vec<String>,

    /// Compatible Clang executable to use for final preprocessing state.
    #[arg(long)]
    clang: Option<PathBuf>,

    /// Additional Clang arguments after --; these follow PostgreSQL's recorded flags.
    #[arg(last = true, allow_hyphen_values = true)]
    clang_args: Vec<String>,
}

#[derive(Args)]
struct EmitArgs {
    /// Configured PostgreSQL major version, such as 18 or pg18.
    pg_version: String,
    /// Override pgrx's version-specific wrapper header with a header or source file.
    header: Option<PathBuf>,
    /// Output Rust source or a JSON report including source and skip reasons.
    #[arg(long, value_enum, default_value_t = EmissionFormat::Rust)]
    format: EmissionFormat,
    /// Select an exact macro name; may be repeated.
    #[arg(long)]
    name: Vec<String>,
    /// Compatible Clang executable to use for final preprocessing state.
    #[arg(long)]
    clang: Option<PathBuf>,
    /// Additional Clang arguments after --; these follow PostgreSQL's recorded flags.
    #[arg(last = true, allow_hyphen_values = true)]
    clang_args: Vec<String>,
}

#[derive(Clone, Copy, ValueEnum)]
enum EmissionFormat {
    Rust,
    Json,
}

#[derive(Serialize)]
struct EmissionReport<'a> {
    postgres_version: String,
    profile: &'a CompilationProfile,
    macros: Vec<MacroEmission>,
}

#[derive(Clone, Copy, ValueEnum)]
enum AnalysisFormat {
    Human,
    Json,
}

#[derive(Serialize)]
struct AnalysisReport<'a> {
    postgres_version: String,
    profile: &'a CompilationProfile,
    macros: Vec<MacroAnalysis>,
}

#[derive(Args)]
struct ListArgs {
    /// Configured PostgreSQL major version, such as 18 or pg18.
    pg_version: String,

    /// Override pgrx's version-specific wrapper header with a header or source file.
    header: Option<PathBuf>,

    /// Output names, normalized C definitions, or a JSON inventory.
    #[arg(long, value_enum, default_value_t = Format::Definitions)]
    format: Format,

    /// Select an exact macro name; may be repeated.
    #[arg(long)]
    name: Vec<String>,

    /// Exclude definitions from included files and macros without a source file.
    #[arg(long)]
    main_file_only: bool,

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
    Frontend(#[from] FrontendError),
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
        | Err(CliError::Frontend(FrontendError::Discovery(Error::Diagnostics(diagnostics))))
        | Err(CliError::Postgres(PostgresError::Discovery(Error::Diagnostics(diagnostics))))
        | Err(CliError::Postgres(PostgresError::Frontend(FrontendError::Discovery(
            Error::Diagnostics(diagnostics),
        )))) => {
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
    match cli.command {
        Command::List(args) => list(args),
        Command::Analyze(args) => analyze_postgres(args),
        Command::Emit(args) => emit_postgres(args),
    }
}

fn list(args: ListArgs) -> Result<(), CliError> {
    let postgres = PostgresConfig::resolve(&args.pg_version)?;
    let scanner = MacroScanner::new()?;
    let mut inventory =
        postgres.scan(&scanner, args.header.as_deref(), &args.clang_args)?.inventory;
    for diagnostic in &inventory.diagnostics {
        print_diagnostic(diagnostic);
    }

    let names: HashSet<_> = args.name.iter().map(String::as_str).collect();
    inventory.macros.retain(|definition| {
        (names.is_empty() || names.contains(definition.name.as_str()))
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

fn analyze_postgres(args: AnalyzeArgs) -> Result<(), CliError> {
    let postgres = PostgresConfig::resolve(&args.pg_version)?;
    let scanner = MacroScanner::new()?;
    let inspected = postgres.inspect(
        &scanner,
        args.header.as_deref(),
        &args.clang_args,
        args.clang.as_deref(),
    )?;
    for diagnostic in &inspected.inventory().diagnostics {
        print_diagnostic(diagnostic);
    }
    let root = postgres.server_include_dir().canonicalize()?;
    let names = selected_macro_names(&inspected, &root, &args.name)?;
    let session = AnalysisSession::prepare(&scanner, &inspected, &names)?;
    let macros = names.iter().map(|name| session.analyze(name)).collect();
    let report = AnalysisReport {
        postgres_version: postgres.pg_config().version().map_err(PostgresError::from)?,
        profile: inspected.profile(),
        macros,
    };
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    match args.format {
        AnalysisFormat::Json => {
            serde_json::to_writer_pretty(&mut output, &report)?;
            writeln!(output)?;
        }
        AnalysisFormat::Human => {
            let candidates = report
                .macros
                .iter()
                .filter(|item| matches!(item.status, AnalysisStatus::Candidate))
                .count();
            writeln!(
                output,
                "{}; C target {}",
                report.postgres_version, report.profile.target.triple
            )?;
            writeln!(
                output,
                "{candidates} analyzed candidates (pending Rust validation), {} skipped",
                report.macros.len() - candidates
            )?;
            for item in &report.macros {
                match &item.status {
                    AnalysisStatus::Candidate => write!(output, "{}: candidate", item.name)?,
                    AnalysisStatus::Skipped { reason } => {
                        write!(output, "{}: skipped: {}", item.name, reason.message)?;
                    }
                }
                if let Some(span) = &item.provenance {
                    write!(
                        output,
                        " ({}:{}-{})",
                        span.file.display(),
                        span.start_line,
                        span.end_line
                    )?;
                }
                writeln!(output)?;
            }
        }
    }
    output.flush()?;
    Ok(())
}

fn emit_postgres(args: EmitArgs) -> Result<(), CliError> {
    let postgres = PostgresConfig::resolve(&args.pg_version)?;
    let scanner = MacroScanner::new()?;
    let inspected = postgres.inspect(
        &scanner,
        args.header.as_deref(),
        &args.clang_args,
        args.clang.as_deref(),
    )?;
    for diagnostic in &inspected.inventory().diagnostics {
        print_diagnostic(diagnostic);
    }
    let root = postgres.server_include_dir().canonicalize()?;
    let names = selected_macro_names(&inspected, &root, &args.name)?;
    let session = AnalysisSession::prepare(&scanner, &inspected, &names)?;
    let report = EmissionReport {
        postgres_version: postgres.pg_config().version().map_err(PostgresError::from)?,
        profile: inspected.profile(),
        macros: emit_batch_with_bindings(&session, &names, &BindingCatalog::default()),
    };
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    match args.format {
        EmissionFormat::Json => {
            serde_json::to_writer_pretty(&mut output, &report)?;
            writeln!(output)?;
        }
        EmissionFormat::Rust => {
            let mut emitted = 0;
            for item in &report.macros {
                match &item.status {
                    EmissionStatus::Emitted { rust, .. } => {
                        writeln!(output, "{rust}")?;
                        emitted += 1;
                    }
                    EmissionStatus::Skipped { reason } => {
                        eprintln!("{}: skipped: {}", item.analysis.name, reason.message);
                    }
                }
            }
            eprintln!(
                "{emitted} emitted (runtime integer family), {} skipped",
                report.macros.len() - emitted
            );
        }
    }
    output.flush()?;
    Ok(())
}

fn selected_macro_names(
    inspected: &FrontendOutput,
    root: &Path,
    selected: &[String],
) -> Result<Vec<String>, PostgresError> {
    let selected: HashSet<_> = selected.iter().map(String::as_str).collect();
    Ok(postgres_function_macro_names(inspected, root)?
        .into_iter()
        .filter(|name| selected.is_empty() || selected.contains(name.as_str()))
        .collect())
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
