//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Establish inline definitions that declaration-only parsing intentionally hides.

//! Declaration-only parsing omits function bodies to keep inspection affordable. Static inline
//! adapters still need proof that an original definition exists, so this module performs one
//! bounded reparse with bodies enabled. Only definitions whose linkage, storage class, and
//! complete signature match the first catalog are admitted to native support generation.
//!
//! The same parse records the preprocessing history used to retain a definition's body for
//! translation: the body's active tokens, and proof that every macro it names expands at the
//! end of the translation unit exactly as it did inside the definition. libclang records no
//! `#undef`, so one driver preprocessing pass reports them.
//!
//! `flavors` applies the same proof to the other build flavor's parse of a definition that
//! differs between the flavors. The driver cannot read that parse's files, which exist only
//! in memory, so the proof there has no report of undefinitions. [`Proved::offsets`] names
//! the bodies of this parse whose proof needs none either, so both flavors hold a body to
//! one standard.

use super::{FrontendError, type_info};
use crate::{
    DeclarationLinkage, FrontendOutput, InlineFunctionDefinition, MacroDefinition, MacroKind,
    MacroScanner, SourceSpan, TokenKind, splice_length, unspliced, without_splices,
};
use clang::source::File;
use clang::{Entity, EntityKind, EntityVisitResult, Index, Linkage, StorageClass, TypeKind};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Bound declaration or expression candidates before constructing a compiler proof batch.
const MAX_CANDIDATES: usize = 16_384;
/// Keep original definition documentation bounded independently of the Clang translation unit.
const MAX_DEFINITION_BYTES: usize = 64 * 1024;
/// Bound all retained inline definition text; unavailable text becomes an explicit emission refusal.
const MAX_SOURCE_BYTES: usize = 16 * 1024 * 1024;

/// A body token and its byte offset in the file that holds the definition.
pub(super) type Located = (u32, crate::Token);

/// What the definition parse proved.
pub(super) struct Proved {
    /// The static inline functions with a matching complete definition, and what was
    /// retained of each.
    pub(super) definitions: BTreeMap<String, Option<InlineFunctionDefinition>>,
    /// The offset of each token of a retained body, for the bodies whose proof holds
    /// without the driver's report of undefinitions.
    pub(super) offsets: BTreeMap<String, Vec<u32>>,
}

