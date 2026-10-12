//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Retain only independently verified, atomic integer object-macro references.
//!
//! The ordinary expansion stays authoritative. A second pass masks constants;
//! substituting their original expansions must reproduce every original token.

//! Symbol retention preserves readable references to object macros only when the reference
//! is an atomic integer expression with independently verified C type and value. A masking
//! pass must reproduce the authoritative ordinary expansion after restoring original tokens.
//! Missing proofs leave the compiler expansion intact and record an explanatory fallback;
//! these C facts are later compared against actual Rust bindings before emission.

use super::{
    ConstantFallback, ExpansionBatch, ExpansionLimits, ExpansionResult, Prepared, ProbeDirectory,
    extract_bodies, namespace, overlay, parameters, verify_original_environment,
};
use crate::{
    Error, FrontendError, FrontendOutput, IntegerConstant, IntegerKind, IntegerValue, MacroKind,
    MacroScanner, Token, TokenKind, TypeCategory,
};
use clang::{EntityKind, EntityVisitResult, EvaluationResult};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write;
use std::io::Read;
use std::path::Path;

/// Bound retry isolation during optional object-constant proof so malformed candidates cannot cause
/// unlimited compiler passes.
const MAX_PROBE_ATTEMPTS: usize = 32;

/// Borrow the coherent scanner/profile and owned-overlay inputs shared by constant-retention proof
/// phases.
struct ConstantContext<'a> {
    /// The live libclang runtime used for parsing and typed witness extraction.
    scanner: &'a MacroScanner,
    /// The coherent inspected environment borrowed by this phase; it is never reconstructed from
    /// snapshots.
    frontend: &'a FrontendOutput,
    /// The unique owned scratch directory containing this phase’s temporary overlays and probes.
    directory: &'a Path,
    /// Original main-file bytes retained to prove instrumentation preserves its preprocessing
    /// context.
    original: &'a [u8],
    /// A collision-free generated identifier prefix chosen from the inspected environment.
    prefix: &'a str,
    /// Finite source/token/run budgets enforced by this proof or dependency inspection.
    limits: ExpansionLimits,
}

/// Compiler-established values for object macros missing from a binding generator.
/// Existing Rust bindings must never be overwritten with these observations.
#[derive(Debug, serde::Serialize)]
pub struct ObjectIntegerConstants {
    /// Values whose original C expression, integer identity and driver witness agree.
    pub constants: BTreeMap<String, IntegerConstant>,
    /// Definitions without a complete constant/type proof, including budget refusals.
    pub rejected: BTreeMap<String, String>,
}

