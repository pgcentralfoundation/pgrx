//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Discover the C macro definitions encountered by Clang while processing a file.
//!
//! Definitions retain preprocessing order, including definitions subsequently redefined or
//! undefined. Inactive conditional branches are excluded. The inventory owns its data and does
//! not borrow from Clang's translation unit. Token spellings are Clang's UTF-8 representations,
//! rather than a byte-for-byte copy of the source.
//! [`PostgresConfig::scan`] separates PostgreSQL function macros from conversion context;
//! [`MacroScanner::scan`] provides the raw inventory without ownership filtering.
//!
//! [`PostgresConfig::inspect`] also obtains the final active macro map, declaration catalog,
//! and target facts under the installation's recorded compiler flags. [`analyze`] identifies
//! supported expression constructs and records explicit reasons for other macros.
//! [`AnalysisSession`] expands a batch with the matched compiler while preserving the main
//! file's preprocessing context. [`generate_with_bindings`] reconciles those C declarations
//! with actual Rust binding storage, producing Rust macros and shared Rust/C adapters.
//! [`emit()`] is available without a binding catalog for expressions that need no such adapters.
//! Generated macros use the semantic support in `pgrx-pg-sys`, retaining C type identity,
//! argument occurrences, lazy branches, places and unevaluated operands. Native operations
//! require the caller's unsafe obligations and the binding generator's FFI guard; partly
//! initialized aggregate results remain in `MaybeUninit` storage. Unsupported profiles,
//! declarations and constructs are explicit skips. Emission is runtime-only and does not
//! itself establish differential validation against C for every possible invocation.

//! The crate owns the discovery-to-emission boundary used by pgrx-bindgen and the CLI.
//! Clang supplies active preprocessing state and C declarations; owned inventories let later
//! phases inspect those facts without keeping a translation unit alive. Sessions tie expansion
//! and analysis to that same input snapshot, and emission checks actual Rust binding storage
//! before producing macros and the adapters they need.

use clang::{Clang, EntityKind, EntityVisitResult, Index};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Resolve pgrx-managed installations and separate PostgreSQL-owned macros from retained expansion
/// context.
mod postgres;
/// Decode recorded GNU or MSVC build options before inspecting C semantics.
mod postgres_flags;
pub use postgres::{
    PostgresConfig, PostgresError, PostgresInventory, postgres_function_macro_names,
    postgres_inline_function_names, postgres_object_macro_names,
};
pub use postgres_flags::{lower_msvc_runtime_flags, postgres_clang_flags};
/// Own compiler profiles, preprocessing state, and declaration identities independently of Clang
/// handles.
mod model;
pub use model::*;
/// Build deterministic macro/constant reference graphs for ordering and refusal propagation.
mod dependencies;
pub use dependencies::{MacroDependencyGraph, MacroDependencyImpact};
/// Establish one agreed compiler environment and copied C catalog before expansion or analysis.
mod frontend;
pub use frontend::{
    FrontendError, c_header_path, compile_native_support, inspect, split_recorded_cflags,
};
/// Parse bounded source syntax while preserving formal holes, grouping, and structural operands.
mod syntax;
pub use syntax::{
    BinaryOperator, Expression, ExpressionKind, ExpressionNode, IntegerLiteral, IntegerSuffix,
    NodeId, OffsetComponent, OffsetRecord, Statement, StatementBody, StringPart, TokenRange,
    UnaryOperator,
};
/// Describe symbolic C types and invocation contracts before Rust lowering claims support.
mod analysis;
pub use analysis::*;

/// Serialize tests using libclang because its wrapper allows only one live runtime handle per
/// process.
#[cfg(test)]
pub(crate) static SCANNER_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
/// Use the matched compiler to preserve preprocessing semantics and surviving formal occurrences.
mod expansion;
pub use expansion::{
    ConstantFallback, ExpandedMacro, ExpansionBatch, ExpansionDependency, ExpansionLimits,
    ExpansionResult, ExpansionSkip, ExpansionSkipCode, ObjectIntegerConstants, ParameterOccurrence,
    prepare_expansions, prepare_expansions_with_limits, probe_integer_object_constants,
};
/// Tie expansion, constant proofs, and analysis to one verified inspection snapshot.
mod session;
pub use session::AnalysisSession;
/// Gate runtime helper compatibility and produce target guards and opaque pg-sys integer bridges.
mod support_generation;
pub use support_generation::*;
/// Lower analyzed candidates using verified binding storage and shared C-semantic runtime
/// capabilities.
mod emit;
pub use emit::*;
/// Record actual target bindgen storage independently of authoritative C type identities.
mod bindings;
pub use bindings::*;
/// Retain direct macro calls only after comparing independently expanded structures and C type facts.
mod delegation;
/// Lay out generated macro token trees while preserving source spellings and semantic structure.
mod formatting;
pub use formatting::format_rust_macros;

