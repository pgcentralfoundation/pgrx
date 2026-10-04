//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Actual Rust representations supplied by the binding generator.
//!
//! C declarations determine conversions and operation semantics. These facts determine
//! which generated Rust items exist and how their storage can be passed across that boundary.

//! Binding facts are an independent input to emission, not a replacement for C declarations.
//!
//! The catalog records what bindgen actually emitted on the target: paths, storage types,
//! record fields, bitfield storage, callable ABIs, and valid Rust enum values. Adapters reconcile
//! those facts with C identities before crossing a native boundary; matching Rust widths alone
//! never establishes matching C integer rank or callback identity.

/// Connect this phase to the crate’s owned compiler facts and shared pipeline result types.
use crate::{IntegerValue, RecordKind};
/// Serialize owned inspection/analysis facts without borrowing from compiler translation units.
use serde::Serialize;
/// Keep catalog lookup and report ordering deterministic while bounding repeated traversal.
use std::collections::BTreeMap;

/// A generated function signature with a verified binding storage ABI.
#[derive(Clone, Debug)]
pub struct CallbackBinding {
    /// Generated semantic callback marker retaining the verified C signature identity.
    pub marker: String,
    /// Actual Rust function-pointer representation whose ABI must match the original C signature.
    pub storage: RustBindingType,
}

/// A generated getter returning the original C function's address.
#[derive(Clone, Debug)]
pub struct FunctionAddressBinding {
    /// Semantic function marker used to tag the original C address.
    pub marker: String,
    /// Generated getter path returning the original C function address.
    pub path: Vec<String>,
}

/// A bounded structural representation of a type in the generated Rust bindings.
///
/// Named paths are relative to the defining crate, except `core` and `std` paths.
/// Integer widths describe Rust storage, never C integer rank or typedef identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RustBindingType {
    /// Rust unit storage used for C void results at the binding boundary.
    Unit,
    /// A distinct boolean representation rather than an arbitrary integer-width alias.
    Bool,
    /// Fixed-width Rust storage or a fundamental C integer category, as identified by the enclosing
    /// catalog.
    Integer {
        /// Signedness of actual Rust integer storage, independent of the corresponding C
        /// conversion rank.
        signed: bool,
        /// Width of actual Rust storage; C rank and typedef identity are established
        /// separately.
        bits: u32,
    },
    /// Rust isize/usize storage whose width alone does not prove a C long identity.
    PointerSizedInteger {
        /// Signedness of actual Rust integer storage, independent of the corresponding C
        /// conversion rank.
        signed: bool,
        /// Width of actual Rust storage; C rank and typedef identity are established
        /// separately.
        bits: u32,
    },
    /// A floating representation whose width/category needs independent target proof.
    Float {
        /// Width of actual Rust storage; C rank and typedef identity are established
        /// separately.
        bits: u32,
    },
    /// A pointer representation retaining its pointee and layer-specific qualifiers.
    Pointer {
        /// Actual Rust pointee storage, normalized through the binding catalog before C compatibility
        /// checks.
        pointee: Box<Self>,
        /// Whether the actual Rust binding allows mutation through this pointer or static.
        mutable: bool,
    },
    /// An element relationship with its length and bound category retained separately.
    Array {
        /// Actual Rust array/helper element storage whose C layout is checked separately.
        element: Box<Self>,
        /// Fixed Rust array element count from actual bindgen output.
        length: u64,
    },
    /// A callable type with prototype/ABI facts, distinct from an addressable function binding.
    Function {
        /// Ordered actual Rust function parameter storage types.
        parameters: Vec<Self>,
        /// Actual Rust function result storage checked at callback ABI boundaries.
        result: Box<Self>,
        /// The actual Rust callable ABI, checked against the C prototype before generating adapters.
        abi: String,
        /// Whether the actual Rust function type requires an unsafe call.
        unsafe_: bool,
        /// Whether the callable has an open argument tail that requires special handling or refusal.
        variadic: bool,
    },
    /// Nullable Rust storage whose inner validity must agree with the original C representation.
    Option {
        /// Inner storage type of this representation wrapper; the wrapper’s validity rules remain
        /// distinct.
        value: Box<Self>,
    },
    /// The standard library's transparent union-field storage wrapper.
    ManuallyDrop {
        /// Inner storage type of this representation wrapper; the wrapper’s validity rules remain
        /// distinct.
        value: Box<Self>,
    },
    /// Raw C record ABI storage preserves uninitialized fields and padding.
    MaybeUninit {
        /// Inner storage type of this representation wrapper; the wrapper’s validity rules remain
        /// distinct.
        value: Box<Self>,
    },
    /// A verified `repr(C)` bindgen helper storing `PhantomData<T>` and `[T; 0]`.
    IncompleteArrayField {
        /// Actual bindgen-generated Rust path relative to the defining crate, except
        /// standard-library paths.
        path: Vec<String>,
        /// Actual Rust array/helper element storage whose C layout is checked separately.
        element: Box<Self>,
    },
    /// An actual generated Rust path resolved through the binding catalog rather than synthesized
    /// from C spelling.
    Named {
        /// Actual bindgen-generated Rust path relative to the defining crate, except
        /// standard-library paths.
        path: Vec<String>,
    },
}