/// Probe selected object macros as independent integer constants under the inspected profile.
///
/// This supplements absent bindings, not disagreed binding values. It establishes
/// constant initialization and exact C type/value with the same witnesses used
/// for macro symbol retention; callers still choose their Rust storage explicitly.
pub fn probe_integer_object_constants(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    names: &[impl AsRef<str>],
) -> Result<ObjectIntegerConstants, FrontendError> {
    let limits = ExpansionLimits::default();
    super::verify_environment(frontend)?;
    crate::frontend::verify_input_files(&frontend.profile().inputs)?;
    let directory = ProbeDirectory::new()?;
    let mut original = Vec::new();
    std::fs::File::open(&frontend.profile().header)
        .map_err(|source| FrontendError::CompilerIo {
            compiler: frontend.profile().header.clone(),
            source,
        })?
        .take(u64::try_from(limits.source_bytes).unwrap_or(u64::MAX).saturating_add(1))
        .read_to_end(&mut original)
        .map_err(|source| FrontendError::CompilerIo {
            compiler: frontend.profile().header.clone(),
            source,
        })?;
    if original.len().saturating_add(2) > limits.source_bytes || names.len() > limits.macros {
        return Err(FrontendError::Output(
            "object constant probe exceeds its source/count budget".into(),
        ));
    }
    original.extend_from_slice(b"\n\n");
    verify_original_environment(
        scanner,
        frontend,
        &directory.0,
        &original,
        &frontend.profile().inputs,
    )?;
    let prefix = namespace(frontend);
    let mut rejected = BTreeMap::new();
    let mut supported = BTreeMap::new();
    let mut dependencies =
        super::Dependencies { frontend, nodes: HashMap::new(), tokens: 0, limits };
    for name in names {
        let name = name.as_ref();
        let Some((name, active)) = frontend.environment().active.get_key_value(name) else {
            rejected.insert(name.to_owned(), "object macro is not active".into());
            continue;
        };
        if active.definition.kind != MacroKind::ObjectLike {
            rejected.insert(name.clone(), "definition is not an object macro".into());
            continue;
        }
        match dependencies.closure(name) {
            Ok((closure, _)) => {
                let invocation_sensitive = closure.iter().any(|dependency| {
                    frontend.environment().active[&dependency.name]
                        .definition
                        .tokens
                        .iter()
                        .any(|token| matches!(token.spelling.as_str(), "__FILE__" | "__LINE__"))
                }) || active
                    .definition
                    .tokens
                    .iter()
                    .any(|token| matches!(token.spelling.as_str(), "__FILE__" | "__LINE__"));
                if invocation_sensitive {
                    rejected.insert(
                        name.clone(),
                        "object value depends on its invocation location".into(),
                    );
                } else {
                    supported.insert(name.clone(), Vec::new());
                }
            }
            Err(reason) => {
                rejected.insert(name.clone(), reason.message);
            }
        }
    }
    let context = ConstantContext {
        scanner,
        frontend,
        directory: &directory.0,
        original: &original,
        prefix: &prefix,
        limits,
    };
    let result = probe_constants(&context, &supported, &mut rejected);
    crate::frontend::verify_input_files(&frontend.profile().inputs)?;
    super::verify_environment(frontend)?;
    Ok(ObjectIntegerConstants { constants: result?, rejected })
}

