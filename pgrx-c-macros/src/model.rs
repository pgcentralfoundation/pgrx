//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Owned C facts shared by discovery, symbolic analysis, and Rust emission.
//!
//! These types keep compiler identity, target representation, preprocessing provenance, and
//! declaration shape separate from Rust binding storage. Equal-width C types retain their
//! rank and nominal identity, and missing facts remain explicit rather than becoming inferred
//! proofs. FrontendOutput bundles the facts established by one inspection so downstream
//! phases cannot casually mix environments.

use crate::{MacroDefinition, MacroDependencyGraph, MacroInventory, SourceSpan};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// An inspection whose profile, macro environment, and declarations were resolved together.
#[derive(Clone, Debug, Serialize)]
pub struct FrontendOutput {
    /// The inspected compiler flags and target facts under which these observations are valid.
    pub(crate) profile: CompilationProfile,
    /// The final active preprocessing state used by subsequent compiler-owned expansion.
    pub(crate) environment: MacroEnvironment,
    /// Owned C declaration facts used to resolve names and establish semantic identities.
    pub(crate) declarations: DeclarationCatalog,
    /// Discovery history and diagnostics, including definitions no longer active at end of
    /// preprocessing.
    pub(crate) inventory: MacroInventory,
    /// Lexical active-macro and declaration-constant references before batch-specific
    /// augmentation.
    #[serde(skip)]
    pub(crate) dependencies: MacroDependencyGraph,
}

/// Expose the inspection bundle without allowing its compiler facts to be assembled independently.
impl FrontendOutput {
    /// Borrow the verified compiler arguments and target facts paired with this inspection.
    pub fn profile(&self) -> &CompilationProfile {
        &self.profile
    }

    /// Borrow the final active macro state rather than replaying discovery history as if it were
    /// final.
    pub fn environment(&self) -> &MacroEnvironment {
        &self.environment
    }

    /// Borrow the owned C identities and structural facts copied from this inspection.
    pub fn declarations(&self) -> &DeclarationCatalog {
        &self.declarations
    }

    /// Borrow discovery history and diagnostics retained independently of active-state resolution.
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
    /// The original main-file spelling that controls preprocessing and quoted include lookup.
    pub header: PathBuf,
    /// Matched driver and library identities recorded for this coherent inspection.
    pub compiler: CompilerIdentity,
    /// Effective input C flags retained for every expansion, type probe, and native compilation
    /// phase.
    pub arguments: Vec<String>,
    /// Verified C ABI and language facts established by driver/libclang agreement.
    pub target: TargetFacts,
    /// The inspected C signed-overflow policy, retained for helper selection and refusal decisions.
    pub signed_overflow: SignedOverflow,
    /// Semantic options this analysis does not yet model.
    pub unsupported_options: Vec<String>,
    /// Tracked compiler inputs that later phases must verify before reusing these facts.
    pub inputs: BuildInputs,
}

/// Check whether the recorded target/profile supports ordinary Rust floating representations and
/// arithmetic.
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

/// Record both Clang paths so generation can reject version mismatches and explain the chosen
/// compiler.
#[derive(Clone, Debug, Serialize)]
pub struct CompilerIdentity {
    /// Selected Clang executable whose preprocessing and typed probes establish driver
    /// observations.
    pub executable: PathBuf,
    /// The driver version observed during selection, used to match libclang release compatibility.
    pub version: String,
    /// The loaded library version used to check the executable observes the same C frontend family.
    pub libclang_version: String,
}

