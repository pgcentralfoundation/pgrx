//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use crate::{MacroDefinition, MacroDependencyGraph, MacroInventory, SourceSpan};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// An inspection whose profile, macro environment, and declarations were resolved together.
#[derive(Clone, Debug, Serialize)]
pub struct FrontendOutput {
    pub(crate) profile: CompilationProfile,
    pub(crate) environment: MacroEnvironment,
    pub(crate) declarations: DeclarationCatalog,
    pub(crate) inventory: MacroInventory,
    #[serde(skip)]
    pub(crate) dependencies: MacroDependencyGraph,
}

impl FrontendOutput {
    pub fn profile(&self) -> &CompilationProfile {
        &self.profile
    }

    pub fn environment(&self) -> &MacroEnvironment {
        &self.environment
    }

    pub fn declarations(&self) -> &DeclarationCatalog {
        &self.declarations
    }

    pub fn inventory(&self) -> &MacroInventory {
        &self.inventory
    }

    /// Conservative lexical macro references in the final active environment.
    pub fn dependencies(&self) -> &MacroDependencyGraph {
        &self.dependencies
    }
}

/// Effective C compiler inputs and the target facts verified during inspection.
#[derive(Clone, Debug, Serialize)]
pub struct CompilationProfile {
    pub header: PathBuf,
    pub compiler: CompilerIdentity,
    pub arguments: Vec<String>,
    pub target: TargetFacts,
    pub signed_overflow: SignedOverflow,
    /// Semantic options this analysis does not yet model.
    pub unsupported_options: Vec<String>,
    pub inputs: BuildInputs,
}

impl CompilationProfile {
    /// Prove the representation and ordinary conversions of binary32/binary64 values.
    ///
    /// This does not prove that a composed arithmetic expression is unaffected by
    /// floating-point contraction. Native arguments and results are C value
    /// boundaries; source pragmas and a changed runtime floating environment are
    /// outside this recorded profile.
    pub fn supports_float_values(&self) -> Result<(), String> {
        self.target.supports_float_values()
    }

