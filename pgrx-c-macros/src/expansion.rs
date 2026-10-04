//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compiler-owned expansion of a bounded batch of symbolic macro invocations.
//!
//! Fresh identifiers stand for arguments without adding parentheses. Clang performs
//! argument prescan, replacement-list rescan, and recursive-macro suppression. The
//! resulting token stream retains every surviving argument occurrence; expression
//! analysis must prove grouping from that stream rather than from the probe itself.

//! Expansion uses the inspected compiler rather than recreating C preprocessing in Rust.
//! Fresh formal markers preserve surviving argument occurrences, while bounded dependency
//! inspection checks provenance and constructs probe sources. Independent passes verify
//! closed token pasting and atomic constant retention. Temporary overlays preserve the main
//! file context, and original-environment checks reject observations from changed inputs.

use crate::{
    ActiveProvenance, BuildInputs, FrontendError, FrontendOutput, MacroDefinition, MacroKind,
    MacroScanner, SourceSpan, TokenKind,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::Read;
use std::path::{Path, PathBuf};

/// Retain readable object-macro symbols only after independent atomicity, type, value, and
/// token-restoration proof.
mod constants;
/// Prove closed token pasting and attribute synthesized dependencies with bounded owned
/// instrumentation.
mod paste;
pub(crate) use constants::retain_integer_constants;
pub use constants::{ObjectIntegerConstants, probe_integer_object_constants};

/// Probe-only diagnostic policy that accepts source markers and makes invalid token pasting an error.
const PROBE_PRAGMAS: &str = "\n\n#pragma clang diagnostic ignored \"-Wgnu-line-marker\"\n#pragma clang diagnostic error \"-Winvalid-token-paste\"\n";

/// Reserved prefix seed used to choose collision-free expansion marker names.
const NAMESPACE: &str = "__pgrx_c_expand_";

/// Compiler-expanded results and newly discovered dependencies for one coherent preparation batch.
#[derive(Clone, Debug, Serialize)]
pub struct ExpansionBatch {
    /// Selected macro names paired with complete expansions or explicit preprocessing refusals.
    pub results: BTreeMap<String, ExpansionResult>,
    /// Additional macro and integer-constant references observed during preprocessing.
    /// Empty entries identify candidates checked for closed token pasting.
    pub discovered_dependencies: BTreeMap<String, BTreeSet<String>>,
    /// Expansion consumes the inspected inputs; temporary probe files are not build inputs.
    pub inputs: BuildInputs,
}

/// Separate owned successful expansions from per-macro preprocessing refusals.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ExpansionResult {
    /// The matched compiler supplied a complete owned symbolic expansion.
    Expanded {
        /// The owned authoritative compiler expansion admitted to symbolic analysis.
        expansion: ExpandedMacro,
    },
    /// A required proof or supported construct is missing, with an explicit refusal attached.
    Skipped {
        /// The structured proof gap that prevents this candidate from being treated as supported.
        reason: ExpansionSkip,
    },
}

/// An original function signature and provenance paired with authoritative compiler-expanded
/// replacement tokens.
#[derive(Clone, Debug, Serialize)]
pub struct ExpandedMacro {
    /// Selected original macro identifier used to index results, retained symbols, and
    /// source-level proofs.
    pub name: String,
    /// Original signature/provenance with the compiler-expanded replacement tokens.
    pub definition: MacroDefinition,
    /// Original C labels, indexed consistently with the fresh symbolic formals.
    pub parameters: Vec<String>,
    /// Fresh marker identifiers indexed with original formals, preventing accidental identifier
    /// capture.
    pub symbolic_parameters: Vec<String>,
    /// Every surviving compiler-expanded formal occurrence needed to preserve argument evaluation
    /// multiplicity.
    pub occurrences: Vec<ParameterOccurrence>,
    /// Conservative closure, including object-like/external context definitions.
    /// This establishes possible origins, not an exact per-token source map.
    pub dependencies: Vec<ExpansionDependency>,
    /// Object dependencies that remain expanded, with the reason symbol retention
    /// could not establish the same C expression semantics.
    pub constant_fallbacks: Vec<ConstantFallback>,
}

/// Why an object-macro dependency could not become an atomic constant reference.
#[derive(Clone, Debug, Serialize)]
pub struct ConstantFallback {
    /// The final active C object-macro name.
    pub name: String,
    /// The missing proof or unsupported construct that required compiler expansion.
    pub reason: String,
}

/// One surviving symbolic argument occurrence, used to preserve C evaluation multiplicity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ParameterOccurrence {
    /// Original formal index represented by this surviving symbolic occurrence.
    pub parameter: usize,
    /// Index within the expanded replacement list, excluding the macro signature.
    pub token: usize,
}

/// A conservative macro origin observed during expansion, including context definitions not emitted
/// as public macros.
#[derive(Clone, Debug, Serialize)]
pub struct ExpansionDependency {
    /// Final active dependency name admitted by closure inspection or compiler discovery.
    pub name: String,
    /// Whether the observed dependency is function-like or object-like expansion context.
    pub kind: MacroKind,
    /// Physical source origins or their resolution status used for ownership filtering and auditing.
    pub provenance: Option<SourceSpan>,
}

/// Explain why a selected macro could not yield a reliable compiler-expanded symbolic invocation.
#[derive(Clone, Debug, Serialize)]
pub struct ExpansionSkip {
    /// Stable machine-readable skip classification, independent of the explanatory text.
    pub code: ExpansionSkipCode,
    /// Human-readable detail explaining the compiler observation or unsupported construct.
    pub message: String,
    /// Immediate referenced macro that prevented expansion, when the refusal has a known owner.
    pub dependency: Option<String>,
    /// Possible physical source ranges supporting the refusal or dependency explanation.
    pub spans: Vec<SourceSpan>,
}

/// Stable preprocessing failure families that session analysis translates into public skip
/// categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpansionSkipCode {
    /// The selected name is absent from the final active preprocessing environment.
    NotActive,
    /// The selected definition has no function-style parameter list and is retained only as expansion
    /// context.
    NotFunctionLike,
    /// An explicitly selected object expression is defined as a function-style macro.
    NotObjectLike,
    /// The original macro signature cannot establish an ordinary bounded formal list.
    MalformedParameters,
    /// An open macro or callable argument tail falls outside the supported substitution contract.
    Variadic,
    /// Token pasting lacks the independent closed-expansion and dependency proof required for
    /// support.
    TokenPaste,
    /// Stringified preprocessing spelling cannot be preserved by the accepted Rust argument contract.
    Stringification,
    /// A context-dependent compiler builtin cannot be frozen into a faithful generated definition.
    DynamicBuiltin,
    /// Multiple physical definitions match the final active body, so unique source ownership is
    /// unproved.
    ProvenanceAmbiguous,
    /// No physical discovery definition could be linked reliably to the final active body.
    ProvenanceUnresolved,
    /// Source, token, dependency, depth, or compiler-pass work exceeded a finite configured limit.
    BudgetExceeded,
    /// The matched compiler rejected a required typed or preprocessing witness.
    CompilerRejected,
    /// Compiler output lacks complete, unambiguous probe boundaries or proof diagnostics.
    UnrecognizedOutput,
}

