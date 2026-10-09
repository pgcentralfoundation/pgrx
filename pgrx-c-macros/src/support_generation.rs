//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check inspected C target facts against the runtime's modeled representations and policies.

//! This is the gate between inspected C profiles and pgrx-pg-sys runtime helpers.
//!
//! The helpers implement a bounded target and policy family, so generation checks widths,
//! ranks, byte order, overflow policy, and the original PostgreSQL integer typedefs first.
//! Emitted target assertions protect consumers from using an artifact on a different Rust
//! target; opaque pg-sys integer bridges preserve the verified C identity.

use crate::model::{
    ByteOrder, CompilationProfile, FrontendOutput, IntegerKind, IntegerType, SignedOverflow,
    TypeCategory, TypeInfo,
};

/// Explain which inspected target or PostgreSQL typedef fact falls outside the runtime helper
/// contract.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SupportProfileError {
    /// The recorded compiler semantic flags exceed the runtime helper contract.
    #[error("runtime support does not model compiler options: {0}")]
    UnsupportedOptions(
        /// Recorded semantic flags not modeled by the runtime helper family.
        String,
    ),
    /// Trapping signed arithmetic lacks an implemented equivalent runtime policy.
    #[error("runtime support does not yet model trapping signed overflow")]
    TrappingOverflow,
    /// The target C byte is not the eight-bit representation implemented by the helpers.
    #[error("runtime support requires 8-bit C bytes, found {0}")]
    ByteWidth(
        /// Observed bits in one target C byte.
        u32,
    ),
    /// The target object-pointer representation lies outside the supported helper family.
    #[error("runtime support requires 32-bit or 64-bit C pointers, found {0}")]
    PointerWidth(
        /// Observed target object-pointer width in bits.
        u32,
    ),
    /// The target sizeof/alignment result does not have the supported C identity.
    #[error("runtime support requires unsigned pointer-width size_t, found {0:?}")]
    SizeType(
        /// Observed sizeof/alignment C integer identity.
        IntegerKind,
    ),
    /// Pointer subtraction identity differs from the selected runtime target alias.
    #[error("runtime support requires signed pointer-width ptrdiff_t, found {0:?}")]
    PtrDiffType(
        /// Observed signed C identity of pointer subtraction.
        IntegerKind,
    ),
    /// A fundamental C type differs in identity, width, signedness, or rank from the helper contract.
    #[error("runtime support requires {expected:?}, found {actual:?}")]
    IntegerLayout {
        /// The complete runtime-supported integer identity and representation.
        expected: IntegerType,
        /// The inspected fact, or its absence, that prevented proving the runtime or pg-sys bridge
        /// contract.
        actual: Option<IntegerType>,
    },
    /// The inspected C target cannot be guarded by the bounded Rust architecture/OS mapping.
    #[error("runtime support cannot establish a downstream Rust target guard for {0:?}")]
    TargetFamily(
        /// The C triple that lacks a supported Rust target guard.
        String,
    ),
    /// A partial or malformed preferred-alignment map cannot configure all runtime markers.
    #[error("runtime support lacks a verified scalar/array alignment for {0}")]
    AlignmentFacts(
        /// The fundamental marker whose compiler alignment witnesses are missing or invalid.
        &'static str,
    ),
    /// An opaque pg-sys integer wrapper lacks its required original C typedef identity or storage.
    #[error(
        "pg-sys {name} requires an unqualified C unsigned int typedef with 4-byte storage, found {actual:?}"
    )]
    PgSysIntegerType {
        /// The PostgreSQL typedef whose opaque Rust bridge could not be established.
        name: &'static str,
        /// The inspected fact, or its absence, that prevented proving the runtime or pg-sys bridge
        /// contract.
        actual: Option<TypeInfo>,
    },
    /// Datum lacks the unqualified pointer-width unsigned C integer contract required by its bridge.
    #[error("pg-sys Datum requires an unqualified pointer-width unsigned C integer, found {0:?}")]
    PgSysDatumType(
        /// The inspected Datum type, or its absence, that prevented its opaque integer bridge.
        Option<TypeInfo>,
    ),
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
    if !matches!(target.pointer_bits, 32 | 64) {
        return Err(SupportProfileError::PointerWidth(target.pointer_bits));
    }
    let (_, os) = rust_target(profile)?;
    let long_bits = if os == "windows" { 32 } else { target.pointer_bits };
    if !matches!(
        target.size_type,
        IntegerKind::UnsignedInt | IntegerKind::UnsignedLong | IntegerKind::UnsignedLongLong
    ) || target
        .integers
        .get(&target.size_type)
        .is_none_or(|ty| ty.signed || ty.bits != target.pointer_bits)
    {
        return Err(SupportProfileError::SizeType(target.size_type));
    }
    if !matches!(target.ptrdiff_type, IntegerKind::Int | IntegerKind::Long | IntegerKind::LongLong)
        || target
            .integers
            .get(&target.ptrdiff_type)
            .is_none_or(|ty| !ty.signed || ty.bits != target.pointer_bits)
    {
        return Err(SupportProfileError::PtrDiffType(target.ptrdiff_type));
    }
    for (kind, bits, signed, rank) in [
        (IntegerKind::Bool, 8, false, 0),
        (IntegerKind::Char, 8, target.char_is_signed, 1),
        (IntegerKind::SignedChar, 8, true, 1),
        (IntegerKind::UnsignedChar, 8, false, 1),
        (IntegerKind::Short, 16, true, 2),
        (IntegerKind::UnsignedShort, 16, false, 2),
        (IntegerKind::Int, 32, true, 3),
        (IntegerKind::UnsignedInt, 32, false, 3),
        (IntegerKind::Long, long_bits, true, 4),
        (IntegerKind::UnsignedLong, long_bits, false, 4),
        (IntegerKind::LongLong, 64, true, 5),
        (IntegerKind::UnsignedLongLong, 64, false, 5),
    ] {
        let expected = IntegerType { kind, bits, signed, rank };
        let actual = target.integers.get(&kind).copied();
        if actual != Some(expected) {
            return Err(SupportProfileError::IntegerLayout { expected, actual });
        }
    }
    // Some valid 32-bit C targets do not expose the Clang 128-bit extension.
    // Check any admitted identities without requiring the extension globally.
    if target.integers.contains_key(&IntegerKind::Int128)
        || target.integers.contains_key(&IntegerKind::UnsignedInt128)
    {
        for (kind, signed) in [(IntegerKind::Int128, true), (IntegerKind::UnsignedInt128, false)] {
            let actual = target.integers.get(&kind).copied();
            let expected = IntegerType { kind, bits: 128, signed, rank: 6 };
            if actual != Some(expected) {
                return Err(SupportProfileError::IntegerLayout { expected, actual });
            }
        }
    }
    if !target.preferred_alignments.is_empty() {
        for (marker, _) in crate::frontend::ALIGNMENT_PROBES {
            if matches!(*marker, "CInt128" | "CUnsignedInt128")
                && !target.integers.contains_key(&IntegerKind::Int128)
            {
                continue;
            }
            if target.preferred_alignments.get(*marker).is_none_or(|alignment| {
                !alignment.scalar.is_power_of_two()
                    || !alignment.array.is_power_of_two()
                    || u128::from(alignment.scalar) >= (1u128 << target.pointer_bits)
                    || u128::from(alignment.array) >= (1u128 << target.pointer_bits)
            }) {
                return Err(SupportProfileError::AlignmentFacts(marker));
            }
        }
    }
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
    let target = &profile.target;
    let pointer_bits = target.pointer_bits.to_string();
    let endian = match target.byte_order {
        ByteOrder::Little => "little",
        ByteOrder::Big => "big",
    };
    let long_bits = target.integers[&IntegerKind::Long].bits;
    let size_rank = target.integers[&target.size_type].rank;
    let ptrdiff_rank = target.integers[&target.ptrdiff_type].rank;
    let char_signed = target.char_is_signed;
    let abi_guard = match (target.arm_float_abi, target.ppc64_elf_abi) {
        (Some(crate::ArmFloatAbi::Base), _) => ", target_abi = \"eabi\"",
        (Some(crate::ArmFloatAbi::Vfp), _) => ", target_abi = \"eabihf\"",
        (_, Some(crate::Ppc64ElfAbi::V1)) => ", target_abi = \"elfv1\"",
        (_, Some(crate::Ppc64ElfAbi::V2)) => ", target_abi = \"elfv2\"",
        (None, None) => "",
    };
    Ok(format!(
        "#[cfg(not(all(target_arch = {arch:?}, target_os = {os:?}, target_pointer_width = {pointer_bits:?}, target_endian = {endian:?}{abi_guard})))]\n\
         compile_error!(\"generated C macros require their inspected C target profile\");\n\
         const _: () = {{\n\
           use crate::__pgrx_c_macros::CInteger as _;\n\
           assert!(crate::__pgrx_c_macros::CChar::SIGNED == {char_signed});\n\
           assert!(crate::__pgrx_c_macros::CLong::BITS == {long_bits});\n\
           assert!(crate::__pgrx_c_macros::CSize::BITS == {pointer_bits});\n\
           assert!(crate::__pgrx_c_macros::CSize::RANK == {size_rank});\n\
           assert!(!crate::__pgrx_c_macros::CSize::SIGNED);\n\
           assert!(crate::__pgrx_c_macros::CPtrDiff::BITS == {pointer_bits});\n\
           assert!(crate::__pgrx_c_macros::CPtrDiff::RANK == {ptrdiff_rank});\n\
           assert!(crate::__pgrx_c_macros::CPtrDiff::SIGNED);\n\
         }};\n"
    ))
}

