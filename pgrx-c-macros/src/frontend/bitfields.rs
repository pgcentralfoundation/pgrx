//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Establish bitfield expression types without guessing promotion from its width.

//! C bitfield types and promotions cannot be recovered from width alone. This phase builds
//! bounded typed probes for readable and writable expressions, checking both compiler paths.
//! LLVM access witnesses compare original, unaligned alias, and enclosing-record operations
//! before admitting volatile capabilities. Failed candidates are isolated rather than assigning
//! unchecked facts to unrelated fields.

use super::{FrontendError, driver_arguments, run_compiler_with_input, type_info};
use crate::{BitfieldFacts, FrontendOutput, MacroScanner, TypeInfo};
use clang::{EntityKind, EntityVisitResult};
use std::collections::BTreeMap;
use std::fmt::Write;

/// Bound bitfield candidate collection before allocating the compiler probe batch.
const MAX_FIELDS: usize = 16_384;
/// Bound one constructed probe source before invoking Clang or allocating additional instrumentation.
const MAX_SOURCE_BYTES: usize = 4 * 1024 * 1024;
/// Bound compiler-pass isolation work for candidate-specific failures.
const MAX_PROBE_RUNS: usize = 128;

/// A named bitfield and enclosing contexts whose type and volatile access behavior require compiler
/// probes.
struct Candidate {
    /// Canonical record/member key under which established bitfield facts are stored.
    key: String,
    /// A usable original C record spelling for compiler witnesses and native capabilities.
    record_type: String,
    /// Original named C bitfield whose expression types and access units need proof.
    field: String,
    /// Whether the original field permits assignment/update probes instead of read-only promotion.
    writable: bool,
    /// Unqualified original field type spelling used by write witnesses.
    declared_type: String,
    /// Named enclosing record/member paths checked for compatible volatile access behavior.
    parents: Vec<(String, String)>,
}

/// An expression that is a bitfield has implementation-specific type properties.
/// Both the driver and libclang must accept the exact typed witnesses before the
/// emitter may build an access capability. Failed candidates are isolated rather
/// than preventing unrelated records from receiving capabilities.
pub(super) fn probe(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
) -> Result<BTreeMap<String, BitfieldFacts>, FrontendError> {
    let declarations = frontend.declarations();
    let mut parents = BTreeMap::<String, Vec<(String, String)>>::new();
    for (canonical, record) in &declarations.records {
        let Some(record_type) = record_type(frontend, canonical) else { continue };
        for field in &record.fields {
            if field.ty.category == crate::TypeCategory::Record
                && let Some(name) = field.name.as_ref().filter(|name| identifier(name))
            {
                parents
                    .entry(field.ty.canonical_spelling.clone())
                    .or_default()
                    .push((record_type.clone(), name.clone()));
            }
        }
    }
    let mut candidates = Vec::new();
    for (canonical, record) in &declarations.records {
        let record_type = record_type(frontend, canonical);
        let Some(record_type) = record_type else { continue };
        for field in &record.fields {
            let Some(name) = field.name.as_ref().filter(|name| identifier(name)) else { continue };
            if field.bit_width.is_none() || field.is_anonymous {
                continue;
            }
            candidates.push(Candidate {
                key: format!("{canonical}::{name}"),
                record_type: record_type.clone(),
                field: name.clone(),
                writable: !field.ty.is_const,
                declared_type: field
                    .ty
                    .canonical_spelling
                    .replace("const ", "")
                    .replace("volatile ", ""),
                parents: parents.get(canonical).cloned().unwrap_or_default(),
            });
            if candidates.len() > MAX_FIELDS {
                return Err(FrontendError::Output(
                    "bitfield probes exceed the 16384-field budget".into(),
                ));
            }
        }
    }
    let mut result = BTreeMap::new();
    let mut pending = vec![(0, candidates.len())];
    let mut runs = 0;
    while let Some((start, end)) = pending.pop() {
        if start == end {
            continue;
        }
        runs += 1;
        if runs > MAX_PROBE_RUNS {
            // A field without witnesses receives no capability. Preserve the
            // independently proved fields instead of failing unrelated macros.
            break;
        }
        let source = source(frontend, &candidates[start..end])?;
        match collect(scanner, frontend, &source) {
            Ok(types) => {
                let access = volatile_units(frontend, &source)?;
                for (index, candidate) in candidates[start..end].iter().enumerate() {
                    let Some(promoted) = types.get(&format!("__pgrx_bitfield_{index}_promoted"))
                    else {
                        continue;
                    };
                    result.insert(
                        candidate.key.clone(),
                        BitfieldFacts {
                            record_type: candidate.record_type.clone(),
                            promoted: promoted.clone(),
                            assignment: types
                                .get(&format!("__pgrx_bitfield_{index}_assigned"))
                                .cloned(),
                            assignment_promoted: types
                                .get(&format!("__pgrx_bitfield_{index}_assigned_promoted"))
                                .cloned(),
                            postfix: types
                                .get(&format!("__pgrx_bitfield_{index}_postfix"))
                                .cloned(),
                            postfix_promoted: types
                                .get(&format!("__pgrx_bitfield_{index}_postfix_promoted"))
                                .cloned(),
                            unaligned_volatile_access: access_compatible(&access, index, candidate),
                        },
                    );
                }
            }
            Err(
                FrontendError::CompilerFailed { .. }
                | FrontendError::Discovery(crate::Error::Diagnostics(_)),
            ) => {
                if end - start > 1 {
                    let middle = start + (end - start) / 2;
                    pending.extend([(start, middle), (middle, end)]);
                }
            }
            Err(error) => return Err(error),
        }
    }
    Ok(result)
}