/// Reparse the same input once with function bodies enabled. Definition identity
/// and the full prototype must agree with the declaration-only catalog; seeing
/// braces in a token range is not sufficient evidence of a function definition.
pub(super) fn prove(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
) -> Result<Proved, FrontendError> {
    let mut proved = Proved { definitions: BTreeMap::new(), offsets: BTreeMap::new() };
    let candidates = frontend
        .declarations()
        .function_signatures
        .iter()
        .filter(|(_, info)| {
            info.is_static && info.is_inline && info.linkage == Some(DeclarationLinkage::Internal)
        })
        .map(|(name, _)| name.as_str())
        .collect::<BTreeSet<_>>();
    if candidates.is_empty() {
        return Ok(proved);
    }
    if candidates.len() > MAX_CANDIDATES {
        return Err(FrontendError::Output(
            "inline definitions exceed the 16384-function budget".into(),
        ));
    }
    let profile = frontend.profile();
    let index = Index::new(&scanner.clang, false, false);
    let arguments = ["-x".to_owned(), "c".to_owned()]
        .into_iter()
        .chain(profile.arguments.iter().cloned())
        .collect::<Vec<_>>();
    let mut parser = index.parser(&profile.header);
    // The preprocessing record supplies macro definitions, top-level expansions and skipped
    // conditional text for body retention.
    parser.arguments(&arguments).skip_function_bodies(false).detailed_preprocessing_record(true);
    let unit = parser
        .parse()
        .map_err(|source| crate::Error::Parse { header: profile.header.clone(), source })?;
    let errors = unit
        .get_diagnostics()
        .into_iter()
        .filter(|diagnostic| {
            matches!(
                diagnostic.get_severity(),
                clang::diagnostic::Severity::Error | clang::diagnostic::Severity::Fatal
            )
        })
        .map(|diagnostic| diagnostic.to_string())
        .collect::<Vec<_>>();
    if !errors.is_empty() {
        return Err(FrontendError::Environment(format!(
            "full inline definition parsing failed: {}",
            errors.join("\n")
        )));
    }
    let mut records = Records::default();
    records.files.extend(unit.get_file(&profile.header));
    let mut admitted = Vec::new();
    let mut seen = BTreeSet::new();
    unit.get_entity().visit_children(|entity, _| {
        if records.record(entity) || entity.get_kind() != EntityKind::FunctionDecl {
            return EntityVisitResult::Continue;
        }
        let Some(name) = entity.get_name().filter(|name| candidates.contains(name.as_str())) else {
            return EntityVisitResult::Continue;
        };
        // An entry exists only after a matching complete definition proof. Its
        // forward declarations share that same definition: charging/copying it
        // again could exhaust the budget and replace valid retained text with None.
        if seen.contains(&name) {
            return EntityVisitResult::Continue;
        }
        if !entity.is_inline_function()
            || entity.get_storage_class() != Some(StorageClass::Static)
            || entity.get_linkage() != Some(Linkage::Internal)
            || !(entity.is_definition() || entity.get_definition().is_some())
        {
            return EntityVisitResult::Continue;
        }
        let Some(ty) = entity.get_type() else { return EntityVisitResult::Continue };
        let expected = &frontend.declarations().function_signatures[&name].signature;
        let parameters = if ty.get_canonical_type().get_kind() == TypeKind::FunctionPrototype {
            ty.get_argument_types()
                .map(|parameters| parameters.into_iter().map(type_info).collect::<Vec<_>>())
        } else {
            None
        };
        if ty.get_result_type().map(type_info).as_ref() == Some(&expected.result)
            && parameters == expected.parameters
            && ty.is_variadic() == expected.variadic
            && ty.get_calling_convention().map(|convention| format!("{convention:?}"))
                == expected.calling_convention
        {
            seen.insert(name.clone());
            admitted.push((name, entity.get_definition().unwrap_or(entity)));
        }
        EntityVisitResult::Continue
    });
    // Only a macro defined again at one location asks where it was undefined; see
    // `MacroHistory::unchanged`. Clang 16 and older define `__LP64__` twice among an
    // AArch64 target's predefined macros, which have no location and so never ask.
    let undefined = if records.redefines() {
        let directives = super::preprocess(
            &profile.compiler.executable,
            &profile.header,
            &profile.arguments,
            &["-E", "-dD"],
        )?;
        Some(undefined_names(&directives.stdout))
    } else {
        None
    };
    let active =
        |name: &str| frontend.environment().active.get(name).map(|active| &active.definition);
    let history = records.history(&active, &profile.arguments, undefined);
    let mut sources = HashMap::new();
    let mut remaining = MAX_SOURCE_BYTES;
    // Clang names a file found through a relative search path relative to the
    // process working directory, which inspection recorded with the profile.
    let directory = std::env::current_dir().map_err(|error| {
        FrontendError::Environment(format!("could not check working directory: {error}"))
    })?;
    if directory != profile.inputs.current_directory {
        return Err(FrontendError::Environment(
            "working directory changed after inspection".into(),
        ));
    }
    for (name, definition) in admitted {
        let source = source(definition, &directory, &mut sources, &mut remaining, &history);
        let (source, offsets) = match source {
            Some((source, offsets)) => (Some(source), offsets),
            None => (None, None),
        };
        proved.offsets.extend(offsets.map(|offsets| (name.clone(), offsets)));
        proved.definitions.insert(name, source);
    }
    Ok(proved)
}

/// The body tokens of the static inline function `definition` as another build flavor's
/// parse read them, under the proofs that retain a body of the inspected parse. `bytes` are
/// the contents of the definition's file.
pub(super) fn flavor_body<'tu>(
    definition: Entity<'tu>,
    bytes: &[u8],
    history: &MacroHistory<'tu, '_>,
) -> Option<Vec<Located>> {
    let range = definition.get_range()?;
    let start = range.get_start().get_file_location();
    let end = range.get_end().get_file_location();
    let file = start.file.filter(|file| Some(*file) == end.file)?;
    let text = bytes.get(usize::try_from(start.offset).ok()?..usize::try_from(end.offset).ok()?)?;
    // As for a retained definition: bounded, and spelled by libclang without loss.
    if text.len() > MAX_DEFINITION_BYTES || std::str::from_utf8(text).is_err() {
        return None;
    }
    body_tokens(definition, file, bytes, history)
        .filter(|body| body.needs_no_undefinitions)
        .map(|body| body.tokens)
}

