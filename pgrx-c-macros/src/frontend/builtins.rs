//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Verify direct compiler operations without treating them as native functions.

//! Compiler builtins do not have ordinary PostgreSQL binding symbols. This phase proves a
//! bounded operation set using typed prototypes and LLVM witnesses from the matched driver.
//! Only accepted signatures with the expected operand identity and effects become capabilities;
//! unsupported witnesses produce reasons. Temporary overlays preserve the original source
//! context while isolating instrumentation.

use super::{FrontendError, driver_arguments, run_compiler, tokenize_snapshot, type_info};
use crate::{
    BuiltinInfo, BuiltinKind, FrontendOutput, FunctionSignature, IntegerKind, MacroScanner,
    TypeCategory,
};
use clang::{Entity, EntityKind, EntityVisitResult, Index, TypeKind};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// The reviewed builtin names and operation identities eligible for signature and effect proof.
const OPERATIONS: &[(&str, BuiltinKind)] = &[
    ("__builtin_bswap16", BuiltinKind::ByteSwap { bits: 16 }),
    ("__builtin_bswap32", BuiltinKind::ByteSwap { bits: 32 }),
    ("__builtin_bswap64", BuiltinKind::ByteSwap { bits: 64 }),
    ("__builtin_expect", BuiltinKind::Expect),
];
/// Bound one constructed probe source before invoking Clang or allocating additional instrumentation.
const MAX_SOURCE_BYTES: usize = 4 * 1024 * 1024;
/// Bound LLVM witness parsing so unsupported compiler bodies remain an explicit refusal.
const MAX_BODY_LINES: usize = 128;
/// Distinguish temporary builtin probe directories within one process without reusing another
/// invocation’s files.
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

/// Separate builtin capabilities with established signature/effect proofs from per-name unavailable
/// reasons.
#[derive(Default)]
pub(super) struct Proof {
    /// Capabilities whose operation signatures and effects were established by both compiler paths.
    pub(super) supported: BTreeMap<String, BuiltinInfo>,
    /// Per-name explanations for compiler operations that could not establish the required proof.
    pub(super) unavailable: BTreeMap<String, String>,
}

/// The intended builtin operation and fresh typed/LLVM witness name to be checked by both compiler
/// paths.
struct Candidate {
    /// Reviewed builtin spelling selected for prototype and effect proof.
    name: &'static str,
    /// Intended compiler operation whose semantics the witnesses must establish.
    kind: BuiltinKind,
    /// Target-derived operand/result width expected by the builtin proof.
    bits: u16,
    /// Fresh probe symbol used to associate typed and LLVM observations with one builtin candidate.
    witness: String,
}

/// Derive only the reviewed operation metadata used to construct builtin witnesses.
impl Candidate {
    /// Return the operation-specific operand count used by typed builtin probes.
    fn arity(&self) -> usize {
        match self.kind {
            BuiltinKind::ByteSwap { .. } => 1,
            BuiltinKind::Expect => 2,
        }
    }
}

/// The source lines owned by one builtin candidate for attributing compiler diagnostics.
struct ProbeRange {
    /// The original C identifier retained for reports, symbol lookup, and readable generated output.
    name: &'static str,
    /// Physical probe-source lines used to attribute diagnostics to one candidate.
    lines: std::ops::Range<u32>,
}

