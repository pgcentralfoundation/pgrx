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

use crate::analysis::{
    AnalysisStatus, AnalyzedExpression, ConstCapability, MacroAnalysis, ResolvedConstant,
    SkipReason, SkipReasonCode, TypeExpression,
};
use crate::model::{IntegerKind, IntegerType, IntegerValue, SignedOverflow};
use crate::syntax::{
    BinaryOperator, ExpressionKind, IntegerLiteral, NodeId, TokenRange, UnaryOperator,
};
use crate::{
    AnalysisSession, InlineFunctionDefinition, MacroDefinition,
    support_generation::support_abi_assertions,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

/// Hygienic path to the defining crate’s C semantic runtime used by exported expansions.
const SUPPORT: &str = "$crate::__pgrx_c_macros";
/// Bound one macro’s emitted source, including diagnostics and original-definition docs.
const MAX_EMISSION_BYTES: usize = 1024 * 1024;

/// Original source for one invocation root, kept separate from its internal call syntax.
enum InvocationSource<'a> {
    /// An actual active preprocessor definition.
    Macro(&'a MacroDefinition),
    /// An actual compiler-proven function definition called through a typed adapter.
    Inline(&'a InlineFunctionDefinition),
}

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
/// Documentation shells: each macro's docs, invocation forms and result type.
mod documentation;
pub use documentation::{DocumentedResult, MacroDocumentation, documentation_shell_support};
/// Nominal enum identities and checked bridges to actual binding storage.
mod enumerations;
/// Optimization-hint planning that preserves C values and operand effects.
mod expectation;
/// Typed field projections, record registrations, and independent offset capabilities.
mod fields;
/// Native C thunks and guarded Rust adapters for callable declarations.
mod functions;
/// Generated support identifiers derived from C names.
mod names;
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
/// The catalog holds no generated state. Callback signatures, function adapters,
/// address getters, field or offset adapters, their refusals, and the inline functions
/// a batch translated exist only inside batch generation, so a caller cannot assert one.
#[derive(Clone, Debug, Default, Serialize)]
pub struct BindingCatalog {
    /// Crate-relative module containing generated declarations; C typedef names
    /// remain relative to this root while emitted Rust paths retain the full prefix.
    pub namespace: Vec<String>,
    /// Binding values and paths used to preserve symbols after checking agreement with Clang.
    pub integer_constants: BTreeMap<String, IntegerBinding>,
    /// Definitions available for preserved cross-macro calls in this defining crate.
    pub macros: BTreeSet<String>,
    /// Actual function binding prototypes. Calls use a generated native adapter
    /// instead when one exists, and never one of these for a prototype that needs one.
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
    /// Optional crate-relative guard for native adapters that may call PostgreSQL.
    #[serde(skip)]
    pub ffi_boundary: Option<Vec<String>>,
}

impl BindingCatalog {
    /// Resolve an original C typedef through its qualified Rust declaration without
    /// confusing a crate-root compatibility item with the generated alias.
    pub fn type_alias(&self, name: &str) -> Option<&crate::AliasBinding> {
        if self.namespace.is_empty() {
            return self.types.get(name);
        }
        let key = self
            .namespace
            .iter()
            .map(|part| part.trim_start_matches("r#"))
            .chain([name])
            .collect::<Vec<_>>()
            .join("::");
        self.types.get(&key)
    }

    /// Recover the declaration-relative C name from a qualified alias map key.
    /// A key outside this catalog's namespace cannot establish a C typedef identity.
    pub fn relative_type_name<'a>(&self, mut key: &'a str) -> Option<&'a str> {
        for part in &self.namespace {
            key = key.strip_prefix(part.trim_start_matches("r#"))?.strip_prefix("::")?;
        }
        Some(key)
    }

    /// Remove only the exact generated root from a binding path; explicit crate-owned
    /// and standard-library types keep their canonical paths instead.
    pub fn relative_path<'a>(&self, path: &'a [String]) -> Option<&'a [String]> {
        path.strip_prefix(self.namespace.as_slice())
    }

    /// Match a proved integer bridge using either its explicit crate-owned path or
    /// the same type reexported through the generated namespace.
    pub fn integer_kind(&self, path: &[String]) -> Option<IntegerKind> {
        let key = |parts: &[String]| {
            parts.iter().map(|part| part.trim_start_matches("r#")).collect::<Vec<_>>().join("::")
        };
        self.integer_storage.get(&key(path)).copied().or_else(|| {
            self.relative_path(path).and_then(|path| self.integer_storage.get(&key(path)).copied())
        })
    }
}

/// A caller's catalog cannot carry a capability or inline translation that only batch
/// generation establishes.
///
/// ```compile_fail
/// pgrx_c_macros::BindingCatalog::default().callback_capabilities;
/// ```
///
/// ```compile_fail
/// pgrx_c_macros::BindingCatalog::default().function_addresses;
/// ```
///
/// ```compile_fail
/// pgrx_c_macros::BindingCatalog::default().field_capabilities;
/// ```
///
/// ```compile_fail
/// pgrx_c_macros::BindingCatalog::default().offset_capabilities;
/// ```
///
/// ```compile_fail
/// pgrx_c_macros::BindingCatalog::default().inline_translations;
/// ```
#[cfg(doctest)]
pub struct GeneratedCapabilitiesAreNotCatalogInput;

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
    /// Why a static inline function's translated definition was not emitted, when the
    /// emitted macro calls the original function instead. It has no token range: that
    /// would index the translated body, while `analysis` is the call's.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<SkipReason>,
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
        /// Rustdoc-facing parts used to publish a documentation snapshot.
        #[serde(skip)]
        documentation: Box<MacroDocumentation>,
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
/// keep the helper bindings valid when that crate is renamed. Like
/// [`emit_with_bindings`], this generates no native adapters.
pub fn emit(session: &AnalysisSession<'_>, name: &str) -> MacroEmission {
    emit_with_bindings(session, name, &BindingCatalog::default())
}