/// The names that `pop_macro` pragmas in these files and in the `-D` definitions among
/// `arguments` restore, or `None` when a file cannot be read, a pragma's operand is not a
/// plain string literal, or a text has a `??/` trigraph or a carriage return outside a CR-LF
/// line end.
///
/// libclang records neither pragma, so the files' text is searched, including text the
/// preprocessor skipped, comments and `_Pragma` strings. Clang writes each `-D` definition
/// into its predefines text, where a `_Pragma` in the replacement list pops a macro
/// wherever the definition is expanded, so those definitions are searched too. Line splices
/// are removed first, as Clang removes them before it reads a pragma. With trigraphs
/// enabled, `??/` is a backslash, and Clang also ends a line at a bare carriage return, so
/// such a text could splice a name in a way the search does not model. A pragma whose name
/// is formed by token pasting cannot be seen.
fn popped_names(
    files: impl IntoIterator<Item = PathBuf>,
    arguments: &[String],
) -> Option<HashSet<String>> {
    const POP: &[u8] = b"pop_macro";
    // A `-D` operand is joined to the option or follows it.
    let previous = std::iter::once(None).chain(arguments.iter().map(Some));
    let definitions = arguments.iter().zip(previous).filter_map(|(argument, previous)| {
        if previous.is_some_and(|previous| previous == "-D") {
            Some(argument.as_str())
        } else {
            argument.strip_prefix("-D")
        }
    });
    let texts = files
        .into_iter()
        .map(|file| std::fs::read(file).ok())
        .chain(definitions.map(|definition| Some(definition.as_bytes().to_vec())));
    let mut names = HashSet::new();
    for text in texts {
        let text = text?;
        if text.windows(3).any(|window| window == b"??/") || has_bare_carriage_return(&text) {
            return None;
        }
        let text = without_splices(&text);
        let mut rest = text.as_ref();
        while let Some(at) = rest.windows(POP.len()).position(|window| window == POP) {
            rest = &rest[at + POP.len()..];
            // `("NAME")`, or `(\"NAME\")` inside a `_Pragma` string.
            let operand = rest.trim_ascii_start().strip_prefix(b"(")?.trim_ascii_start();
            let operand = operand.strip_prefix(b"\\").unwrap_or(operand).strip_prefix(b"\"")?;
            let length = operand
                .iter()
                .take_while(|&&byte| byte.is_ascii_alphanumeric() || byte == b'_')
                .count();
            let after = &operand[length..];
            if length == 0 || !after.strip_prefix(b"\\").unwrap_or(after).starts_with(b"\"") {
                return None;
            }
            names.insert(String::from_utf8_lossy(&operand[..length]).into_owned());
        }
    }
    Some(names)
}

/// The macros that some `#undef` in `output` leaves undefined: those whose `#undef` line is
/// not followed, before anything else the preprocessor wrote, by a `#define` of the same
/// name. `output` is the driver's preprocessing output with `-dD`, which writes each
/// executed `#define` and `#undef` on a line of its own, in order, among the unit's tokens.
///
/// Only blank lines and line markers may separate the two directives, so no token, other
/// macro directive or pragma was read while the name was undefined. Clang writes a space
/// before a `#` token that would start a line, so only a directive starts with `#`.
fn undefined_names(output: &str) -> HashSet<String> {
    let marker = |line: &str| {
        line.is_empty()
            || line.strip_prefix("#line ").or_else(|| line.strip_prefix("# ")).is_some_and(|rest| {
                rest.trim_ascii_start().starts_with(|digit: char| digit.is_ascii_digit())
            })
    };
    let mut names = HashSet::new();
    let mut lines = output.lines().map(str::trim_ascii_end).peekable();
    while let Some(line) = lines.next() {
        let Some(name) = line.strip_prefix("#undef ").map(str::trim_ascii) else { continue };
        while lines.next_if(|line| marker(line)).is_some() {}
        let redefined = lines
            .peek()
            .and_then(|line| line.strip_prefix("#define "))
            .and_then(|definition| definition.strip_prefix(name))
            .is_some_and(|rest| rest.is_empty() || rest.starts_with([' ', '(']));
        if !redefined {
            names.insert(name.to_owned());
        }
    }
    names
}

/// Read physical original source once per header and retain only bounded definition text.
///
/// A header's bytes are read from its file, which need not be UTF-8 and may contain NUL
/// bytes. Definition text that is not UTF-8 is retained lossily, to document the native
/// call adapter. libclang also spells the body's tokens lossily, which could change a
/// literal, so such a definition has no body.
///
/// Headers are cached by Clang's file identity, so a definition is read from the file Clang
/// parsed however its span spells that file's path.
fn source<'tu>(
    definition: Entity<'tu>,
    directory: &Path,
    files: &mut HashMap<File<'tu>, Option<Vec<u8>>>,
    remaining: &mut usize,
    history: &MacroHistory<'tu, '_>,
) -> Option<(InlineFunctionDefinition, Option<Vec<u32>>)> {
    let range = definition.get_range()?;
    let provenance = SourceSpan::from_range(range, directory).ok()??;
    let start = range.get_start().get_file_location();
    let end = range.get_end().get_file_location();
    let file = start.file?;
    let size = usize::try_from(end.offset.checked_sub(start.offset)?).ok()?;
    let contents = if size <= MAX_DEFINITION_BYTES && size <= *remaining {
        files.entry(file).or_insert_with(|| std::fs::read(file.get_path()).ok()).as_deref()
    } else {
        None
    };
    let text = contents
        .and_then(|contents| {
            contents.get(usize::try_from(start.offset).ok()?..usize::try_from(end.offset).ok()?)
        })
        .map(String::from_utf8_lossy);
    let exact = matches!(text, Some(Cow::Borrowed(_)));
    // Replacement characters can make the text longer than its bytes.
    let source = text
        .filter(|text| text.len() <= MAX_DEFINITION_BYTES && text.len() <= *remaining)
        .map(|text| {
            *remaining -= text.len();
            text.into_owned()
        });
    let body = contents
        .filter(|_| exact && source.is_some())
        .and_then(|contents| body_tokens(definition, file, contents, history));
    let offsets = body
        .as_ref()
        .filter(|body| body.needs_no_undefinitions)
        .map(|body| body.tokens.iter().map(|(offset, _)| *offset).collect());
    let definition = InlineFunctionDefinition {
        provenance,
        parameters: parameter_names(definition),
        body: body.map(|body| body.tokens.into_iter().map(|(_, token)| token).collect()),
        source,
    };
    Some((definition, offsets))
}