/// Prove at most four referenced operations. An original-file overlay preserves
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
    for &(name, kind) in OPERATIONS {
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
            let (bits, suffix) = match kind {
                BuiltinKind::ByteSwap { bits } => (bits, bits.to_string()),
                BuiltinKind::Expect => {
                    let Some(bits) = frontend
                        .profile()
                        .target
                        .integers
                        .get(&IntegerKind::Long)
                        .and_then(|facts| u16::try_from(facts.bits).ok())
                    else {
                        proof.unavailable.insert(
                            name.into(),
                            "builtin expect requires a verified C long representation".into(),
                        );
                        continue;
                    };
                    (bits, "expect".into())
                }
            };
            candidates.push(Candidate { name, kind, bits, witness: format!("{prefix}{suffix}") });
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

/// Reconcile typed prototypes and driver LLVM witnesses for the bounded candidate set, recording
/// per-name refusals.
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
                proof
                    .supported
                    .insert(candidate.name.into(), BuiltinInfo { kind: candidate.kind, signature });
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
    // One ordinary batch, or that batch plus at most four individual retries.
    // A rejected operation must not remove its unrelated peers.
    let mut pending = vec![remaining];
    let mut runs = 0;
    while let Some(remaining) = pending.pop() {
        if remaining.is_empty() {
            continue;
        }
        runs += 1;
        if runs > 1 + OPERATIONS.len() {
            return Err(FrontendError::Output(
                "builtin LLVM probes exceed the five-run budget".into(),
            ));
        }
        let selected = remaining
            .iter()
            .map(|candidate| Candidate {
                name: candidate.name,
                kind: candidate.kind,
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
            if let Err(reason) =
                llvm_witness(&output.stdout, &candidate.witness, candidate.bits, candidate.kind)
            {
                proof.supported.remove(candidate.name);
                proof.unavailable.insert(candidate.name.into(), reason);
            }
        }
    }
    Ok(())
}

/// Validate the builtin call result and parameter identity against the intended operation and target.
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
    let parameters = ty
        .get_argument_types()
        .ok_or("builtin prototype has no parameter types")?
        .into_iter()
        .map(|parameter| type_info(parameter.get_canonical_type()))
        .collect::<Vec<_>>();
    if parameters.len() != candidate.arity() {
        return Err("compiler builtin prototype has an incompatible arity".into());
    }
    let result = type_info(
        ty.get_result_type().ok_or("builtin prototype has no result type")?.get_canonical_type(),
    );
    let TypeCategory::Integer(kind) = result.category else {
        return Err("compiler builtin result is not a fundamental integer".into());
    };
    let facts = frontend
        .profile()
        .target
        .integers
        .get(&kind)
        .ok_or("builtin integer type is absent from target facts")?;
    let identity = match candidate.kind {
        BuiltinKind::ByteSwap { .. } => !facts.signed,
        BuiltinKind::Expect => kind == IntegerKind::Long && facts.signed && facts.rank == 4,
    };
    if parameters.iter().any(|parameter| parameter != &result)
        || result.is_const
        || result.is_volatile
        || !identity
        || facts.bits != u32::from(candidate.bits)
        || frontend.profile().target.char_bits != 8
        || result.size != Some(u64::from(candidate.bits / 8))
        || call.get_type().map(|ty| type_info(ty.get_canonical_type())) != Some(result.clone())
        || call.get_arguments().is_none_or(|arguments| arguments.len() != candidate.arity())
    {
        return Err(match candidate.kind {
            BuiltinKind::ByteSwap { .. } => {
                "byte-swap builtin lacks matching unsigned parameter/result identity and width"
            }
            BuiltinKind::Expect => "expect builtin lacks the exact C long(long, long) identity",
        }
        .into());
    }
    Ok(FunctionSignature {
        result,
        parameters: Some(parameters),
        variadic: false,
        calling_convention: ty.get_calling_convention().map(|convention| format!("{convention:?}")),
    })
}

/// Produce the common reason when a candidate signature cannot prove the expected builtin contract.
fn prototype_message(candidate: &Candidate) -> String {
    format!("PGRX builtin prototype {}", candidate.name)
}

/// Build bounded typed and dynamic witnesses in the original preprocessing context.
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
        let zero_arguments = vec!["0"; candidate.arity()].join(", ");
        let result_type = format!("__typeof__({}({zero_arguments}))", candidate.name);
        if let Some(info) = signatures.get(candidate.name) {
            writeln!(
                probe,
                "_Static_assert(__builtin_types_compatible_p({result_type}, {}), \"{}\");",
                info.signature.result.canonical_spelling,
                prototype_message(candidate)
            )
            .expect("String output");
        }
        let values =
            (0..candidate.arity()).map(|index| format!("{prefix}value{index}")).collect::<Vec<_>>();
        let parameters = values
            .iter()
            .map(|value| format!("{result_type} {value}"))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            probe,
            "{result_type} {0}({parameters});\n{result_type} {0}({parameters}) {{ return {1}({2}); }}\n#endif",
            candidate.witness,
            candidate.name,
            values.join(", "),
        )
        .expect("String output");
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

