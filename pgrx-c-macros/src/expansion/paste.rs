//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compiler validation for token pastes that do not depend on invocation operands.
//!
//! Clang exempts tokens in earlier macro definitions from `GCC poison`, but checks
//! identifiers produced by `##` before rescan. Poisoning the final active names
//! therefore exposes even generated helpers whose eventual expansion is empty.
//! A separate braced-operand pass rejects a live paste involving an operand,
//! including a pasted name that argument prescan subsequently erases.

//! Token pasting is admitted only for closed expansions whose resulting names can be traced
//! to the inspected environment. Poison instrumentation discovers synthesized dependencies,
//! and boundary guards verify complete, owned probe output. Diagnostic ownership prevents
//! one failed invocation from validating another. Run/source budgets and bounded summaries
//! keep this proof finite and make rejected constructs explicit.

/// Reuse the enclosing phase’s compiler/parser primitives so this subphase shares the same validation
/// and input contract.
use super::{ExpansionLimits, ExpansionSkipCode, PROBE_PRAGMAS, Prepared, overlay, probe_tag};
/// Connect this phase to the crate’s owned compiler facts and shared pipeline result types.
use crate::{FrontendError, FrontendOutput, MacroKind};
/// Keep catalog lookup and report ordering deterministic while bounding repeated traversal.
use std::collections::{BTreeMap, BTreeSet};
/// Retain filesystem spellings separately from canonical identities for inspection and rebuild
/// tracking.
use std::path::Path;

/// Bound the total preprocessing passes spent proving closed token pasting.
pub(super) const MAX_CPP_RUNS: usize = 8;

// RegisterBuiltinMacros in upstream Clang 6--21. Unknown compiler majors do not
// enter this proof: a newly registered preprocessor builtin could erase its own
// evidence or alter the expansion environment before the ordinary probe.
/// Context-dependent compiler names that paste instrumentation must guard rather than admitting as
/// ordinary dependencies.
const BUILTINS: &[&str] = &[
    "_Pragma",
    "__BASE_FILE__",
    "__COUNTER__",
    "__DATE__",
    "__FILE_NAME__",
    "__FILE__",
    "__FLT_EVAL_METHOD__",
    "__INCLUDE_LEVEL__",
    "__LINE__",
    "__MODULE__",
    "__TIMESTAMP__",
    "__TIME__",
    "__building_module",
    "__has_attribute",
    "__has_builtin",
    "__has_c_attribute",
    "__has_constexpr_builtin",
    "__has_cpp_attribute",
    "__has_declspec_attribute",
    "__has_embed",
    "__has_extension",
    "__has_feature",
    "__has_include",
    "__has_include_next",
    "__has_warning",
    "__identifier",
    "__is_identifier",
    "__is_target_arch",
    "__is_target_environment",
    "__is_target_os",
    "__is_target_variant_environment",
    "__is_target_variant_os",
    "__is_target_vendor",
    "__pragma",
];

/// Per-probe synthesized dependencies and refusals collected within the bounded paste proof run
/// budget.
#[derive(Default)]
pub(super) struct Proof {
    /// References established for this phase and used to explain or propagate downstream skips.
    pub(super) dependencies: BTreeMap<usize, BTreeSet<String>>,
    /// Candidates lacking a required proof, with stable categories and bounded reasons.
    pub(super) rejected: BTreeMap<usize, (ExpansionSkipCode, String)>,
    /// Preprocessing passes already consumed by the bounded paste proof.
    pub(super) runs: usize,
}