    /// Verify binary32/binary64 arithmetic with the default floating-point environment.
    /// Caller changes to rounding, exception masks, or denormal handling are outside
    /// this contract, as are source pragmas that override the recorded compiler mode.
    pub fn supports_float_expressions(&self) -> Result<(), String> {
        self.target.supports_float_expressions()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CompilerIdentity {
    pub executable: PathBuf,
    pub version: String,
    pub libclang_version: String,
}

/// Compiler input dependencies, including searches whose result can change after file creation.
#[derive(Clone, Debug, Default, Serialize)]
pub struct BuildInputs {
    /// Working directory for relative compiler arguments.
    pub current_directory: PathBuf,
    pub files: Vec<PathBuf>,
    /// Content identity at inspection; None records a tracked path that was absent.
    pub fingerprints: BTreeMap<PathBuf, Option<String>>,
    /// Header, resource and loaded-input search directories, including absent roots.
    pub directories: Vec<PathBuf>,
    /// Executable lookup roots from PATH, separate from header searches.
    ///
    /// Build integration must track these roots or reject searches overlapping its
    /// own output, because creating a higher-priority executable changes selection.
    pub executable_search_directories: Vec<PathBuf>,
    pub environment: BTreeMap<String, Option<String>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TargetFacts {
    pub triple: String,
    pub pointer_bits: u32,
    pub function_pointer: PointerLayout,
    pub char_bits: u32,
    pub char_is_signed: bool,
    /// Both compiler and libclang verified the supported ASCII characters and basic escapes.
    pub ascii_execution_charset: bool,
    pub byte_order: ByteOrder,
    pub c_standard: Option<u64>,
    pub integers: BTreeMap<IntegerKind, IntegerType>,
    pub floating_point: FloatingPointFacts,
}

/// Layout of a native C function pointer, verified by libclang and the driver.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct PointerLayout {
    pub size: u64,
    pub alignment: u64,
}

impl TargetFacts {
    pub fn supports_float_values(&self) -> Result<(), String> {
        if self.char_bits != 8 {
            return Err("floating-point lowering requires eight-bit C bytes".into());
        }
        self.floating_point.supports_float_values()
    }

    pub fn supports_float_expressions(&self) -> Result<(), String> {
        if self.char_bits != 8 {
            return Err("floating-point lowering requires eight-bit C bytes".into());
        }
        self.floating_point.supports_float_expressions()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FloatingKind {
    Float,
    Double,
    LongDouble,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FloatingType {
    pub storage_bits: u32,
    pub alignment: Option<u64>,
    pub radix: u32,
    pub mantissa_digits: u32,
    pub min_exponent: i32,
    pub max_exponent: i32,
    pub has_subnormals: bool,
    pub has_infinity: bool,
    pub has_quiet_nan: bool,
}

/// Agreed compiler facts, including modes that cannot use Rust's ordinary float operators.
/// The default records no proof and therefore rejects floating-point lowering.
#[derive(Clone, Debug, Default, Serialize)]
pub struct FloatingPointFacts {
    pub types: BTreeMap<FloatingKind, FloatingType>,
    pub evaluation_method: Option<i32>,
    pub fast_math: bool,
    pub finite_math_only: bool,
    pub effective_options: Vec<String>,
    pub unsupported_options: Vec<String>,
}

impl FloatingPointFacts {
    pub fn supports_float_values(&self) -> Result<(), String> {
        for (kind, storage_bits, mantissa_digits, min_exponent, max_exponent) in
            [(FloatingKind::Float, 32, 24, -125, 128), (FloatingKind::Double, 64, 53, -1021, 1024)]
        {
            let Some(ty) = self.types.get(&kind) else {
                return Err(format!("compiler floating-point facts for {kind:?} are unavailable"));
            };
            if (ty.storage_bits, ty.radix, ty.mantissa_digits, ty.min_exponent, ty.max_exponent)
                != (storage_bits, 2, mantissa_digits, min_exponent, max_exponent)
                || !ty.has_subnormals
                || !ty.has_infinity
                || !ty.has_quiet_nan
            {
                return Err(format!(
                    "{kind:?} does not have the supported IEEE binary representation"
                ));
            }
        }
        if self.evaluation_method != Some(0) {
            return Err(format!(
                "floating-point evaluation method {:?} requires unsupported excess precision",
                self.evaluation_method
            ));
        }
        if self.fast_math || self.finite_math_only {
            return Err(
                "compiler fast-math or finite-math-only mode changes floating-point semantics"
                    .into(),
            );
        }
        let unsupported = self
            .unsupported_options
            .iter()
            .filter(|option| option.as_str() != "-ffp-contract=on")
            .collect::<Vec<_>>();
        if !unsupported.is_empty() {
            return Err(format!(
                "floating-point semantic options are unsupported: {}",
                unsupported.iter().map(|option| option.as_str()).collect::<Vec<_>>().join(", ")
            ));
        }
        Ok(())
    }

    pub fn supports_float_expressions(&self) -> Result<(), String> {
        self.supports_float_values()?;
        if !self.effective_options.iter().any(|option| option == "-ffp-contract=off") {
            return Err(format!(
                "floating-point contraction is not established as disabled: {}",
                self.effective_options
                    .iter()
                    .filter(|option| option.starts_with("-ffp-contract="))
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ByteOrder {
    Little,
    Big,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignedOverflow {
    Undefined,
    Wrapping,
    Trapping,
}

/// C type identity is retained even when several types have the same Rust representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegerKind {
    Bool,
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Int128,
    UnsignedInt128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct IntegerType {
    pub kind: IntegerKind,
    /// Storage width in C bits; `_Bool`'s value range is still only zero and one.
    pub bits: u32,
    pub signed: bool,
    pub rank: u8,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct MacroEnvironment {
    /// Final active definitions, with object-like and external macros retained as context.
    pub active: BTreeMap<String, ActiveMacro>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ActiveMacro {
    pub definition: MacroDefinition,
    pub provenance: ActiveProvenance,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", content = "spans", rename_all = "snake_case")]
pub enum ActiveProvenance {
    /// One physical definition, or one compiler/command-line definition, matches the final state.
    Resolved,
    /// Matching definition history does not establish which source is active.
    Ambiguous(Vec<SourceSpan>),
    /// The final definition could not be linked to the inspected definition history.
    Unresolved,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct DeclarationCatalog {
    pub types: BTreeMap<String, TypeInfo>,
    /// Compiler-owned integer constants, such as enum constants; never mutable variables.
    pub integer_constants: BTreeMap<String, IntegerConstant>,
    pub variables: BTreeMap<String, TypeInfo>,
    pub functions: BTreeMap<String, TypeInfo>,
    /// Canonical C types and their immediate edges. Cycles are shared by spelling.
    pub type_shapes: BTreeMap<String, TypeShape>,
    /// Complete and incomplete record declarations, keyed by canonical type spelling.
    pub records: BTreeMap<String, RecordInfo>,
    pub function_signatures: BTreeMap<String, FunctionInfo>,
    /// Profile-specific expression types for bitfields, keyed by `record::field`.
    pub bitfields: BTreeMap<String, BitfieldFacts>,
}

/// C bitfield identity and promotion can differ even for one declared base type.
/// These expression types come from typed compiler probes under the same profile.
#[derive(Clone, Debug, Serialize)]
pub struct BitfieldFacts {
    /// An addressable tag or typedef spelling proved by the same C probe.
    pub record_type: String,
    pub promoted: TypeInfo,
    /// Assignment is absent for a const field or an unsupported compiler expression.
    pub assignment: Option<TypeInfo>,
    pub assignment_promoted: Option<TypeInfo>,
    /// Postfix operators can lose the width-sensitive bitfield expression identity.
    pub postfix: Option<TypeInfo>,
    pub postfix_promoted: Option<TypeInfo>,
    /// Same-profile LLVM witnesses agree on volatile access units after alignment narrowing.
    pub unaligned_volatile_access: bool,
}

/// Owned compiler type information without recursively copying referenced types.
#[derive(Clone, Debug, Serialize)]
pub struct TypeShape {
    pub ty: TypeInfo,
    pub is_restrict: bool,
    pub kind: TypeShapeKind,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypeShapeKind {
    Scalar,
    Pointer { pointee: TypeInfo },
    Array { element: TypeInfo, length: Option<u64>, array_kind: ArrayKind },
    Function { signature: FunctionSignature },
    Record { identity: String },
    Enum { underlying: Option<TypeInfo> },
    Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArrayKind {
    Constant,
    Incomplete,
    Variable,
    Dependent,
}

#[derive(Clone, Debug, Serialize)]
pub struct RecordInfo {
    /// Clang's declaration identity; not a guessed Rust binding name.
    pub identity: String,
    pub name: Option<String>,
    pub kind: RecordKind,
    pub fields: Vec<FieldInfo>,
    pub size: Option<u64>,
    pub alignment: Option<u64>,
    pub is_anonymous: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordKind {
    Struct,
    Union,
}

#[derive(Clone, Debug, Serialize)]
pub struct FieldInfo {
    pub name: Option<String>,
    pub ty: TypeInfo,
    pub offset_bits: Option<u64>,
    /// Includes zero-width fields; None means this is not a bit-field.
    pub bit_width: Option<u32>,
    pub is_anonymous: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct FunctionSignature {
    pub result: TypeInfo,
    /// None denotes a C function without a prototype, not a function taking no arguments.
    pub parameters: Option<Vec<TypeInfo>>,
    pub variadic: bool,
    /// The convention exposed by libclang, or None when it cannot establish one.
    pub calling_convention: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FunctionInfo {
    pub signature: FunctionSignature,
    pub linkage: Option<DeclarationLinkage>,
    pub is_static: bool,
    pub is_inline: bool,
    /// A definition exposed in this AST. Skipped function bodies can hide definitions.
    pub definition_available: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeclarationLinkage {
    Automatic,
    Internal,
    External,
    UniqueExternal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TypeInfo {
    pub spelling: String,
    pub canonical_spelling: String,
    pub category: TypeCategory,
    pub size: Option<u64>,
    pub alignment: Option<u64>,
    pub is_const: bool,
    pub is_volatile: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeCategory {
    Integer(IntegerKind),
    Floating,
    Pointer,
    Record,
    Enum,
    Function,
    Void,
    Other,
}

#[derive(Clone, Debug, Serialize)]
pub struct IntegerConstant {
    pub ty: TypeInfo,
    pub value: IntegerValue,
    /// Original literal spelling when the complete object expansion is a literal.
    /// Enum values and compound expressions have no single literal to preserve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub literal: Option<crate::IntegerLiteral>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum IntegerValue {
    Signed(i64),
    Unsigned(u64),
}