/// Own builtin instrumentation and an overlay while retaining original bytes for environment
/// verification.
struct ProbeFiles {
    /// The unique owned scratch directory containing this phase’s temporary overlays and probes.
    directory: PathBuf,
    /// Owned instrumentation source file mapped onto the original header by the overlay.
    source: PathBuf,
    /// Owned VFS mapping that preserves the original main-file identity during driver probes.
    overlay: PathBuf,
    /// Original main-file bytes retained to prove instrumentation preserves its preprocessing
    /// context.
    original: String,
}

/// Manage owned instrumentation/overlays while keeping the original main-file preprocessing context.
impl ProbeFiles {
    /// Create owned builtin probe files and retain original header bytes for overlay verification.
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

    /// Replace the owned probe source contents between bounded witness phases.
    fn write(&self, source: &str) -> Result<(), FrontendError> {
        fs::write(&self.source, source)
            .map_err(|source| FrontendError::CompilerIo { compiler: self.source.clone(), source })
    }

    /// Construct the driver arguments that apply the owned overlay to the original header context.
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

/// Release resources owned by ProbeFiles even when a compiler or proof phase exits early.
impl Drop for ProbeFiles {
    /// Clean up the temporary builtin probe files owned by this proof.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

/// Require instrumented preprocessing to retain the inspected macro environment before accepting
/// builtin evidence.
fn verify_snapshot(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    files: &ProbeFiles,
) -> Result<(), FrontendError> {
    let output = run_compiler(
        &frontend.profile().compiler.executable,
        &files.arguments(frontend, &["-E", "-dM"])?,
    )?;
    let actual = tokenize_snapshot(scanner, &output.stdout, &frontend.profile().arguments)?;
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

/// The operand identity tracked through a builtin LLVM witness, including a byte-swapped result.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Origin {
    /// The original dynamic operand identity tracked through permitted LLVM copies.
    Parameter(
        /// The original dynamic operand identity tracked through permitted LLVM copies.
        usize,
    ),
    /// A witness value derived from the original operand by exactly the admitted byte swap.
    Swapped,
}

/// A supported LLVM local copy with tracked value identity and required alignment.
struct Slot {
    /// Operand identity stored in a supported local LLVM copy.
    value: Option<Origin>,
    /// Recorded local access alignment checked when accepting stack-copy witnesses.
    alignment: Option<u64>,
}

/// Recognize a bounded LLVM body whose result identity and allowed effects prove the intended
/// operation.
fn llvm_witness(ir: &str, witness: &str, bits: u16, kind: BuiltinKind) -> Result<(), String> {
    let marker = format!("@{witness}(");
    let mut functions = ir.lines().filter(|line| line.starts_with("define "));
    let header = functions
        .find(|line| line.contains(&marker))
        .ok_or("driver did not emit a dynamic builtin witness")?;
    if !header.trim_end().ends_with('{') {
        return Err("LLVM builtin witness has no function body".into());
    }
    let (prefix, tail) = header.split_once(&marker).expect("matched witness marker");
    let (parameters, _) = tail.split_once(')').ok_or("malformed LLVM witness parameters")?;
    let ty = format!("i{bits}");
    if prefix.split_whitespace().last() != Some(ty.as_str()) {
        return Err("LLVM builtin witness has an incompatible parameter/result width".into());
    }
    let arity = match kind {
        BuiltinKind::ByteSwap { .. } => 1,
        BuiltinKind::Expect => 2,
    };
    let parameters = parameters.split(',').collect::<Vec<_>>();
    if parameters.len() != arity {
        return Err("LLVM builtin witness has an incompatible parameter arity".into());
    }
    let mut values = BTreeMap::new();
    for (index, parameter) in parameters.into_iter().enumerate() {
        let words = parameter.split_whitespace().collect::<Vec<_>>();
        if words.len() < 2
            || words.first().copied() != Some(ty.as_str())
            || !words[1..words.len().saturating_sub(1)].iter().all(|word| {
                matches!(*word, "noundef" | "zeroext" | "signext")
                    || *word == "returned" && kind == BuiltinKind::Expect && index == 0
            })
        {
            return Err("LLVM builtin witness has incompatible scalar parameter attributes".into());
        }
        let parameter = words
            .last()
            .copied()
            .filter(|word| word.starts_with('%'))
            .ok_or("LLVM builtin witness has no dynamic scalar parameter")?;
        if values.insert(parameter, Origin::Parameter(index)).is_some() {
            return Err("LLVM builtin witness repeats a scalar parameter".into());
        }
    }
    let body = ir.lines().skip_while(|line| *line != header).skip(1);
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
                let valid = match kind {
                    BuiltinKind::ByteSwap { .. } => {
                        values.get(value) == Some(&Origin::Swapped) && calls == 1
                    }
                    BuiltinKind::Expect => values.get(value) == Some(&Origin::Parameter(0)),
                };
                if !valid {
                    return Err(
                        "LLVM builtin return does not preserve the operation's result identity"
                            .into(),
                    );
                }
                returned = true;
            }
            _ => {
                let operation = operation
                    .strip_prefix("tail ")
                    .or_else(|| operation.strip_prefix("notail "))
                    .unwrap_or(operation);
                let intrinsic = match kind {
                    BuiltinKind::ByteSwap { .. } => "bswap",
                    BuiltinKind::Expect => "expect",
                };
                let Some(call) =
                    operation.strip_prefix(&format!("call {ty} @llvm.{intrinsic}.{ty}("))
                else {
                    return Err(format!(
                        "LLVM builtin witness has unsupported effects: {operation}"
                    ));
                };
                let (arguments, suffix) =
                    call.split_once(')').ok_or("malformed LLVM builtin intrinsic call")?;
                let arguments = arguments.split(',').collect::<Vec<_>>();
                if arguments.len() != arity || calls != 0 {
                    return Err(
                        "LLVM builtin intrinsic has incompatible arity or repetition".into()
                    );
                }
                for (index, argument) in arguments.into_iter().enumerate() {
                    let argument = argument
                        .trim()
                        .strip_prefix(&format!("{ty} "))
                        .ok_or("LLVM builtin intrinsic has an incompatible argument width")?;
                    if values.get(argument) != Some(&Origin::Parameter(index)) {
                        return Err(
                            "LLVM builtin intrinsic lost its dynamic parameter identity".into()
                        );
                    }
                }
                let suffix = suffix.trim_start();
                let suffix = if let Some(attribute) = suffix.strip_prefix('#') {
                    let digits = attribute.bytes().take_while(u8::is_ascii_digit).count();
                    if digits == 0 {
                        return Err("LLVM builtin intrinsic has malformed attributes".into());
                    }
                    &attribute[digits..]
                } else {
                    suffix
                }
                .trim();
                if !suffix.is_empty() {
                    let suffix = suffix
                        .strip_prefix(',')
                        .ok_or("LLVM builtin intrinsic has unsupported trailing operands")?;
                    if access_suffix(&suffix.split(',').map(str::trim).collect::<Vec<_>>())?
                        .is_some()
                    {
                        return Err(
                            "LLVM builtin intrinsic has unsupported alignment operands".into()
                        );
                    }
                }
                let destination =
                    destination.ok_or("LLVM builtin intrinsic discards its result")?;
                let origin = match kind {
                    BuiltinKind::ByteSwap { .. } => Origin::Swapped,
                    BuiltinKind::Expect => Origin::Parameter(0),
                };
                if slots.contains_key(destination) || values.insert(destination, origin).is_some() {
                    return Err("LLVM builtin witness redefines a value".into());
                }
                calls += 1;
            }
        }
    }
    if closed && returned {
        Ok(())
    } else {
        Err("LLVM builtin witness has no proved operation return".into())
    }
}

