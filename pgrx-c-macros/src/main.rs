//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! The CLI exposes the same PostgreSQL resolution and compiler inspection used by library
//! clients. List shows PostgreSQL-owned function macros; analyze reports symbolic candidates
//! and structured skips; emit writes supported translations or a JSON report. Reports retain
//! the selected profile for auditing, diagnostics go to stderr, and broken output pipes are
//! treated as successful termination for ordinary shell pipelines.

use clap::{Args, Parser, Subcommand, ValueEnum};
use pgrx_c_macros::{
    AnalysisSession, AnalysisStatus, BindingCatalog, CompilationProfile, Diagnostic,
    EmissionStatus, Error, FrontendError, FrontendOutput, MacroAnalysis, MacroEmission,
    MacroScanner, PostgresConfig, PostgresError, emit_batch_with_bindings,
    postgres_function_macro_names, postgres_inline_function_names,
};
use serde::Serialize;
use std::collections::HashSet;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Top-level clap grammar shared by the inventory, analysis, and emission commands.
#[derive(Parser)]
#[command(
    version,
    about = "Discover, analyze, and emit C macros for a pgrx-managed PostgreSQL installation"
)]
struct Cli {
    /// The requested discovery, symbolic-analysis, or Rust-emission stage.
    #[command(subcommand)]
    command: Command,
}

/// Select the observable pipeline stage without giving discovery output the stronger guarantees of
/// emission.
#[derive(Subcommand)]
enum Command {
    /// List function-like macros from the selected PostgreSQL version's pgrx headers.
    List(
        /// Installation and inventory filters for raw function-macro discovery.
        ListArgs,
    ),
    /// Analyze PostgreSQL macro and inline invocation roots without claiming validated Rust support.
    Analyze(
        /// Installation, compiler overrides, and reporting options for symbolic candidates.
        AnalyzeArgs,
    ),
    /// Emit Rust macros for supported C expressions and statements; log skipped definitions.
    Emit(
        /// Installation, compiler overrides, and source/report options for standalone lowering.
        EmitArgs,
    ),
}

/// Select an installation and compiler context for symbolic analysis and structured skip reporting.
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

/// Select standalone Rust emission and its report format using the same inspected profile as
/// analysis.
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

/// Choose compilable Rust source or a machine-readable emission report.
#[derive(Clone, Copy, ValueEnum)]
enum EmissionFormat {
    /// Write generated macro_rules source for supported standalone translations.
    Rust,
    /// Write a machine-readable inventory or pipeline report with structured facts and skips.
    Json,
}

/// Include the selected C profile alongside emission results so generated text remains auditable.
#[derive(Serialize)]
struct EmissionReport<'a> {
    /// Resolved installation version associated with the report’s compiler profile.
    postgres_version: String,
    /// The inspected compiler flags and target facts under which these observations are valid.
    profile: &'a CompilationProfile,
    /// Selected per-macro results, retaining candidates or explicit refusals in deterministic
    /// output order.
    macros: Vec<MacroEmission>,
}

/// Choose readable candidate diagnostics or a machine-readable analysis report.
#[derive(Clone, Copy, ValueEnum)]
enum AnalysisFormat {
    /// Write readable analysis outcomes and physical source locations.
    Human,
    /// Write a machine-readable inventory or pipeline report with structured facts and skips.
    Json,
}

/// Include the selected C profile alongside symbolic analysis results and skip reasons.
#[derive(Serialize)]
struct AnalysisReport<'a> {
    /// Resolved installation version associated with the report’s compiler profile.
    postgres_version: String,
    /// The inspected compiler flags and target facts under which these observations are valid.
    profile: &'a CompilationProfile,
    /// Selected per-macro results, retaining candidates or explicit refusals in deterministic
    /// output order.
    macros: Vec<MacroAnalysis>,
}

/// Select PostgreSQL-owned function macro discovery and inventory output without preparing
/// expansions.
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