/// The caller must verify the inspected inputs before and after this compiler phase.
pub(crate) fn retain_integer_constants(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    batch: &mut ExpansionBatch,
    limits: ExpansionLimits,
) -> Result<BTreeMap<String, IntegerConstant>, FrontendError> {
    // Record pasted enum references before retained object bindings can hide
    // them. Ordinary references already have lexical edges; flattening those
    // would replace the original dependency paths in skip explanations.
    for result in batch.results.values() {
        let ExpansionResult::Expanded { expansion } = result else { continue };
        if !batch.discovered_dependencies.contains_key(&expansion.name) {
            continue;
        }
        let start = if expansion.definition.kind == MacroKind::ObjectLike {
            1
        } else {
            parameters(&expansion.definition).expect("prepared function signatures remain intact").1
        };
        record_integer_references(
            frontend,
            &expansion.name,
            &expansion.definition.tokens[start..],
            &expansion.symbolic_parameters,
            &mut batch.discovered_dependencies,
        );
    }
    let names = batch
        .results
        .values()
        .filter_map(|result| match result {
            ExpansionResult::Expanded { expansion } => Some(&expansion.dependencies),
            _ => None,
        })
        .flatten()
        .filter(|dependency| dependency.kind == MacroKind::ObjectLike)
        .map(|dependency| dependency.name.clone())
        .collect::<BTreeSet<_>>();
    if names.is_empty() {
        return Ok(BTreeMap::new());
    }
    let mut rejected = BTreeMap::<String, String>::new();
    let mut original = Vec::new();
    std::fs::File::open(&frontend.profile().header)
        .map_err(|source| FrontendError::CompilerIo {
            compiler: frontend.profile().header.clone(),
            source,
        })?
        .take(u64::try_from(limits.source_bytes).unwrap_or(u64::MAX).saturating_add(1))
        .read_to_end(&mut original)
        .map_err(|source| FrontendError::CompilerIo {
            compiler: frontend.profile().header.clone(),
            source,
        })?;
    if original.len().saturating_add(2) > limits.source_bytes || names.len() > limits.macros {
        reject_all(&names, &mut rejected, "constant discovery exceeds its source/count budget");
        record_fallbacks(batch, &rejected);
        return Ok(BTreeMap::new());
    }
    original.extend_from_slice(b"\n\n");
    let directory = ProbeDirectory::new()?;
    verify_original_environment(scanner, frontend, &directory.0, &original, &batch.inputs)?;
    let prefix = namespace(frontend);
    let mut source_bytes = original.len();
    let mut prepared = Vec::new();
    let mut dependencies =
        super::Dependencies { frontend, nodes: HashMap::new(), tokens: 0, limits };
    for (index, name) in names.iter().enumerate() {
        let (active_name, active) = frontend
            .environment()
            .active
            .get_key_value(name)
            .expect("object dependencies are active macros");
        let definition = &active.definition;
        let (closure, pastes) = match dependencies.closure(active_name) {
            Ok(closure) => closure,
            Err(reason) => {
                rejected.insert(name.clone(), reason.message);
                continue;
            }
        };
        let begin = format!("{prefix}constant_begin_{index}");
        let end = format!("{prefix}constant_end_{index}");
        let invocation = format!("{begin}\n{name}\n{end}\n");
        if source_bytes.saturating_add(invocation.len()) > limits.source_bytes {
            rejected.insert(name.clone(), "constant expansion exceeds its source budget".into());
            continue;
        }
        source_bytes += invocation.len();
        prepared.push(Prepared {
            definition,
            parameters: Vec::new(),
            body_start: 1,
            markers: Vec::new(),
            begin,
            end,
            dependencies: closure,
            pastes,
        });
    }
    let mut proof =
        super::paste::prove(frontend, &directory.0, &original, &prefix, &prepared, limits)?;
    for (&index, references) in &proof.dependencies {
        batch
            .discovered_dependencies
            .entry(prepared[index].definition.name.clone())
            .or_default()
            .extend(references.iter().cloned());
        let mut merged = prepared[index]
            .dependencies
            .iter()
            .map(|dependency| dependency.name.clone())
            .collect::<BTreeSet<_>>();
        for reference in references {
            let (active_name, _) = frontend
                .environment()
                .active
                .get_key_value(reference)
                .expect("paste discovery returns active macro names");
            match dependencies.closure(active_name) {
                Ok((closure, _)) => {
                    merged.insert(reference.clone());
                    merged.extend(closure.into_iter().map(|dependency| dependency.name));
                }
                Err(reason) => {
                    proof.rejected.insert(index, (reason.code, reason.message));
                    break;
                }
            }
        }
        if merged.len().saturating_add(1) > limits.dependencies_per_macro {
            proof.rejected.insert(
                index,
                (
                    super::ExpansionSkipCode::BudgetExceeded,
                    "synthesized object-macro dependencies exceed their budget".into(),
                ),
            );
        }
    }
    for (&index, (_, reason)) in &proof.rejected {
        rejected.insert(prepared[index].definition.name.clone(), reason.clone());
    }
    prepared = prepared
        .into_iter()
        .enumerate()
        .filter_map(|(index, item)| (!proof.rejected.contains_key(&index)).then_some(item))
        .collect();
    // Rebuild without the rejected probes; use only this clean ordinary output.
    let mut source = original.clone();
    for item in &prepared {
        source.extend_from_slice(
            format!("{}\n{}\n{}\n", item.begin, item.definition.name, item.end).as_bytes(),
        );
    }
    if prepared.is_empty() {
        record_fallbacks(batch, &rejected);
        return Ok(BTreeMap::new());
    }
    let bodies = match expand_bodies(scanner, frontend, &directory.0, &source, &prepared, limits) {
        Ok(bodies) => bodies,
        Err(error) if optional_failure(&error) => {
            reject_all(&names, &mut rejected, "Clang could not establish object-macro expansions");
            record_fallbacks(batch, &rejected);
            return Ok(BTreeMap::new());
        }
        Err(error) => return Err(error),
    };
    let mut supported = BTreeMap::new();
    for (item, tokens) in prepared.iter().zip(bodies) {
        let name = &item.definition.name;
        if item.pastes {
            record_integer_references(
                frontend,
                name,
                &tokens,
                &[],
                &mut batch.discovered_dependencies,
            );
        }
        if let Err(reason) = crate::analysis::validate_constant_expression(frontend, &tokens) {
            rejected.insert(
                name.clone(),
                format!("object macro is not a supported pure integer expression: {reason}"),
            );
        } else {
            supported.insert(name.clone(), tokens);
        }
    }
    let context = ConstantContext {
        scanner,
        frontend,
        directory: &directory.0,
        original: &original,
        prefix: &prefix,
        limits,
    };
    let mut constants = probe_constants(&context, &supported, &mut rejected)?;
    for (name, constant) in &mut constants {
        if let Ok(expression) = crate::syntax::parse_expression(&supported[name], &[], |_| false) {
            let mut node = expression.root;
            while let crate::ExpressionKind::Group { operand } = expression.nodes[node].kind {
                node = operand;
            }
            if let crate::ExpressionKind::IntegerLiteral { literal } = &expression.nodes[node].kind
            {
                constant.literal = Some(literal.clone());
            }
        }
    }
    let mut eligible = BTreeMap::new();
    for (name, tokens) in supported {
        if !constants.contains_key(&name) {
            continue;
        }
        if atomic(&tokens) {
            eligible.insert(name, tokens);
        } else {
            rejected.insert(name, "a binding reference could change C grouping because the object expansion is neither atomic nor enclosed in parentheses".into());
        }
    }
    if !eligible.is_empty() {
        mask_constants(&context, batch, &eligible, &mut rejected)?;
    }
    record_fallbacks(batch, &rejected);
    Ok(constants)
}

