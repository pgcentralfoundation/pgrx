//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check inspected C target facts against the runtime's modeled representations and policies.

use crate::model::{
    ByteOrder, CompilationProfile, FrontendOutput, IntegerKind, IntegerType, SignedOverflow,
    TypeCategory, TypeInfo,
};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SupportProfileError {
    #[error("runtime support does not model compiler options: {0}")]
    UnsupportedOptions(String),
    #[error("runtime support does not yet model trapping signed overflow")]
    TrappingOverflow,
    #[error("runtime support requires 8-bit C bytes, found {0}")]
    ByteWidth(u32),
    #[error("runtime support requires 64-bit C pointers, found {0}")]
    PointerWidth(u32),
    #[error("runtime support currently requires signed plain char")]
    UnsignedChar,
    #[error("runtime support requires {expected:?}, found {actual:?}")]
    IntegerLayout { expected: IntegerType, actual: Option<IntegerType> },
    #[error("runtime support cannot establish a downstream Rust target guard for {0:?}")]
    TargetFamily(String),
    #[error("runtime support's recognized target architecture requires little endian")]
    ByteOrder,
    #[error(
        "pg-sys {name} requires an unqualified C unsigned int typedef with 4-byte storage, found {actual:?}"
    )]
    PgSysIntegerType { name: &'static str, actual: Option<TypeInfo> },
    #[error("pg-sys Datum requires an unqualified pointer-width unsigned C integer, found {0:?}")]
    PgSysDatumType(Option<TypeInfo>),
}

/// Require the complete integer representation and compiler policy implemented by the helpers.
///
/// This is a target-family gate, not evidence that an individual macro is emittable.
/// Fundamental identities remain distinct even when their storage types coincide.
pub fn validate_support_profile(profile: &CompilationProfile) -> Result<(), SupportProfileError> {
    if !profile.unsupported_options.is_empty() {
        return Err(SupportProfileError::UnsupportedOptions(
            profile.unsupported_options.join(", "),
        ));
    }
    if profile.signed_overflow == SignedOverflow::Trapping {
        return Err(SupportProfileError::TrappingOverflow);
    }
    let target = &profile.target;
    if target.char_bits != 8 {
        return Err(SupportProfileError::ByteWidth(target.char_bits));
    }
    if target.pointer_bits != 64 {
        return Err(SupportProfileError::PointerWidth(target.pointer_bits));
    }
    if !target.char_is_signed {
        return Err(SupportProfileError::UnsignedChar);
    }
    for (kind, bits, signed, rank) in [
        (IntegerKind::Bool, 8, false, 0),
        (IntegerKind::Char, 8, true, 1),
        (IntegerKind::SignedChar, 8, true, 1),
        (IntegerKind::UnsignedChar, 8, false, 1),
        (IntegerKind::Short, 16, true, 2),
        (IntegerKind::UnsignedShort, 16, false, 2),
        (IntegerKind::Int, 32, true, 3),
        (IntegerKind::UnsignedInt, 32, false, 3),
        (IntegerKind::Long, 64, true, 4),
        (IntegerKind::UnsignedLong, 64, false, 4),
        (IntegerKind::LongLong, 64, true, 5),
        (IntegerKind::UnsignedLongLong, 64, false, 5),
        (IntegerKind::Int128, 128, true, 6),
        (IntegerKind::UnsignedInt128, 128, false, 6),
    ] {
        let expected = IntegerType { kind, bits, signed, rank };
        let actual = target.integers.get(&kind).copied();
        if actual != Some(expected) {
            return Err(SupportProfileError::IntegerLayout { expected, actual });
        }
    }
    rust_target(profile)?;
    Ok(())
}

/// Emit a compile-time guard preventing a C-profile artifact from moving to another Rust target.
///
/// The supported architecture and operating-system names are derived from the inspected
/// compiler triple. Width and byte order are also checked explicitly. This function shares
/// the runtime support profile gate instead of assuming the generator's host is the target.
pub fn support_abi_assertions(profile: &CompilationProfile) -> Result<String, SupportProfileError> {
    validate_support_profile(profile)?;
    let (arch, os) = rust_target(profile)?;
    Ok(format!(
        "#[cfg(not(all(target_arch = {arch:?}, target_os = {os:?}, target_pointer_width = \"64\", target_endian = \"little\")))]\n\
         compile_error!(\"generated C macros require their inspected C target profile\");\n"
    ))
}

