//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Actual Rust representations supplied by the binding generator.
//!
//! C declarations determine conversions and operation semantics. These facts determine
//! which generated Rust items exist and how their storage can be passed across that boundary.

use crate::{IntegerValue, RecordKind};
use serde::Serialize;
use std::collections::BTreeMap;

/// A generated function signature with a verified binding storage ABI.
#[derive(Clone, Debug)]
pub struct CallbackBinding {
    pub marker: String,
    pub storage: RustBindingType,
}

/// A generated getter returning the original C function's address.
#[derive(Clone, Debug)]
pub struct FunctionAddressBinding {
    pub marker: String,
    pub path: Vec<String>,
}

/// A bounded structural representation of a type in the generated Rust bindings.
///
/// Named paths are relative to the defining crate, except `core` and `std` paths.
/// Integer widths describe Rust storage, never C integer rank or typedef identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RustBindingType {
    Unit,
    Bool,
    Integer {
        signed: bool,
        bits: u32,
    },
    PointerSizedInteger {
        signed: bool,
        bits: u32,
    },
    Float {
        bits: u32,
    },
    Pointer {
        pointee: Box<Self>,
        mutable: bool,
    },
    Array {
        element: Box<Self>,
        length: u64,
    },
    Function {
        parameters: Vec<Self>,
        result: Box<Self>,
        abi: String,
        unsafe_: bool,
        variadic: bool,
    },
    Option {
        value: Box<Self>,
    },
    /// The standard library's transparent union-field storage wrapper.
    ManuallyDrop {
        value: Box<Self>,
    },
    /// Raw C record ABI storage preserves uninitialized fields and padding.
    MaybeUninit {
        value: Box<Self>,
    },
    /// A verified `repr(C)` bindgen helper storing `PhantomData<T>` and `[T; 0]`.
    IncompleteArrayField {
        path: Vec<String>,
        element: Box<Self>,
    },
    Named {
        path: Vec<String>,
    },
}

/// A callable declaration actually emitted by bindgen, before pgrx applies its FFI guards.
#[derive(Clone, Debug, Serialize)]
pub struct FunctionBinding {
    pub path: Vec<String>,
    pub parameters: Vec<RustBindingType>,
    pub result: RustBindingType,
    pub abi: String,
    pub variadic: bool,
    /// The foreign symbol, which can differ for generated static-inline wrappers.
    pub link_name: String,
    /// pgrx's existing rewrite guards non-variadic foreign functions.
    pub guarded: bool,
    pub uses_cshim: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct FieldBinding {
    /// Retains a raw-identifier prefix when needed by the Rust source.
    pub rust_name: String,
    pub ty: RustBindingType,
}

/// Storage facts recovered from bindgen's generated bitfield accessors.
/// Accessors themselves are never used to read partially initialized C storage.
#[derive(Clone, Debug, Serialize)]
pub struct BitfieldBinding {
    pub record_path: Vec<String>,
    pub rust_name: String,
    pub ty: RustBindingType,
    pub storage_field: String,
    /// Offset relative to the named bitfield storage unit.
    pub offset_bits: u64,
    pub width: u32,
}

/// Named fields present in the actual Rust record, rather than inferred from C spelling.
#[derive(Clone, Debug, Serialize)]
pub struct RecordBinding {
    pub path: Vec<String>,
    pub kind: RecordKind,
    pub fields: BTreeMap<String, FieldBinding>,
    pub packed: bool,
    pub copy: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct AliasBinding {
    pub path: Vec<String>,
    pub target: RustBindingType,
}

/// A Rust enum's valid values; arbitrary C integer values need not be valid Rust variants.
#[derive(Clone, Debug, Serialize)]
pub struct EnumBinding {
    pub path: Vec<String>,
    pub repr: Option<RustBindingType>,
    pub variants: BTreeMap<String, IntegerValue>,
}

#[derive(Clone, Debug, Serialize)]
pub struct VariableBinding {
    pub path: Vec<String>,
    pub ty: RustBindingType,
    pub mutable: bool,
}