/// Find a usable C record spelling or typedef for probes without inventing names for anonymous
/// declarations.
fn record_type(frontend: &FrontendOutput, canonical: &str) -> Option<String> {
    let record = frontend.declarations().records.get(canonical)?;
    if record.name.is_some() && !canonical.contains('(') {
        Some(canonical.replace("const ", "").replace("volatile ", ""))
    } else {
        frontend.declarations().types.iter().find_map(|(name, ty)| {
            (identifier(name) && ty.canonical_spelling == canonical).then(|| name.clone())
        })
    }
}

/// Restrict probe designators to ordinary C identifiers before embedding them into generated source.
fn identifier(value: &str) -> bool {
    let mut bytes = value.bytes();
    bytes.next().is_some_and(|byte| byte == b'_' || byte.is_ascii_alphabetic())
        && bytes.all(|byte| byte == b'_' || byte.is_ascii_alphanumeric())
}

/// Generate bounded type and access witnesses for original, unaligned, and enclosing-record bitfield
/// expressions.
fn source(frontend: &FrontendOutput, candidates: &[Candidate]) -> Result<String, FrontendError> {
    let quoted = super::c_header_path(&frontend.profile().header)?;
    let mut source = format!("#include \"{quoted}\"\n");
    for (index, candidate) in candidates.iter().enumerate() {
        let place = format!("(({} *)0)->{}", candidate.record_type, candidate.field);
        writeln!(source, "typedef __typeof__(+({place})) __pgrx_bitfield_{index}_promoted;")
            .expect("String output");
        if candidate.writable {
            writeln!(source, "typedef __typeof__(({place}) = 0) __pgrx_bitfield_{index}_assigned;\ntypedef __typeof__(+(({place}) = 0)) __pgrx_bitfield_{index}_assigned_promoted;").expect("String output");
            writeln!(source,"typedef __typeof__(({place})++) __pgrx_bitfield_{index}_postfix;\ntypedef __typeof__(+(({place})++)) __pgrx_bitfield_{index}_postfix_promoted;").expect("String output");
        }
        let alias = format!("__pgrx_bitfield_{index}_alias");
        writeln!(source,"typedef {} {alias} __attribute__((aligned(1),may_alias));\n_Static_assert(_Alignof({alias}) == 1, \"bitfield alias alignment\");\n_Static_assert(sizeof({alias}) == sizeof({}), \"bitfield alias layout\");",candidate.record_type,candidate.record_type).expect("String output");
        for (label, ty, path) in std::iter::once((
            "original".into(),
            candidate.record_type.clone(),
            candidate.field.clone(),
        ))
        .chain(std::iter::once(("alias".into(), alias, candidate.field.clone())))
        .chain(candidate.parents.iter().enumerate().map(|(parent, (ty, field))| {
            (format!("parent_{parent}"), ty.clone(), format!("{field}.{}", candidate.field))
        })) {
            writeln!(source,"__pgrx_bitfield_{index}_promoted __pgrx_bitfield_{index}_{label}_get(const volatile {ty} *p) {{ return +(p->{path}); }}").expect("String output");
            if candidate.writable {
                writeln!(source,"__pgrx_bitfield_{index}_promoted __pgrx_bitfield_{index}_{label}_set(volatile {ty} *p, {} value) {{ return +(p->{path} = value); }}",candidate.declared_type).expect("String output");
            }
        }
        if source.len() > MAX_SOURCE_BYTES {
            return Err(FrontendError::Output(
                "bitfield probes exceed the 4 MiB source budget".into(),
            ));
        }
    }
    Ok(source)
}

