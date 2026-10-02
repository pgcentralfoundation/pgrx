//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Nominal enum identities and checked bridges for the actual binding storage.

use super::types::{Lowering, enum_key};
use crate::{
    BindingCatalog, DeclarationCatalog, IntegerValue, RustBindingType, TargetFacts, TypeInfo,
    TypeShapeKind,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

const EXPRESSION: &str = "crate::__pgrx_c_macros::expression";
const SOURCE_LIMIT: usize = 16 * 1024 * 1024;

pub(super) struct EnumAdapters {
    pub rust: String,
    pub unsupported: BTreeMap<String, String>,
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
) -> Result<EnumAdapters, String> {
    let lowering = Lowering::new(declarations, bindings, target);
    let mut rust = String::new();
    let mut identities = BTreeSet::new();
    let mut unsupported = BTreeMap::new();
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
        let kind = numeric.marker.replace("$crate", "crate");
        let repr = numeric.storage.replace("$crate", "crate");
        let identity = identity_path(&shape.ty);
        let name = identity.rsplit("::").next().expect("generated identity has a namespace");
        writeln!(rust, "#[derive(Clone,Copy,Debug,PartialEq,Eq)]\npub struct {name};\nimpl crate::__pgrx_c_macros::sealed::Sealed for {name} {{}}\nimpl {EXPRESSION}::EnumIdentity for {name} {{}}").expect("String output");
        let size = shape.ty.size.ok_or("C enum has no compiler-established size")?;
        let alignment = shape.ty.alignment.ok_or("C enum has no compiler-established alignment")?;
        writeln!(rust, "const _: () = {{ assert!(::core::mem::size_of::<{repr}>() == {size}); assert!(::core::mem::align_of::<{repr}>() == {alignment}); }};").expect("String output");
        writeln!(rust, "// SAFETY: Clang's compatible integer type and the assertions establish identical enum layout; every integer storage value is valid.\nunsafe impl {EXPRESSION}::EnumStorage<{name},{kind}> for {repr} {{ fn decode(value:Self)->{repr} {{ value }} fn encode(value:{repr})->Self {{ value }} }}").expect("String output");
        // Enum and compatible integer pointer identities are compatible in C,
        // while two independently declared enums remain distinct identities.
        writeln!(rust, "impl {EXPRESSION}::CompatibleIdentity<{kind}> for {EXPRESSION}::CEnum<{name},{kind}> {{}}\nimpl {EXPRESSION}::CompatibleIdentity<{EXPRESSION}::CEnum<{name},{kind}>> for {kind} {{}}").expect("String output");
        let binding = match lowering.enum_binding(&shape.ty) {
            Ok(binding) => binding,
            Err(reason) => {
                unsupported.insert(enum_key(&shape.ty), reason);
                continue;
            }
        };
        if let Some(binding) = binding {
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
            let storage = object.storage.replace("$crate", "crate");
            let marker = format!("{EXPRESSION}::CEnumObject<{name},{kind},{storage}>");
            writeln!(rust, "const _: () = {{ assert!(::core::mem::size_of::<{storage}>() == {size}); assert!(::core::mem::align_of::<{storage}>() == {alignment}); }};").expect("String output");
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
            writeln!(rust, "impl crate::__pgrx_c_macros::sealed::Sealed for {storage} {{}}\nimpl {EXPRESSION}::NativeType for {storage} {{ type Marker = {marker}; }}\nimpl {EXPRESSION}::IntoExpression for {storage} {{ type Value = crate::__pgrx_c_macros::CValue<{EXPRESSION}::CEnum<{name},{kind}>>; fn into_expression(self)->Self::Value {{ <{marker} as {EXPRESSION}::CType>::from_storage(self) }} }}").expect("String output");
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