/// Emit with verified references to constants and other generated macros.
///
/// This returns only the macro's Rust source and generates no adapters or other
/// capabilities. A macro that needs a callback signature, an original function
/// address, or a field or offset adapter is therefore skipped.
/// A direct call to a function whose prototype passes or returns plain `char`, an
/// enum or a record by value needs a generated C adapter, because Rust binding
/// storage cannot prove that call ABI. Such a macro is skipped with
/// [`SkipReasonCode::Call`] even when `bindings` holds the function, as is a call to
/// a function without a binding. [`generate_with_bindings`] generates these
/// capabilities when the declarations admit them, and otherwise skips the macro too.
///
/// A call to a static inline function never expands a translated body here. Only
/// batch generation knows which inline functions it translated, so this output calls
/// the function as described above.
///
/// The source can still name a C enum's generated identity, which only the support
/// artifact from [`generate_with_bindings`] defines. The defining crate must
/// include that item, or using the macro fails to compile.
pub fn emit_with_bindings(
    session: &AnalysisSession<'_>,
    name: &str,
    bindings: &BindingCatalog,
) -> MacroEmission {
    emit_with_lowering(session, name, bindings, Generated::NONE, &mut None)
}

/// Apply admission gates before rendering, sharing storage reconciliation across a batch.
///
/// The optional lowering is populated only after a macro survives analysis and ABI
/// checks, so skipped batches do not build indexes they cannot use. Only batch
/// generation fills `generated`; a caller's bindings cannot stand in for it.
fn emit_with_lowering<'a>(
    session: &'a AnalysisSession<'_>,
    name: &str,
    bindings: &'a BindingCatalog,
    generated: Generated<'a>,
    lowering: &mut Option<types::Lowering<'a>>,
) -> MacroEmission {
    let mut analysis = session.analyze(name);
    let mut lowered = lower(session, &analysis, bindings, generated, lowering);
    let mut fallback = None;
    // A translated inline function that cannot be rendered is still callable through
    // its guarded native binding. The report keeps the reason the translation failed.
    if let Err(reason) = &lowered
        && translated_inline(session, &analysis)
        && let Some(call) = session.analyze_inline_call(name)
    {
        let call_lowered = lower(session, &call, bindings, generated, lowering);
        if call_lowered.is_ok() {
            fallback = Some(SkipReason { tokens: None, ..reason.clone() });
            analysis = call;
            lowered = call_lowered;
        }
    }
    let status = match lowered {
        Ok((rust, documentation)) => EmissionStatus::Emitted {
            rust,
            const_capability: ConstCapability::RuntimeOnly,
            documentation: Box::new(documentation),
        },
        Err(reason) => EmissionStatus::Skipped { reason },
    };
    MacroEmission { analysis, status, fallback }
}

/// Check bindings and render one analyzed root.
fn lower<'a>(
    session: &'a AnalysisSession<'_>,
    analysis: &MacroAnalysis,
    bindings: &'a BindingCatalog,
    generated: Generated<'a>,
    lowering: &mut Option<types::Lowering<'a>>,
) -> Result<(String, MacroDocumentation), SkipReason> {
    if let Some(reason) = binding_mismatch(session, analysis, bindings) {
        return Err(reason);
    }
    if let AnalysisStatus::Skipped { reason } = &analysis.status {
        return Err(reason.clone());
    }
    let original = match session.frontend().environment().active.get(&analysis.name) {
        Some(active) => InvocationSource::Macro(&active.definition),
        None => InvocationSource::Inline(
            session
                .inline_definition(&analysis.name)
                .expect("analyzed native roots retain their compiler-proven definition"),
        ),
    };
    let assertions = support_abi_assertions(session.frontend().profile()).map_err(|error| {
        skip(analysis, SkipReasonCode::UnsupportedProfile, error.to_string(), None)
    })?;
    let lowering = lowering.get_or_insert_with(|| {
        types::Lowering::new(
            session.frontend().declarations(),
            bindings,
            generated.adapters.map_or(types::GeneratedTypes::NONE, GeneratedAdapters::types),
            &session.frontend().profile().target,
        )
    });
    render(session, analysis, original, &assertions, bindings, generated, lowering)
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
    let requests = demands::plan(session, &renderable(session, names), bindings);
    let adapters = generated_adapters(session, &requests, bindings)?;
    Ok(emit_prepared_batch(session, names, bindings, Some(&adapters)))
}