/// Compiler input dependencies, including searches whose result can change after file creation.
#[derive(Clone, Debug, Default, Serialize)]
pub struct BuildInputs {
    /// Working directory for relative compiler arguments.
    pub current_directory: PathBuf,
    /// Observed or change-sensitive file inputs retained for content fingerprinting and rebuild
    /// invalidation.
    pub files: Vec<PathBuf>,
    /// Content identity at inspection; None records a tracked path that was absent.
    pub fingerprints: BTreeMap<PathBuf, Option<String>>,
    /// Physical input identity at inspection; changing a symlink target invalidates provenance even
    /// when both targets have identical bytes and the old target remains present.
    pub file_identities: BTreeMap<PathBuf, Option<PathBuf>>,
    /// Header, resource and loaded-input search directories, including absent roots.
    pub directories: Vec<PathBuf>,
    /// Legacy executable-directory watches, retained for consumers of this report format.
    /// Compiler lookup now records attempted executable paths in `files`, including absent
    /// predecessors, rather than watching every PATH root. Cargo integrations still need safe
    /// parent-directory watches for absent candidates to detect their later creation.
    pub executable_search_directories: Vec<PathBuf>,
    /// Recorded optional environment values whose changes require fresh inspection.
    pub environment: BTreeMap<String, Option<String>>,
}

/// Agreed C representation and language facts used for promotions, casts, and runtime support gating.
#[derive(Clone, Debug, Serialize)]
pub struct TargetFacts {
    /// Effective C target triple agreed by the executable and libclang frontend.
    pub triple: String,
    /// C object-pointer width used by support profile and integer-pointer conversion gates.
    pub pointer_bits: u32,
    /// Verified native function-pointer size/alignment, separate from object-pointer assumptions.
    pub function_pointer: PointerLayout,
    /// Canonical unsigned C identity of `sizeof` and `_Alignof` results.
    pub size_type: IntegerKind,
    /// Canonical signed C identity of pointer subtraction, verified independently of its width.
    pub ptrdiff_type: IntegerKind,
    /// Original C `_Alignof` results for scalar and array types, separate from storage ABI alignment.
    pub preferred_alignments: BTreeMap<String, PreferredAlignment>,
    /// ARM EABI float argument convention from protected compiler predefines, when applicable.
    pub arm_float_abi: Option<ArmFloatAbi>,
    /// PowerPC64 ELF function representation from the protected `_CALL_ELF` compiler witness.
    pub ppc64_elf_abi: Option<Ppc64ElfAbi>,
    /// The intrinsic's result identity and direct/nested/array offsets agree with Clang.
    pub offsetof_supported: bool,
    /// Bits in one C byte; supported helpers require eight-bit storage units.
    pub char_bits: u32,
    /// Implementation-selected signedness of plain char, retained independently from signed char.
    pub char_is_signed: bool,
    /// Both compiler and libclang verified the supported ASCII characters and basic escapes.
    pub ascii_execution_charset: bool,
    /// Agreed C endian representation used for downstream target guards and storage operations.
    pub byte_order: ByteOrder,
    /// Effective __STDC_VERSION__ value, when present, used for version-dependent syntax such as
    /// binary literals.
    pub c_standard: Option<u64>,
    /// Representation facts keyed by distinct fundamental C integer identities.
    pub integers: BTreeMap<IntegerKind, IntegerType>,
    /// Agreed float representation and semantic modes used to gate ordinary Rust arithmetic.
    pub floating_point: FloatingPointFacts,
}

/// Layout of a native C function pointer, verified by libclang and the driver.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct PointerLayout {
    /// Verified native function-pointer storage size in C bytes.
    pub size: u64,
    /// Verified native function-pointer storage alignment in C bytes.
    pub alignment: u64,
}

/// C type-operator alignment can exceed the ABI alignment reported for record storage.
/// Keeping scalar and array witnesses separate prevents Rust layout queries from silently changing
/// original `_Alignof` expressions on targets such as i686 Linux.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct PreferredAlignment {
    /// Original `_Alignof(T)` result for the named fundamental C type.
    pub scalar: u64,
    /// Original `_Alignof(T[2])` result, also checked against a nested array witness.
    pub array: u64,
}

/// ARM's base and VFP procedure-call variants pass float arguments in different registers.
/// The effective compiler predefines, rather than the target triple alone, select this contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ArmFloatAbi {
    /// Base AAPCS uses general-purpose registers, including the softfp instruction mode.
    Base,
    /// AAPCS-VFP uses floating-point registers for eligible arguments.
    Vfp,
}