/// Select runtime marker configuration from the verified C identities instead of Rust host defaults.
/// Each string is one rustc `--cfg` operand; build scripts and standalone oracle compilers share
/// this mapping so command-line char overrides and platform typedef ranks cannot diverge.
pub fn support_rust_cfg(profile: &CompilationProfile) -> Result<Vec<String>, SupportProfileError> {
    validate_support_profile(profile)?;
    let size = match profile.target.size_type {
        IntegerKind::UnsignedInt => "unsigned_int",
        IntegerKind::UnsignedLong => "unsigned_long",
        IntegerKind::UnsignedLongLong => "unsigned_long_long",
        _ => unreachable!("the support gate admitted only modeled unsigned size_t identities"),
    };
    let ptrdiff = match profile.target.ptrdiff_type {
        IntegerKind::Int => "int",
        IntegerKind::Long => "long",
        IntegerKind::LongLong => "long_long",
        _ => unreachable!(
            "the support gate admitted only modeled signed pointer difference identities"
        ),
    };
    let mut cfg = vec![
        if profile.target.char_is_signed { "pgrx_c_char_signed" } else { "pgrx_c_char_unsigned" }
            .into(),
        format!("pgrx_c_size_type={size:?}"),
        format!("pgrx_c_ptrdiff_type={ptrdiff:?}"),
    ];
    if !profile.target.preferred_alignments.is_empty() {
        cfg.push("pgrx_c_alignment".into());
        if !profile.target.integers.contains_key(&IntegerKind::Int128) {
            cfg.push("pgrx_c_int128_unavailable".into());
        }
    }
    Ok(cfg)
}

