//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exact native callback signatures recovered from matching C and binding edges.

use super::types::{Lowering, rust_path};
use crate::{
    BindingCatalog, CallbackBinding, DeclarationCatalog, FunctionSignature, RustBindingType,
    TargetFacts, TypeCategory, TypeInfo, TypeShapeKind,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Write;

const EXPRESSION: &str = "crate::__pgrx_c_macros::expression";
const SOURCE_LIMIT: usize = 16 * 1024 * 1024;
const DEPTH_LIMIT: usize = 64;

pub(super) struct CallbackAdapters {
    pub rust: String,
    pub markers: BTreeMap<String, CallbackBinding>,
    pub unsupported: BTreeMap<String, String>,
}

struct Candidate {
    pointer: TypeInfo,
    function: TypeInfo,
    signature: FunctionSignature,
    storage: RustBindingType,
}

pub(super) fn generate(
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    target: &TargetFacts,
    used_functions: &BTreeSet<String>,
) -> Result<CallbackAdapters, String> {
    let mut candidates = BTreeMap::new();
    let mut unsupported = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for (name, alias) in &bindings.types {
        if let Some(ty) = declarations.types.get(name) {
            collect(
                ty,
                &alias.target,
                declarations,
                bindings,
                &mut candidates,
                &mut unsupported,
                &mut seen,
                0,
            );
        }
    }
    for (name, variable) in &bindings.variables {
        if let Some(ty) = declarations.variables.get(name) {
            collect(
                ty,
                &variable.ty,
                declarations,
                bindings,
                &mut candidates,
                &mut unsupported,
                &mut seen,
                0,
            );
        }
    }
    let witnessed_fields = Lowering::new(declarations, bindings, target);
    for (canonical, record) in &declarations.records {
        for field in &record.fields {
            if let Some(binding) = field
                .name
                .as_ref()
                .and_then(|name| witnessed_fields.named_field_binding(canonical, name))
            {
                collect(
                    &field.ty,
                    &binding.ty,
                    declarations,
                    bindings,
                    &mut candidates,
                    &mut unsupported,
                    &mut seen,
                    0,
                );
            }
        }
    }
    for (name, binding) in &bindings.functions {
        let Some(function) = declarations.function_signatures.get(name) else { continue };
        for (ty, storage) in function.signature.parameters.iter().flatten().zip(&binding.parameters)
        {
            collect(
                ty,
                storage,
                declarations,
                bindings,
                &mut candidates,
                &mut unsupported,
                &mut seen,
                0,
            );
        }
        collect(
            &function.signature.result,
            &binding.result,
            declarations,
            bindings,
            &mut candidates,
            &mut unsupported,
            &mut seen,
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
            ty,
            &RustBindingType::Option {
                value: Box::new(RustBindingType::Function {
                    parameters: binding.parameters.clone(),
                    result: Box::new(binding.result.clone()),
                    abi: binding.abi.clone(),
                    unsafe_: true,
                    variadic: binding.variadic,
                }),
            },
            declarations,
            bindings,
            &mut candidates,
            &mut unsupported,
            &mut seen,
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
        hash.update(serde_json::to_vec(target).map_err(|error| error.to_string())?);
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
    let lowering = Lowering::new(declarations, &capabilities, target);
    let mut generated = BTreeMap::new();
    let mut reverse = BTreeMap::<String, BTreeSet<String>>::new();
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
        for dependency in dependencies {
            reverse.entry(dependency).or_default().insert(key.clone());
        }
        if !unsupported.contains_key(key) {
            match adapter(candidate, &marker_names[key], bindings, &lowering, target) {
                Ok(rust) => {
                    generated.insert(key.clone(), rust);
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
    let mut output =
        CallbackAdapters { rust: String::new(), markers: BTreeMap::new(), unsupported };
    let mut emitted = BTreeSet::new();
    for (key, rust) in generated {
        if output.unsupported.contains_key(&key) {
            continue;
        }
        let candidate = &candidates[&key];
        let binding = capabilities.callback_capabilities[&key].clone();
        output.markers.insert(key, binding.clone());
        output.markers.insert(candidate.function.canonical_spelling.clone(), binding);
        if emitted.insert(marker_names[&candidate.pointer.canonical_spelling].clone()) {
            output.rust.push_str(&rust);
        }
        if output.rust.len() > SOURCE_LIMIT {
            return Err("generated callback adapters exceed the 16 MiB source budget".into());
        }
    }
    Ok(output)
}

#[allow(clippy::too_many_arguments)] // One bounded walk owns matching C/Rust type edges.
fn collect(
    ty: &TypeInfo,
    storage: &RustBindingType,
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    candidates: &mut BTreeMap<String, Candidate>,
    unsupported: &mut BTreeMap<String, String>,
    seen: &mut BTreeSet<(String, String)>,
    depth: usize,
) {
    if depth > DEPTH_LIMIT {
        unsupported.insert(
            ty.canonical_spelling.clone(),
            "callback C/Rust type walk exceeds its bounded depth".into(),
        );
        return;
    }
    let storage = match normalize_storage(storage, bindings, 0) {
        Ok(storage) => storage,
        Err(reason) => {
            unsupported.insert(ty.canonical_spelling.clone(), reason);
            return;
        }
    };
    let fingerprint = serde_json::to_string(&storage).expect("binding types are serializable");
    if !seen.insert((ty.canonical_spelling.clone(), fingerprint)) {
        return;
    }
    let Some(shape) = declarations.type_shapes.get(&ty.canonical_spelling) else { return };
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
                seen,
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
                seen,
                depth,
            );
        }
        (TypeShapeKind::Pointer { pointee }, RustBindingType::Pointer { pointee: actual, .. }) => {
            collect(
                pointee,
                actual,
                declarations,
                bindings,
                candidates,
                unsupported,
                seen,
                depth + 1,
            );
        }
        (TypeShapeKind::Array { element, .. }, RustBindingType::Array { element: actual, .. }) => {
            collect(
                element,
                actual,
                declarations,
                bindings,
                candidates,
                unsupported,
                seen,
                depth + 1,
            );
        }
        _ => {}
    }
}

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
    seen: &mut BTreeSet<(String, String)>,
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
        for (ty, storage) in signature.parameters.iter().flatten().zip(parameters) {
            collect(ty, storage, declarations, bindings, candidates, unsupported, seen, depth + 1);
        }
        collect(
            &signature.result,
            result,
            declarations,
            bindings,
            candidates,
            unsupported,
            seen,
            depth + 1,
        );
    }
}

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

fn adapter(
    candidate: &Candidate,
    name: &str,
    bindings: &BindingCatalog,
    lowering: &Lowering<'_>,
    target: &TargetFacts,
) -> Result<String, String> {
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
    let mut rust = String::new();
    let storage = lowering.storage_type(&candidate.storage, 0)?.replace("$crate", "crate");
    let native_storage =
        |ty: &TypeInfo, actual: &RustBindingType| -> Result<RustBindingType, String> {
            value_type(ty)?;
            if ty.category == TypeCategory::Enum {
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
    abi_assertions(&mut rust, &storage, &candidate.pointer);
    for (ty, lowered) in parameters.iter().zip(&lowered) {
        abi_assertions(&mut rust, &lowered.storage.replace("$crate", "crate"), ty);
    }
    if let Some(result) = &lowered_result {
        abi_assertions(&mut rust, &result.storage.replace("$crate", "crate"), &signature.result);
    }
    writeln!(rust, "#[doc(hidden)] #[derive(Clone, Copy)] pub struct {name};\nimpl crate::__pgrx_c_macros::sealed::Sealed for {name} {{}}\nimpl {EXPRESSION}::FunctionSignature for {name} {{\n type Pointer = {storage};\n fn null() -> Self::Pointer {{ None }}\n fn address(pointer: Self::Pointer) -> *const () {{ pointer.map_or(::core::ptr::null(), |function| function as *const ()) }}\n}}").expect("String output");
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
    writeln!(rust, "impl{generic} {EXPRESSION}::Call<({args})> for {name} {{\n type Output = {output};\n unsafe fn call(pointer: Self::Pointer, {argument_name}: ({args})) -> Self::Output {{").expect("String output");
    writeln!(
        rust,
        "let function = pointer.expect(\"C indirect call requires a non-null function pointer\");"
    )
    .expect("String output");
    if &parameter_storage != actual || &result_storage != result.as_ref() {
        let original = lowering.storage_type(value, 0)?.replace("$crate", "crate");
        let raw = lowering
            .storage_type(
                &RustBindingType::Function {
                    parameters: parameter_storage.clone(),
                    result: Box::new(result_storage.clone()),
                    abi: abi.clone(),
                    unsafe_: *unsafe_,
                    variadic: false,
                },
                0,
            )?
            .replace("$crate", "crate");
        writeln!(rust, "// SAFETY: Only by-value records and enums change representation. MaybeUninit<R> has R's guaranteed layout/ABI; each enum uses Clang's compatible integer type, validated against the actual Rust enum storage and the layout checks above. The calling convention is unchanged. No partially initialized record or unnamed Rust enum variant is materialized.\nlet function: {raw} = unsafe {{ ::core::mem::transmute::<{original}, {raw}>(function) }};").expect("String output");
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

fn value_type(ty: &TypeInfo) -> Result<(), String> {
    if matches!(ty.category, TypeCategory::Void | TypeCategory::Function | TypeCategory::Other) {
        return Err("callback value has no established scalar or pointer ABI representation".into());
    }
    Ok(())
}

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
