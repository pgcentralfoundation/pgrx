//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Verify direct compiler operations without treating them as native functions.

use super::{FrontendError, driver_arguments, run_compiler, tokenize_snapshot, type_info};
use crate::{
    BuiltinInfo, BuiltinKind, FrontendOutput, FunctionSignature, MacroScanner, TypeCategory,
};
use clang::{Entity, EntityKind, EntityVisitResult, Index, TypeKind};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const BYTE_SWAPS: &[(&str, u16)] =
    &[("__builtin_bswap16", 16), ("__builtin_bswap32", 32), ("__builtin_bswap64", 64)];
const MAX_SOURCE_BYTES: usize = 4 * 1024 * 1024;
const MAX_BODY_LINES: usize = 128;
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
pub(super) struct Proof {
    pub(super) supported: BTreeMap<String, BuiltinInfo>,
    pub(super) unavailable: BTreeMap<String, String>,
}

struct Candidate {
    name: &'static str,
    bits: u16,
    witness: String,
}

struct ProbeRange {
    name: &'static str,
    lines: std::ops::Range<u32>,
}

/// Prove at most three referenced operations. An original-file overlay preserves
/// include depth, filename and optimization-dependent header branches. Typed
/// libclang calls establish rank; the driver independently establishes result
/// identity and pure dynamic value flow under the original flags.
pub(super) fn prove(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
) -> Result<Proof, FrontendError> {
    let mut proof = Proof::default();
    let mut candidates = Vec::new();
    let mut prefix = "__pgrx_c_builtin_".to_owned();
    while frontend.environment().active.keys().any(|name| name.starts_with(&prefix))
        || frontend.declarations().types.keys().any(|name| name.starts_with(&prefix))
        || frontend.declarations().functions.keys().any(|name| name.starts_with(&prefix))
        || frontend.declarations().variables.keys().any(|name| name.starts_with(&prefix))
    {
        prefix.push('_');
    }
    for &(name, bits) in BYTE_SWAPS {
        if !frontend
            .environment()
            .active
            .values()
            .any(|active| active.definition.tokens.iter().any(|token| token.spelling == name))
        {
            continue;
        }
        let unavailable = if frontend.environment().active.contains_key(name) {
            Some("an active C macro shadows the compiler builtin")
        } else if frontend.declarations().functions.contains_key(name)
            || frontend.declarations().variables.contains_key(name)
            || frontend.declarations().types.contains_key(name)
        {
            Some("an original C declaration conflicts with the compiler builtin")
        } else if ["__has_builtin", "__builtin_types_compatible_p", "__typeof__", "_Static_assert"]
            .iter()
            .any(|helper| frontend.environment().active.contains_key(*helper))
        {
            Some("an active C macro shadows a builtin proof operation")
        } else {
            None
        };
        if let Some(reason) = unavailable {
            proof.unavailable.insert(name.into(), reason.into());
        } else {
            candidates.push(Candidate { name, bits, witness: format!("{prefix}{bits}") });
        }
    }
    if candidates.is_empty() {
        return Ok(proof);
    }
    let inputs = &frontend.profile().inputs;
    super::verify_input_files(inputs)?;
    let result = prove_inner(scanner, frontend, &prefix, &candidates, &mut proof);
    super::verify_input_files(inputs)?;
    result?;
    Ok(proof)
}