/// Choose the amount of inventory detail written by the list command.
#[derive(Clone, Copy, ValueEnum)]
enum Format {
    /// Write only selected discovered macro identifiers.
    Names,
    /// Write normalized #define text with the original macro signature and replacement tokens.
    Definitions,
    /// Write a machine-readable inventory or pipeline report with structured facts and skips.
    Json,
}

/// Preserve configuration, compiler, output, and serialization failures through CLI dispatch.
#[derive(Debug, thiserror::Error)]
enum CliError {
    /// Installation resolution or PostgreSQL header ownership failed.
    #[error(transparent)]
    Postgres(
        /// Original PostgreSQL resolution error.
        #[from]
        PostgresError,
    ),
    /// The underlying scanner could not produce a complete reliable inventory.
    #[error(transparent)]
    Discovery(
        /// Original scanner failure preserved through the frontend/CLI boundary.
        #[from]
        Error,
    ),
    /// Compiler inspection or a required probe failed.
    #[error(transparent)]
    Frontend(
        /// Original coherent-inspection or probe error.
        #[from]
        FrontendError,
    ),
    /// Writing reports or reading configuration failed at the filesystem/stream boundary.
    #[error(transparent)]
    Io(
        /// Original report-stream I/O error.
        #[from]
        io::Error,
    ),
    /// Report serialization failed, retaining possible broken-pipe context for shell termination.
    #[error(transparent)]
    Json(
        /// Original serialization error, including possible broken-pipe context.
        #[from]
        serde_json::Error,
    ),
}

/// Translate CLI completion and compiler diagnostics into shell exit status, accepting intentional
/// broken pipes.
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

/// Dispatch the parsed command while keeping argument parsing outside the library pipeline.
fn run(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Command::List(args) => list(args),
        Command::Analyze(args) => analyze_postgres(args),
        Command::Emit(args) => emit_postgres(args),
    }
}

/// Discover and filter PostgreSQL-owned function macros, then write the selected inventory format.
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

/// Resolve the installation, prepare a coherent session, and report candidates and skip reasons with
/// their profile.
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
    let (mut names, inline_names) = selected_invocation_names(&inspected, &root, &args.name)?;
    let session = AnalysisSession::prepare_with_inline_functions(
        &scanner,
        &inspected,
        &names,
        &[] as &[String],
        &inline_names,
    )?;
    names.extend(inline_names);
    names.sort();
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

/// Run supported standalone emission and send untranslated macro reasons to stderr or the JSON
/// report.
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
    let (mut names, inline_names) = selected_invocation_names(&inspected, &root, &args.name)?;
    let session = AnalysisSession::prepare_with_inline_functions(
        &scanner,
        &inspected,
        &names,
        &[] as &[String],
        &inline_names,
    )?;
    names.extend(inline_names);
    names.sort();
    let report = EmissionReport {
        postgres_version: postgres.pg_config().version().map_err(PostgresError::from)?,
        profile: inspected.profile(),
        macros: emit_batch_with_bindings(&session, &names, &BindingCatalog::default())
            .map_err(FrontendError::Output)?,
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
                "{emitted} emitted (runtime C expressions), {} skipped",
                report.macros.len() - emitted
            );
        }
    }
    output.flush()?;
    Ok(())
}

/// Apply exact CLI name filters after authoritative PostgreSQL source ownership filtering.
fn selected_invocation_names(
    inspected: &FrontendOutput,
    root: &Path,
    selected: &[String],
) -> Result<(Vec<String>, Vec<String>), PostgresError> {
    let selected: HashSet<_> = selected.iter().map(String::as_str).collect();
    let macros = postgres_function_macro_names(inspected, root)?
        .into_iter()
        .filter(|name| selected.is_empty() || selected.contains(name.as_str()))
        .collect();
    let inlines = postgres_inline_function_names(inspected, root)?
        .into_iter()
        .filter(|name| selected.is_empty() || selected.contains(name.as_str()))
        .collect();
    Ok((macros, inlines))
}

/// Render owned compiler diagnostics with physical locations when available.
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
