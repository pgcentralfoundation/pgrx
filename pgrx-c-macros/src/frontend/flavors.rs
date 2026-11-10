//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Give assert-enabled and release PostgreSQL one set of generated macros.
//!
//! `USE_ASSERT_CHECKING` changes which names PostgreSQL's headers declare, how `Assert` and
//! its relatives are defined, and what some static inline functions contain. A program that
//! uses the generated macros must compile against either build flavor, and a macro must
//! never skip a check that the assert-enabled headers make.
//!
//! Inspection therefore parses the headers a second time, in memory, as the other flavor:
//! with `USE_ASSERT_CHECKING` defined on the command line, or with its definitions blanked.
//! Both flavors then hold the same two views, and every decision here is a function of that
//! pair, so both reach it:
//!
//! - A name that one flavor alone declares is generated and bound by neither.
//! - A function-like macro that the flavors define differently gets one definition in both,
//!   under which each build does what its own C definition does. A macro that is nothing
//!   but checks, beside a release definition that does nothing, keeps its assert-enabled
//!   definition: whether a check runs is a constant of the installation. Any other gets a
//!   definition that holds both, in which that constant selects the installation's own.
//!   When no one definition can hold both, or one of them reaches a name the other flavor
//!   lacks or declares otherwise, neither flavor generates the macro. A static inline
//!   function that only an assert-enabled build defines is the exception: a release build
//!   compiles that definition itself, where its assertions do nothing, so code that names
//!   it compiles there.
//! - A function that an assert-enabled build declares where a release build defines a macro
//!   that does nothing, or the reverse of a static inline function, becomes a macro that
//!   calls the function in both. A release macro that does anything is a conflict.
//! - An external function that only an assert-enabled build declares, and that returns
//!   nothing, is called in both by the code that only an assert-enabled build has: the
//!   statements of a merged inline body and a macro's assert-enabled definition. A release
//!   build's call does nothing.
//! - A static inline function whose definition differs between the flavors is translated
//!   from one body merged from both definitions, in which the installation's flavor selects
//!   the statements that only one of them has. When the definitions do not merge, both
//!   flavors call the function.
//!
//! The shared definitions follow the main file wherever the compiler expands macros, so no
//! file is written.

use super::definitions::{self, Located};
use super::{FrontendError, collect_declarations};
use crate::syntax::BUILD_FLAVOR;
use crate::{
    ActiveMacro, ActiveProvenance, DeclarationCatalog, DiagnosticSeverity, FrontendOutput,
    FunctionInfo, MacroDefinition, MacroKind, MacroScanner, SourceSpan, Token, TokenKind,
    TypeCategory,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// Merge the two definitions of a static inline function's body into one.
mod merge;
/// Give a macro that the build flavors define differently one definition.
mod share;

pub(crate) use share::Inert;

/// The macro whose definition selects PostgreSQL's assertion-enabled declarations.
const ASSERT_CHECKING: &str = "USE_ASSERT_CHECKING";
/// The function through which PostgreSQL reports a failed assertion.
pub(crate) const ASSERTION_REPORT: &str = "ExceptionalCondition";
/// Bound the presence probes, as other in-memory probes are bounded.
const MAX_SOURCE_BYTES: usize = 16 * 1024 * 1024;
/// Bound a function definition a release build compiles itself, as retained inline
/// definitions are bounded.
const MAX_DEFINITION_BYTES: usize = 64 * 1024;

/// What an inspection decided by comparing the two build flavors of its headers.
///
/// A report can serialize it, so the decisions of two builds can be compared.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Flavors {
    /// Whether the inspected headers enable assertions.
    pub(crate) assert_enabled: bool,
    /// Names neither flavor generates or binds: those the inspected flavor declares and the
    /// other does not, and functions the flavors declare with different prototypes.
    pub(crate) flavor_specific: BTreeSet<String>,
    /// Function-like macros that no definition can serve in both flavors, with the reason.
    pub(crate) conflicts: BTreeMap<String, String>,
    /// Macros defined here to call the function of their own name, which one build flavor
    /// declares differently or not at all.
    pub(crate) calls: BTreeSet<String>,
    /// External functions that only an assert-enabled build declares and that return
    /// nothing. Generated code calls one through its own adapter, which does nothing in a
    /// release build, and no build binds one.
    pub(crate) assert_functions: BTreeSet<String>,
    /// Functions that an assert-enabled build declares where a release build defines a
    /// macro no call can stand for. The macro is a conflict, and no build binds the function.
    pub(crate) unbound_functions: BTreeSet<String>,
    /// Static inline functions that only an assert-enabled build defines and that shared
    /// definitions may call, with their C definitions. A release build compiles them with
    /// its own macros, so their assertions do nothing there.
    pub(crate) helpers: BTreeMap<String, String>,
    /// Static inline functions whose definitions differ between the flavors and cannot be
    /// merged, or which report assertions themselves. Both flavors call them.
    pub(crate) native_only: BTreeSet<String>,
    /// Static inline functions whose definitions differ between the flavors and which both
    /// give one body, merged from the two definitions, in which the installation's flavor
    /// selects the statements that only one of them has. Both translate that body, or call
    /// the function when it cannot be translated.
    pub(crate) merged: BTreeSet<String>,
    /// Function-like macros that the flavors define differently and that keep the
    /// assert-enabled definition in both, each with what its release definition yields.
    /// That definition does nothing, and the assert-enabled one is nothing but checks, so
    /// with its checks off it does nothing either. The emitter proves that on the analyzed
    /// definition before it generates anything that expands the macro.
    pub(crate) checked: BTreeMap<String, Inert>,
    /// Function-like macros that the flavors define differently and that have one
    /// definition holding both, in which the installation's flavor selects its own.
    pub(crate) selected: BTreeMap<String, Selected>,
    /// Whether the keyword that selects a build flavor is a token no header can produce:
    /// no flavor declares its spelling and no macro definition spells it. Every token so
    /// spelled in a compiler expansion then comes from a definition written here.
    #[serde(skip)]
    pub(crate) keyword_free: bool,
    /// Definitions that follow the main file wherever the compiler expands macros.
    pub(crate) definitions: String,
    /// Source the inspected flavor's preprocessor skipped.
    #[serde(skip)]
    pub(crate) skipped: BTreeSet<Skipped>,
}

/// The two definitions of a macro that has one definition holding both.
#[derive(Clone, Debug, serde::Serialize)]
pub struct Selected {
    /// The definition of a PostgreSQL built with assertions.
    pub(crate) assert_enabled: String,
    /// The definition of a PostgreSQL built without assertions.
    pub(crate) release: String,
}

/// A range of lines the preprocessor skipped: its file and first and last line.
pub(crate) type Skipped = (PathBuf, u32, u32);

impl Flavors {
    /// Whether a call to `name` reaches a function this installation lacks: one that only an
    /// assert-enabled build declares, inspected in a release build.
    pub(crate) fn lacks_function(&self, name: &str) -> bool {
        !self.assert_enabled && self.assert_functions.contains(name)
    }

    /// Whether both flavors expand the macro `name` to a definition written for them: the
    /// assert-enabled definition, or one that holds both definitions.
    pub(crate) fn shares_definition(&self, name: &str) -> bool {
        self.checked.contains_key(name) || self.selected.contains_key(name)
    }