/// Record one conservative reason for a bounded group of constant candidates that cannot be verified.
fn reject_all(names: &BTreeSet<String>, rejected: &mut BTreeMap<String, String>, reason: &str) {
    for name in names {
        rejected.insert(name.clone(), reason.into());
    }
}

/// Preserve compiler-discovered integer references before masking object constants hides their
/// tokens.
fn record_integer_references(
    frontend: &FrontendOutput,
    name: &str,
    tokens: &[Token],
    parameters: &[String],
    references: &mut BTreeMap<String, BTreeSet<String>>,
) {
    let Ok(expression) = crate::syntax::parse_expression(tokens, parameters, |name| {
        frontend.declarations().types.contains_key(name)
    }) else {
        return;
    };
    for node in expression.nodes {
        if let crate::ExpressionKind::Identifier { name: reference } = node.kind
            && !frontend.environment().active.contains_key(&reference)
            && frontend.declarations().integer_constants.contains_key(&reference)
        {
            references.entry(name.into()).or_default().insert(reference);
        }
    }
}

/// Attach failed symbol-retention reasons to macro expansions that must keep compiler-expanded
/// values.
fn record_fallbacks(batch: &mut ExpansionBatch, rejected: &BTreeMap<String, String>) {
    for result in batch.results.values_mut() {
        let ExpansionResult::Expanded { expansion } = result else { continue };
        for dependency in &expansion.dependencies {
            if let Some(reason) = rejected.get(&dependency.name) {
                expansion.constant_fallbacks.push(ConstantFallback {
                    name: dependency.name.clone(),
                    reason: reason.clone(),
                });
            }
        }
    }
}

/// Classify candidate-specific proof failures that can leave ordinary expansion intact rather than
/// aborting inspection.
fn optional_failure(error: &FrontendError) -> bool {
    matches!(
        error,
        FrontendError::CompilerFailed { .. }
            | FrontendError::OutputLimit(_)
            | FrontendError::Timeout(_)
            | FrontendError::Output(_)
            | FrontendError::Discovery(Error::Diagnostics(_))
    )
}

/// Build phase-specific arguments for constant probes while retaining the inspected header overlay
/// context.
fn probe_arguments(
    frontend: &FrontendOutput,
    directory: &Path,
    source: &[u8],
) -> Result<Vec<String>, FrontendError> {
    let path = directory.join("constants.c");
    std::fs::write(&path, source)
        .map_err(|source| FrontendError::CompilerIo { compiler: path.clone(), source })?;
    let overlay = overlay(directory, &frontend.profile().header, &path)?;
    let mut arguments = frontend.profile().arguments.clone();
    arguments.extend([
        "-ivfsoverlay".into(),
        overlay
            .to_str()
            .ok_or_else(|| FrontendError::Arguments("constant probe path is not UTF-8".into()))?
            .into(),
    ]);
    Ok(arguments)
}

