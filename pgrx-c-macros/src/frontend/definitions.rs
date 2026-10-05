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
//! end of the translation unit exactly as it did inside the definition.

use super::{FrontendError, type_info};
use crate::{
    ActiveMacro, DeclarationLinkage, FrontendOutput, InlineFunctionDefinition, MacroKind,
    MacroScanner, SourceSpan, TokenKind,
};
use clang::source::File;
use clang::{Entity, EntityKind, EntityVisitResult, Index, Linkage, StorageClass, TypeKind};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Bound declaration or expression candidates before constructing a compiler proof batch.
const MAX_CANDIDATES: usize = 16_384;
/// Keep original definition documentation bounded independently of the Clang translation unit.
const MAX_DEFINITION_BYTES: usize = 64 * 1024;
/// Bound all retained inline definition text; unavailable text becomes an explicit emission refusal.
const MAX_SOURCE_BYTES: usize = 16 * 1024 * 1024;

/// Reparse the same input once with function bodies enabled. Definition identity
/// and the full prototype must agree with the declaration-only catalog; seeing
/// braces in a token range is not sufficient evidence of a function definition.
pub(super) fn prove(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
) -> Result<BTreeMap<String, Option<InlineFunctionDefinition>>, FrontendError> {
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
        return Ok(BTreeMap::new());
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
    let mut history = MacroHistory {
        active: &frontend.environment().active,
        last_definition: HashMap::new(),
        expansions: HashMap::new(),
    };
    let mut admitted = Vec::new();
    let mut seen = BTreeSet::new();
    // libclang visits the whole preprocessing record, in translation-unit order, before
    // declarations, so positions order macro definitions and expansions only.
    let mut position = 0_usize;
    unit.get_entity().visit_children(|entity, _| {
        position += 1;
        match entity.get_kind() {
            EntityKind::MacroDefinition => {
                if let Some(name) = entity.get_name() {
                    history.last_definition.insert(name, position);
                }
                return EntityVisitResult::Continue;
            }
            EntityKind::MacroExpansion => {
                if let Some(location) = entity.get_location().map(|at| at.get_spelling_location())
                    && let Some(file) = location.file
                {
                    history.expansions.entry(file).or_default().insert(location.offset, position);
                }
                return EntityVisitResult::Continue;
            }
            EntityKind::FunctionDecl => {}
            _ => return EntityVisitResult::Continue,
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
    let mut definitions = BTreeMap::new();
    let mut files = BTreeMap::new();
    let mut remaining = MAX_SOURCE_BYTES;
    let directory = profile.header.parent().unwrap_or_else(|| Path::new("."));
    for (name, definition) in admitted {
        let source = source(definition, directory, &mut files, &mut remaining, &history);
        definitions.insert(name, source);
    }
    Ok(definitions)
}

/// Copy physical original source once per header and retain only bounded definition text.
fn source<'tu>(
    definition: Entity<'tu>,
    directory: &Path,
    files: &mut BTreeMap<PathBuf, Option<String>>,
    remaining: &mut usize,
    history: &MacroHistory<'tu, '_>,
) -> Option<InlineFunctionDefinition> {
    let range = definition.get_range()?;
    let provenance = SourceSpan::from_range(range, directory).ok()??;
    let start = range.get_start().get_spelling_location();
    let end = range.get_end().get_spelling_location();
    let file = start.file?;
    let size = usize::try_from(end.offset.checked_sub(start.offset)?).ok()?;
    let source = if size <= MAX_DEFINITION_BYTES && size <= *remaining {
        let contents = files.entry(provenance.file.clone()).or_insert_with(|| file.get_contents());
        let text = contents.as_ref().and_then(|contents| {
            contents.get(usize::try_from(start.offset).ok()?..usize::try_from(end.offset).ok()?)
        });
        text.map(|text| {
            *remaining -= size;
            text.to_owned()
        })
    } else {
        None
    };
    Some(InlineFunctionDefinition {
        provenance,
        parameters: definition
            .get_arguments()
            .unwrap_or_default()
            .into_iter()
            .map(|parameter| parameter.get_name().filter(|name| !name.is_empty()))
            .collect(),
        body: source.as_ref().and_then(|_| body_tokens(definition, file, history)),
        source,
    })
}

/// Recover the tokens the preprocessor read for a definition's body, without its braces.
///
/// Conditional directives and the text they skipped are removed; any other directive makes
/// the body unavailable. Every remaining token must belong to one of the compiler's
/// statements or be a `;` between them, so a statement that ends inside a macro argument
/// cannot leave a fragment behind. The tokens must expand at the end of the translation
/// unit as they did in the definition.
fn body_tokens<'tu>(
    definition: Entity<'tu>,
    file: File<'tu>,
    history: &MacroHistory<'tu, '_>,
) -> Option<Vec<crate::Token>> {
    let body = definition
        .get_children()
        .into_iter()
        .find(|child| child.get_kind() == EntityKind::CompoundStmt)?;
    let located = |at: clang::source::SourceLocation<'tu>| {
        let location = at.get_spelling_location();
        (location.file == Some(file)).then_some(location)
    };
    let all = body.get_range()?.tokenize();
    let (open, rest) = all.split_first()?;
    let (close, inner) = rest.split_last()?;
    if open.get_spelling() != "{" || close.get_spelling() != "}" {
        return None;
    }
    let skipped = file
        .get_skipped_ranges()
        .into_iter()
        .filter_map(|range| {
            Some(located(range.get_start())?.offset..located(range.get_end())?.offset)
        })
        .collect::<Vec<_>>();
    let contents = file.get_contents()?;
    let mut active = Vec::new();
    let mut directive_line = None;
    for (index, token) in inner.iter().enumerate() {
        let location = located(token.get_location())?;
        let skipped = skipped.iter().any(|range| range.contains(&location.offset));
        let spelling = token.get_spelling();
        if spelling == "#" {
            directive_line = Some(location.line);
            // A directive other than a conditional, or one continued onto another line,
            // changes or hides preprocessing state inside the body.
            let name = inner.get(index + 1).map(|next| next.get_spelling());
            let line_end =
                contents[location.offset as usize..].find('\n').unwrap_or(contents.len());
            let continued =
                contents[location.offset as usize..][..line_end].trim_end().ends_with('\\');
            if !skipped
                && (continued
                    || !name.as_deref().is_some_and(|name| {
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
                    }))
            {
                return None;
            }
            continue;
        }
        if directive_line == Some(location.line) || skipped {
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
    if !history.expands_unchanged(file, &active) {
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
    Some(active.into_iter().map(|(_, token)| token).collect())
}

/// Preprocessing history of the definition parse, in translation-unit order.
struct MacroHistory<'tu, 'a> {
    /// Final active definitions, which later expansion of a retained body uses.
    active: &'a BTreeMap<String, ActiveMacro>,
    /// Position of the last definition of each macro name.
    last_definition: HashMap<String, usize>,
    /// Position of each top-level macro expansion, by file and offset of its name.
    expansions: HashMap<File<'tu>, HashMap<u32, usize>>,
}

/// Prove that body macros keep their meaning at the end of the translation unit.
impl<'tu> MacroHistory<'tu, '_> {
    /// Whether body tokens at these offsets expand at the end of the translation unit exactly
    /// as they did in the definition.
    ///
    /// A name expanded in the definition, and every macro it reaches, must be unchanged since.
    /// A name active at the end but not expanded must lie in the arguments of an expanded
    /// function-like invocation, which dropped or stringified it, and be unchanged since that
    /// invocation; the unchanged invocation treats it the same way at the end.
    fn expands_unchanged(&self, file: File<'tu>, tokens: &[(u32, crate::Token)]) -> bool {
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
            if crate::expansion::dynamic_builtin(name) {
                return false;
            }
            match expansions.and_then(|expansions| expansions.get(offset)) {
                Some(&position) => {
                    if !self.unchanged(name, position, &mut HashSet::new()) {
                        return false;
                    }
                    if self.active[name].definition.kind == MacroKind::FunctionLike {
                        pending = Some(position);
                    }
                }
                None if self.active.contains_key(name) => {
                    let Some(&(_, position)) = invocations.last() else { return false };
                    if !self.unchanged(name, position, &mut HashSet::new()) {
                        return false;
                    }
                }
                None => {}
            }
        }
        true
    }

    /// Whether `name` and each macro its replacement list names have the definition at
    /// `position` that is active at the end of the translation unit.
    ///
    /// Undefinitions are not recorded. A macro that is active at the end and has no
    /// definition after `position` was therefore defined by its last definition at
    /// `position` too. Token pasting can form names this check cannot see.
    fn unchanged(&self, name: &str, position: usize, visited: &mut HashSet<String>) -> bool {
        if !visited.insert(name.to_owned()) {
            return true;
        }
        let (Some(&last), Some(active)) = (self.last_definition.get(name), self.active.get(name))
        else {
            return false;
        };
        if last >= position || active.definition.builtin {
            return false;
        }
        let tokens = &active.definition.tokens;
        let (parameters, replacement) = match active.definition.kind {
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
            if crate::expansion::dynamic_builtin(spelling) {
                return false;
            }
            !(self.last_definition.contains_key(spelling) || self.active.contains_key(spelling))
                || self.unchanged(spelling, position, visited)
        })
    }
}