/// Publish original C type-operator alignment separately from Rust's storage ABI alignment.
///
/// Each marker constant contains its scalar and array alignment. Frontend inspection proves
/// both with the original compiler, including nested arrays. An empty map belongs only to
/// synthetic profiles without those proofs; it leaves runtime storage fallbacks selected.
pub fn support_alignment_profile(
    profile: &CompilationProfile,
) -> Result<String, SupportProfileError> {
    validate_support_profile(profile)?;
    if profile.target.preferred_alignments.is_empty() {
        return Ok(String::new());
    }
    let mut rust = String::from(
        "/// Original compiler type-operator alignments; these may exceed record storage alignment.\n\
         #[doc(hidden)]\n\
         #[allow(non_upper_case_globals)]\n\
         pub mod __pgrx_c_alignment {\n",
    );
    for (marker, _) in crate::frontend::ALIGNMENT_PROBES {
        if let Some(alignment) = profile.target.preferred_alignments.get(*marker) {
            rust.push_str(&format!(
                "/// Verified C scalar and array alignment for {marker}, in bytes.\n\
             pub const {marker}: (usize, usize) = ({}, {});\n",
                alignment.scalar, alignment.array,
            ));
        }
    }
    for marker in ["CInt128", "CUnsignedInt128"] {
        if !profile.target.preferred_alignments.contains_key(marker) {
            rust.push_str(&format!(
                "/// Unavailable C identity: zero sentinels reject runtime values and alignment queries.\n\
                 pub const {marker}: (usize, usize) = (0, 0);\n",
            ));
        }
    }
    rust.push_str("}\n");
    Ok(rust)
}