/// Parse load/store pointer and alignment details needed to track supported O0 value copies.
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

/// Recognize the bounded local pointer spelling used by LLVM stack-copy witnesses.
fn local_pointer<'a>(part: Option<&'a str>, ty: &str) -> Result<&'a str, String> {
    let mut words = part.ok_or("LLVM scalar access has no pointer")?.split_whitespace();
    let storage = words.next().ok_or("LLVM scalar access has no pointer type")?;
    let pointer = words.next().ok_or("LLVM scalar access has no pointer value")?;
    if !matches!(storage, "ptr") && storage != format!("{ty}*") || words.next().is_some() {
        return Err("LLVM scalar access has incompatible pointer storage".into());
    }
    Ok(pointer)
}

/// Exercise this phase’s semantic boundaries with owned fixtures.
/// These regressions check accepted proofs and explicit refusals without changing production
/// headers or weakening the C identity and evaluation contracts.
#[cfg(test)]
mod tests {
    use super::*;

    /// Build a byte-swap LLVM test body for the production witness recognizer.
    fn witness(body: &str) -> String {
        format!("define dso_local i32 @probe(i32 noundef %0) {{\n{body}\n}}\n")
    }

    /// Build a branch-hint LLVM test body for checking first-operand identity and allowed effects.
    fn expect_witness(body: &str) -> String {
        format!("define dso_local i64 @probe(i64 noundef %0, i64 noundef %1) {{\n{body}\n}}\n")
    }