/// Extract LLVM volatile access units so generated bitfield capabilities can preserve the compiler
/// operation.
fn volatile_units(
    frontend: &FrontendOutput,
    source: &str,
) -> Result<BTreeMap<String, Vec<(String, String)>>, FrontendError> {
    let profile = frontend.profile();
    let mut arguments =
        driver_arguments(&profile.arguments, &["-S", "-emit-llvm", "-O1", "-o", "-"], None);
    arguments.push("-".into());
    let ir =
        run_compiler_with_input(&profile.compiler.executable, &arguments, Some(source.to_owned()))?
            .stdout;
    let mut functions = BTreeMap::new();
    let mut current = None;
    for line in ir.lines() {
        if line.starts_with("define ") {
            current = line
                .split_once('@')
                .and_then(|(_, tail)| tail.split_once('('))
                .map(|(name, _)| name.to_owned())
                .filter(|name| name.starts_with("__pgrx_bitfield_"));
            if let Some(name) = &current {
                functions.insert(name.clone(), Vec::new());
            }
        } else if line == "}" {
            current = None;
        } else if let Some(name) = &current {
            let words = line.split_whitespace().collect::<Vec<_>>();
            for (index, word) in words.iter().enumerate() {
                if *word == "volatile"
                    && index > 0
                    && matches!(words[index - 1], "load" | "store")
                    && let Some(ty) = words.get(index + 1)
                {
                    functions
                        .get_mut(name)
                        .expect("function header initialized units")
                        .push((words[index - 1].into(), ty.trim_end_matches(',').into()));
                }
            }
        }
    }
    Ok(functions)
}

/// Require the tested access units to agree across the relevant bitfield representations.
fn access_compatible(
    units: &BTreeMap<String, Vec<(String, String)>>,
    index: usize,
    candidate: &Candidate,
) -> bool {
    let modes = if candidate.writable { &["get", "set"][..] } else { &["get"][..] };
    modes.iter().all(|operation| {
        let Some(original) = units.get(&format!("__pgrx_bitfield_{index}_original_{operation}"))
        else {
            return false;
        };
        !original.is_empty()
            && units.get(&format!("__pgrx_bitfield_{index}_alias_{operation}")) == Some(original)
            && candidate.parents.iter().enumerate().all(|(parent, _)| {
                units.get(&format!("__pgrx_bitfield_{index}_parent_{parent}_{operation}"))
                    == Some(original)
            })
    })
}

/// Require both Clang paths to accept a typed bitfield probe before copying its expression type
/// facts.
fn collect(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    source: &str,
) -> Result<BTreeMap<String, TypeInfo>, FrontendError> {
    let profile = frontend.profile();
    let mut arguments = driver_arguments(&profile.arguments, &["-fsyntax-only"], None);
    arguments.push("-".into());
    run_compiler_with_input(&profile.compiler.executable, &arguments, Some(source.to_owned()))?;
    let header = std::env::temp_dir().join("pgrx-c-macros-bitfields.h");
    scanner
        .with_declarations(&header, &profile.arguments, Some(source), |unit| {
            let mut result = BTreeMap::new();
            unit.get_entity().visit_children(|entity, _| {
                if entity.get_kind() == EntityKind::TypedefDecl
                    && let Some(name) =
                        entity.get_name().filter(|name| name.starts_with("__pgrx_bitfield_"))
                    && let Some(ty) = entity.get_typedef_underlying_type()
                {
                    result.insert(name, type_info(ty));
                }
                EntityVisitResult::Continue
            });
            Ok(result)
        })
        .map_err(FrontendError::from)
}