/// Render candidates, close mismatch failures through the dependency graph, and rerender calls.
///
/// A second propagation step handles late rendering failures so no surviving macro
/// references an exported definition that failed its output bound.
fn emit_prepared_batch(
    session: &AnalysisSession<'_>,
    names: &[impl AsRef<str>],
    bindings: &BindingCatalog,
    adapters: Option<&GeneratedAdapters>,
) -> Vec<MacroEmission> {
    // Each pass borrows one immutable catalog and generated state. Index its C/Rust
    // storage identities once, after the first macro passes its admission gates.
    let initial = {
        let generated = Generated { adapters, ..Generated::NONE };
        let mut lowering = None;
        names
            .iter()
            .map(|name| {
                emit_with_lowering(session, name.as_ref(), bindings, generated, &mut lowering)
            })
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
    let mut translated = translated_callees(session, &initial, &available.macros);
    let mut translations = translation_panics(&initial, &translated);
    let finish = |translations: &BTreeMap<String, BTreeSet<&'static str>>| {
        let generated = Generated { adapters, translations };
        let mut available_lowering = None;
        let mut emissions = initial
            .iter()
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
                    let reason =
                        dependency_skip(&emission.analysis, dependency, root, &roots[root]);
                    MacroEmission {
                        analysis: emission.analysis.clone(),
                        status: EmissionStatus::Skipped { reason },
                        fallback: None,
                    }
                } else if roots.contains_key(name) {
                    let reason = roots[name].clone();
                    MacroEmission {
                        analysis: emission.analysis.clone(),
                        status: EmissionStatus::Skipped { reason },
                        fallback: None,
                    }
                } else {
                    emit_with_lowering(
                        session,
                        name,
                        &available,
                        generated,
                        &mut available_lowering,
                    )
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
                emission.fallback = None;
            }
        }
        emissions
    };
    // A translated callee must itself be emitted as a translation; otherwise its callers
    // keep calling the native binding.
    let mut emissions = finish(&translations);
    loop {
        let lost = translations
            .keys()
            .filter(|name| {
                !emissions.iter().any(|emission| {
                    emission.analysis.name == **name
                        && matches!(emission.status, EmissionStatus::Emitted { .. })
                        && translated_inline(session, &emission.analysis)
                })
            })
            .cloned()
            .collect::<Vec<_>>();
        if lost.is_empty() {
            return emissions;
        }
        translated.retain(|name| !lost.contains(name));
        translations = translation_panics(&initial, &translated);
        emissions = finish(&translations);
    }
}

/// Analyses a batch can render for these roots, for demand planning.
///
/// A translated inline function that cannot be rendered falls back to its native call
/// root, so its plan covers that call root as well as the translated body.
fn renderable(session: &AnalysisSession<'_>, names: &[impl AsRef<str>]) -> Vec<MacroAnalysis> {
    let mut analyses = Vec::with_capacity(names.len());
    for name in names {
        let analysis = session.analyze(name.as_ref());
        let call = translated_inline(session, &analysis)
            .then(|| session.analyze_inline_call(name.as_ref()))
            .flatten();
        analyses.push(analysis);
        analyses.extend(call);
    }
    analyses
}

/// Whether an analysis translates a static inline function's own definition.
fn translated_inline(session: &AnalysisSession<'_>, analysis: &MacroAnalysis) -> bool {
    session.inline_definition(&analysis.name).is_some()
        && analysis
            .expression
            .as_ref()
            .is_some_and(|expression| expression.syntax.statement_body.is_some())
}