/// Limits apply to source construction, dependency inspection, and expanded output.
#[derive(Clone, Copy, Debug)]
pub struct ExpansionLimits {
    /// Maximum selected macro candidates admitted to one expansion batch.
    pub macros: usize,
    /// Maximum instrumented source bytes allocated for a bounded expansion pass.
    pub source_bytes: usize,
    /// Maximum preprocessor output size accepted for body extraction.
    pub expanded_bytes: usize,
    /// Maximum replacement tokens admitted for any one macro.
    pub macro_tokens: usize,
    /// Maximum expanded tokens retained across the selected batch.
    pub total_tokens: usize,
    /// Maximum tokens inspected across dependency closure discovery.
    pub dependency_tokens: usize,
    /// Maximum distinct dependencies admitted to one macro closure.
    pub dependencies_per_macro: usize,
}

/// Provide bounded preparation budgets for ordinary library and CLI callers.
impl Default for ExpansionLimits {
    /// Provide finite source, token, macro-count, and dependency budgets for normal preparation.
    fn default() -> Self {
        Self {
            macros: 8192,
            source_bytes: 1024 * 1024,
            expanded_bytes: 8 * 1024 * 1024,
            macro_tokens: 8192,
            total_tokens: 262_144,
            dependency_tokens: 1_048_576,
            dependencies_per_macro: 4096,
        }
    }
}

/// Expand a batch under the inspected profile, preserving original definition history.
pub fn prepare_expansions(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    names: &[impl AsRef<str>],
) -> Result<ExpansionBatch, FrontendError> {
    prepare_expansions_with_limits(scanner, frontend, names, ExpansionLimits::default())
}

/// Expand selected final active macros with caller budgets and record independently owned results or
/// skips.
pub fn prepare_expansions_with_limits(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    names: &[impl AsRef<str>],
    limits: ExpansionLimits,
) -> Result<ExpansionBatch, FrontendError> {
    verify_environment(frontend)?;
    crate::frontend::verify_input_files(&frontend.profile().inputs)?;
    let result = prepare_inner(scanner, frontend, names, limits);
    crate::frontend::verify_input_files(&frontend.profile().inputs)?;
    verify_environment(frontend)?;
    result
}

/// The caller must verify the inspected inputs before and after this compiler phase.
pub(crate) fn prepare_inner(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    names: &[impl AsRef<str>],
    limits: ExpansionLimits,
) -> Result<ExpansionBatch, FrontendError> {
    prepare_inner_with_objects(scanner, frontend, names, &BTreeSet::new(), limits)
}