/// A callable declaration actually emitted by bindgen, before pgrx applies its FFI guards.
#[derive(Clone, Debug, Serialize)]
pub struct FunctionBinding {
    /// Actual bindgen-generated Rust path relative to the defining crate, except standard-library
    /// paths.
    pub path: Vec<String>,
    /// Actual Rust parameter storage types before pgrx applies FFI guard rewriting.
    pub parameters: Vec<RustBindingType>,
    /// Actual Rust result storage checked against the original C return identity.
    pub result: RustBindingType,
    /// The actual Rust callable ABI, checked against the C prototype before generating adapters.
    pub abi: String,
    /// Whether the callable has an open argument tail that requires special handling or refusal.
    pub variadic: bool,
    /// The foreign symbol, which can differ for generated static-inline wrappers.
    pub link_name: String,
    /// pgrx's existing rewrite guards non-variadic foreign functions.
    pub guarded: bool,
    /// Whether linkage follows the configured cshim path rather than an ordinary direct symbol.
    pub uses_cshim: bool,
}

/// The actual Rust member name and storage type used to establish a native field adapter.
#[derive(Clone, Debug, Serialize)]
pub struct FieldBinding {
    /// Retains a raw-identifier prefix when needed by the Rust source.
    pub rust_name: String,
    /// Actual target Rust storage type checked against the original C identity before generating an
    /// adapter.
    pub ty: RustBindingType,
}

/// Storage facts recovered from bindgen's generated bitfield accessors.
/// Accessors themselves are never used to read partially initialized C storage.
#[derive(Clone, Debug, Serialize)]
pub struct BitfieldBinding {
    /// Actual Rust record path containing the bindgen bitfield storage unit.
    pub record_path: Vec<String>,
    /// Rust accessor label used only to recover storage facts, never to load partial C records.
    pub rust_name: String,
    /// Actual target Rust storage type checked against the original C identity before generating an
    /// adapter.
    pub ty: RustBindingType,
    /// Actual bindgen-generated backing member containing this bitfield’s bits.
    pub storage_field: String,
    /// Offset relative to the named bitfield storage unit.
    pub offset_bits: u64,
    /// Number of field bits copied from bindgen storage-accessor metadata.
    pub width: u32,
}

/// Named fields present in the actual Rust record, rather than inferred from C spelling.
#[derive(Clone, Debug, Serialize)]
pub struct RecordBinding {
    /// Actual bindgen-generated Rust path relative to the defining crate, except standard-library
    /// paths.
    pub path: Vec<String>,
    /// Struct-versus-union shape that must agree with the original C declaration.
    pub kind: RecordKind,
    /// Actual named Rust storage members indexed by their original C field names.
    pub fields: BTreeMap<String, FieldBinding>,
    /// Whether the Rust representation requires unaligned field-access reasoning.
    pub packed: bool,
    /// Whether the actual Rust record implements Copy; raw C storage can require a different value
    /// path.
    pub copy: bool,
}

/// A named Rust typedef edge; C identity still comes from the inspected declaration catalog.
#[derive(Clone, Debug, Serialize)]
pub struct AliasBinding {
    /// Actual bindgen-generated Rust path relative to the defining crate, except standard-library
    /// paths.
    pub path: Vec<String>,
    /// Actual Rust alias target used for bounded storage normalization.
    pub target: RustBindingType,
}

/// A Rust enum's valid values; arbitrary C integer values need not be valid Rust variants.
#[derive(Clone, Debug, Serialize)]
pub struct EnumBinding {
    /// Actual bindgen-generated Rust path relative to the defining crate, except standard-library
    /// paths.
    pub path: Vec<String>,
    /// Established Rust discriminant storage, if representable by the binding catalog.
    pub repr: Option<RustBindingType>,
    /// Rust-valid discriminant values checked before C enum values can be materialized.
    pub variants: BTreeMap<String, IntegerValue>,
}

/// A generated static binding and its Rust storage/mutability for native variable access
/// capabilities.
#[derive(Clone, Debug, Serialize)]
pub struct VariableBinding {
    /// Actual bindgen-generated Rust path relative to the defining crate, except standard-library
    /// paths.
    pub path: Vec<String>,
    /// Actual target Rust storage type checked against the original C identity before generating an
    /// adapter.
    pub ty: RustBindingType,
    /// Whether the actual Rust binding allows mutation through this pointer or static.
    pub mutable: bool,
}