/// Translated inline functions whose calls can expand their translation, each after the
/// functions it calls.
///
/// A function that reaches itself through such calls keeps its native binding for them,
/// since expanding it would never terminate.
fn translated_callees(
    session: &AnalysisSession<'_>,
    emissions: &[MacroEmission],
    available: &BTreeSet<String>,
) -> Vec<String> {
    let translated = emissions
        .iter()
        .filter(|emission| {
            available.contains(&emission.analysis.name)
                && matches!(emission.status, EmissionStatus::Emitted { .. })
                && translated_inline(session, &emission.analysis)
        })
        .map(|emission| &emission.analysis)
        .collect::<Vec<_>>();
    let index = translated
        .iter()
        .enumerate()
        .map(|(index, analysis)| (analysis.name.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    let mut graph = petgraph::graph::DiGraph::<(), ()>::with_capacity(translated.len(), 0);
    let nodes = translated.iter().map(|_| graph.add_node(())).collect::<Vec<_>>();
    for (caller, analysis) in translated.iter().enumerate() {
        for callee in direct_calls(analysis) {
            if let Some(&callee) = index.get(callee) {
                graph.add_edge(nodes[caller], nodes[callee], ());
            }
        }
    }
    // Components come in reverse topological order, so callees precede their callers.
    petgraph::algo::tarjan_scc(&graph)
        .into_iter()
        .filter(|component| {
            component.len() == 1 && !graph.contains_edge(component[0], component[0])
        })
        .map(|component| translated[component[0].index()].name.clone())
        .collect()
}

/// The panic conditions of each translation in `translated`, which lists every function
/// after the translated functions it calls.
fn translation_panics(
    emissions: &[MacroEmission],
    translated: &[String],
) -> BTreeMap<String, BTreeSet<&'static str>> {
    let analyses = emissions
        .iter()
        .map(|emission| (emission.analysis.name.as_str(), &emission.analysis))
        .collect::<BTreeMap<_, _>>();
    let mut panics = BTreeMap::new();
    for name in translated {
        let conditions = panic_conditions(analyses[name.as_str()], &panics);
        panics.insert(name.clone(), conditions);
    }
    panics
}

/// Rust panic conditions of an analyzed expression's operations, as documentation text.
///
/// A call that expands a translation has that translation's conditions; any other call
/// is an indirect or guarded native call.
fn panic_conditions(
    analysis: &MacroAnalysis,
    translations: &BTreeMap<String, BTreeSet<&'static str>>,
) -> BTreeSet<&'static str> {
    let nodes = analysis.expression.as_ref().map_or(&[][..], |expression| &expression.syntax.nodes);
    let mut conditions = BTreeSet::new();
    for node in nodes {
        match &node.kind {
            ExpressionKind::Binary { operator, .. }
            | ExpressionKind::Assignment { operator: Some(operator), .. } => match operator {
                BinaryOperator::Divide | BinaryOperator::Remainder => {
                    conditions.insert("Integer division/remainder rejects zero divisors and signed MIN / -1 overflow.");
                }
                BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => {
                    conditions.insert("Shifts reject negative counts and counts at least the promoted left operand's width; signed left shifts also reject values outside the defined C domain.");
                }
                BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Multiply => {
                    conditions.insert("Checked signed arithmetic rejects overflow when the recorded C profile does not define wrapping; pointer arithmetic checks representable offsets and distances but still requires valid allocation bounds.");
                }
                _ => {}
            },
            ExpressionKind::Unary { operator: UnaryOperator::Negate, .. }
            | ExpressionKind::Update { .. } => {
                conditions.insert("Checked signed arithmetic rejects overflow when the recorded C profile does not define wrapping.");
            }
            ExpressionKind::Cast { .. } | ExpressionKind::TypeParameterCast { .. } => {
                conditions.insert("Conversions involving floats reject nonfinite or out-of-range integer results; conversions into Rust enums reject values without a represented discriminant.");
            }
            ExpressionKind::Call { callee, .. } => {
                match direct_callee(nodes, *callee).and_then(|name| translations.get(name)) {
                    Some(translation) => conditions.extend(translation),
                    None => {
                        conditions.insert("Indirect calls reject null function pointers; guarded native calls reject the wrong PostgreSQL thread and propagate PostgreSQL errors.");
                    }
                }
            }
            _ => {}
        }
    }
    conditions
}

/// Names of the functions an analyzed expression calls directly.
fn direct_calls(analysis: &MacroAnalysis) -> impl Iterator<Item = &str> {
    let nodes = analysis.expression.as_ref().map_or(&[][..], |expression| &expression.syntax.nodes);
    nodes.iter().filter_map(move |node| {
        let ExpressionKind::Call { callee, .. } = node.kind else { return None };
        direct_callee(nodes, callee)
    })
}

/// The name of the function a call's callee names, through any grouping.
fn direct_callee(nodes: &[crate::syntax::ExpressionNode], callee: NodeId) -> Option<&str> {
    let mut callee = callee;
    while let ExpressionKind::Group { operand } = nodes[callee].kind {
        callee = operand;
    }
    match &nodes[callee].kind {
        ExpressionKind::Identifier { name } => Some(name.as_str()),
        _ => None,
    }
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
    /// Per-root output and skips produced with the finalized generated capabilities.
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
    let requests = demands::plan(session, &renderable(session, names), bindings);
    let mut adapters = generated_adapters(session, &requests, bindings)?;
    let macros = emit_prepared_batch(session, names, bindings, Some(&adapters));
    // Plan the support from the analyses actually emitted: a translated inline function
    // may have fallen back to its call root.
    let final_requests = demands::plan(
        session,
        macros
            .iter()
            .filter(|emission| matches!(emission.status, EmissionStatus::Emitted { .. }))
            .map(|emission| &emission.analysis),
        bindings,
    );
    if final_requests != requests {
        // Rejected roots cannot retain support that no emitted macro can use.
        // Marker identities depend on C/ABI facts, not the requested root set.
        adapters = generated_adapters(session, &final_requests, bindings)?;
    }
    let mut support = render_adapters(adapters, &macros)?;
    if let Ok(alignment) = crate::support_alignment_profile(session.frontend().profile()) {
        support.rust.push_str(&alignment);
    }
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
    let callbacks = !adapters.callbacks.typedefs.is_empty();
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
    if callbacks {
        rust.push_str(
            "/// Explicit callback typedef identities validated against the current C profile.\n\
             #[doc(hidden)]\npub use self::__pgrx_c_generated::__pgrx_c_callbacks;\n",
        );
    }
    let exports = macros
        .iter()
        .filter(|emission| matches!(emission.status, EmissionStatus::Emitted { .. }))
        .filter_map(|emission| macro_identifier(&emission.analysis.name))
        .collect::<BTreeSet<_>>();
    let operands = macros.iter().any(|emission| {
        matches!(emission.status, EmissionStatus::Emitted { .. })
            && emission.analysis.parameters.iter().any(|parameter| {
                parameter.roles.contains(&crate::ParameterRole::Value)
                    || parameter.roles.contains(&crate::ParameterRole::Spelling)
            })
    });
    rust.push_str(&crate::format_rust_macros(&arguments::shared(&exports, operands)?)?);
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

/// Reconciled capability families before pruning and final artifact assembly.
///
/// Batch rendering reads these capabilities and refusals beside the caller's
/// [`BindingCatalog`]. No public API accepts them, so a caller cannot assert one.
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

impl GeneratedAdapters {
    /// The callback signatures and enum refusals that storage reconciliation consults.
    fn types(&self) -> types::GeneratedTypes<'_> {
        types::GeneratedTypes {
            callbacks: &self.callbacks.markers,
            callback_unavailable: &self.callbacks.unsupported,
            enum_unavailable: &self.enumerations.unsupported,
        }
    }
}