/// Establish bounded token-paste closure and synthesized dependencies using owned poison and boundary
/// probes.
pub(super) fn prove(
    frontend: &FrontendOutput,
    directory: &Path,
    original: &[u8],
    prefix: &str,
    prepared: &[Prepared<'_>],
    limits: ExpansionLimits,
) -> Result<Proof, FrontendError> {
    let mut proof = Proof::default();
    let mut selected = prepared
        .iter()
        .enumerate()
        .filter_map(|(index, item)| item.pastes.then_some(index))
        .collect::<BTreeSet<_>>();
    if selected.is_empty() {
        return Ok(proof);
    }
    if !crate::frontend::version_major(&frontend.profile().compiler.version)
        .is_some_and(|major| (6..=21).contains(&major))
    {
        reject_all(
            &mut proof,
            &selected,
            ExpansionSkipCode::TokenPaste,
            "closed token-paste validation requires the audited Clang 6--21 preprocessor builtin set",
        );
        return Ok(proof);
    }

    let mut poisoned = frontend.environment().active.keys().cloned().collect::<BTreeSet<_>>();
    poisoned.extend(BUILTINS.iter().map(|name| (*name).to_owned()));
    for (index, item) in prepared.iter().enumerate() {
        poisoned.extend(item.markers.iter().cloned());
        poisoned.insert(item.begin.clone());
        poisoned.insert(item.end.clone());
        poisoned.insert(format!("{prefix}paste_wrapper_{index}"));
    }
    // Constant retention numbers candidates from the same active environment.
    // A closed paste must not fabricate an atomic constant marker either.
    poisoned.extend(
        (0..frontend.environment().active.len()).map(|index| super::constant_marker(prefix, index)),
    );

    'modes: for braced in [true, false] {
        let mode = if braced { "braced" } else { "plain" };
        let completion = format!("{prefix}paste_complete_{mode}");
        let completion_path = directory.join(format!("paste-complete-{mode}"));
        let completion_path = completion_path
            .to_str()
            .ok_or_else(|| FrontendError::Arguments("probe path is not UTF-8".into()))?;
        loop {
            if selected.is_empty() {
                break 'modes;
            }
            // Reserve one run for the parent's authoritative clean expansion.
            if proof.runs >= MAX_CPP_RUNS - 1 {
                reject_all(
                    &mut proof,
                    &selected,
                    ExpansionSkipCode::BudgetExceeded,
                    "closed token-paste isolation exhausted the bounded compiler-run budget",
                );
                break 'modes;
            }
            let locations = selected
                .iter()
                .map(|&index| Ok((probe_tag(directory, index)?, index)))
                .collect::<Result<BTreeMap<_, _>, FrontendError>>()?;
            let source = match source(
                original,
                prefix,
                prepared,
                &selected,
                &poisoned,
                &locations,
                braced,
                completion_path,
                &completion,
                limits.source_bytes,
            ) {
                Ok(source) => source,
                Err(message) => {
                    reject_all(&mut proof, &selected, ExpansionSkipCode::BudgetExceeded, &message);
                    break 'modes;
                }
            };
            let path = directory.join(format!("paste-{mode}.c"));
            std::fs::write(&path, source)
                .map_err(|source| FrontendError::CompilerIo { compiler: path.clone(), source })?;
            let overlay = overlay(directory, &frontend.profile().header, &path)?;
            let mut arguments = normalized_arguments(&frontend.profile().arguments);
            arguments.extend([
                "-ivfsoverlay".into(),
                overlay
                    .to_str()
                    .ok_or_else(|| FrontendError::Arguments("probe path is not UTF-8".into()))?
                    .into(),
            ]);
            proof.runs += 1;
            let diagnostics = match crate::frontend::preprocess(
                &frontend.profile().compiler.executable,
                &frontend.profile().header,
                &arguments,
                &["-E", "-P"],
            ) {
                Err(FrontendError::CompilerFailed { diagnostics, status, .. })
                    if status.code().is_some() =>
                {
                    diagnostics
                }
                Err(FrontendError::OutputLimit(_) | FrontendError::Timeout(_)) => {
                    reject_all(
                        &mut proof,
                        &selected,
                        ExpansionSkipCode::BudgetExceeded,
                        "closed token-paste validation exceeded its output or time budget",
                    );
                    break 'modes;
                }
                Err(error) => return Err(error),
                Ok(output) if braced => {
                    if let Err((code, message)) =
                        guard_boundaries(&output.stdout, prepared, &selected, limits.expanded_bytes)
                    {
                        reject_all(&mut proof, &selected, code, &message);
                        break 'modes;
                    }
                    break;
                }
                Ok(_) => {
                    reject_all(
                        &mut proof,
                        &selected,
                        ExpansionSkipCode::UnrecognizedOutput,
                        "closed token-paste validation did not report its completion witness",
                    );
                    break 'modes;
                }
            };
            if diagnostics.len() > limits.expanded_bytes {
                reject_all(
                    &mut proof,
                    &selected,
                    ExpansionSkipCode::BudgetExceeded,
                    "closed token-paste validation diagnostics exceed the expansion byte budget",
                );
                break 'modes;
            }
            let trapped = trapped_probes(&diagnostics, &locations, prefix);
            if braced {
                match failed_probes(&diagnostics, &locations) {
                    Ok(failed) => {
                        for index in failed {
                            let rejection = if trapped.contains(&index) {
                                (
                                    ExpansionSkipCode::DynamicBuiltin,
                                    "token pasting invokes an unsupported preprocessing operation"
                                        .into(),
                                )
                            } else {
                                (ExpansionSkipCode::TokenPaste,
                                    "Clang rejected a braced operand; token pasting depends on invocation operands or the expansion is malformed".into())
                            };
                            proof.rejected.insert(index, rejection);
                            selected.remove(&index);
                        }
                        continue;
                    }
                    Err(message) => {
                        reject_all(
                            &mut proof,
                            &selected,
                            ExpansionSkipCode::UnrecognizedOutput,
                            &message,
                        );
                        break 'modes;
                    }
                }
            }
            let pass = match parse_proof(
                &diagnostics,
                &locations,
                completion_path,
                &completion,
                &poisoned,
            ) {
                Ok(pass) => pass,
                Err(message) => {
                    reject_all(
                        &mut proof,
                        &selected,
                        ExpansionSkipCode::UnrecognizedOutput,
                        &message,
                    );
                    break 'modes;
                }
            };
            for (&index, events) in &pass.pasted {
                for name in events {
                    let rejection = if BUILTINS.contains(&name.as_str()) {
                        Some((
                            ExpansionSkipCode::DynamicBuiltin,
                            format!("token pasting invokes preprocessor builtin {name}"),
                        ))
                    } else if name.contains(prefix) {
                        Some((
                            ExpansionSkipCode::TokenPaste,
                            "token pasting fabricated a symbolic probe identifier".into(),
                        ))
                    } else {
                        None
                    };
                    if let Some(rejection) = rejection {
                        proof.rejected.entry(index).or_insert(rejection);
                    }
                }
            }
            let retry = !pass.failed.is_empty();
            for (index, message) in pass.failed {
                let rejection = if trapped.contains(&index) {
                    (
                        ExpansionSkipCode::DynamicBuiltin,
                        "token pasting invokes an unsupported preprocessing operation".into(),
                    )
                } else {
                    (
                        ExpansionSkipCode::CompilerRejected,
                        format!("Clang rejected {mode} token-paste validation: {message}"),
                    )
                };
                proof.rejected.entry(index).or_insert(rejection);
            }
            selected.retain(|index| !proof.rejected.contains_key(index));
            if retry {
                // An unterminated invocation can swallow later wrappers while the
                // preprocessor still processes the EOF directive. Only an iteration
                // with exclusively known poison errors establishes their traversal.
                continue;
            }
            for &index in &selected {
                proof
                    .dependencies
                    .insert(index, pass.pasted.get(&index).cloned().unwrap_or_default());
            }
            break;
        }
    }
    Ok(proof)
}