/// A definition's parameter names in prototype order, without the names of unnamed ones.
pub(super) fn parameter_names(definition: Entity<'_>) -> Vec<Option<String>> {
    definition
        .get_arguments()
        .unwrap_or_default()
        .into_iter()
        .map(|parameter| parameter.get_name().filter(|name| !name.is_empty()))
        .collect()
}

/// The tokens the preprocessor read for a definition's body.
struct Body {
    /// Each token with its offset in the definition's file.
    tokens: Vec<Located>,
    /// Whether the proof that the tokens expand unchanged holds without a report of
    /// undefinitions, which only the inspected parse has.
    needs_no_undefinitions: bool,
}

/// Recover the tokens the preprocessor read for a definition's body, without its braces.
/// `bytes` are the contents of the definition's file.
///
/// Conditional directives, through the end of their logical lines, and the text they
/// skipped are removed; any other directive makes the body unavailable, as does a trigraph
/// or a carriage return that is not part of a CR-LF line end. Every remaining
/// token must belong to one of the compiler's statements or be a `;` between them, so a
/// statement that ends inside a macro argument cannot leave a fragment behind. The tokens
/// must expand at the end of the translation unit as they did in the definition.
///
/// Offsets, skipped ranges and expansion records describe the first entry of a file the
/// preprocessor entered more than once, so a definition in a later entry has no body.
fn body_tokens<'tu>(
    definition: Entity<'tu>,
    file: File<'tu>,
    bytes: &[u8],
    history: &MacroHistory<'tu, '_>,
) -> Option<Body> {
    let body = definition
        .get_children()
        .into_iter()
        .find(|child| child.get_kind() == EntityKind::CompoundStmt)?;
    let located = |at: clang::source::SourceLocation<'tu>| {
        let location = at.get_file_location();
        (location.file == Some(file)).then_some(location)
    };
    let all = body.get_range()?.tokenize();
    let (open, rest) = all.split_first()?;
    let (close, inner) = rest.split_last()?;
    if unspliced(open.get_spelling()) != "{" || unspliced(close.get_spelling()) != "}" {
        return None;
    }
    // Tokens are lexed in the entry that holds the definition.
    let opened = located(open.get_location())?;
    if file.get_offset_location(opened.offset) != open.get_location() {
        return None;
    }
    // The line handling below ends lines only at newlines and splices them only with a
    // backslash. Clang also ends a line at a bare carriage return, and with trigraphs
    // enabled `??/` is a backslash and `??=` a `#`.
    let start = usize::try_from(opened.offset).ok()?;
    let end = usize::try_from(located(close.get_location())?.offset).ok()?;
    let text = bytes.get(start..end)?;
    if text.windows(3).any(|window| window[..2] == *b"??" && b"=(/)'<!>-".contains(&window[2]))
        || has_bare_carriage_return(text)
    {
        return None;
    }
    let skipped = file
        .get_skipped_ranges()
        .into_iter()
        .filter_map(|range| {
            Some(located(range.get_start())?.offset..located(range.get_end())?.offset)
        })
        .collect::<Vec<_>>();
    let mut active = Vec::new();
    // Tokens before this offset belong to the last directive. A skipped range ends with
    // the directive that closes it, not with that directive's logical line.
    let mut directive_end = 0;
    for (index, token) in inner.iter().enumerate() {
        let location = located(token.get_location())?;
        let offset = usize::try_from(location.offset).ok()?;
        if offset < directive_end {
            continue;
        }
        let skipped = skipped.iter().any(|range| range.contains(&location.offset));
        let spelling = unspliced(token.get_spelling());
        if matches!(spelling.as_str(), "#" | "%:") {
            // A conditional directive only selects tokens. Any other directive, or a `#`
            // this check cannot place at the start of a line, changes or hides
            // preprocessing state inside the body. A token that follows a splice begins
            // at the backslash, so this offset is where the directive's line starts.
            directive_end = logical_line_end(bytes, offset)?;
            let name = inner
                .get(index + 1)
                .filter(|next| {
                    located(next.get_location())
                        .is_some_and(|next| (next.offset as usize) < directive_end)
                })
                .map(|next| unspliced(next.get_spelling()));
            let conditional = starts_line(bytes, offset)
                && name.as_deref().is_some_and(|name| {
                    matches!(
                        name,
                        "if" | "ifdef"
                            | "ifndef"
                            | "elif"
                            | "elifdef"
                            | "elifndef"
                            | "else"
                            | "endif"
                    )
                });
            if !skipped && !conditional {
                return None;
            }
            continue;
        }
        if skipped {
            continue;
        }
        let kind = match token.get_kind() {
            clang::token::TokenKind::Comment => continue,
            clang::token::TokenKind::Identifier => TokenKind::Identifier,
            clang::token::TokenKind::Keyword => TokenKind::Keyword,
            clang::token::TokenKind::Literal => TokenKind::Literal,
            clang::token::TokenKind::Punctuation => TokenKind::Punctuation,
        };
        active.push((location.offset, crate::Token { kind, spelling }));
    }
    let mut reported = false;
    if !history.expands_unchanged(file, &active, &mut reported) {
        return None;
    }
    // Statement extents end just past their last token. Account for every active token.
    let mut position = 0;
    for statement in body.get_children() {
        let range = statement.get_range()?;
        let (start, end) = (located(range.get_start())?.offset, located(range.get_end())?.offset);
        while active.get(position).is_some_and(|(offset, _)| *offset < start) {
            if active[position].1.spelling != ";" {
                return None;
            }
            position += 1;
        }
        if active.get(position).is_none_or(|(offset, _)| *offset != start) {
            return None;
        }
        while active.get(position).is_some_and(|(offset, _)| *offset < end) {
            position += 1;
        }
    }
    if active[position..].iter().any(|(_, token)| token.spelling != ";") {
        return None;
    }
    Some(Body { tokens: active, needs_no_undefinitions: !reported })
}