fn prove_inner(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    prefix: &str,
    candidates: &[Candidate],
    proof: &mut Proof,
) -> Result<(), FrontendError> {
    let files = ProbeFiles::new(frontend)?;
    let (typed_source, ranges) = source(&files.original, prefix, candidates, &BTreeMap::new())?;
    let index = Index::new(&scanner.clang, false, false);
    let arguments = driver_arguments(&frontend.profile().arguments, &[], None);
    let mut parser = index.parser(&frontend.profile().header);
    parser
        .arguments(&arguments)
        .skip_function_bodies(false)
        .unsaved(&[clang::Unsaved::new(&frontend.profile().header, &typed_source)]);
    let unit = parser.parse().map_err(|source| crate::Error::Parse {
        header: frontend.profile().header.clone(),
        source,
    })?;
    let diagnostics = unit.get_diagnostics();
    if diagnostics.len() > 256 {
        return Err(FrontendError::Output("builtin probes exceed the diagnostic budget".into()));
    }
    for diagnostic in diagnostics {
        if !matches!(
            diagnostic.get_severity(),
            clang::diagnostic::Severity::Error | clang::diagnostic::Severity::Fatal
        ) {
            continue;
        }
        let location = diagnostic.get_location().get_expansion_location();
        let candidate = location
            .file
            .filter(|file| file.get_path() == frontend.profile().header)
            .and_then(|_| ranges.iter().find(|range| range.lines.contains(&location.line)));
        if let Some(range) = candidate {
            proof
                .unavailable
                .insert(range.name.into(), format!("typed builtin probe rejected: {diagnostic}"));
        } else {
            return Err(FrontendError::Environment(format!(
                "original-header builtin probe failed: {diagnostic}"
            )));
        }
    }
    unit.get_entity().visit_children(|entity, _| {
        if entity.get_kind() != EntityKind::FunctionDecl || !entity.is_definition() {
            return EntityVisitResult::Continue;
        }
        let Some(candidate) = entity
            .get_name()
            .and_then(|name| candidates.iter().find(|candidate| candidate.witness == name))
        else {
            return EntityVisitResult::Continue;
        };
        if proof.unavailable.contains_key(candidate.name) {
            return EntityVisitResult::Continue;
        }
        match prototype(entity, candidate, frontend) {
            Ok(signature) => {
                proof.supported.insert(
                    candidate.name.into(),
                    BuiltinInfo { kind: BuiltinKind::ByteSwap { bits: candidate.bits }, signature },
                );
            }
            Err(reason) => {
                proof.unavailable.insert(candidate.name.into(), reason);
            }
        }
        EntityVisitResult::Continue
    });
    for candidate in candidates {
        if !proof.supported.contains_key(candidate.name) {
            proof.unavailable.entry(candidate.name.into()).or_insert_with(|| {
                "libclang did not establish a direct builtin call and prototype".into()
            });
        }
    }
    let remaining = candidates
        .iter()
        .filter(|candidate| proof.supported.contains_key(candidate.name))
        .collect::<Vec<_>>();
    // One ordinary batch, or that batch plus at most three individual retries.
    // A rejected operation must not remove its unrelated peers.
    let mut pending = vec![remaining];
    let mut runs = 0;
    while let Some(remaining) = pending.pop() {
        if remaining.is_empty() {
            continue;
        }
        runs += 1;
        if runs > 4 {
            return Err(FrontendError::Output(
                "builtin LLVM probes exceed the four-run budget".into(),
            ));
        }
        let selected = remaining
            .iter()
            .map(|candidate| Candidate {
                name: candidate.name,
                bits: candidate.bits,
                witness: candidate.witness.clone(),
            })
            .collect::<Vec<_>>();
        let (source, _) = source(&files.original, prefix, &selected, &proof.supported)?;
        files.write(&source)?;
        verify_snapshot(scanner, frontend, &files)?;
        let arguments = files.arguments(frontend, &["-S", "-emit-llvm", "-o", "-"])?;
        let output = match run_compiler(&frontend.profile().compiler.executable, &arguments) {
            Ok(output) => output,
            Err(FrontendError::CompilerFailed { diagnostics, .. }) => {
                if remaining.len() > 1 {
                    pending.extend(remaining.into_iter().map(|candidate| vec![candidate]));
                } else {
                    let candidate = remaining[0];
                    proof.supported.remove(candidate.name);
                    proof.unavailable.insert(
                        candidate.name.into(),
                        format!(
                            "driver rejected the builtin prototype or dynamic witness: {}",
                            diagnostics.chars().take(2048).collect::<String>()
                        ),
                    );
                }
                continue;
            }
            Err(error) => return Err(error),
        };
        for candidate in remaining {
            if let Err(reason) = llvm_witness(&output.stdout, &candidate.witness, candidate.bits) {
                proof.supported.remove(candidate.name);
                proof.unavailable.insert(candidate.name.into(), reason);
            }
        }
    }
    Ok(())
}