/// Record a proof failure for all remaining probes when ownership or budgets cannot be established.
fn reject_all(
    proof: &mut Proof,
    selected: &BTreeSet<usize>,
    code: ExpansionSkipCode,
    message: &str,
) {
    for &index in selected {
        proof.rejected.entry(index).or_insert_with(|| (code, message.to_owned()));
        proof.dependencies.remove(&index);
    }
}

/// Keep diagnostic ownership independent of user diagnostic-display flags.
pub(super) fn normalized_arguments(arguments: &[String]) -> Vec<String> {
    let mut arguments = arguments.to_vec();
    arguments.extend(
        [
            "-ferror-limit=0",
            "-fmacro-backtrace-limit=0",
            "-fno-color-diagnostics",
            "-fdiagnostics-format=clang",
            "-fcaret-diagnostics",
            "-fshow-column",
            "-fshow-source-location",
            "-fmessage-length=0",
        ]
        .map(str::to_owned),
    );
    arguments
}

/// Construct bounded paste instrumentation while preserving original object/function invocation
/// shapes.
#[allow(clippy::too_many_arguments)]
fn source(
    original: &[u8],
    prefix: &str,
    prepared: &[Prepared<'_>],
    selected: &BTreeSet<usize>,
    poisoned: &BTreeSet<String>,
    locations: &BTreeMap<String, usize>,
    braced: bool,
    completion_path: &str,
    completion: &str,
    limit: usize,
) -> Result<Vec<u8>, String> {
    let mut source = Vec::new();
    let mut append = |bytes: &[u8]| -> Result<(), String> {
        if source.len().saturating_add(bytes.len()) > limit {
            return Err("closed token-paste validation source exceeds its byte budget".into());
        }
        source.extend_from_slice(bytes);
        Ok(())
    };
    append(original)?;
    append(PROBE_PRAGMAS.as_bytes())?;
    for &index in selected {
        let item = &prepared[index];
        let operands = item
            .markers
            .iter()
            .map(|marker| if braced { format!("{{ {marker} }}") } else { marker.clone() })
            .collect::<Vec<_>>()
            .join(",");
        let invocation = match item.definition.kind {
            MacroKind::ObjectLike => item.definition.name.clone(),
            MacroKind::FunctionLike => format!("{}({operands})", item.definition.name),
        };
        // The inner wrapper is poisoned; the entry token must remain invocable.
        // Both definitions precede poison, so ordinary uses stay exempt.
        append(
            format!(
                "#define {prefix}paste_wrapper_{index} {invocation}\n#define {prefix}paste_entry_{index} {prefix}paste_wrapper_{index}\n"
            )
            .as_bytes(),
        )?;
    }
    append(b"#pragma clang diagnostic push\n#pragma clang diagnostic ignored \"-Weverything\"\n")?;
    append(format!("#define {prefix}paste_forbidden()\n").as_bytes())?;
    for builtin in BUILTINS {
        // A pasted helper can contain an ordinary exempt builtin token. Trap
        // every use before effects such as _Pragma or __COUNTER__ can occur.
        append(
            format!(
                "#undef {builtin}\n#define {builtin} {prefix}paste_forbidden({prefix}paste_forbidden)\n"
            )
            .as_bytes(),
        )?;
    }
    // Instrumentation warnings are scoped off; chunked pragmas retain every
    // name without spending the source budget on one directive per identifier.
    if !braced {
        append_poison_pragmas(poisoned, &mut append)?;
    }
    append(b"#pragma clang diagnostic pop\n")?;
    let tags = locations.iter().map(|(tag, &index)| (index, tag)).collect::<BTreeMap<_, _>>();
    for &index in selected {
        let tag = tags[&index];
        append(format!("# 1 {tag:?}\n").as_bytes())?;
        if braced {
            append(format!("{}\n", prepared[index].begin).as_bytes())?;
        }
        append(format!("{prefix}paste_entry_{index}\n").as_bytes())?;
        if braced {
            append(format!("{}\n", prepared[index].end).as_bytes())?;
        }
    }
    // The clean braced pass proves every invocation was traversed. This final
    // error confirms the subsequent poison pass completed; any additional
    // structural error requires removal and retry. Failed stdout is never read.
    if !braced {
        append(format!("# 1 {completion_path:?}\n#error {completion}\n").as_bytes())?;
    }
    Ok(source)
}

/// Encode every inspected name into bounded poison pragmas that reveal synthesized references.
fn append_poison_pragmas(
    poisoned: &BTreeSet<String>,
    mut append: impl FnMut(&[u8]) -> Result<(), String>,
) -> Result<(), String> {
    /// Begin poison instrumentation without interpreting inspected macro names as source directives.
    const PREFIX: &str = "#pragma GCC poison ";
    /// Bound poison pragma line length while retaining every inspected identifier.
    const LINE_BYTES: usize = 8192;
    let mut line = String::from(PREFIX);
    for name in poisoned {
        // Each name has a trailing space; reserve the final newline too.
        if PREFIX.len().saturating_add(name.len()).saturating_add(2) > LINE_BYTES {
            return Err("closed token-paste poison identifier exceeds its line byte budget".into());
        }
        if line.len().saturating_add(name.len()).saturating_add(2) > LINE_BYTES {
            line.push('\n');
            append(line.as_bytes())?;
            line.truncate(PREFIX.len());
        }
        line.push_str(name);
        line.push(' ');
    }
    if line.len() > PREFIX.len() {
        line.push('\n');
        append(line.as_bytes())?;
    }
    Ok(())
}

/// Require complete boundary evidence for every probe before accepting a paste pass.
fn guard_boundaries(
    output: &str,
    prepared: &[Prepared<'_>],
    selected: &BTreeSet<usize>,
    limit: usize,
) -> Result<(), (ExpansionSkipCode, String)> {
    if output.len() > limit {
        return Err((
            ExpansionSkipCode::BudgetExceeded,
            "braced token-paste validation output exceeds its byte budget".into(),
        ));
    }
    let expected = selected
        .iter()
        .flat_map(|&index| [&prepared[index].begin, &prepared[index].end])
        .collect::<Vec<_>>();
    let boundaries = expected.iter().map(|name| name.as_str()).collect::<BTreeSet<_>>();
    let mut at = 0;
    for line in output.lines() {
        let line = line.trim();
        if boundaries.contains(line) {
            if expected.get(at).is_none_or(|expected| expected.as_str() != line) {
                return Err((
                    ExpansionSkipCode::UnrecognizedOutput,
                    "braced token-paste validation lost an ordered probe boundary".into(),
                ));
            }
            at += 1;
        }
    }
    if at != expected.len() {
        return Err((
            ExpansionSkipCode::UnrecognizedOutput,
            "braced token-paste validation did not traverse every probe".into(),
        ));
    }
    Ok(())
}

/// One parsed primary diagnostic whose location and fatality control paste-proof ownership.
struct Primary<'a> {
    /// Diagnostic filename, line, and column used for exact probe ownership checks.
    location: &'a str,
    /// Primary compiler diagnostic text interpreted only by exact proof recognizers.
    message: &'a str,
    /// Whether a diagnostic invalidates bounded proof ownership for the pass.
    fatal: bool,
}

/// Parse one primary diagnostic into location, message, and fatality for ownership checks.
fn primary(line: &str) -> Result<Option<Primary<'_>>, String> {
    let (prefix, message, fatal) =
        if let Some((prefix, message)) = line.split_once(": fatal error: ") {
            (prefix, message, true)
        } else if let Some((prefix, message)) = line.split_once(": error: ") {
            (prefix, message, false)
        } else {
            return Ok(None);
        };
    let location = location(prefix)?;
    Ok(Some(Primary { location, message, fatal }))
}