/// Whether only blanks separate `offset` from the start of its logical line.
fn starts_line(contents: &[u8], offset: usize) -> bool {
    let before = contents[..offset].trim_ascii_end();
    let blanks = &contents[before.len()..offset];
    let mut newlines = blanks.iter().filter(|&&byte| byte == b'\n');
    if newlines.next().is_none() {
        return before.is_empty();
    }
    // A backslash before the first newline splices it away.
    !before.ends_with(b"\\") || newlines.next().is_some()
}

/// Whether `text` has a carriage return that does not start a CR-LF line end. Clang ends a
/// line there, but line handling here ends lines only at newlines.
fn has_bare_carriage_return(text: &[u8]) -> bool {
    text.iter()
        .enumerate()
        .any(|(index, &byte)| byte == b'\r' && text.get(index + 1) != Some(&b'\n'))
}

/// The offset of the newline that ends the logical line containing `start`, or the end
/// of the file.
///
/// Translation phases 2 and 3 run before directives: a line splice joins lines, and a
/// comment, which can span lines, is one space. A literal ends at its closing quote or at
/// the end of the line. An unterminated comment has no end.
fn logical_line_end(contents: &[u8], start: usize) -> Option<usize> {
    let mut at = start;
    // The bytes from `start` and their offsets, without line splices.
    let mut bytes = std::iter::from_fn(|| {
        while let Some(length) = contents.get(at..).and_then(splice_length) {
            at += length;
        }
        let byte = *contents.get(at)?;
        at += 1;
        Some((at - 1, byte))
    })
    .peekable();
    while let Some((offset, byte)) = bytes.next() {
        match byte {
            b'\n' => return Some(offset),
            b'/' if bytes.next_if(|&(_, next)| next == b'*').is_some() => {
                let mut star = false;
                loop {
                    let (_, byte) = bytes.next()?;
                    if star && byte == b'/' {
                        break;
                    }
                    star = byte == b'*';
                }
            }
            b'/' if bytes.next_if(|&(_, next)| next == b'/').is_some() => {
                return Some(
                    bytes.find(|&(_, byte)| byte == b'\n').map_or(contents.len(), |(end, _)| end),
                );
            }
            b'"' | b'\'' => {
                while let Some((end, next)) = bytes.next() {
                    match next {
                        b'\n' => return Some(end),
                        b'\\' => {
                            bytes.next();
                        }
                        quote if quote == byte => break,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Some(contents.len())
}

/// The preprocessing records of a parse with function bodies, in translation-unit order.
#[derive(Default)]
pub(super) struct Records<'tu> {
    /// The recorded definitions of each macro name.
    definitions: HashMap<String, Definitions<'tu>>,
    /// Each top-level macro expansion in the first entry of its file, by file and offset of
    /// its name, with the location of the definition it used.
    expansions: HashMap<File<'tu>, HashMap<u32, Recorded<'tu>>>,
    /// The files the unit included. The caller adds a main file that is on disk.
    files: HashSet<File<'tu>>,
    /// How many records came before the next one. libclang visits the whole preprocessing
    /// record, in translation-unit order, before declarations, so positions order macro
    /// definitions and expansions.
    position: usize,
}

impl<'tu> Records<'tu> {
    /// Record `entity` when it is an inclusion, macro definition or macro expansion, and
    /// report whether it is one.
    pub(super) fn record(&mut self, entity: Entity<'tu>) -> bool {
        self.position += 1;
        let position = self.position;
        match entity.get_kind() {
            EntityKind::InclusionDirective => self.files.extend(entity.get_file()),
            EntityKind::MacroDefinition => {
                if let Some(name) = entity.get_name() {
                    let location = entity.get_location().and_then(|at| {
                        let location = at.get_file_location();
                        Some((location.file?, location.offset))
                    });
                    let last = Recorded { position, location };
                    self.definitions
                        .entry(name)
                        .and_modify(|definitions| {
                            definitions.one_location &= definitions.last.location == location;
                            definitions.last = Recorded { position, location };
                        })
                        .or_insert(Definitions {
                            first: position,
                            one_location: location.is_some(),
                            last,
                        });
                }
            }
            EntityKind::MacroExpansion => {
                // A file and offset name a place in the file's first entry. Records from a
                // later entry of the same file would replace the first entry's records.
                let Some(at) = entity.get_location() else { return true };
                let location = at.get_file_location();
                if let Some(file) = location.file
                    && file.get_offset_location(location.offset) == at
                {
                    // The definition this expansion used, which push and pop pragmas can
                    // make differ from the last one recorded.
                    let definition = entity
                        .get_reference()
                        .and_then(|definition| definition.get_location())
                        .and_then(|at| {
                            let location = at.get_file_location();
                            Some((location.file?, location.offset))
                        });
                    self.expansions
                        .entry(file)
                        .or_default()
                        .insert(location.offset, Recorded { position, location: definition });
                }
            }
            _ => return false,
        }
        true
    }

    /// Whether some macro was defined again at the location of its first definition. Only
    /// such a macro asks where it was undefined; see [`MacroHistory::unchanged`].
    fn redefines(&self) -> bool {
        self.definitions.values().any(|definitions| {
            definitions.one_location && definitions.first != definitions.last.position
        })
    }

    /// The history of a parse whose final macro definitions `active` gives and whose
    /// command line was `arguments`. `undefined` names the macros that an `#undef` left
    /// undefined while the preprocessor read something else, when the driver reported them.
    pub(super) fn history<'a>(
        self,
        active: &'a dyn Fn(&str) -> Option<&'a MacroDefinition>,
        arguments: &[String],
        undefined: Option<HashSet<String>>,
    ) -> MacroHistory<'tu, 'a> {
        MacroHistory {
            active,
            popped: popped_names(self.files.iter().map(|file| file.get_path()), arguments),
            undefined,
            definitions: self.definitions,
            expansions: self.expansions,
        }
    }
}

/// Preprocessing history of a parse with function bodies, in translation-unit order.
pub(super) struct MacroHistory<'tu, 'a> {
    /// Final active definitions, which later expansion of a retained body uses.
    active: &'a dyn Fn(&str) -> Option<&'a MacroDefinition>,
    /// The recorded definitions of each macro name.
    definitions: HashMap<String, Definitions<'tu>>,
    /// Each top-level macro expansion in the first entry of its file, by file and offset of
    /// its name, with the location of the definition it used.
    expansions: HashMap<File<'tu>, HashMap<u32, Recorded<'tu>>>,
    /// Names a `pop_macro` pragma can restore, or `None` for every name.
    popped: Option<HashSet<String>>,
    /// Names that an `#undef` left undefined while the preprocessor read something else.
    /// `None` when no macro was defined twice at one location, since nothing then asks, and
    /// when no driver could report them, which refuses every macro that asks.
    undefined: Option<HashSet<String>>,
}

/// The definitions the preprocessor recorded for one macro name.
struct Definitions<'tu> {
    /// The position of the first.
    first: usize,
    /// The last, at its own location.
    last: Recorded<'tu>,
    /// Whether every one is at the same physical location, and so has the same parameters
    /// and replacement list.
    one_location: bool,
}

/// A preprocessing record's position in the translation unit and a macro definition's
/// physical location: the record's own for a definition, and the definition used by an
/// expansion.
struct Recorded<'tu> {
    /// Order among preprocessing records.
    position: usize,
    /// File and offset of the definition's name, when it has a physical source.
    location: Option<(File<'tu>, u32)>,
}

/// Prove that body macros keep their meaning at the end of the translation unit.
impl<'tu> MacroHistory<'tu, '_> {
    /// Whether body tokens at these offsets expand at the end of the translation unit exactly
    /// as they did in the definition.
    ///
    /// A name expanded in the definition, and every macro it reaches, must be unchanged since.
    /// A name active at the end but not expanded must lie in the arguments of an expanded
    /// function-like invocation, which dropped or stringified it, and be unchanged since that
    /// invocation; the unchanged invocation treats it the same way at the end. Any other name
    /// must never have been defined as a macro.
    ///
    /// `reported` is set when the proof needs the driver's report of undefinitions; see
    /// [`Self::unchanged`].
    fn expands_unchanged(&self, file: File<'tu>, tokens: &[Located], reported: &mut bool) -> bool {
        let expansions = self.expansions.get(&file);
        // Each open invocation records the parenthesis depth that closes it and its position.
        let mut invocations = Vec::<(usize, usize)>::new();
        let mut depth = 0_usize;
        let mut pending = None;
        for (offset, token) in tokens {
            let name = token.spelling.as_str();
            match name {
                "(" => {
                    depth += 1;
                    if let Some(position) = pending.take() {
                        invocations.push((depth, position));
                    }
                    continue;
                }
                ")" => {
                    if invocations.last().is_some_and(|&(closing, _)| closing == depth) {
                        invocations.pop();
                    }
                    depth = depth.saturating_sub(1);
                    continue;
                }
                _ => pending = None,
            }
            if !matches!(token.kind, TokenKind::Identifier | TokenKind::Keyword) {
                continue;
            }
            // `__FILE__` and `__LINE__` name the Rust invocation wherever they expand,
            // so they need not expand as they did in the definition.
            if crate::expansion::invocation_diagnostic(name) {
                continue;
            }
            if crate::expansion::dynamic_builtin(name) {
                return false;
            }
            match expansions.and_then(|expansions| expansions.get(offset)) {
                Some(expansion) => {
                    let position = expansion.position;
                    // `unchanged` ties the active definition to the last one recorded; the
                    // expansion must have used that same definition, which a pop formed by
                    // token pasting could otherwise make differ.
                    if !self.unchanged(name, position, reported, &mut HashSet::new())
                        || expansion.location.is_none()
                        || expansion.location != self.definitions[name].last.location
                    {
                        return false;
                    }
                    // `unchanged` found the active definition.
                    if (self.active)(name)
                        .is_some_and(|definition| definition.kind == MacroKind::FunctionLike)
                    {
                        pending = Some(position);
                    }
                }
                None if (self.active)(name).is_some() => {
                    let Some(&(_, position)) = invocations.last() else { return false };
                    if !self.unchanged(name, position, reported, &mut HashSet::new()) {
                        return false;
                    }
                }
                // Nested expansions are not recorded, so a name that was ever a macro
                // could have been expanded when a macro's result was rescanned.
                None if self.definitions.contains_key(name) => return false,
                None => {}
            }
        }
        true
    }

    /// Whether `name` and each macro its replacement list names have the definition at
    /// `position` that is active at the end of the translation unit.
    ///
    /// Undefinitions are not recorded. A macro whose active definition at the end is its
    /// last recorded definition, made before `position`, had that definition at `position`
    /// too: undefining it later would leave it undefined or redefined at the end. Only a
    /// `pop_macro` pragma can restore a definition without recording one, so a name that
    /// a pop can restore is refused. Push and pop pragmas are not recorded, so the active
    /// definition must also be that last definition by location. Token pasting can form
    /// names this check cannot see.
    ///
    /// A header that a system header includes again can define a macro again after
    /// `position`, as Clang's `<stddef.h>` defines `NULL` each time glibc asks for it. Such a
    /// macro also had its active definition at `position` when it was first defined before
    /// `position`, every definition of it is the one at that location, and no `#undef` left
    /// it undefined while the preprocessor read anything else.
    ///
    /// That last case needs the driver's report of undefinitions and sets `reported`. The
    /// parse of the other build flavor has no report, so it never admits the case.
    fn unchanged(
        &self,
        name: &str,
        position: usize,
        reported: &mut bool,
        visited: &mut HashSet<String>,
    ) -> bool {
        if !visited.insert(name.to_owned()) {
            return true;
        }
        if self.popped.as_ref().is_none_or(|popped| popped.contains(name)) {
            return false;
        }
        let (Some(definitions), Some(active)) = (self.definitions.get(name), (self.active)(name))
        else {
            return false;
        };
        let last = &definitions.last;
        let defined_last = last.location.is_some_and(|(file, offset)| {
            active.location.as_ref().is_some_and(|location| {
                location.offset == offset && location.file == file.get_path()
            })
        });
        let defined_again = last.position >= position
            && definitions.first < position
            && definitions.one_location
            && self.undefined.as_ref().is_some_and(|undefined| !undefined.contains(name));
        *reported |= defined_again;
        let defined_before = last.position < position || defined_again;
        if !defined_before || active.builtin || !defined_last {
            return false;
        }
        let tokens = &active.tokens;
        let (parameters, replacement) = match active.kind {
            MacroKind::ObjectLike => (HashSet::new(), tokens.get(1..).unwrap_or_default()),
            MacroKind::FunctionLike => {
                let Some(close) = tokens.iter().position(|token| token.spelling == ")") else {
                    return false;
                };
                (
                    tokens[..close]
                        .iter()
                        .skip(2)
                        .filter(|token| token.spelling != ",")
                        .map(|token| token.spelling.as_str())
                        .chain(["__VA_ARGS__", "__VA_OPT__"])
                        .collect(),
                    &tokens[close + 1..],
                )
            }
        };
        replacement.iter().all(|token| {
            let spelling = token.spelling.as_str();
            if matches!(spelling, "##" | "%:%:") {
                return false;
            }
            if !matches!(token.kind, TokenKind::Identifier | TokenKind::Keyword)
                || parameters.contains(spelling)
                || spelling == name
            {
                return true;
            }
            if crate::expansion::invocation_diagnostic(spelling) {
                return true;
            }
            if crate::expansion::dynamic_builtin(spelling) {
                return false;
            }
            !(self.definitions.contains_key(spelling) || (self.active)(spelling).is_some())
                || self.unchanged(spelling, position, reported, visited)
        })
    }
}

/// Check the pop pragma search on raw source text.
#[cfg(test)]
mod tests {
    use super::{popped_names, undefined_names};

    /// An `#undef` leaves its name undefined unless the next directive or token the
    /// preprocessor wrote defines that name again. Blank lines and line markers are neither.
    #[test]
    fn undefinitions_followed_by_their_definition_leave_no_name() {
        let output = "\
# 1 \"a.h\" 1 3
#undef A

# 89 \"a.h\" 3
#define A ((void*)0)
#undef B
#line 4 \"b.h\"
#define B(x) x
#undef C
#define C
#undef D
#define DD 1
#define D 1
#undef E
int e;
#define E 1
#undef F
#pragma pack(1)
#define F 1
#undef G
#undef G
#define G 1
#undef H
 # 1
#define H 1
#undef I";
        let names = undefined_names(output);
        assert_eq!(names, ["D", "E", "F", "G", "H", "I"].map(String::from).into());
    }

    /// Pop pragmas name what they restore, also inside `_Pragma` and in a `-D` definition;
    /// any other mention of the pragma name leaves every name unproved.
    #[test]
    fn pop_pragmas_name_restored_macros() {
        let directory = tempfile::tempdir().unwrap();
        let file = |name: &str, text: &str| {
            let path = directory.path().join(name);
            std::fs::write(&path, text).unwrap();
            path
        };
        let pragmas = file(
            "pragmas.h",
            "#pragma pop_macro(\"A\")\n#pragma pop_macro ( \"B_1\" )\n_Pragma(\"pop_macro(\\\"C\\\")\")\n",
        );
        let plain = file("plain.h", "#pragma push_macro(\"D\")\n");
        let names = popped_names([pragmas.clone(), plain.clone()], &[]).unwrap();
        assert_eq!(names, ["A", "B_1", "C"].map(String::from).into());
        // Only `-D` operands, joined or separate, reach the predefines text.
        let arguments = [
            "-DE=_Pragma(\"pop_macro(\\\"E\\\")\")",
            "-D",
            "F=_Pragma(\"pop_macro(\\\"F\\\")\")",
            "-Ipop_macro",
        ]
        .map(String::from);
        let names = popped_names([plain], &arguments).unwrap();
        assert_eq!(names, ["E", "F"].map(String::from).into());
        let comment = file("comment.h", "/* see pop_macro */\n");
        assert_eq!(popped_names([pragmas.clone(), comment], &[]), None);
        assert_eq!(popped_names([pragmas.clone()], &["-DG=pop_macro".into()]), None);
        assert_eq!(popped_names([pragmas, directory.path().join("missing.h")], &[]), None);
    }

    /// Line splices are removed before the search, in the pragma's name and its operand.
    /// A `??/` trigraph or a bare carriage return could splice in ways the search does not
    /// model, so it leaves every name unproved.
    #[test]
    fn pop_pragmas_are_found_across_splices() {
        let directory = tempfile::tempdir().unwrap();
        let file = |name: &str, text: &str| {
            let path = directory.path().join(name);
            std::fs::write(&path, text).unwrap();
            path
        };
        let spliced = file(
            "spliced.h",
            "#pragma pop_mac\\\nro(\"A\")\r\n#pragma pop_ma\\ \t\r\ncro(\"B\\\n_1\")\r\n",
        );
        let names = popped_names([spliced.clone()], &[]).unwrap();
        assert_eq!(names, ["A", "B_1"].map(String::from).into());
        let trigraph = file("trigraph.h", "/* ??/ */\n");
        assert_eq!(popped_names([spliced.clone(), trigraph], &[]), None);
        let carriage = file("carriage.h", "#define X 1\r#define Y 2\n");
        assert_eq!(popped_names([spliced, carriage], &[]), None);
    }
}
