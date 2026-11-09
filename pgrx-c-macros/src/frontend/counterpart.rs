//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Generate the same macros for an assert-enabled PostgreSQL as for a release build.
//!
//! `USE_ASSERT_CHECKING` changes more than `Assert`. An assert-enabled build defines
//! `castNode` to call `castNodeImpl`, which only it declares, and declares checks such as
//! `AssertTransactionIdInAllowableRange` as functions where a release build defines no-op
//! macros. Which macros exist, and how they are called, must not depend on the build.
//!
//! An assert-enabled inspection therefore also parses its headers as a release build
//! would, with `USE_ASSERT_CHECKING` removed from `pg_config.h` in memory. A macro takes
//! its release definition when the assert-enabled build declares it some other way or
//! its assert-enabled definition reaches a declaration the release build lacks. Those
//! definitions follow the headers in a derived main file, so every later compiler pass
//! sees them. Functions and macros that only the assert-enabled build defines are not
//! generated.

use super::{FrontendError, collect_declarations};
use crate::{FrontendOutput, MacroDefinition, MacroKind, MacroScanner, SourceSpan, TokenKind};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write as _;
use std::path::{Path, PathBuf};

/// The macro whose definition selects PostgreSQL's assertion-enabled declarations.
const ASSERT_CHECKING: &str = "USE_ASSERT_CHECKING";

/// What an assert-enabled inspection takes from its release counterpart.
#[derive(Clone, Debug, Default)]
pub(crate) struct AssertionMode {
    /// Functions and macros that only the assert-enabled build declares. They are not generated.
    pub(crate) assert_only: BTreeSet<String>,
    /// Macros defined as in a release build, with the location of each release definition.
    pub(crate) release_definitions: BTreeMap<String, Option<SourceSpan>>,
}

/// The release counterpart's declarations and function-like macro definitions.
struct Release {
    /// The last definition of each function-like macro.
    macros: BTreeMap<String, MacroDefinition>,
    /// Static inline functions without parameters, result or `return`, each with its body's
    /// tokens and location. A call to one runs its body as a statement macro would.
    procedures: BTreeMap<String, (Vec<String>, Option<SourceSpan>)>,
    /// Every macro, function, variable, type and enumerator name.
    declared: BTreeSet<String>,
}

/// Whether the inspected headers enable PostgreSQL's assertions.
pub(super) fn assert_enabled(frontend: &FrontendOutput) -> bool {
    frontend.environment().active.contains_key(ASSERT_CHECKING)
}

