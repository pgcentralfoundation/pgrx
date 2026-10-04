//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Lower trusted C expression analyses to hygienic Rust macros and storage adapters.
//!
//! Each C parameter occurrence remains a separate Rust expression occurrence. Results
//! retain their C identity behind the semantic support's evaluated `CExpression` wrapper;
//! callers use `.get()` to extract native storage. Shared adapters reconcile compiler
//! declarations with actual binding types and preserve raw aggregate and enum validity.
//! Place and native-call operations retain explicit caller safety obligations. Emission
//! does not establish differential validation, and the generic helpers are runtime calls.
//!
//! Lowering is admitted only after analysis establishes the selected C profile and
//! its declaration facts. Batch generation first plans the required capabilities,
//! reconciles them with the fresh Rust bindings, and propagates rejected dependencies
//! so exported macros never call a missing generated definition.

/// Consume trusted analysis facts and preserve structured rejection context during lowering.
use crate::analysis::{
    AnalysisStatus, AnalyzedExpression, ConstCapability, MacroAnalysis, ResolvedConstant,
    SkipReason, SkipReasonCode, TypeExpression,
};
/// Preserve target integer width, rank, literal value, and overflow semantics in emitted helpers.
use crate::model::{IntegerKind, IntegerType, IntegerValue, SignedOverflow};
/// Match analyzed C arena nodes and operators without reparsing header tokens during emission.
use crate::syntax::{
    BinaryOperator, ExpressionKind, IntegerLiteral, NodeId, TokenRange, UnaryOperator,
};
/// Combine immutable session provenance with target ABI assertions before producing reviewable source.
use crate::{AnalysisSession, MacroDefinition, support_generation::support_abi_assertions};
/// Serialize emission outcomes and binding facts for CLI reports and generation consumers.
use serde::Serialize;
/// Keep symbol catalogs and requested capability names deterministic and deduplicated.
use std::collections::{BTreeMap, BTreeSet};
/// Write source fragments directly into the output buffer without intermediate formatting streams.
use std::fmt::Write;

/// Hygienic path to the defining crate’s C semantic runtime used by exported expansions.
const SUPPORT: &str = "$crate::__pgrx_c_macros";
/// Bound one macro’s emitted source, including diagnostics and original-definition docs.
const MAX_EMISSION_BYTES: usize = 1024 * 1024;

/// Original-function address adapters that bypass callable thunks and Rust guard wrappers.
mod addresses;
/// Token normalization that preserves contextual C substitution at Rust macro call sites.
mod arguments;
/// Compiler-owned storage access for fields Rust cannot project directly.
mod bitfields;
/// Prototype identity and native callback ABI reconciliation.
mod callbacks;
/// Batch capability selection derived from C expression constraints.
mod demands;
/// Nominal enum identities and checked bridges to actual binding storage.
mod enumerations;
/// Optimization-hint planning that preserves C values and operand effects.
mod expectation;
/// Typed field projections, record registrations, and independent offset capabilities.
mod fields;
/// Native C thunks and guarded Rust adapters for callable declarations.
mod functions;
/// Lexically ordered statement lowering over the shared expression arena.
mod statements;
/// Contextual expression rendering for values, places, size operands, and effects.
mod typed;
/// Shared reconciliation of compiler C identity with fresh Rust binding storage.
mod types;

/// Names and values in the defining Rust crate, supplied by its binding generator.
///
/// These values are checked against independently resolved C constants before use.
/// Paths are relative to `$crate`; `macros` contains macros actually emitted there.
#[derive(Clone, Debug, Default, Serialize)]
pub struct BindingCatalog {
    /// Binding values and paths used to preserve symbols after checking agreement with Clang.
    pub integer_constants: BTreeMap<String, IntegerBinding>,
    /// Definitions available for preserved cross-macro calls in this defining crate.
    pub macros: BTreeSet<String>,
    /// Actual function binding prototypes and any generated native replacements.
    pub functions: BTreeMap<String, crate::FunctionBinding>,
    /// Binding-owned record constructors and field storage, independent of C layout facts.
    pub records: BTreeMap<String, crate::RecordBinding>,
    /// Rust typedef aliases available for storage reconciliation and readable casts.
    pub types: BTreeMap<String, crate::AliasBinding>,
    /// Rust enum constructors whose discriminants require checked C storage conversion.
    pub enums: BTreeMap<String, crate::EnumBinding>,
    /// Global binding paths and storage for C object reads, addresses, and mutations.
    pub variables: BTreeMap<String, crate::VariableBinding>,
    /// Binding-owned integer newtypes whose storage adapters have been verified.
    pub integer_storage: BTreeMap<String, IntegerKind>,
    /// Bindgen storage-unit descriptions used to anchor original-C bitfield adapters.
    pub bitfields: BTreeMap<String, crate::BitfieldBinding>,
    /// Generated field capabilities, populated by batch lowering from both catalogs.
    #[serde(skip)]
    pub field_capabilities: BTreeMap<String, String>,
    /// Offset-only field metadata, independent of load or projection support.
    #[serde(skip)]
    pub offset_capabilities: BTreeMap<String, Option<String>>,
    /// Rejected offset paths whose reasons must remain visible to dependent macros.
    #[serde(skip)]
    pub offset_unavailable: BTreeMap<String, String>,
    /// Optional crate-relative guard for native adapters that may call PostgreSQL.
    #[serde(skip)]
    pub ffi_boundary: Option<Vec<String>>,
    /// Native-call rejections retained instead of silently using an unverified binding.
    #[serde(skip)]
    pub function_unavailable: BTreeMap<String, String>,
    /// Verified nominal callback markers and exact native signature storage.
    #[serde(skip)]
    pub callback_capabilities: BTreeMap<String, crate::CallbackBinding>,
    /// Callback identity or ABI failures that prevent unsafe inference during rendering.
    #[serde(skip)]
    pub callback_unavailable: BTreeMap<String, String>,
    /// Enum layout or representation failures retained for contextual lowering.
    #[serde(skip)]
    pub enum_unavailable: BTreeMap<String, String>,
    /// Getters for the original C function addresses, distinct from Rust call wrappers.
    #[serde(skip)]
    pub function_addresses: BTreeMap<String, crate::FunctionAddressBinding>,
}

/// A symbol reference and its independently comparable binding value.
///
/// The representation records binding rewrites so preserving a name does not lose
/// the conversion required by a nonprimitive Rust storage type.
#[derive(Clone, Debug, Serialize)]
pub struct IntegerBinding {
    /// Crate-relative binding path, validated before insertion into hygienic macro source.
    pub path: Vec<String>,
    /// Bindgen’s value, checked against Clang without rewriting the binding.
    pub value: IntegerValue,
    /// Storage bridge needed to wrap this binding value with its C integer identity.
    pub representation: IntegerBindingRepresentation,
}

/// Storage introduced by the binding generator, including pgrx's OID rewrite.
#[derive(Clone, Copy, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegerBindingRepresentation {
    /// Ordinary Rust integer storage whose bits can enter the C value wrapper directly.
    #[default]
    Primitive,
    /// pgrx’s OID newtype, requiring its storage conversion rather than a primitive assumption.
    Oid,
}

/// The original analysis and the outcome of lowering it under the same trusted profile.
#[derive(Clone, Debug, Serialize)]
pub struct MacroEmission {
    /// Trusted source analysis retained alongside output for provenance and skip reporting.
    pub analysis: MacroAnalysis,
    /// Lowering result under the binding catalog and C profile used for this generation.
    pub status: EmissionStatus,
}

/// Either reviewable Rust source or a structured explanation of why lowering was rejected.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EmissionStatus {
    /// Rust source was produced; downstream compilation and C comparison are separate checks.
    Emitted {
        /// Formatted macro definition retaining source docs and profile checks.
        rust: String,
        /// Advertised evaluation capability; emission currently produces runtime expressions.
        const_capability: ConstCapability,
    },
    /// No definition is available; callers must propagate the reason to dependent macros.
    Skipped {
        /// Structured root rejection with token location and provenance for reporting.
        reason: SkipReason,
    },
}

/// Generate one macro from an immutable analysis session, never from caller-mutated IR.
///
/// The defining crate must provide the matching semantic support at
/// `__pgrx_c_macros`. Each emitted string includes a target guard. `$crate` paths
/// keep the helper bindings valid when that crate is renamed.
pub fn emit(session: &AnalysisSession<'_>, name: &str) -> MacroEmission {
    emit_with_bindings(session, name, &BindingCatalog::default())
}

/// Emit with verified references to constants and other generated macros.
pub fn emit_with_bindings(
    session: &AnalysisSession<'_>,
    name: &str,
    bindings: &BindingCatalog,
) -> MacroEmission {
    emit_with_lowering(session, name, bindings, &mut None)
}