/// PowerPC64's ELF variants use different function-pointer representations and native call rules.
/// Big-endian Linux GNU and musl select different defaults even with otherwise equal scalar layouts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Ppc64ElfAbi {
    /// ELFv1 function pointers address descriptors rather than direct function entry points.
    V1,
    /// ELFv2 function pointers identify code entry points directly.
    V2,
}

/// Gate floating support on the verified byte representation and C evaluation mode.
impl TargetFacts {
    /// Reject unsupported storage, excess-precision, or semantic modes before accepting ordinary Rust
    /// float values.
    pub fn supports_float_values(&self) -> Result<(), String> {
        if self.char_bits != 8 {
            return Err("floating-point lowering requires eight-bit C bytes".into());
        }
        self.floating_point.supports_float_values()
    }

    /// Require supported float values and explicitly disabled contraction before lowering composed
    /// arithmetic.
    pub fn supports_float_expressions(&self) -> Result<(), String> {
        if self.char_bits != 8 {
            return Err("floating-point lowering requires eight-bit C bytes".into());
        }
        self.floating_point.supports_float_expressions()
    }
}

/// Preserve the distinct fundamental C floating types before checking which representations are
/// supported.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FloatingKind {
    /// The distinct fundamental C float identity checked against binary32 support.
    Float,
    /// The distinct fundamental C double identity.
    Double,
    /// The distinct C long double identity, retained even when ordinary Rust float lowering cannot
    /// support it.
    LongDouble,
}

/// Compiler-reported representation facts needed to prove ordinary Rust float values are compatible.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FloatingType {
    /// Total stored bits for this C floating representation.
    pub storage_bits: u32,
    /// Compiler-reported C storage alignment used when checking native float representation.
    pub alignment: Option<u64>,
    /// Exponent base reported by the compiler; supported float helpers require binary
    /// representations.
    pub radix: u32,
    /// Significand precision used to prove IEEE binary32/binary64 compatibility.
    pub mantissa_digits: u32,
    /// Minimum normalized exponent reported by the C implementation.
    pub min_exponent: i32,
    /// Maximum normalized exponent reported by the C implementation.
    pub max_exponent: i32,
    /// Whether the representation supports the subnormal values ordinary Rust floats preserve.
    pub has_subnormals: bool,
    /// Whether positive/negative infinity belong to the verified C representation.
    pub has_infinity: bool,
    /// Whether quiet NaNs belong to the verified C representation.
    pub has_quiet_nan: bool,
}

/// Agreed compiler facts, including modes that cannot use Rust's ordinary float operators.
/// The default records no proof and therefore rejects floating-point lowering.
#[derive(Clone, Debug, Default, Serialize)]
pub struct FloatingPointFacts {
    /// Representation facts keyed by fundamental C float identity.
    pub types: BTreeMap<FloatingKind, FloatingType>,
    /// Effective intermediate evaluation mode; unsupported excess precision blocks lowering.
    pub evaluation_method: Option<i32>,
    /// Whether compiler fast-math permits semantic changes ordinary helpers cannot model.
    pub fast_math: bool,
    /// Whether compiler assumptions exclude nonfinite values supported by Rust’s ordinary operators.
    pub finite_math_only: bool,
    /// Effective frontend float flags, including contraction policy used to gate composed
    /// expressions.
    pub effective_options: Vec<String>,
    /// Observed semantic options for which the helpers have not established equivalent behavior.
    pub unsupported_options: Vec<String>,
}

/// Reject representations and compiler modes whose semantics ordinary float helpers cannot preserve.
impl FloatingPointFacts {
    /// Reject unsupported storage, excess-precision, or semantic modes before accepting ordinary Rust
    /// float values.
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