/// Compare an assert-enabled inspection with its release counterpart, returning the
/// assertion mode and, when release definitions must replace some macros, the contents
/// of a derived main file that defines them after `header`.
pub(super) fn compare(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    header: &Path,
) -> Result<(AssertionMode, Option<String>), FrontendError> {
    let release = inspect_release(scanner, frontend)?;
    let declarations = frontend.declarations();
    let active = &frontend.environment().active;
    let declared = |name: &str| {
        active.contains_key(name)
            || declarations.function_signatures.contains_key(name)
            || declarations.functions.contains_key(name)
            || declarations.variables.contains_key(name)
            || declarations.types.contains_key(name)
            || declarations.integer_constants.contains_key(name)
    };
    let mut mode = AssertionMode::default();
    let names = active
        .keys()
        .chain(declarations.function_signatures.keys())
        .chain(declarations.functions.keys())
        .chain(declarations.variables.keys())
        .chain(declarations.types.keys())
        .chain(declarations.integer_constants.keys());
    for name in names {
        if !release.declared.contains(name) {
            mode.assert_only.insert(name.clone());
        }
    }
    // Names an assert-enabled macro must not reach: those only that build declares, and
    // those it declares as something other than the release build's function-like macro.
    let mut differing = mode.assert_only.clone();
    let function_like = |name: &str| {
        active.get(name).is_some_and(|active| active.definition.kind == MacroKind::FunctionLike)
    };
    differing.extend(
        release.macros.keys().filter(|name| declared(name) && !function_like(name)).cloned(),
    );
    let mut overrides = String::new();
    for (name, definition) in &release.macros {
        let replace = match active.get(name) {
            Some(debug) if debug.definition.kind == MacroKind::FunctionLike => {
                significant(&debug.definition) != significant(definition)
                    && reaches(frontend, name, &differing)
            }
            Some(_) => false,
            None => true,
        };
        if !replace {
            continue;
        }
        let Some(text) = define(definition) else { continue };
        overrides.push_str(&format!("#undef {name}\n#define {text}\n"));
        mode.release_definitions.insert(name.clone(), definition.provenance.clone());
    }
    // A release procedure that this build declares only as a function becomes the
    // statement macro its call is equivalent to.
    for (name, (body, provenance)) in &release.procedures {
        let inline = declarations
            .function_signatures
            .get(name)
            .is_some_and(|function| function.is_static && function.is_inline);
        if active.contains_key(name) || inline || !declared(name) {
            continue;
        }
        overrides.push_str(&format!("#define {name}() do {{ {} }} while (0)\n", body.join(" ")));
        mode.release_definitions.insert(name.clone(), provenance.clone());
    }
    if overrides.is_empty() {
        return Ok((mode, None));
    }
    // An include names its file verbatim; it has no escapes.
    let include = super::path_string(header)?;
    if include.contains(['"', '\n', '\r']) {
        return Err(FrontendError::Arguments(format!(
            "{include}: a main file whose path contains a quote or line break cannot be included"
        )));
    }
    let contents = format!(
        "/* PostgreSQL headers with the release definitions of macros whose assert-enabled forms differ in kind or reach declarations a release build lacks. */\n#include \"{include}\"\n{overrides}"
    );
    Ok((mode, Some(contents)))
}

/// Write a derived main file once, at a path named by its contents.
pub(super) fn materialize(contents: &str, header: &Path) -> Result<PathBuf, FrontendError> {
    let name = header.file_name().ok_or_else(|| {
        FrontendError::Arguments("the inspected main file has no file name".into())
    })?;
    let directory = std::env::temp_dir()
        .join("pgrx-c-macros-release-definitions")
        .join(crate::sha256_hex(contents.as_bytes()));
    let path = directory.join(name);
    let failure = |source| FrontendError::FileIo { path: path.clone(), source };
    match std::fs::read(&path) {
        Ok(existing) if existing == contents.as_bytes() => return Ok(path),
        Ok(_) => {
            return Err(failure(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "a derived main file disagrees with the contents that name it",
            )));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(failure(error)),
    }
    std::fs::create_dir_all(&directory).map_err(failure)?;
    let mut staged = tempfile::NamedTempFile::new_in(&directory).map_err(failure)?;
    staged.write_all(contents.as_bytes()).map_err(failure)?;
    match staged.persist_noclobber(&path) {
        Ok(_) => Ok(path),
        // Another process published the same contents first.
        Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
            if std::fs::read(&path).map_err(failure)? == contents.as_bytes() {
                Ok(path)
            } else {
                Err(failure(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "a concurrently derived main file disagrees with its contents",
                )))
            }
        }
        Err(error) => Err(failure(error.error)),
    }
}

/// Point each release definition's provenance at its PostgreSQL header, rather than at
/// the derived main file that repeats it.
pub(super) fn locate(frontend: &mut FrontendOutput, mode: &AssertionMode) {
    for (name, provenance) in &mode.release_definitions {
        if let Some(active) = frontend.environment.active.get_mut(name) {
            active.definition.provenance.clone_from(provenance);
        }
    }
}

