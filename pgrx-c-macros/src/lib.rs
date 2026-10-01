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

use clang::{Clang, EntityKind, EntityVisitResult, Index};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

mod postgres;
pub use postgres::{PostgresConfig, PostgresError, PostgresInventory};

/// An owned record of the definitions and diagnostics encountered while processing a file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroInventory {
    pub macros: Vec<MacroDefinition>,
    pub diagnostics: Vec<Diagnostic>,
}

/// A C macro definition, with its name and replacement tokens left unexpanded.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroDefinition {
    pub name: String,
    pub kind: MacroKind,
    pub location: Option<SourceLocation>,
    /// Physical definition span; absent for definitions without a source file.
    pub provenance: Option<SourceSpan>,
    /// Tokens include the macro name and, for function-like macros, the parameter list.
    pub tokens: Vec<Token>,
    pub builtin: bool,
    /// Whether this definition originates in the scanned file rather than an included file.
    pub main_file: bool,
}

/// Whether the macro accepts a parenthesized list of arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroKind {
    ObjectLike,
    FunctionLike,
}

/// A physical source location, unaffected by `#line` directives.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: u32,
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
    pub file: PathBuf,
    pub start_line: u32,
    pub end_line: u32,
}

impl SourceSpan {
    fn from_range(
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
    pub kind: TokenKind,
    pub spelling: String,
}

/// Clang's lexical categories for the tokens in a macro definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenKind {
    Comment,
    Identifier,
    Keyword,
    Literal,
    Punctuation,
}

/// A compiler diagnostic copied out of the translation unit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub location: Option<SourceLocation>,
}

/// The severity assigned by Clang to a diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    Ignored,
    Note,
    Warning,
    Error,
    Fatal,
}

/// An error loading Clang, processing the input, or obtaining a complete inventory.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not initialize libclang: {0}")]
    Clang(String),
    #[error("invalid scanner input: {0}")]
    InvalidInput(String),
    #[error("could not parse {}: {source}", header.display())]
    Parse { header: PathBuf, source: clang::SourceError },
    #[error("Clang reported errors while processing the input")]
    Diagnostics(Vec<Diagnostic>),
    #[error("incomplete Clang macro definition: {0}")]
    InvalidDefinition(String),
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
    clang: Clang,
    // Fields drop in declaration order: restore the runtime after Clang unloads it.
    _restore_runtime: RestoreRuntime,
}

#[derive(Debug)]
struct RestoreRuntime(Arc<clang_sys::SharedLibrary>);

impl Drop for RestoreRuntime {
    fn drop(&mut self) {
        clang_sys::set_library(Some(Arc::clone(&self.0)));
    }
}

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
        let translation_unit = index
            .parser(header)
            .arguments(&arguments)
            .detailed_preprocessing_record(true)
            .skip_function_bodies(true)
            .parse()
            .map_err(|source| Error::Parse { header: header.into(), source })?;
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
                error =
                    Some(Error::InvalidDefinition(format!("source range for {name} is missing")));
                return EntityVisitResult::Break;
            };
            let provenance = match SourceSpan::from_range(range, &directory) {
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
}

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