    /// Require supported float values and explicitly disabled contraction before lowering composed
    /// arithmetic.
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

/// The agreed C target byte order, independent of the generator host.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ByteOrder {
    /// Least-significant bytes precede more-significant bytes in the agreed C target.
    Little,
    /// Most-significant bytes precede less-significant bytes in the agreed C target.
    Big,
}

/// The inspected compiler policy that selects or rejects signed-arithmetic lowering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignedOverflow {
    /// Signed overflow remains outside the defined C invocation domain.
    Undefined,
    /// Recorded compiler flags require two’s-complement wrapping signed arithmetic.
    Wrapping,
    /// Recorded compiler flags require a trapping policy that runtime support currently rejects.
    Trapping,
}

/// C type identity is retained even when several types have the same Rust representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegerKind {
    /// A distinct boolean representation rather than an arbitrary integer-width alias.
    Bool,
    /// Plain char identity whose signedness is selected by the target implementation.
    Char,
    /// Distinct signed char identity with the lowest ordinary signed integer rank.
    SignedChar,
    /// Distinct unsigned char identity whose range affects ordinary integer promotion.
    UnsignedChar,
    /// Distinct signed short identity used by promotion and usual-conversion rules.
    Short,
    /// Distinct unsigned short identity used by promotion and usual-conversion rules.
    UnsignedShort,
    /// Distinct signed int identity and the ordinary integer-promotion destination.
    Int,
    /// Distinct unsigned int identity; equal-width signed storage is not an interchangeable C type.
    UnsignedInt,
    /// Distinct signed long identity, retaining its rank even on equal-width targets.
    Long,
    /// Distinct unsigned long identity, including the supported sizeof result family.
    UnsignedLong,
    /// Distinct signed long long identity, retaining its higher rank than long.
    LongLong,
    /// Distinct unsigned long long identity, retaining its higher rank than unsigned long.
    UnsignedLongLong,
    /// Compiler extended signed 128-bit identity supported by the modeled integer family.
    Int128,
    /// Compiler extended unsigned 128-bit identity supported by the modeled integer family.
    UnsignedInt128,
}

/// A fundamental C identity with width, signedness, and conversion rank retained independently.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct IntegerType {
    /// Fundamental C identity; equal bit widths never make long and long long interchangeable.
    pub kind: IntegerKind,
    /// Storage width in C bits; `_Bool`'s value range is still only zero and one.
    pub bits: u32,
    /// Whether integer storage interprets its top bit as sign, kept independent of C rank.
    pub signed: bool,
    /// C integer conversion rank, retained even when another identity has the same bit width.
    pub rank: u8,
}

/// The final active preprocessing state, distinct from discovery history that includes replaced
/// definitions.
#[derive(Clone, Debug, Default, Serialize)]
pub struct MacroEnvironment {
    /// Final active definitions, with object-like and external macros retained as context.
    pub active: BTreeMap<String, ActiveMacro>,
}

/// A final active definition and the proof status linking it back to physical discovery provenance.
#[derive(Clone, Debug, Serialize)]
pub struct ActiveMacro {
    /// Final active replacement and original signature selected from preprocessing state.
    pub definition: MacroDefinition,
    /// Physical source origins or their resolution status used for ownership filtering and auditing.
    pub provenance: ActiveProvenance,
}

/// Whether a final active macro has one physical origin, multiple matching origins, or no resolved
/// origin.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", content = "spans", rename_all = "snake_case")]
pub enum ActiveProvenance {
    /// One physical definition, or one compiler/command-line definition, matches the final state.
    Resolved,
    /// Matching definition history does not establish which source is active.
    Ambiguous(
        /// Distinct matching physical definition spans; none is silently chosen as a unique origin.
        Vec<SourceSpan>,
    ),
    /// The final definition could not be linked to the inspected definition history.
    Unresolved,
}