/// Decode the diagnostic source coordinates used to attribute failures to a prepared probe.
fn location(prefix: &str) -> Result<&str, String> {
    let mut parts = prefix.rsplitn(3, ':');
    let column = parts.next().and_then(|value| value.parse::<usize>().ok());
    let line = parts.next().and_then(|value| value.parse::<usize>().ok());
    let path = parts.next().filter(|value| !value.is_empty());
    if column.is_none_or(|column| column == 0) || line.is_none_or(|line| line == 0) {
        return Err("Clang validation error has no exact source location".into());
    }
    path.ok_or_else(|| "Clang validation error has no source path".into())
}

// Classification only: the ownership/proof parsers still validate every error.
// Clang's arity error omits the macro name, so require its exact definition note
// in the same primary-error block before assigning the dynamic-builtin reason.
/// Recognize exact owned preprocessing traps and their required definition notes.
fn trapped_probes(
    diagnostics: &str,
    locations: &BTreeMap<String, usize>,
    prefix: &str,
) -> BTreeSet<usize> {
    let mut trapped = BTreeSet::new();
    let mut current = None;
    let suffix = format!(": note: macro '{prefix}paste_forbidden' defined here");
    for line in diagnostics.lines() {
        match primary(line) {
            Ok(Some(error)) => {
                current = if error.message
                    == "too many arguments provided to function-like macro invocation"
                {
                    locations.get(error.location).copied()
                } else {
                    None
                };
            }
            Err(_) => current = None,
            Ok(None) => {
                if let Some(index) = current
                    && let Some(origin) = line.strip_suffix(&suffix)
                    && location(origin).is_ok()
                {
                    trapped.insert(index);
                }
            }
        }
    }
    trapped
}