fn prototype(
    function: Entity<'_>,
    candidate: &Candidate,
    frontend: &FrontendOutput,
) -> Result<FunctionSignature, String> {
    let mut calls = Vec::new();
    for body in function
        .get_children()
        .into_iter()
        .filter(|child| child.get_kind() == EntityKind::CompoundStmt)
    {
        body.visit_children(|entity, _| {
            if entity.get_kind() == EntityKind::CallExpr {
                calls.push(entity);
            }
            EntityVisitResult::Recurse
        });
    }
    let [call] = calls.as_slice() else {
        return Err("typed witness must contain exactly one direct builtin call".into());
    };
    let declaration = call
        .get_reference()
        .filter(|declaration| {
            declaration.get_kind() == EntityKind::FunctionDecl
                && declaration.get_name().as_deref() == Some(candidate.name)
                && !declaration.is_definition()
        })
        .ok_or("typed witness does not call the compiler builtin directly")?;
    let ty = declaration.get_type().ok_or("builtin declaration has no type")?.get_canonical_type();
    if ty.get_kind() != TypeKind::FunctionPrototype || ty.is_variadic() {
        return Err("builtin requires a nonvariadic function prototype".into());
    }
    if ty.get_calling_convention() != Some(clang::CallingConvention::Cdecl) {
        return Err("builtin prototype lacks the default C calling convention".into());
    }
    let parameters = ty.get_argument_types().ok_or("builtin prototype has no parameter types")?;
    let [parameter] = parameters.as_slice() else {
        return Err("byte-swap builtin requires exactly one parameter".into());
    };
    let parameter = type_info(parameter.get_canonical_type());
    let result = type_info(
        ty.get_result_type().ok_or("builtin prototype has no result type")?.get_canonical_type(),
    );
    let TypeCategory::Integer(kind) = result.category else {
        return Err("byte-swap builtin result is not a fundamental integer".into());
    };
    let facts = frontend
        .profile()
        .target
        .integers
        .get(&kind)
        .ok_or("builtin integer type is absent from target facts")?;
    if parameter != result
        || result.is_const
        || result.is_volatile
        || facts.signed
        || facts.bits != u32::from(candidate.bits)
        || frontend.profile().target.char_bits != 8
        || result.size != Some(u64::from(candidate.bits / 8))
        || call.get_type().map(|ty| type_info(ty.get_canonical_type())) != Some(result.clone())
        || call.get_arguments().is_none_or(|arguments| arguments.len() != 1)
    {
        return Err(
            "byte-swap builtin lacks matching unsigned parameter/result identity and width".into(),
        );
    }
    Ok(FunctionSignature {
        result,
        parameters: Some(vec![parameter]),
        variadic: false,
        calling_convention: ty.get_calling_convention().map(|convention| format!("{convention:?}")),
    })
}

fn prototype_message(candidate: &Candidate) -> String {
    format!("PGRX builtin prototype {}", candidate.name)
}

fn source(
    original: &str,
    prefix: &str,
    candidates: &[Candidate],
    signatures: &BTreeMap<String, BuiltinInfo>,
) -> Result<(String, Vec<ProbeRange>), FrontendError> {
    let mut source = original.to_owned();
    source.push_str("\n\n");
    let mut ranges = Vec::new();
    let mut line = u32::try_from(source.bytes().filter(|byte| *byte == b'\n').count() + 1)
        .map_err(|_| FrontendError::Output("builtin probe line count overflow".into()))?;
    for candidate in candidates {
        let mut probe = format!("#if __has_builtin({})\n", candidate.name);
        if let Some(info) = signatures.get(candidate.name) {
            writeln!(
                probe,
                "_Static_assert(__builtin_types_compatible_p(__typeof__({}(0)), {}), \"{}\");",
                candidate.name,
                info.signature.result.canonical_spelling,
                prototype_message(candidate)
            )
            .expect("String output");
        }
        writeln!(probe, "__typeof__({0}(0)) {1}(__typeof__({0}(0)) {2}value);\n__typeof__({0}(0)) {1}(__typeof__({0}(0)) {2}value) {{ return {0}({2}value); }}\n#endif", candidate.name, candidate.witness, prefix).expect("String output");
        let end = line
            .checked_add(probe.bytes().filter(|byte| *byte == b'\n').count() as u32)
            .ok_or_else(|| FrontendError::Output("builtin probe line count overflow".into()))?;
        ranges.push(ProbeRange { name: candidate.name, lines: line..end });
        line = end;
        source.push_str(&probe);
    }
    if source.len() > MAX_SOURCE_BYTES {
        return Err(FrontendError::Output("builtin probes exceed the 4 MiB source budget".into()));
    }
    Ok((source, ranges))
}