/// Generate numeric input adapters for pg-sys's own opaque integer types.
///
/// The C declarations establish each identity independently. Rust's `Oid` and
/// `TransactionId` expose exactly 32 bits of numeric storage; no adapter is
/// emitted unless that storage matches the inspected C typedef. Datum's pointer
/// storage uses exposed provenance when crossing its C integer representation.
pub fn pg_sys_integer_bridges(frontend: &FrontendOutput) -> Result<String, SupportProfileError> {
    let mut rust = support_abi_assertions(frontend.profile())?;
    for (name, accessor, constructor) in
        [("Oid", "to_u32", "from_u32"), ("TransactionId", "into_inner", "from_inner")]
    {
        let actual = frontend.declarations().types.get(name);
        if !actual.is_some_and(|ty| {
            ty.category == TypeCategory::Integer(IntegerKind::UnsignedInt)
                && ty.size == Some(4)
                && !ty.is_const
                && !ty.is_volatile
        }) {
            return Err(SupportProfileError::PgSysIntegerType { name, actual: actual.cloned() });
        }
        rust.push_str(&format!(
            "impl crate::__pgrx_c_macros::sealed::Sealed for crate::{name} {{}}\n\
             impl crate::__pgrx_c_macros::IntoCValue for crate::{name} {{\n\
                 type Kind = crate::__pgrx_c_macros::CUnsignedInt;\n\
                 fn into_c_value(self) -> crate::__pgrx_c_macros::CValue<Self::Kind> {{\n\
                     crate::__pgrx_c_macros::CValue::new(self.{accessor}())\n\
                 }}\n\
             }}\n"
        ));
        rust.push_str(&format!(
            "impl crate::__pgrx_c_macros::expression::IntegerStorage<crate::__pgrx_c_macros::CUnsignedInt> for crate::{name} {{\n\
               fn decode(self) -> crate::__pgrx_c_macros::CValue<crate::__pgrx_c_macros::CUnsignedInt> {{ crate::__pgrx_c_macros::CValue::new(self.{accessor}()) }}\n\
               fn encode(value: crate::__pgrx_c_macros::CValue<crate::__pgrx_c_macros::CUnsignedInt>) -> Self {{ Self::{constructor}(value.get()) }}\n\
             }}\n\
             impl crate::__pgrx_c_macros::expression::NativeType for crate::{name} {{ type Marker = crate::__pgrx_c_macros::expression::CIntegerStorage<crate::__pgrx_c_macros::CUnsignedInt, Self>; }}\n\
             impl crate::__pgrx_c_macros::expression::IntoExpression for crate::{name} {{ type Value = crate::__pgrx_c_macros::CValue<crate::__pgrx_c_macros::CUnsignedInt>; fn into_expression(self) -> Self::Value {{ crate::__pgrx_c_macros::IntoCValue::into_c_value(self) }} }}\n"
        ));
    }
    let datum = frontend.declarations().types.get("Datum");
    if datum.is_none() {
        return Ok(rust);
    }
    let datum_kind = datum
        .and_then(|ty| match ty.category {
            TypeCategory::Integer(
                kind @ (IntegerKind::UnsignedLong | IntegerKind::UnsignedLongLong),
            ) if ty.size == Some(u64::from(frontend.profile().target.pointer_bits / 8))
                && ty.alignment == Some(8)
                && !ty.is_const
                && !ty.is_volatile =>
            {
                Some(kind)
            }
            _ => None,
        })
        .ok_or_else(|| SupportProfileError::PgSysDatumType(datum.cloned()))?;
    let marker = match datum_kind {
        IntegerKind::UnsignedLong => "CUnsignedLong",
        IntegerKind::UnsignedLongLong => "CUnsignedLongLong",
        _ => unreachable!(
            "the Datum representation gate admits only pointer-width unsigned integers"
        ),
    };
    rust.push_str(&format!(
        "// C Datum is an integer that can retain an exposed Rust pointer's provenance.\n\
         const _: () = {{ assert!(::core::mem::size_of::<crate::Datum>() == 8); assert!(::core::mem::align_of::<crate::Datum>() == 8); }};\n\
         impl crate::__pgrx_c_macros::sealed::Sealed for crate::Datum {{}}\n\
         impl crate::__pgrx_c_macros::IntoCValue for crate::Datum {{\n\
           type Kind = crate::__pgrx_c_macros::{marker};\n\
           fn into_c_value(self) -> crate::__pgrx_c_macros::CValue<Self::Kind> {{ crate::__pgrx_c_macros::CValue::new(self.cast_mut_ptr::<()>().expose_provenance() as u64) }}\n\
         }}\n\
         impl crate::__pgrx_c_macros::expression::IntegerStorage<crate::__pgrx_c_macros::{marker}> for crate::Datum {{\n\
           fn decode(self) -> crate::__pgrx_c_macros::CValue<crate::__pgrx_c_macros::{marker}> {{ crate::__pgrx_c_macros::IntoCValue::into_c_value(self) }}\n\
           fn encode(value: crate::__pgrx_c_macros::CValue<crate::__pgrx_c_macros::{marker}>) -> Self {{ Self::from(::core::ptr::with_exposed_provenance_mut::<()>(value.get() as usize)) }}\n\
         }}\n\
         impl crate::__pgrx_c_macros::expression::NativeType for crate::Datum {{ type Marker = crate::__pgrx_c_macros::expression::CIntegerStorage<crate::__pgrx_c_macros::{marker}, Self>; }}\n\
         impl crate::__pgrx_c_macros::expression::IntoExpression for crate::Datum {{ type Value = crate::__pgrx_c_macros::CValue<crate::__pgrx_c_macros::{marker}>; fn into_expression(self) -> Self::Value {{ crate::__pgrx_c_macros::IntoCValue::into_c_value(self) }} }}\n"
    ));
    Ok(rust)
}