/// Parse the inspected headers with `USE_ASSERT_CHECKING` undefined.
///
/// Command-line definitions are dropped, and each file that defines the macro is read
/// with that definition's lines blanked, so every other line keeps its number.
fn inspect_release(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
) -> Result<Release, FrontendError> {
    let arguments = without_assert_checking(&frontend.profile().arguments);
    let mut replaced = BTreeMap::<PathBuf, Vec<String>>::new();
    for definition in
        frontend.inventory().macros.iter().filter(|definition| definition.name == ASSERT_CHECKING)
    {
        let Some(span) = &definition.provenance else { continue };
        let lines = match replaced.get_mut(&span.file) {
            Some(lines) => lines,
            None => {
                let contents = std::fs::read_to_string(&span.file)
                    .map_err(|source| FrontendError::FileIo { path: span.file.clone(), source })?;
                replaced
                    .entry(span.file.clone())
                    .or_insert(contents.split('\n').map(str::to_owned).collect())
            }
        };
        for line in span.start_line..=span.end_line {
            if let Some(text) = usize::try_from(line).ok().and_then(|line| lines.get_mut(line - 1))
            {
                text.clear();
            }
        }
    }
    let replaced =
        replaced.into_iter().map(|(path, lines)| (path, lines.join("\n"))).collect::<Vec<_>>();
    let borrowed = replaced
        .iter()
        .map(|(path, contents)| (path.as_path(), contents.as_str()))
        .collect::<Vec<_>>();
    let directory = std::env::current_dir().map_err(|error| {
        FrontendError::Environment(format!("could not resolve the working directory: {error}"))
    })?;
    let (inventory, declarations, procedures) = scanner.with_replaced_files(
        &frontend.profile().header,
        &arguments,
        &borrowed,
        true,
        |unit, inventory| {
            Ok((inventory, collect_declarations(unit).0, procedures(unit, &directory)))
        },
    )?;
    if inventory.macros.iter().any(|definition| definition.name == ASSERT_CHECKING) {
        return Err(FrontendError::Environment(format!(
            "{ASSERT_CHECKING} is still defined after removing its definitions; the release counterpart of these headers cannot be inspected"
        )));
    }
    let mut macros = BTreeMap::new();
    let mut declared = BTreeSet::new();
    let procedures = procedures
        .into_iter()
        .filter(|(name, _)| {
            declarations
                .function_signatures
                .get(name)
                .is_some_and(|function| function.is_static && function.is_inline)
        })
        .collect();
    for definition in inventory.macros {
        declared.insert(definition.name.clone());
        if definition.kind == MacroKind::FunctionLike {
            macros.insert(definition.name.clone(), definition);
        } else {
            macros.remove(&definition.name);
        }
    }
    declared.extend(declarations.function_signatures.into_keys());
    declared.extend(declarations.functions.into_keys());
    declared.extend(declarations.variables.into_keys());
    declared.extend(declarations.types.into_keys());
    declared.extend(declarations.integer_constants.into_keys());
    Ok(Release { macros, procedures, declared })
}

/// Static inline `void` functions without parameters or `return`, with their bodies' tokens.
fn procedures(
    unit: &clang::TranslationUnit<'_>,
    directory: &Path,
) -> BTreeMap<String, (Vec<String>, Option<SourceSpan>)> {
    use clang::{EntityKind, EntityVisitResult, StorageClass, TypeKind};
    let mut procedures = BTreeMap::new();
    unit.get_entity().visit_children(|entity, _| {
        if entity.get_kind() != EntityKind::FunctionDecl
            || !entity.is_definition()
            || !entity.is_inline_function()
            || entity.get_storage_class() != Some(StorageClass::Static)
        {
            return EntityVisitResult::Continue;
        }
        let void = entity.get_type().is_some_and(|ty| {
            ty.get_kind() == TypeKind::FunctionPrototype
                && !ty.is_variadic()
                && ty.get_argument_types().is_some_and(|arguments| arguments.is_empty())
                && ty
                    .get_result_type()
                    .is_some_and(|result| result.get_canonical_type().get_kind() == TypeKind::Void)
        });
        let (Some(name), Some(range)) = (entity.get_name(), entity.get_range()) else {
            return EntityVisitResult::Continue;
        };
        // The body is the definition's tokens from its first brace, which follows the
        // parameter list.
        let tokens = range
            .tokenize()
            .into_iter()
            .map(|token| crate::unspliced(token.get_spelling()))
            .collect::<Vec<_>>();
        let body = tokens
            .iter()
            .position(|token| token == "{")
            .and_then(|open| Some((open, tokens.len().checked_sub(1)?)))
            .filter(|&(open, close)| open < close && tokens[close] == "}")
            .map(|(open, close)| tokens[open + 1..close].to_vec());
        if let Some(body) = body.filter(|_| void)
            && !body.iter().any(|token| matches!(token.as_str(), "return" | "#" | "%:"))
        {
            let provenance = SourceSpan::from_range(range, directory).ok().flatten();
            procedures.insert(name, (body, provenance));
        }
        EntityVisitResult::Continue
    });
    procedures
}

