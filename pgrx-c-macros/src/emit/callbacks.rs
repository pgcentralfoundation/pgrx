//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exact native callback signatures recovered from matching C and binding edges.
//!
//! Callback identity is the compiler C prototype, not merely a Rust function-pointer
//! representation. This pass validates the complete C/Rust witness catalog before
//! pruning requested operations, rejects ambiguous raw-input bridges, and shares
//! physical call machinery without merging distinct C identities.
//! Explicit typedef wrappers preserve a selected C identity when the complete
//! catalog cannot justify inferring that identity from raw Rust storage alone.

use super::types::{LoweredType, Lowering, rust_path};
use crate::{
    BindingCatalog, CallbackBinding, DeclarationCatalog, FunctionSignature, RustBindingType,
    TargetFacts, TypeCategory, TypeInfo, TypeShapeKind,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Write;

/// Private semantic runtime path shared by the native callback support module.
const EXPRESSION: &str = "c::expression";
/// Limit total callback support so an unusually large catalog fails as a structured error.
const SOURCE_LIMIT: usize = 16 * 1024 * 1024;
/// Bound nested C/Rust signature and alias walks, treating exhaustion as unresolved evidence.
const DEPTH_LIMIT: usize = 64;

/// Validated callback identities plus only the operations needed by emitted roots.
pub(super) struct CallbackAdapters {
    /// Nominal signature markers, physical pointer families, and selected call adapters.
    pub rust: String,
    /// Whether selected, validated typedef wrappers need a defining-crate re-export.
    pub has_typedefs: bool,
    /// Compiler canonical function types mapped to verified nullable Rust pointer storage.
    pub markers: BTreeMap<String, CallbackBinding>,
    /// Rejected signature witnesses retained even when demand would omit their code.
    pub unsupported: BTreeMap<String, String>,
    /// Prototype dependencies whose semantic markers need native record or enum support.
    pub required_types: Vec<TypeInfo>,
}

/// Operations that emitted macros can perform on compiler-owned callbacks.
#[derive(Default, PartialEq, Eq)]
pub(super) struct CallbackRequests {
    /// Exact C identities needed for values, comparisons, or other noncall operations.
    pub identity_types: BTreeSet<String>,
    /// Caller operands may introduce raw binding storage, unlike tagged
    /// callback values recovered from globals, fields, or native call results.
    pub native_input_types: BTreeSet<String>,
    /// Exact prototypes whose call capability is required by an analyzed expression.
    pub call_types: BTreeSet<String>,
    /// Retain all possible identities when caller-provided callback types remain unknown.
    pub open_identity: bool,
    /// Each arity excludes only canonical function types incompatible with all
    /// requested open call sites. Missing compiler facts never exclude a type.
    pub open_calls: BTreeMap<usize, BTreeSet<String>>,
}

/// One matched C prototype and Rust storage witness awaiting complete ABI validation.
struct Candidate {
    /// Source callback object type, retaining pointer layout and canonical identity.
    pointer: TypeInfo,
    /// Function declaration type underlying this callback identity.
    function: TypeInfo,
    /// Compiler-owned parameter, result, and calling-convention facts.
    signature: FunctionSignature,
    /// Actual binding storage, including nullable function-pointer representation.
    storage: RustBindingType,
}

/// Catalog-wide evidence used to decide whether raw Rust storage identifies one C callback.
///
/// Missing or rejected edges remain witnesses because pruning them could falsely
/// prove that an equal-storage callback identity is unique.
struct CallbackWitnesses<'catalog, 'lowering> {
    /// Visited C identity and normalized storage pairs, preventing repeated nested walks.
    seen: BTreeSet<(Option<String>, String)>,
    /// Retain missing/rejected C edges as well as usable callback candidates.
    native: Vec<(Option<String>, RustBindingType)>,
    /// A bounded walk failure cannot prove which physical callbacks it hides.
    unresolved: bool,
    /// Known record and enum constructors that cannot hide unresolved alias storage.
    nominal_paths: BTreeSet<&'catalog [String]>,
    /// Central storage renderer used to distinguish constructors from unknown binding aliases.
    lowering: &'lowering Lowering<'catalog>,
}

/// Exact native storage and semantic parameter/result markers after prototype reconciliation.
struct ValidatedAdapter {
    /// Nullable Rust function-pointer storage for the original binding signature.
    storage: String,
    /// Argument markers and ABI storage used after C implicit conversion.
    parameters: Vec<LoweredType>,
    /// Result conversion facts, absent for a C void return.
    result: Option<LoweredType>,
    /// Original and raw ABI signatures when enums or records need validity-preserving transport.
    native_cast: Option<(String, String)>,
}

/// Validate all callback witnesses before selecting identity, raw-input, and call capabilities.
///
/// Raw storage bridges require catalog-wide uniqueness. Rejections and unresolved
/// edges prevent that proof, even if their operations are not requested for output.
pub(super) fn generate(
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    target: &TargetFacts,
    used_functions: &BTreeSet<String>,
    requests: &CallbackRequests,
) -> Result<CallbackAdapters, String> {
    let mut candidates = BTreeMap::new();
    let mut unsupported = BTreeMap::new();
    let witnessed_fields = Lowering::new_native(declarations, bindings, target);
    let mut witnesses = CallbackWitnesses {
        seen: BTreeSet::new(),
        native: Vec::new(),
        unresolved: false,
        nominal_paths: bindings
            .records
            .values()
            .map(|record| record.path.as_slice())
            .chain(bindings.enums.values().map(|binding| binding.path.as_slice()))
            .collect(),
        lowering: &witnessed_fields,
    };
    for (name, alias) in &bindings.types {
        collect(
            bindings.relative_type_name(name).and_then(|name| declarations.types.get(name)),
            &alias.target,
            declarations,
            bindings,
            &mut candidates,
            &mut unsupported,
            &mut witnesses,
            0,
        );
    }
    for (name, variable) in &bindings.variables {
        collect(
            declarations.variables.get(name),
            &variable.ty,
            declarations,
            bindings,
            &mut candidates,
            &mut unsupported,
            &mut witnesses,
            0,
        );
    }
    let mut known_fields = BTreeSet::new();
    for (canonical, record) in &declarations.records {
        let owner = declarations
            .type_shapes
            .get(canonical)
            .and_then(|shape| witnessed_fields.record_binding(&shape.ty).ok());
        for field in &record.fields {
            if let Some(binding) = field
                .name
                .as_ref()
                .and_then(|name| witnessed_fields.named_field_binding(canonical, name))
            {
                if let Some(owner) = owner {
                    known_fields.insert((owner.path.as_slice(), binding.rust_name.as_str()));
                }
                collect(
                    Some(&field.ty),
                    &binding.ty,
                    declarations,
                    bindings,
                    &mut candidates,
                    &mut unsupported,
                    &mut witnesses,
                    0,
                );
            }
        }
    }
    for record in bindings.records.values() {
        for field in record.fields.values() {
            if !known_fields.contains(&(record.path.as_slice(), field.rust_name.as_str())) {
                collect(
                    None,
                    &field.ty,
                    declarations,
                    bindings,
                    &mut candidates,
                    &mut unsupported,
                    &mut witnesses,
                    0,
                );
            }
        }
    }
    for (name, binding) in &bindings.functions {
        // Function items also coerce to native pointers. Their storage can
        // erase a C prototype even when no macro mentions their names.
        match normalize_storage(&function_pointer_storage(binding), bindings, 0) {
            Ok(storage) => witnesses.native.push((
                declarations.functions.get(name).map(|ty| ty.canonical_spelling.clone()),
                storage,
            )),
            Err(_) => witnesses.unresolved = true,
        }
        let function = declarations.function_signatures.get(name);
        for (index, storage) in binding.parameters.iter().enumerate() {
            collect(
                function.and_then(|function| function.signature.parameters.as_ref()?.get(index)),
                storage,
                declarations,
                bindings,
                &mut candidates,
                &mut unsupported,
                &mut witnesses,
                0,
            );
        }
        collect(
            function.map(|function| &function.signature.result),
            &binding.result,
            declarations,
            bindings,
            &mut candidates,
            &mut unsupported,
            &mut witnesses,
            0,
        );
    }
    let witnessed_functions = candidates
        .values()
        .map(|candidate| candidate.function.canonical_spelling.clone())
        .collect::<BTreeSet<_>>();
    for name in used_functions {
        let (Some(binding), Some(ty)) =
            (bindings.functions.get(name), declarations.functions.get(name))
        else {
            continue;
        };
        // A native field/typedef witness takes precedence over a callable thunk,
        // whose MaybeUninit record ABI or C-unwind annotation may deliberately
        // differ in Rust spelling while representing the same original C ABI.
        if witnessed_functions.contains(&ty.canonical_spelling) {
            continue;
        }
        collect(
            Some(ty),
            &function_pointer_storage(binding),
            declarations,
            bindings,
            &mut candidates,
            &mut unsupported,
            &mut witnesses,
            0,
        );
    }

    // A function type can occur behind several typedefs or pointer qualifiers.
    // Preserve one actual ABI witness for that C identity, rather than silently
    // choosing whichever alias was visited last.
    let mut storage_by_function = BTreeMap::<String, RustBindingType>::new();
    let mut inconsistent = BTreeSet::new();
    for candidate in candidates.values() {
        let key = candidate.function.canonical_spelling.clone();
        if storage_by_function.get(&key).is_some_and(|storage| storage != &candidate.storage) {
            inconsistent.insert(key);
        } else {
            storage_by_function.entry(key).or_insert_with(|| candidate.storage.clone());
        }
    }
    for (key, candidate) in &candidates {
        if inconsistent.contains(&candidate.function.canonical_spelling) {
            unsupported.insert(
                key.clone(),
                "one canonical C function type has incompatible actual Rust ABI witnesses".into(),
            );
        }
    }
    for candidate in candidates.values_mut() {
        if candidate.pointer.category == TypeCategory::Function {
            // Clang's GNU sizeof(function) extension describes a function, not
            // its address. Synthetic address signatures use the independently
            // compiler-verified function-pointer layout from this profile.
            candidate.pointer.size = Some(target.function_pointer.size);
            candidate.pointer.alignment = Some(target.function_pointer.alignment);
        }
    }

    // Provisional capabilities permit nested callback arguments/results. No
    // provisional signature is emitted until every dependency is validated.
    let mut capabilities = bindings.clone();
    let mut marker_names = BTreeMap::new();
    for (key, candidate) in &candidates {
        let mut hash = Sha256::new();
        hash.update(&candidate.function.canonical_spelling);
        hash.update(serde_json::to_vec(&candidate.storage).map_err(|error| error.to_string())?);
        hash.update(super::semantic_target_bytes(target)?);
        let suffix =
            hash.finalize()[..16].iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let name = format!("Signature_{suffix}");
        let binding = CallbackBinding {
            marker: format!("$crate::__pgrx_c_generated::{name}"),
            storage: candidate.storage.clone(),
        };
        capabilities.callback_capabilities.insert(key.clone(), binding.clone());
        capabilities
            .callback_capabilities
            .insert(candidate.function.canonical_spelling.clone(), binding);
        marker_names.insert(key.clone(), name);
    }
    let lowering = Lowering::new_native(declarations, &capabilities, target);
    let mut generated = BTreeMap::new();
    let mut reverse = BTreeMap::<String, BTreeSet<String>>::new();
    let mut dependencies_by_key = BTreeMap::new();
    for (key, candidate) in &candidates {
        let mut dependencies = BTreeSet::new();
        for ty in candidate
            .signature
            .parameters
            .iter()
            .flatten()
            .chain(std::iter::once(&candidate.signature.result))
        {
            callback_dependencies(ty, declarations, &mut dependencies, &mut BTreeSet::new());
        }
        for dependency in &dependencies {
            reverse.entry(dependency.clone()).or_default().insert(key.clone());
        }
        dependencies_by_key.insert(key.clone(), dependencies);
        if !unsupported.contains_key(key) {
            match validate_adapter(candidate, &lowering, target) {
                Ok(adapter) => {
                    generated.insert(key.clone(), adapter);
                }
                Err(reason) => {
                    unsupported.insert(key.clone(), reason);
                }
            }
        }
    }
    let mut queue = unsupported.keys().cloned().collect::<VecDeque<_>>();
    while let Some(failed) = queue.pop_front() {
        if let Some(dependents) = reverse.get(&failed) {
            for dependent in dependents {
                if !unsupported.contains_key(dependent) {
                    unsupported.insert(
                        dependent.clone(),
                        format!(
                            "callback depends on unsupported signature {failed}: {}",
                            unsupported[&failed]
                        ),
                    );
                    queue.push_back(dependent.clone());
                }
            }
        }
    }
    let mut output = CallbackAdapters {
        rust: String::new(),
        has_typedefs: false,
        markers: BTreeMap::new(),
        unsupported,
        required_types: Vec::new(),
    };
    // Rust aliases can erase distinct C prototypes. Unselected, rejected, and
    // missing C witnesses still prevent assigning a false native identity.
    let mut validated_storage_by_type = BTreeMap::new();
    for (key, adapter) in &generated {
        if !output.unsupported.contains_key(key) {
            let candidate = &candidates[key];
            let witness =
                (candidate.function.canonical_spelling.as_str(), adapter.storage.as_str());
            validated_storage_by_type.insert(key.as_str(), witness);
            validated_storage_by_type
                .insert(candidate.function.canonical_spelling.as_str(), witness);
        }
    }
    let mut native_identities = BTreeMap::<String, BTreeSet<Option<&str>>>::new();
    for (canonical, storage) in &witnesses.native {
        // These outer Rust function constructors cannot equal a validated fixed
        // C/C-unwind pointer type. Their nested callback edges were still walked.
        if matches!(storage, RustBindingType::Option { value } if matches!(value.as_ref(), RustBindingType::Function { abi, variadic, .. } if *variadic || !matches!(abi.as_str(), "C" | "C-unwind")))
        {
            continue;
        }
        let storage = match lowering.storage_type(storage, 0) {
            Ok(storage) => storage,
            Err(_) => {
                witnesses.unresolved = true;
                continue;
            }
        };
        let storage = storage.replace("$crate", "crate");
        let identity = canonical
            .as_deref()
            .and_then(|canonical| validated_storage_by_type.get(canonical))
            .filter(|(_, known_storage)| *known_storage == storage)
            .map(|(identity, _)| *identity);
        native_identities.entry(storage).or_default().insert(identity);
    }
    // Validation remains independent of requests: an unused alias can expose an
    // incompatible ABI witness, and nested failures invalidate their dependents.
    let mut identities = BTreeSet::new();
    let mut calls = BTreeSet::new();
    let mut native_markers = BTreeSet::new();
    let mut keys_by_type = BTreeMap::<&str, BTreeSet<&str>>::new();
    for key in generated.keys().filter(|key| !output.unsupported.contains_key(*key)) {
        let candidate = &candidates[key];
        keys_by_type.entry(key).or_default().insert(key);
        keys_by_type.entry(&candidate.function.canonical_spelling).or_default().insert(key);
        if requests.call_types.contains(key)
            || requests.call_types.contains(&candidate.function.canonical_spelling)
            || candidate.signature.parameters.as_ref().is_some_and(|parameters| {
                requests.open_calls.get(&parameters.len()).is_some_and(|excluded| {
                    !excluded.contains(&candidate.function.canonical_spelling)
                })
            })
        {
            calls.insert(key.clone());
            identities.insert(key.clone());
        }
        if requests.open_identity
            || requests.identity_types.contains(key)
            || requests.identity_types.contains(&candidate.function.canonical_spelling)
        {
            identities.insert(key.clone());
        }
        if requests.native_input_types.contains(key)
            || requests.native_input_types.contains(&candidate.function.canonical_spelling)
        {
            identities.insert(key.clone());
            native_markers.insert(&marker_names[key]);
        }
    }
    // Call argument/result markers can name callbacks through pointer and array
    // layers. Retain those identities; their raw function storage does not name
    // semantic markers for callbacks nested inside their own prototypes.
    for key in &calls {
        for dependency in &dependencies_by_key[key] {
            identities.extend(
                keys_by_type
                    .get(dependency.as_str())
                    .into_iter()
                    .flatten()
                    .map(|key| (*key).to_owned()),
            );
        }
    }
    let call_markers = calls.iter().map(|key| &marker_names[key]).collect::<BTreeSet<_>>();
    let mut required_types = BTreeMap::new();
    let mut emitted = BTreeSet::new();
    let mut physical_families = BTreeSet::new();
    for (key, adapter) in generated {
        if output.unsupported.contains_key(&key) {
            continue;
        }
        let candidate = &candidates[&key];
        let binding = capabilities.callback_capabilities[&key].clone();
        output.markers.insert(key.clone(), binding.clone());
        output.markers.insert(candidate.function.canonical_spelling.clone(), binding);
        if identities.contains(&key) && emitted.insert(marker_names[&key].clone()) {
            let emit_call = call_markers.contains(&marker_names[&key]);
            output.rust.push_str(&render_adapter(
                candidate,
                &marker_names[&key],
                &adapter,
                bindings,
                emit_call,
                &mut physical_families,
            )?);
            if !witnesses.unresolved
                && native_markers.contains(&marker_names[&key])
                && native_identities.get(&adapter.storage).is_some_and(|identities| {
                    identities.len() == 1
                        && identities
                            .contains(&Some(candidate.function.canonical_spelling.as_str()))
                })
            {
                let storage = &adapter.storage;
                let name = &marker_names[&key];
                writeln!(output.rust, "impl c::sealed::Sealed for {storage} {{}}\nimpl {EXPRESSION}::NativeType for {storage} {{ type Marker = {EXPRESSION}::CFunction<{name}>; }}\nimpl {EXPRESSION}::IntoExpression for {storage} {{ type Value = {EXPRESSION}::FunctionValue<{name}>; fn into_expression(self) -> Self::Value {{ {EXPRESSION}::FunctionValue::new(self) }} }}").expect("String output");
            }
            if emit_call {
                for ty in candidate
                    .signature
                    .parameters
                    .iter()
                    .flatten()
                    .chain(std::iter::once(&candidate.signature.result))
                {
                    required_types
                        .entry(ty.canonical_spelling.clone())
                        .or_insert_with(|| ty.clone());
                }
            }
        }
        if output.rust.len() > SOURCE_LIMIT {
            return Err("generated callback adapters exceed the 16 MiB source budget".into());
        }
    }
    output.required_types = required_types.into_values().collect();
    let aliases = typedef_aliases(declarations, bindings, &output.markers, &emitted)?;
    output.has_typedefs = !aliases.is_empty();
    output.rust.push_str(&aliases);
    if output.rust.len() > SOURCE_LIMIT {
        return Err("generated callback adapters exceed the 16 MiB source budget".into());
    }
    Ok(output)
}

/// Expose selected, validated C typedef identities without inferring a raw storage identity.
///
/// `FunctionValue::new` accepts only its signature's exact native representation.
/// The explicit typedef therefore adds no ABI cast and cannot bypass missing or
/// rejected prototype evidence. Nested modules mirror the actual binding path.
fn typedef_aliases(
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    markers: &BTreeMap<String, CallbackBinding>,
    emitted: &BTreeSet<String>,
) -> Result<String, String> {
    let mut aliases = BTreeMap::new();
    for (name, alias) in &bindings.types {
        let Some(name) = bindings.relative_type_name(name) else { continue };
        let Some(ty) = declarations.types.get(name) else { continue };
        let Some(binding) = markers.get(&ty.canonical_spelling) else { continue };
        let Some(marker) = binding.marker.strip_prefix("$crate::__pgrx_c_generated::") else {
            continue;
        };
        if !emitted.contains(marker)
            || alias.path.len() > DEPTH_LIMIT
            || rust_path(&alias.path).is_err()
        {
            continue;
        }
        let Ok(storage) = normalize_storage(&alias.target, bindings, 0) else { continue };
        if storage != binding.storage {
            continue;
        }
        let Some(path) = bindings.relative_path(&alias.path) else { continue };
        if let Some(previous) = aliases.insert(path.to_vec(), (name, marker))
            && previous.1 != marker
        {
            return Err("callback typedefs have conflicting actual Rust binding paths".into());
        }
    }
    if aliases.is_empty() {
        return Ok(String::new());
    }
    let mut rust = String::from(
        "/// Explicit C typedef identities for the selected native callback capabilities.\n\
         pub mod __pgrx_c_callbacks {\n",
    );
    render_typedef_aliases(&mut rust, &aliases, 0)?;
    rust.push_str("}\n");
    Ok(rust)
}

/// Render one level of validated typedef paths without flattening distinct binding modules.
fn render_typedef_aliases(
    rust: &mut String,
    aliases: &BTreeMap<Vec<String>, (&str, &str)>,
    depth: usize,
) -> Result<(), String> {
    let mut modules = BTreeMap::<&str, BTreeMap<Vec<String>, (&str, &str)>>::new();
    let mut names = BTreeSet::new();
    for (path, (name, marker)) in aliases {
        let identifier = &path[depth];
        if path.len() == depth + 1 {
            names.insert(identifier.as_str());
            writeln!(
                rust,
                "#[doc = {:?}]\npub type {identifier} = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::{marker}>;",
                format!("Explicit C callback typedef `{name}` with its exact native binding storage.")
            )
            .expect("String output");
        } else {
            modules.entry(identifier).or_default().insert(path.clone(), (*name, *marker));
        }
    }
    for (module, aliases) in modules {
        if names.contains(module) {
            return Err("callback typedef path conflicts with a binding module".into());
        }
        writeln!(
            rust,
            "/// Callback typedef identities from the corresponding binding module.\npub mod {module} {{"
        )
        .expect("String output");
        render_typedef_aliases(rust, &aliases, depth + 1)?;
        rust.push_str("}\n");
    }
    Ok(())
}

/// Walk matching C/Rust edges, retaining usable callbacks and negative uniqueness evidence.
///
/// Alias failure or bounded-depth exhaustion remains unresolved rather than silently
/// removing a possible competing callback representation.
#[allow(clippy::too_many_arguments)] // One bounded walk owns matching C/Rust type edges.
fn collect(
    ty: Option<&TypeInfo>,
    storage: &RustBindingType,
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    candidates: &mut BTreeMap<String, Candidate>,
    unsupported: &mut BTreeMap<String, String>,
    witnesses: &mut CallbackWitnesses<'_, '_>,
    depth: usize,
) {
    if depth > DEPTH_LIMIT {
        witnesses.unresolved = true;
        if let Some(ty) = ty {
            unsupported.insert(
                ty.canonical_spelling.clone(),
                "callback C/Rust type walk exceeds its bounded depth".into(),
            );
        }
        return;
    }
    let storage = match normalize_storage(storage, bindings, 0) {
        Ok(storage) => storage,
        Err(reason) => {
            witnesses.unresolved = true;
            if let Some(ty) = ty {
                unsupported.insert(ty.canonical_spelling.clone(), reason);
            }
            return;
        }
    };
    let fingerprint = serde_json::to_string(&storage).expect("binding types are serializable");
    let canonical = ty.map(|ty| ty.canonical_spelling.clone());
    if !witnesses.seen.insert((canonical.clone(), fingerprint)) {
        return;
    }
    if matches!(&storage, RustBindingType::Option { value } if matches!(value.as_ref(), RustBindingType::Function { .. }))
    {
        witnesses.native.push((canonical, storage.clone()));
    }
    let shape = ty.and_then(|ty| declarations.type_shapes.get(&ty.canonical_spelling));
    let (Some(ty), Some(shape)) = (ty, shape) else {
        collect_unknown_storage(
            &storage,
            declarations,
            bindings,
            candidates,
            unsupported,
            witnesses,
            depth,
        );
        return;
    };
    match (&shape.kind, &storage) {
        (
            TypeShapeKind::Function { signature },
            RustBindingType::Option { .. } | RustBindingType::Function { .. },
        ) => {
            collect_signature(
                ty,
                ty,
                signature,
                &storage,
                declarations,
                bindings,
                candidates,
                unsupported,
                witnesses,
                depth,
            );
        }
        (
            TypeShapeKind::Pointer { pointee },
            RustBindingType::Option { .. } | RustBindingType::Function { .. },
        ) if pointee.category == TypeCategory::Function => {
            let Some(crate::TypeShape { kind: TypeShapeKind::Function { signature }, .. }) =
                declarations.type_shapes.get(&pointee.canonical_spelling)
            else {
                unsupported.insert(
                    ty.canonical_spelling.clone(),
                    "callback has no compiler-owned function prototype".into(),
                );
                collect_unknown_storage(
                    &storage,
                    declarations,
                    bindings,
                    candidates,
                    unsupported,
                    witnesses,
                    depth,
                );
                return;
            };
            collect_signature(
                ty,
                pointee,
                signature,
                &storage,
                declarations,
                bindings,
                candidates,
                unsupported,
                witnesses,
                depth,
            );
        }
        (TypeShapeKind::Pointer { pointee }, RustBindingType::Pointer { pointee: actual, .. }) => {
            collect(
                Some(pointee),
                actual,
                declarations,
                bindings,
                candidates,
                unsupported,
                witnesses,
                depth + 1,
            );
        }
        (TypeShapeKind::Array { element, .. }, RustBindingType::Array { element: actual, .. }) => {
            collect(
                Some(element),
                actual,
                declarations,
                bindings,
                candidates,
                unsupported,
                witnesses,
                depth + 1,
            );
        }
        _ => collect_unknown_storage(
            &storage,
            declarations,
            bindings,
            candidates,
            unsupported,
            witnesses,
            depth,
        ),
    }
}

/// Inspect nested binding storage even when its C edge is missing or incompatible.
///
/// Unknown aliases block uniqueness; recognized record, enum, and c_void constructors
/// do not conceal another function-pointer representation.
fn collect_unknown_storage(
    storage: &RustBindingType,
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    candidates: &mut BTreeMap<String, Candidate>,
    unsupported: &mut BTreeMap<String, String>,
    witnesses: &mut CallbackWitnesses<'_, '_>,
    depth: usize,
) {
    if let RustBindingType::Named { path } = storage {
        // A missing alias definition can hide another callback representation.
        // Actual record/enum constructors cannot; the central renderer also
        // recognizes the standard library's fixed c_void constructor.
        if !witnesses.nominal_paths.contains(path.as_slice())
            && witnesses
                .lowering
                .storage_type(storage, 0)
                .map_or(true, |storage| storage != "::core::ffi::c_void")
        {
            witnesses.unresolved = true;
        }
        return;
    }
    let mut collect_child = |storage| {
        collect(
            None,
            storage,
            declarations,
            bindings,
            candidates,
            unsupported,
            witnesses,
            depth + 1,
        );
    };
    match storage {
        RustBindingType::Option { value }
        | RustBindingType::MaybeUninit { value }
        | RustBindingType::ManuallyDrop { value }
        | RustBindingType::Pointer { pointee: value, .. }
        | RustBindingType::Array { element: value, .. }
        | RustBindingType::IncompleteArrayField { element: value, .. } => {
            collect_child(value.as_ref())
        }
        RustBindingType::Function { parameters, result, .. } => {
            for storage in parameters.iter().chain(std::iter::once(result.as_ref())) {
                collect_child(storage);
            }
        }
        _ => {}
    }
}

/// Register a canonical prototype witness and recursively inspect its parameter/result storage.
#[allow(clippy::too_many_arguments)] // Matching prototypes share the same bounded C/Rust walk.
fn collect_signature(
    pointer: &TypeInfo,
    function_type: &TypeInfo,
    signature: &FunctionSignature,
    storage: &RustBindingType,
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    candidates: &mut BTreeMap<String, Candidate>,
    unsupported: &mut BTreeMap<String, String>,
    witnesses: &mut CallbackWitnesses<'_, '_>,
    depth: usize,
) {
    let candidate = Candidate {
        pointer: pointer.clone(),
        function: function_type.clone(),
        signature: signature.clone(),
        storage: storage.clone(),
    };
    if candidates
        .get(&pointer.canonical_spelling)
        .is_some_and(|previous| previous.storage != *storage)
    {
        unsupported.insert(
            pointer.canonical_spelling.clone(),
            "one canonical C callback has incompatible actual Rust storage witnesses".into(),
        );
    } else {
        candidates.entry(pointer.canonical_spelling.clone()).or_insert(candidate);
    }
    let function = match storage {
        RustBindingType::Option { value } => value.as_ref(),
        function => function,
    };
    if let RustBindingType::Function { parameters, result, .. } = function {
        for (index, storage) in parameters.iter().enumerate() {
            collect(
                signature.parameters.as_ref().and_then(|parameters| parameters.get(index)),
                storage,
                declarations,
                bindings,
                candidates,
                unsupported,
                witnesses,
                depth + 1,
            );
        }
        collect(
            Some(&signature.result),
            result,
            declarations,
            bindings,
            candidates,
            unsupported,
            witnesses,
            depth + 1,
        );
    }
}

/// Expand binding aliases and nested callback storage within a fixed depth budget.
fn normalize_storage(
    storage: &RustBindingType,
    bindings: &BindingCatalog,
    depth: usize,
) -> Result<RustBindingType, String> {
    if depth > DEPTH_LIMIT {
        return Err("callback storage alias/type chain exceeds its bounded depth".into());
    }
    Ok(match storage {
        RustBindingType::Named { path } => {
            let key = path
                .iter()
                .map(|part| part.strip_prefix("r#").unwrap_or(part))
                .collect::<Vec<_>>()
                .join("::");
            if let Some(alias) = bindings.types.get(&key) {
                return normalize_storage(&alias.target, bindings, depth + 1);
            }
            storage.clone()
        }
        RustBindingType::Pointer { pointee, mutable } => RustBindingType::Pointer {
            pointee: Box::new(normalize_storage(pointee, bindings, depth + 1)?),
            mutable: *mutable,
        },
        RustBindingType::Array { element, length } => RustBindingType::Array {
            element: Box::new(normalize_storage(element, bindings, depth + 1)?),
            length: *length,
        },
        RustBindingType::Option { value } => RustBindingType::Option {
            value: Box::new(normalize_storage(value, bindings, depth + 1)?),
        },
        RustBindingType::MaybeUninit { value } => RustBindingType::MaybeUninit {
            value: Box::new(normalize_storage(value, bindings, depth + 1)?),
        },
        RustBindingType::Function { parameters, result, abi, unsafe_, variadic } => {
            RustBindingType::Function {
                parameters: parameters
                    .iter()
                    .map(|ty| normalize_storage(ty, bindings, depth + 1))
                    .collect::<Result<_, _>>()?,
                result: Box::new(normalize_storage(result, bindings, depth + 1)?),
                abi: abi.clone(),
                unsafe_: *unsafe_,
                variadic: *variadic,
            }
        }
        _ => storage.clone(),
    })
}

/// Represent a function binding as nullable unsafe storage for callback ABI comparisons.
fn function_pointer_storage(binding: &crate::FunctionBinding) -> RustBindingType {
    RustBindingType::Option {
        value: Box::new(RustBindingType::Function {
            parameters: binding.parameters.clone(),
            result: Box::new(binding.result.clone()),
            abi: binding.abi.clone(),
            unsafe_: true,
            variadic: binding.variadic,
        }),
    }
}

/// Collect nested function identities without demanding call operations for those identities.
fn callback_dependencies(
    ty: &TypeInfo,
    declarations: &DeclarationCatalog,
    output: &mut BTreeSet<String>,
    seen: &mut BTreeSet<String>,
) {
    let mut pending = vec![ty];
    while let Some(ty) = pending.pop() {
        if !seen.insert(ty.canonical_spelling.clone()) {
            continue;
        }
        match declarations.type_shapes.get(&ty.canonical_spelling).map(|shape| &shape.kind) {
            Some(TypeShapeKind::Pointer { pointee })
                if pointee.category == TypeCategory::Function =>
            {
                output.insert(ty.canonical_spelling.clone());
            }
            Some(TypeShapeKind::Pointer { pointee }) => pending.push(pointee),
            Some(TypeShapeKind::Array { element, .. }) => pending.push(element),
            Some(TypeShapeKind::Function { .. }) => {
                output.insert(ty.canonical_spelling.clone());
            }
            _ => {}
        }
    }
}

/// Reconcile arity, ABI, nullability, qualifiers, and every parameter/result representation.
///
/// By-value aggregates use raw record storage and C enums use their compatible
/// integer ABI so the native call does not materialize invalid Rust values.
fn validate_adapter(
    candidate: &Candidate,
    lowering: &Lowering<'_>,
    target: &TargetFacts,
) -> Result<ValidatedAdapter, String> {
    let signature = &candidate.signature;
    let parameters = signature.parameters.as_ref().ok_or("callback has no C prototype")?;
    if signature.variadic {
        return Err("variadic callbacks have no fixed ABI capability".into());
    }
    if signature.calling_convention.as_deref() != Some("Cdecl") {
        return Err("callback does not use the verified default C calling convention".into());
    }
    let RustBindingType::Option { value } = &candidate.storage else {
        return Err(
            "bare Rust function storage cannot represent nullable C function pointers".into()
        );
    };
    let RustBindingType::Function { parameters: actual, result, abi, variadic, unsafe_, .. } =
        value.as_ref()
    else {
        return Err(
            "nullable callback storage does not contain an actual native function type".into()
        );
    };
    if !*unsafe_ {
        return Err("native C callback storage must require an unsafe call".into());
    }
    if *variadic || !matches!(abi.as_str(), "C" | "C-unwind") || parameters.len() != actual.len() {
        return Err(
            "callback binding arity, ABI, or variadic status differs from the C prototype".into()
        );
    }
    let storage = lowering.storage_type(&candidate.storage, 0)?.replace("$crate", "crate");
    let native_storage =
        |ty: &TypeInfo, actual: &RustBindingType| -> Result<RustBindingType, String> {
            value_type(ty)?;
            if ty.category == TypeCategory::Integer(crate::IntegerKind::Char) {
                lowering.resolve_with_storage(ty, actual)?;
                let integer = target
                    .integers
                    .get(&crate::IntegerKind::Char)
                    .ok_or("plain-char identity is absent from the target profile")?;
                // Calls use the actual C prototype's extension attributes, not
                // the sign of the binding's byte-compatible storage alias.
                Ok(RustBindingType::Integer { signed: integer.signed, bits: integer.bits })
            } else if ty.category == TypeCategory::Enum {
                // A closed Rust enum cannot transport arbitrary valid C enum
                // values. First reconcile the actual nominal enum storage,
                // then use Clang's compatible integer type at the call ABI.
                lowering.resolve_with_storage(ty, actual)?;
                let underlying = lowering.enum_underlying(ty)?;
                let TypeCategory::Integer(kind) = underlying.category else {
                    unreachable!("enum_underlying verifies the integer category")
                };
                let integer = target
                    .integers
                    .get(&kind)
                    .ok_or("enum compatible integer is absent from the target profile")?;
                Ok(RustBindingType::Integer { signed: integer.signed, bits: integer.bits })
            } else if ty.category == TypeCategory::Record
                && !matches!(actual, RustBindingType::MaybeUninit { .. })
            {
                Ok(RustBindingType::MaybeUninit { value: Box::new(actual.clone()) })
            } else {
                Ok(actual.clone())
            }
        };
    let parameter_storage = parameters
        .iter()
        .zip(actual)
        .map(|(ty, actual)| native_storage(ty, actual))
        .collect::<Result<Vec<_>, _>>()?;
    let result_storage = if signature.result.category == TypeCategory::Void {
        RustBindingType::Unit
    } else {
        native_storage(&signature.result, result)?
    };
    let lowered = parameters
        .iter()
        .zip(&parameter_storage)
        .map(|(ty, storage)| lowering.resolve_with_storage(ty, storage))
        .collect::<Result<Vec<_>, String>>()?;
    let lowered_result = if signature.result.category == TypeCategory::Void {
        if result.as_ref() != &RustBindingType::Unit {
            return Err("C void callback return differs from actual Rust storage".into());
        }
        None
    } else {
        Some(lowering.resolve_with_storage(&signature.result, &result_storage)?)
    };
    let native_cast = if &parameter_storage != actual || &result_storage != result.as_ref() {
        let original = lowering.storage_type(value, 0)?.replace("$crate", "crate");
        let raw = lowering
            .storage_type(
                &RustBindingType::Function {
                    parameters: parameter_storage,
                    result: Box::new(result_storage),
                    abi: abi.clone(),
                    unsafe_: *unsafe_,
                    variadic: false,
                },
                0,
            )?
            .replace("$crate", "crate");
        Some((original, raw))
    } else {
        None
    };
    Ok(ValidatedAdapter { storage, parameters: lowered, result: lowered_result, native_cast })
}

/// Emit a nominal callback shell and, when requested, its explicit unsafe call capability.
///
/// Argument conversions precede the guarded native call and result decoding follows
/// it; physical pointer operations can be shared without merging C prototypes.
fn render_adapter(
    candidate: &Candidate,
    name: &str,
    adapter: &ValidatedAdapter,
    bindings: &BindingCatalog,
    emit_call: bool,
    physical_families: &mut BTreeSet<(String, usize)>,
) -> Result<String, String> {
    let mut rust = String::new();
    let storage = &adapter.storage;
    abi_assertions(&mut rust, storage, &candidate.pointer);
    let RustBindingType::Option { value } = &candidate.storage else {
        unreachable!("callback emission follows nullable pointer validation")
    };
    let RustBindingType::Function { parameters, abi, .. } = value.as_ref() else {
        unreachable!("callback emission follows native function storage validation")
    };
    let family = physical_family(&mut rust, abi, parameters.len(), physical_families);
    writeln!(rust, "#[doc(hidden)] #[derive(Clone, Copy)] pub struct {name};\nimpl c::sealed::Sealed for {name} {{}}\nimpl {EXPRESSION}::NativeFunctionSignature for {name} {{\n type Physical = {family}<{storage}>;\n}}").expect("String output");
    if !emit_call {
        return Ok(rust);
    }
    let lowered = &adapter.parameters;
    let lowered_result = &adapter.result;
    for (ty, lowered) in candidate.signature.parameters.iter().flatten().zip(lowered) {
        abi_assertions(&mut rust, &lowered.storage.replace("$crate", "crate"), ty);
    }
    if let Some(result) = lowered_result {
        abi_assertions(
            &mut rust,
            &result.storage.replace("$crate", "crate"),
            &candidate.signature.result,
        );
    }
    let generics = lowered
        .iter()
        .enumerate()
        .map(|(index, ty)| {
            format!("A{index}: {EXPRESSION}::ImplicitTo<{}>", ty.marker.replace("$crate", "crate"))
        })
        .collect::<Vec<_>>();
    let args = (0..lowered.len()).map(|index| format!("A{index},")).collect::<String>();
    let output = lowered_result.as_ref().map_or("()".into(), |ty| {
        format!("<{} as {EXPRESSION}::CType>::Value", ty.marker.replace("$crate", "crate"))
    });
    let generic =
        if generics.is_empty() { String::new() } else { format!("<{}>", generics.join(", ")) };
    let argument_name = if lowered.is_empty() { "_args" } else { "args" };
    writeln!(rust, "// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.\nunsafe impl{generic} {EXPRESSION}::Call<({args})> for {name} {{\n type Output = {output};\n unsafe fn call(pointer: Self::Pointer, {argument_name}: ({args})) -> Self::Output {{").expect("String output");
    writeln!(
        rust,
        "let function = pointer.expect(\"C indirect call requires a non-null function pointer\");"
    )
    .expect("String output");
    if let Some((original, raw)) = &adapter.native_cast {
        writeln!(rust, "// SAFETY: The caller establishes that this address has the original C prototype, including plain-char signedness. The raw signature uses that compiler-proven char sign and enum-compatible integer ABI; MaybeUninit<R> preserves record layout/ABI. The binding signature is pointer storage only and is not invoked. Calling convention and pointer representation are unchanged; no invalid enum or uninitialized record value is materialized.\nlet function: {raw} = unsafe {{ ::core::mem::transmute::<{original}, {raw}>(function) }};").expect("String output");
    }
    for (index, ty) in lowered.iter().enumerate() {
        let marker = ty.marker.replace("$crate", "crate");
        let storage = ty.storage.replace("$crate", "crate");
        writeln!(rust, "const {{ assert!(!::core::mem::needs_drop::<A{index}>()); }}\nconst _: () = assert!(!::core::mem::needs_drop::<{storage}>());\nlet native{index} = <{marker} as {EXPRESSION}::CType>::into_storage({EXPRESSION}::implicit::<{marker}, _>(args.{index}));").expect("String output");
    }
    let native = format!(
        "function({})",
        (0..lowered.len()).map(|index| format!("native{index}")).collect::<Vec<_>>().join(", ")
    );
    let call = if let Some(boundary) = &bindings.ffi_boundary {
        format!("{}(move || {native})", rust_path(boundary)?.replace("$crate", "crate"))
    } else {
        native
    };
    writeln!(rust, "// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.\nlet result = unsafe {{ {call} }};").expect("String output");
    if let Some(result) = lowered_result {
        writeln!(
            rust,
            "<{} as {EXPRESSION}::CType>::from_storage(result)",
            result.marker.replace("$crate", "crate")
        )
        .expect("String output");
    } else {
        rust.push_str("result\n");
    }
    rust.push_str("}\n}\n");
    Ok(rust)
}

/// Only native storage participates in a physical family. Identity-only shells
/// therefore do not pull semantic markers from their prototypes' nested types.
fn physical_family(
    rust: &mut String,
    abi: &str,
    arity: usize,
    emitted: &mut BTreeSet<(String, usize)>,
) -> String {
    let suffix = match abi {
        "C" => "C",
        "C-unwind" => "C_unwind",
        _ => unreachable!("callback emission follows verified C ABI validation"),
    };
    let name = format!("PhysicalFunction_{suffix}_{arity}");
    if emitted.insert((abi.to_owned(), arity)) {
        let arguments = (0..arity).map(|index| format!("A{index}")).collect::<Vec<_>>();
        let parameters = arguments.join(", ");
        let mut generics = arguments;
        generics.push("R".into());
        let generics = generics.join(", ");
        let pointer =
            format!("::core::option::Option<unsafe extern \"{abi}\" fn({parameters}) -> R>");
        writeln!(rust, "#[doc(hidden)] pub struct {name}<P>(::core::marker::PhantomData<P>);\nimpl<P> c::sealed::Sealed for {name}<P> {{}}\nimpl<{generics}> {EXPRESSION}::PhysicalFunctionPointer for {name}<{pointer}> {{\n type Pointer = {pointer};\n fn null() -> Self::Pointer {{ None }}\n fn address(pointer: Self::Pointer) -> *const () {{ pointer.map_or(::core::ptr::null(), |function| function as *const ()) }}\n}}").expect("String output");
    }
    name
}

/// Reject function, void, and unresolved object categories at by-value ABI positions.
fn value_type(ty: &TypeInfo) -> Result<(), String> {
    if matches!(ty.category, TypeCategory::Void | TypeCategory::Function | TypeCategory::Other) {
        return Err("callback value has no established scalar or pointer ABI representation".into());
    }
    Ok(())
}

/// Emit compiler-owned size and alignment witnesses for admitted Rust callback storage.
fn abi_assertions(output: &mut String, storage: &str, ty: &TypeInfo) {
    if let Some(size) = ty.size {
        writeln!(output, "const _: () = assert!(::core::mem::size_of::<{storage}>() == {size});")
            .expect("String output");
    }
    if let Some(alignment) = ty.alignment {
        writeln!(
            output,
            "const _: () = assert!(::core::mem::align_of::<{storage}>() == {alignment});"
        )
        .expect("String output");
    }
}

/// Synthetic-catalog regressions for callback identity, complete witness validation, and demand pruning.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AliasBinding, ByteOrder, IntegerKind, IntegerType, PointerLayout, TypeShape};

    /// Construct a fixed-width C int witness for synthetic callback catalogs.
    fn integer() -> TypeInfo {
        TypeInfo {
            spelling: "int".into(),
            canonical_spelling: "int".into(),
            category: TypeCategory::Integer(IntegerKind::Int),
            size: Some(4),
            alignment: Some(4),
            is_const: false,
            is_volatile: false,
        }
    }

    /// Provide the Rust integer storage paired with the synthetic C int witness.
    fn storage() -> RustBindingType {
        RustBindingType::Integer { signed: true, bits: 32 }
    }

    /// Add a matched named C callback and Rust alias, including nested signature facts.
    fn add_callback(
        declarations: &mut DeclarationCatalog,
        bindings: &mut BindingCatalog,
        name: &str,
        arity: usize,
        result: TypeInfo,
        result_storage: RustBindingType,
    ) -> TypeInfo {
        let arguments = vec!["int"; arity].join(", ");
        let function = TypeInfo {
            spelling: format!("{} ({arguments})", result.canonical_spelling),
            canonical_spelling: format!("{} ({arguments})", result.canonical_spelling),
            category: TypeCategory::Function,
            size: None,
            alignment: None,
            ..integer()
        };
        let pointer = TypeInfo {
            spelling: name.into(),
            canonical_spelling: format!("{} (*)({arguments})", result.canonical_spelling),
            category: TypeCategory::Pointer,
            size: Some(8),
            alignment: Some(8),
            ..integer()
        };
        declarations.types.insert(name.into(), pointer.clone());
        declarations.type_shapes.insert(
            function.canonical_spelling.clone(),
            TypeShape {
                ty: function.clone(),
                is_restrict: false,
                kind: TypeShapeKind::Function {
                    signature: FunctionSignature {
                        result,
                        parameters: Some(vec![integer(); arity]),
                        variadic: false,
                        calling_convention: Some("Cdecl".into()),
                    },
                },
            },
        );
        declarations.type_shapes.insert(
            pointer.canonical_spelling.clone(),
            TypeShape {
                ty: pointer.clone(),
                is_restrict: false,
                kind: TypeShapeKind::Pointer { pointee: function },
            },
        );
        bindings.types.insert(
            name.into(),
            AliasBinding {
                path: vec![name.into()],
                target: RustBindingType::Option {
                    value: Box::new(RustBindingType::Function {
                        parameters: vec![storage(); arity],
                        result: Box::new(result_storage),
                        abi: "C-unwind".into(),
                        unsafe_: true,
                        variadic: false,
                    }),
                },
            },
        );
        pointer
    }

    /// Build a small target and callback catalog with both scalar and nested signatures.
    fn fixture() -> (DeclarationCatalog, BindingCatalog, TargetFacts) {
        let target = TargetFacts {
            triple: "fixture".into(),
            pointer_bits: 64,
            function_pointer: PointerLayout { size: 8, alignment: 8 },
            size_type: IntegerKind::UnsignedLong,
            ptrdiff_type: crate::IntegerKind::Long,
            preferred_alignments: Default::default(),
            arm_float_abi: None,
            ppc64_elf_abi: None,
            offsetof_supported: true,
            char_bits: 8,
            char_is_signed: true,
            ascii_execution_charset: true,
            byte_order: ByteOrder::Little,
            c_standard: Some(201710),
            integers: [(
                IntegerKind::Int,
                IntegerType { kind: IntegerKind::Int, bits: 32, signed: true, rank: 3 },
            )]
            .into_iter()
            .collect(),
            floating_point: Default::default(),
        };
        let mut declarations = DeclarationCatalog::default();
        let mut bindings = BindingCatalog::default();
        add_callback(&mut declarations, &mut bindings, "One", 1, integer(), storage());
        add_callback(&mut declarations, &mut bindings, "Two", 2, integer(), storage());
        (declarations, bindings, target)
    }

    /// Check demand omits unused call bodies while keeping complete callback metadata for validation.
    #[test]
    fn requests_select_identity_and_call_operations_without_pruning_metadata() {
        let (declarations, bindings, target) = fixture();
        let empty = generate(
            &declarations,
            &bindings,
            &target,
            &BTreeSet::new(),
            &CallbackRequests::default(),
        )
        .unwrap();
        assert!(empty.rust.is_empty());
        assert_eq!(empty.markers.len(), 4);
        assert!(empty.required_types.is_empty());
        let requests = CallbackRequests {
            identity_types: [declarations.types["One"].canonical_spelling.clone()]
                .into_iter()
                .collect(),
            ..Default::default()
        };
        let identity =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert_eq!(identity.rust.matches("::NativeFunctionSignature for").count(), 1);
        assert!(!identity.rust.contains("::Call<"));
        assert!(!identity.rust.contains("::NativeType for"));
        assert!(!identity.rust.contains("::IntoExpression for"));
        assert_eq!(identity.markers.len(), empty.markers.len());
        assert!(identity.required_types.is_empty());
        let requests = CallbackRequests {
            open_identity: true,
            open_calls: [(1, BTreeSet::new())].into_iter().collect(),
            ..Default::default()
        };
        let calls =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert_eq!(calls.rust.matches("::NativeFunctionSignature for").count(), 2);
        assert_eq!(calls.rust.matches("::Call<").count(), 1);
        assert_eq!(calls.required_types, [integer()]);
    }

    /// Check exact raw-input demand emits only the uniquely justified storage bridge.
    #[test]
    fn one_native_input_identity_retains_only_its_concrete_storage_bridge() {
        let (mut declarations, mut bindings, target) = fixture();
        declarations.types.insert("Alias".into(), declarations.types["One"].clone());
        bindings.types.insert("Alias".into(), bindings.types["One"].clone());
        let requests = CallbackRequests {
            native_input_types: [declarations.types["One"].canonical_spelling.clone()].into(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert_eq!(output.rust.matches("::NativeFunctionSignature for").count(), 1);
        assert_eq!(output.rust.matches("::NativeType for").count(), 1);
        assert_eq!(output.rust.matches("::IntoExpression for").count(), 1);
        assert!(output.rust.contains("::FunctionValue::new(self)"));
        assert!(!output.rust.contains("::Call<"));
        assert!(!output.rust.contains("transmute"));
        assert!(output.required_types.is_empty());
    }

    /// Keep explicit typedef constructors when unrelated evidence prevents global raw-input inference.
    #[test]
    fn explicit_typedefs_do_not_require_catalog_wide_native_uniqueness() {
        let (mut declarations, mut bindings, target) = fixture();
        declarations.types.insert("Alias".into(), declarations.types["One"].clone());
        bindings.types.insert(
            "Alias".into(),
            AliasBinding {
                path: vec!["nested".into(), "r#type".into()],
                target: RustBindingType::Named { path: vec!["One".into()] },
            },
        );
        bindings.types.insert(
            "Unresolved".into(),
            AliasBinding {
                path: vec!["Unresolved".into()],
                target: RustBindingType::Named { path: vec!["MissingAlias".into()] },
            },
        );
        let requests = CallbackRequests {
            native_input_types: [declarations.types["One"].canonical_spelling.clone()].into(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert!(output.has_typedefs);
        assert!(output.rust.contains("pub type One ="));
        assert!(output.rust.contains("pub mod nested {"));
        assert!(output.rust.contains("pub type r#type ="));
        assert!(!output.rust.contains("pub type Two ="));
        assert!(!output.rust.contains("pub type Unresolved ="));
        assert!(!output.rust.contains("::NativeType for"));
        assert!(!output.rust.contains("::IntoExpression for"));
        assert!(!output.rust.contains("transmute"));
    }

    /// An explicit typedef cannot admit an incompatible or unproven callback ABI.
    #[test]
    fn rejected_prototypes_have_no_explicit_typedef_constructor() {
        let (declarations, mut bindings, target) = fixture();
        let RustBindingType::Option { value } = &mut bindings.types.get_mut("One").unwrap().target
        else {
            unreachable!()
        };
        let RustBindingType::Function { abi, .. } = value.as_mut() else { unreachable!() };
        *abi = "Rust".into();
        let requests = CallbackRequests {
            native_input_types: [declarations.types["One"].canonical_spelling.clone()].into(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert!(output.unsupported.contains_key(&declarations.types["One"].canonical_spelling));
        assert!(!output.has_typedefs);
        assert!(!output.rust.contains("pub type One ="));
    }

    /// Construct equal-width long and long long callback types with distinct C arithmetic ranks.
    fn rank_fixture() -> (DeclarationCatalog, BindingCatalog, TargetFacts, TypeInfo, TypeInfo) {
        let (mut declarations, mut bindings, mut target) = fixture();
        let [long, wide] =
            [("long", IntegerKind::Long, 4), ("long long", IntegerKind::LongLong, 5)].map(
                |(name, kind, rank)| {
                    target
                        .integers
                        .insert(kind, IntegerType { kind, bits: 64, signed: true, rank });
                    add_callback(
                        &mut declarations,
                        &mut bindings,
                        name,
                        0,
                        TypeInfo {
                            spelling: name.into(),
                            canonical_spelling: name.into(),
                            category: TypeCategory::Integer(kind),
                            size: Some(8),
                            alignment: Some(8),
                            ..integer()
                        },
                        RustBindingType::Integer { signed: true, bits: 64 },
                    )
                },
            );
        (declarations, bindings, target, long, wide)
    }

    /// Prove an unselected equal-storage prototype still prevents inferring a unique raw callback identity.
    #[test]
    fn unselected_equal_storage_c_ranks_keep_native_inputs_tagged() {
        let (declarations, bindings, target, selected, other) = rank_fixture();
        let requests = CallbackRequests {
            native_input_types: [selected.canonical_spelling.clone()].into(),
            call_types: [selected.canonical_spelling.clone()].into(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert!(output.unsupported.is_empty());
        assert_eq!(output.rust.matches("::NativeFunctionSignature for").count(), 1);
        assert_eq!(output.rust.matches("::Call<").count(), 1);
        assert!(!output.rust.contains("::NativeType for"));
        assert!(!output.rust.contains("::IntoExpression for"));
        assert_ne!(
            output.markers[&selected.canonical_spelling].marker,
            output.markers[&other.canonical_spelling].marker,
        );
    }

    /// Prove rejected callback evidence cannot be discarded to manufacture a raw-input uniqueness proof.
    #[test]
    fn rejected_equal_storage_witness_cannot_prove_native_uniqueness() {
        let (mut declarations, bindings, target, selected, other) = rank_fixture();
        let TypeShapeKind::Pointer { pointee } =
            &declarations.type_shapes[&other.canonical_spelling].kind
        else {
            unreachable!()
        };
        let function = pointee.canonical_spelling.clone();
        let TypeShapeKind::Function { signature } =
            &mut declarations.type_shapes.get_mut(&function).unwrap().kind
        else {
            unreachable!()
        };
        signature.calling_convention = None;
        let requests = CallbackRequests {
            native_input_types: [selected.canonical_spelling.clone()].into(),
            call_types: [selected.canonical_spelling].into(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert!(output.unsupported.contains_key(&other.canonical_spelling));
        assert_eq!(output.rust.matches("::Call<").count(), 1);
        assert!(!output.rust.contains("::NativeType for"));
        assert!(!output.rust.contains("::IntoExpression for"));
    }

    /// Check missing C declarations preserve tagged callback requirements for equal native storage.
    #[test]
    fn missing_c_witnesses_keep_matching_native_storage_tagged() {
        for missing_declaration in [false, true] {
            let (mut declarations, mut bindings, target) = fixture();
            bindings.types.insert("Unknown".into(), bindings.types["One"].clone());
            if !missing_declaration {
                let mut unknown = declarations.types["One"].clone();
                unknown.canonical_spelling = "missing C callback shape".into();
                declarations.types.insert("Unknown".into(), unknown);
            }
            let requests = CallbackRequests {
                native_input_types: [declarations.types["One"].canonical_spelling.clone()].into(),
                call_types: [declarations.types["One"].canonical_spelling.clone()].into(),
                ..Default::default()
            };
            let output =
                generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
            assert_eq!(output.rust.matches("::Call<").count(), 1);
            assert!(!output.rust.contains("::NativeType for"));
            assert!(!output.rust.contains("::IntoExpression for"));
        }
    }

    /// Check unresolved binding aliases block raw-input inference instead of hiding possible callbacks.
    #[test]
    fn a_missing_rust_alias_cannot_prove_native_uniqueness() {
        let (declarations, mut bindings, target, selected, _) = rank_fixture();
        bindings.types.get_mut("long long").unwrap().target =
            RustBindingType::Named { path: vec!["MissingAlias".into()] };
        let requests = CallbackRequests {
            native_input_types: [selected.canonical_spelling.clone()].into(),
            call_types: [selected.canonical_spelling.clone()].into(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert!(output.markers.contains_key(&selected.canonical_spelling));
        assert_eq!(output.rust.matches("::NativeFunctionSignature for").count(), 1);
        assert_eq!(output.rust.matches("::Call<").count(), 1);
        assert!(!output.rust.contains("::NativeType for"));
        assert!(!output.rust.contains("::IntoExpression for"));
    }

    /// Check a function binding without a matching C witness prevents unjustified native uniqueness.
    #[test]
    fn an_unmatched_function_item_blocks_matching_native_storage() {
        let (declarations, mut bindings, target) = fixture();
        bindings.functions.insert(
            "unknown".into(),
            crate::FunctionBinding {
                path: vec!["unknown".into()],
                parameters: vec![storage()],
                result: storage(),
                abi: "C-unwind".into(),
                variadic: false,
                link_name: "unknown".into(),
                guarded: false,
                uses_cshim: false,
            },
        );
        let requests = CallbackRequests {
            native_input_types: [declarations.types["One"].canonical_spelling.clone()].into(),
            call_types: [declarations.types["One"].canonical_spelling.clone()].into(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert_eq!(output.rust.matches("::Call<").count(), 1);
        assert!(!output.rust.contains("::NativeType for"));
        assert!(!output.rust.contains("::IntoExpression for"));
    }

    /// Check bounded alias-walk failure remains unresolved evidence rather than proving uniqueness.
    #[test]
    fn a_bounded_unknown_storage_failure_cannot_prove_native_uniqueness() {
        let (declarations, mut bindings, target) = fixture();
        let mut unknown = storage();
        for _ in 0..=DEPTH_LIMIT {
            unknown = RustBindingType::Pointer { pointee: Box::new(unknown), mutable: true };
        }
        bindings.types.insert(
            "DeepUnknown".into(),
            crate::AliasBinding { path: vec!["DeepUnknown".into()], target: unknown },
        );
        let requests = CallbackRequests {
            native_input_types: [declarations.types["One"].canonical_spelling.clone()].into(),
            call_types: [declarations.types["One"].canonical_spelling.clone()].into(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert_eq!(output.rust.matches("::Call<").count(), 1);
        assert!(!output.rust.contains("::NativeType for"));
        assert!(!output.rust.contains("::IntoExpression for"));
    }

    /// Check unrenderable callback storage still prevents a competing raw-input identity bridge.
    #[test]
    fn an_unrenderable_native_witness_cannot_prove_uniqueness() {
        let (declarations, mut bindings, target) = fixture();
        let mut unknown = bindings.types["One"].clone();
        let RustBindingType::Option { value } = &mut unknown.target else { unreachable!() };
        let RustBindingType::Function { parameters, .. } = value.as_mut() else { unreachable!() };
        parameters[0] = RustBindingType::Named { path: vec!["<unknown Rust storage>".into()] };
        bindings.types.insert("Unknown".into(), unknown);
        let requests = CallbackRequests {
            native_input_types: [declarations.types["One"].canonical_spelling.clone()].into(),
            call_types: [declarations.types["One"].canonical_spelling.clone()].into(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert_eq!(output.rust.matches("::Call<").count(), 1);
        assert!(!output.rust.contains("::NativeType for"));
        assert!(!output.rust.contains("::IntoExpression for"));
    }

    /// Check mismatched outer signatures still expose callback types nested in binding storage.
    #[test]
    fn incompatible_outer_function_storage_does_not_hide_nested_witnesses() {
        for (abi, variadic, nested) in
            [("C-unwind", true, false), ("Rust", false, false), ("Rust", false, true)]
        {
            let (declarations, mut bindings, target) = fixture();
            let parameter = if nested { bindings.types["One"].target.clone() } else { storage() };
            bindings.functions.insert(
                "unknown".into(),
                crate::FunctionBinding {
                    path: vec!["unknown".into()],
                    parameters: vec![parameter],
                    result: storage(),
                    abi: abi.into(),
                    variadic,
                    link_name: "unknown".into(),
                    guarded: false,
                    uses_cshim: false,
                },
            );
            let requests = CallbackRequests {
                native_input_types: [declarations.types["One"].canonical_spelling.clone()].into(),
                ..Default::default()
            };
            let output =
                generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
            assert_eq!(output.rust.matches("::NativeType for").count(), usize::from(!nested));
            assert_eq!(output.rust.matches("::IntoExpression for").count(), usize::from(!nested));
        }
    }

    /// Check open-call exclusions prune bodies only and cannot override an exact requested prototype.
    #[test]
    fn open_exclusions_keep_metadata_and_yield_to_exact_calls() {
        let (declarations, bindings, target) = fixture();
        let pointer = &declarations.types["One"];
        let TypeShapeKind::Pointer { pointee } =
            &declarations.type_shapes[&pointer.canonical_spelling].kind
        else {
            unreachable!()
        };
        let mut requests = CallbackRequests {
            open_calls: [(1, [pointee.canonical_spelling.clone()].into())].into(),
            ..Default::default()
        };
        let excluded =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert!(excluded.rust.is_empty());
        assert_eq!(excluded.markers.len(), 4, "pruning does not erase compiler metadata");
        requests.call_types.insert(pointer.canonical_spelling.clone());
        let exact =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert_eq!(exact.rust.matches("::Call<").count(), 1);
        assert_eq!(exact.markers.len(), excluded.markers.len());
    }

    /// Check shared physical pointer mechanics retain distinct nominal C prototype markers.
    #[test]
    fn physical_bodies_are_shared_without_merging_equal_storage_c_prototypes() {
        let (declarations, bindings, target, _, _) = rank_fixture();
        let requests = CallbackRequests { open_identity: true, ..Default::default() };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert!(output.unsupported.is_empty());
        assert_eq!(output.rust.matches("::NativeFunctionSignature for").count(), 4);
        assert_eq!(output.rust.matches("::PhysicalFunctionPointer for").count(), 3);
        assert_eq!(output.rust.matches("fn address(pointer:").count(), 3);
        assert_eq!(output.rust.matches("fn null()").count(), 3);
        assert!(!output.rust.contains("::Call<"));
        assert_ne!(
            output.markers[&declarations.types["long"].canonical_spelling].marker,
            output.markers[&declarations.types["long long"].canonical_spelling].marker,
        );
        assert_eq!(
            output.markers[&declarations.types["long"].canonical_spelling].storage,
            output.markers[&declarations.types["long long"].canonical_spelling].storage,
        );
        assert_eq!(output.rust.matches("type Physical = PhysicalFunction_C_unwind_0<").count(), 2,);
        assert!(output.required_types.is_empty());
    }

    /// Check physical pointer support is shared only within the same ABI and argument count.
    #[test]
    fn physical_families_keep_abi_and_arity_distinct() {
        let mut rust = String::new();
        let mut emitted = BTreeSet::new();
        assert_ne!(
            physical_family(&mut rust, "C", 1, &mut emitted),
            physical_family(&mut rust, "C-unwind", 1, &mut emitted),
        );
        let large = physical_family(&mut rust, "C", 40, &mut emitted);
        assert_ne!(large, physical_family(&mut rust, "C", 1, &mut emitted));
        assert_eq!(physical_family(&mut rust, "C", 40, &mut emitted), large);
        assert_eq!(rust.matches("::PhysicalFunctionPointer for").count(), 3);
        assert!(rust.contains("fn(A0, A1"));
        assert!(rust.contains("A39) -> R"));
    }

    /// Check selected calls retain nested prototype identities without unnecessarily adding nested call bodies.
    #[test]
    fn selected_calls_retain_nested_callback_identities_without_nested_calls() {
        let (mut declarations, mut bindings, target) = fixture();
        let nested = declarations.types["One"].clone();
        let nested_storage = bindings.types["One"].target.clone();
        let factory =
            add_callback(&mut declarations, &mut bindings, "Factory", 0, nested, nested_storage);
        let requests = CallbackRequests {
            call_types: [factory.canonical_spelling].into_iter().collect(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert_eq!(output.rust.matches("::NativeFunctionSignature for").count(), 2);
        assert_eq!(output.rust.matches("::Call<").count(), 1);
        assert!(output.rust.contains("::Call<()>"));
        assert_eq!(output.required_types, [declarations.types["One"].clone()]);
    }

    /// Check a noncall callback shell does not pull in its prototype’s semantic conversion dependencies.
    #[test]
    fn identity_only_callback_does_not_require_nested_signature_bridges() {
        let (mut declarations, mut bindings, target) = fixture();
        let nested = declarations.types["One"].clone();
        let nested_storage = bindings.types["One"].target.clone();
        let factory =
            add_callback(&mut declarations, &mut bindings, "Factory", 0, nested, nested_storage);
        let requests = CallbackRequests {
            identity_types: [factory.canonical_spelling].into_iter().collect(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert_eq!(output.rust.matches("::NativeFunctionSignature for").count(), 1);
        assert!(!output.rust.contains("::Call<"));
        assert!(output.required_types.is_empty());
        assert_eq!(output.markers.len(), 6, "validation metadata remains complete");
    }

    /// Check call dependency closure excludes nested types of unrelated identity-only callbacks.
    #[test]
    fn selected_calls_do_not_retain_identity_only_prototype_dependencies() {
        let (mut declarations, mut bindings, target) = fixture();
        let nested = declarations.types["One"].clone();
        let nested_storage = bindings.types["One"].target.clone();
        let factory =
            add_callback(&mut declarations, &mut bindings, "Factory", 0, nested, nested_storage);
        let factory_storage = bindings.types["Factory"].target.clone();
        let outer = add_callback(
            &mut declarations,
            &mut bindings,
            "Outer",
            0,
            factory.clone(),
            factory_storage,
        );
        let requests = CallbackRequests {
            call_types: [outer.canonical_spelling].into_iter().collect(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert_eq!(output.rust.matches("::NativeFunctionSignature for").count(), 2);
        assert_eq!(output.rust.matches("::Call<").count(), 1);
        assert_eq!(output.required_types, [factory]);
        assert_eq!(output.markers.len(), 8, "validation metadata remains complete");
        let nested = &output.markers[&declarations.types["One"].canonical_spelling].marker;
        let nested_name = nested.rsplit("::").next().unwrap();
        assert!(
            !output.rust.contains(&format!("pub struct {nested_name};")),
            "the identity-only factory embeds the nested callback as raw ABI storage"
        );
    }

    /// Prove an unselected conflicting ABI witness invalidates a selected callback identity.
    #[test]
    fn alternate_unselected_abi_witness_still_invalidates_selected_callback() {
        let (mut declarations, mut bindings, target) = fixture();
        declarations.types.insert("Alternate".into(), declarations.types["One"].clone());
        let mut alternate = bindings.types["One"].clone();
        let RustBindingType::Option { value } = &mut alternate.target else { unreachable!() };
        let RustBindingType::Function { abi, .. } = value.as_mut() else { unreachable!() };
        *abi = "C".into();
        bindings.types.insert("Alternate".into(), alternate);
        let requests = CallbackRequests {
            call_types: [declarations.types["One"].canonical_spelling.clone()]
                .into_iter()
                .collect(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &requests).unwrap();
        assert!(output.rust.is_empty());
        assert!(
            output.unsupported[&declarations.types["One"].canonical_spelling]
                .contains("incompatible actual Rust storage witnesses")
        );
        assert!(!output.markers.contains_key(&declarations.types["One"].canonical_spelling));
        let TypeShapeKind::Pointer { pointee } =
            &declarations.type_shapes[&declarations.types["One"].canonical_spelling].kind
        else {
            unreachable!()
        };
        let excluded = CallbackRequests {
            open_calls: [(1, [pointee.canonical_spelling.clone()].into())].into(),
            ..Default::default()
        };
        let output =
            generate(&declarations, &bindings, &target, &BTreeSet::new(), &excluded).unwrap();
        assert!(output.rust.is_empty());
        assert!(output.unsupported.contains_key(&declarations.types["One"].canonical_spelling));
        assert!(!output.markers.contains_key(&declarations.types["One"].canonical_spelling));
    }

    /// Check nested prototype validation failures propagate through the catalog before demand pruning.
    #[test]
    fn nested_callback_failure_propagates_even_without_emission_requests() {
        let (mut declarations, mut bindings, target) = fixture();
        let nested = declarations.types["One"].clone();
        let TypeShapeKind::Pointer { pointee } =
            &declarations.type_shapes[&nested.canonical_spelling].kind
        else {
            unreachable!()
        };
        let function = pointee.canonical_spelling.clone();
        let TypeShapeKind::Function { signature } =
            &mut declarations.type_shapes.get_mut(&function).unwrap().kind
        else {
            unreachable!()
        };
        signature.variadic = true;
        let nested_storage = bindings.types["One"].target.clone();
        let factory =
            add_callback(&mut declarations, &mut bindings, "Factory", 0, nested, nested_storage);
        let output = generate(
            &declarations,
            &bindings,
            &target,
            &BTreeSet::new(),
            &CallbackRequests::default(),
        )
        .unwrap();
        assert!(output.rust.is_empty());
        assert!(
            output.unsupported[&factory.canonical_spelling]
                .contains("depends on unsupported signature")
        );
        assert!(!output.markers.contains_key(&factory.canonical_spelling));
    }
}