/// Prepare function roots and an explicitly selected set of object-expression roots.
pub(crate) fn prepare_inner_with_objects(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    names: &[impl AsRef<str>],
    objects: &BTreeSet<String>,
    limits: ExpansionLimits,
) -> Result<ExpansionBatch, FrontendError> {
    let mut results = BTreeMap::new();
    let mut discovered_dependencies = BTreeMap::new();
    let mut dependencies = Dependencies { frontend, nodes: HashMap::new(), tokens: 0, limits };
    let names = names.iter().map(|name| name.as_ref().to_owned()).collect::<BTreeSet<_>>();
    let file = std::fs::File::open(&frontend.profile().header).map_err(|source| {
        FrontendError::CompilerIo { compiler: frontend.profile().header.clone(), source }
    })?;
    let read_limit = u64::try_from(limits.source_bytes).unwrap_or(u64::MAX).saturating_add(1);
    let mut original = Vec::new();
    file.take(read_limit).read_to_end(&mut original).map_err(|source| {
        FrontendError::CompilerIo { compiler: frontend.profile().header.clone(), source }
    })?;
    if original.len().checked_add(2).is_none_or(|bytes| bytes > limits.source_bytes) {
        for name in names {
            let code = if frontend.environment().active.contains_key(&name) {
                ExpansionSkipCode::BudgetExceeded
            } else {
                ExpansionSkipCode::NotActive
            };
            record_skip(
                &mut results,
                &name,
                skip(
                    code,
                    "original main file and probe separators exceed the source byte budget",
                    Some(&name),
                    frontend,
                ),
                frontend,
            );
        }
        return Ok(ExpansionBatch {
            results,
            discovered_dependencies,
            inputs: frontend.profile().inputs.clone(),
        });
    }
    let directory = ProbeDirectory::new()?;
    let prefix = namespace(frontend);
    let mut source_bytes = original.len();
    // Separate a possible final splice/comment, and enforce the validation
    // diagnostic after any header pragmas under MS compatibility modes too.
    source_bytes = source_bytes.saturating_add(PROBE_PRAGMAS.len());
    let mut prepared = Vec::new();
    for name in names {
        let Some((active_name, active)) = frontend.environment().active.get_key_value(&name) else {
            record_skip(
                &mut results,
                &name,
                skip(ExpansionSkipCode::NotActive, "macro is not active", None, frontend),
                frontend,
            );
            continue;
        };
        let object = objects.contains(&name);
        let expected = if object { MacroKind::ObjectLike } else { MacroKind::FunctionLike };
        if active.definition.kind != expected {
            record_skip(
                &mut results,
                &name,
                skip(
                    if object {
                        ExpansionSkipCode::NotObjectLike
                    } else {
                        ExpansionSkipCode::NotFunctionLike
                    },
                    if object {
                        "selected object expression is not an object-like macro"
                    } else {
                        "only function-like macros are primary expansion candidates"
                    },
                    Some(&name),
                    frontend,
                ),
                frontend,
            );
            continue;
        }
        if prepared.len() >= limits.macros || active.definition.tokens.len() > limits.macro_tokens {
            record_skip(
                &mut results,
                &name,
                skip(
                    ExpansionSkipCode::BudgetExceeded,
                    "macro count or original token budget exceeded",
                    Some(&name),
                    frontend,
                ),
                frontend,
            );
            continue;
        }
        let (parameters, body_start) = match parameters(&active.definition) {
            Ok(parameters) => parameters,
            Err((code, message)) => {
                record_skip(
                    &mut results,
                    &name,
                    skip(code, message, Some(&name), frontend),
                    frontend,
                );
                continue;
            }
        };
        if active.definition.tokens[body_start..]
            .iter()
            .any(|token| matches!(token.spelling.as_str(), "#" | "%:"))
        {
            record_skip(
                &mut results,
                &name,
                skip(
                    ExpansionSkipCode::Stringification,
                    "root stringification requires a C preprocessing operand-spelling contract",
                    Some(&name),
                    frontend,
                ),
                frontend,
            );
            continue;
        }
        let (closure, pastes) = match dependencies.closure(active_name) {
            Ok(closure) => closure,
            Err(reason) => {
                record_skip(&mut results, &name, reason, frontend);
                continue;
            }
        };
        let index = prepared.len();
        let markers = parameters
            .iter()
            .enumerate()
            .map(|(parameter, _)| format!("{prefix}parameter_{index}_{parameter}"))
            .collect::<Vec<_>>();
        let begin = format!("{prefix}begin_{index}");
        let end = format!("{prefix}end_{index}");
        let tag = probe_tag(&directory.0, index)?;
        // GNU markers without flags reset inherited system-header status.
        let invocation = format!(
            "# 1 {tag:?}\n{begin}\n{}\n{end}\n",
            symbolic_invocation(&active.definition, &markers)
        );
        if source_bytes.saturating_add(invocation.len()) > limits.source_bytes {
            record_skip(
                &mut results,
                &name,
                skip(
                    ExpansionSkipCode::BudgetExceeded,
                    "symbolic invocation source exceeds its byte budget",
                    Some(&name),
                    frontend,
                ),
                frontend,
            );
            continue;
        }
        source_bytes += invocation.len();
        prepared.push(Prepared {
            definition: &active.definition,
            parameters,
            body_start,
            markers,
            begin,
            end,
            dependencies: closure,
            pastes,
        });
    }
    let inputs = frontend.profile().inputs.clone();
    if prepared.is_empty() {
        return Ok(ExpansionBatch { results, discovered_dependencies, inputs });
    }
    verify_original_environment(scanner, frontend, &directory.0, &original, &inputs)?;
    let proof = paste::prove(frontend, &directory.0, &original, &prefix, &prepared, limits)?;
    let mut rejected = proof.rejected;
    for (&index, discovered) in &proof.dependencies {
        let item = &mut prepared[index];
        discovered_dependencies.insert(item.definition.name.clone(), discovered.clone());
        let mut merged = item
            .dependencies
            .iter()
            .map(|dependency| (dependency.name.clone(), dependency.clone()))
            .collect::<BTreeMap<_, _>>();
        for name in discovered {
            let (active_name, active) = frontend
                .environment()
                .active
                .get_key_value(name)
                .expect("paste proof only returns active macro identifiers");
            match dependencies.closure(active_name) {
                Ok((closure, _)) => {
                    merged.insert(
                        name.clone(),
                        ExpansionDependency {
                            name: name.clone(),
                            kind: active.definition.kind,
                            provenance: active.definition.provenance.clone(),
                        },
                    );
                    merged.extend(
                        closure.into_iter().map(|dependency| (dependency.name.clone(), dependency)),
                    );
                }
                Err(reason) => {
                    record_skip(&mut results, &item.definition.name, reason, frontend);
                    rejected.insert(
                        index,
                        (
                            ExpansionSkipCode::CompilerRejected,
                            "synthesized dependency failed admission".into(),
                        ),
                    );
                    break;
                }
            }
        }
        if merged.len().saturating_add(1) > limits.dependencies_per_macro {
            rejected.insert(
                index,
                (
                    ExpansionSkipCode::BudgetExceeded,
                    "synthesized macro dependency closure exceeds its budget".into(),
                ),
            );
        }
        item.dependencies = merged.into_values().collect();
    }
    for (&index, (code, message)) in &rejected {
        let item = &prepared[index];
        // Keep the precise helper provenance rejection from closure inspection.
        if !results.contains_key(&item.definition.name) {
            record_skip(
                &mut results,
                &item.definition.name,
                skip(*code, message, Some(&item.definition.name), frontend),
                frontend,
            );
        }
    }
    prepared = prepared
        .into_iter()
        .enumerate()
        .filter_map(|(index, item)| (!rejected.contains_key(&index)).then_some(item))
        .collect();
    let Some((prepared, output)) = preprocess_prepared(
        frontend,
        &directory.0,
        &original,
        prepared,
        limits,
        proof.runs,
        &mut results,
    )?
    else {
        return Ok(ExpansionBatch { results, discovered_dependencies, inputs });
    };
    let bodies = match extract_bodies(&output, &prepared, limits.expanded_bytes) {
        Ok(bodies) => bodies,
        Err((code, message)) => {
            skip_prepared(&mut results, &prepared, code, message, frontend);
            return Ok(ExpansionBatch { results, discovered_dependencies, inputs });
        }
    };
    let mut snapshot = String::new();
    for (index, body) in bodies.iter().enumerate() {
        snapshot.push_str(&format!("#define {prefix}tokens_{index} {body}\n"));
    }
    let definitions =
        crate::frontend::tokenize_snapshot(scanner, &snapshot, &frontend.profile().arguments)?;
    let mut definitions = definitions
        .into_iter()
        .map(|definition| (definition.name.clone(), definition))
        .collect::<HashMap<_, _>>();
    let mut token_count = 0usize;
    for (index, item) in prepared.into_iter().enumerate() {
        let Some(definition) = definitions.remove(&format!("{prefix}tokens_{index}")) else {
            record_skip(
                &mut results,
                &item.definition.name,
                skip(
                    ExpansionSkipCode::UnrecognizedOutput,
                    "tokenization did not retain the symbolic replacement",
                    Some(&item.definition.name),
                    frontend,
                ),
                frontend,
            );
            continue;
        };
        let mut body = definition.tokens.into_iter().skip(1).collect::<Vec<_>>();
        token_count = token_count.saturating_add(body.len());
        if body.len() > limits.macro_tokens || token_count > limits.total_tokens {
            record_skip(
                &mut results,
                &item.definition.name,
                skip(
                    ExpansionSkipCode::BudgetExceeded,
                    "expanded replacement exceeds its token budget",
                    Some(&item.definition.name),
                    frontend,
                ),
                frontend,
            );
            continue;
        }
        let markers = item
            .markers
            .iter()
            .enumerate()
            .map(|(index, marker)| (marker.as_str(), index))
            .collect::<HashMap<_, _>>();
        // Clang owns dependency stringification and its surrounding template.
        // Current formal markers can retain Rust invocation spelling later;
        // invocation builtins inside a string still have no such contract.
        if body.iter().any(|token| {
            token.kind == TokenKind::Literal
                && token.spelling.contains(&prefix)
                && !formal_string(&token.spelling, &item.markers, &prefix)
        }) {
            record_skip(
                &mut results,
                &item.definition.name,
                skip(
                    ExpansionSkipCode::Stringification,
                    "dependency stringification contains unsupported invocation context",
                    Some(&item.definition.name),
                    frontend,
                ),
                frontend,
            );
            continue;
        }
        restore_invocation_diagnostics(&mut body, &prefix);
        let mut occurrences = Vec::new();
        for (index, token) in body.iter().enumerate() {
            if let Some(&parameter) = markers.get(token.spelling.as_str()) {
                occurrences.push(ParameterOccurrence { parameter, token: index });
            }
        }
        if body.iter().any(|token| {
            token.spelling.contains(&prefix)
                && !markers.contains_key(token.spelling.as_str())
                && !(token.kind == TokenKind::Literal
                    && formal_string(&token.spelling, &item.markers, &prefix))
        }) {
            record_skip(
                &mut results,
                &item.definition.name,
                skip(
                    ExpansionSkipCode::UnrecognizedOutput,
                    "unexpected probe identifier survived expansion",
                    Some(&item.definition.name),
                    frontend,
                ),
                frontend,
            );
            continue;
        }
        if let Some(token) = body.iter().enumerate().find_map(|(index, token)| {
            if markers.contains_key(token.spelling.as_str()) {
                return None;
            }
            let modeled = frontend.declarations().builtins.contains_key(&token.spelling)
                || (token.spelling == "__builtin_offsetof"
                    && frontend.profile().target.offsetof_supported);
            let reserved_call = token.spelling.starts_with("__")
                && !frontend.environment().active.contains_key(&token.spelling)
                && !frontend.declarations().functions.contains_key(&token.spelling)
                && !modeled
                && body
                    .iter()
                    .skip(index + 1)
                    .find(|next| next.kind != TokenKind::Comment)
                    .is_some_and(|next| next.spelling == "(");
            ((dynamic_builtin(&token.spelling) && !invocation_diagnostic(&token.spelling))
                || reserved_call)
                .then_some(token)
        }) {
            record_skip(
                &mut results,
                &item.definition.name,
                skip(
                    ExpansionSkipCode::DynamicBuiltin,
                    format!(
                        "expanded {} needs an explicit compiler/preprocessing semantic contract",
                        token.spelling
                    ),
                    Some(&item.definition.name),
                    frontend,
                ),
                frontend,
            );
            continue;
        }
        let mut expanded = item.definition.clone();
        expanded.tokens.truncate(item.body_start);
        let original_parameters = item
            .parameters
            .iter()
            .enumerate()
            .map(|(index, name)| (name.as_str(), index))
            .collect::<HashMap<_, _>>();
        for token in expanded.tokens.iter_mut().skip(1) {
            if let Some(&parameter) = original_parameters.get(token.spelling.as_str()) {
                token.spelling.clone_from(&item.markers[parameter]);
                token.kind = TokenKind::Identifier;
            }
        }
        expanded.tokens.extend(body);
        results.insert(
            item.definition.name.clone(),
            ExpansionResult::Expanded {
                expansion: ExpandedMacro {
                    name: item.definition.name.clone(),
                    definition: expanded,
                    parameters: item.parameters,
                    symbolic_parameters: item.markers,
                    occurrences,
                    dependencies: item.dependencies,
                    constant_fallbacks: Vec::new(),
                },
            },
        );
    }
    Ok(ExpansionBatch { results, discovered_dependencies, inputs })
}