/// An owned record of the definitions and diagnostics encountered while processing a file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroInventory {
    /// Owned macro definitions/results retained in discovery or selected pipeline order.
    pub macros: Vec<MacroDefinition>,
    /// Owned compiler diagnostics associated with this discovery or tool phase.
    pub diagnostics: Vec<Diagnostic>,
}

/// A C macro definition, with its name and replacement tokens left unexpanded.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroDefinition {
    /// The original C identifier retained for reports, symbol lookup, and readable generated output.
    pub name: String,
    /// The semantic or structural category kept separate from representation and source spelling.
    pub kind: MacroKind,
    /// Owned physical diagnostic coordinates or an unresolved location when Clang supplied none.
    pub location: Option<SourceLocation>,
    /// Physical definition span; absent for definitions without a source file.
    pub provenance: Option<SourceSpan>,
    /// Tokens include the macro name and, for function-like macros, the parameter list.
    pub tokens: Vec<Token>,
    /// Whether libclang identifies the definition as compiler-provided rather than ordinary header
    /// source.
    pub builtin: bool,
    /// Whether this definition originates in the scanned file rather than an included file.
    pub main_file: bool,
}

/// Whether the macro accepts a parenthesized list of arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroKind {
    /// An unparameterized definition retained for listing/expansion context rather than
    /// function-macro emission.
    ObjectLike,
    /// A definition with a C formal list, eligible for public macro inventory and analysis.
    FunctionLike,
}

/// A physical source location, unaffected by `#line` directives.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Absolute physical source filename used for diagnostics and source ownership.
    pub file: PathBuf,
    /// One-based physical source line, unaffected by #line remapping.
    pub line: u32,
    /// One-based source column or current layout column, according to the enclosing representation.
    pub column: u32,
    /// Byte offset from the start of the file.
    pub offset: u32,
}

/// The absolute filename and inclusive, one-based physical lines of a macro's tokens.
///
/// The span starts at the macro name and ends at its last token. It includes line splices
/// and interior comments, but excludes trailing comments and empty continuation lines.
/// `#line` directives do not change these coordinates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpan {
    /// Absolute physical source filename used for diagnostics and source ownership.
    pub file: PathBuf,
    /// First physical line occupied by the macro definition’s source tokens.
    pub start_line: u32,
    /// Last physical line occupied by the macro definition’s source tokens.
    pub end_line: u32,
}

/// Copy physical Clang ranges into owned inclusive source spans.
impl SourceSpan {
    /// Convert Clang exclusive endpoints into an inclusive physical line span for auditing and module
    /// grouping.
    pub(crate) fn from_range(
        range: clang::source::SourceRange<'_>,
        directory: &Path,
    ) -> Result<Option<Self>, String> {
        let start = range.get_start().get_spelling_location();
        let end = range.get_end().get_spelling_location();
        let (Some(file), Some(end_file)) = (start.file, end.file) else {
            if start.file.is_none() && end.file.is_none() {
                return Ok(None);
            }
            return Err("source range has only one physical endpoint".into());
        };
        let path = file.get_path();
        if path != end_file.get_path() || end.offset <= start.offset {
            return Err("source range does not span one file".into());
        }
        // Clang's range end is exclusive. Locating its final byte handles a range
        // that ends at column one of the next line without including that line.
        let last = file.get_offset_location(end.offset - 1).get_spelling_location();
        if start.line == 0 || last.line < start.line {
            return Err("source range has invalid line numbers".into());
        }
        Ok(Some(Self {
            file: if path.is_absolute() { path } else { directory.join(path) },
            start_line: start.line,
            end_line: last.line,
        }))
    }
}

/// A token's spelling and lexical category, including comments.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    /// The semantic or structural category kept separate from representation and source spelling.
    pub kind: TokenKind,
    /// Original token/type spelling retained for readable source and diagnostic reconstruction.
    pub spelling: String,
}

/// Clang's lexical categories for the tokens in a macro definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenKind {
    /// Source commentary retained in original token coordinates but excluded from semantic parsing.
    Comment,
    /// An identifier/designator substitution that must retain C structural spelling.
    Identifier,
    /// A Clang keyword token; macro formals can still use its spelling as a substitution hole.
    Keyword,
    /// A source literal awaiting bounded syntax and target-dependent type selection.
    Literal,
    /// An operator or delimiter token used to preserve C grammar and grouping.
    Punctuation,
}