/// Apply admission gates before rendering, sharing storage reconciliation across a batch.
///
/// The optional lowering is populated only after a macro survives analysis and ABI
/// checks, so skipped batches do not build indexes they cannot use.
fn emit_with_lowering<'a>(
    session: &'a AnalysisSession<'_>,
    name: &str,
    bindings: &'a BindingCatalog,
    lowering: &mut Option<types::Lowering<'a>>,
) -> MacroEmission {
    let analysis = session.analyze(name);
    let lowered = if let Some(reason) = binding_mismatch(session, &analysis, bindings) {
        Err(reason)
    } else if let AnalysisStatus::Skipped { reason } = &analysis.status {
        Err(reason.clone())
    } else {
        let original = &session
            .frontend()
            .environment()
            .active
            .get(name)
            .expect("analyzed candidates come from the immutable final active macro map")
            .definition;
        support_abi_assertions(session.frontend().profile())
            .map_err(|error| {
                skip(&analysis, SkipReasonCode::UnsupportedProfile, error.to_string(), None)
            })
            .and_then(|assertions| {
                let lowering = lowering.get_or_insert_with(|| {
                    types::Lowering::new(
                        session.frontend().declarations(),
                        bindings,
                        &session.frontend().profile().target,
                    )
                });
                render(session, &analysis, original, &assertions, bindings, lowering)
            })
    };
    let status = match lowered {
        Ok(rust) => {
            EmissionStatus::Emitted { rust, const_capability: ConstCapability::RuntimeOnly }
        }
        Err(reason) => EmissionStatus::Skipped { reason },
    };
    MacroEmission { analysis, status }
}

/// Emit a set of macros, preserving available calls and propagating value mismatches.
///
/// A disagreement never changes bindgen's output. Reverse dependency traversal
/// also prevents callers from hiding a skipped macro by expanding its body.
/// Shared capability-generation failures are returned instead of being mistaken
/// for individual unsupported macros.
pub fn emit_batch_with_bindings(
    session: &AnalysisSession<'_>,
    names: &[impl AsRef<str>],
    bindings: &BindingCatalog,
) -> Result<Vec<MacroEmission>, String> {
    let mut capabilities = bindings.clone();
    let requests = demands::plan(session, names, bindings);
    let adapters = generated_adapters(session, &requests, bindings)?;
    register_adapters(&mut capabilities, &adapters);
    Ok(emit_prepared_batch(session, names, &capabilities))
}

/// Install verified capabilities and their rejection reasons into one rendering catalog.
fn register_adapters(capabilities: &mut BindingCatalog, adapters: &GeneratedAdapters) {
    capabilities.callback_capabilities.extend(adapters.callbacks.markers.clone());
    capabilities.callback_unavailable.extend(adapters.callbacks.unsupported.clone());
    capabilities.enum_unavailable.extend(adapters.enumerations.unsupported.clone());
    capabilities.function_addresses.extend(adapters.addresses.bindings.clone());
    capabilities.field_capabilities.extend(adapters.fields.markers.clone());
    capabilities.offset_capabilities.extend(adapters.fields.offsets.clone());
    capabilities.offset_unavailable.extend(adapters.fields.offset_unsupported.clone());
    capabilities.functions.extend(adapters.functions.bindings.clone());
    capabilities.function_unavailable.extend(adapters.functions.unsupported.clone());
}