/// Owned C identities, shapes, callable prototypes, and probe facts used to establish emission
/// capabilities.
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeclarationCatalog {
    /// Named C declarations, including typedefs used to retain readable aliases in generated casts.
    pub types: BTreeMap<String, TypeInfo>,
    /// Compiler-owned integer constants, such as enum constants; never mutable variables.
    pub integer_constants: BTreeMap<String, IntegerConstant>,
    /// Original C variable types used to establish native access identity.
    pub variables: BTreeMap<String, TypeInfo>,
    /// Function declaration type facts used for name resolution before complete prototype checks.
    pub functions: BTreeMap<String, TypeInfo>,
    /// Canonical C types and their immediate edges. Cycles are shared by spelling.
    pub type_shapes: BTreeMap<String, TypeShape>,
    /// Complete and incomplete record declarations, keyed by canonical type spelling.
    pub records: BTreeMap<String, RecordInfo>,
    /// Complete callable metadata used to verify external and inline native adapters.
    pub function_signatures: BTreeMap<String, FunctionInfo>,
    /// Compiler-owned operations proved under this profile, separate from native functions.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub builtins: BTreeMap<String, BuiltinInfo>,
    /// Referenced compiler operations whose type or expression semantics were not established.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub builtin_unavailable: BTreeMap<String, String>,
    /// Profile-specific expression types for bitfields, keyed by `record::field`.
    pub bitfields: BTreeMap<String, BitfieldFacts>,
}

/// C bitfield identity and promotion can differ even for one declared base type.
/// These expression types come from typed compiler probes under the same profile.
#[derive(Clone, Debug, Serialize)]
pub struct BitfieldFacts {
    /// An addressable tag or typedef spelling proved by the same C probe.
    pub record_type: String,
    /// Compiler-established result type after unary-plus integer promotion of the field expression.
    pub promoted: TypeInfo,
    /// Assignment is absent for a const field or an unsupported compiler expression.
    pub assignment: Option<TypeInfo>,
    /// Compiler-established promoted type of an assignment result, when writing is supported.
    pub assignment_promoted: Option<TypeInfo>,
    /// Postfix operators can lose the width-sensitive bitfield expression identity.
    pub postfix: Option<TypeInfo>,
    /// Compiler-established promoted type of a postfix update result, when writing is supported.
    pub postfix_promoted: Option<TypeInfo>,
    /// Same-profile LLVM witnesses agree on volatile access units after alignment narrowing.
    pub unaligned_volatile_access: bool,
}

/// Owned compiler type information without recursively copying referenced types.
#[derive(Clone, Debug, Serialize)]
pub struct TypeShape {
    /// Canonical C identity, representation, and qualifiers of the enclosing structural shape.
    pub ty: TypeInfo,
    /// Restrict qualification copied separately from ordinary type spelling for capability
    /// validation.
    pub is_restrict: bool,
    /// Structural edges that preserve pointee, element, record, and callable relationships.
    pub kind: TypeShapeKind,
}

/// Structural C type edges that storage-width facts alone cannot express.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypeShapeKind {
    /// A fundamental nonaggregate C shape with no further structural type edges.
    Scalar,
    /// A pointer representation retaining its pointee and layer-specific qualifiers.
    Pointer {
        /// C pointee facts whose nested identity and qualifiers are preserved independently.
        pointee: TypeInfo,
    },
    /// An element relationship with its length and bound category retained separately.
    Array {
        /// The array element type, retained separately from length and enclosing layout.
        element: TypeInfo,
        /// The fixed element count when known; absent C bounds remain explicit.
        length: Option<u64>,
        /// The C bound category used to reject incomplete or variable layout assumptions.
        array_kind: ArrayKind,
    },
    /// Complete or explicitly missing C prototype facts behind a function type.
    Function {
        /// Complete C function prototype, including whether its parameters were established.
        signature: FunctionSignature,
    },
    /// A nominal struct/union identity whose field/layout catalog determines native capabilities.
    Record {
        /// The nominal C declaration identity retained even when storage layouts coincide.
        identity: String,
    },
    /// An original C enum identity whose underlying representation and Rust-valid values need
    /// separate checks.
    Enum {
        /// The compiler-selected integer representation of an enum, if established.
        underlying: Option<TypeInfo>,
    },
    /// A C structural form lacking enough modeled facts for native capability generation.
    Unsupported,
}