/// Attribute ordinary errors to their probes and reject global, fatal, or unowned failures.
pub(super) fn failed_probes(
    diagnostics: &str,
    locations: &BTreeMap<String, usize>,
) -> Result<BTreeSet<usize>, String> {
    let mut failed = BTreeSet::new();
    for line in diagnostics.lines() {
        if let Some((index, _)) = owned_error(line, locations)? {
            failed.insert(index);
        }
    }
    if failed.is_empty() {
        return Err("Clang failed without an owned symbolic-probe error".into());
    }
    Ok(failed)
}

/// Store one bounded primary reason per probe rather than copying batch stderr.
pub(super) fn failed_probe_summaries(
    diagnostics: &str,
    locations: &BTreeMap<String, usize>,
) -> Result<BTreeMap<usize, String>, String> {
    let mut summaries = BTreeMap::new();
    for line in diagnostics.lines() {
        if let Some((index, message)) = owned_error(line, locations)? {
            summaries.entry(index).or_insert_with(|| bounded_summary(message));
        }
    }
    if summaries.is_empty() {
        return Err("Clang failed without an owned symbolic-probe error".into());
    }
    Ok(summaries)
}

/// Check a diagnostic location lies within exactly one prepared probe range.
fn owned_error<'a>(
    line: &'a str,
    locations: &BTreeMap<String, usize>,
) -> Result<Option<(usize, &'a str)>, String> {
    let Some(error) = primary(line)? else { return Ok(None) };
    if error.fatal {
        return Err("Clang stopped with a fatal validation error".into());
    }
    let index = locations
        .get(error.location)
        .ok_or_else(|| "Clang validation error is not owned by a symbolic probe".to_owned())?;
    Ok(Some((*index, error.message)))
}

/// Retain one Unicode-safe bounded reason per failed probe instead of copying arbitrary compiler
/// output.
fn bounded_summary(message: &str) -> String {
    let mut summary = message.chars().take(1024).collect::<String>();
    if summary.len() < message.len() {
        summary.pop();
        summary.push('…');
    }
    summary
}

/// Synthesized references and owned failures recovered from one complete paste instrumentation pass.
#[derive(Default)]
struct Pass {
    /// Synthesized names discovered by poison instrumentation, indexed by their owning probe.
    pasted: BTreeMap<usize, BTreeSet<String>>,
    /// Owned probe failures retained without allowing another candidate’s diagnostics to establish
    /// success.
    failed: BTreeMap<usize, String>,
}

/// Extract poison references and owned failures only from a complete, unambiguous diagnostic proof.
fn parse_proof(
    diagnostics: &str,
    locations: &BTreeMap<String, usize>,
    completion_path: &str,
    completion: &str,
    poisoned: &BTreeSet<String>,
) -> Result<Pass, String> {
    let lines = diagnostics.lines().collect::<Vec<_>>();
    let mut pass = Pass::default();
    let mut completed = false;
    let mut at = 0;
    while at < lines.len() {
        let Some(error) = primary(lines[at])? else {
            at += 1;
            continue;
        };
        if error.fatal {
            return Err("Clang stopped before completing token-paste validation".into());
        }
        let mut end = at + 1;
        while end < lines.len() && primary(lines[end])?.is_none() {
            end += 1;
        }
        if error.location == completion_path && error.message == completion {
            if completed || end < lines.len() {
                return Err("Clang token-paste completion witness is repeated or not final".into());
            }
            completed = true;
        } else {
            let index = *locations.get(error.location).ok_or_else(|| {
                "Clang token-paste error is not owned by a symbolic probe".to_owned()
            })?;
            if error.message == "attempt to use a poisoned identifier" {
                let name = poisoned_name(&lines[at + 1..end])?;
                if !poisoned.contains(name) {
                    return Err("Clang reported a pasted identifier outside the poison set".into());
                }
                pass.pasted.entry(index).or_default().insert(name.to_owned());
            } else {
                pass.failed.entry(index).or_insert_with(|| error.message.to_owned());
            }
        }
        at = end;
    }
    if !completed {
        return Err("Clang did not reach the token-paste completion witness".into());
    }
    Ok(pass)
}