fn rust_target(
    profile: &CompilationProfile,
) -> Result<(&'static str, &'static str), SupportProfileError> {
    let triple = &profile.target.triple;
    let mut components = triple.split('-');
    let arch = match components.next() {
        Some("x86_64") => "x86_64",
        Some("aarch64" | "arm64") => "aarch64",
        _ => return Err(SupportProfileError::TargetFamily(triple.clone())),
    };
    let components = components.collect::<Vec<_>>();
    let os = if components.contains(&"linux") {
        "linux"
    } else if components.first() == Some(&"apple")
        && components.get(1).is_some_and(|os| {
            os.starts_with("darwin") || os.starts_with("macosx") || *os == "macos"
        })
    {
        "macos"
    } else {
        return Err(SupportProfileError::TargetFamily(triple.clone()));
    };
    if profile.target.byte_order != ByteOrder::Little {
        return Err(SupportProfileError::ByteOrder);
    }
    Ok((arch, os))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BuildInputs, CompilerIdentity, TargetFacts};
    use std::collections::BTreeMap;

    fn profile() -> CompilationProfile {
        let integers = [
            (IntegerKind::Bool, 8, false, 0),
            (IntegerKind::Char, 8, true, 1),
            (IntegerKind::SignedChar, 8, true, 1),
            (IntegerKind::UnsignedChar, 8, false, 1),
            (IntegerKind::Short, 16, true, 2),
            (IntegerKind::UnsignedShort, 16, false, 2),
            (IntegerKind::Int, 32, true, 3),
            (IntegerKind::UnsignedInt, 32, false, 3),
            (IntegerKind::Long, 64, true, 4),
            (IntegerKind::UnsignedLong, 64, false, 4),
            (IntegerKind::LongLong, 64, true, 5),
            (IntegerKind::UnsignedLongLong, 64, false, 5),
            (IntegerKind::Int128, 128, true, 6),
            (IntegerKind::UnsignedInt128, 128, false, 6),
        ]
        .into_iter()
        .map(|(kind, bits, signed, rank)| (kind, IntegerType { kind, bits, signed, rank }))
        .collect::<BTreeMap<_, _>>();
        CompilationProfile {
            header: "header.h".into(),
            compiler: CompilerIdentity {
                executable: "clang".into(),
                version: "test".into(),
                libclang_version: "test".into(),
            },
            arguments: Vec::new(),
            target: TargetFacts {
                triple: "aarch64-apple-macosx26.0.0".into(),
                pointer_bits: 64,
                function_pointer: crate::PointerLayout { size: 8, alignment: 8 },
                char_bits: 8,
                char_is_signed: true,
                ascii_execution_charset: true,
                byte_order: ByteOrder::Little,
                c_standard: Some(201710),
                integers,
                floating_point: Default::default(),
            },
            signed_overflow: SignedOverflow::Wrapping,
            unsupported_options: Vec::new(),
            inputs: BuildInputs::default(),
        }
    }

    #[test]
    fn pg_sys_bridges_require_the_original_typedef_identity_and_storage() {
        use crate::{DeclarationCatalog, MacroEnvironment, MacroInventory};
        let environment = MacroEnvironment::default();
        let dependencies = crate::MacroDependencyGraph::from_environment(&environment);
        let mut frontend = FrontendOutput {
            profile: profile(),
            environment,
            declarations: DeclarationCatalog::default(),
            inventory: MacroInventory { macros: Vec::new(), diagnostics: Vec::new() },
            dependencies,
        };
        for name in ["Oid", "TransactionId"] {
            frontend.declarations.types.insert(
                name.into(),
                TypeInfo {
                    spelling: name.into(),
                    canonical_spelling: "unsigned int".into(),
                    category: TypeCategory::Integer(IntegerKind::UnsignedInt),
                    size: Some(4),
                    alignment: Some(4),
                    is_const: false,
                    is_volatile: false,
                },
            );
        }
        frontend.declarations.types.insert(
            "Datum".into(),
            TypeInfo {
                spelling: "Datum".into(),
                canonical_spelling: "unsigned long".into(),
                category: TypeCategory::Integer(IntegerKind::UnsignedLong),
                size: Some(8),
                alignment: Some(8),
                is_const: false,
                is_volatile: false,
            },
        );
        let rust = pg_sys_integer_bridges(&frontend).unwrap();
        assert!(rust.contains("for crate::Oid"));
        assert!(rust.contains("self.to_u32()"));
        assert!(rust.contains("self.into_inner()"));
        assert!(rust.contains("with_exposed_provenance_mut"));
        assert!(rust.contains("expose_provenance"));
        frontend.declarations.types.get_mut("TransactionId").unwrap().category =
            TypeCategory::Integer(IntegerKind::UnsignedLong);
        assert!(matches!(
            pg_sys_integer_bridges(&frontend),
            Err(SupportProfileError::PgSysIntegerType { name: "TransactionId", .. })
        ));
        frontend.declarations.types.get_mut("TransactionId").unwrap().category =
            TypeCategory::Integer(IntegerKind::UnsignedInt);
        frontend.declarations.types.get_mut("Oid").unwrap().is_volatile = true;
        assert!(matches!(
            pg_sys_integer_bridges(&frontend),
            Err(SupportProfileError::PgSysIntegerType { name: "Oid", .. })
        ));
    }

    #[test]
    fn rejects_erased_identity_mismatches_and_unmodeled_policies() {
        let mut profile = profile();
        assert!(validate_support_profile(&profile).is_ok());
        profile.target.integers.get_mut(&IntegerKind::Long).unwrap().rank = 5;
        assert!(matches!(
            validate_support_profile(&profile),
            Err(SupportProfileError::IntegerLayout { .. })
        ));
        profile.target.integers.get_mut(&IntegerKind::Long).unwrap().rank = 4;
        profile.signed_overflow = SignedOverflow::Trapping;
        assert_eq!(validate_support_profile(&profile), Err(SupportProfileError::TrappingOverflow));
    }

    #[test]
    fn downstream_assertions_follow_inspected_target_not_host() {
        let mut profile = profile();
        let macos = support_abi_assertions(&profile).unwrap();
        assert!(macos.contains("target_arch = \"aarch64\""));
        assert!(macos.contains("target_os = \"macos\""));
        profile.target.triple = "x86_64-unknown-linux-gnu".into();
        let linux = support_abi_assertions(&profile).unwrap();
        assert!(linux.contains("target_arch = \"x86_64\""));
        assert!(linux.contains("target_os = \"linux\""));
        profile.target.triple = "x86_64-pc-windows-msvc".into();
        assert!(matches!(
            support_abi_assertions(&profile),
            Err(SupportProfileError::TargetFamily(_))
        ));
    }
}