/// Generated state that rendering reads beside the caller's [`BindingCatalog`].
///
/// Only batch emission fills it, so a caller cannot assert any of it. Single-macro
/// emission renders with [`Generated::NONE`].
#[derive(Clone, Copy)]
struct Generated<'a> {
    /// Capabilities and refusals that batch generation established.
    adapters: Option<&'a GeneratedAdapters>,
    /// Static inline functions emitted as translated bodies in this batch, with the
    /// Rust panic conditions their translation can raise, including those of the
    /// translations it expands. Calls to them expand that translation instead of
    /// calling the native binding.
    translations: &'a BTreeMap<String, BTreeSet<&'static str>>,
}

impl Generated<'static> {
    /// No generated capabilities and no inline translations.
    const NONE: Self = Self { adapters: None, translations: &BTreeMap::new() };
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
    let preliminary = callbacks::generate(
        frontend.declarations(),
        bindings,
        types::GeneratedTypes::NONE,
        &frontend.profile().target,
        &requests.functions,
        &callbacks::CallbackRequests::default(),
    )?;
    let functions = functions::generate(
        frontend.declarations(),
        bindings,
        preliminary.types(),
        &requests.functions,
        &requests.called_functions,
        &frontend.profile().target,
    )?;
    // Later adapter passes read replacement storage from `functions`. Rendering
    // instead receives the generated adapters beside the caller's catalog.
    let mut capabilities = bindings.clone();
    capabilities.functions.extend(functions.bindings.clone());
    let callbacks = callbacks::generate(
        frontend.declarations(),
        &capabilities,
        preliminary.types(),
        &frontend.profile().target,
        &requests.functions,
        &requests.callbacks,
    )?;
    // Only the second pass's signatures are emitted. A thunk can make a type's witnesses
    // disagree, so a first-pass marker must not reach a later adapter.
    let generated = callbacks.types();
    let mut required_types = requests.types.values().cloned().collect::<Vec<_>>();
    required_types.extend(callbacks.required_types.iter().cloned());
    let mut enum_requests = enumerations::EnumRequests {
        types: requests.enums.types.clone(),
        open_scalar: requests.enums.open_scalar,
    };
    let lowering = types::Lowering::new(
        frontend.declarations(),
        &capabilities,
        generated,
        &frontend.profile().target,
    );
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
            generated,
            &frontend.profile().target,
            &enum_requests,
        )?,
        fields: fields::generate(
            frontend.declarations(),
            &capabilities,
            generated,
            &requests.fields,
            &frontend.profile().target,
            &required_types,
        )?,
        addresses: addresses::generate(
            frontend.declarations(),
            &capabilities,
            generated,
            &requests.addresses,
            &frontend.profile().target,
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
         (@walk []; $($rest:tt)+) => {{ ::core::compile_error!(\"offsetof member path exceeds the 64-component bound\") }};\n",
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
    rust.push_str("($($unknown:tt)*) => { ::core::compile_error!(\"field has no compiler-verified PostgreSQL binding capability\") };\n}\n");
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
    original: InvocationSource<'_>,
    assertions: &str,
    bindings: &BindingCatalog,
    generated: Generated<'_>,
    lowering: &types::Lowering<'_>,
) -> Result<(String, MacroDocumentation), SkipReason> {
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
    let mut constants = vec![None; expression.syntax.nodes.len()];
    for constant in &expression.constants {
        constants[constant.node] = Some(constant);
    }
    if let Some(node) = typed::null_pointer_access(session.frontend(), analysis, &constants) {
        return Err(skip(
            analysis,
            SkipReasonCode::PointerOperation,
            "an evaluated member access, subscript or dereference through a null pointer constant designates no object",
            Some(expression.syntax.nodes[node].tokens),
        ));
    }
    // A translated inline function is a statement body whose returns yield the macro's
    // value; it is emitted as an expression macro with the function's result type.
    let translated = matches!(original, InvocationSource::Inline(_))
        && expression.syntax.statement_body.is_some();
    let renderer = typed::Renderer::new(
        session.frontend(),
        analysis,
        bindings,
        generated,
        &constants,
        lowering,
        translated,
    );
    let inline_body = translated.then(|| inline_result(session, analysis, lowering)).transpose()?;
    // A translated void body has no return, so its root expression is empty, but the
    // body itself is the macro's value.
    let empty = inline_body.is_none()
        && matches!(expression.syntax.nodes[expression.syntax.root].kind, ExpressionKind::Empty);
    let statement_body =
        expression.syntax.statement_body.as_ref().filter(|_| inline_body.is_none());
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
    let value = if let Some(result) = &inline_body {
        statements::render_inline(session, analysis, &renderer, result.marker())?
    } else if statement_body.is_some() {
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
    let title = match original {
        InvocationSource::Macro(_) => format!("C macro {}", analysis.name),
        InvocationSource::Inline(_) if inline_body.is_some() => {
            format!("C inline function {}", analysis.name)
        }
        InvocationSource::Inline(_) => {
            format!("Typed call adapter for C inline function {}", analysis.name)
        }
    };
    let location = analysis.provenance.as_ref().and_then(|span| {
        let file = span.file.file_name()?.to_string_lossy();
        Some(format!("{file}:{}", span.start_line))
    });
    // The title and location stay separate so documentation snapshots can merge
    // the locations of identical definitions from several PostgreSQL versions.
    let mut comment = String::new();
    let definition = match original {
        InvocationSource::Macro(definition) => definition_comment(definition),
        InvocationSource::Inline(definition) => {
            definition.source.as_deref().and_then(|source| source_comment(source, "c"))
        }
    };
    comment.push_str(&definition.ok_or_else(|| {
        skip(
            analysis,
            SkipReasonCode::BudgetExceeded,
            "original C invocation source is unavailable or exceeds the bounded output size",
            None,
        )
    })?);
    if inline_body.is_some() {
        comment.push_str("\n\n");
        comment.push_str(documentation::INLINE_BODY);
    } else if matches!(original, InvocationSource::Inline(_)) {
        comment.push_str("\n\n");
        comment.push_str(documentation::INLINE_CALL);
    }
    if expression.syntax.nodes.iter().any(|node| {
        matches!(node.kind, ExpressionKind::InvocationFile | ExpressionKind::InvocationLine)
    }) {
        comment.push_str("\n\n");
        comment.push_str(documentation::INVOCATION_DIAGNOSTICS);
    }
    if expression
        .syntax
        .nodes
        .iter()
        .any(|node| matches!(node.kind, ExpressionKind::Stringification { .. }))
    {
        comment.push_str("\n\n");
        comment.push_str(documentation::STRINGIFICATION);
    }
    if expression.syntax.nodes.iter().any(|node| {
        matches!(node.kind, ExpressionKind::StringLiteral { .. } | ExpressionKind::InvocationFile)
    }) {
        comment.push_str(" String arrays retain their complete char-array extent for size and address operations, and decay to read-only pointers for values. Literal mutation is rejected because C string-literal writes are undefined.");
    }
    if returning {
        comment.push_str("\n\n");
        comment.push_str(documentation::CALLER_RETURNS);
    } else if statement_body.is_some() {
        comment.push_str("\n\n");
        comment.push_str(documentation::STATEMENTS);
    }
    if local_guard.is_some() {
        comment.push_str(" Caller argument tokens must not mention the macro's C local names, even inside groups: those invocations are rejected because C substitution can capture locals that Rust hygiene would resolve differently. The scope check also inspects forwarded expression fragments and is bounded to 4096 stringified bytes. Matches in fields, paths or strings are conservatively rejected.");
    }
    let explicit_boundary = inline_body.is_none()
        && analysis.invocation == crate::InvocationContract::ExplicitExpressionBoundary;
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
        let argument_word = if analysis.parameters.len() == 1 { "argument" } else { "arguments" };
        let formal_count = analysis.parameters.len() - captures.len();
        let parameter_word = if formal_count == 1 { "parameter" } else { "parameters" };
        write!(comment, "\n\nRust callers supply {} {argument_word}: the {formal_count} original C {parameter_word}, followed by explicit caller-scope operands in this order: ", analysis.parameters.len()).expect("String output");
        for (index, parameter) in captures.iter().enumerate() {
            if index != 0 {
                comment.push_str(", ");
            }
            write!(comment, "`{}`", parameter.name).expect("String output");
        }
        comment.push_str(". Each operand must preserve its C type and place requirements.");
    }
    comment.push_str("\n\n# Safety\n\n");
    comment.push_str(documentation::SAFETY);
    let failures = panic_conditions(analysis, generated.translations);
    if !failures.is_empty() {
        comment.push_str("\n\n# Panics\n\n");
        for failure in failures {
            comment.push_str(failure);
            comment.push(' ');
        }
        comment.push_str("These checks apply to the operand types selected by this invocation. A Rust panic is converted to PostgreSQL ERROR when it reaches a pgrx extension entry guard; otherwise normal Rust panic behavior applies.");
    }
    let hidden = matches!(analysis.name.as_str(), "_" | "self" | "Self" | "super" | "crate");
    write_doc_comments(&mut rust, &documentation::comment(&title, location.as_deref(), &comment))
        .ok_or_else(|| {
        skip(
            analysis,
            SkipReasonCode::BudgetExceeded,
            "original C macro source comment exceeds the bounded output size",
            None,
        )
    })?;
    if hidden {
        rust.push_str("#[doc(hidden)]\n");
    }
    write!(&mut rust, "#[macro_export]\nmacro_rules! {identifier} {{\n").expect("String output");
    let matcher = arguments::matcher(analysis);
    let operands = (0..analysis.parameters.len())
        .map(|index| format!("${}", arguments::name(analysis, index)))
        .collect::<Vec<_>>()
        .join(", ");
    let safety_checks = arguments::render_safety_checks(analysis, false);
    writeln!(&mut rust, "(@__pgrx_emit_check_safety; {matcher}) => {{ {safety_checks} }};")
        .expect("String output");
    // A size context forwards an operand's tokens unchanged for its safety checks,
    // so an explicit-boundary operand still carries the prefix every mode requires.
    let expression_prefix = if explicit_boundary { "@__pgrx_c_expression; " } else { "" };
    let safety_call = invoke("__pgrx_emit_check_safety", "");
    writeln!(
        &mut rust,
        "(@__pgrx_c_check_safety; {expression_prefix}$($raw:tt)*) => {{ {safety_call} }};"
    )
    .expect("String output");
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
                writeln!(&mut rust, "(@__pgrx_emit_{mode}; $($invalid:tt)*) => {{ ::core::compile_error!(\"arguments do not satisfy this C macro's invocation contract\") }};").expect("String output");
            }
        }
        if !returning && let Some(message) = boundary_error {
            writeln!(
                &mut rust,
                "(@__pgrx_c_discard; $($raw:tt)*) => {{ ::core::compile_error!({message:?}) }};"
            )
            .expect("String output");
        }
        writeln!(&mut rust, "(@$mode:ident; $($raw:tt)*) => {{ ::core::compile_error!(\"a C statement body is not an expression operand\") }};").expect("String output");
    } else {
        for (mode, context) in [
            ("value", typed::Context::Value),
            ("place", typed::Context::Place),
            ("read_place", typed::Context::ReadPlace),
            ("size", typed::Context::Size),
            ("discard", typed::Context::Discard),
        ] {
            let body = if empty {
                "::core::compile_error!(\"an empty C macro has no expression operand\")".into()
            } else if mode == "value" {
                value.clone()
            } else if let Some(result) = &inline_body {
                match (context, result) {
                    (typed::Context::Discard, _) => format!("{{ let _ = {value}; }}"),
                    (typed::Context::Size, InlineResult::Void) => {
                        "compile_error!(\"a void C function call has no size\")".into()
                    }
                    (typed::Context::Size, InlineResult::Value { marker, .. }) => {
                        format!("{SUPPORT}::expression::size_of::<{marker}>()")
                    }
                    _ => "compile_error!(\"a C function call result is not an lvalue\")".into(),
                }
            } else {
                match render_body(session, analysis, bindings, &renderer, context) {
                    Ok(body) => body,
                    Err(reason) => format!("::core::compile_error!({:?})", reason.message),
                }
            };
            writeln!(&mut rust, "(@__pgrx_emit_{mode}; {matcher}) => {{ {body} }};")
                .expect("String output");
            let call = invoke(&format!("__pgrx_emit_{mode}"), "");
            writeln!(
                &mut rust,
                "(@__pgrx_c_{mode}; {expression_prefix}$($raw:tt)*) => {{ {call} }};"
            )
            .expect("String output");
            if arguments.is_none()
                && let Some(message) = boundary_error
            {
                writeln!(
                    &mut rust,
                    "(@__pgrx_c_{mode}; $($raw:tt)*) => {{ ::core::compile_error!({message:?}) }};"
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
            writeln!(&mut rust, "(@$mode:ident; $($invalid:tt)*) => {{ ::core::compile_error!(\"arguments do not satisfy this C macro's invocation contract\") }};").expect("String output");
        }
    }
    if let Some(message) = boundary_error {
        writeln!(&mut rust, "($($raw:tt)*) => {{ ::core::compile_error!({message:?}) }};\n}}")
            .expect("String output");
    } else if arguments.is_none() {
        writeln!(&mut rust, "() => {{ $crate::{identifier}!(@__pgrx_emit_public;) }};\n($($invalid:tt)*) => {{ ::core::compile_error!(\"arguments do not satisfy this C macro's invocation contract\") }};\n}}").expect("String output");
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
    let documentation = MacroDocumentation {
        identifier,
        title,
        location,
        body: comment,
        forms: documentation::documentation_forms(
            analysis,
            returning,
            statement_boundary,
            explicit_boundary,
        ),
        hidden,
        result: documentation::result(analysis, renderer, inline_body.as_ref()),
    };
    Ok((rust, documentation))
}

/// The C result of a translated inline function body.
enum InlineResult<'a> {
    /// A void function, whose body evaluates to `()`.
    Void,
    /// A function that returns a value of its declared C type.
    Value {
        /// The function's declared C return type.
        declared: &'a crate::TypeInfo,
        /// The runtime marker of that type.
        marker: String,
    },
}

impl InlineResult<'_> {
    /// The runtime marker of the returned value, or `None` for a void function.
    fn marker(&self) -> Option<&str> {
        match self {
            Self::Void => None,
            Self::Value { marker, .. } => Some(marker),
        }
    }
}

/// Resolve a translated inline function's C return type.
fn inline_result<'a>(
    session: &'a AnalysisSession<'_>,
    analysis: &MacroAnalysis,
    lowering: &types::Lowering<'_>,
) -> Result<InlineResult<'a>, SkipReason> {
    let refuse = |message: String| skip(analysis, SkipReasonCode::UnsupportedType, message, None);
    let function = session
        .frontend()
        .declarations()
        .function_signatures
        .get(&analysis.name)
        .ok_or_else(|| refuse("inline function has no compiler prototype".into()))?;
    if function.signature.result.category == crate::TypeCategory::Void {
        return Ok(InlineResult::Void);
    }
    let declared = &function.signature.result;
    lowering
        .resolve(declared)
        .map(|lowered| InlineResult::Value { declared, marker: lowered.marker })
        .map_err(refuse)
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
            if expression.syntax.nodes.iter().any(|node| {
                matches!(node.kind, crate::syntax::ExpressionKind::Stringification { .. })
            }) {
                write_fallback(
                    &mut rust,
                    &delegation.callee,
                    "preserving its call would replace diagnostic operand spelling with compiled operand descriptors",
                );
                renderer.render(expression.syntax.root, context, &mut rust)?;
                return Ok(rust);
            }
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
                    ExpressionKind::StringLiteral { .. }
                    | ExpressionKind::Stringification { .. }
                    | ExpressionKind::InvocationFile
                    | ExpressionKind::InvocationLine => {
                        return Err(skip(
                            analysis,
                            SkipReasonCode::UnsupportedLiteral,
                            "string and invocation diagnostics require contextual typed lowering",
                            Some(node.tokens),
                        ));
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
    source_comment(&definition.to_string(), "text")
}