/// Recover the inspected identifier named by an accepted poison diagnostic.
fn poisoned_name<'a>(lines: &[&'a str]) -> Result<&'a str, String> {
    let mut found = None;
    for (index, line) in lines.iter().enumerate() {
        let Some(prefix) = line.strip_suffix(": note: expanded from here") else { continue };
        if location(prefix)? != "<scratch space>" {
            return Err("Clang poison note has an unexpected spelling origin".into());
        }
        let spelling = lines.get(index + 1).and_then(|line| {
            let (number, spelling) = line.split_once('|')?;
            (number.trim().parse::<usize>().ok()? > 0).then_some(spelling.trim())
        });
        let caret = lines.get(index + 2).and_then(|line| line.trim().strip_prefix('|'));
        let valid_caret = caret.is_some_and(|caret| {
            let caret = caret.trim();
            caret.starts_with('^') && caret[1..].bytes().all(|byte| byte == b'~')
        });
        let Some(spelling) = spelling.filter(|spelling| identifier(spelling) && valid_caret) else {
            return Err("Clang poison note lacks one complete identifier and caret excerpt".into());
        };
        if found.replace(spelling).is_some() {
            return Err("Clang poison diagnostic has multiple spelling origins".into());
        }
    }
    found.ok_or_else(|| "Clang poison diagnostic lacks its pasted spelling origin".into())
}