    /// The C definition of a static inline function that the inspected headers lack and
    /// the native support must therefore compile before it calls the function.
    pub(crate) fn helper_definition(&self, name: &str) -> Option<&str> {
        self.helpers.get(name).filter(|_| !self.assert_enabled).map(String::as_str)
    }
}

/// The other flavor's macros and declarations.
struct Other {
    /// The final definition of each macro the inspected flavor names, and the last
    /// definition of each macro it never names.
    macros: BTreeMap<String, MacroDefinition>,
    /// Its declarations.
    declarations: DeclarationCatalog,
    /// Source its preprocessor skipped.
    skipped: BTreeSet<Skipped>,
    /// The spans and source text of its static inline function definitions.
    inline_definitions: BTreeMap<String, (SourceSpan, Option<String>)>,
    /// The source text of its function-like macro definitions, from each macro's name.
    texts: BTreeMap<String, String>,
    /// Its bodies of the static inline functions whose definitions span source that only
    /// one flavor's preprocessor skips, where its parse proves them as the inspected
    /// flavor's parse proves a retained body.
    bodies: BTreeMap<String, Body>,
    /// Whether any macro definition it ever had spells the keyword that selects a build
    /// flavor.
    spells_keyword: bool,
}

/// A static inline function's body in the other build flavor.
struct Body {
    /// The definition's parameter names.
    parameters: Vec<Option<String>>,
    /// The body's tokens, each with its offset in the definition's file.
    tokens: Vec<Located>,
}

/// One flavor's macros and declarations.
#[derive(Clone, Copy)]
struct View<'a> {
    /// Its final macro definitions.
    macros: &'a BTreeMap<String, MacroDefinition>,
    /// Its declarations.
    declarations: &'a DeclarationCatalog,
}

impl<'a> View<'a> {
    /// The function-like macro named `name`.
    fn function_like(self, name: &str) -> Option<&'a MacroDefinition> {
        self.macros.get(name).filter(|definition| definition.kind == MacroKind::FunctionLike)
    }

    /// The function named `name`, when no macro hides it.
    fn function(self, name: &str) -> Option<&'a FunctionInfo> {
        self.declarations.function_signatures.get(name).filter(|_| !self.macros.contains_key(name))
    }

    /// Whether `name` is a macro, function, variable, type or enumerator.
    fn declares(self, name: &str) -> bool {
        self.macros.contains_key(name)
            || self.declarations.function_signatures.contains_key(name)
            || self.declarations.functions.contains_key(name)
            || self.declarations.variables.contains_key(name)
            || self.declarations.types.contains_key(name)
            || self.declarations.integer_constants.contains_key(name)
    }
}

/// Collect the source the preprocessor skipped in `unit`.
pub(super) fn skipped(unit: &clang::TranslationUnit<'_>, directory: &Path) -> BTreeSet<Skipped> {
    unit.get_skipped_ranges()
        .into_iter()
        .filter_map(|range| SourceSpan::from_range(range, directory).ok().flatten())
        .map(|span| (span.file, span.start_line, span.end_line))
        .collect()
}

/// Whether a definition at `span` holds source that only one flavor's preprocessor skips.
fn overlaps(differing: &[Skipped], span: &SourceSpan) -> bool {
    differing.iter().any(|(file, start, end)| {
        *file == span.file && *start <= span.end_line && span.start_line <= *end
    })
}

