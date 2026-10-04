//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Nominal enum identities and checked bridges for the actual binding storage.

use super::types::{LoweredType, Lowering, enum_key};
use crate::{
    BindingCatalog, DeclarationCatalog, EnumBinding, IntegerValue, RustBindingType, TargetFacts,
    TypeInfo, TypeShapeKind,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

const EXPRESSION: &str = "c::expression";
const SOURCE_LIMIT: usize = 16 * 1024 * 1024;

pub(super) struct EnumAdapters {
    pub rust: String,
    pub unsupported: BTreeMap<String, String>,
}

#[derive(Default, PartialEq, Eq)]
pub(super) struct EnumRequests {
    pub types: BTreeSet<String>,
    /// Open native operands can introduce actual Rust enum objects. Integer
    /// aliases already use primitive bridges and require a C enum identity only
    /// when a compiler-owned expression or prototype names that identity.
    pub open_scalar: bool,
}

struct ValidatedEnum<'a> {
    ty: &'a TypeInfo,
    numeric: LoweredType,
    object: Option<(&'a EnumBinding, LoweredType)>,
}

pub(super) fn identity_path(ty: &TypeInfo) -> String {
    let digest = Sha256::digest(enum_key(ty).as_bytes());
    let identity = digest.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    format!("$crate::__pgrx_c_generated::EnumIdentity_{identity}")
}

pub(super) fn generate(
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    target: &TargetFacts,
    requests: &EnumRequests,
) -> Result<EnumAdapters, String> {
    let lowering = Lowering::new_native(declarations, bindings, target);
    let mut identities = BTreeSet::new();
    let mut unsupported = BTreeMap::new();
    let mut validated = Vec::new();
    for shape in declarations.type_shapes.values() {
        if !matches!(shape.kind, TypeShapeKind::Enum { underlying: Some(_) })
            || !identities.insert(enum_key(&shape.ty))
        {
            continue;
        }
        let numeric = match lowering.enum_underlying(&shape.ty).and_then(|underlying| {
            if shape.ty.size.is_none()
                || shape.ty.alignment.is_none()
                || shape.ty.size != underlying.size
                || shape.ty.alignment != underlying.alignment
            {
                return Err(
                    "C enum layout differs from its compiler-established compatible integer".into(),
                );
            }
            lowering.resolve(underlying)
        }) {
            Ok(numeric) => numeric,
            Err(reason) => {
                unsupported.insert(enum_key(&shape.ty), reason);
                continue;
            }
        };
        let binding = match lowering.enum_binding(&shape.ty) {
            Ok(binding) => binding,
            Err(reason) => {
                unsupported.insert(enum_key(&shape.ty), reason);
                continue;
            }
        };
        let object = if let Some(binding) = binding {
            let object = match lowering.resolve_with_storage(
                &shape.ty,
                &RustBindingType::Named { path: binding.path.clone() },
            ) {
                Ok(object) => object,
                Err(reason) => {
                    unsupported.insert(enum_key(&shape.ty), reason);
                    continue;
                }
            };
            for variant in binding.variants.keys() {
                super::rust_identifier(variant)
                    .ok_or("Rust enum variant has no usable public identifier")?;
            }
            Some((binding, object))
        } else {
            None
        };
        validated.push(ValidatedEnum { ty: &shape.ty, numeric, object });
    }
    // All compiler/binding identities were reconciled above, including aliases
    // excluded by demand. Native integer aliases cannot introduce a nominal C
    // enum marker; explicit expression dependencies retain those identities.
    let mut rust = String::new();
    let mut layouts = BTreeSet::new();
    for adapter in validated {
        if !(requests.types.contains(&adapter.ty.canonical_spelling)
            || requests.types.contains(&enum_key(adapter.ty))
            || requests.open_scalar && adapter.object.is_some())
        {
            continue;
        }
        let kind = adapter.numeric.marker.replace("$crate", "crate");
        let repr = adapter.numeric.storage.replace("$crate", "crate");
        let identity = identity_path(adapter.ty);
        let name = identity.rsplit("::").next().expect("generated identity has a namespace");
        writeln!(rust, "#[derive(Clone,Copy,Debug,PartialEq,Eq)]\npub struct {name};\nimpl c::sealed::Sealed for {name} {{}}\nimpl {EXPRESSION}::EnumIdentity for {name} {{}}").expect("String output");
        let size = adapter.ty.size.expect("validated enum size");
        let alignment = adapter.ty.alignment.expect("validated enum alignment");
        if layouts.insert((repr.clone(), size, alignment)) {
            writeln!(rust, "const _: () = {{ assert!(::core::mem::size_of::<{repr}>() == {size}); assert!(::core::mem::align_of::<{repr}>() == {alignment}); }};").expect("String output");
        }
        writeln!(
            rust,
            "impl {EXPRESSION}::enumeration::NumericEnum for {name} {{ type Compatible = {kind}; }}"
        )
        .expect("String output");
        if let Some((binding, object)) = adapter.object {
            let storage = object.storage.replace("$crate", "crate");
            let marker = format!("{EXPRESSION}::CEnumObject<{name},{kind},{storage}>");
            if layouts.insert((storage.clone(), size, alignment)) {
                writeln!(rust, "const _: () = {{ assert!(::core::mem::size_of::<{storage}>() == {size}); assert!(::core::mem::align_of::<{storage}>() == {alignment}); }};").expect("String output");
            }
            write!(rust, "// SAFETY: The explicit Rust integer repr matches Clang's compatible enum type and the assertions prove layout. Decoding receives a valid unit Rust variant; encoding checks every discriminant.\nunsafe impl {EXPRESSION}::EnumStorage<{name},{kind}> for {storage} {{ fn decode(value:Self)->{repr} {{ value as {repr} }} fn encode(value:{repr})->Self {{ match value {{").expect("String output");
            for (variant, value) in &binding.variants {
                let variant = super::rust_identifier(variant)
                    .ok_or("Rust enum variant has no usable public identifier")?;
                let value = integer_literal(*value);
                write!(rust, "{value} => {storage}::{variant},").expect("String output");
            }
            writeln!(
                rust,
                "_ => panic!(\"C enum value has no corresponding Rust variant\"), }} }} }}"
            )
            .expect("String output");
            writeln!(rust, "impl c::sealed::Sealed for {storage} {{}}\nimpl {EXPRESSION}::NativeType for {storage} {{ type Marker = {marker}; }}\nimpl {EXPRESSION}::IntoExpression for {storage} {{ type Value = c::CValue<{EXPRESSION}::CEnum<{name},{kind}>>; fn into_expression(self)->Self::Value {{ <{marker} as {EXPRESSION}::CType>::from_storage(self) }} }}").expect("String output");
        }
        if rust.len() > SOURCE_LIMIT {
            return Err("enum adapters exceed the bounded source budget".into());
        }
    }
    Ok(EnumAdapters { rust, unsupported })
}

fn integer_literal(value: IntegerValue) -> String {
    match value {
        IntegerValue::Signed(value) => value.to_string(),
        IntegerValue::Unsigned(value) => value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AliasBinding, ByteOrder, IntegerKind, IntegerType, PointerLayout, TypeCategory, TypeShape,
    };

    fn fixture() -> (DeclarationCatalog, BindingCatalog, TargetFacts) {
        let target = TargetFacts {
            triple: "fixture".into(),
            pointer_bits: 64,
            function_pointer: PointerLayout { size: 8, alignment: 8 },
            size_type: IntegerKind::UnsignedLong,
            offsetof_supported: true,
            char_bits: 8,
            char_is_signed: true,
            ascii_execution_charset: true,
            byte_order: ByteOrder::Little,
            c_standard: Some(201710),
            integers: [(
                IntegerKind::UnsignedInt,
                IntegerType { kind: IntegerKind::UnsignedInt, bits: 32, signed: false, rank: 3 },
            )]
            .into_iter()
            .collect(),
            floating_point: Default::default(),
        };
        let integer = TypeInfo {
            spelling: "unsigned int".into(),
            canonical_spelling: "unsigned int".into(),
            category: TypeCategory::Integer(IntegerKind::UnsignedInt),
            size: Some(4),
            alignment: Some(4),
            is_const: false,
            is_volatile: false,
        };
        let mut declarations = DeclarationCatalog::default();
        let mut bindings = BindingCatalog::default();
        for name in ["First", "Second", "Invalid", "Alias", "AliasInvalid"] {
            let ty = TypeInfo {
                spelling: name.into(),
                canonical_spelling: format!("enum {name}"),
                category: TypeCategory::Enum,
                size: Some(if name.ends_with("Invalid") { 8 } else { 4 }),
                ..integer.clone()
            };
            declarations.types.insert(name.into(), ty.clone());
            declarations.type_shapes.insert(
                ty.canonical_spelling.clone(),
                TypeShape {
                    ty,
                    is_restrict: false,
                    kind: TypeShapeKind::Enum { underlying: Some(integer.clone()) },
                },
            );
            if name.starts_with("Alias") {
                bindings.types.insert(
                    name.into(),
                    AliasBinding {
                        path: vec![name.into()],
                        target: RustBindingType::Integer { signed: false, bits: 32 },
                    },
                );
                continue;
            }
            bindings.enums.insert(
                name.into(),
                EnumBinding {
                    path: vec![name.into()],
                    repr: Some(RustBindingType::Integer { signed: false, bits: 32 }),
                    variants: [(format!("{name}One"), IntegerValue::Unsigned(1))]
                        .into_iter()
                        .collect(),
                },
            );
        }
        (declarations, bindings, target)
    }

    #[test]
    fn selection_preserves_unselected_enum_validation_and_checked_storage_bridges() {
        let (declarations, bindings, target) = fixture();
        let empty = generate(&declarations, &bindings, &target, &EnumRequests::default()).unwrap();
        assert!(empty.rust.is_empty());
        assert!(empty.unsupported.contains_key("enum Invalid"));
        let requests = EnumRequests {
            types: ["enum First".into()].into_iter().collect(),
            ..Default::default()
        };
        let selected = generate(&declarations, &bindings, &target, &requests).unwrap();
        assert_eq!(selected.unsupported, empty.unsupported);
        assert_eq!(selected.rust.matches("::EnumIdentity for").count(), 1);
        assert!(selected.rust.contains("::NativeType for crate::First"));
        assert!(selected.rust.contains("::IntoExpression for crate::First"));
        assert!(!selected.rust.contains("crate::Second"));
        assert!(selected.rust.contains("C enum value has no corresponding Rust variant"));
    }

    #[test]
    fn shared_layout_assertions_preserve_distinct_identities_and_rejected_layouts() {
        let (declarations, bindings, target) = fixture();
        let requests = EnumRequests {
            types: ["enum First".into(), "enum Second".into(), "enum Invalid".into()]
                .into_iter()
                .collect(),
            ..Default::default()
        };
        let output = generate(&declarations, &bindings, &target, &requests).unwrap();
        assert_eq!(output.rust.matches("::EnumIdentity for").count(), 2);
        assert_eq!(output.rust.matches("::enumeration::NumericEnum for").count(), 2);
        assert!(!output.rust.contains("::CompatibleIdentity<"));
        assert_eq!(output.rust.matches("size_of::<u32>() == 4").count(), 1);
        assert_eq!(output.rust.matches("size_of::<crate::First>() == 4").count(), 1);
        assert_eq!(output.rust.matches("size_of::<crate::Second>() == 4").count(), 1);
        assert!(output.unsupported.contains_key("enum Invalid"));
        assert!(output.unsupported.contains_key("enum AliasInvalid"));
        assert!(!output.rust.contains("crate::Invalid"));
    }

    #[test]
    fn open_native_operands_retain_rust_enum_objects_without_alias_identities() {
        let (declarations, bindings, target) = fixture();
        let requests = EnumRequests { open_scalar: true, ..Default::default() };
        let output = generate(&declarations, &bindings, &target, &requests).unwrap();
        assert_eq!(output.rust.matches("::EnumIdentity for").count(), 2);
        assert!(output.rust.contains("::IntoExpression for crate::First"));
        assert!(output.rust.contains("::IntoExpression for crate::Second"));
        assert!(!output.rust.contains("crate::Invalid"));
        assert!(output.unsupported.contains_key("enum Invalid"));
        assert!(output.unsupported.contains_key("enum AliasInvalid"));
        let identity = identity_path(&declarations.types["Alias"]);
        assert!(!output.rust.contains(identity.rsplit("::").next().unwrap()));
    }

    #[test]
    fn explicit_alias_dependencies_retain_their_nominal_integer_storage_bridge() {
        let (declarations, bindings, target) = fixture();
        let requests = EnumRequests {
            types: ["enum Alias".into()].into_iter().collect(),
            ..Default::default()
        };
        let output = generate(&declarations, &bindings, &target, &requests).unwrap();
        assert_eq!(output.rust.matches("::EnumIdentity for").count(), 1);
        assert!(output.rust.contains("::enumeration::NumericEnum for"));
        assert!(output.rust.contains("type Compatible = c::CUnsignedInt;"));
        assert!(!output.rust.contains("fn decode"));
        assert!(!output.rust.contains("::CompatibleIdentity<"));
        assert!(!output.rust.contains("::NativeType for crate::Alias"));
        assert!(!output.rust.contains("::IntoExpression for crate::Alias"));
        assert!(output.unsupported.contains_key("enum AliasInvalid"));
    }
}