/// Check a pasted name belongs to the ordinary C identifier grammar admitted by this proof.
fn identifier(spelling: &str) -> bool {
    let mut bytes = spelling.bytes();
    bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

/// Exercise this phase’s semantic boundaries with owned fixtures.
/// These regressions check accepted proofs and explicit refusals without changing production
/// headers or weakening the C identity and evaluation contracts.
#[cfg(test)]
mod tests {
    /// Reuse the enclosing phase’s compiler/parser primitives so this subphase shares the same
    /// validation and input contract.
    use super::*;

    /// Construct minimal original macro definitions for paste instrumentation tests.
    fn definition(kind: MacroKind) -> crate::MacroDefinition {
        crate::MacroDefinition {
            name: "ROOT".into(),
            kind,
            location: None,
            provenance: None,
            tokens: Vec::new(),
            builtin: false,
            main_file: true,
        }
    }

    /// Build a prepared probe fixture with explicit boundaries and invocation metadata.
    fn item(definition: &crate::MacroDefinition, index: usize) -> Prepared<'_> {
        Prepared {
            definition,
            parameters: vec!["operand".into()],
            body_start: 0,
            markers: vec![format!("fresh_parameter_{index}")],
            begin: format!("fresh_begin_{index}"),
            end: format!("fresh_end_{index}"),
            dependencies: Vec::new(),
            pastes: true,
        }
    }

    /// Construct deterministic boundary-location facts for diagnostic ownership tests.
    fn locations() -> BTreeMap<String, usize> {
        BTreeMap::from([("/tmp/probe-0".into(), 0), ("/tmp/probe-1".into(), 1)])
    }

    /// Render poison diagnostics used to test synthesized-name discovery.
    fn poison(index: usize, name: &str) -> String {
        format!(
            "/tmp/probe-{index}:1:1: error: attempt to use a poisoned identifier\n    1 | entry\n      | ^\n/header.h:5:2: note: expanded from macro 'CAT'\n    5 | a##b\n      |  ^\n<scratch space>:3:1: note: expanded from here\n    3 | {name}\n      | ^\n"
        )
    }

    /// Completion marker expected by paste-proof fixtures so partial diagnostics cannot establish
    /// success.
    const COMPLETE: &str = "/tmp/complete:1:2: error: DONE\n    1 | #error DONE\n      |  ^\n";

    /// Run paste-proof parsing against fixture names and prepared probes.
    fn parse(text: &str) -> Result<Pass, String> {
        parse_proof(
            text,
            &locations(),
            "/tmp/complete",
            "DONE",
            &BTreeSet::from(["HELPER".into(), "OBJECT".into()]),
        )
    }

    /// Checks poison dependencies preserve probe ownership and completion.
    #[test]
    fn poison_dependencies_preserve_probe_ownership_and_completion() {
        let text = format!(
            "{}{}{}{COMPLETE}",
            poison(0, "HELPER"),
            poison(0, "OBJECT"),
            poison(1, "HELPER")
        );
        let pass = parse(&text).unwrap();
        assert!(pass.failed.is_empty());
        assert_eq!(pass.pasted[&0], BTreeSet::from(["HELPER".into(), "OBJECT".into()]));
        assert_eq!(pass.pasted[&1], BTreeSet::from(["HELPER".into()]));
    }

    /// Checks proof requires complete and unambiguous diagnostics.
    #[test]
    fn proof_requires_complete_and_unambiguous_diagnostics() {
        let valid = format!("{}{COMPLETE}", poison(0, "HELPER"));
        for text in [
            poison(0, "HELPER"),
            valid.replace("/tmp/probe-0", "/unowned/header.h"),
            valid.replace("HELPER", "UNKNOWN"),
            valid.replace("    3 | HELPER", "    3 | HELPER OTHER"),
            valid.replace("      | ^\n", ""),
            valid.replace(": note: expanded from here", ": note: unknown spelling origin"),
            format!("{valid}{COMPLETE}"),
            format!("{COMPLETE}{}", poison(0, "HELPER")),
            format!("{valid}/tmp/probe-1:1:1: fatal error: stopped\n"),
            format!("{}{COMPLETE}", poison(0, "HELPER").replace("<scratch space>", "/header.h")),
        ] {
            assert!(parse(&text).is_err(), "accepted {text}");
        }
        let mut multiple = poison(0, "HELPER");
        multiple
            .push_str("<scratch space>:4:1: note: expanded from here\n    4 | OBJECT\n      | ^\n");
        multiple.push_str(COMPLETE);
        assert!(parse(&multiple).is_err());
        assert!(parse(COMPLETE).unwrap().pasted.is_empty());
    }

    /// Checks recoverable operand error rejects only its owned probe.
    #[test]
    fn recoverable_operand_error_rejects_only_its_owned_probe() {
        let text = format!(
            "/tmp/probe-1:1:1: error: pasting formed an invalid preprocessing token\n/header.h:2:1: note: expanded from macro 'CAT'\n{}{COMPLETE}",
            poison(0, "HELPER")
        );
        let pass = parse(&text).unwrap();
        assert_eq!(pass.failed.len(), 1);
        assert!(pass.failed.contains_key(&1));
        assert_eq!(pass.pasted[&0], BTreeSet::from(["HELPER".into()]));
    }

    /// Checks ordinary failure ownership rejects global fatal or missing errors.
    #[test]
    fn ordinary_failure_ownership_rejects_global_fatal_or_missing_errors() {
        let owned = "/tmp/probe-0:4:2: error: wrong arity\n/header.h:1:1: note: macro defined here\n/tmp/probe-1:1:9: error: wrong operand\n";
        assert_eq!(failed_probes(owned, &locations()).unwrap(), BTreeSet::from([0, 1]));
        for text in [
            "clang: error: invalid option\n",
            "/header.h:1:1: error: unrelated\n",
            "/tmp/probe-0:1:1: fatal error: stopped\n",
            "/tmp/probe-0:0:1: error: malformed location\n",
            "/tmp/probe-0:1:1: warning: no error\n",
            "",
        ] {
            assert!(failed_probes(text, &locations()).is_err());
        }
    }

    /// Checks failure summaries keep only one bounded unicode reason per probe.
    #[test]
    fn failure_summaries_keep_only_one_bounded_unicode_reason_per_probe() {
        let locations = (0..64).map(|index| (format!("/tmp/probe-{index}"), index)).collect();
        let mut diagnostics = String::new();
        for index in 0..64 {
            diagnostics.push_str(&format!(
                "/tmp/probe-{index}:1:1: error: macro {index}: {}\n/header.h:1:1: note: expanded here\n/tmp/probe-{index}:2:1: error: secondary reason\n",
                "é🦀".repeat(2048)
            ));
        }
        let summaries = failed_probe_summaries(&diagnostics, &locations).unwrap();
        assert_eq!(summaries.len(), 64);
        assert_eq!(
            summaries.keys().copied().collect::<BTreeSet<_>>(),
            failed_probes(&diagnostics, &locations).unwrap()
        );
        for (index, summary) in summaries {
            assert!(summary.starts_with(&format!("macro {index}: ")));
            assert_eq!(summary.chars().count(), 1024);
            assert!(summary.ends_with('…'));
            assert!(!summary.contains("secondary reason"));
        }
        let exact = "🦀".repeat(1024);
        assert_eq!(bounded_summary(&exact), exact);
        assert_eq!(bounded_summary("short reason"), "short reason");
        for line in [
            "clang: error: invalid option\n",
            "/header.h:1:1: error: unowned\n",
            "/tmp/probe-0:1:1: fatal error: stopped\n",
            "",
        ] {
            let summary = failed_probe_summaries(line, &locations).unwrap_err();
            assert_eq!(summary, failed_probes(line, &locations).unwrap_err());
            assert!(summary.chars().count() < 100);
            if !line.is_empty() {
                assert!(!summary.contains(line.trim()));
            }
        }
    }

    /// Checks compact poison pragmas keep all names with bounded lines and source.
    #[test]
    fn compact_poison_pragmas_keep_all_names_with_bounded_lines_and_source() {
        let names = (0..1024)
            .map(|index| format!("fresh_identifier_{index:05}"))
            .chain(["if".into(), "while".into(), "__COUNTER__".into()])
            .collect::<BTreeSet<_>>();
        let separate_bytes = names.iter().map(|name| 19 + name.len() + 1).sum::<usize>();
        let budget = 30_000;
        assert!(separate_bytes > budget);
        let mut bytes = Vec::new();
        append_poison_pragmas(&names, |chunk| {
            if bytes.len() + chunk.len() > budget {
                return Err("source budget".into());
            }
            bytes.extend_from_slice(chunk);
            Ok(())
        })
        .unwrap();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.len() <= budget);
        assert!(text.lines().count() > 1);
        assert!(text.lines().all(|line| line.len() < 8192));
        let emitted = text
            .lines()
            .flat_map(|line| line.strip_prefix("#pragma GCC poison ").unwrap().split_whitespace())
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        assert_eq!(emitted, names);
        assert_eq!(
            text.lines().flat_map(|line| line.split_whitespace().skip(3)).count(),
            names.len()
        );
        assert!(append_poison_pragmas(&BTreeSet::from(["x".repeat(8192)]), |_| Ok(())).is_err());
    }

    /// Checks preprocessing trap requires exact owned arity error and definition note.
    #[test]
    fn preprocessing_trap_requires_exact_owned_arity_error_and_definition_note() {
        let diagnostic = "/tmp/probe-0:1:1: error: too many arguments provided to function-like macro invocation\n/header.h:8:1: note: macro 'fresh_paste_forbidden' defined here\n";
        assert_eq!(trapped_probes(diagnostic, &locations(), "fresh_"), BTreeSet::from([0]));
        for text in [
            diagnostic.replace("fresh_paste_forbidden", "fresh_paste_forbidden_extra"),
            diagnostic.replace("/tmp/probe-0", "/unowned/header.h"),
            diagnostic.replace("too many arguments", "too few arguments"),
            diagnostic.replace("/header.h:8:1", "clang"),
            "/header.h:8:1: note: macro 'fresh_paste_forbidden' defined here\n".into(),
            diagnostic.replace(": error: ", ": warning: "),
        ] {
            assert!(trapped_probes(&text, &locations(), "fresh_").is_empty());
        }
        let separated = diagnostic.replace(
            "/header.h:8:1: note:",
            "/tmp/probe-1:1:1: error: unrelated\n/header.h:8:1: note:",
        );
        assert!(trapped_probes(&separated, &locations(), "fresh_").is_empty());
    }

    /// Checks clean guard requires every boundary once in numeric probe order.
    #[test]
    fn clean_guard_requires_every_boundary_once_in_numeric_probe_order() {
        let definition = definition(MacroKind::FunctionLike);
        let prepared = (0..13).map(|index| item(&definition, index)).collect::<Vec<_>>();
        let selected = (0..13).collect::<BTreeSet<_>>();
        let output = selected
            .iter()
            .map(|&index| format!("fresh_begin_{index}\nbody\nfresh_end_{index}\n"))
            .collect::<String>();
        assert!(guard_boundaries(&output, &prepared, &selected, output.len()).is_ok());
        for malformed in [
            output.replace("fresh_begin_5\nbody\nfresh_end_5\n", ""),
            output.replace("fresh_begin_5", "fresh_begin_6"),
            format!("{output}fresh_end_12\n"),
            "fresh_begin_0 fresh_end_0\n".into(),
        ] {
            assert!(guard_boundaries(&malformed, &prepared, &selected, usize::MAX).is_err());
        }
        assert_eq!(
            guard_boundaries(&output, &prepared, &selected, output.len() - 1).unwrap_err().0,
            ExpansionSkipCode::BudgetExceeded
        );
    }

    /// Checks proof source preserves object invocation and isolates instrumentation.
    #[test]
    fn proof_source_preserves_object_invocation_and_isolates_instrumentation() {
        let object = definition(MacroKind::ObjectLike);
        let function = definition(MacroKind::FunctionLike);
        let prepared = vec![item(&object, 0), item(&function, 1)];
        let selected = BTreeSet::from([0, 1]);
        let poisoned = BTreeSet::from(["ROOT".into(), "fresh_parameter_0".into()]);
        for braced in [true, false] {
            let bytes = source(
                b"original_header\n",
                "fresh_",
                &prepared,
                &selected,
                &poisoned,
                &locations(),
                braced,
                "/tmp/complete",
                "DONE",
                usize::MAX,
            )
            .unwrap();
            let text = String::from_utf8(bytes).unwrap();
            assert!(text.contains("#define fresh_paste_wrapper_0 ROOT\n"));
            assert!(!text.contains("#define fresh_paste_wrapper_0 ROOT("));
            assert!(text.contains(if braced {
                "#define fresh_paste_wrapper_1 ROOT({ fresh_parameter_1 })"
            } else {
                "#define fresh_paste_wrapper_1 ROOT(fresh_parameter_1)"
            }));
            assert!(text.find("original_header").unwrap() < text.find("diagnostic push").unwrap());
            assert!(
                text.find("diagnostic pop").unwrap() < text.find("# 1 \"/tmp/probe-0\"").unwrap()
            );
            assert_eq!(text.contains("#pragma GCC poison ROOT"), !braced);
            assert_eq!(text.contains("#error DONE"), !braced);
            assert_eq!(text.contains("fresh_begin_0\n"), braced);
            assert!(
                source(
                    b"original_header\n",
                    "fresh_",
                    &prepared,
                    &selected,
                    &poisoned,
                    &locations(),
                    braced,
                    "/tmp/complete",
                    "DONE",
                    text.len() - 1,
                )
                .is_err()
            );
        }
    }
}