/// Compare the inspected headers with their other build flavor and make `frontend` describe
/// the macros both flavors generate.
///
/// This runs after the proofs that compare the headers with the unchanged macro environment
/// and before any pass that expands macros. `offsets` gives the offset of each token of an
/// inline function body that those proofs retained and that the other flavor's parse could
/// prove alike.
pub(super) fn reconcile(
    scanner: &MacroScanner,
    frontend: &mut FrontendOutput,
    offsets: &BTreeMap<String, Vec<u32>>,
) -> Result<(), FrontendError> {
    let assert_enabled = frontend.environment.active.contains_key(ASSERT_CHECKING);
    let other = inspect_other(scanner, frontend, assert_enabled)?;
    let own_macros = frontend
        .environment
        .active
        .iter()
        .map(|(name, active)| (name.clone(), active.definition.clone()))
        .collect::<BTreeMap<_, _>>();
    let own = View { macros: &own_macros, declarations: &frontend.declarations };
    let counterpart = View { macros: &other.macros, declarations: &other.declarations };
    let (assert, release) = if assert_enabled { (own, counterpart) } else { (counterpart, own) };

    let mut flavors = Flavors {
        assert_enabled,
        skipped: std::mem::take(&mut frontend.flavors.skipped),
        ..Flavors::default()
    };
    let declarations = &frontend.declarations;
    let names = own_macros
        .keys()
        .chain(declarations.function_signatures.keys())
        .chain(declarations.functions.keys())
        .chain(declarations.variables.keys())
        .chain(declarations.types.keys())
        .chain(declarations.integer_constants.keys());
    for name in names {
        if !counterpart.declares(name) {
            flavors.flavor_specific.insert(name.clone());
        }
    }

    // Macros that call a function of their own name.
    let mut calls = BTreeMap::<String, MacroDefinition>::new();
    let call = |name: &str, formals: &[String], provenance: Option<SourceSpan>| {
        let token = |kind, spelling: &str| Token { kind, spelling: spelling.into() };
        let mut list = vec![token(TokenKind::Punctuation, "(")];
        for (index, formal) in formals.iter().enumerate() {
            if index != 0 {
                list.push(token(TokenKind::Punctuation, ","));
            }
            list.push(token(TokenKind::Identifier, formal));
        }
        list.push(token(TokenKind::Punctuation, ")"));
        // A parenthesized name is never a macro invocation, so this calls the function.
        let mut tokens = vec![token(TokenKind::Identifier, name)];
        tokens.extend(list.iter().cloned());
        tokens.extend([
            token(TokenKind::Punctuation, "("),
            token(TokenKind::Identifier, name),
            token(TokenKind::Punctuation, ")"),
        ]);
        tokens.extend(list);
        MacroDefinition {
            name: name.into(),
            kind: MacroKind::FunctionLike,
            location: None,
            provenance,
            tokens,
            builtin: false,
            main_file: false,
        }
    };
    // A macro in a release build that an assert-enabled build declares as a function.
    for (name, definition) in release.macros {
        if definition.kind != MacroKind::FunctionLike || assert.macros.contains_key(name) {
            continue;
        }
        let Some(function) = assert.function(name) else { continue };
        let formals = formals(definition);
        // A release build's call to the function does nothing, so it can stand only for a
        // release macro that does nothing.
        let callable = if nothing(release, definition).is_some() {
            assertion_function(function, Some(&formals), release.declarations)
        } else {
            Err("the macro does work that a call a release build does not make would leave out"
                .into())
        };
        match callable {
            Ok(()) => {
                calls.insert(name.clone(), call(name, &formals, definition.provenance.clone()));
                flavors.assert_functions.insert(name.clone());
            }
            Err(reason) => {
                flavors.conflicts.insert(
                    name.clone(),
                    format!(
                        "an assert-enabled build declares {name} as a function and a release build defines it as a macro, and {reason}"
                    ),
                );
                // A release build has no function of this name to bind.
                flavors.unbound_functions.insert(name.clone());
            }
        }
    }
    // A function that only an assert-enabled build declares. Nothing in a release build
    // has its name, so only translated code calls it.
    for (name, function) in &assert.declarations.function_signatures {
        if !release.declares(name)
            && assert.function(name).is_some()
            && assertion_function(function, None, release.declarations).is_ok()
        {
            flavors.assert_functions.insert(name.clone());
        }
    }
    // The reverse has no form both flavors can call.
    for (name, definition) in assert.macros {
        if definition.kind == MacroKind::FunctionLike
            && !release.macros.contains_key(name)
            && release.function(name).is_some()
        {
            flavors.conflicts.insert(
                name.clone(),
                format!(
                    "a release build declares {name} as a function and an assert-enabled build defines it as a macro"
                ),
            );
        }
    }
    // A function that one flavor defines inline and the other only declares.
    let inline = |function: &FunctionInfo| function.is_static && function.is_inline;
    for (name, function) in &assert.declarations.function_signatures {
        let (Some(asserted), Some(released)) = (assert.function(name), release.function(name))
        else {
            continue;
        };
        // One name with two prototypes has no binding that serves both flavors.
        if prototype(asserted) != prototype(released) {
            flavors.flavor_specific.insert(name.clone());
            continue;
        }
        let Some(parameters) = &function.signature.parameters else { continue };
        if inline(asserted) == inline(released) {
            continue;
        }
        let formals = (0..parameters.len()).map(|index| format!("arg{index}")).collect::<Vec<_>>();
        let provenance = if inline(released) == assert_enabled {
            other.inline_definitions.get(name).map(|(span, _)| span.clone())
        } else {
            frontend
                .declarations
                .function_signatures
                .get(name)
                .and_then(|function| function.definition.as_ref())
                .map(|definition| definition.provenance.clone())
        };
        calls.insert(name.clone(), call(name, &formals, provenance));
    }

    // Static inline functions that only an assert-enabled build defines, with their source.
    let candidates = assert
        .declarations
        .function_signatures
        .iter()
        .filter(|(name, function)| inline(function) && !release.declares(name))
        .filter_map(|(name, function)| {
            let text = if assert_enabled {
                function.definition.as_ref()?.source.clone()
            } else {
                other.inline_definitions.get(name)?.1.clone()
            };
            let parameters = function.signature.parameters.as_ref()?;
            let declared = std::iter::once(&function.signature.result)
                .chain(parameters)
                .all(|ty| shared_type(ty, release.declarations));
            (declared && !function.signature.variadic).then_some((name, text?))
        })
        .collect::<Vec<_>>();
    // Members that only one flavor's record has. No declaration names them, so a
    // definition that mentions one is kept out by its spelling.
    let members = differing_members(assert.declarations, release.declarations);
    // Names an assert-enabled definition cannot reach in a release build, given the
    // functions a release build compiles itself. A `translated` definition becomes Rust,
    // which calls an assertion function through its adapter. A definition that a release
    // build compiles as C cannot call a function that build does not declare.
    let assert_functions = &flavors.assert_functions;
    let unshared_given = |name: &str,
                          helpers: &BTreeMap<String, String>,
                          translated: bool|
     -> Option<&'static str> {
        if calls.contains_key(name)
            || helpers.contains_key(name)
            || (translated && assert_functions.contains(name))
        {
            return None;
        }
        match (assert.macros.get(name), release.macros.get(name)) {
            (Some(asserted), Some(released)) => {
                if asserted.kind != released.kind {
                    Some("which the build flavors define as different kinds of macro")
                } else if asserted.kind == MacroKind::ObjectLike
                    && significant(asserted) != significant(released)
                {
                    Some("which the build flavors define differently")
                } else {
                    None
                }
            }
            (Some(_), None) if release.declares(name) => {
                Some("which only an assert-enabled build defines as a macro")
            }
            (Some(_), None) => Some("which a release build does not define"),
            (None, Some(_)) => Some("which only a release build defines as a macro"),
            (None, None) => match (assert.declares(name), release.declares(name)) {
                (true, false) => Some("which a release build does not declare"),
                (false, true) => Some("which an assert-enabled build does not declare"),
                _ => None,
            },
        }
    };
    // A candidate is shared once everything its definition names is. One candidate may
    // call another, so repeat until none is added.
    loop {
        let unshared = |name: &str| unshared_given(name, &flavors.helpers, false);
        let added = candidates
            .iter()
            .filter(|(name, _)| !flavors.helpers.contains_key(*name))
            .filter(|(name, text)| {
                identifiers(text).all(|token| {
                    token == name.as_str()
                        || (!members.contains(token)
                            && unshared(token).is_none()
                            && reaches(assert, token, &unshared).is_none())
                })
            })
            .map(|(name, text)| ((*name).clone(), text.clone()))
            .collect::<Vec<_>>();
        if added.is_empty() {
            break;
        }
        flavors.helpers.extend(added);
    }
    // A translated definition calls an assertion function through its adapter, which does
    // nothing in a release build. So a macro definition or a merged inline body may name one.
    let unshared = |name: &str| unshared_given(name, &flavors.helpers, true);
    // The keyword that selects a build flavor must be a token that no header can produce.
    let spells = |definition: &MacroDefinition| {
        definition
            .tokens
            .iter()
            .any(|token| token.kind != TokenKind::Comment && token.spelling == BUILD_FLAVOR)
    };
    let keyword_free = !assert.declares(BUILD_FLAVOR)
        && !release.declares(BUILD_FLAVOR)
        && !other.spells_keyword
        && !own_macros
            .values()
            .chain(&frontend.inventory.macros)
            .chain(other.macros.values())
            .any(spells);
    // Function-like macros the flavors define differently, whose assert-enabled definition
    // a release build can expand.
    let mut differing = Vec::new();
    for (name, asserted) in assert.macros {
        let Some(released) = release.function_like(name) else { continue };
        if asserted.kind != MacroKind::FunctionLike || flavors.conflicts.contains_key(name) {
            continue;
        }
        if significant(asserted) == significant(released) {
            // Release C compiles such a report whether or not assertions are enabled, and
            // the generated one would run only when they are.
            if significant(asserted).contains(&ASSERTION_REPORT) {
                flavors.conflicts.insert(
                    name.clone(),
                    format!(
                        "{name} calls {ASSERTION_REPORT} in a build without assertions, where generated assertions do not run"
                    ),
                );
            }
            continue;
        }
        let reason = match (reaches(assert, name, &unshared), member(asserted, &members)) {
            (Some((reached, reason)), _) => format!("uses {reached}, {reason}"),
            (None, Some(member)) => {
                format!("names {member}, a member that only one build flavor's record has")
            }
            (None, None) => {
                differing.push((name, asserted, released));
                continue;
            }
        };
        flavors.conflicts.insert(
            name.clone(),
            format!(
                "the assert-enabled definition of {name} differs from the release definition and {reason}"
            ),
        );
    }
    // A macro that is nothing but checks, beside a release definition that does nothing,
    // keeps the assert-enabled definition in both. One such macro may invoke another, so
    // repeat until none is added.
    loop {
        let before = flavors.checked.len();
        differing.retain(|(name, asserted, released)| {
            let off = share::checks(&parameters_and_body(asserted).1, &flavors.checked);
            match (off, nothing(release, released)) {
                (Some(off), Some(nothing)) if off == nothing => {
                    flavors.checked.insert((*name).clone(), nothing);
                    false
                }
                _ => true,
            }
        });
        if flavors.checked.len() == before {
            break;
        }
    }
    // Any other gets one definition that holds both, in which each build evaluates its own.
    let mut selected = Vec::new();
    for (name, asserted, released) in differing {
        let select = || {
            if !keyword_free {
                return Err(format!(
                    "the headers use the name {BUILD_FLAVOR}, which would select between its definitions"
                ));
            }
            if let Some((reached, reason)) = reaches(release, name, &unshared) {
                return Err(format!("its release definition uses {reached}, {reason}"));
            }
            if let Some(member) = member(released, &members) {
                return Err(format!(
                    "its release definition names {member}, a member that only one build flavor's record has"
                ));
            }
            let parameters =
                "its two definitions do not take the same parameters under names that both can use";
            let (formals, checked) = replacement(asserted).ok_or(parameters)?;
            let (release_formals, unchecked) = replacement(released).ok_or(parameters)?;
            let unchecked =
                share::rename(&unchecked, &release_formals, &formals).ok_or(parameters)?;
            let formal = |name: &str| formals.contains(&name);
            let body = share::select(
                &side(assert, checked, &formal, nothing(assert, asserted).is_some()),
                &side(
                    release,
                    unchecked.iter().collect(),
                    &formal,
                    nothing(release, released).is_some(),
                ),
            )?;
            // The signature and location stay the assert-enabled definition's.
            let mut definition = asserted.clone();
            let mut signature = Vec::new();
            for token in asserted.tokens.iter().filter(|token| token.kind != TokenKind::Comment) {
                signature.push(token.clone());
                if token.spelling == ")" {
                    break;
                }
            }
            definition.tokens = signature.into_iter().chain(body).collect();
            Ok(definition)
        };
        match select() {
            Ok(definition) => {
                flavors.selected.insert(
                    name.clone(),
                    Selected {
                        assert_enabled: asserted.to_string(),
                        release: released.to_string(),
                    },
                );
                selected.push(definition);
            }
            Err(reason) => {
                flavors.conflicts.insert(
                    name.clone(),
                    format!("the build flavors define {name} differently, and {reason}"),
                );
            }
        }
    }

    // Static inline functions whose definitions the flavors do not share. Both translate
    // the body merged from the two definitions when there is one, and call the function
    // otherwise.
    let differing =
        flavors.skipped.symmetric_difference(&other.skipped).cloned().collect::<Vec<_>>();
    // Whether every flavor can name `token` in translated code, and means the same by it.
    let shared_name = |token: &str| {
        !members.contains(token)
            && (assert.declares(token) || !release.declares(token))
            && unshared(token).is_none()
            && reaches(assert, token, &unshared).is_none()
            && reaches(release, token, &unshared).is_none()
    };
    let reports =
        |token: &Token| token.kind == TokenKind::Identifier && token.spelling == ASSERTION_REPORT;
    let mut merged = BTreeMap::new();
    for (name, function) in &frontend.declarations.function_signatures {
        let Some(definition) = &function.definition else { continue };
        let span = &definition.provenance;
        let reporting = definition.body.as_ref().is_some_and(|body| body.iter().any(reports));
        if !overlaps(&differing, span) {
            if reporting {
                flavors.native_only.insert(name.clone());
            }
            continue;
        }
        let body = || {
            let (asserted, released) = (assert.function(name)?, release.function(name)?);
            let theirs = other.bodies.get(name)?;
            let ours = definition.body.as_ref()?;
            let offsets = offsets.get(name).filter(|offsets| offsets.len() == ours.len())?;
            // One function in both flavors, defined in one place with one set of names,
            // so its documentation shows both flavors the same definition.
            if !keyword_free
                || !inline(asserted)
                || !inline(released)
                || prototype(asserted) != prototype(released)
                || other.inline_definitions.get(name)?.0 != *span
                || theirs.parameters != definition.parameters
                || definition.parameters.iter().flatten().any(|name| name == BUILD_FLAVOR)
                || reporting
                || theirs.tokens.iter().any(|(_, token)| reports(token))
            {
                return None;
            }
            let ours = offsets.iter().copied().zip(ours.iter().cloned()).collect::<Vec<_>>();
            let tokens = if assert_enabled {
                merge::merge(&ours, &theirs.tokens)
            } else {
                merge::merge(&theirs.tokens, &ours)
            }?;
            tokens
                .iter()
                .filter(|token| matches!(token.kind, TokenKind::Identifier | TokenKind::Keyword))
                .all(|token| token.spelling == BUILD_FLAVOR || shared_name(&token.spelling))
                .then_some(tokens)
        };
        match body() {
            Some(tokens) => {
                merged.insert(name.clone(), tokens);
            }
            None => {
                flavors.native_only.insert(name.clone());
            }
        }
    }

    // A release build has none of the assertion functions. It declares them as the
    // assert-enabled build does, so both check a call's operands alike, and calls nothing.
    let lacking = flavors
        .assert_functions
        .iter()
        .filter(|_| !assert_enabled)
        .map(|name| (name, false))
        .chain(flavors.helpers.keys().filter(|_| !assert_enabled).map(|name| (name, true)))
        .map(|(name, defined)| {
            let mut function = assert.declarations.function_signatures[name].clone();
            function.definition = None;
            function.definition_available = defined;
            (name.clone(), function, assert.declarations.functions.get(name).cloned())
        })
        .collect::<Vec<_>>();
    // Definitions the inspected headers lack. An assert-enabled build already has the
    // definitions that are nothing but checks. A macro that only the other flavor defines,
    // and that neither can generate, is defined so that both report why.
    let mut adopted = flavors
        .checked
        .keys()
        .filter(|_| !assert_enabled)
        .map(|name| (name.clone(), assert.macros[name].clone()))
        .collect::<BTreeMap<_, _>>();
    for name in flavors.conflicts.keys() {
        if !own.macros.contains_key(name)
            && let Some(definition) = counterpart.function_like(name)
        {
            adopted.insert(name.clone(), definition.clone());
        }
    }
    let mut definitions = String::new();
    let mut define_macro = |text: Option<String>, definition: MacroDefinition| {
        let text = text.or_else(|| define(&definition));
        let text = text.expect("a function-like definition has a parameter list");
        definitions.push_str(&format!("#undef {}\n#define {text}\n", definition.name));
        frontend.environment.active.insert(
            definition.name.clone(),
            ActiveMacro { definition, provenance: ActiveProvenance::Resolved },
        );
    };
    for (name, definition) in adopted {
        define_macro(other.texts.get(&name).cloned(), definition);
    }
    // Both flavors take the definitions that hold both of theirs.
    for definition in selected {
        define_macro(None, definition);
    }
    flavors.calls = calls.keys().cloned().collect();
    for definition in calls.into_values() {
        define_macro(None, definition);
    }
    for (name, function, ty) in lacking {
        frontend.declarations.functions.extend(ty.map(|ty| (name.clone(), ty)));
        frontend.declarations.function_signatures.insert(name, function);
    }
    for (name, tokens) in merged {
        let definition = frontend
            .declarations
            .function_signatures
            .get_mut(&name)
            .and_then(|function| function.definition.as_mut())
            .expect("a merged body belongs to a catalogued definition");
        definition.body = Some(tokens);
        flavors.merged.insert(name);
    }
    flavors.definitions = definitions;
    flavors.keyword_free = keyword_free;
    frontend.dependencies = crate::MacroDependencyGraph::from_environment_with_constants(
        &frontend.environment,
        &frontend.declarations.integer_constants,
    );
    frontend.flavors = flavors;
    Ok(())
}