/// Recognize strings containing only this invocation's fresh formal markers.
/// Context markers remain refusals, and syntax analysis still validates literal
/// decoding and the native consumer before admitting diagnostic substitution.
fn formal_string(spelling: &str, markers: &[String], prefix: &str) -> bool {
    let mut remaining = spelling;
    while let Some(position) = remaining.find(prefix) {
        let candidate = &remaining[position..];
        let boundary = |byte: u8| !byte.is_ascii_alphanumeric() && byte != b'_';
        if position != 0 && !boundary(remaining.as_bytes()[position - 1]) {
            return false;
        }
        let Some(marker) = markers.iter().find(|marker| {
            candidate.starts_with(marker.as_str())
                && candidate.as_bytes().get(marker.len()).is_none_or(|byte| boundary(*byte))
        }) else {
            return false;
        };
        remaining = &candidate[marker.len()..];
    }
    true
}

/// Reject changes to recorded compiler environment values before combining observations across
/// phases.
pub(crate) fn verify_environment(frontend: &FrontendOutput) -> Result<(), FrontendError> {
    let directory = std::env::current_dir().map_err(|error| {
        FrontendError::Environment(format!("could not check working directory: {error}"))
    })?;
    if directory != frontend.profile().inputs.current_directory {
        return Err(FrontendError::Environment(
            "working directory changed after inspection".into(),
        ));
    }
    for (name, expected) in &frontend.profile().inputs.environment {
        let actual = match std::env::var(name) {
            Ok(value) => Some(value),
            Err(std::env::VarError::NotPresent) => None,
            Err(std::env::VarError::NotUnicode(_)) => {
                return Err(FrontendError::Environment(format!(
                    "{name} became non-UTF-8 after inspection"
                )));
            }
        };
        if &actual != expected {
            return Err(FrontendError::Environment(format!("{name} changed after inspection")));
        }
    }
    Ok(())
}

/// Choose fresh per-probe boundary markers that cannot collide with the inspected source namespace.
fn probe_tag(directory: &Path, index: usize) -> Result<String, FrontendError> {
    directory
        .join(format!("probe-{index}"))
        .to_str()
        .map(str::to_owned)
        .ok_or_else(|| FrontendError::Arguments("probe path is not UTF-8".into()))
}

// Retention and paste validation must use the same finite marker spelling.
/// Choose a fresh symbolic token for retaining one verified object-macro dependency.
fn constant_marker(prefix: &str, index: usize) -> String {
    format!("{prefix}c{index}")
}