struct ProbeFiles {
    directory: PathBuf,
    source: PathBuf,
    overlay: PathBuf,
    original: String,
}

impl ProbeFiles {
    fn new(frontend: &FrontendOutput) -> Result<Self, FrontendError> {
        let header = &frontend.profile().header;
        let io_error = |source| FrontendError::CompilerIo { compiler: header.clone(), source };
        let mut original = Vec::new();
        fs::File::open(header)
            .map_err(io_error)?
            .take(MAX_SOURCE_BYTES as u64 + 1)
            .read_to_end(&mut original)
            .map_err(io_error)?;
        if original.len() > MAX_SOURCE_BYTES {
            return Err(FrontendError::Output(
                "builtin main file exceeds the 4 MiB source budget".into(),
            ));
        }
        let original = String::from_utf8(original)
            .map_err(|_| FrontendError::Output("builtin main file is not UTF-8".into()))?;
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| {
                FrontendError::Output(format!("could not name builtin probe: {error}"))
            })?
            .as_nanos();
        let number = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir()
            .join(format!("pgrx-c-macros-builtins-{}-{nonce}-{number}", std::process::id()));
        fs::create_dir(&directory).map_err(io_error)?;
        let files = Self {
            source: directory.join("probe.c"),
            overlay: directory.join("overlay.json"),
            directory,
            original,
        };
        let overlay = serde_json::json!({
            "version": 0,
            "use-external-names": false,
            "roots": [{"type": "file", "name": header, "external-contents": files.source}],
        });
        fs::write(&files.overlay, overlay.to_string()).map_err(io_error)?;
        Ok(files)
    }

    fn write(&self, source: &str) -> Result<(), FrontendError> {
        fs::write(&self.source, source)
            .map_err(|source| FrontendError::CompilerIo { compiler: self.source.clone(), source })
    }

    fn arguments(
        &self,
        frontend: &FrontendOutput,
        options: &[&str],
    ) -> Result<Vec<String>, FrontendError> {
        let mut arguments = driver_arguments(&frontend.profile().arguments, options, None);
        arguments.extend(["-ivfsoverlay".into(), super::path_string(&self.overlay)?.into()]);
        arguments.push(super::path_string(&frontend.profile().header)?.into());
        Ok(arguments)
    }
}