/// A compiler diagnostic copied out of the translation unit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Clang diagnostic severity used to distinguish reports from discovery failures.
    pub severity: DiagnosticSeverity,
    /// Human-readable detail explaining the compiler observation or unsupported construct.
    pub message: String,
    /// Owned physical diagnostic coordinates or an unresolved location when Clang supplied none.
    pub location: Option<SourceLocation>,
}

/// The severity assigned by Clang to a diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    /// A diagnostic ignored by Clang’s configured warning policy.
    Ignored,
    /// Supplementary compiler information retained with other discovery diagnostics.
    Note,
    /// A nonfatal compiler diagnostic that remains visible to the caller.
    Warning,
    /// A compiler failure that prevents accepting a complete discovery/proof result.
    Error,
    /// A compiler failure that invalidates the remainder of the parsing or proof pass.
    Fatal,
}

/// An error loading Clang, processing the input, or obtaining a complete inventory.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The libclang runtime could not be initialized for discovery.
    #[error("could not initialize libclang: {0}")]
    Clang(
        /// Library-loading detail returned by the libclang wrapper.
        String,
    ),
    /// Scanner arguments or paths failed validation before parsing.
    #[error("invalid scanner input: {0}")]
    InvalidInput(
        /// The invalid scanner path or argument explanation.
        String,
    ),
    /// The selected C input could not be parsed into a translation unit.
    #[error("could not parse {}: {source}", header.display())]
    Parse {
        /// The original main-file spelling that controls preprocessing and quoted include lookup.
        header: PathBuf,
        /// The underlying I/O or parsing failure retained for actionable diagnostics.
        source: clang::SourceError,
    },
    /// Clang reported errors, with owned diagnostics retained for the caller.
    #[error("Clang reported errors while processing the input")]
    Diagnostics(
        /// Owned Clang diagnostics retained instead of exposing a partial inventory.
        Vec<Diagnostic>,
    ),
    /// A discovered preprocessing entity lacked a complete owned definition.
    #[error("incomplete Clang macro definition: {0}")]
    InvalidDefinition(
        /// The missing source/token fact that prevented a complete macro record.
        String,
    ),
}

/// A reusable scanner that owns Clang's runtime handle.
///
/// The `clang` wrapper permits one live handle per process and keeps it on the thread that
/// created it. This type preserves those restrictions. Parsing, token copying, and translation
/// unit destruction all finish within [`Self::scan`], before the runtime handle can be dropped.
///
/// An existing `clang-sys` runtime entry, such as bindgen's, is reused during discovery. The
/// runtime stays loaded on this thread after the scanner is dropped, so Clang objects created
/// by bindgen during the scanner's lifetime also keep access to their library.
#[derive(Debug)]
pub struct MacroScanner {
    /// The live thread-bound runtime handle required while indexes and translation units are in use.
    clang: Clang,
    // Fields drop in declaration order: restore the runtime after Clang unloads it.
    /// Retained shared library restored after clang drops, preserving other users’ thread-local
    /// runtime.
    _restore_runtime: RestoreRuntime,
}

/// Retain the shared libclang runtime so scanner teardown restores the library used by existing
/// bindgen handles.
#[derive(Debug)]
struct RestoreRuntime(
    /// The shared runtime retained until scanner teardown restores libclang’s thread-local entry.
    Arc<clang_sys::SharedLibrary>,
);

/// Release resources owned by RestoreRuntime even when a compiler or proof phase exits early.
impl Drop for RestoreRuntime {
    /// Restore the shared thread-local runtime after the scanner handle releases its own Clang
    /// ownership.
    fn drop(&mut self) {
        clang_sys::set_library(Some(Arc::clone(&self.0)));
    }
}

/// Own the libclang parsing boundary so no translation-unit borrow escapes discovery/probe visitors.
impl MacroScanner {
    /// Load libclang using `LIBCLANG_PATH` and normal paths, retaining any existing runtime.
    pub fn new() -> Result<Self, Error> {
        let previous = clang_sys::get_library();
        let clang = Clang::new().map_err(Error::Clang)?;
        // Clang::new replaces TLS even when bindgen already owns a loaded library. All
        // existing translation units and indexes must keep using their original library.
        let library = previous
            .or_else(clang_sys::get_library)
            .expect("successful Clang::new installs the thread's runtime");
        clang_sys::set_library(Some(Arc::clone(&library)));
        Ok(Self { clang, _restore_runtime: RestoreRuntime(library) })
    }