/// Expand prepared object candidates and isolate bounded failures without replacing authoritative
/// ordinary tokens.
fn expand_bodies(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    directory: &Path,
    source: &[u8],
    prepared: &[Prepared<'_>],
    limits: ExpansionLimits,
) -> Result<Vec<Vec<Token>>, FrontendError> {
    if prepared.is_empty() {
        return Ok(Vec::new());
    }
    let arguments = probe_arguments(frontend, directory, source)?;
    let output = crate::frontend::preprocess(
        &frontend.profile().compiler.executable,
        &frontend.profile().header,
        &arguments,
        &["-E", "-P"],
    )?;
    let bodies = extract_bodies(&output.stdout, prepared, limits.expanded_bytes)
        .map_err(|(_, message)| FrontendError::Output(message))?;
    let prefix = namespace(frontend);
    let mut snapshot = String::new();
    for (index, body) in bodies.iter().enumerate() {
        writeln!(&mut snapshot, "#define {prefix}constant_tokens_{index} {body}")
            .expect("writing to a String cannot fail");
    }
    let mut definitions =
        crate::frontend::tokenize_snapshot(scanner, &snapshot, &frontend.profile().arguments)?
            .into_iter()
            .map(|definition| (definition.name.clone(), definition))
            .collect::<HashMap<_, _>>();
    let mut total_tokens = 0usize;
    (0..prepared.len())
        .map(|index| {
            let definition =
                definitions.remove(&format!("{prefix}constant_tokens_{index}")).ok_or_else(
                    || FrontendError::Output("constant expansion tokens disappeared".into()),
                )?;
            let tokens = definition.tokens.into_iter().skip(1).collect::<Vec<_>>();
            total_tokens = total_tokens.saturating_add(tokens.len());
            if tokens.len() > limits.macro_tokens || total_tokens > limits.total_tokens {
                return Err(FrontendError::Output(
                    "constant expansions exceed their token budget".into(),
                ));
            }
            Ok(tokens)
        })
        .collect()
}

/// Check that one replacement is a complete atomic expression suitable for safe symbol retention.
fn atomic(tokens: &[Token]) -> bool {
    if tokens.len() == 1 {
        return matches!(tokens[0].kind, TokenKind::Literal | TokenKind::Identifier);
    }
    if tokens.first().is_none_or(|token| token.spelling != "(")
        || tokens.last().is_none_or(|token| token.spelling != ")")
    {
        return false;
    }
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match token.spelling.as_str() {
            "(" => depth += 1,
            ")" => {
                let Some(next) = depth.checked_sub(1) else { return false };
                depth = next;
                if depth == 0 && index + 1 != tokens.len() {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}

/// Establish original object-macro integer types and values through matched compiler observations.
fn probe_constants(
    context: &ConstantContext<'_>,
    supported: &BTreeMap<String, Vec<Token>>,
    rejected: &mut BTreeMap<String, String>,
) -> Result<BTreeMap<String, IntegerConstant>, FrontendError> {
    let ConstantContext { scanner, frontend, directory, original, prefix, limits } = context;
    let names = supported.keys().collect::<Vec<_>>();
    let mut pending = vec![(0, names.len())];
    let mut attempts = 0usize;
    let mut constants = BTreeMap::new();
    while let Some((start, end)) = pending.pop() {
        if start == end {
            continue;
        }
        if attempts == MAX_PROBE_ATTEMPTS {
            for name in &names[start..end] {
                rejected.insert(
                    (*name).clone(),
                    "typed constant probes exhausted their bounded retry budget".into(),
                );
            }
            continue;
        }
        attempts += 1;
        let mut source = original.to_vec();
        let mut probes = BTreeMap::new();
        let mut over_budget = false;
        for (index, name) in names[start..end].iter().enumerate() {
            if rejected.contains_key(*name) {
                continue;
            }
            let probe = format!("{prefix}constant_value_{index}");
            let declaration = format!(
                "#line 1 \"{probe}\"\n_Static_assert(__builtin_constant_p(({name})), \"pgrx_constant_initializer\");\nstatic __typeof__(({name})) {probe} __attribute__((unused)) = ({name});\n"
            );
            if source.len().saturating_add(declaration.len()) > limits.source_bytes {
                over_budget = true;
                break;
            }
            source.extend_from_slice(declaration.as_bytes());
            probes.insert(probe, (*name).clone());
        }
        let result = if over_budget {
            Err(FrontendError::Output("typed constant probes exceed their source budget".into()))
        } else {
            probe_values(scanner, frontend, directory, &source, original, &probes, *limits)
        };
        match result {
            Ok(found) => {
                for name in &names[start..end] {
                    if rejected.contains_key(*name) {
                        continue;
                    }
                    if let Some(constant) = found.get(*name) {
                        constants.insert((*name).clone(), constant.clone());
                    } else {
                        rejected.insert((*name).clone(), "Clang did not resolve a canonical integer type and constant value within 64 bits".into());
                    }
                }
            }
            Err(error) if optional_failure(&error) => {
                // A primary diagnostic names its own isolated witness. Remove
                // only those candidates and require a clean compiler pass for
                // the remainder; malformed siblings cannot consume the entire
                // binary-isolation budget before valid constants are visited.
                let failed = if let FrontendError::CompilerFailed { diagnostics, .. } = &error {
                    probes
                        .iter()
                        .filter_map(|(probe, name)| {
                            diagnostics
                                .lines()
                                .any(|line| {
                                    line.starts_with(&format!("{probe}:"))
                                        && (line.contains(": error:")
                                            || line.contains(": fatal error:"))
                                })
                                .then_some(name.clone())
                        })
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                };
                if !failed.is_empty() {
                    for name in failed {
                        rejected.insert(
                            name,
                            "Clang rejected the independently typed constant probe".into(),
                        );
                    }
                    pending.push((start, end));
                    continue;
                }
                if end - start > 1 {
                    let middle = start + (end - start) / 2;
                    pending.push((middle, end));
                    pending.push((start, middle));
                } else {
                    rejected.insert(
                        names[start].clone(),
                        "Clang rejected the independently typed constant probe".into(),
                    );
                }
            }
            Err(error) => return Err(error),
        }
    }
    Ok(constants)
}

/// Evaluate bounded typed constant witnesses with libclang and verify the driver accepts the same
/// source.
fn probe_values(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    directory: &Path,
    source: &[u8],
    original: &[u8],
    probes: &BTreeMap<String, String>,
    limits: ExpansionLimits,
) -> Result<BTreeMap<String, IntegerConstant>, FrontendError> {
    let arguments = probe_arguments(frontend, directory, source)?;
    crate::frontend::preprocess(
        &frontend.profile().compiler.executable,
        &frontend.profile().header,
        &arguments,
        &["-fsyntax-only"],
    )?;
    let constants =
        scanner.with_declarations(&frontend.profile().header, &arguments, None, |unit| {
            let mut constants = BTreeMap::new();
            unit.get_entity().visit_children(|entity, _| {
                if entity.get_kind() != EntityKind::VarDecl {
                    return EntityVisitResult::Continue;
                }
                let Some(name) = entity.get_name().and_then(|name| probes.get(&name)) else {
                    return EntityVisitResult::Continue;
                };
                let Some(ty) = entity.get_type() else { return EntityVisitResult::Continue };
                let info = crate::frontend::type_info(ty);
                let TypeCategory::Integer(kind) = info.category else {
                    return EntityVisitResult::Continue;
                };
                let Some(integer) = frontend.profile().target.integers.get(&kind) else {
                    return EntityVisitResult::Continue;
                };
                let bits = info.size.and_then(|size| {
                    size.checked_mul(u64::from(frontend.profile().target.char_bits))
                });
                if integer.bits > 64 || bits != Some(u64::from(integer.bits)) {
                    return EntityVisitResult::Continue;
                }
                let value = match (entity.evaluate(), integer.signed) {
                    (Some(EvaluationResult::SignedInteger(value)), true) => {
                        Some(IntegerValue::Signed(value))
                    }
                    (Some(EvaluationResult::UnsignedInteger(value)), false) => {
                        Some(IntegerValue::Unsigned(value))
                    }
                    (Some(EvaluationResult::SignedInteger(value)), false) => {
                        u64::try_from(value).ok().map(IntegerValue::Unsigned)
                    }
                    (Some(EvaluationResult::UnsignedInteger(value)), true) => {
                        i64::try_from(value).ok().map(IntegerValue::Signed)
                    }
                    _ => None,
                };
                if let Some(value) = value {
                    constants
                        .insert(name.clone(), IntegerConstant { ty: info, value, literal: None });
                }
                EntityVisitResult::Continue
            });
            Ok(constants)
        })?;
    let mut checks = original.to_vec();
    for (name, constant) in &constants {
        let TypeCategory::Integer(kind) = constant.ty.category else {
            unreachable!("only canonical integer types enter the probe result")
        };
        let value = match constant.value {
            IntegerValue::Signed(i64::MIN) => "(-9223372036854775807LL - 1LL)".into(),
            IntegerValue::Signed(value) => format!("({value}LL)"),
            IntegerValue::Unsigned(value) => format!("{value}ULL"),
        };
        let declaration = format!(
            "_Static_assert(_Generic(({name}), {}: 1, default: 0), \"pgrx_constant_type\");\n_Static_assert(({name}) == ({value}), \"pgrx_constant_value\");\n",
            c_integer_name(kind),
        );
        checks.extend_from_slice(declaration.as_bytes());
    }
    if checks.len() > limits.source_bytes {
        return Err(FrontendError::Output(
            "constant verification exceeds its source budget".into(),
        ));
    }
    let arguments = probe_arguments(frontend, directory, &checks)?;
    crate::frontend::preprocess(
        &frontend.profile().compiler.executable,
        &frontend.profile().header,
        &arguments,
        &["-fsyntax-only"],
    )?;
    Ok(constants)
}

/// Render the fundamental C spelling for a verified integer identity in proof source.
fn c_integer_name(kind: IntegerKind) -> &'static str {
    match kind {
        IntegerKind::Bool => "_Bool",
        IntegerKind::Char => "char",
        IntegerKind::SignedChar => "signed char",
        IntegerKind::UnsignedChar => "unsigned char",
        IntegerKind::Short => "short",
        IntegerKind::UnsignedShort => "unsigned short",
        IntegerKind::Int => "int",
        IntegerKind::UnsignedInt => "unsigned int",
        IntegerKind::Long => "long",
        IntegerKind::UnsignedLong => "unsigned long",
        IntegerKind::LongLong => "long long",
        IntegerKind::UnsignedLongLong => "unsigned long long",
        IntegerKind::Int128 => "__int128",
        IntegerKind::UnsignedInt128 => "unsigned __int128",
    }
}

/// Replace candidates by fresh marker macros for an independent exact-token restoration comparison.
fn mask_constants(
    context: &ConstantContext<'_>,
    batch: &mut ExpansionBatch,
    eligible: &BTreeMap<String, Vec<Token>>,
    rejected: &mut BTreeMap<String, String>,
) -> Result<(), FrontendError> {
    let ConstantContext { scanner, frontend, directory, original, prefix, limits } = context;
    let mut source = original.to_vec();
    source.extend_from_slice(super::invocation_diagnostic_preamble(prefix).as_bytes());
    let mut markers = HashMap::new();
    for (index, name) in eligible.keys().enumerate() {
        let marker = super::constant_marker(prefix, index);
        let definition = format!("#undef {name}\n#define {name} {marker}\n");
        if source.len().saturating_add(definition.len()) > limits.source_bytes {
            for name in eligible.keys() {
                rejected.insert(
                    name.clone(),
                    "masked constant expansions exceed their source budget".into(),
                );
            }
            return Ok(());
        }
        source.extend_from_slice(definition.as_bytes());
        markers.insert(marker, name.as_str());
    }
    let mut prepared = Vec::new();
    for (index, result) in batch.results.values().enumerate() {
        let ExpansionResult::Expanded { expansion } = result else { continue };
        let definition = &frontend.environment().active[&expansion.name].definition;
        let body_start = if definition.kind == MacroKind::ObjectLike {
            1
        } else {
            parameters(definition)
                .expect("successfully expanded macros have an established parameter list")
                .1
        };
        let begin = format!("{prefix}retained_begin_{index}");
        let end = format!("{prefix}retained_end_{index}");
        let invocation = format!(
            "{begin}\n{}\n{end}\n",
            super::symbolic_invocation(definition, &expansion.symbolic_parameters)
        );
        if source.len().saturating_add(invocation.len()) > limits.source_bytes {
            for name in eligible.keys() {
                rejected.insert(
                    name.clone(),
                    "masked constant expansions exceed their source budget".into(),
                );
            }
            return Ok(());
        }
        source.extend_from_slice(invocation.as_bytes());
        prepared.push(Prepared {
            definition,
            parameters: expansion.parameters.clone(),
            body_start,
            markers: expansion.symbolic_parameters.clone(),
            begin,
            end,
            dependencies: Vec::new(),
            pastes: false,
        });
    }
    let bodies = if source.len() > limits.source_bytes {
        Err(FrontendError::Output("masked constant expansions exceed their source budget".into()))
    } else {
        expand_bodies(scanner, frontend, directory, &source, &prepared, *limits)
    };
    let bodies = match bodies {
        Ok(bodies) => bodies,
        Err(error) if optional_failure(&error) => {
            for name in eligible.keys() {
                rejected.insert(
                    name.clone(),
                    "Clang could not verify expansion with retained constant references".into(),
                );
            }
            return Ok(());
        }
        Err(error) => return Err(error),
    };
    let mut total_tokens = 0usize;
    for (item, mut body) in prepared.into_iter().zip(bodies) {
        super::restore_invocation_diagnostics(&mut body, prefix);
        let Some(ExpansionResult::Expanded { expansion }) =
            batch.results.get_mut(&item.definition.name)
        else {
            unreachable!("masked probes are constructed from the original expansion batch")
        };
        let mut reconstructed = Vec::new();
        let mut retained = BTreeSet::new();
        for token in &body {
            if let Some(&name) = markers.get(&token.spelling) {
                retained.insert(name);
                reconstructed.extend(eligible[name].iter().map(|token| token.spelling.as_str()));
            } else {
                reconstructed.push(token.spelling.as_str());
            }
            if reconstructed.len() > limits.macro_tokens {
                break;
            }
        }
        total_tokens = total_tokens.saturating_add(reconstructed.len());
        let expected = expansion.definition.tokens[item.body_start..]
            .iter()
            .filter(|token| token.kind != TokenKind::Comment)
            .map(|token| token.spelling.as_str());
        if reconstructed.len() > limits.macro_tokens
            || total_tokens > limits.total_tokens
            || !reconstructed.into_iter().eq(expected)
        {
            let affected = expansion
                .dependencies
                .iter()
                .map(|dependency| dependency.name.as_str())
                .filter(|name| eligible.contains_key(*name))
                .chain(retained.iter().copied())
                .collect::<BTreeSet<_>>();
            for name in affected {
                expansion.constant_fallbacks.push(ConstantFallback {
                    name: name.into(),
                    reason: "retaining this constant did not reproduce the original compiler-expanded token stream".into(),
                });
            }
            continue;
        }
        for token in &mut body {
            if let Some(&name) = markers.get(&token.spelling) {
                token.spelling = name.into();
                token.kind = TokenKind::Identifier;
            }
        }
        expansion.definition.tokens.truncate(item.body_start);
        expansion.definition.tokens.extend(body);
        let parameters = expansion
            .symbolic_parameters
            .iter()
            .enumerate()
            .map(|(index, marker)| (marker.as_str(), index))
            .collect::<HashMap<_, _>>();
        expansion.occurrences = expansion.definition.tokens[item.body_start..]
            .iter()
            .enumerate()
            .filter_map(|(token, item)| {
                parameters
                    .get(item.spelling.as_str())
                    .map(|&parameter| super::ParameterOccurrence { parameter, token })
            })
            .collect();
    }
    Ok(())
}