/// Drop command-line definitions of `USE_ASSERT_CHECKING`.
fn without_assert_checking(arguments: &[String]) -> Vec<String> {
    let defines = |argument: &str| {
        argument == ASSERT_CHECKING
            || argument.strip_prefix(ASSERT_CHECKING).is_some_and(|rest| rest.starts_with('='))
    };
    let mut result = Vec::with_capacity(arguments.len());
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        if argument == "-D" {
            match arguments.next() {
                Some(value) if defines(value) => {}
                Some(value) => result.extend([argument.clone(), value.clone()]),
                None => result.push(argument.clone()),
            }
        } else if !argument.strip_prefix("-D").is_some_and(defines) {
            result.push(argument.clone());
        }
    }
    result
}

/// Whether an assert-enabled macro's expansion reaches a name the release build declares
/// differently, through the active macros it names.
fn reaches(frontend: &FrontendOutput, root: &str, differing: &BTreeSet<String>) -> bool {
    let active = &frontend.environment().active;
    let mut pending = vec![root];
    let mut visited = BTreeSet::new();
    while let Some(name) = pending.pop() {
        if !visited.insert(name) {
            continue;
        }
        let Some(macro_) = active.get(name) else { continue };
        let tokens = significant(&macro_.definition);
        let (parameters, body) = parameters_and_body(macro_.definition.kind, &tokens);
        for token in body {
            if parameters.contains(token) || *token == name {
                continue;
            }
            if differing.contains(*token) {
                return true;
            }
            if let Some((name, _)) = active.get_key_value(*token) {
                pending.push(name.as_str());
            }
        }
    }
    false
}

/// The spellings of a definition's tokens, without comments.
fn significant(definition: &MacroDefinition) -> Vec<&str> {
    definition
        .tokens
        .iter()
        .filter(|token| token.kind != TokenKind::Comment)
        .map(|token| token.spelling.as_str())
        .collect()
}

/// A definition's parameter names and replacement list, after its name.
fn parameters_and_body<'a>(
    kind: MacroKind,
    tokens: &'a [&'a str],
) -> (BTreeSet<&'a str>, &'a [&'a str]) {
    let tokens = tokens.get(1..).unwrap_or_default();
    if kind == MacroKind::ObjectLike {
        return (BTreeSet::new(), tokens);
    }
    let close =
        tokens.iter().position(|token| *token == ")").unwrap_or(tokens.len().saturating_sub(1));
    let parameters = tokens
        .get(1..close)
        .unwrap_or_default()
        .iter()
        .copied()
        .filter(|token| *token != ",")
        .collect();
    (parameters, tokens.get(close + 1..).unwrap_or_default())
}

/// The `#define` text of a function-like definition, or `None` when its tokens stringify or
/// paste, whose spelling the token stream does not preserve.
fn define(definition: &MacroDefinition) -> Option<String> {
    let tokens = significant(definition);
    if tokens.iter().any(|token| matches!(*token, "#" | "##" | "%:" | "%:%:")) {
        return None;
    }
    let (name, rest) = tokens.split_first()?;
    let close = rest.iter().position(|token| *token == ")")?;
    if rest.first() != Some(&"(") {
        return None;
    }
    Some(format!("{name}{} {}", rest[..=close].concat(), rest[close + 1..].join(" ")))
}
