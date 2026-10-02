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
    pub char_bits: u32,
    pub char_is_signed: bool,
    /// Both compiler and libclang verified the supported ASCII characters and basic escapes.
    pub ascii_execution_charset: bool,
    pub byte_order: ByteOrder,
    pub c_standard: Option<u64>,
    pub integers: BTreeMap<IntegerKind, IntegerType>,
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