impl Drop for ProbeFiles {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn verify_snapshot(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    files: &ProbeFiles,
) -> Result<(), FrontendError> {
    let output = run_compiler(
        &frontend.profile().compiler.executable,
        &files.arguments(frontend, &["-E", "-dM"])?,
    )?;
    let actual = tokenize_snapshot(scanner, &output.stdout)?;
    let expected = &frontend.environment().active;
    if actual.len() != expected.len()
        || actual.iter().any(|definition| {
            expected.get(&definition.name).is_none_or(|active| {
                definition.kind != active.definition.kind
                    || !definition
                        .tokens
                        .iter()
                        .filter(|token| token.kind != crate::TokenKind::Comment)
                        .map(|token| token.spelling.as_str())
                        .eq(active
                            .definition
                            .tokens
                            .iter()
                            .filter(|token| token.kind != crate::TokenKind::Comment)
                            .map(|token| token.spelling.as_str()))
            })
        })
    {
        return Err(FrontendError::Environment(
            "builtin overlay changed the final macro environment; inspected inputs differ".into(),
        ));
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Origin {
    Parameter,
    Swapped,
}

struct Slot {
    value: Option<Origin>,
    alignment: Option<u64>,
}

fn llvm_witness(ir: &str, witness: &str, bits: u16) -> Result<(), String> {
    let marker = format!("@{witness}(");
    let mut functions = ir.lines().filter(|line| line.starts_with("define "));
    let header = functions
        .find(|line| line.contains(&marker))
        .ok_or("driver did not emit a dynamic builtin witness")?;
    if !header.trim_end().ends_with('{') {
        return Err("LLVM builtin witness has no function body".into());
    }
    let (prefix, tail) = header.split_once(&marker).expect("matched witness marker");
    let (parameter, _) = tail.split_once(')').ok_or("malformed LLVM witness parameter")?;
    let ty = format!("i{bits}");
    let parameter_words = parameter.split_whitespace().collect::<Vec<_>>();
    if parameter_words.len() < 2
        || prefix.split_whitespace().last() != Some(ty.as_str())
        || parameter_words.first().copied() != Some(ty.as_str())
        || parameter.contains(',')
        || !parameter_words[1..parameter_words.len().saturating_sub(1)]
            .iter()
            .all(|word| matches!(*word, "noundef" | "zeroext" | "signext"))
    {
        return Err("LLVM builtin witness has an incompatible parameter/result width".into());
    }
    let parameter = parameter_words
        .last()
        .copied()
        .filter(|word| word.starts_with('%'))
        .ok_or("LLVM builtin witness has no dynamic scalar parameter")?;
    let body = ir.lines().skip_while(|line| *line != header).skip(1);
    let mut values = BTreeMap::from([(parameter, Origin::Parameter)]);
    let mut slots = BTreeMap::<&str, Slot>::new();
    let mut calls = 0;
    let mut returned = false;
    let mut closed = false;
    let mut labelled = false;
    let mut instructions = 0;
    for line in body {
        let line = line.split(';').next().unwrap_or_default().trim();
        if line == "}" {
            closed = true;
            break;
        }
        if line.is_empty()
            || line.starts_with("#dbg_value(")
            || line.starts_with("#dbg_declare(")
            || line.starts_with("#dbg_assign(")
        {
            continue;
        }
        if line.ends_with(':') && instructions == 0 && !labelled {
            labelled = true;
            continue;
        }
        instructions += 1;
        if instructions > MAX_BODY_LINES || returned {
            return Err("LLVM builtin witness exceeds its straight-line instruction budget".into());
        }
        let (destination, operation) =
            line.split_once(" = ").map_or((None, line), |(name, op)| (Some(name), op));
        if operation.starts_with("call void @llvm.dbg.declare(")
            || operation.starts_with("call void @llvm.dbg.value(")
            || operation.starts_with("call void @llvm.dbg.assign(")
        {
            continue;
        }
        let parts = operation.split(',').map(str::trim).collect::<Vec<_>>();
        let words = parts[0].split_whitespace().collect::<Vec<_>>();
        match words.as_slice() {
            ["alloca", allocated] if *allocated == ty && destination.is_some() => {
                let destination = destination.expect("matched alloca destination");
                let alignment = access_suffix(&parts[1..])?;
                if slots.len() >= 8
                    || values.contains_key(destination)
                    || slots.insert(destination, Slot { value: None, alignment }).is_some()
                {
                    return Err("LLVM builtin witness has invalid scalar stack storage".into());
                }
            }
            ["store", stored, value] if *stored == ty && destination.is_none() => {
                let origin = values
                    .get(value)
                    .copied()
                    .ok_or("LLVM builtin store lost dynamic value identity")?;
                let pointer = local_pointer(parts.get(1).copied(), &ty)?;
                let slot =
                    slots.get_mut(pointer).ok_or("LLVM builtin store targets nonlocal storage")?;
                if access_suffix(&parts[2..])? != slot.alignment {
                    return Err("LLVM builtin store changes its scalar slot alignment".into());
                }
                slot.value = Some(origin);
            }
            ["load", loaded] if *loaded == ty && destination.is_some() => {
                let pointer = local_pointer(parts.get(1).copied(), &ty)?;
                let slot =
                    slots.get(pointer).ok_or("LLVM builtin load targets nonlocal storage")?;
                if access_suffix(&parts[2..])? != slot.alignment {
                    return Err("LLVM builtin load changes its scalar slot alignment".into());
                }
                let origin = slot.value.ok_or("LLVM builtin load is not definitely initialized")?;
                let destination = destination.expect("matched load destination");
                if slots.contains_key(destination) || values.insert(destination, origin).is_some() {
                    return Err("LLVM builtin witness redefines a value".into());
                }
            }
            ["ret", returned_type, value] if *returned_type == ty && destination.is_none() => {
                if access_suffix(&parts[1..])?.is_some() {
                    return Err("LLVM builtin return has unsupported operands".into());
                }
                if values.get(value) != Some(&Origin::Swapped) || calls != 1 {
                    return Err("LLVM builtin return is not the single byte-swap result".into());
                }
                returned = true;
            }
            _ => {
                let operation = operation
                    .strip_prefix("tail ")
                    .or_else(|| operation.strip_prefix("notail "))
                    .unwrap_or(operation);
                let Some(call) = operation.strip_prefix(&format!("call {ty} @llvm.bswap.{ty}("))
                else {
                    return Err(format!(
                        "LLVM builtin witness has unsupported effects: {operation}"
                    ));
                };
                let (argument, suffix) =
                    call.split_once(')').ok_or("malformed LLVM byte-swap call")?;
                let argument = argument
                    .strip_prefix(&format!("{ty} "))
                    .ok_or("LLVM byte-swap call has an incompatible argument width")?;
                if values.get(argument) != Some(&Origin::Parameter) || calls != 0 {
                    return Err(
                        "LLVM byte-swap call does not consume the dynamic parameter exactly once"
                            .into(),
                    );
                }
                let suffix = suffix.trim_start();
                let suffix = if let Some(attribute) = suffix.strip_prefix('#') {
                    let digits = attribute.bytes().take_while(u8::is_ascii_digit).count();
                    if digits == 0 {
                        return Err("LLVM byte-swap call has malformed attributes".into());
                    }
                    &attribute[digits..]
                } else {
                    suffix
                }
                .trim();
                if !suffix.is_empty() {
                    let suffix = suffix
                        .strip_prefix(',')
                        .ok_or("LLVM byte-swap call has unsupported trailing operands")?;
                    if access_suffix(&suffix.split(',').map(str::trim).collect::<Vec<_>>())?
                        .is_some()
                    {
                        return Err("LLVM byte-swap call has unsupported alignment operands".into());
                    }
                }
                let destination = destination.ok_or("LLVM byte-swap call discards its result")?;
                if slots.contains_key(destination)
                    || values.insert(destination, Origin::Swapped).is_some()
                {
                    return Err("LLVM builtin witness redefines a value".into());
                }
                calls += 1;
            }
        }
    }
    if closed && returned && calls == 1 {
        Ok(())
    } else {
        Err("LLVM builtin witness has no proved byte-swap return".into())
    }
}

fn access_suffix(parts: &[&str]) -> Result<Option<u64>, String> {
    let mut alignment = None;
    for part in parts {
        let words = part.split_whitespace().collect::<Vec<_>>();
        match words.as_slice() {
            ["align", value] if alignment.is_none() => {
                alignment = Some(
                    value
                        .parse::<u64>()
                        .ok()
                        .filter(|value| value.is_power_of_two())
                        .ok_or("LLVM scalar access has invalid alignment")?,
                );
            }
            ["!dbg" | "!DIAssignID" | "!tbaa", value] if value.starts_with('!') => {}
            _ => return Err("LLVM scalar access has unsupported trailing operands".into()),
        }
    }
    Ok(alignment)
}

fn local_pointer<'a>(part: Option<&'a str>, ty: &str) -> Result<&'a str, String> {
    let mut words = part.ok_or("LLVM scalar access has no pointer")?.split_whitespace();
    let storage = words.next().ok_or("LLVM scalar access has no pointer type")?;
    let pointer = words.next().ok_or("LLVM scalar access has no pointer value")?;
    if !matches!(storage, "ptr") && storage != format!("{ty}*") || words.next().is_some() {
        return Err("LLVM scalar access has incompatible pointer storage".into());
    }
    Ok(pointer)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn witness(body: &str) -> String {
        format!("define dso_local i32 @probe(i32 noundef %0) {{\n{body}\n}}\n")
    }

    #[test]
    fn dynamic_swap_accepts_optimized_and_initialized_o0_copies_with_debug_records() {
        for body in [
            "  %1 = tail call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %1",
            "  %2 = alloca i32, align 4\n  store i32 %0, ptr %2, align 4\n  #dbg_declare(ptr %2, !1, !DIExpression(), !2)\n  %3 = load i32, ptr %2, align 4, !dbg !2\n  %4 = call i32 @llvm.bswap.i32(i32 %3), !dbg !2\n  ret i32 %4, !dbg !2",
            "entry:\n  %2 = alloca i32, align 4\n  store i32 %0, i32* %2, align 4\n  call void @llvm.dbg.declare(metadata i32* %2, metadata !1, metadata !DIExpression())\n  %3 = load i32, i32* %2, align 4\n  %4 = call i32 @llvm.bswap.i32(i32 %3)\n  ret i32 %4",
        ] {
            llvm_witness(&witness(body), "probe", 32).unwrap();
        }
    }

    #[test]
    fn byte_swap_proof_rejects_extra_effects_wrong_width_and_lost_value_identity() {
        for body in [
            "  %1 = add i32 %0, 1\n  %2 = call i32 @llvm.bswap.i32(i32 %1)\n  ret i32 %2",
            "  %1 = call i32 @llvm.bswap.i32(i32 0)\n  ret i32 %1",
            "  %1 = call i32 @llvm.bswap.i32(i32 %0)\n  %2 = call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %2",
            "  %1 = call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %0",
            "  %1 = call i16 @llvm.bswap.i16(i16 %0)\n  ret i16 %1",
            "  %2 = alloca i32, align 4\n  %3 = load i32, ptr %2, align 4\n  %4 = call i32 @llvm.bswap.i32(i32 %3)\n  ret i32 %4",
            "  %2 = alloca i32, align 4\n  store volatile i32 %0, ptr %2, align 4\n  %3 = load i32, ptr %2, align 4\n  %4 = call i32 @llvm.bswap.i32(i32 %3)\n  ret i32 %4",
            "  store i32 %0, ptr @global, align 4\n  %1 = call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %1",
            "  %1 = call i32 @llvm.bswap.i32(i32 %0)\n  br label %exit\nexit:\n  ret i32 %1",
            "  %2 = alloca i32, i64 %0, align 4\n  store i32 %0, ptr %2, align 4\n  %3 = load i32, ptr %2, align 4\n  %4 = call i32 @llvm.bswap.i32(i32 %3)\n  ret i32 %4",
            "  %2 = alloca i32, align 2\n  store i32 %0, ptr %2, align 4\n  %3 = load i32, ptr %2, align 4\n  %4 = call i32 @llvm.bswap.i32(i32 %3)\n  ret i32 %4",
            "entry:\nextra:\n  %1 = call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %1",
            "  %1 = call i32 @llvm.bswap.i32(i32 %0) [ \"deopt\"() ]\n  ret i32 %1",
        ] {
            assert!(llvm_witness(&witness(body), "probe", 32).is_err(), "accepted {body}");
        }
        assert!(llvm_witness(&witness("  ret i32 %0"), "probe", 16).is_err());
        assert!(llvm_witness(&witness("  ret i32 %0"), "missing", 32).is_err());
    }

    #[test]
    fn malformed_or_incomplete_llvm_witnesses_are_errors() {
        for ir in [
            "define i32 @probe(i32) {\n  ret i32 0\n}",
            "define i32 @probe() {\n  ret i32 0\n}",
            "define i32 @probe(i32 %0)\n  %1 = call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %1\n}",
            "define i32 @probe(i32 %0) {\n  %1 = call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %1",
            "define i32 @probe(i32 %0) {\n  %1 = call i32 @llvm.bswap.i32(i32 %0)",
        ] {
            assert!(llvm_witness(ir, "probe", 32).is_err(), "accepted {ir}");
        }
    }
}