/// Whether a release build can do nothing in place of a call to an assert-enabled build's
/// function `function`: the function is external, returns nothing, and takes parameters of
/// types a release build declares too. A release build's macro of the same name must take
/// as many `formals`.
fn assertion_function(
    function: &FunctionInfo,
    formals: Option<&[String]>,
    release: &DeclarationCatalog,
) -> Result<(), String> {
    let signature = &function.signature;
    let Some(parameters) = &signature.parameters else {
        return Err("the function has no prototype".into());
    };
    if signature.variadic
        || formals.is_some_and(|formals| formals.iter().any(|formal| formal == "..."))
    {
        return Err("one of them is variadic".into());
    }
    if signature.result.category != TypeCategory::Void {
        return Err("the function returns a value that a release build cannot produce".into());
    }
    if formals.is_some_and(|formals| parameters.len() != formals.len()) {
        return Err("they take different numbers of arguments".into());
    }
    if function.is_static {
        return Err("the function is not external".into());
    }
    for parameter in parameters {
        if !shared_type(parameter, release) {
            return Err(format!(
                "a release build does not declare its parameter type {}",
                parameter.spelling
            ));
        }
    }
    Ok(())
}

/// Whether a release build declares a type an assert-enabled prototype uses.
fn shared_type(ty: &crate::TypeInfo, release: &DeclarationCatalog) -> bool {
    matches!(ty.category, TypeCategory::Integer(_) | TypeCategory::Floating | TypeCategory::Void)
        || release.type_shapes.contains_key(&ty.canonical_spelling)
}