    /// Checks dynamic swap accepts optimized and initialized O0 copies with debug records.
    #[test]
    fn dynamic_swap_accepts_optimized_and_initialized_o0_copies_with_debug_records() {
        for body in [
            "  %1 = tail call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %1",
            "  %2 = alloca i32, align 4\n  store i32 %0, ptr %2, align 4\n  #dbg_declare(ptr %2, !1, !DIExpression(), !2)\n  %3 = load i32, ptr %2, align 4, !dbg !2\n  %4 = call i32 @llvm.bswap.i32(i32 %3), !dbg !2\n  ret i32 %4, !dbg !2",
            "entry:\n  %2 = alloca i32, align 4\n  store i32 %0, i32* %2, align 4\n  call void @llvm.dbg.declare(metadata i32* %2, metadata !1, metadata !DIExpression())\n  %3 = load i32, i32* %2, align 4\n  %4 = call i32 @llvm.bswap.i32(i32 %3)\n  ret i32 %4",
        ] {
            llvm_witness(&witness(body), "probe", 32, BuiltinKind::ByteSwap { bits: 32 }).unwrap();
        }
    }

    /// Checks byte swap proof rejects extra effects wrong width and lost value identity.
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
            "  ret i32 %0",
        ] {
            assert!(
                llvm_witness(&witness(body), "probe", 32, BuiltinKind::ByteSwap { bits: 32 })
                    .is_err(),
                "accepted {body}"
            );
        }
        assert!(
            llvm_witness(&witness("  ret i32 %0"), "probe", 16, BuiltinKind::ByteSwap { bits: 16 })
                .is_err()
        );
        assert!(
            llvm_witness(
                &witness("  ret i32 %0"),
                "missing",
                32,
                BuiltinKind::ByteSwap { bits: 32 }
            )
            .is_err()
        );
        assert!(llvm_witness("define i32 @probe(i32 returned %0) {\n  %1 = call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %1\n}", "probe", 32, BuiltinKind::ByteSwap { bits: 32 }).is_err());
    }

    /// Checks malformed or incomplete LLVM witnesses are errors.
    #[test]
    fn malformed_or_incomplete_llvm_witnesses_are_errors() {
        for ir in [
            "define i32 @probe(i32) {\n  ret i32 0\n}",
            "define i32 @probe() {\n  ret i32 0\n}",
            "define i32 @probe(i32 %0)\n  %1 = call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %1\n}",
            "define i32 @probe(i32 %0) {\n  %1 = call i32 @llvm.bswap.i32(i32 %0)\n  ret i32 %1",
            "define i32 @probe(i32 %0) {\n  %1 = call i32 @llvm.bswap.i32(i32 %0)",
        ] {
            assert!(
                llvm_witness(ir, "probe", 32, BuiltinKind::ByteSwap { bits: 32 }).is_err(),
                "accepted {ir}"
            );
        }
    }

    /// Checks expect proves first operand identity with stack copies or a pure hint.
    #[test]
    fn expect_proves_first_operand_identity_with_stack_copies_or_a_pure_hint() {
        for body in [
            "  ret i64 %0",
            "  %2 = tail call i64 @llvm.expect.i64(i64 %0, i64 %1)\n  ret i64 %2",
            "entry:\n  %2 = alloca i64, align 8\n  %3 = alloca i64, align 8\n  store i64 %0, ptr %2, align 8\n  store i64 %1, ptr %3, align 8\n  #dbg_declare(ptr %2, !1, !DIExpression(), !2)\n  %4 = load i64, ptr %2, align 8, !dbg !2\n  %5 = load i64, ptr %3, align 8\n  ret i64 %4, !dbg !2",
            "  %2 = alloca i64, align 8\n  %3 = alloca i64, align 8\n  store i64 %0, ptr %2, align 8\n  store i64 %1, ptr %3, align 8\n  %4 = load i64, ptr %2, align 8\n  %5 = load i64, ptr %3, align 8\n  %6 = call i64 @llvm.expect.i64(i64 %4, i64 %5) #2, !dbg !2\n  ret i64 %6",
        ] {
            llvm_witness(&expect_witness(body), "probe", 64, BuiltinKind::Expect).unwrap();
        }
        llvm_witness(
            "define i64 @probe(i64 noundef returned %first, i64 noundef %second) {\n  ret i64 %first\n}",
            "probe",
            64,
            BuiltinKind::Expect,
        )
        .unwrap();
    }

    /// Checks expect rejects reversed identity arithmetic and non hint effects.
    #[test]
    fn expect_rejects_reversed_identity_arithmetic_and_non_hint_effects() {
        for body in [
            "  ret i64 %1",
            "  %2 = call i64 @llvm.expect.i64(i64 %1, i64 %0)\n  ret i64 %2",
            "  %2 = add i64 %0, 0\n  ret i64 %2",
            "  %2 = call i64 @llvm.bswap.i64(i64 %0)\n  ret i64 %2",
            "  %2 = call i64 @llvm.expect.i64(i64 %0, i64 %1)\n  %3 = call i64 @llvm.expect.i64(i64 %0, i64 %1)\n  ret i64 %3",
            "  %2 = call i64 @llvm.expect.i64(i64 %0, i64 1)\n  ret i64 %2",
            "  %2 = call i64 @llvm.expect.i64(i64 %0, i64 poison)\n  ret i64 %2",
            "  %2 = call i64 @llvm.expect.i64(i64 %0, i64 %1) [ \"deopt\"() ]\n  ret i64 %2",
            "  %2 = alloca i64, align 8\n  %3 = load i64, ptr %2, align 8\n  ret i64 %3",
            "  %2 = alloca i64, align 8\n  store i64 %0, ptr %2, align 8\n  store i64 %1, ptr %2, align 8\n  %3 = load i64, ptr %2, align 8\n  ret i64 %3",
            "  store i64 %1, ptr @global, align 8\n  ret i64 %0",
        ] {
            assert!(
                llvm_witness(&expect_witness(body), "probe", 64, BuiltinKind::Expect).is_err(),
                "accepted {body}"
            );
        }
        for ir in [
            "define i64 @probe(i64 %0) {\n  ret i64 %0\n}",
            "define i64 @probe(i64 %0, i64) {\n  ret i64 %0\n}",
            "define i64 @probe(i64 %0, i64 %0) {\n  ret i64 %0\n}",
            "define i64 @probe(i64 %0, i64 returned %1) {\n  ret i64 %0\n}",
            "define i64 @probe(i64 %0, i32 %1) {\n  ret i64 %0\n}",
            "define i64 @probe(i64 %0, i64 %1) {\n  ret i64 %0",
        ] {
            assert!(llvm_witness(ir, "probe", 64, BuiltinKind::Expect).is_err(), "accepted {ir}");
        }
    }
}