/// Fence bounded original source without letting embedded backticks terminate its documentation.
fn source_comment(definition: &str, language: &str) -> Option<String> {
    let fence_length = definition.split(|character| character != '`').map(str::len).max()?;
    let fence_length = fence_length.saturating_add(1).max(3);
    if definition.len().saturating_add(fence_length.saturating_mul(2)).saturating_add(9)
        > MAX_EMISSION_BYTES
    {
        return None;
    }
    let fence = "`".repeat(fence_length);
    Some(format!("\n\n{fence}{language}\n{definition}\n{fence}\n"))
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
pub(super) fn rust_identifier(name: &str) -> Option<String> {
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
    use super::{MAX_EMISSION_BYTES, definition_comment, rust_identifier, write_doc_comments};
    use crate::{MacroDefinition, MacroKind, SkipReasonCode, Token, TokenKind};

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

    /// An identity-only adapter has no call wrapper. If a call to its function reaches
    /// rendering without demand planning seeing it, the call is refused. It resolves
    /// neither to the wrapper the adapter does not define nor to a bindgen binding that
    /// cannot prove a native-transport signature's ABI.
    #[test]
    fn calls_do_not_resolve_to_identity_only_adapters() {
        let _lock = crate::SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = crate::MacroScanner::new().expect("libclang must be available");
        let directory = tempfile::tempdir().unwrap();
        let header = directory.path().join("identity.h");
        std::fs::write(
            &header,
            "static inline int IdentityOnly(int value) { return value + 1; }\n\
             static inline int CharWiden(char value) { return value; }\n\
             #define IDENTITY_ADDRESS() (&IdentityOnly)\n\
             #define IDENTITY_CALL(value) IdentityOnly(value)\n\
             #define WIDEN_ADDRESS() (&CharWiden)\n\
             #define WIDEN_CALL(value) CharWiden(value)\n",
        )
        .unwrap();
        // A fixed target keeps the support profile and calling convention independent of the host.
        let frontend =
            crate::inspect(&scanner, &header, &["--target=x86_64-unknown-linux-gnu".into()], None)
                .unwrap();
        let addresses = ["IDENTITY_ADDRESS", "WIDEN_ADDRESS"];
        let calls = ["IDENTITY_CALL", "WIDEN_CALL"];
        let session =
            crate::AnalysisSession::prepare(&scanner, &frontend, &[addresses, calls].concat())
                .unwrap();
        // IdentityOnly has no binding. CharWiden has the shape bindgen's static-function
        // wrappers give it, which cannot prove plain char's callable extension attributes.
        let mut bindings = super::BindingCatalog::default();
        bindings.functions.insert(
            "CharWiden".into(),
            crate::FunctionBinding {
                path: vec!["CharWiden".into()],
                parameters: vec![crate::RustBindingType::NativeChar],
                result: crate::RustBindingType::Integer { signed: true, bits: 32 },
                abi: "C".into(),
                variadic: false,
                link_name: "CharWiden__pgrx_cshim".into(),
                guarded: false,
                uses_cshim: true,
            },
        );
        // Plan only the address uses, as if planning had missed the calls.
        let requests =
            super::demands::plan(&session, &super::renderable(&session, &addresses), &bindings);
        let adapters = super::generated_adapters(&session, &requests, &bindings).unwrap();
        for name in ["IdentityOnly", "CharWiden"] {
            assert!(adapters.functions.identities.contains(name), "{name}");
            assert!(adapters.functions.bindings.contains_key(name), "{name}");
            assert!(!adapters.functions.rust.contains(&format!("Inline_{name}")), "{name}");
        }
        let emissions = super::emit_prepared_batch(&session, &calls, &bindings, Some(&adapters));
        assert_eq!(emissions.len(), calls.len());
        for (emission, name) in emissions.iter().zip(["IdentityOnly", "CharWiden"]) {
            let super::EmissionStatus::Skipped { reason } = &emission.status else {
                panic!("{} rendered a call to {name}: {emission:?}", emission.analysis.name);
            };
            assert_eq!(reason.code, SkipReasonCode::Call, "{reason:?}");
            assert!(
                reason.message.contains(&format!("{name}: only an address witness")),
                "{reason:?}"
            );
        }
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