/// The names of members that only one flavor's version of a record has.
fn differing_members(
    assert: &DeclarationCatalog,
    release: &DeclarationCatalog,
) -> BTreeSet<String> {
    let names = |record: Option<&crate::RecordInfo>| {
        record
            .into_iter()
            .flat_map(|record| &record.fields)
            .filter_map(|field| field.name.clone())
            .collect::<BTreeSet<_>>()
    };
    let mut members = BTreeSet::new();
    for record in assert.records.keys().chain(release.records.keys()) {
        let asserted = names(assert.records.get(record));
        let released = names(release.records.get(record));
        members.extend(asserted.symmetric_difference(&released).cloned());
    }
    members
}

/// The identifiers in C source text, including any in its comments and literals.
fn identifiers(text: &str) -> impl Iterator<Item = &str> {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|word| word.bytes().next().is_some_and(|byte| !byte.is_ascii_digit()))
}

/// What a macro of `view` yields when it does nothing wherever it is used: its replacement
/// list names no parameter, function or variable and writes nothing, also through the
/// object-like macros it names. See [`share::nothing`].
fn nothing<'a>(view: View<'a>, definition: &'a MacroDefinition) -> Option<Inert> {
    let (parameters, body) = parameters_and_body(definition);
    // A parameter's operand may have an effect, whatever the parameter is called.
    if body.iter().any(|token| parameters.contains(token)) {
        return None;
    }
    let object = |name: &str| {
        view.macros
            .get(name)
            .filter(|named| named.kind == MacroKind::ObjectLike)
            .map(|named| parameters_and_body(named).1)
    };
    share::nothing(&body, &object)
}

/// Whether the identifier `name` of `view` stands for itself or for a closed expression,
/// so that it can be a closed expression's callee: it is no macro, or a function-like
/// macro whose replacement list is closed.
fn closed(view: View<'_>, name: &str, depth: usize) -> bool {
    let Some(definition) = view.macros.get(name) else { return true };
    let Some((formals, body)) = replacement(definition).filter(|_| depth < 16) else {
        return false;
    };
    share::closed_expression(&body, &|name| formals.contains(&name), &|name| {
        closed(view, name, depth + 1)
    })
}

/// A name in a definition's replacement list that selects a member only one flavor's
/// record has: after `.` or `->`, or anywhere in a definition that takes a member's offset.
fn member<'a>(definition: &'a MacroDefinition, members: &BTreeSet<String>) -> Option<&'a str> {
    let (parameters, body) = parameters_and_body(definition);
    let offsets = body.iter().any(|token| matches!(*token, "offsetof" | "__builtin_offsetof"));
    body.iter().enumerate().find_map(|(index, token)| {
        let selected = offsets
            || index.checked_sub(1).is_some_and(|before| matches!(body[before], "." | "->"));
        (selected && members.contains(*token) && !parameters.contains(token)).then_some(*token)
    })
}

/// One flavor's replacement list `body` for [`share::select`], whose identifiers `view`
/// defines. `formal` names the macro's parameters and `nothing` says that it does nothing.
fn side<'a>(
    view: View<'_>,
    body: Vec<&'a Token>,
    formal: &impl Fn(&str) -> bool,
    nothing: bool,
) -> share::Side<'a> {
    share::Side {
        expression: share::closed_expression(&body, formal, &|name| closed(view, name, 0)),
        statement: share::closed_statement(&body)
            || share::invoked(&body).is_some_and(|name| !formal(name) && statement(view, name, 0)),
        nothing,
        body,
    }
}

/// Whether the function-like macro `name` of `view` is one statement wherever it is
/// invoked: its replacement list is `do { } while (0)`, or one invocation of such a macro.
fn statement(view: View<'_>, name: &str, depth: usize) -> bool {
    let Some((formals, body)) = view.macros.get(name).and_then(replacement).filter(|_| depth < 16)
    else {
        return false;
    };
    share::closed_statement(&body)
        || share::invoked(&body)
            .is_some_and(|name| !formals.contains(&name) && statement(view, name, depth + 1))
}

/// A function-like definition's parameter names and the tokens of its replacement list.
fn replacement(definition: &MacroDefinition) -> Option<(Vec<&str>, Vec<&Token>)> {
    if definition.kind != MacroKind::FunctionLike {
        return None;
    }
    let mut tokens =
        definition.tokens.iter().filter(|token| token.kind != TokenKind::Comment).skip(1);
    if tokens.next()?.spelling != "(" {
        return None;
    }
    let mut formals = Vec::new();
    loop {
        let token = tokens.next()?;
        match token.spelling.as_str() {
            ")" => break,
            "," => {}
            name => formals.push(name),
        }
    }
    Some((formals, tokens.collect()))
}

/// A function's result and parameter types, by canonical spelling.
fn prototype(function: &FunctionInfo) -> (String, Option<Vec<String>>, bool) {
    let signature = &function.signature;
    (
        signature.result.canonical_spelling.clone(),
        signature.parameters.as_ref().map(|parameters| {
            parameters.iter().map(|parameter| parameter.canonical_spelling.clone()).collect()
        }),
        signature.variadic,
    )
}