/// Remove only uniquely attributed failures, then require a clean compiler pass.
/// Each retry rebuilds the original main file, so a rejected invocation cannot
/// alter the macro environment for the accepted batch.
fn preprocess_prepared<'a>(
    frontend: &FrontendOutput,
    directory: &Path,
    original: &[u8],
    prepared: Vec<Prepared<'a>>,
    limits: ExpansionLimits,
    mut runs: usize,
    results: &mut BTreeMap<String, ExpansionResult>,
) -> Result<Option<(Vec<Prepared<'a>>, String)>, FrontendError> {
    let mut pending = (0..prepared.len()).collect::<BTreeSet<_>>();
    let path = directory.join("expansion.c");
    let overlay = overlay(directory, &frontend.profile().header, &path)?;
    let mut arguments = paste::normalized_arguments(&frontend.profile().arguments);
    arguments.extend([
        "-ivfsoverlay".into(),
        overlay
            .to_str()
            .ok_or_else(|| FrontendError::Arguments("probe path is not UTF-8".into()))?
            .into(),
    ]);
    while !pending.is_empty() {
        if runs >= paste::MAX_CPP_RUNS {
            for &index in &pending {
                let item = &prepared[index];
                record_skip(
                    results,
                    &item.definition.name,
                    skip(
                        ExpansionSkipCode::BudgetExceeded,
                        "symbolic preprocessing exhausted its bounded batch retries",
                        Some(&item.definition.name),
                        frontend,
                    ),
                    frontend,
                );
            }
            return Ok(None);
        }
        let mut source = original.to_vec();
        source.extend_from_slice(PROBE_PRAGMAS.as_bytes());
        // Preserve invocation diagnostics as symbolic nodes. Redefinition is
        // confined to this verified probe, after the original header; generated
        // code supplies the Rust invocation's filename and line instead.
        source.extend_from_slice(invocation_diagnostic_preamble(&namespace(frontend)).as_bytes());
        let mut locations = BTreeMap::new();
        for &index in &pending {
            let item = &prepared[index];
            let tag = probe_tag(directory, index)?;
            let invocation = format!(
                "# 1 {tag:?}\n{}\n{}\n{}\n",
                item.begin,
                symbolic_invocation(item.definition, &item.markers),
                item.end
            );
            if source.len().saturating_add(invocation.len()) > limits.source_bytes {
                for &index in &pending {
                    let item = &prepared[index];
                    record_skip(
                        results,
                        &item.definition.name,
                        skip(
                            ExpansionSkipCode::BudgetExceeded,
                            "symbolic invocation source exceeds its byte budget",
                            Some(&item.definition.name),
                            frontend,
                        ),
                        frontend,
                    );
                }
                return Ok(None);
            }
            source.extend_from_slice(invocation.as_bytes());
            locations.insert(tag, index);
        }
        std::fs::write(&path, source)
            .map_err(|source| FrontendError::CompilerIo { compiler: path.clone(), source })?;
        runs += 1;
        match crate::frontend::preprocess(
            &frontend.profile().compiler.executable,
            &frontend.profile().header,
            &arguments,
            &["-E", "-P"],
        ) {
            Ok(output) => {
                let selected = prepared
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, item)| pending.contains(&index).then_some(item))
                    .collect();
                return Ok(Some((selected, output.stdout)));
            }
            Err(FrontendError::CompilerFailed { diagnostics, .. }) => {
                let summaries = match paste::failed_probe_summaries(&diagnostics, &locations) {
                    Ok(summaries) if summaries.keys().all(|index| pending.contains(index)) => {
                        summaries
                    }
                    Err(reason) => pending.iter().map(|&index| (index, reason.clone())).collect(),
                    _ => pending
                        .iter()
                        .map(|&index| (index, "Clang returned an unexpected probe owner".into()))
                        .collect(),
                };
                for (index, message) in summaries {
                    let item = &prepared[index];
                    record_skip(
                        results,
                        &item.definition.name,
                        skip(
                            ExpansionSkipCode::CompilerRejected,
                            format!("Clang rejected symbolic preprocessing: {message}"),
                            Some(&item.definition.name),
                            frontend,
                        ),
                        frontend,
                    );
                    pending.remove(&index);
                }
            }
            Err(FrontendError::OutputLimit(_) | FrontendError::Timeout(_)) => {
                for &index in &pending {
                    let item = &prepared[index];
                    record_skip(
                        results,
                        &item.definition.name,
                        skip(
                            ExpansionSkipCode::BudgetExceeded,
                            "symbolic preprocessing exceeded its output or time budget",
                            Some(&item.definition.name),
                            frontend,
                        ),
                        frontend,
                    );
                }
                return Ok(None);
            }
            Err(error) => return Err(error),
        }
    }
    Ok(None)
}

/// Require temporary source/overlay preprocessing to reproduce the original inspected macro
/// environment and inputs.
fn verify_original_environment(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    directory: &Path,
    original: &[u8],
    inputs: &BuildInputs,
) -> Result<(), FrontendError> {
    let header = directory.join("environment.c");
    let dependencies = directory.join("environment.d");
    let mut guarded = original.to_vec();
    guarded.extend_from_slice(b"\n\n");
    std::fs::write(&header, guarded)
        .map_err(|source| FrontendError::CompilerIo { compiler: header.clone(), source })?;
    let overlay = overlay(directory, &frontend.profile().header, &header)?;
    let mut arguments = frontend.profile().arguments.clone();
    arguments.extend([
        "-ivfsoverlay".into(),
        overlay
            .to_str()
            .ok_or_else(|| FrontendError::Arguments("probe path is not UTF-8".into()))?
            .into(),
    ]);
    let dependency_path = dependencies
        .to_str()
        .ok_or_else(|| FrontendError::Arguments("probe path is not UTF-8".into()))?;
    let output = crate::frontend::preprocess(
        &frontend.profile().compiler.executable,
        &frontend.profile().header,
        &arguments,
        &["-E", "-dM", "-v", "-MD", "-MF", dependency_path],
    )?;
    crate::frontend::validate_driver_configuration(&output.stderr)?;
    let triple = crate::frontend::compiler_triple(&output.stderr)?;
    if triple != frontend.profile().target.triple {
        return Err(FrontendError::Environment("compiler target changed after inspection".into()));
    }
    let definitions =
        crate::frontend::tokenize_snapshot(scanner, &output.stdout, &frontend.profile().arguments)?;
    let actual = definitions
        .iter()
        .map(|definition| (definition.name.as_str(), definition))
        .collect::<BTreeMap<_, _>>();
    let expected = &frontend.environment().active;
    if actual.len() != expected.len()
        || actual.iter().any(|(name, definition)| {
            expected.get(*name).is_none_or(|active| {
                significant_tokens(definition) != significant_tokens(&active.definition)
                    || definition.kind != active.definition.kind
            })
        })
    {
        return Err(FrontendError::Environment(
            "original-header overlay changed the final macro environment; inspected inputs differ"
                .into(),
        ));
    }
    let dependency_text = std::fs::read_to_string(&dependencies)
        .map_err(|source| FrontendError::CompilerIo { compiler: dependencies.clone(), source })?;
    let files = inputs.files.iter().cloned().collect::<BTreeSet<_>>();
    for file in crate::frontend::parse_dependencies(&dependency_text)? {
        if file.starts_with(directory) {
            continue;
        }
        let identity = file
            .canonicalize()
            .map_err(|source| FrontendError::CompilerIo { compiler: file.clone(), source })?;
        if !files.contains(&file) || !files.contains(&identity) {
            return Err(FrontendError::Environment(format!(
                "a new header dependency appeared after inspection: {} ({})",
                file.display(),
                identity.display()
            )));
        }
    }
    Ok(())
}