/// Render candidates, close mismatch failures through the dependency graph, and rerender calls.
///
/// A second propagation step handles late rendering failures so no surviving macro
/// references an exported definition that failed its output bound.
fn emit_prepared_batch(
    session: &AnalysisSession<'_>,
    names: &[impl AsRef<str>],
    bindings: &BindingCatalog,
) -> Vec<MacroEmission> {
    // Each pass borrows one immutable capability catalog. Index its C/Rust
    // storage identities once, after the first macro passes its admission gates.
    let initial = {
        let mut lowering = None;
        names
            .iter()
            .map(|name| emit_with_lowering(session, name.as_ref(), bindings, &mut lowering))
            .collect::<Vec<_>>()
    };
    let mut roots = initial
        .iter()
        .filter_map(|emission| match &emission.status {
            EmissionStatus::Skipped { reason }
                if reason.code == SkipReasonCode::BindingValueMismatch =>
            {
                Some((emission.analysis.name.clone(), reason.clone()))
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let graph = session.dependencies();
    // Seed failures from the entire active environment, not only requested
    // functions: a retained object binding can hide a callee's enum identifier.
    for (name, binding) in &bindings.integer_constants {
        let constant = if session.frontend().environment().active.contains_key(name) {
            session.integer_constants().get(name)
        } else {
            session.frontend().declarations().integer_constants.get(name).filter(|constant| {
                let crate::TypeCategory::Integer(kind) = constant.ty.category else { return false };
                session
                    .frontend()
                    .profile()
                    .target
                    .integers
                    .get(&kind)
                    .is_some_and(|ty| ty.bits <= 64)
            })
        };
        let Some(constant) = constant else { continue };
        if integer_numeric(binding.value) == integer_numeric(constant.value) {
            continue;
        }
        for caller in graph.dependents(name).chain(graph.constant_users(name)) {
            roots.entry(caller.into()).or_insert_with(|| {
                mismatch_reason(&session.analyze(caller), name, binding.value, constant.value)
            });
        }
    }
    let impacts = graph.impacts(&roots.keys().collect::<Vec<_>>());
    let impacted =
        impacts.iter().map(|impact| (impact.name.as_str(), impact)).collect::<BTreeMap<_, _>>();
    let mut available = bindings.clone();
    available.macros.extend(
        initial
            .iter()
            .filter(|emission| {
                matches!(emission.status, EmissionStatus::Emitted { .. })
                    && !impacted.contains_key(emission.analysis.name.as_str())
            })
            .map(|emission| emission.analysis.name.clone()),
    );
    for name in roots.keys().map(String::as_str).chain(impacted.keys().copied()) {
        available.macros.remove(name);
    }
    let mut available_lowering = None;
    let mut emissions = initial
        .into_iter()
        .map(|emission| {
            let name = &emission.analysis.name;
            let failure = if let Some(impact) = impacted.get(name.as_str()) {
                Some((impact.dependency.as_str(), impact.root.as_str()))
            } else if roots.contains_key(name) {
                // A wrapper may independently see the same folded constant.
                // Prefer its direct dependency as the explanation where possible.
                graph.dependencies(name).find_map(|dependency| {
                    if dependency != name && roots.contains_key(dependency) {
                        Some((dependency, dependency))
                    } else {
                        impacted.get(dependency).and_then(|impact| {
                            (impact.root != *name).then_some((dependency, impact.root.as_str()))
                        })
                    }
                })
            } else {
                None
            };
            if let Some((dependency, root)) = failure {
                let reason = dependency_skip(&emission.analysis, dependency, root, &roots[root]);
                MacroEmission {
                    analysis: emission.analysis,
                    status: EmissionStatus::Skipped { reason },
                }
            } else if roots.contains_key(name) {
                let reason = roots[name].clone();
                MacroEmission {
                    analysis: emission.analysis,
                    status: EmissionStatus::Skipped { reason },
                }
            } else {
                emit_with_lowering(session, name, &available, &mut available_lowering)
            }
        })
        .collect::<Vec<_>>();
    // If a final rendering hits an output limit, reject its dependents too rather
    // than leaving a preserved call to a macro whose definition is absent.
    let late = emissions
        .iter()
        .filter_map(|emission| match &emission.status {
            EmissionStatus::Skipped { reason }
                if available.macros.contains(&emission.analysis.name) =>
            {
                Some((emission.analysis.name.clone(), reason.clone()))
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let impacts = graph.impacts(&late.keys().collect::<Vec<_>>());
    let impacted =
        impacts.iter().map(|impact| (impact.name.as_str(), impact)).collect::<BTreeMap<_, _>>();
    for emission in &mut emissions {
        if let Some(impact) = impacted.get(emission.analysis.name.as_str()) {
            emission.status = EmissionStatus::Skipped {
                reason: dependency_skip(
                    &emission.analysis,
                    &impact.dependency,
                    &impact.root,
                    &late[&impact.root],
                ),
            };
        }
    }
    emissions
}

/// Generate Rust support when the selected macros need no native C artifact.
///
/// Include this source once in the defining crate, before or after its macro definitions.
/// The adapters derive from original Clang declarations and actual bindgen storage.
/// Returns an error when native support is required; use [`generate_with_bindings`]
/// to obtain and compile that support together with the macro set.
pub fn emit_support_with_bindings(
    session: &AnalysisSession<'_>,
    names: &[impl AsRef<str>],
    bindings: &BindingCatalog,
) -> Result<String, String> {
    let artifact = emit_support_artifact_with_bindings(session, names, bindings)?;
    if !artifact.c_source.is_empty() {
        return Err("these field capabilities require compiling the C source from emit_support_artifact_with_bindings".into());
    }
    Ok(artifact.rust)
}

/// Generated storage capabilities, C access primitives and native ABI adapters.
///
/// Include the Rust source once in the defining crate. Nonempty C source must include
/// the inspected header, be compiled under the session's profile, and be linked into
/// that crate. Native primitives preserve objects such as partly initialized bitfields
/// and call original C functions; they do not replace complete macro expressions.
#[derive(Clone, Debug, Default)]
pub struct MacroSupportArtifact {
    /// Support items included once alongside the generated macro set.
    pub rust: String,
    /// Same-profile C primitives that the caller must compile when nonempty.
    pub c_source: String,
}

/// One preparation supplies both the macros and their shared native capabilities.
#[derive(Clone, Debug)]
pub struct MacroGeneration {
    /// Per-root output and skips produced with the finalized capability catalog.
    pub macros: Vec<MacroEmission>,
    /// Only the shared Rust and native C support retained by successful roots.
    pub support: MacroSupportArtifact,
}

/// Prepare the selected macros and their shared capabilities from one trusted session.
///
/// The catalog must describe the defining crate's actual generated bindings. Set its
/// [`BindingCatalog::ffi_boundary`] when native adapters can call PostgreSQL functions;
/// those adapters must retain the crate's nonlocal-error and callback guard.
/// Include [`MacroGeneration::support`] once, compile its native C artifact under the
/// same inspected profile, and include each successfully emitted macro in that crate.
/// Unsupported macros retain their structured skip reasons.
///
/// ```no_run
/// use pgrx_c_macros::{AnalysisSession, BindingCatalog, EmissionStatus, generate_with_bindings};
///
/// fn sources(session: &AnalysisSession<'_>, bindings: &BindingCatalog)
///     -> Result<(String, String), String>
/// {
///     let generated = generate_with_bindings(session, &["TYPEALIGN", "BUFFERALIGN"], bindings)?;
///     let mut rust = generated.support.rust;
///     for emission in generated.macros {
///         if let EmissionStatus::Emitted { rust: definition, .. } = emission.status {
///             rust.push_str(&definition);
///         }
///     }
///     Ok((rust, generated.support.c_source))
/// }
/// ```
pub fn generate_with_bindings(
    session: &AnalysisSession<'_>,
    names: &[impl AsRef<str>],
    bindings: &BindingCatalog,
) -> Result<MacroGeneration, String> {
    let requests = demands::plan(session, names, bindings);
    let mut adapters = generated_adapters(session, &requests, bindings)?;
    let mut capabilities = bindings.clone();
    register_adapters(&mut capabilities, &adapters);
    let macros = emit_prepared_batch(session, names, &capabilities);
    let admitted = macros
        .iter()
        .filter(|emission| matches!(emission.status, EmissionStatus::Emitted { .. }))
        .map(|emission| emission.analysis.name.as_str())
        .collect::<Vec<_>>();
    let final_requests = demands::plan(session, &admitted, bindings);
    if final_requests != requests {
        // Rejected roots cannot retain support that no emitted macro can use.
        // Marker identities depend on C/ABI facts, not the requested root set.
        adapters = generated_adapters(session, &final_requests, bindings)?;
    }
    let support = render_adapters(adapters, &macros)?;
    Ok(MacroGeneration { macros, support })
}

/// Generate the complete batch and return its mutually consistent support artifacts.
pub fn emit_support_artifact_with_bindings(
    session: &AnalysisSession<'_>,
    names: &[impl AsRef<str>],
    bindings: &BindingCatalog,
) -> Result<MacroSupportArtifact, String> {
    Ok(generate_with_bindings(session, names, bindings)?.support)
}

/// Assemble native items and shared token classifiers needed by the successful macro set.
fn render_adapters(
    adapters: GeneratedAdapters,
    macros: &[MacroEmission],
) -> Result<MacroSupportArtifact, String> {
    let body = format!(
        "{}\n{}\n{}\n{}\n{}",
        adapters.callbacks.rust,
        adapters.fields.rust,
        adapters.functions.rust,
        adapters.addresses.rust,
        adapters.enumerations.rust
    );
    let mut rust = if body.trim().is_empty() {
        String::new()
    } else {
        format!(
            "#[doc(hidden)]\n#[allow(non_snake_case, non_camel_case_types)]\npub mod __pgrx_c_generated {{\nuse crate::__pgrx_c_macros as c;\n{body}\n}}\n"
        )
    };
    let exports = macros
        .iter()
        .filter(|emission| matches!(emission.status, EmissionStatus::Emitted { .. }))
        .filter_map(|emission| macro_identifier(&emission.analysis.name))
        .collect::<BTreeSet<_>>();
    if macros.iter().any(|emission| {
        matches!(emission.status, EmissionStatus::Emitted { .. })
            && emission
                .analysis
                .parameters
                .iter()
                .any(|parameter| parameter.roles.contains(&crate::ParameterRole::Value))
    }) {
        rust.push_str(&crate::format_rust_macros(&arguments::shared(&exports)?)?);
    }
    if !adapters.fields.markers.is_empty() {
        rust.push_str(&crate::format_rust_macros(&field_registry(&adapters.fields.markers)?)?);
    }
    Ok(MacroSupportArtifact {
        rust,
        c_source: format!(
            "{}\n{}\n{}",
            adapters.fields.c_source, adapters.functions.c_source, adapters.addresses.c_source
        )
        .trim()
        .into(),
    })
}

/// Reconciled capability families before registration, pruning, and final artifact assembly.
struct GeneratedAdapters {
    /// Member-access and offset capabilities, including same-profile bitfield primitives.
    fields: fields::FieldAdapters,
    /// Callable declaration adapters and any replacement binding signatures.
    functions: functions::FunctionAdapters,
    /// Nominal function-pointer identities and admitted call operations.
    callbacks: callbacks::CallbackAdapters,
    /// Original C address getters for referenced functions.
    addresses: addresses::AddressAdapters,
    /// Enum identities and checked storage bridges for requested types.
    enumerations: enumerations::EnumAdapters,
}

/// Build capability families in dependency order from the planned batch requirements.
///
/// Callback witnesses establish signatures before function thunks are generated;
/// those thunks then contribute storage facts to the final callback and field passes.
fn generated_adapters(
    session: &AnalysisSession<'_>,
    requests: &demands::Requests,
    bindings: &BindingCatalog,
) -> Result<GeneratedAdapters, String> {
    let frontend = session.frontend();
    /// Fingerprint compiler identity and profile facts for deterministic, collision-resistant native symbols.
    use sha2::{Digest, Sha256};
    let profile = serde_json::to_vec(frontend.profile())
        .map_err(|error| format!("cannot fingerprint the C function profile: {error}"))?;
    let profile_identity =
        Sha256::digest(profile).iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let callbacks = callbacks::generate(
        frontend.declarations(),
        bindings,
        &frontend.profile().target,
        &requests.functions,
        &callbacks::CallbackRequests::default(),
    )?;
    let mut capabilities = bindings.clone();
    capabilities.callback_capabilities.extend(callbacks.markers.clone());
    capabilities.callback_unavailable.extend(callbacks.unsupported.clone());
    let functions = functions::generate(
        frontend.declarations(),
        &capabilities,
        &requests.functions,
        &requests.called_functions,
        &frontend.profile().target,
        &profile_identity,
    )?;
    capabilities.functions.extend(functions.bindings.clone());
    let callbacks = callbacks::generate(
        frontend.declarations(),
        &capabilities,
        &frontend.profile().target,
        &requests.functions,
        &requests.callbacks,
    )?;
    capabilities.callback_capabilities.extend(callbacks.markers.clone());
    capabilities.callback_unavailable.extend(callbacks.unsupported.clone());
    let mut required_types = requests.types.values().cloned().collect::<Vec<_>>();
    required_types.extend(callbacks.required_types.iter().cloned());
    let mut enum_requests = enumerations::EnumRequests {
        types: requests.enums.types.clone(),
        open_scalar: requests.enums.open_scalar,
    };
    let lowering =
        types::Lowering::new(frontend.declarations(), &capabilities, &frontend.profile().target);
    demands::type_dependencies(
        frontend.declarations(),
        &lowering,
        &required_types,
        &mut enum_requests.types,
        &mut BTreeSet::new(),
    );
    Ok(GeneratedAdapters {
        enumerations: enumerations::generate(
            frontend.declarations(),
            &capabilities,
            &frontend.profile().target,
            &enum_requests,
        )?,
        fields: fields::generate(
            frontend.declarations(),
            &capabilities,
            &requests.fields,
            &frontend.profile().target,
            &required_types,
        )?,
        addresses: addresses::generate(
            frontend.declarations(),
            &capabilities,
            &requests.addresses,
            &frontend.profile().target,
            &profile_identity,
        )?,
        functions,
        callbacks,
    })
}

/// Render a bounded identifier-to-capability lookup for fields and offsetof paths.
fn field_registry(markers: &BTreeMap<String, String>) -> Result<String, String> {
    let mut rust =
        String::from("#[doc(hidden)]\n#[macro_export]\nmacro_rules! __pgrx_c_field_marker {\n");
    rust.push_str(&format!(
        "(@path; $($path:tt)+) => {{ $crate::__pgrx_c_field_marker!(@walk [{}]; $($path)+) }};\n\
         (@walk [@ $($budget:tt)*]; ($($inner:tt)+) . $($rest:tt)+) => {{ $crate::__pgrx_c_field_marker!(@walk [@ $($budget)*]; $($inner)+ . $($rest)+) }};\n\
         (@walk [@ $($budget:tt)*]; ($($inner:tt)+)) => {{ $crate::__pgrx_c_field_marker!(@walk [@ $($budget)*]; $($inner)+) }};\n\
         (@walk [@ $($budget:tt)*]; $field:ident . $($rest:tt)+) => {{ $crate::__pgrx_c_macros::expression::OffsetStep<$crate::__pgrx_c_field_marker!($field), $crate::__pgrx_c_field_marker!(@walk [$($budget)*]; $($rest)+)> }};\n\
         (@walk [@ $($budget:tt)*]; $field:ident) => {{ $crate::__pgrx_c_macros::expression::OffsetStep<$crate::__pgrx_c_field_marker!($field), $crate::__pgrx_c_macros::expression::OffsetEnd> }};\n\
         (@walk []; $($rest:tt)+) => {{ compile_error!(\"offsetof member path exceeds the 64-component bound\") }};\n",
        "@ ".repeat(64)
    ));
    for (field, marker) in markers {
        let Some(identifier) = field_identifier(field) else { continue };
        writeln!(rust, "({identifier}) => {{ {marker} }};").expect("String output");
        if let Some(raw) = rust_identifier(field).filter(|identifier| identifier.starts_with("r#"))
        {
            writeln!(rust, "({raw}) => {{ {marker} }};").expect("String output");
        }
        if rust.len() > MAX_EMISSION_BYTES {
            return Err("generated C field registry exceeds its source budget".into());
        }
    }
    rust.push_str("($($unknown:tt)*) => { compile_error!(\"field has no compiler-verified PostgreSQL binding capability\") };\n}\n");
    Ok(rust)
}

// Field tokens select a verified capability rather than declare a Rust item.
// C members may therefore use Rust keywords such as `self` or `crate`.
/// Accept C member tokens without applying Rust item-name keyword restrictions.
///
/// These tokens select verified capabilities rather than declare Rust identifiers.
pub(super) fn field_identifier(name: &str) -> Option<&str> {
    let mut bytes = name.bytes();
    (bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_'))
    .then_some(name)
}

/// Retain the failing dependency and root cause in a dependent macro’s skip diagnostic.
fn dependency_skip(
    analysis: &MacroAnalysis,
    dependency: &str,
    root: &str,
    reason: &SkipReason,
) -> SkipReason {
    skip(
        analysis,
        SkipReasonCode::DependencySkipped,
        format!(
            "depends on skipped macro `{dependency}` (root `{root}`): {}; skipping macro `{}`",
            reason.message, analysis.name
        ),
        None,
    )
}

/// Compare signed and unsigned binding facts in a lossless common representation.
fn integer_numeric(value: IntegerValue) -> i128 {
    match value {
        IntegerValue::Signed(value) => i128::from(value),
        IntegerValue::Unsigned(value) => i128::from(value),
    }
}

/// Explain a bindgen/Clang value disagreement without correcting either source of facts.
fn mismatch_reason(
    analysis: &MacroAnalysis,
    name: &str,
    bindgen: IntegerValue,
    clang: IntegerValue,
) -> SkipReason {
    skip(
        analysis,
        SkipReasonCode::BindingValueMismatch,
        format!(
            "bindgen and clang do not agree on {name}'s value; bindgen={}, clang={}; skipping macro `{}`",
            integer_numeric(bindgen),
            integer_numeric(clang),
            analysis.name
        ),
        None,
    )
}

/// Reject differing binding values even when object-macro expansion already folded the name.
fn binding_mismatch(
    session: &AnalysisSession<'_>,
    analysis: &MacroAnalysis,
    bindings: &BindingCatalog,
) -> Option<SkipReason> {
    let mut constants = BTreeMap::new();
    if let Some(expression) = &analysis.expression {
        constants.extend(
            expression.constants.iter().map(|constant| (constant.name.as_str(), constant.value)),
        );
    }
    // Non-atomic object expansions may already be literals in the analyzed tree.
    // Check their trusted standalone C facts as well as retained identifiers.
    for dependency in &analysis.dependencies {
        if dependency.kind == crate::MacroKind::ObjectLike
            && let Some(constant) = session.integer_constants().get(&dependency.name)
        {
            constants.insert(dependency.name.as_str(), constant.value);
        }
    }
    constants.into_iter().find_map(|(name, value)| {
        let binding = bindings.integer_constants.get(name)?;
        (integer_numeric(binding.value) != integer_numeric(value))
            .then(|| mismatch_reason(analysis, name, binding.value, value))
    })
}

/// Attach token and provenance information to a rendering rejection for audit output.
fn skip(
    analysis: &MacroAnalysis,
    code: SkipReasonCode,
    message: impl Into<String>,
    tokens: Option<TokenRange>,
) -> SkipReason {
    SkipReason {
        code,
        message: message.into(),
        tokens,
        spans: analysis.provenance.iter().cloned().collect(),
    }
}

/// Assemble the public macro, contextual entry arms, profile guard, and original C docs.
fn render(
    session: &AnalysisSession<'_>,
    analysis: &MacroAnalysis,
    original: &MacroDefinition,
    assertions: &str,
    bindings: &BindingCatalog,
    lowering: &types::Lowering<'_>,
) -> Result<String, SkipReason> {
    let identifier = macro_identifier(&analysis.name).ok_or_else(|| {
        skip(
            analysis,
            SkipReasonCode::UnsupportedType,
            "the C macro name cannot be represented by a Rust macro identifier",
            None,
        )
    })?;
    let arguments = arguments::generate(analysis)?;
    let expression = analysis.expression.as_ref().ok_or_else(|| {
        skip(
            analysis,
            SkipReasonCode::InvalidExpression,
            "candidate has no complete expression",
            None,
        )
    })?;
    let empty =
        matches!(expression.syntax.nodes[expression.syntax.root].kind, ExpressionKind::Empty);
    let mut constants = vec![None; expression.syntax.nodes.len()];
    for constant in &expression.constants {
        constants[constant.node] = Some(constant);
    }
    let renderer =
        typed::Renderer::new(session.frontend(), analysis, bindings, &constants, lowering);
    let statement_body = expression.syntax.statement_body.as_ref();
    let returning = statement_body.is_some_and(|body| body.return_tokens.is_some());
    let statement_boundary = statement_body.is_some_and(|body| body.requires_boundary);
    let normalizer = arguments.as_ref().map(|arguments| arguments.normalizer.as_str());
    let local_guard =
        if statement_body.is_some() { statements::guard(analysis, normalizer)? } else { None };
    let invoke = |mode: &str, prefix: &str| match normalizer {
        None => format!("$crate::{identifier}!(@{mode}; {prefix}$($raw)*)"),
        Some(_) if local_guard.is_some() => {
            format!("$crate::{identifier}!(@__pgrx_c_guard_locals {mode} [{prefix}]; $($raw)*)")
        }
        Some(normalizer) => {
            format!("$crate::{normalizer}!(@collect {mode} [{prefix}]; $($raw)*)")
        }
    };
    let value = if statement_body.is_some() {
        statements::render(session, analysis, bindings, &renderer, None)?
    } else if empty {
        String::new()
    } else {
        render_body(session, analysis, bindings, &renderer, typed::Context::Value)?
    };
    let mut rust = String::from(assertions);
    if let Some(arguments) = &arguments {
        rust.push_str(&arguments.rust);
    }
    let mut comment = format!("C macro {}", analysis.name);
    if let Some(span) = &analysis.provenance
        && let Some(file) = span.file.file_name()
    {
        write!(&mut comment, " from {}:{}", file.to_string_lossy(), span.start_line)
            .expect("String output");
    }
    comment.push_str(&definition_comment(original).ok_or_else(|| {
        skip(
            analysis,
            SkipReasonCode::BudgetExceeded,
            "original C macro source comment exceeds the bounded output size",
            None,
        )
    })?);
    if returning {
        comment.push_str("\n\nC return statements in this macro exit the enclosing Rust function or closure. Call it directly, without an outer `return`. The enclosing result must have an unambiguous C identity; otherwise use `@__pgrx_c_return_as [CMarker];` before the arguments to specify the original C function's return type. Return conversion uses C assignment rules, including truncation and pointer qualification. Rust caller cleanup follows normal Rust return behavior. Pointer access and native calls keep their usual caller safety obligations.");
    } else if statement_body.is_some() {
        comment.push_str("\n\nThis macro executes C statements in order and yields no value. Local blocks retain their C scope. Pointer access and native calls keep their usual caller safety obligations.");
    }
    if local_guard.is_some() {
        comment.push_str(" Caller argument tokens must not mention the macro's C local names, even inside groups: those invocations are rejected because C substitution can capture locals that Rust hygiene would resolve differently. The scope check also inspects forwarded expression fragments and is bounded to 4096 stringified bytes. Matches in fields, paths or strings are conservatively rejected.");
    }
    let explicit_boundary =
        analysis.invocation == crate::InvocationContract::ExplicitExpressionBoundary;
    let boundary_error = if statement_boundary {
        Some(
            "this C replacement requires an explicit braced invocation: use @__pgrx_c_statement; before its arguments",
        )
    } else if explicit_boundary {
        Some(
            "this C replacement requires an explicit parenthesized invocation: use @__pgrx_c_expression; before its arguments",
        )
    } else {
        None
    };
    if explicit_boundary {
        write!(comment, "\n\nCall as `{identifier}!(@__pgrx_c_expression; arguments...)`. This explicitly requests the semantics of the parenthesized C invocation `({}(arguments...))`. The original unparenthesized replacement can interact with surrounding C operators; that textual interaction is outside this Rust invocation contract.", analysis.name).expect("String output");
    }
    if statement_boundary {
        write!(comment, "\n\nCall as `{identifier}!(@__pgrx_c_statement; arguments...)`. This requests the semantics of a braced C invocation `{{ {}(arguments...); }}`. The original unbraced conditional can capture a surrounding C `else`, so the boundary is required. When specifying a return type, put `@__pgrx_c_return_as [CMarker];` after the statement boundary.", analysis.name).expect("String output");
    }
    let captures = analysis
        .parameters
        .iter()
        .filter(|parameter| parameter.origin == crate::ParameterOrigin::FreeIdentifier)
        .collect::<Vec<_>>();
    if !captures.is_empty() {
        write!(comment, "\n\nRust callers supply {} arguments: the {} original C parameters, followed by explicit caller-scope operands in this order: ", analysis.parameters.len(), analysis.parameters.len() - captures.len()).expect("String output");
        for (index, parameter) in captures.iter().enumerate() {
            if index != 0 {
                comment.push_str(", ");
            }
            write!(comment, "`{}`", parameter.name).expect("String output");
        }
        comment.push_str(". Each operand must preserve its C type and place requirements.");
    }
    write_doc_comments(&mut rust, &comment).ok_or_else(|| {
        skip(
            analysis,
            SkipReasonCode::BudgetExceeded,
            "original C macro source comment exceeds the bounded output size",
            None,
        )
    })?;
    write!(&mut rust, "#[macro_export]\nmacro_rules! {identifier} {{\n").expect("String output");
    let matcher = arguments::matcher(analysis);
    let operands = (0..analysis.parameters.len())
        .map(|index| format!("${}", arguments::name(analysis, index)))
        .collect::<Vec<_>>()
        .join(", ");
    let public_body = if statement_body.is_some() {
        value.clone()
    } else if empty {
        String::new()
    } else {
        format!(
            "{SUPPORT}::expression_result::finish($crate::{identifier}!(@__pgrx_emit_value; {operands}))"
        )
    };
    writeln!(&mut rust, "(@__pgrx_emit_public; {matcher}) => {{ {public_body} }};")
        .expect("String output");
    if statement_body.is_some() {
        let boundary_prefix = if statement_boundary { "@__pgrx_c_statement; " } else { "" };
        if returning {
            let return_marker = arguments::return_marker(analysis);
            let explicit =
                statements::render(session, analysis, bindings, &renderer, Some(&return_marker))?;
            writeln!(
                &mut rust,
                "(@__pgrx_emit_return_as; ${return_marker}:ty, {matcher}) => {{ {explicit} }};"
            )
            .expect("String output");
            let call = invoke("__pgrx_emit_return_as", &format!("${return_marker},"));
            writeln!(&mut rust, "({boundary_prefix}@__pgrx_c_return_as [${return_marker}:ty]; $($raw:tt)*) => {{ {call} }};").expect("String output");
        } else {
            let call = invoke("__pgrx_emit_discard", "");
            writeln!(&mut rust, "(@__pgrx_emit_discard; {matcher}) => {{ $crate::{identifier}!(@__pgrx_emit_public; {operands}) }};\n(@__pgrx_c_discard; {boundary_prefix}$($raw:tt)*) => {{ {call} }};").expect("String output");
        }
        if let Some(guard) = &local_guard {
            rust.push_str(guard);
        }
        if statement_boundary {
            let call = invoke("__pgrx_emit_public", "");
            writeln!(&mut rust, "(@__pgrx_c_statement; $($raw:tt)*) => {{ {call} }};")
                .expect("String output");
        }
        if arguments.is_none() {
            for mode in
                if returning { &["public", "return_as"][..] } else { &["public", "discard"][..] }
            {
                writeln!(&mut rust, "(@__pgrx_emit_{mode}; $($invalid:tt)*) => {{ compile_error!(\"arguments do not satisfy this C macro's invocation contract\") }};").expect("String output");
            }
        }
        if !returning && let Some(message) = boundary_error {
            writeln!(
                &mut rust,
                "(@__pgrx_c_discard; $($raw:tt)*) => {{ compile_error!({message:?}) }};"
            )
            .expect("String output");
        }
        writeln!(&mut rust, "(@$mode:ident; $($raw:tt)*) => {{ compile_error!(\"a C statement body is not an expression operand\") }};").expect("String output");
    } else {
        for (mode, context) in [
            ("value", typed::Context::Value),
            ("place", typed::Context::Place),
            ("read_place", typed::Context::ReadPlace),
            ("size", typed::Context::Size),
            ("discard", typed::Context::Discard),
        ] {
            let body = if empty {
                "compile_error!(\"an empty C macro has no expression operand\")".into()
            } else if mode == "value" {
                value.clone()
            } else {
                match render_body(session, analysis, bindings, &renderer, context) {
                    Ok(body) => body,
                    Err(reason) => format!("compile_error!({:?})", reason.message),
                }
            };
            writeln!(&mut rust, "(@__pgrx_emit_{mode}; {matcher}) => {{ {body} }};")
                .expect("String output");
            let boundary_prefix = if explicit_boundary { "@__pgrx_c_expression; " } else { "" };
            let call = invoke(&format!("__pgrx_emit_{mode}"), "");
            writeln!(
                &mut rust,
                "(@__pgrx_c_{mode}; {boundary_prefix}$($raw:tt)*) => {{ {call} }};"
            )
            .expect("String output");
            if arguments.is_none()
                && let Some(message) = boundary_error
            {
                writeln!(
                    &mut rust,
                    "(@__pgrx_c_{mode}; $($raw:tt)*) => {{ compile_error!({message:?}) }};"
                )
                .expect("String output");
            }
        }
        if explicit_boundary {
            let call = invoke("__pgrx_emit_public", "");
            writeln!(&mut rust, "(@__pgrx_c_expression; $($raw:tt)*) => {{ {call} }};")
                .expect("String output");
        }
        if arguments.is_none() {
            writeln!(&mut rust, "(@$mode:ident; $($invalid:tt)*) => {{ compile_error!(\"arguments do not satisfy this C macro's invocation contract\") }};").expect("String output");
        }
    }
    if let Some(message) = boundary_error {
        writeln!(&mut rust, "($($raw:tt)*) => {{ compile_error!({message:?}) }};\n}}")
            .expect("String output");
    } else if arguments.is_none() {
        writeln!(&mut rust, "() => {{ $crate::{identifier}!(@__pgrx_emit_public;) }};\n($($invalid:tt)*) => {{ compile_error!(\"arguments do not satisfy this C macro's invocation contract\") }};\n}}").expect("String output");
    } else {
        let call = invoke("__pgrx_emit_public", "");
        writeln!(&mut rust, "($($raw:tt)*) => {{ {call} }};\n}}").expect("String output");
    }
    if rust.len() > MAX_EMISSION_BYTES {
        let message = if statement_body.is_some() {
            "generated statement macro exceeds the bounded output size"
        } else {
            "generated macro exceeds the bounded output size"
        };
        return Err(skip(analysis, SkipReasonCode::BudgetExceeded, message, None));
    }
    let rust = crate::format_rust_macros(&rust)
        .map_err(|message| skip(analysis, SkipReasonCode::InvalidExpression, message, None))?;
    if rust.len() > MAX_EMISSION_BYTES {
        return Err(skip(
            analysis,
            SkipReasonCode::BudgetExceeded,
            "formatted macro exceeds the bounded output size",
            None,
        ));
    }
    Ok(rust)
}

/// Render a compiler-owned expression in the context its C caller requests.
fn render_body(
    session: &AnalysisSession<'_>,
    analysis: &MacroAnalysis,
    bindings: &BindingCatalog,
    renderer: &typed::Renderer<'_>,
    context: typed::Context,
) -> Result<String, SkipReason> {
    let expression = analysis.expression.as_ref().expect("candidate expression");
    let mut rust = String::new();
    if let Some(crate::ExpansionResult::Expanded { expansion }) =
        session.expansions().results.get(&analysis.name)
    {
        for fallback in &expansion.constant_fallbacks {
            write_fallback(&mut rust, &fallback.name, &fallback.reason);
        }
    }
    match crate::delegation::direct_delegation(session, analysis) {
        Ok(Some(delegation)) if bindings.macros.contains(&delegation.callee) => {
            if !renderer.preserves_expectations(|| session.analyze(&delegation.callee)) {
                write_fallback(
                    &mut rust,
                    &delegation.callee,
                    "preserving its call would lose a fixed expectation or retain an overridden nested branch hint",
                );
                renderer.render(expression.syntax.root, context, &mut rust)?;
                return Ok(rust);
            }
            if let Some(callee) = macro_identifier(&delegation.callee) {
                let integer_zero = matches!(context, typed::Context::Value)
                    && expression.integer_zero_constants.contains(&expression.syntax.root);
                if integer_zero {
                    write!(rust, "{SUPPORT}::expression::null_constant(").expect("String output");
                }
                let mode = match context {
                    typed::Context::Value => "value",
                    typed::Context::Place => "place",
                    typed::Context::ReadPlace => "read_place",
                    typed::Context::Size => "size",
                    typed::Context::Discard => "discard",
                };
                write!(rust, "$crate::{callee}!(@__pgrx_emit_{mode}; ").expect("String output");
                for root in delegation.arguments {
                    renderer.delegated_argument(root, &mut rust);
                }
                rust.push(')');
                if integer_zero {
                    rust.push(')');
                }
                return Ok(rust);
            }
            write_fallback(
                &mut rust,
                &delegation.callee,
                "its name cannot be represented as a Rust macro identifier",
            );
        }
        Ok(Some(delegation)) => write_fallback(
            &mut rust,
            &delegation.callee,
            "the callee is not in the set of emitted Rust macros",
        ),
        Err(reason) => write_fallback(&mut rust, &analysis.name, &reason),
        Ok(None) => {
            for dependency in &analysis.dependencies {
                if dependency.kind == crate::MacroKind::FunctionLike {
                    write_fallback(
                        &mut rust,
                        &dependency.name,
                        "preserving a call inside this expression has not been proved equivalent to C substitution",
                    );
                }
            }
        }
    }
    renderer.render(expression.syntax.root, context, &mut rust)?;
    Ok(rust)
}

/// Lower the integer expression arena with explicit work tasks and retained binding names.
fn render_expression(
    analysis: &MacroAnalysis,
    root: NodeId,
    bindings: &BindingCatalog,
    constants: &[Option<&ResolvedConstant>],
    rust: &mut String,
) -> Result<(), SkipReason> {
    let expression = analysis.expression.as_ref().ok_or_else(|| {
        skip(
            analysis,
            SkipReasonCode::InvalidExpression,
            "candidate has no complete expression",
            None,
        )
    })?;
    let policy = match analysis.signed_overflow {
        SignedOverflow::Undefined => "Undefined",
        SignedOverflow::Wrapping => "Wrapping",
        SignedOverflow::Trapping => {
            return Err(skip(
                analysis,
                SkipReasonCode::UnsupportedProfile,
                "trapping signed arithmetic has no established support policy",
                None,
            ));
        }
    };

    // The arena is a tree from the bounded parser. Render it once into a single
    // buffer, rather than copying each rendered subtree into all its ancestors.
    let mut tasks = vec![RenderTask::Node(root)];
    while let Some(task) = tasks.pop() {
        match task {
            RenderTask::Text(text) => rust.push_str(text),
            RenderTask::Node(index) => {
                let node = &expression.syntax.nodes[index];
                match &node.kind {
                    ExpressionKind::Empty => {}
                    ExpressionKind::Parameter { index } => {
                        write!(
                            rust,
                            "{SUPPORT}::expression::input(${})",
                            arguments::name(analysis, *index)
                        )
                        .expect("writing to a String cannot fail");
                    }
                    ExpressionKind::IntegerLiteral { literal } => {
                        let ty = concrete_type(expression, index, analysis)?;
                        if literal.value == 0 {
                            write!(rust, "{SUPPORT}::expression::null_constant(")
                                .expect("String output");
                        }
                        write_literal(rust, ty, literal);
                        if literal.value == 0 {
                            rust.push(')');
                        }
                    }
                    ExpressionKind::Identifier { .. } => {
                        let Some(constant) = constants[index] else {
                            return Err(skip(
                                analysis,
                                SkipReasonCode::InvalidExpression,
                                "identifier has no compiler-resolved integer value",
                                Some(node.tokens),
                            ));
                        };
                        let null_constant = integer_numeric(constant.value) == 0;
                        if null_constant {
                            write!(rust, "{SUPPORT}::expression::null_constant(")
                                .expect("String output");
                        }
                        match binding_path(bindings, &constant.name) {
                            Ok((path, representation)) => {
                                write!(
                                    rust,
                                    "{SUPPORT}::CValue::<{SUPPORT}::{}>::new(",
                                    marker(constant.ty.kind)
                                )
                                .expect("writing to a String cannot fail");
                                if constant.ty.kind == IntegerKind::Bool {
                                    rust.push('(');
                                }
                                write!(rust, "$crate::{path}")
                                    .expect("writing to a String cannot fail");
                                if matches!(representation, IntegerBindingRepresentation::Oid) {
                                    rust.push_str(".to_u32()");
                                }
                                if constant.ty.kind == IntegerKind::Bool {
                                    rust.push_str(" as u8 != 0)");
                                } else {
                                    write!(
                                        rust,
                                        " as {}{}",
                                        if constant.ty.signed { 'i' } else { 'u' },
                                        constant.ty.bits
                                    )
                                    .expect("writing to a String cannot fail");
                                }
                                rust.push(')');
                            }
                            Err(reason) => {
                                write_fallback(rust, &constant.name, reason);
                                let value = match constant.value {
                                    IntegerValue::Signed(value) => value.to_string(),
                                    IntegerValue::Unsigned(value) => value.to_string(),
                                };
                                if let Some(literal) = &constant.literal {
                                    write_literal(rust, constant.ty, literal);
                                } else {
                                    write_value(rust, constant.ty, &value);
                                }
                            }
                        }
                        if null_constant {
                            rust.push(')');
                        }
                    }
                    ExpressionKind::Group { operand } => {
                        rust.push('(');
                        tasks.push(RenderTask::Text(")"));
                        tasks.push(RenderTask::Node(*operand));
                    }
                    ExpressionKind::Cast { type_name, operand } if type_name == "void" => {
                        rust.push_str("{ let _ = ");
                        tasks.extend([RenderTask::Text("; }"), RenderTask::Node(*operand)]);
                    }
                    ExpressionKind::Comma { left, right } => {
                        rust.push_str("{ let _ = ");
                        tasks.extend([
                            RenderTask::Text(" }"),
                            RenderTask::Node(*right),
                            RenderTask::Text("; "),
                            RenderTask::Node(*left),
                        ]);
                    }
                    ExpressionKind::Cast { operand, .. } => {
                        let ty = concrete_type(expression, index, analysis)?;
                        write!(rust, "{SUPPORT}::cast::<{SUPPORT}::{}, _>(", marker(ty.kind))
                            .expect("writing to a String cannot fail");
                        tasks.push(RenderTask::Text(")"));
                        tasks.push(RenderTask::Node(*operand));
                    }
                    ExpressionKind::Unary { operator, operand } => {
                        let helper = match operator {
                            UnaryOperator::Plus => "promote",
                            UnaryOperator::Negate => "neg",
                            UnaryOperator::BitwiseNot => "bitnot",
                            UnaryOperator::LogicalNot => "logical_not",
                        };
                        write!(rust, "{SUPPORT}::{helper}")
                            .expect("writing to a String cannot fail");
                        if *operator == UnaryOperator::Negate {
                            write!(rust, "::<{SUPPORT}::{policy}, _>")
                                .expect("writing to a String cannot fail");
                        }
                        rust.push('(');
                        tasks.push(RenderTask::Text(")"));
                        tasks.push(RenderTask::Node(*operand));
                    }
                    ExpressionKind::Binary { operator, left, right } => match operator {
                        BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
                            write!(
                                rust,
                                "{SUPPORT}::CValue::<{SUPPORT}::CInt>::new(if {SUPPORT}::truth("
                            )
                            .expect("writing to a String cannot fail");
                            tasks.push(RenderTask::Text(")"));
                            tasks.push(RenderTask::Text(" { 1 } else { 0 }"));
                            tasks.push(RenderTask::Text(")"));
                            tasks.push(RenderTask::Node(*right));
                            tasks.push(RenderTask::Text(
                                if *operator == BinaryOperator::LogicalAnd {
                                    ") && $crate::__pgrx_c_macros::truth("
                                } else {
                                    ") || $crate::__pgrx_c_macros::truth("
                                },
                            ));
                            tasks.push(RenderTask::Node(*left));
                        }
                        _ => {
                            let helper = binary_helper(*operator);
                            write!(rust, "{SUPPORT}::{helper}")
                                .expect("writing to a String cannot fail");
                            if matches!(
                                operator,
                                BinaryOperator::Add
                                    | BinaryOperator::Subtract
                                    | BinaryOperator::Multiply
                                    | BinaryOperator::ShiftLeft
                            ) {
                                write!(rust, "::<{SUPPORT}::{policy}, _, _>")
                                    .expect("writing to a String cannot fail");
                            }
                            rust.push('(');
                            tasks.push(RenderTask::Text(")"));
                            tasks.push(RenderTask::Node(*right));
                            tasks.push(RenderTask::Text(", "));
                            tasks.push(RenderTask::Node(*left));
                        }
                    },
                    ExpressionKind::Conditional { condition, then_value, else_value } => {
                        if matches!(expression.types[index], TypeExpression::Void) {
                            write!(rust, "if {SUPPORT}::truth(")
                                .expect("writing to a String cannot fail");
                            tasks.extend([
                                RenderTask::Text(" }"),
                                RenderTask::Node(*else_value),
                                RenderTask::Text(" } else { "),
                                RenderTask::Node(*then_value),
                                RenderTask::Text(") { "),
                                RenderTask::Node(*condition),
                            ]);
                            continue;
                        }
                        write!(rust, "{SUPPORT}::select(if {SUPPORT}::truth(")
                            .expect("writing to a String cannot fail");
                        tasks.push(RenderTask::Text(")"));
                        tasks.push(RenderTask::Text(") }"));
                        tasks.push(RenderTask::Node(*else_value));
                        tasks.push(RenderTask::Text(
                            ") } else { $crate::__pgrx_c_macros::Either::Right(",
                        ));
                        tasks.push(RenderTask::Node(*then_value));
                        tasks.push(RenderTask::Text(") { $crate::__pgrx_c_macros::Either::Left("));
                        tasks.push(RenderTask::Node(*condition));
                    }
                    _ => {
                        return Err(skip(
                            analysis,
                            SkipReasonCode::UnsupportedType,
                            "typed expression lowering is not available for this construct",
                            Some(node.tokens),
                        ));
                    }
                }
            }
        }
        if rust.len() > MAX_EMISSION_BYTES {
            return Err(skip(
                analysis,
                SkipReasonCode::BudgetExceeded,
                "generated macro exceeds the bounded output size",
                None,
            ));
        }
    }
    Ok(())
}

/// Validate a crate-relative constant path and retain its binding representation bridge.
fn binding_path(
    bindings: &BindingCatalog,
    name: &str,
) -> Result<(String, IntegerBindingRepresentation), &'static str> {
    let binding = bindings
        .integer_constants
        .get(name)
        .ok_or("no integer constant binding is available in the defining Rust crate")?;
    if binding.path.is_empty() {
        return Err("the Rust binding has no usable path");
    }
    binding
        .path
        .iter()
        .map(|part| {
            rust_identifier(part).ok_or("the Rust binding path cannot be represented hygienically")
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|parts| (parts.join("::"), binding.representation))
}

/// Record why a C name remains expanded, sanitizing header text as inert Rust commentary.
fn write_fallback(rust: &mut String, name: &str, reason: &str) {
    // Diagnostic text can contain header tokens; prevent it from ending or nesting
    // the comment, or putting source directives on a new physical line.
    let clean =
        |text: &str| text.replace("/*", "/ *").replace("*/", "* /").replace(['\n', '\r'], " ");
    write!(
        rust,
        "/* PGRX: {} remains expanded because {}. */ ",
        clean(name),
        clean(reason.trim_end_matches('.'))
    )
    .expect("writing to a String cannot fail");
}

/// Prefix every physical source line and reserve the full comment budget before writing.
///
/// Header text can contain comments or line endings, so it must never escape into
/// active Rust items or leave a partially emitted documentation block.
fn write_doc_comments(rust: &mut String, comment: &str) -> Option<()> {
    // Prefix every physical line, including those inside retained C block
    // comments or filenames. Header text must never become Rust source.
    let bytes =
        comment.lines().flat_map(|line| line.split('\r')).fold(rust.len(), |bytes, line| {
            bytes.saturating_add(line.len()).saturating_add(if line.is_empty() { 4 } else { 5 })
        });
    if bytes > MAX_EMISSION_BYTES {
        return None;
    }
    for line in comment.lines().flat_map(|line| line.split('\r')) {
        rust.push_str("///");
        if !line.is_empty() {
            rust.push(' ');
            rust.push_str(line);
        }
        rust.push('\n');
    }
    Some(())
}

/// Reconstruct the original C definition with a fenced block and stable source provenance.
fn definition_comment(definition: &MacroDefinition) -> Option<String> {
    // Bound normalization before allocating: one retained comment can be much
    // larger than the token count suggests. Display adds at most one separator
    // per token, and omits the repeated name token and line comments.
    let bytes = definition
        .tokens
        .iter()
        .fold("#define ".len().saturating_add(definition.name.len()), |bytes, token| {
            bytes.saturating_add(token.spelling.len()).saturating_add(1)
        });
    if bytes > MAX_EMISSION_BYTES {
        return None;
    }
    let definition = definition.to_string();
    let fence_length = definition.split(|character| character != '`').map(str::len).max()?;
    let fence_length = fence_length.saturating_add(1).max(3);
    if definition.len().saturating_add(fence_length.saturating_mul(2)).saturating_add(9)
        > MAX_EMISSION_BYTES
    {
        return None;
    }
    let fence = "`".repeat(fence_length);
    Some(format!("\n\n{fence}text\n{definition}\n{fence}\n"))
}

/// Pending work for iterative integer-expression rendering without recursive source copies.
enum RenderTask {
    /// Render one expression-arena node after scheduling its surrounding delimiters.
    Node(
        /// Arena index identifying the next integer-expression subtree.
        NodeId,
    ),
    /// Write a fixed delimiter or helper fragment between scheduled child nodes.
    Text(
        /// Fixed source fragment placed between child-expression tasks.
        &'static str,
    ),
}

/// Map the compiler integer category to target width, signedness, and C rank facts.
fn concrete_type(
    expression: &AnalyzedExpression,
    index: NodeId,
    analysis: &MacroAnalysis,
) -> Result<IntegerType, SkipReason> {
    match &expression.types[index] {
        TypeExpression::Concrete { ty } => Ok(*ty),
        _ => Err(skip(
            analysis,
            SkipReasonCode::InvalidExpression,
            "literal, constant or cast has no concrete C integer type",
            Some(expression.syntax.nodes[index].tokens),
        )),
    }
}

/// Wrap a literal spelling with explicit Rust storage and the original C integer marker.
fn write_value(rust: &mut String, ty: IntegerType, magnitude: &str) {
    let value = if ty.kind == IntegerKind::Bool {
        if magnitude == "0" { "false" } else { "true" }
    } else {
        magnitude
    };
    write!(rust, "{SUPPORT}::CValue::<{SUPPORT}::{}>::new({value}", marker(ty.kind))
        .expect("writing to a String cannot fail");
    if ty.kind != IntegerKind::Bool {
        write!(rust, "{}{}", if ty.signed { 'i' } else { 'u' }, ty.bits)
            .expect("writing to a String cannot fail");
    }
    rust.push(')');
}

/// Preserve the header’s numeric radix while replacing C suffixes with Rust storage suffixes.
fn write_literal(rust: &mut String, ty: IntegerType, literal: &IntegerLiteral) {
    if literal.spelling.starts_with('\'') {
        write!(rust, "{SUPPORT}::CValue::<{SUPPORT}::{}>::new(", marker(ty.kind))
            .expect("writing to a String cannot fail");
        let body = &literal.spelling[1..literal.spelling.len() - 1];
        if !body.starts_with('\\')
            || matches!(body, r"\'" | r#"\""# | r"\\" | r"\n" | r"\r" | r"\t" | r"\0")
            || (body.starts_with(r"\x") && body.len() == 4)
        {
            rust.push_str(&literal.spelling);
        } else if body == r"\?" {
            rust.push_str("'?'");
        } else {
            // Analysis established an ASCII C int value. Rust has no C octal
            // character escapes and requires exactly two hexadecimal digits.
            write!(rust, "'\\x{:02x}'", literal.value).expect("writing to a String cannot fail");
        }
        write!(rust, " as {}{})", if ty.signed { 'i' } else { 'u' }, ty.bits)
            .expect("writing to a String cannot fail");
        return;
    }

    // The immutable session's parser already validated this ASCII spelling.
    // Remove the known suffix length rather than trimming digit-like letters.
    let suffix_bytes = usize::from(literal.suffix.unsigned) + usize::from(literal.suffix.long);
    let digits = &literal.spelling[..literal.spelling.len() - suffix_bytes];
    let magnitude = match literal.radix {
        16 => format!("0x{}", &digits[2..]),
        2 => format!("0b{}", &digits[2..]),
        8 if digits.len() > 1 => format!("0o{}", &digits[1..]),
        _ => digits.to_owned(),
    };
    write_value(rust, ty, &magnitude);
}

/// Name the semantic C integer marker, keeping rank distinct from equal-width Rust storage.
pub(crate) fn marker(kind: IntegerKind) -> &'static str {
    match kind {
        IntegerKind::Bool => "CBool",
        IntegerKind::Char => "CChar",
        IntegerKind::SignedChar => "CSignedChar",
        IntegerKind::UnsignedChar => "CUnsignedChar",
        IntegerKind::Short => "CShort",
        IntegerKind::UnsignedShort => "CUnsignedShort",
        IntegerKind::Int => "CInt",
        IntegerKind::UnsignedInt => "CUnsignedInt",
        IntegerKind::Long => "CLong",
        IntegerKind::UnsignedLong => "CUnsignedLong",
        IntegerKind::LongLong => "CLongLong",
        IntegerKind::UnsignedLongLong => "CUnsignedLongLong",
        IntegerKind::Int128 => "CInt128",
        IntegerKind::UnsignedInt128 => "CUnsignedInt128",
    }
}

/// Select the runtime operation implementing the analyzed C binary operator.
fn binary_helper(operator: BinaryOperator) -> &'static str {
    match operator {
        BinaryOperator::Multiply => "mul",
        BinaryOperator::Divide => "div",
        BinaryOperator::Remainder => "rem",
        BinaryOperator::Add => "add",
        BinaryOperator::Subtract => "sub",
        BinaryOperator::ShiftLeft => "shl",
        BinaryOperator::ShiftRight => "shr",
        BinaryOperator::Less => "lt",
        BinaryOperator::LessEqual => "le",
        BinaryOperator::Greater => "gt",
        BinaryOperator::GreaterEqual => "ge",
        BinaryOperator::Equal => "eq",
        BinaryOperator::NotEqual => "ne",
        BinaryOperator::BitAnd => "bitand",
        BinaryOperator::BitXor => "bitxor",
        BinaryOperator::BitOr => "bitor",
        BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
            unreachable!("logical operators have their own lazy lowering")
        }
    }
}

/// Render a usable Rust item name, rejecting special keywords that raw identifiers cannot fix.
fn rust_identifier(name: &str) -> Option<String> {
    let mut bytes = name.bytes();
    if !bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        || matches!(name, "_" | "self" | "Self" | "super" | "crate")
    {
        return None;
    }
    let keyword = matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "static"
            | "struct"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "gen"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    );
    Some(if keyword { format!("r#{name}") } else { name.into() })
}

/// Preserve C macro identity while accommodating Rust keywords where macro names allow it.
fn macro_identifier(name: &str) -> Option<String> {
    rust_identifier(name).or_else(|| {
        matches!(name, "_" | "self" | "Self" | "super" | "crate").then(|| {
            let mut encoded = String::from("__pgrx_c_macro_");
            for byte in name.bytes() {
                write!(encoded, "{byte:02x}").expect("String output");
            }
            encoded
        })
    })
}

/// Regression tests for output bounds, inert source documentation, hygienic names, and batch errors.
#[cfg(test)]
mod tests {
    /// Exercise the private rendering and budget helpers directly in regression tests.
    use super::{MAX_EMISSION_BYTES, definition_comment, rust_identifier, write_doc_comments};
    /// Construct independent compiler and binding fixtures for the lowering invariants exercised here.
    use crate::{MacroDefinition, MacroKind, Token, TokenKind};

    /// Check that batch generation propagates profile serialization errors instead of emitting partial support.
    #[cfg(unix)]
    #[test]
    fn batch_emission_reports_shared_profile_fingerprint_errors() {
        /// Construct a non-UTF-8 tracked-input path to exercise profile fingerprint error propagation.
        use std::os::unix::ffi::OsStringExt;
        let _lock = crate::SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = crate::MacroScanner::new().expect("libclang required");
        let header = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/emit_scalar.h");
        let mut frontend = crate::inspect(&scanner, &header, &[], None).unwrap();
        // A missing tracked input is a valid recorded state, but this spelling
        // cannot be serialized into the shared profile's JSON fingerprint.
        let input = std::path::PathBuf::from(std::ffi::OsString::from_vec(vec![0xff]));
        frontend.profile.inputs.fingerprints.insert(input, None);
        let session = crate::AnalysisSession::prepare(&scanner, &frontend, &["EMIT_ID"]).unwrap();
        let error = super::emit_batch_with_bindings(
            &session,
            &["EMIT_ID"],
            &crate::BindingCatalog::default(),
        )
        .unwrap_err();
        assert!(error.contains("cannot fingerprint the C function profile"), "{error}");
    }

    /// Check that large retained C definitions account for both source text and documentation fences.
    #[test]
    fn source_comment_budget_includes_comment_bytes_and_markdown_fences() {
        for comment in [
            format!("/* {} */", "x".repeat(MAX_EMISSION_BYTES)),
            format!("/* {} */", "`".repeat(MAX_EMISSION_BYTES / 2)),
        ] {
            let mut tokens = [
                (TokenKind::Identifier, "F"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Identifier, "x"),
                (TokenKind::Punctuation, ")"),
                (TokenKind::Identifier, "x"),
            ]
            .into_iter()
            .map(|(kind, spelling)| Token { kind, spelling: spelling.into() })
            .collect::<Vec<_>>();
            tokens.push(Token { kind: TokenKind::Comment, spelling: comment });
            let definition = MacroDefinition {
                name: "F".into(),
                kind: MacroKind::FunctionLike,
                location: None,
                provenance: None,
                tokens,
                builtin: false,
                main_file: true,
            };
            assert!(definition_comment(&definition).is_none());
        }
    }

    /// Prove embedded line endings and C comment text remain doc-comment content rather than Rust items.
    #[test]
    fn doc_comments_cover_every_physical_line_and_cannot_inject_rust_items() {
        let mut rust = String::new();
        write_doc_comments(
            &mut rust,
            "C macro F from file.h:1\r\npub fn injected() {}\rpub fn also_injected() {}\n\n/* C comment */",
        )
        .unwrap();
        assert!(rust.lines().all(|line| line.starts_with("///")));
        assert!(rust.contains("/// pub fn injected() {}\n"));
        assert!(rust.contains("/// pub fn also_injected() {}\n"));
        rust.push_str("pub const ORIGINAL: () = ();\n");
        let parsed = syn::parse_file(&rust).unwrap();
        assert_eq!(parsed.items.len(), 1, "header text cannot introduce Rust items");
        let syn::Item::Const(item) = &parsed.items[0] else {
            panic!("doc comments must attach to the original item");
        };
        assert_eq!(item.ident, "ORIGINAL");
        let documentation = item
            .attrs
            .iter()
            .filter_map(|attribute| match &attribute.meta {
                syn::Meta::NameValue(meta) if meta.path.is_ident("doc") => match &meta.value {
                    syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(value), .. }) => {
                        Some(value.value())
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(documentation.contains("pub fn injected() {}"));
        assert!(documentation.contains("pub fn also_injected() {}"));
    }

    /// Check that per-line prefixes count toward the budget and rejection leaves the output unchanged.
    #[test]
    fn source_comment_budget_includes_per_line_prefixes_without_writing_partial_output() {
        let mut rust = String::from("existing Rust source\n");
        let before = rust.clone();
        assert!(write_doc_comments(&mut rust, &"\n".repeat(MAX_EMISSION_BYTES / 4)).is_none());
        assert_eq!(rust, before);
    }

    /// Check raw keyword handling while keeping macro names separate from operand metavariable rules.
    #[test]
    fn macro_names_preserve_identity_without_using_c_parameters_as_rust_identifiers() {
        assert_eq!(rust_identifier("TYPEALIGN").as_deref(), Some("TYPEALIGN"));
        assert_eq!(rust_identifier("match").as_deref(), Some("r#match"));
        for rejected in ["", "_", "self", "Self", "super", "crate", "bad-name", "1name", "é"] {
            assert_eq!(rust_identifier(rejected), None);
        }
    }
}