/// Parse the inspected headers as the other build flavor.
///
/// Command-line definitions of `USE_ASSERT_CHECKING` are dropped or added. For a release
/// view, each file that defines the macro is read with that definition's lines blanked, so
/// every other line keeps its number and offset. For an assert-enabled view of headers that undefine
/// the macro, as a `pg_config.h` that meson wrote does, the lines that undefine it are
/// blanked the same way. A main file that exists only in memory includes the inspected one
/// and follows it with one `#ifdef` per name the inspected flavor declares, which shows
/// whether a macro of that name is still defined at the end, and how.
fn inspect_other(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    assert_enabled: bool,
) -> Result<Other, FrontendError> {
    let header = &frontend.profile().header;
    let mut arguments = without_assert_checking(&frontend.profile().arguments);
    let mut replaced = BTreeMap::<PathBuf, Vec<String>>::new();
    if assert_enabled {
        for definition in frontend
            .inventory()
            .macros
            .iter()
            .filter(|definition| definition.name == ASSERT_CHECKING)
        {
            let Some(span) = &definition.provenance else { continue };
            if !replaced.contains_key(&span.file) {
                let contents = std::fs::read_to_string(&span.file)
                    .map_err(|source| FrontendError::FileIo { path: span.file.clone(), source })?;
                replaced
                    .insert(span.file.clone(), contents.split('\n').map(str::to_owned).collect());
            }
            let lines = replaced.get_mut(&span.file).expect("inserted above");
            for line in span.start_line..=span.end_line {
                if let Some(text) =
                    usize::try_from(line).ok().and_then(|line| lines.get_mut(line.wrapping_sub(1)))
                {
                    *text = blank(text);
                }
            }
        }
    } else {
        arguments.push(format!("-D{ASSERT_CHECKING}=1"));
    }
    let mut replaced = replaced
        .into_iter()
        .map(|(path, lines)| (path, lines.join("\n")))
        .collect::<BTreeMap<_, _>>();
    // The probes follow the header in a main file that exists only in memory, so they
    // cannot change how the header's own last line is read.
    let mut name = header.file_name().unwrap_or_default().to_owned();
    name.push(".pgrx-build-flavor.h");
    let main_file = header.with_file_name(name);
    let mut main = format!("#include \"{}\"\n", super::c_header_path(header)?);
    let declarations = frontend.declarations();
    let identifier = |name: &&String| {
        let mut bytes = name.bytes();
        bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
            && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    };
    let probed = frontend
        .environment()
        .active
        .keys()
        .chain(frontend.inventory().macros.iter().map(|definition| &definition.name))
        .chain(declarations.function_signatures.keys())
        .chain(declarations.functions.keys())
        .chain(declarations.variables.keys())
        .chain(declarations.types.keys())
        .chain(declarations.integer_constants.keys())
        .filter(identifier)
        .map(String::as_str)
        .chain([ASSERT_CHECKING])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    for name in &probed {
        main.push_str(&format!("#ifdef {name}\n#endif\n"));
    }
    if main.len() > MAX_SOURCE_BYTES {
        return Err(FrontendError::Output(
            "the build flavor probes exceed the 16 MiB source budget".into(),
        ));
    }
    replaced.insert(main_file.clone(), main);
    let directory = std::env::current_dir().map_err(|error| {
        FrontendError::Environment(format!("could not resolve the working directory: {error}"))
    })?;
    let own_skipped = &frontend.flavors.skipped;
    let parse = |replaced: &BTreeMap<PathBuf, String>| {
        let borrowed = replaced
            .iter()
            .map(|(path, contents)| (path.as_path(), contents.as_str()))
            .collect::<Vec<_>>();
        scanner.with_replaced_files(&main_file, &arguments, &borrowed, true, |unit, inventory| {
            let mut skipped = skipped(unit, &directory);
            // The probes skip source of their own.
            skipped.retain(|(file, ..)| *file != main_file);
            let differing = own_skipped.symmetric_difference(&skipped).cloned().collect::<Vec<_>>();
            let Definitions { inline, texts, finals, conditional, records, mut files } =
                definitions(unit, &directory, &main_file, &differing)?;
            let defined = finals.contains_key(ASSERT_CHECKING);
            let spells_keyword = inventory.macros.iter().any(|definition| {
                definition
                    .tokens
                    .iter()
                    .any(|token| token.kind != TokenKind::Comment && token.spelling == BUILD_FLAVOR)
            });
            let mut macros = BTreeMap::new();
            for definition in inventory.macros {
                // A name the inspected flavor never mentions has no probe; its last
                // definition stands.
                if probed.binary_search(&definition.name.as_str()).is_err() {
                    macros.insert(definition.name.clone(), definition);
                }
            }
            for (name, definition) in finals {
                // A probe that found a macro without a definition of its own found a
                // builtin, which no build flavor redefines.
                let definition = definition
                    .or_else(|| Some(frontend.environment().active.get(&name)?.definition.clone()));
                macros.extend(definition.map(|definition| (name, definition)));
            }
            // A body is read from a parse without errors, as the inspected flavor's are.
            let sound = !inventory.diagnostics.iter().any(|diagnostic| {
                matches!(diagnostic.severity, DiagnosticSeverity::Error | DiagnosticSeverity::Fatal)
            });
            let mut bodies = BTreeMap::new();
            if sound && !conditional.is_empty() {
                let active = |name: &str| macros.get(name);
                // The driver cannot read this parse's files, so nothing reports where a
                // macro was undefined.
                let history = records.history(&active, &arguments, None);
                for (name, span, definition) in conditional {
                    let Some(path) = definition
                        .get_range()
                        .and_then(|range| range.get_start().get_file_location().file)
                        .map(|file| file.get_path())
                    else {
                        continue;
                    };
                    // The parse read a replaced file's contents from memory.
                    let bytes = match replaced.get(&span.file) {
                        Some(contents) => Some(contents.as_bytes()),
                        None => files
                            .entry(path)
                            .or_insert_with_key(|path| std::fs::read(path).ok())
                            .as_deref(),
                    };
                    let tokens = bytes
                        .and_then(|bytes| definitions::flavor_body(definition, bytes, &history));
                    bodies.extend(tokens.map(|tokens| {
                        let parameters = definitions::parameter_names(definition);
                        (name, Body { parameters, tokens })
                    }));
                }
            }
            let other = Other {
                macros,
                declarations: collect_declarations(unit).0,
                skipped,
                inline_definitions: inline,
                texts,
                bodies,
                spells_keyword,
            };
            Ok((other, defined))
        })
    };
    let mut parsed = parse(&replaced)?;
    if !assert_enabled && !parsed.1 {
        // A header undefined what the command line defined.
        for file in &frontend.profile().inputs.files {
            let Ok(contents) = std::fs::read_to_string(file) else { continue };
            if !contents.lines().any(undefines_assert_checking) {
                continue;
            }
            let blanked = contents
                .split('\n')
                .map(|line| if undefines_assert_checking(line) { blank(line) } else { line.into() })
                .collect::<Vec<_>>()
                .join("\n");
            replaced.insert(file.clone(), blanked);
        }
        parsed = parse(&replaced)?;
    }
    let (other, defined) = parsed;
    if defined == assert_enabled {
        return Err(FrontendError::Environment(format!(
            "{ASSERT_CHECKING} could not be {} to inspect the other build flavor of these headers",
            if assert_enabled { "removed" } else { "defined" }
        )));
    }
    Ok(other)
}