/// Copy only semantic token spellings for exact expansion comparisons, excluding comments.
fn significant_tokens(definition: &MacroDefinition) -> Vec<&str> {
    definition
        .tokens
        .iter()
        .filter(|token| token.kind != TokenKind::Comment)
        .map(|token| token.spelling.as_str())
        .collect()
}

/// Attach a structured refusal to one requested macro while preserving dependency source spans.
fn record_skip(
    results: &mut BTreeMap<String, ExpansionResult>,
    name: &str,
    mut reason: ExpansionSkip,
    frontend: &FrontendOutput,
) {
    if let Some(span) = frontend
        .environment()
        .active
        .get(name)
        .and_then(|active| active.definition.provenance.as_ref())
        && !reason.spans.contains(span)
    {
        reason.spans.push(span.clone());
    }
    reason.spans.sort_by(|left, right| {
        (&left.file, left.start_line, left.end_line).cmp(&(
            &right.file,
            right.start_line,
            right.end_line,
        ))
    });
    reason.spans.dedup();
    results.insert(name.into(), ExpansionResult::Skipped { reason });
}

/// Reject a prepared probe with the dependency and source origins already established during
/// inspection.
fn skip_prepared(
    results: &mut BTreeMap<String, ExpansionResult>,
    prepared: &[Prepared<'_>],
    code: ExpansionSkipCode,
    message: String,
    frontend: &FrontendOutput,
) {
    for item in prepared {
        record_skip(
            results,
            &item.definition.name,
            skip(code, &message, Some(&item.definition.name), frontend),
            frontend,
        );
    }
}

/// Recover each completed boundary-delimited expansion, rejecting missing, repeated, or malformed
/// probe output.
fn extract_bodies(
    output: &str,
    prepared: &[Prepared<'_>],
    limit: usize,
) -> Result<Vec<String>, (ExpansionSkipCode, String)> {
    let begins = prepared
        .iter()
        .enumerate()
        .map(|(index, item)| (item.begin.as_str(), index))
        .collect::<HashMap<_, _>>();
    let ends = prepared
        .iter()
        .enumerate()
        .map(|(index, item)| (item.end.as_str(), index))
        .collect::<HashMap<_, _>>();
    let mut bodies = vec![String::new(); prepared.len()];
    let mut seen = vec![false; prepared.len()];
    let mut completed = 0;
    let mut active = None;
    let mut bytes = 0usize;
    let invalid = || {
        (
            ExpansionSkipCode::UnrecognizedOutput,
            "symbolic expansion boundaries were duplicated, missing, or nested".into(),
        )
    };
    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(&index) = begins.get(trimmed) {
            if active.is_some() || seen[index] {
                return Err(invalid());
            }
            seen[index] = true;
            active = Some(index);
        } else if let Some(&index) = ends.get(trimmed) {
            if active != Some(index) {
                return Err(invalid());
            }
            active = None;
            completed += 1;
        } else if let Some(index) = active {
            bytes = bytes.saturating_add(line.len()).saturating_add(1);
            if bytes > limit {
                return Err((
                    ExpansionSkipCode::BudgetExceeded,
                    "expanded replacement text exceeds its byte budget".into(),
                ));
            }
            bodies[index].push_str(line);
            bodies[index].push(' ');
        }
    }
    if active.is_some() || completed != prepared.len() {
        return Err(invalid());
    }
    Ok(bodies)
}

/// A validated invocation and its fresh boundaries, formals, and dependency proof inputs.
struct Prepared<'a> {
    /// Borrowed original active definition whose signature and provenance remain authoritative.
    definition: &'a MacroDefinition,
    /// Original signature labels aligned with the fresh symbolic formal markers.
    parameters: Vec<String>,
    /// Original token offset at which replacement content begins after the signature.
    body_start: usize,
    /// Fresh symbolic formal identifiers supplied to the driver without added parentheses.
    markers: Vec<String>,
    /// Fresh opening probe boundary that must appear exactly once in accepted output.
    begin: String,
    /// Fresh closing probe boundary that must follow the matching opening boundary.
    end: String,
    /// References established for this phase and used to explain or propagate downstream skips.
    dependencies: Vec<ExpansionDependency>,
    /// Whether the dependency closure contains token pasting and requires independent closure proof.
    pastes: bool,
}

/// Cached local replacement facts used to build bounded macro dependency closures.
struct DependencyNode<'a> {
    /// References established for this phase and used to explain or propagate downstream skips.
    dependencies: Vec<&'a str>,
    /// Cached local preprocessing construct refusal, reused by each dependent closure.
    rejection: Option<ExpansionSkip>,
    /// Whether this local body includes ## and therefore requires paste-proof instrumentation.
    pastes: bool,
}

/// Own the per-batch dependency cache and token accounting while borrowing the inspected environment.
struct Dependencies<'a> {
    /// The coherent inspected environment borrowed by this phase; it is never reconstructed from
    /// snapshots.
    frontend: &'a FrontendOutput,
    /// Cached local macro facts shared by bounded closure discovery across the batch.
    nodes: HashMap<&'a str, DependencyNode<'a>>,
    /// Tokens already charged to the bounded dependency-inspection budget.
    tokens: usize,
    /// Finite source/token/run budgets enforced by this proof or dependency inspection.
    limits: ExpansionLimits,
}