/// Retain the C array bound category so incomplete and variable layouts are never treated as fixed
/// storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArrayKind {
    /// A C array with a compile-time fixed element count.
    Constant,
    /// A C array with no established bound, including flexible/incomplete storage contexts.
    Incomplete,
    /// A runtime-bound C array whose layout cannot be frozen as a fixed Rust array.
    Variable,
    /// An unresolved dependent array bound outside the ordinary supported C layout family.
    Dependent,
}

/// A nominal C record and physical field layout, copied while the translation unit is live.
#[derive(Clone, Debug, Serialize)]
pub struct RecordInfo {
    /// Clang's declaration identity; not a guessed Rust binding name.
    pub identity: String,
    /// Original record tag when present; anonymous records retain nominal identity separately.
    pub name: Option<String>,
    /// The semantic or structural category kept separate from representation and source spelling.
    pub kind: RecordKind,
    /// Original C members in declaration order, including anonymous-member placement facts.
    pub fields: Vec<FieldInfo>,
    /// Known storage size in C bytes; absence prevents pretending an incomplete type has a layout.
    pub size: Option<u64>,
    /// Required storage alignment when known, retained for ABI and access checks.
    pub alignment: Option<u64>,
    /// Whether this declaration/member has no ordinary name and needs structural projection
    /// reasoning.
    pub is_anonymous: bool,
}

/// Keep C struct and union storage rules distinct for binding and field adapters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordKind {
    /// C struct layout with distinct member storage locations.
    Struct,
    /// C union layout with overlapping member storage and separate validity obligations.
    Union,
}

/// Original C member identity, type, and physical offset used to verify generated Rust projections.
#[derive(Clone, Debug, Serialize)]
pub struct FieldInfo {
    /// Original C member name, absent for anonymous member projections.
    pub name: Option<String>,
    /// Original C field type with qualifiers needed to distinguish reads, writes, and volatile
    /// operations.
    pub ty: TypeInfo,
    /// Physical bit offset copied from Clang; absence prevents unchecked field projection.
    pub offset_bits: Option<u64>,
    /// Includes zero-width fields; None means this is not a bit-field.
    pub bit_width: Option<u32>,
    /// Whether this declaration/member has no ordinary name and needs structural projection
    /// reasoning.
    pub is_anonymous: bool,
}

/// A copied C prototype retaining argument/result identities and calling convention for native call
/// verification.
#[derive(Clone, Debug, Serialize)]
pub struct FunctionSignature {
    /// Original C result identity checked independently from Rust return storage.
    pub result: TypeInfo,
    /// None denotes a C function without a prototype, not a function taking no arguments.
    pub parameters: Option<Vec<TypeInfo>>,
    /// Whether the callable has an open argument tail that requires special handling or refusal.
    pub variadic: bool,
    /// The convention exposed by libclang, or None when it cannot establish one.
    pub calling_convention: Option<String>,
}

/// The bounded compiler-operation family for which separate signature and effect witnesses exist.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BuiltinKind {
    /// A proved integer byte-swap operation with a fixed supported operand width.
    ByteSwap {
        /// Fixed integer width proved for the builtin operation, independent of host Rust type
        /// aliases.
        bits: u16,
    },
    /// The compiler’s first-operand-preserving branch hint operation proved by signature and LLVM
    /// witnesses.
    Expect,
}

/// A direct builtin call has this verified C prototype and pure operation.
#[derive(Clone, Debug, Serialize)]
pub struct BuiltinInfo {
    /// Reviewed builtin operation whose signature and effect witnesses passed.
    pub kind: BuiltinKind,
    /// Compiler-established operation prototype; it is not a native addressable function declaration.
    pub signature: FunctionSignature,
}