/// What a build flavor's translation unit defines.
struct Definitions<'tu> {
    /// The spans and source text of its static inline function definitions.
    inline: BTreeMap<String, (SourceSpan, Option<String>)>,
    /// The source text of each function-like macro's last definition, from its name.
    texts: BTreeMap<String, String>,
    /// The definition of each probed name that is a macro at the end of the inspected
    /// main file, when the compiler records one.
    finals: BTreeMap<String, Option<MacroDefinition>>,
    /// The static inline function definitions that hold source only one flavor's
    /// preprocessor skips, each with its name and span.
    conditional: Vec<(String, SourceSpan, clang::Entity<'tu>)>,
    /// The unit's preprocessing records, which prove a body's macros unchanged.
    records: definitions::Records<'tu>,
    /// The contents of the files that source text was read from, by Clang's path.
    files: BTreeMap<PathBuf, Option<Vec<u8>>>,
}

/// Collect a build flavor's definitions. Each `#ifdef` probe in `main_file` whose name is a
/// macro refers to the definition in force there. `differing` lists the source that only
/// one flavor's preprocessor skips.
fn definitions<'tu>(
    unit: &'tu clang::TranslationUnit<'_>,
    directory: &Path,
    main_file: &Path,
    differing: &[Skipped],
) -> Result<Definitions<'tu>, crate::Error> {
    use clang::{EntityKind, EntityVisitResult, StorageClass};
    let mut result = Definitions {
        inline: BTreeMap::new(),
        texts: BTreeMap::new(),
        finals: BTreeMap::new(),
        conditional: Vec::new(),
        records: definitions::Records::default(),
        files: BTreeMap::new(),
    };
    let mut error = None;
    unit.get_entity().visit_children(|entity, _| {
        // Only a definition that differs between the flavors asks for the records.
        if !differing.is_empty() {
            result.records.record(entity);
        }
        let (Some(name), Some(range)) = (entity.get_name(), entity.get_range()) else {
            return EntityVisitResult::Continue;
        };
        match entity.get_kind() {
            EntityKind::FunctionDecl
                if entity.is_definition()
                    && entity.is_inline_function()
                    && entity.get_storage_class() == Some(StorageClass::Static) =>
            {
                if let Ok(Some(span)) = SourceSpan::from_range(range, directory) {
                    let text = text(range, &mut result.files)
                        .filter(|text| text.len() <= MAX_DEFINITION_BYTES);
                    if overlaps(differing, &span) {
                        result.conditional.push((name.clone(), span.clone(), entity));
                    }
                    result.inline.insert(name, (span, text));
                }
            }
            EntityKind::MacroDefinition if entity.is_function_like_macro() => {
                match text(range, &mut result.files) {
                    Some(text) => result.texts.insert(name, text),
                    None => result.texts.remove(&name),
                };
            }
            EntityKind::MacroExpansion
                if range
                    .get_start()
                    .get_file_location()
                    .file
                    .is_some_and(|file| file.get_path() == main_file) =>
            {
                let definition = entity
                    .get_reference()
                    .filter(|definition| definition.get_kind() == EntityKind::MacroDefinition)
                    .map(|definition| crate::macro_definition(definition, directory));
                match definition.transpose() {
                    Ok(definition) => {
                        result.finals.insert(name, definition);
                    }
                    Err(cause) => {
                        error = Some(cause);
                        return EntityVisitResult::Break;
                    }
                }
            }
            _ => {}
        }
        EntityVisitResult::Continue
    });
    match error {
        Some(error) => Err(error),
        None => Ok(result),
    }
}

/// The source text of a range within one file, when it is UTF-8.
fn text(
    range: clang::source::SourceRange<'_>,
    files: &mut BTreeMap<PathBuf, Option<Vec<u8>>>,
) -> Option<String> {
    let start = range.get_start().get_file_location();
    let end = range.get_end().get_file_location();
    let file = start.file.filter(|file| Some(*file) == end.file)?;
    let contents = files
        .entry(file.get_path())
        .or_insert_with_key(|path| std::fs::read(path).ok())
        .as_deref()?;
    let text =
        contents.get(usize::try_from(start.offset).ok()?..usize::try_from(end.offset).ok()?)?;
    String::from_utf8(text.to_vec()).ok()
}

/// A line of spaces as long as `line`, so that blanking it moves no later byte of its file
/// and source text can still be read from the file at the offsets the compiler reports.
fn blank(line: &str) -> String {
    " ".repeat(line.len())
}