/// Generate numeric input adapters for pg-sys's own opaque integer types.
///
/// The C declarations establish each identity independently. Rust's `Oid` and
/// `TransactionId` expose exactly 32 bits of numeric storage; no adapter is
/// emitted unless that storage matches the inspected C typedef. Datum's pointer
/// storage uses exposed provenance when crossing its C integer representation.
pub fn pg_sys_integer_bridges(frontend: &FrontendOutput) -> Result<String, SupportProfileError> {
    let mut rust = support_abi_assertions(frontend.profile())?;
    rust.push_str("/// C integer identities established from this installation's typedef declarations.\n#[doc(hidden)]\n#[allow(non_camel_case_types)]\npub mod __pgrx_c_types {\n");
    for (name, ty) in &frontend.declarations().types {
        let TypeCategory::Integer(kind) = ty.category else { continue };
        // C type spellings with spaces or declarator syntax are not typedef
        // identifiers and cannot form a Rust alias. Share the emitter's checks
        // for names, including special tokens that raw identifiers cannot fix.
        if crate::emit::rust_identifier(name).is_none() {
            continue;
        }
        let Some(layout) = frontend.profile().target.integers.get(&kind) else { continue };
        if ty.size != Some(u64::from(layout.bits / 8)) || ty.is_const || ty.is_volatile {
            continue;
        }
        rust.push_str(&format!(
            "/// Preserve the compiler-established rank and width of C `{name}`.\npub type r#{name} = crate::__pgrx_c_macros::{};\n",
            crate::emit::marker(kind),
        ));
    }
    rust.push_str("}\n");
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
                kind @ (IntegerKind::UnsignedInt
                | IntegerKind::UnsignedLong
                | IntegerKind::UnsignedLongLong),
            ) if ty.size == Some(u64::from(frontend.profile().target.pointer_bits / 8))
                && ty.alignment == Some(u64::from(frontend.profile().target.pointer_bits / 8))
                && !ty.is_const
                && !ty.is_volatile =>
            {
                Some(kind)
            }
            _ => None,
        })
        .ok_or_else(|| SupportProfileError::PgSysDatumType(datum.cloned()))?;
    let marker = match datum_kind {
        IntegerKind::UnsignedInt => "CUnsignedInt",
        IntegerKind::UnsignedLong => "CUnsignedLong",
        IntegerKind::UnsignedLongLong => "CUnsignedLongLong",
        _ => unreachable!(
            "the Datum representation gate admits only pointer-width unsigned integers"
        ),
    };
    let pointer_bytes = frontend.profile().target.pointer_bits / 8;
    let repr = if pointer_bytes == 4 { "u32" } else { "u64" };
    rust.push_str(&format!(
        "// C Datum is an integer that can retain an exposed Rust pointer's provenance.\n\
         const _: () = {{ assert!(::core::mem::size_of::<crate::Datum>() == {pointer_bytes}); assert!(::core::mem::align_of::<crate::Datum>() == {pointer_bytes}); }};\n\
         impl crate::__pgrx_c_macros::sealed::Sealed for crate::Datum {{}}\n\
         impl crate::__pgrx_c_macros::IntoCValue for crate::Datum {{\n\
           type Kind = crate::__pgrx_c_macros::{marker};\n\
           fn into_c_value(self) -> crate::__pgrx_c_macros::CValue<Self::Kind> {{ crate::__pgrx_c_macros::CValue::new(self.cast_mut_ptr::<()>().expose_provenance() as {repr}) }}\n\
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

/// Map the inspected C triple to the bounded Rust target guard supported by runtime helpers.
fn rust_target(
    profile: &CompilationProfile,
) -> Result<(&'static str, &'static str), SupportProfileError> {
    let triple = &profile.target.triple;
    let mut components = triple.split('-');
    let component = components.next().unwrap_or_default();
    let arch = match component {
        "x86_64" => "x86_64",
        "i386" | "i486" | "i586" | "i686" => "x86",
        "aarch64" | "aarch64_be" | "arm64" => "aarch64",
        "powerpc" | "powerpcle" => "powerpc",
        "powerpc64" | "powerpc64le" => "powerpc64",
        "mips" | "mipsel" => "mips",
        "mips64" | "mips64el" => "mips64",
        "s390x" => "s390x",
        "sparc" => "sparc",
        "sparcv9" | "sparc64" => "sparc64",
        "loongarch64" => "loongarch64",
        name if name.starts_with("arm") || name.starts_with("thumb") => "arm",
        name if name.starts_with("riscv32") => "riscv32",
        name if name.starts_with("riscv64") => "riscv64",
        _ => return Err(SupportProfileError::TargetFamily(triple.clone())),
    };
    let components = components.collect::<Vec<_>>();
    let os = if components.iter().any(|os| os.starts_with("android")) {
        "android"
    } else if components.iter().any(|os| matches!(*os, "windows" | "win32" | "mingw32")) {
        "windows"
    } else if components.first() == Some(&"apple")
        && components.get(1).is_some_and(|os| os.starts_with("darwin") || os.starts_with("macos"))
    {
        "macos"
    } else {
        [
            "linux",
            "freebsd",
            "netbsd",
            "openbsd",
            "dragonfly",
            "illumos",
            "solaris",
            "haiku",
            "hurd",
            "aix",
            "fuchsia",
            "redox",
        ]
        .into_iter()
        .find(|os| components.iter().any(|component| component.starts_with(os)))
        .ok_or_else(|| SupportProfileError::TargetFamily(triple.clone()))?
    };
    Ok((arch, os))
}

/// Exercise this phase’s semantic boundaries with owned fixtures.
/// These regressions check accepted proofs and explicit refusals without changing production
/// headers or weakening the C identity and evaluation contracts.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BuildInputs, CompilerIdentity, TargetFacts};
    use std::collections::BTreeMap;

    /// Construct a reviewed C target profile for support gates without depending on the host
    /// compiler.
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
                size_type: IntegerKind::UnsignedLong,
                ptrdiff_type: IntegerKind::Long,
                preferred_alignments: Default::default(),
                arm_float_abi: None,
                ppc64_elf_abi: None,
                offsetof_supported: true,
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

    /// Checks pg-sys bridges require the original typedef identity and storage.
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
        for name in ["Oid", "TransactionId", "_", "type"] {
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
        // Valid C typedefs can be Rust keywords or the unusable standalone underscore.
        // Parse the complete generated source so alias spelling failures are caught here.
        syn::parse_file(&rust).expect("integer bridges must remain valid Rust syntax");
        assert!(rust.contains("pub type r#type ="));
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

    /// Checks rejects erased identity mismatches and unmodeled policies.
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

    /// Checks size type gate keeps same width integer ranks distinct.
    #[test]
    fn size_type_gate_keeps_same_width_integer_ranks_distinct() {
        let mut profile = profile();
        profile.target.offsetof_supported = false;
        assert!(validate_support_profile(&profile).is_ok());
        profile.target.size_type = IntegerKind::UnsignedLongLong;
        validate_support_profile(&profile).unwrap();
        assert!(
            support_rust_cfg(&profile)
                .unwrap()
                .contains(&"pgrx_c_size_type=\"unsigned_long_long\"".into())
        );
        profile.target.size_type = IntegerKind::UnsignedInt;
        assert_eq!(
            validate_support_profile(&profile),
            Err(SupportProfileError::SizeType(IntegerKind::UnsignedInt))
        );
    }

    /// Checks downstream assertions follow inspected target not host.
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
        for kind in [IntegerKind::Long, IntegerKind::UnsignedLong] {
            profile.target.integers.get_mut(&kind).unwrap().bits = 32;
        }
        profile.target.size_type = IntegerKind::UnsignedLongLong;
        profile.target.ptrdiff_type = IntegerKind::LongLong;
        let windows = support_abi_assertions(&profile).unwrap();
        assert!(windows.contains("target_os = \"windows\""));
        assert!(windows.contains("CLong::BITS == 32"));
        assert!(windows.contains("CSize::RANK == 5"));
        profile.target.triple = "unmodeled-unknown-linux-gnu".into();
        assert!(matches!(
            validate_support_profile(&profile),
            Err(SupportProfileError::TargetFamily(_))
        ));
    }
    /// Admit ILP32 and big-endian layouts from facts while preserving signedness/rank invariants.
    #[test]
    fn verified_layouts_admit_unsigned_char_ilp32_and_big_endian() {
        let mut profile = profile();
        profile.target.char_is_signed = false;
        profile.target.integers.get_mut(&IntegerKind::Char).unwrap().signed = false;
        profile.target.triple = "powerpc64-unknown-linux-gnu".into();
        profile.target.byte_order = ByteOrder::Big;
        let guard = support_abi_assertions(&profile).unwrap();
        assert!(guard.contains("target_endian = \"big\""));
        assert!(guard.contains("CChar::SIGNED == false"));
        assert!(support_rust_cfg(&profile).unwrap().contains(&"pgrx_c_char_unsigned".into()));
        profile.target.triple = "i686-unknown-openbsd".into();
        profile.target.pointer_bits = 32;
        profile.target.byte_order = ByteOrder::Little;
        profile.target.function_pointer = crate::PointerLayout { size: 4, alignment: 4 };
        for kind in [IntegerKind::Long, IntegerKind::UnsignedLong] {
            profile.target.integers.get_mut(&kind).unwrap().bits = 32;
        }
        profile.target.integers.remove(&IntegerKind::Int128);
        profile.target.integers.remove(&IntegerKind::UnsignedInt128);
        // OpenBSD retains long rank for pointer-width typedefs even on ILP32.
        let guard = support_abi_assertions(&profile).unwrap();
        assert!(guard.contains("target_pointer_width = \"32\""));
        assert!(guard.contains("CSize::RANK == 4"));
        assert!(guard.contains("CPtrDiff::RANK == 4"));
        profile.target.ptrdiff_type = IntegerKind::LongLong;
        assert_eq!(
            validate_support_profile(&profile),
            Err(SupportProfileError::PtrDiffType(IntegerKind::LongLong))
        );
    }

    /// Complete compiler alignment profiles configure all markers while absent extensions remain
    /// explicit unavailability sentinels instead of guessed Rust layouts.
    #[test]
    fn alignment_profiles_require_all_fundamental_witnesses() {
        let mut profile = profile();
        assert!(support_alignment_profile(&profile).unwrap().is_empty());
        assert!(!support_rust_cfg(&profile).unwrap().contains(&"pgrx_c_alignment".into()));
        for (marker, _) in crate::frontend::ALIGNMENT_PROBES {
            profile
                .target
                .preferred_alignments
                .insert((*marker).into(), crate::PreferredAlignment { scalar: 8, array: 8 });
        }
        assert!(support_rust_cfg(&profile).unwrap().contains(&"pgrx_c_alignment".into()));
        assert!(support_alignment_profile(&profile).unwrap().contains("pub const CFloat64"));
        profile.target.preferred_alignments.remove("CLongLong");
        assert_eq!(
            validate_support_profile(&profile),
            Err(SupportProfileError::AlignmentFacts("CLongLong"))
        );
        profile
            .target
            .preferred_alignments
            .insert("CLongLong".into(), crate::PreferredAlignment { scalar: 8, array: 8 });
        for kind in [IntegerKind::Int128, IntegerKind::UnsignedInt128] {
            profile.target.integers.remove(&kind);
        }
        for marker in ["CInt128", "CUnsignedInt128"] {
            profile.target.preferred_alignments.remove(marker);
        }
        let rust = support_alignment_profile(&profile).unwrap();
        assert!(rust.contains("pub const CInt128: (usize, usize) = (0, 0)"));
        assert!(rust.contains("pub const CUnsignedInt128: (usize, usize) = (0, 0)"));
    }
    /// Keep typedef aliases and Datum provenance bridges tied to LLP64's actual C integer ranks.
    #[test]
    fn windows_typedef_aliases_preserve_long_long_identity() {
        use crate::{DeclarationCatalog, MacroEnvironment, MacroInventory};
        let environment = MacroEnvironment::default();
        let dependencies = crate::MacroDependencyGraph::from_environment(&environment);
        let mut windows = profile();
        windows.target.triple = "x86_64-pc-windows-msvc".into();
        for kind in [IntegerKind::Long, IntegerKind::UnsignedLong] {
            windows.target.integers.get_mut(&kind).unwrap().bits = 32;
        }
        windows.target.size_type = IntegerKind::UnsignedLongLong;
        windows.target.ptrdiff_type = IntegerKind::LongLong;
        let mut frontend = FrontendOutput {
            profile: windows,
            environment,
            declarations: DeclarationCatalog::default(),
            inventory: MacroInventory { macros: Vec::new(), diagnostics: Vec::new() },
            dependencies,
        };
        for (name, kind, size) in [
            ("Oid", IntegerKind::UnsignedInt, 4),
            ("TransactionId", IntegerKind::UnsignedInt, 4),
            ("size_t", IntegerKind::UnsignedLongLong, 8),
            ("uintptr_t", IntegerKind::UnsignedLongLong, 8),
            ("ptrdiff_t", IntegerKind::LongLong, 8),
            ("Datum", IntegerKind::UnsignedLongLong, 8),
        ] {
            frontend.declarations.types.insert(
                name.into(),
                TypeInfo {
                    spelling: name.into(),
                    canonical_spelling: match kind {
                        IntegerKind::UnsignedInt => "unsigned int",
                        IntegerKind::UnsignedLongLong => "unsigned long long",
                        IntegerKind::LongLong => "long long",
                        _ => unreachable!("fixture table has three reviewed integer identities"),
                    }
                    .into(),
                    category: TypeCategory::Integer(kind),
                    size: Some(size),
                    alignment: Some(size),
                    is_const: false,
                    is_volatile: false,
                },
            );
        }
        let rust = pg_sys_integer_bridges(&frontend).unwrap();
        assert!(rust.contains("pub type r#size_t = crate::__pgrx_c_macros::CUnsignedLongLong;"));
        assert!(rust.contains("pub type r#uintptr_t = crate::__pgrx_c_macros::CUnsignedLongLong;"));
        assert!(rust.contains("pub type r#ptrdiff_t = crate::__pgrx_c_macros::CLongLong;"));
        assert!(rust.contains("type Kind = crate::__pgrx_c_macros::CUnsignedLongLong;"));
        assert!(rust.contains("with_exposed_provenance_mut"));
    }
}