/// Cache local macro facts and bound closure inspection across the prepared batch.
impl<'a> Dependencies<'a> {
    /// Inspect and cache a bounded dependency closure, including nested unsupported preprocessing
    /// constructs.
    fn closure(
        &mut self,
        root: &'a str,
    ) -> Result<(Vec<ExpansionDependency>, bool), ExpansionSkip> {
        let mut pending = vec![root];
        let mut visited = HashSet::new();
        let mut pastes = false;
        while let Some(name) = pending.pop() {
            if !visited.insert(name) {
                continue;
            }
            if visited.len() > self.limits.dependencies_per_macro {
                return Err(skip(
                    ExpansionSkipCode::BudgetExceeded,
                    "macro dependency closure exceeds its budget",
                    Some(name),
                    self.frontend,
                ));
            }
            if !self.nodes.contains_key(name) {
                let definition = &self.frontend.environment().active[name].definition;
                self.tokens = self.tokens.saturating_add(definition.tokens.len());
                if self.tokens > self.limits.dependency_tokens {
                    return Err(skip(
                        ExpansionSkipCode::BudgetExceeded,
                        "batch dependency token budget exceeded",
                        Some(name),
                        self.frontend,
                    ));
                }
                self.nodes.insert(name, inspect_dependency(self.frontend, name));
            }
            let node = &self.nodes[name];
            if let Some(rejection) = &node.rejection {
                return Err(rejection.clone());
            }
            pastes |= node.pastes;
            pending.extend(node.dependencies.iter().copied());
        }
        let mut names = visited.into_iter().filter(|name| *name != root).collect::<Vec<_>>();
        names.sort_unstable();
        Ok((
            names
                .into_iter()
                .map(|name| {
                    let definition = &self.frontend.environment().active[name].definition;
                    ExpansionDependency {
                        name: name.into(),
                        kind: definition.kind,
                        provenance: definition.provenance.clone(),
                    }
                })
                .collect(),
            pastes,
        ))
    }
}

/// Read one active replacement body and record named references, token pastes, and explicit rejection
/// reasons.
fn inspect_dependency<'a>(frontend: &'a FrontendOutput, name: &'a str) -> DependencyNode<'a> {
    let active = &frontend.environment().active[name];
    let reject = |code, message: &str| DependencyNode {
        dependencies: Vec::new(),
        rejection: Some(skip(code, message, Some(name), frontend)),
        pastes: false,
    };
    match &active.provenance {
        ActiveProvenance::Resolved => {}
        ActiveProvenance::Ambiguous(_) => {
            return reject(
                ExpansionSkipCode::ProvenanceAmbiguous,
                "dependency provenance is ambiguous",
            );
        }
        ActiveProvenance::Unresolved => {
            return reject(
                ExpansionSkipCode::ProvenanceUnresolved,
                "dependency provenance is unresolved",
            );
        }
    }
    let (parameters, start) = if active.definition.kind == MacroKind::FunctionLike {
        match parameters(&active.definition) {
            Ok(parameters) => parameters,
            Err((code, message)) => return reject(code, &message),
        }
    } else {
        (Vec::new(), 1)
    };
    if start > active.definition.tokens.len() {
        return reject(
            ExpansionSkipCode::MalformedParameters,
            "dependency has no macro name token",
        );
    }
    let parameters = parameters.iter().map(String::as_str).collect::<HashSet<_>>();
    let mut dependencies = BTreeSet::new();
    let mut pastes = false;
    let body = &active.definition.tokens[start..];
    for (index, token) in body.iter().enumerate() {
        if token.kind == TokenKind::Comment || parameters.contains(token.spelling.as_str()) {
            continue;
        }
        match token.spelling.as_str() {
            "##" | "%:%:" => {
                pastes = true;
                continue;
            }
            "#" | "%:" => {
                // Clang will stringify the substituted dependency operand. The
                // resulting literal is admitted only when no symbolic marker
                // survives inside its spelling.
                continue;
            }
            spelling if dynamic_builtin(spelling) && !invocation_diagnostic(spelling) => {
                return reject(
                    ExpansionSkipCode::DynamicBuiltin,
                    "dependency uses an invocation-sensitive preprocessing builtin",
                );
            }
            _ => {}
        }
        let modeled_intrinsic = frontend.declarations().builtins.contains_key(&token.spelling)
            || (token.spelling == "__builtin_offsetof"
                && frontend.profile().target.offsetof_supported);
        if token.spelling.starts_with("__")
            && !frontend.environment().active.contains_key(&token.spelling)
            && !frontend.declarations().functions.contains_key(&token.spelling)
            && !modeled_intrinsic
            && body
                .iter()
                .skip(index + 1)
                .find(|token| token.kind != TokenKind::Comment)
                .is_some_and(|token| token.spelling == "(")
        {
            return reject(
                ExpansionSkipCode::DynamicBuiltin,
                frontend.declarations().builtin_unavailable.get(&token.spelling).map(String::as_str).unwrap_or("unrecognized reserved compiler/preprocessing invocation needs an explicit semantic contract"),
            );
        }
        if let Some((name, _)) = frontend.environment().active.get_key_value(&token.spelling) {
            dependencies.insert(name.as_str());
        }
    }
    DependencyNode { dependencies: dependencies.into_iter().collect(), rejection: None, pastes }
}

/// Recognize context-sensitive builtins whose changing expansion cannot be represented by a static
/// translation.
fn dynamic_builtin(name: &str) -> bool {
    name.starts_with("__has_")
        || name.starts_with("__is_")
        || matches!(
            name,
            "_Pragma"
                | "__pragma"
                | "__LINE__"
                | "__FILE__"
                | "__FILE_NAME__"
                | "__BASE_FILE__"
                | "__INCLUDE_LEVEL__"
                | "__COUNTER__"
                | "__DATE__"
                | "__TIME__"
                | "__TIMESTAMP__"
                | "__VA_ARGS__"
                | "__VA_OPT__"
                | "__identifier"
                | "__FLT_EVAL_METHOD__"
                | "__is_identifier"
                | "__building_module"
                | "__MODULE__"
                | "__func__"
                | "__FUNCTION__"
                | "__PRETTY_FUNCTION__"
        )
}

/// Invocation-sensitive diagnostics with an explicit Rust source-location contract.
fn invocation_diagnostic(name: &str) -> bool {
    matches!(name, "__FILE__" | "__LINE__")
}

/// Preserve the original object/function invocation shape in independent compiler probes.
pub(super) fn symbolic_invocation(definition: &MacroDefinition, parameters: &[String]) -> String {
    match definition.kind {
        MacroKind::ObjectLike => definition.name.clone(),
        MacroKind::FunctionLike => format!("{}({})", definition.name, parameters.join(",")),
    }
}

