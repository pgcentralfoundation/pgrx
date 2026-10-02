//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Retain only independently verified, atomic integer object-macro references.
//!
//! The ordinary expansion stays authoritative. A second pass masks constants;
//! substituting their original expansions must reproduce every original token.

use super::{
    ConstantFallback, ExpansionBatch, ExpansionLimits, ExpansionResult, Prepared, ProbeDirectory,
    extract_bodies, namespace, overlay, parameters, verify_environment,
    verify_original_environment,
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

const MAX_PROBE_ATTEMPTS: usize = 32;

struct ConstantContext<'a> {
    scanner: &'a MacroScanner,
    frontend: &'a FrontendOutput,
    directory: &'a Path,
    original: &'a [u8],
    prefix: &'a str,
    limits: ExpansionLimits,
}

pub(crate) fn retain_integer_constants(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    batch: &mut ExpansionBatch,
    limits: ExpansionLimits,
) -> Result<BTreeMap<String, IntegerConstant>, FrontendError> {
    verify_environment(frontend)?;
    crate::frontend::verify_input_files(&frontend.profile().inputs)?;
    let result = retain_inner(scanner, frontend, batch, limits);
    crate::frontend::verify_input_files(&frontend.profile().inputs)?;
    verify_environment(frontend)?;
    result
}

fn retain_inner(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    batch: &mut ExpansionBatch,
    limits: ExpansionLimits,
) -> Result<BTreeMap<String, IntegerConstant>, FrontendError> {
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
    let mut source = original.clone();
    let mut prepared = Vec::new();
    for (index, name) in names.iter().enumerate() {
        let definition = &frontend.environment().active[name].definition;
        let begin = format!("{prefix}constant_begin_{index}");
        let end = format!("{prefix}constant_end_{index}");
        let invocation = format!("{begin}\n{name}\n{end}\n");
        if source.len().saturating_add(invocation.len()) > limits.source_bytes {
            rejected.insert(name.clone(), "constant expansion exceeds its source budget".into());
            continue;
        }
        source.extend_from_slice(invocation.as_bytes());
        prepared.push(Prepared {
            definition,
            parameters: Vec::new(),
            body_start: 1,
            markers: Vec::new(),
            begin,
            end,
            dependencies: Vec::new(),
        });
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

fn reject_all(names: &BTreeSet<String>, rejected: &mut BTreeMap<String, String>, reason: &str) {
    for name in names {
        rejected.insert(name.clone(), reason.into());
    }
}

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
    let mut definitions = crate::frontend::tokenize_snapshot(scanner, &snapshot)?
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
            let probe = format!("{prefix}constant_value_{index}");
            let declaration = format!(
                "_Static_assert(__builtin_constant_p(({name})), \"pgrx_constant_initializer\");\nstatic __typeof__(({name})) {probe} __attribute__((unused)) = ({name});\n"
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
                    if let Some(constant) = found.get(*name) {
                        constants.insert((*name).clone(), constant.clone());
                    } else {
                        rejected.insert((*name).clone(), "Clang did not resolve a canonical integer type and constant value within 64 bits".into());
                    }
                }
            }
            Err(error) if optional_failure(&error) => {
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
    let constants = scanner.with_translation_unit(
        &frontend.profile().header,
        &arguments,
        None,
        |unit, _| {
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
        },
    )?;
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

fn mask_constants(
    context: &ConstantContext<'_>,
    batch: &mut ExpansionBatch,
    eligible: &BTreeMap<String, Vec<Token>>,
    rejected: &mut BTreeMap<String, String>,
) -> Result<(), FrontendError> {
    let ConstantContext { scanner, frontend, directory, original, prefix, limits } = context;
    let mut source = original.to_vec();
    let mut markers = HashMap::new();
    for (index, name) in eligible.keys().enumerate() {
        let marker = format!("{prefix}constant_ref_{index}");
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
        let (_, body_start) = parameters(definition)
            .expect("successfully expanded macros have an established parameter list");
        let begin = format!("{prefix}retained_begin_{index}");
        let end = format!("{prefix}retained_end_{index}");
        let invocation = format!(
            "{begin}\n{}({})\n{end}\n",
            expansion.name,
            expansion.symbolic_parameters.join(",")
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
            for name in retained {
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
