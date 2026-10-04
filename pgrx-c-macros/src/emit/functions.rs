//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exact native ABI primitives for callable C declarations.
//!
//! Generated C thunks cover callable static inline declarations and by-value objects
//! that cannot safely use a binding directly. Exact prototype and layout checks
//! anchor both sides of the ABI; Rust argument conversions stay outside the guarded
//! native call so PostgreSQL error jumps cannot cross conversion-owned destructors.

use super::types::{Lowering, rust_path};
use crate::{
    BindingCatalog, DeclarationCatalog, DeclarationLinkage, FunctionBinding, FunctionInfo,
    IntegerKind, RustBindingType, TargetFacts, TypeCategory, TypeInfo, TypeShapeKind,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

/// Bound the combined Rust/C native thunk source emitted by this capability family.
const OUTPUT_LIMIT: usize = 16 * 1024 * 1024;

/// Native call replacements and exact binding signatures for requested C declarations.
pub(super) struct FunctionAdapters {
    /// Rust ABI declarations and semantic argument/result adapters around native thunks.
    pub rust: String,
    /// Same-profile C definitions that call the original declaration or transport by-value objects.
    pub c_source: String,
    /// Macro-facing binding replacements indexed by the original C function name.
    pub bindings: BTreeMap<String, FunctionBinding>,
    /// Linkage and prototype failures retained for dependent lowering diagnostics.
    pub unsupported: BTreeMap<String, String>,
}

/// Emit required thunks for static inline declarations and ABI-sensitive value transport.
///
/// A profile salt includes record layouts so otherwise-identical signatures cannot
/// reuse a native symbol across different PostgreSQL compilation profiles.
pub(super) fn generate(
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    used_functions: &BTreeSet<String>,
    called_functions: &BTreeSet<String>,
    target: &TargetFacts,
    profile_identity: &str,
) -> Result<FunctionAdapters, String> {
    let lowering = Lowering::new_native(declarations, bindings, target);
    let mut result = FunctionAdapters {
        rust: String::new(),
        c_source: String::new(),
        bindings: BTreeMap::new(),
        unsupported: BTreeMap::new(),
    };
    // Include every compiler-owned layout once in the profile salt. A helper
    // signature can contain nested pointers whose pointee layout differs across
    // PostgreSQL profiles even when the immediate prototype is identical.
    let layouts = serde_json::to_vec(&declarations.records)
        .map_err(|error| format!("cannot fingerprint inline record layouts: {error}"))?;
    let mut profile_hash = Sha256::new();
    profile_hash.update(profile_identity);
    profile_hash.update(super::semantic_target_bytes(target)?);
    profile_hash.update(layouts);
    let profile_hash = profile_hash.finalize();
    for name in used_functions {
        let Some(function) = declarations.function_signatures.get(name) else {
            continue;
        };
        // A char alias's byte layout cannot prove its callable extension
        // attributes. Let the original C compiler own those calls as it owns
        // record and enum transport, instead of invoking bindgen's Rust wrapper.
        let native_transport = matches!(
            function.signature.result.category,
            TypeCategory::Record | TypeCategory::Enum | TypeCategory::Integer(IntegerKind::Char)
        ) || function.signature.parameters.iter().flatten().any(|ty| {
            matches!(
                ty.category,
                TypeCategory::Record
                    | TypeCategory::Enum
                    | TypeCategory::Integer(IntegerKind::Char)
            )
        });
        if bindings.functions.contains_key(name) && !native_transport {
            continue;
        }
        match adapter(
            name,
            function,
            declarations,
            bindings,
            &lowering,
            target,
            &profile_hash,
            called_functions.contains(name),
        ) {
            Ok((rust, c_source, binding)) => {
                result.rust.push_str(&rust);
                result.c_source.push_str(&c_source);
                result.bindings.insert(name.clone(), binding);
                if result.rust.len() + result.c_source.len() > OUTPUT_LIMIT {
                    return Err("generated inline adapters exceed the source budget".into());
                }
            }
            Err(reason) => {
                result.unsupported.insert(name.clone(), reason);
            }
        }
    }
    Ok(result)
}

/// Validate one declaration and produce matched C/Rust ABI code plus its binding replacement.
///
/// Availability, convention, object validity, and destructor restrictions are checked
/// before emitting call code; C owns the original function invocation.
#[allow(clippy::too_many_arguments)] // One ABI proof owns declaration, storage, profile and demand.
fn adapter(
    name: &str,
    function: &FunctionInfo,
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    lowering: &Lowering<'_>,
    target: &TargetFacts,
    profile_hash: &[u8],
    emit_call: bool,
) -> Result<(String, String, FunctionBinding), String> {
    if !c_identifier(name) {
        return Err("C function has no usable identifier".into());
    }
    if function.linkage == Some(DeclarationLinkage::Internal)
        && (!function.is_static || !function.is_inline || !function.definition_available)
    {
        return Err("internal function is not a compiler-proven static inline definition".into());
    } else if !matches!(
        function.linkage,
        Some(
            DeclarationLinkage::Internal
                | DeclarationLinkage::External
                | DeclarationLinkage::UniqueExternal
        )
    ) {
        return Err("function has no verified native linkage".into());
    }
    let signature = &function.signature;
    let parameters = signature.parameters.as_ref().ok_or("inline function has no C prototype")?;
    if signature.variadic {
        return Err("variadic inline adapters require a separate ABI capability".into());
    }
    if signature.calling_convention.as_deref() != Some("Cdecl") {
        return Err(
            "inline function calling convention is not the verified default C convention".into()
        );
    }
    let existing = bindings.functions.get(name);
    if let Some(existing) = existing
        && (existing.parameters.len() != parameters.len()
            || existing.variadic
            || !matches!(existing.abi.as_str(), "C" | "C-unwind"))
    {
        return Err("actual binding differs from the fixed C function prototype".into());
    }
    let value_storage =
        |ty: &TypeInfo, actual: Option<&RustBindingType>| -> Result<RustBindingType, String> {
            // C enums admit values outside a rustified enum's named variants. The
            // native primitive uses the compiler-proven compatible integer instead
            // of creating or encoding an invalid Rust enum at an ABI boundary.
            // Char's binding bytes can be compatible despite different C/Rust
            // signs. Callable extension attributes require the compiler's C
            // sign, so the original-C thunk transports canonical scalar storage.
            if ty.category == TypeCategory::Integer(IntegerKind::Char) {
                if let Some(actual) = actual {
                    lowering.resolve_with_storage(ty, actual)?;
                }
                let integer = target
                    .integers
                    .get(&IntegerKind::Char)
                    .ok_or("plain-char identity is absent from the target profile")?;
                return Ok(RustBindingType::Integer { signed: integer.signed, bits: integer.bits });
            }
            let storage = match actual {
                Some(actual) if ty.category != TypeCategory::Enum => actual.clone(),
                _ => abi_storage(ty, declarations, bindings, target, lowering, 0)?,
            };
            if ty.category == TypeCategory::Record {
                let storage = if matches!(storage, RustBindingType::MaybeUninit { .. }) {
                    storage
                } else {
                    RustBindingType::MaybeUninit { value: Box::new(storage) }
                };
                lowering.resolve_with_storage(ty, &storage)?;
                Ok(storage)
            } else {
                lowering.resolve_with_storage(ty, &storage)?;
                Ok(storage)
            }
        };
    let parameter_storage = parameters
        .iter()
        .enumerate()
        .map(|(index, ty)| {
            value_shape(ty)?;
            value_storage(ty, existing.and_then(|binding| binding.parameters.get(index)))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let result_storage = if signature.result.category == TypeCategory::Void {
        RustBindingType::Unit
    } else {
        value_shape(&signature.result)?;
        value_storage(&signature.result, existing.map(|binding| &binding.result))?
    };
    let rust_parameters = parameters
        .iter()
        .zip(&parameter_storage)
        .map(|(ty, storage)| {
            lowering
                .resolve_with_storage(ty, storage)
                .map(|ty| ty.storage.replace("$crate", "crate"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let rust_result = if result_storage == RustBindingType::Unit {
        "()".into()
    } else {
        lowering
            .resolve_with_storage(&signature.result, &result_storage)?
            .storage
            .replace("$crate", "crate")
    };
    let mut hash = Sha256::new();
    hash.update(profile_hash);
    hash.update(name);
    hash.update(serde_json::to_vec(function).map_err(|error| error.to_string())?);
    hash.update(serde_json::to_vec(&parameter_storage).map_err(|error| error.to_string())?);
    hash.update(serde_json::to_vec(&result_storage).map_err(|error| error.to_string())?);
    let hash = hash.finalize();
    let suffix = hash[..16].iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let symbol = format!("__pgrx_inline_{suffix}");
    let wrapper = format!("Inline_{suffix}");
    let raw = format!("raw_inline_{suffix}");
    let binding = FunctionBinding {
        path: vec!["__pgrx_c_generated".into(), wrapper.clone()],
        parameters: parameter_storage,
        result: result_storage,
        abi: "C-unwind".into(),
        variadic: false,
        link_name: symbol.clone(),
        guarded: bindings.ffi_boundary.is_some(),
        uses_cshim: true,
    };
    let mut c_source = String::new();
    let mut rust = String::new();
    let mut c_parameters = Vec::new();
    let mut arguments = Vec::new();
    let mut c_arguments = Vec::new();
    let mut rust_signature = Vec::new();
    for (index, (parameter, storage)) in parameters.iter().zip(&rust_parameters).enumerate() {
        let alias = format!("{symbol}_arg{index}");
        let native = enum_abi_type(parameter, declarations)?;
        c_alias(&mut c_source, &alias, native, declarations)?;
        abi_assertions(&mut rust, storage, parameter);
        c_parameters.push(format!("{alias} arg{index}"));
        rust_signature.push(format!("arg{index}: {storage}"));
        if parameter.category == TypeCategory::Enum {
            let original = format!("{alias}_enum");
            c_alias(&mut c_source, &original, parameter, declarations)?;
            writeln!(c_source, "_Static_assert(__builtin_types_compatible_p({original}, {alias}), \"C enum compatible integer ABI\");").expect("String output");
            c_arguments.push(format!("({original})arg{index}"));
        } else {
            c_arguments.push(format!("arg{index}"));
        }
        arguments.push(format!("arg{index}"));
    }
    let c_result = if signature.result.category == TypeCategory::Void {
        "void".to_owned()
    } else {
        let alias = format!("{symbol}_result");
        c_alias(
            &mut c_source,
            &alias,
            enum_abi_type(&signature.result, declarations)?,
            declarations,
        )?;
        if signature.result.category == TypeCategory::Enum {
            let original = format!("{alias}_enum");
            c_alias(&mut c_source, &original, &signature.result, declarations)?;
            writeln!(c_source, "_Static_assert(__builtin_types_compatible_p({original}, {alias}), \"C enum compatible integer ABI\");").expect("String output");
        }
        abi_assertions(&mut rust, &rust_result, &signature.result);
        alias
    };
    if !emit_call {
        // Address getters need the complete native storage witness and ABI
        // assertions, but their callback shells contain no callable thunk.
        for storage in &rust_parameters {
            writeln!(rust, "const _: () = assert!(!::core::mem::needs_drop::<{storage}>());")
                .expect("String output");
        }
        return Ok((rust, c_source, binding));
    }
    let c_parameters =
        if c_parameters.is_empty() { "void".into() } else { c_parameters.join(", ") };
    let arguments = arguments.join(", ");
    let c_arguments = c_arguments.join(", ");
    writeln!(
        c_source,
        "{c_result} {symbol}({c_parameters}) {{ {}({name})({c_arguments}); }}",
        if signature.result.category == TypeCategory::Void { "" } else { "return " }
    )
    .expect("String output");
    let rust_signature = rust_signature.join(", ");
    // C-unwind permits existing callback unwinding behavior. PostgreSQL ERROR
    // itself is caught by the explicit boundary below, not Rust unwinding.
    writeln!(rust, "unsafe extern \"C-unwind\" {{ #[link_name = {symbol:?}] fn {raw}({rust_signature}) -> {rust_result}; }}").expect("String output");
    let guard_contract = if bindings.ffi_boundary.is_some() {
        " Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."
    } else {
        ""
    };
    let documentation = format!(
        "Call the original C function `{name}`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements.{guard_contract}"
    );
    writeln!(rust, "#[doc = {documentation:?}]\n#[inline]\npub unsafe fn {wrapper}({rust_signature}) -> {rust_result} {{").expect("String output");
    for storage in &rust_parameters {
        writeln!(rust, "const _: () = assert!(!::core::mem::needs_drop::<{storage}>());")
            .expect("String output");
    }
    let native_call = format!("{raw}({arguments})");
    if let Some(boundary) = &bindings.ffi_boundary {
        let boundary = rust_path(boundary)?.replace("$crate", "crate");
        writeln!(rust, "// SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.\nunsafe {{ {boundary}(move || {native_call}) }}").expect("String output");
    } else {
        writeln!(rust, "// SAFETY: The caller establishes the original C function's contract.\nunsafe {{ {native_call} }}").expect("String output");
    }
    rust.push_str("}\n");
    Ok((rust, c_source, binding))
}

/// Select Clang’s compatible integer only when it preserves the enum’s object layout.
fn enum_abi_type<'a>(
    ty: &'a TypeInfo,
    declarations: &'a DeclarationCatalog,
) -> Result<&'a TypeInfo, String> {
    if ty.category != TypeCategory::Enum {
        return Ok(ty);
    }
    let Some(crate::TypeShape {
        kind: TypeShapeKind::Enum { underlying: Some(underlying) }, ..
    }) = declarations.type_shapes.get(&ty.canonical_spelling)
    else {
        return Err("C enum has no compiler-established compatible integer ABI".into());
    };
    if underlying.size != ty.size
        || underlying.alignment != ty.alignment
        || !matches!(underlying.category, TypeCategory::Integer(_))
    {
        return Err("C enum compatible integer does not preserve its object layout".into());
    }
    Ok(underlying)
}

/// Reject unresolved, void, or function objects at nonvoid by-value prototype positions.
fn value_shape(ty: &TypeInfo) -> Result<(), String> {
    if matches!(ty.category, TypeCategory::Function | TypeCategory::Other | TypeCategory::Void) {
        return Err("inline value requires a proven scalar or pointer ABI representation".into());
    }
    Ok(())
}

/// Recover native storage recursively from compiler shapes and verified binding rewrites.
///
/// Binding-owned integer newtypes remain intact when their typedef identity is
/// proved; records and enums use the transport representation required by C.
fn abi_storage(
    ty: &TypeInfo,
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    target: &TargetFacts,
    lowering: &Lowering<'_>,
    depth: usize,
) -> Result<RustBindingType, String> {
    if depth > 64 {
        return Err("inline ABI type exceeds the bounded shape depth".into());
    }
    if let Some(crate::TypeShape {
        kind:
            TypeShapeKind::Array {
                element,
                length: Some(length),
                array_kind: crate::ArrayKind::Constant,
            },
        ..
    }) = declarations.type_shapes.get(&ty.canonical_spelling)
    {
        return Ok(RustBindingType::Array {
            element: Box::new(abi_storage(
                element,
                declarations,
                bindings,
                target,
                lowering,
                depth + 1,
            )?),
            length: *length,
        });
    }
    if let TypeCategory::Integer(kind) = ty.category {
        // A binding-owned representation can carry more than numeric bits (for
        // example exposed pointer provenance). Keep it whenever the compiler
        // names that typedef and the binding generator verified its bridge.
        let spelling = ty
            .spelling
            .split_whitespace()
            .filter(|word| !matches!(*word, "const" | "volatile" | "restrict"))
            .collect::<Vec<_>>()
            .join(" ");
        if c_identifier(&spelling) {
            let path = bindings
                .types
                .get(&spelling)
                .map_or_else(|| vec![spelling.clone()], |alias| alias.path.clone());
            let key = path
                .iter()
                .map(|part| part.strip_prefix("r#").unwrap_or(part))
                .collect::<Vec<_>>()
                .join("::");
            if bindings.integer_storage.get(&key) == Some(&kind) {
                let declared = declarations.types.get(&spelling).ok_or(
                    "registered inline integer storage has no compiler typedef declaration",
                )?;
                let unqualified = |spelling: &str| {
                    spelling
                        .split_whitespace()
                        .filter(|word| !matches!(*word, "const" | "volatile" | "restrict"))
                        .collect::<Vec<_>>()
                        .join(" ")
                };
                if unqualified(&declared.canonical_spelling) != unqualified(&ty.canonical_spelling)
                    || declared.category != ty.category
                    || declared.size != ty.size
                    || declared.alignment != ty.alignment
                {
                    return Err(
                        "registered inline integer typedef differs from the compiler prototype"
                            .into(),
                    );
                }
                return Ok(RustBindingType::Named { path });
            }
        }
    }
    Ok(match ty.category {
        TypeCategory::Integer(IntegerKind::Bool) => RustBindingType::Bool,
        TypeCategory::Integer(kind) => {
            let integer =
                target.integers.get(&kind).ok_or("inline integer has no target representation")?;
            if !matches!(integer.bits, 8 | 16 | 32 | 64) {
                return Err("inline integer width has no supported native Rust ABI storage".into());
            }
            RustBindingType::Integer { signed: integer.signed, bits: integer.bits }
        }
        TypeCategory::Floating => RustBindingType::Float {
            bits: match ty.canonical_spelling.as_str() {
                "float" => 32,
                "double" => 64,
                _ => return Err("inline extended float has no native Rust ABI storage".into()),
            },
        },
        TypeCategory::Void => {
            RustBindingType::Named { path: vec!["core".into(), "ffi".into(), "c_void".into()] }
        }
        TypeCategory::Pointer => {
            let Some(crate::TypeShape { kind: TypeShapeKind::Pointer { pointee }, .. }) =
                declarations.type_shapes.get(&ty.canonical_spelling)
            else {
                return Err("inline pointer has no compiler-owned pointee shape".into());
            };
            RustBindingType::Pointer {
                pointee: Box::new(abi_storage(
                    pointee,
                    declarations,
                    bindings,
                    target,
                    lowering,
                    depth + 1,
                )?),
                mutable: !pointee.is_const,
            }
        }
        TypeCategory::Record => {
            RustBindingType::Named { path: lowering.record_binding(ty)?.path.clone() }
        }
        TypeCategory::Enum => {
            let Some(crate::TypeShape {
                kind: TypeShapeKind::Enum { underlying: Some(underlying) },
                ..
            }) = declarations.type_shapes.get(&ty.canonical_spelling)
            else {
                return Err("inline enum has no compiler-owned ABI integer".into());
            };
            abi_storage(underlying, declarations, bindings, target, lowering, depth + 1)?
        }
        TypeCategory::Function | TypeCategory::Other => {
            return Err("inline ABI type has no native storage capability".into());
        }
    })
}

/// Emit a compiler-visible type alias and layout assertions without accepting injected C source.
fn c_alias(
    output: &mut String,
    alias: &str,
    ty: &TypeInfo,
    declarations: &DeclarationCatalog,
) -> Result<(), String> {
    let spelling = if ty.canonical_spelling.contains("(unnamed ")
        || ty.canonical_spelling.contains("(anonymous ")
    {
        if !ty.spelling.contains("(unnamed ") && !ty.spelling.contains("(anonymous ") {
            ty.spelling.as_str()
        } else {
            declarations
                .types
                .iter()
                .find_map(|(name, candidate)| {
                    (c_identifier(name) && candidate.canonical_spelling == ty.canonical_spelling)
                        .then_some(name.as_str())
                })
                .ok_or("anonymous inline ABI type has no compiler-visible typedef spelling")?
        }
    } else {
        ty.canonical_spelling.as_str()
    };
    if spelling.contains(['\n', '\r', ';', '{', '}']) {
        return Err("inline C type spelling is not a single compiler-owned type".into());
    }
    writeln!(output, "typedef __typeof__(({spelling}){{0}}) {alias};").expect("String output");
    if let Some(size) = ty.size {
        writeln!(output, "_Static_assert(sizeof({alias}) == {size}, \"inline ABI size\");")
            .expect("String output");
    }
    if let Some(alignment) = ty.alignment {
        writeln!(
            output,
            "_Static_assert(_Alignof({alias}) == {alignment}, \"inline ABI alignment\");"
        )
        .expect("String output");
    }
    Ok(())
}

/// Anchor the Rust thunk storage size and alignment to the compiler prototype facts.
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

/// Admit a single ASCII C identifier before using it in a native declaration or call.
fn c_identifier(name: &str) -> bool {
    let mut characters = name.chars();
    characters.next().is_some_and(|c| c == '_' || c.is_ascii_alphabetic())
        && characters.all(|c| c == '_' || c.is_ascii_alphanumeric())
}