/// Defer file/line values to the Rust source invocation in probe-only preprocessing.
pub(super) fn invocation_diagnostic_preamble(prefix: &str) -> String {
    format!(
        "\n#pragma clang diagnostic ignored \"-Wbuiltin-macro-redefined\"\n\
         #undef __FILE__\n#define __FILE__ {prefix}invocation_file\n\
         #undef __LINE__\n#define __LINE__ {prefix}invocation_line\n"
    )
}

/// Restore diagnostic nodes after a compiler pass without altering literal spelling.
pub(super) fn restore_invocation_diagnostics(tokens: &mut [crate::Token], prefix: &str) {
    let file = format!("{prefix}invocation_file");
    let line = format!("{prefix}invocation_line");
    for token in tokens {
        if token.spelling == file {
            token.spelling = "__FILE__".into();
        } else if token.spelling == line {
            token.spelling = "__LINE__".into();
        }
    }
}

/// Parse a macro signature and locate its replacement body without adding probe-side argument
/// grouping.
fn parameters(
    definition: &MacroDefinition,
) -> Result<(Vec<String>, usize), (ExpansionSkipCode, String)> {
    if definition.kind == MacroKind::ObjectLike {
        return Ok((Vec::new(), 1));
    }
    let tokens = &definition.tokens;
    let mut index = 1;
    while tokens.get(index).is_some_and(|token| token.kind == TokenKind::Comment) {
        index += 1;
    }
    if tokens.get(index).is_none_or(|token| token.spelling != "(") {
        return Err((
            ExpansionSkipCode::MalformedParameters,
            "macro signature has no parameter list".into(),
        ));
    }
    index += 1;
    let mut parameters = Vec::new();
    let mut want_parameter = true;
    while let Some(token) = tokens.get(index) {
        index += 1;
        if token.kind == TokenKind::Comment {
            continue;
        }
        if token.spelling == "..." {
            return Err((
                ExpansionSkipCode::Variadic,
                "variadic macros need a separate invocation contract".into(),
            ));
        }
        if token.spelling == ")" && (!want_parameter || parameters.is_empty()) {
            return Ok((parameters, index));
        }
        if want_parameter && matches!(token.kind, TokenKind::Identifier | TokenKind::Keyword) {
            parameters.push(token.spelling.clone());
            want_parameter = false;
        } else if !want_parameter && token.spelling == "," {
            want_parameter = true;
        } else {
            return Err((
                ExpansionSkipCode::MalformedParameters,
                "unsupported macro parameter list".into(),
            ));
        }
    }
    Err((ExpansionSkipCode::MalformedParameters, "unterminated macro parameter list".into()))
}

/// Construct a preprocessing refusal with its direct dependency and possible physical source origins.
fn skip(
    code: ExpansionSkipCode,
    message: impl Into<String>,
    dependency: Option<&str>,
    frontend: &FrontendOutput,
) -> ExpansionSkip {
    let mut spans = Vec::new();
    if let Some(active) = dependency.and_then(|name| frontend.environment().active.get(name)) {
        match &active.provenance {
            ActiveProvenance::Ambiguous(ambiguous) => spans.extend(ambiguous.iter().cloned()),
            _ => spans.extend(active.definition.provenance.iter().cloned()),
        }
    }
    ExpansionSkip {
        code,
        message: message.into(),
        dependency: dependency.map(str::to_owned),
        spans,
    }
}

/// Choose a prefix absent from inspected tokens so generated markers cannot capture original
/// identifiers.
fn namespace(frontend: &FrontendOutput) -> String {
    let mut occupied = HashSet::new();
    for spelling in frontend
        .inventory()
        .macros
        .iter()
        .flat_map(|definition| &definition.tokens)
        .map(|token| token.spelling.as_str())
        .chain(frontend.environment().active.keys().map(String::as_str))
        .chain(frontend.declarations().types.keys().map(String::as_str))
        .chain(frontend.declarations().integer_constants.keys().map(String::as_str))
        .chain(frontend.declarations().variables.keys().map(String::as_str))
        .chain(frontend.declarations().functions.keys().map(String::as_str))
    {
        for (start, _) in spelling.match_indices(NAMESPACE) {
            // Include embedded names: rescan can consume a prefixed argument hole.
            let offset = start + NAMESPACE.len();
            if let Some(number) =
                spelling[offset..].split('_').next().and_then(|number| number.parse::<usize>().ok())
            {
                occupied.insert(number);
            }
        }
    }
    let mut number = 0;
    while occupied.contains(&number) {
        number += 1;
    }
    format!("{NAMESPACE}{number}_")
}

/// Own a unique temporary directory for overlays and staged outputs, removing only its own resources.
struct ProbeDirectory(
    /// Unique owned scratch directory removed on drop.
    PathBuf,
);

/// Allocate unique owned scratch space for compiler probes and staged native outputs.
impl ProbeDirectory {
    /// Create a uniquely owned temporary directory for probe overlays and native staging artifacts.
    fn new() -> Result<Self, FrontendError> {
        for attempt in 0..128 {
            let directory = std::env::temp_dir()
                .join(format!("pgrx-c-expansion-{}-{attempt}", std::process::id()));
            match std::fs::create_dir(&directory) {
                Ok(()) => return Ok(Self(directory)),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(source) => {
                    return Err(FrontendError::CompilerIo { compiler: directory, source });
                }
            }
        }
        Err(FrontendError::Output("could not reserve a symbolic expansion directory".into()))
    }
}

/// Release resources owned by ProbeDirectory even when a compiler or proof phase exits early.
impl Drop for ProbeDirectory {
    /// Remove only the temporary probe directory owned by this invocation.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Write a VFS overlay mapping the original header spelling to instrumented contents while preserving
/// include context.
fn overlay(directory: &Path, header: &Path, external: &Path) -> Result<PathBuf, FrontendError> {
    let path = directory.join("overlay.json");
    let contents = serde_json::json!({
        "version": 0,
        "use-external-names": false,
        "roots": [{"type": "file", "name": header, "external-contents": external}]
    });
    let bytes = serde_json::to_vec(&contents).map_err(|error| {
        FrontendError::Output(format!("could not encode original-header overlay: {error}"))
    })?;
    std::fs::write(&path, bytes)
        .map_err(|source| FrontendError::CompilerIo { compiler: path.clone(), source })?;
    Ok(path)
}