/// Whether a source line is a directive that undefines `USE_ASSERT_CHECKING`.
fn undefines_assert_checking(line: &str) -> bool {
    let Some(directive) = line.trim_start().strip_prefix('#') else { return false };
    let mut words = directive.split_ascii_whitespace();
    words.next() == Some("undef") && words.next() == Some(ASSERT_CHECKING)
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

/// The first name an assert-enabled macro's expansion reaches, through the macros it names,
/// that a release build cannot share, with why.
fn reaches<'a>(
    assert: View<'a>,
    root: &'a str,
    unshared: &impl Fn(&str) -> Option<&'static str>,
) -> Option<(&'a str, &'static str)> {
    let mut pending = vec![root];
    let mut visited = BTreeSet::new();
    while let Some(name) = pending.pop() {
        if !visited.insert(name) {
            continue;
        }
        let Some(definition) = assert.macros.get(name) else { continue };
        let (parameters, body) = parameters_and_body(definition);
        for token in body {
            if parameters.contains(token) || token == name {
                continue;
            }
            if let Some(reason) = unshared(token) {
                return Some((token, reason));
            }
            if assert.macros.contains_key(token) {
                pending.push(token);
            }
        }
    }
    None
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
fn parameters_and_body(definition: &MacroDefinition) -> (BTreeSet<&str>, Vec<&str>) {
    let mut tokens = significant(definition);
    tokens.drain(..tokens.len().min(1));
    if definition.kind == MacroKind::ObjectLike {
        return (BTreeSet::new(), tokens);
    }
    let close =
        tokens.iter().position(|token| *token == ")").unwrap_or(tokens.len().saturating_sub(1));
    let body = tokens.split_off((close + 1).min(tokens.len()));
    let parameters =
        tokens.into_iter().skip(1).filter(|token| !matches!(*token, "," | ")")).collect();
    (parameters, body)
}

/// A function-like definition's parameter names, in order.
fn formals(definition: &MacroDefinition) -> Vec<String> {
    let tokens = significant(definition);
    let tokens = tokens.get(1..).unwrap_or_default();
    let close = tokens.iter().position(|token| *token == ")").unwrap_or(tokens.len());
    tokens
        .get(1..close)
        .unwrap_or_default()
        .iter()
        .filter(|token| **token != ",")
        .map(|token| (*token).to_owned())
        .collect()
}

/// The `#define` text of a function-like definition's tokens, or `None` when it has no
/// parameter list.
fn define(definition: &MacroDefinition) -> Option<String> {
    let tokens = significant(definition);
    let (name, rest) = tokens.split_first()?;
    let close = rest.iter().position(|token| *token == ")")?;
    if rest.first() != Some(&"(") {
        return None;
    }
    Some(format!("{name}{} {}", rest[..=close].concat(), rest[close + 1..].join(" ")))
}

#[cfg(test)]
mod tests {
    use super::{Inert, View, closed, nothing, replacement, statement};
    use crate::{DeclarationCatalog, MacroDefinition, MacroKind, Token, TokenKind};
    use std::collections::BTreeMap;

    /// A definition from its tokens' spellings, the macro's name first.
    fn definition(kind: MacroKind, spellings: &[&str]) -> MacroDefinition {
        MacroDefinition {
            name: spellings[0].into(),
            kind,
            location: None,
            provenance: None,
            tokens: spellings
                .iter()
                .map(|spelling| Token { kind: TokenKind::Identifier, spelling: (*spelling).into() })
                .collect(),
            builtin: false,
            main_file: false,
        }
    }

    /// The macros of one build flavor, by name.
    fn macros(definitions: Vec<MacroDefinition>) -> BTreeMap<String, MacroDefinition> {
        definitions.into_iter().map(|definition| (definition.name.clone(), definition)).collect()
    }

    /// A release macro does nothing only when it names no operand, function or variable
    /// and writes nothing, also through the object-like macros it names.
    #[test]
    fn a_release_macro_does_nothing_only_without_operands_calls_and_writes() {
        let macros = macros(vec![
            definition(MacroKind::ObjectLike, &["true", "1"]),
            definition(MacroKind::ObjectLike, &["NOTHING", "(", "(", "void", ")", "true", ")"]),
            definition(MacroKind::ObjectLike, &["COUNTED", "counter", "++"]),
            definition(MacroKind::ObjectLike, &["LOOP", "LOOP"]),
            definition(MacroKind::FunctionLike, &["CALLED", "(", ")", "0"]),
        ]);
        let declarations = DeclarationCatalog::default();
        let view = View { macros: &macros, declarations: &declarations };
        for (replacement, yields) in [
            (vec!["F", "(", "x", ")"], Inert::Nothing),
            (vec!["F", "(", "x", ")", "(", "(", "void", ")", "true", ")"], Inert::Nothing),
            (vec!["F", "(", "x", ")", "(", "(", "void", ")", "0", ")"], Inert::Nothing),
            (
                vec!["F", "(", "a", ",", "b", ")", "do", "{", "}", "while", "(", "0", ")"],
                Inert::Nothing,
            ),
            (vec!["F", "(", ")", "NOTHING"], Inert::Nothing),
            (vec!["F", "(", "x", ")", "(", "true", ")"], Inert::Integer(1)),
        ] {
            let definition = definition(MacroKind::FunctionLike, &replacement);
            assert_eq!(nothing(view, &definition), Some(yields), "{replacement:?}");
        }
        for work in [
            // It evaluates its operand, which may have an effect.
            vec!["F", "(", "x", ")", "(", "(", "void", ")", "(", "x", ")", ")"],
            vec!["F", "(", "x", ")", "(", "*", "(", "x", ")", "=", "0", ")"],
            vec!["F", "(", ")", "counter", "++"],
            vec!["F", "(", ")", "function", "(", ")"],
            vec!["F", "(", ")", "COUNTED"],
            vec!["F", "(", ")", "CALLED", "(", ")"],
            vec!["F", "(", ")", "(", "(", "void", ")", "\"text\"", ")"],
            // A macro that names itself leaves an identifier, which names something.
            vec!["F", "(", ")", "LOOP"],
            // An operand is an operand, whatever the parameter is called.
            vec!["F", "(", "true", ")", "(", "(", "void", ")", "true", ")"],
            // This loop never ends.
            vec!["F", "(", ")", "do", "{", "}", "while", "(", "1", ")"],
        ] {
            let definition = definition(MacroKind::FunctionLike, &work);
            assert_eq!(nothing(view, &definition), None, "{work:?}");
        }
    }

    /// A definition's parameters and replacement list are told apart, and a callee is
    /// closed when no macro's replacement list can open it.
    #[test]
    fn a_callee_is_closed_through_the_macros_it_names() {
        let macros = macros(vec![
            definition(MacroKind::FunctionLike, &["GROUPED", "(", "x", ")", "(", "x", ")"]),
            definition(MacroKind::FunctionLike, &["OPEN", "(", "x", ")", "x", "+", "1"]),
            definition(
                MacroKind::FunctionLike,
                &["CALLS", "(", "x", ")", "GROUPED", "(", "x", ")"],
            ),
            definition(MacroKind::FunctionLike, &["OPENS", "(", "x", ")", "OPEN", "(", "x", ")"]),
            definition(MacroKind::FunctionLike, &["SELF", "(", ")", "SELF", "(", ")"]),
            definition(MacroKind::ObjectLike, &["OBJECT", "(", "1", ")"]),
        ]);
        let declarations = DeclarationCatalog::default();
        let view = View { macros: &macros, declarations: &declarations };
        let (formals, body) = replacement(&macros["CALLS"]).unwrap();
        assert_eq!(formals, ["x"]);
        assert_eq!(
            body.iter().map(|token| token.spelling.as_str()).collect::<Vec<_>>(),
            ["GROUPED", "(", "x", ")"]
        );
        assert_eq!(replacement(&macros["SELF"]).unwrap().0, Vec::<&str>::new());
        assert!(replacement(&macros["OBJECT"]).is_none());
        for name in ["function", "GROUPED", "CALLS"] {
            assert!(closed(view, name, 0), "{name}");
        }
        for name in ["OPEN", "OPENS", "SELF", "OBJECT"] {
            assert!(!closed(view, name, 0), "{name}");
        }
    }

    /// A macro is one statement when its replacement list is, or invokes a macro that is.
    #[test]
    fn a_macro_is_a_statement_through_the_macros_it_invokes() {
        let looped = ["do", "{", "work", "(", "x", ")", ";", "}", "while", "(", "0", ")"];
        let macros = macros(vec![
            definition(MacroKind::FunctionLike, &[&["LOOP", "(", "x", ")"][..], &looped].concat()),
            definition(MacroKind::FunctionLike, &["INVOKES", "(", "x", ")", "LOOP", "(", "x", ")"]),
            definition(
                MacroKind::FunctionLike,
                &["AFTER", "(", "x", ")", "LOOP", "(", "x", ")", ";"],
            ),
            definition(MacroKind::FunctionLike, &["APPLIES", "(", "x", ")", "x", "(", "1", ")"]),
            definition(MacroKind::FunctionLike, &["VALUE", "(", "x", ")", "(", "x", ")"]),
        ]);
        let declarations = DeclarationCatalog::default();
        let view = View { macros: &macros, declarations: &declarations };
        for name in ["LOOP", "INVOKES"] {
            assert!(statement(view, name, 0), "{name}");
        }
        for name in ["AFTER", "APPLIES", "VALUE", "function"] {
            assert!(!statement(view, name, 0), "{name}");
        }
    }
}