/// Prototype and linkage facts that determine whether a declaration can receive a native function
/// adapter.
#[derive(Clone, Debug, Serialize)]
pub struct FunctionInfo {
    /// The complete copied C prototype required for identity-aware native calls.
    pub signature: FunctionSignature,
    /// C linkage facts used to select an external symbol or require an original local definition.
    pub linkage: Option<DeclarationLinkage>,
    /// Whether C storage class makes the declaration local to its translation unit.
    pub is_static: bool,
    /// Whether the original function permits inline-definition support rather than an ordinary
    /// foreign symbol.
    pub is_inline: bool,
    /// A definition exposed in this AST. Skipped function bodies can hide definitions.
    pub definition_available: bool,
}

/// C declaration linkage used to choose external calls or verified local-definition adapters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeclarationLinkage {
    /// A declaration with no external linkage, such as automatic local storage.
    Automatic,
    /// A translation-unit-local C declaration requiring original-source definition handling.
    Internal,
    /// A normal externally linked C symbol eligible for verified foreign binding adapters.
    External,
    /// Compiler-reported unique external linkage retained without treating it as ordinary external
    /// proof.
    UniqueExternal,
}

/// Original and canonical C spelling with representation and qualifiers for identity-aware lowering.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TypeInfo {
    /// Original C spelling used for readable casts and named-declaration lookup.
    pub spelling: String,
    /// Compiler canonical C spelling used for identity lookup without erasing qualifiers or rank.
    pub canonical_spelling: String,
    /// Broad C type family used to dispatch supported semantic operations.
    pub category: TypeCategory,
    /// Known storage size in C bytes; absence prevents pretending an incomplete type has a layout.
    pub size: Option<u64>,
    /// Required storage alignment when known, retained for ABI and access checks.
    pub alignment: Option<u64>,
    /// The qualifier on this C layer, retained to restrict writes and type compatibility.
    pub is_const: bool,
    /// The qualifier on this C layer, requiring accesses to preserve volatile operation semantics.
    pub is_volatile: bool,
}

/// Classify broad C operations without replacing nominal type or structural facts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeCategory {
    /// Fixed-width Rust storage or a fundamental C integer category, as identified by the enclosing
    /// catalog.
    Integer(
        /// Fixed-width Rust storage or a fundamental C integer category, as identified by the
        /// enclosing catalog.
        IntegerKind,
    ),
    /// A fundamental C floating category subject to representation and compiler-mode gates.
    Floating,
    /// A pointer representation retaining its pointee and layer-specific qualifiers.
    Pointer,
    /// A nominal struct/union identity whose field/layout catalog determines native capabilities.
    Record,
    /// An original C enum identity whose underlying representation and Rust-valid values need
    /// separate checks.
    Enum,
    /// A callable type with prototype/ABI facts, distinct from an addressable function binding.
    Function,
    /// C void identity, which cannot become ordinary value storage.
    Void,
    /// A C category not covered by the bounded scalar/pointer/record/function model.
    Other,
}

/// A compiler-established C integer constant kept independent of its actual Rust binding value.
#[derive(Clone, Debug, Serialize)]
pub struct IntegerConstant {
    /// Original C integer identity, including rank and signedness, established independently of Rust
    /// storage.
    pub ty: TypeInfo,
    /// Compiler-established value to be compared with actual bindgen output before retaining a
    /// symbol.
    pub value: IntegerValue,
    /// Original literal spelling when the complete object expansion is a literal.
    /// Enum values and compound expressions have no single literal to preserve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub literal: Option<crate::IntegerLiteral>,
}

/// Signed or unsigned C integer magnitude large enough to retain compiler constants without host
/// narrowing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum IntegerValue {
    /// Signed compiler constant storage retaining values without host-width narrowing.
    Signed(
        /// The exact signed constant retained without host-width narrowing.
        i64,
    ),
    /// Unsigned compiler constant storage retaining values without host-width narrowing.
    Unsigned(
        /// The exact unsigned constant retained without host-width narrowing.
        u64,
    ),
}