    /// Process a header or source file as C, followed by the supplied Clang arguments.
    ///
    /// Arguments may override the default language and specify include paths, defines, and
    /// targets. Compiler-provided and command-line definitions are included. Errors and fatal
    /// diagnostics reject the scan; warnings and notes remain in a successful inventory.
    /// Paths must be UTF-8, and paths and arguments must not contain NUL bytes.
    /// Use the process working directory instead of Clang's `-working-directory` option,
    /// which would make relative filenames ambiguous when recording provenance.
    pub fn scan(&self, header: &Path, clang_args: &[String]) -> Result<MacroInventory, Error> {
        self.with_translation_unit(header, clang_args, None, |_, inventory| Ok(inventory))
    }

    /// Scan in-memory probe contents through the same inventory path as physical headers.
    pub(crate) fn scan_unsaved(
        &self,
        header: &Path,
        contents: &str,
        clang_args: &[String],
    ) -> Result<MacroInventory, Error> {
        self.with_translation_unit(header, clang_args, Some(contents), |_, inventory| Ok(inventory))
    }

    /// Parse a C input and finish owned inventory extraction before invoking a phase-specific
    /// translation-unit visitor.
    pub(crate) fn with_translation_unit<R>(
        &self,
        header: &Path,
        clang_args: &[String],
        contents: Option<&str>,
        callback: impl FnOnce(&clang::TranslationUnit<'_>, MacroInventory) -> Result<R, Error>,
    ) -> Result<R, Error> {
        self.with_parsed_translation_unit(
            header,
            clang_args,
            contents,
            |parser| {
                parser.detailed_preprocessing_record(true);
            },
            |unit, directory, diagnostics| {
                callback(unit, collect_macro_inventory(unit, directory, diagnostics)?)
            },
        )
    }

    /// Inspect typed declarations without recording or copying preprocessing entities.
    pub(crate) fn with_declarations<R>(
        &self,
        header: &Path,
        clang_args: &[String],
        contents: Option<&str>,
        callback: impl FnOnce(&clang::TranslationUnit<'_>) -> Result<R, Error>,
    ) -> Result<R, Error> {
        self.with_parsed_translation_unit(
            header,
            clang_args,
            contents,
            |_| {},
            |unit, _, _| callback(unit),
        )
    }

    /// Provide the bounded parsing boundary used by discovery and probes while all borrowed Clang
    /// handles remain live.
    fn with_parsed_translation_unit<R>(
        &self,
        header: &Path,
        clang_args: &[String],
        contents: Option<&str>,
        configure: impl FnOnce(&mut clang::Parser<'_>),
        callback: impl FnOnce(&clang::TranslationUnit<'_>, &Path, Vec<Diagnostic>) -> Result<R, Error>,
    ) -> Result<R, Error> {
        let path = header
            .to_str()
            .ok_or_else(|| Error::InvalidInput("header path is not UTF-8".into()))?;
        if path.contains('\0') {
            return Err(Error::InvalidInput("header path contains a NUL byte".into()));
        }
        if clang_args.iter().any(|argument| argument.contains('\0')) {
            return Err(Error::InvalidInput("Clang argument contains a NUL byte".into()));
        }
        if clang_args.iter().any(|argument| {
            argument == "-working-directory" || argument.starts_with("-working-directory=")
        }) {
            return Err(Error::InvalidInput(
                "Clang's -working-directory is unsupported for source provenance; set the process working directory instead".into(),
            ));
        }
        if clang_args.len() > i32::MAX as usize - 2 {
            return Err(Error::InvalidInput("too many Clang arguments".into()));
        }
        let directory = std::env::current_dir().map_err(|error| {
            Error::InvalidInput(format!("could not resolve the working directory: {error}"))
        })?;

        let arguments = ["-x", "c"]
            .into_iter()
            .chain(clang_args.iter().map(String::as_str))
            .collect::<Vec<_>>();
        let index = Index::new(&self.clang, false, false);
        let mut parser = index.parser(header);
        parser.arguments(&arguments).skip_function_bodies(true);
        configure(&mut parser);
        if let Some(contents) = contents {
            parser.unsaved(&[clang::Unsaved::new(header, contents)]);
        }
        let translation_unit =
            parser.parse().map_err(|source| Error::Parse { header: header.into(), source })?;
        let diagnostics = translation_unit
            .get_diagnostics()
            .into_iter()
            .map(|diagnostic| Diagnostic {
                severity: match diagnostic.get_severity() {
                    clang::diagnostic::Severity::Ignored => DiagnosticSeverity::Ignored,
                    clang::diagnostic::Severity::Note => DiagnosticSeverity::Note,
                    clang::diagnostic::Severity::Warning => DiagnosticSeverity::Warning,
                    clang::diagnostic::Severity::Error => DiagnosticSeverity::Error,
                    clang::diagnostic::Severity::Fatal => DiagnosticSeverity::Fatal,
                },
                message: diagnostic.get_text(),
                location: source_location(diagnostic.get_location()),
            })
            .collect::<Vec<_>>();
        if diagnostics.iter().any(|diagnostic| {
            matches!(diagnostic.severity, DiagnosticSeverity::Error | DiagnosticSeverity::Fatal)
        }) {
            return Err(Error::Diagnostics(diagnostics));
        }
        callback(&translation_unit, &directory, diagnostics)
    }
}

/// Copy macro definitions and diagnostics from preprocessing entities into data independent of the
/// translation unit.
fn collect_macro_inventory(
    translation_unit: &clang::TranslationUnit<'_>,
    directory: &Path,
    diagnostics: Vec<Diagnostic>,
) -> Result<MacroInventory, Error> {
    let mut macros = Vec::new();
    let mut error = None;
    translation_unit.get_entity().visit_children(|entity, _| {
        if entity.get_kind() != EntityKind::MacroDefinition {
            return EntityVisitResult::Continue;
        }
        let Some(name) = entity.get_name() else {
            error = Some(Error::InvalidDefinition("macro name is missing".into()));
            return EntityVisitResult::Break;
        };
        let Some(range) = entity.get_range() else {
            error = Some(Error::InvalidDefinition(format!("source range for {name} is missing")));
            return EntityVisitResult::Break;
        };
        let provenance = match SourceSpan::from_range(range, directory) {
            Ok(provenance) => provenance,
            Err(cause) => {
                error = Some(Error::InvalidDefinition(format!("{name}: {cause}")));
                return EntityVisitResult::Break;
            }
        };
        let tokens = range
            .tokenize()
            .into_iter()
            .map(|token| {
                let spelling = token.get_spelling();
                // Some Clang token spellings retain physical line continuations, including
                // a continuation between a macro name and its opening parenthesis.
                let spelling = if spelling.contains('\\')
                    && (spelling.contains('\n') || spelling.contains('\r'))
                {
                    spelling.replace("\\\r\n", "").replace("\\\n", "").replace("\\\r", "")
                } else {
                    spelling
                };
                Token {
                    kind: match token.get_kind() {
                        clang::token::TokenKind::Comment => TokenKind::Comment,
                        clang::token::TokenKind::Identifier => TokenKind::Identifier,
                        clang::token::TokenKind::Keyword => TokenKind::Keyword,
                        clang::token::TokenKind::Literal => TokenKind::Literal,
                        clang::token::TokenKind::Punctuation => TokenKind::Punctuation,
                    },
                    spelling,
                }
            })
            .collect::<Vec<_>>();
        // Clang distinguishes dynamic builtin macros from ordinary compiler predefines.
        // The latter originate in its synthetic buffer, which has no physical file.
        let builtin = entity.is_builtin_macro()
            || entity.get_location().is_some_and(|location| {
                location.get_spelling_location().file.is_none()
                    && location.get_presumed_location().0 == "<built-in>"
            });
        macros.push(MacroDefinition {
            name,
            kind: if entity.is_function_like_macro() {
                MacroKind::FunctionLike
            } else {
                MacroKind::ObjectLike
            },
            location: entity.get_location().and_then(source_location),
            provenance,
            tokens,
            builtin,
            main_file: entity.is_in_main_file(),
        });
        EntityVisitResult::Continue
    });
    if let Some(error) = error {
        return Err(error);
    }
    Ok(MacroInventory { macros, diagnostics })
}

/// Copy the physical spelling location used for diagnostics and provenance ownership checks.
fn source_location(location: clang::source::SourceLocation<'_>) -> Option<SourceLocation> {
    let location = location.get_spelling_location();
    Some(SourceLocation {
        file: location.file?.get_path(),
        line: location.line,
        column: location.column,
        offset: location.offset,
    })
}

/// Format a definition for inspection, normalizing whitespace between tokens.
///
/// Block comments remain intact. Line comments are omitted so they cannot hide later tokens
/// after a multiline definition is rendered. The inventory retains both kinds of comments.
impl fmt::Display for MacroDefinition {
    /// Render a normalized C #define for inspection reports and generated source documentation.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "#define {}", self.name)?;
        for (index, token) in self.tokens.iter().skip(1).enumerate() {
            if token.kind == TokenKind::Comment && token.spelling.starts_with("//") {
                continue;
            }
            if index != 0 || self.kind != MacroKind::FunctionLike || token.spelling != "(" {
                formatter.write_str(" ")?;
            }
            formatter.write_str(&token.spelling)?;
        }
        Ok(())
    }
}
