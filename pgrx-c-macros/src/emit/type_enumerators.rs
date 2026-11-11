//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Enumerators that C names by pasting a fixed prefix onto a structure type's name.
//!
//! PostgreSQL's `T_##_type_` names the node tag of a node type. `macro_rules!` cannot
//! build an identifier, and the macro needs only the enumerator's value for the type its
//! caller supplies. Each prefix therefore gets a trait whose constant is that value, and
//! each bound structure type whose C name completes an enumerator implements it. A type
//! that does not fails to compile, as its pasted name fails to resolve in C.
//!
//! C can give one structure several names, each with an enumerator of its own, as
//! PostgreSQL's `OpExpr`, `DistinctExpr` and `NullIfExpr` name one structure. Rust sees
//! one type, so the type alone selects no enumerator and implements nothing. The C name
//! comes from the operand's spelling instead: an exported macro per prefix walks the
//! operand's path and, when its last segment is one of those names, wraps the path in
//! that name's carrier type. The carrier implements the lookup for the shared structure
//! only, with that name's enumerator, and the lookup's `Type` gives the structure back
//! for the macro's other uses of the operand.
//!
//! The traits are sealed, so only this support decides which types name an enumerator.

use super::names::strip_location_directories;
use super::types::{Lowering, rust_path};
use super::{BindingCatalog, binding_path, integer_numeric, marker, write_binding_value};
use crate::{FrontendOutput, IntegerValue};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

/// Bound this family's source independently of the number of retained macros.
const SOURCE_LIMIT: usize = 16 * 1024 * 1024;
/// The private module whose traits seal the lookups.
const SEALS: &str = "type_enumerator_seals";

/// How many path segments a name walk reads before it leaves the operand as written.
const PATH_BUDGET: usize = 64;

/// The lookup trait of each requested prefix and the types that implement it.
pub(super) struct TypeEnumeratorAdapters {
    /// Sealed lookup traits with one implementation per bound structure type, and the
    /// carrier types of the C names that share a structure.
    pub rust: String,
    /// The exported macros that find those C names in a type operand's spelling.
    pub macros: String,
    /// Each requested prefix's lookup, or why it has none.
    pub lookups: BTreeMap<String, Result<Lookup, Refusal>>,
}

/// One prefix's lookup, as an exported macro uses it.
pub(super) struct Lookup {
    /// The lookup trait's path.
    pub path: String,
    /// The runtime marker of the C type every enumerator of the lookup has.
    pub marker: &'static str,
    /// The exported macro that replaces a type operand whose last path segment is a C name
    /// of a shared structure with that name's carrier. `None` when no structure of this
    /// prefix has a carrier, so operands stay as written.
    pub names: Option<String>,
}

/// Why a prefix has no lookup.
pub(super) enum Refusal {
    /// The declarations or the bindings give no type an enumerator.
    Unavailable(String),
    /// Bindgen and Clang disagree on the value of an enumerator the prefix pastes into.
    ValueMismatch {
        /// The enumerator.
        name: String,
        /// Its value in the bindings.
        bindgen: IntegerValue,
        /// Its value in C.
        clang: IntegerValue,
    },
}

/// Name a prefix's lookup trait after the prefix, such as `TypeEnumerator_T_`.
fn lookup_name(prefix: &str) -> String {
    format!("TypeEnumerator_{prefix}")
}

/// Name the module that holds a prefix's carrier types, such as `type_names_T_`.
fn carriers_name(prefix: &str) -> String {
    format!("type_names_{prefix}")
}

/// Name the exported macro that finds a prefix's shared C names in a type operand.
fn names_macro(prefix: &str) -> String {
    format!("__pgrx_c_type_name_{prefix}")
}

/// Generate the lookup of every requested prefix.
///
/// A structure type implements a lookup when the bindings hold its record and its
/// enumerator. One whose record or enumerator is not bound is left out. A record that
/// several C names would give different enumerators implements nothing itself, as
/// PostgreSQL's `OpExpr`, `DistinctExpr` and `NullIfExpr` name one structure: Rust sees
/// one type, and choosing one of its enumerators would silently change what the other
/// names mean. Each of those names with a bound enumerator gets a carrier type instead,
/// which implements the lookup for that record with that name's enumerator. A name whose
/// enumerator a macro hides or the bindings lack still counts among those names and gets
/// no carrier.
pub(super) fn generate(
    frontend: &FrontendOutput,
    bindings: &BindingCatalog,
    lowering: &Lowering<'_>,
    prefixes: &BTreeSet<String>,
) -> Result<TypeEnumeratorAdapters, String> {
    let mut seals = String::new();
    let mut rust = String::new();
    let mut macros = String::new();
    let mut lookups = BTreeMap::new();
    for prefix in prefixes {
        let lookup = lookup(frontend, bindings, lowering, prefix).map(|generated| {
            writeln!(seals, "pub trait Seal_{prefix} {{}}").expect("String output");
            rust.push_str(&generated.rust);
            macros.push_str(&generated.macros);
            Lookup {
                path: format!("$crate::__pgrx_c_generated::{}", lookup_name(prefix)),
                marker: generated.marker,
                names: (!generated.macros.is_empty()).then(|| names_macro(prefix)),
            }
        });
        lookups.insert(prefix.clone(), lookup);
        if rust.len() + macros.len() > SOURCE_LIMIT {
            return Err("type enumerator lookups exceed the bounded source budget".into());
        }
    }
    if !seals.is_empty() {
        rust = format!(
            "/// Seals the type enumerator lookups: only this support names these traits.\nmod {SEALS} {{\n{seals}}}\n{rust}"
        );
    }
    Ok(TypeEnumeratorAdapters { rust, macros, lookups })
}

/// One prefix's generated lookup.
struct GeneratedLookup {
    /// The lookup trait, its implementations and its carrier types.
    rust: String,
    /// The exported macro that finds the carriers' C names, or nothing without carriers.
    macros: String,
    /// The runtime marker of the enumerators' C type.
    marker: &'static str,
}

/// Write one prefix's lookup trait, its implementations, and the carriers and name macro
/// of the C names that share a structure.
fn lookup(
    frontend: &FrontendOutput,
    bindings: &BindingCatalog,
    lowering: &Lowering<'_>,
    prefix: &str,
) -> Result<GeneratedLookup, Refusal> {
    let declarations = frontend.declarations();
    let enumerators = frontend.type_enumerators(prefix).map_err(Refusal::Unavailable)?;
    let enumeration = strip_location_directories(enumerators.enumeration);
    // Each bound record with the C names that C pastes into an enumerator for it, and that
    // enumerator's value when a macro hides neither name and the bindings hold it.
    let mut records = BTreeMap::<String, Vec<(&str, Option<String>)>>::new();
    for (type_name, enumerator) in &enumerators.members {
        let Ok(record) = lowering
            .record_binding(&declarations.types[*type_name])
            .and_then(|record| rust_path(&record.path))
        else {
            continue;
        };
        let mut value = None;
        if let Some(enumerator) = enumerator
            && let Ok((path, representation)) = binding_path(bindings, enumerator)
        {
            let (bindgen, clang) = (
                bindings.integer_constants[*enumerator].value,
                declarations.integer_constants[*enumerator].value,
            );
            if integer_numeric(bindgen) != integer_numeric(clang) {
                return Err(Refusal::ValueMismatch { name: (*enumerator).into(), bindgen, clang });
            }
            let mut rust = String::new();
            let path = format!("crate::{path}");
            write_binding_value(&mut rust, "c", &path, representation, enumerators.ty);
            value = Some(rust);
        }
        records.entry(record.replace("$crate", "crate")).or_default().push((type_name, value));
    }
    // A record with one C name implements the lookup itself. Rust sees one type where C has
    // several names, each with its own enumerator, so no one enumerator is that type's, even
    // when a macro hides one of those names. Each of its names with an enumerator selects
    // that enumerator through a carrier, when the name can be matched as a Rust identifier.
    let mut implemented = Vec::new();
    let mut carriers = Vec::new();
    for (record, names) in &records {
        match names.as_slice() {
            [(_, Some(value))] => implemented.push((record, value)),
            [_] => {}
            names => carriers.extend(names.iter().filter_map(|(name, value)| {
                let value = value.as_ref()?;
                (super::rust_identifier(name).as_deref() == Some(*name))
                    .then_some((record, *name, value))
            })),
        }
    }
    if implemented.is_empty() && carriers.is_empty() {
        return Err(Refusal::Unavailable(format!(
            "no bound structure type has a name that `{prefix}` pastes into a bound enumerator of `{enumeration}`"
        )));
    }
    let name = lookup_name(prefix);
    let marker = marker(enumerators.ty.kind);
    // The attribute formats its text, in which only `{Self}` is a parameter.
    let literal = enumeration.replace('{', "{{").replace('}', "}}");
    let message =
        format!("no enumerator of `{literal}` is `{prefix}` followed by the C name of `{{Self}}`");
    let label = format!("`{prefix}` pasted onto this type's C name must name an enumerator");
    // The carriers are listed by record, in the order of the records.
    let mut shared = Vec::<Vec<&str>>::new();
    let mut previous = None;
    for (record, name, _) in &carriers {
        if previous != Some(*record) {
            shared.push(Vec::new());
            previous = Some(*record);
        }
        shared.last_mut().expect("pushed above").push(name);
    }
    let note = if shared.is_empty() {
        String::new()
    } else {
        format!(
            ", note = {:?}",
            format!(
                "a structure that several C type names share has an enumerator for each, so the operand must end in one of those names, written as the last segment of a plain path and not through an alias or a captured `ty` fragment: {}",
                shared.iter().map(|names| names.join(", ")).collect::<Vec<_>>().join("; ")
            )
        )
    };
    let mut rust = format!(
        "/// The enumerator of `{enumeration}` that C names by pasting `{prefix}` onto a structure type's name.\n#[diagnostic::on_unimplemented(message = {message:?}, label = {label:?}{note})]\npub trait {name}: {SEALS}::Seal_{prefix} {{\n/// The structure type the operand names.\ntype Type;\n/// The enumerator, with its C type.\nconst VALUE: c::CValue<c::{marker}>;\n}}\n"
    );
    for (record, value) in implemented {
        writeln!(rust, "impl {SEALS}::Seal_{prefix} for {record} {{}}\nimpl {name} for {record} {{ type Type = Self; const VALUE: c::CValue<c::{marker}> = {value}; }}")
            .expect("String output");
    }
    let mut macros = String::new();
    if !carriers.is_empty() {
        let module = carriers_name(prefix);
        let names_macro = names_macro(prefix);
        writeln!(rust, "/// The C type names of `{prefix}` that share a structure with another name. A generated macro wraps a type operand that ends in one of them in the type of that name here, which selects that name's enumerator for the shared structure and for no other type.\npub mod {module} {{").expect("String output");
        for (_, carried, _) in &carriers {
            writeln!(rust, "/// The C type name `{carried}`, spelled as the last segment of the path `T`.\npub struct {carried}<T: ?Sized>(::core::marker::PhantomData<T>);").expect("String output");
        }
        rust.push_str("}\n");
        write!(
            macros,
            "#[doc(hidden)]\n#[macro_export]\nmacro_rules! {names_macro} {{\n\
             (@type [$callback:ident] $state:tt; $($raw:tt)*) => {{ $crate::{names_macro}!(@walk [$callback] $state [$($raw)*] [] [{}]; $($raw)*) }};\n",
            "@ ".repeat(PATH_BUDGET)
        )
        .expect("String output");
        for (record, carried, value) in &carriers {
            writeln!(rust, "impl {SEALS}::Seal_{prefix} for {module}::{carried}<{record}> {{}}\nimpl {name} for {module}::{carried}<{record}> {{ type Type = {record}; const VALUE: c::CValue<c::{marker}> = {value}; }}")
                .expect("String output");
            // The name ends the operand: nothing follows it, or the comma before the next
            // argument does. A longer path or a generic argument list is some other type.
            for (ending, rest) in [("", ""), (", $($rest:tt)*", "$($rest)*")] {
                writeln!(macros, "(@walk [$callback:ident] [$next:ident $plain:ident $mode:ident [$($done:tt)*]] $original:tt [$($path:tt)*] $budget:tt; {carried}{ending}) => {{ $crate::$callback!(@$next $mode [$($done)* $crate::__pgrx_c_generated::{module}::{carried}<$($path)* {carried}>,]; {rest}) }};").expect("String output");
            }
        }
        // One token tree at a time: a repetition of segments before a literal name would be
        // ambiguous to `macro_rules!`. Anything that is not a plain path to one of the names
        // returns the operand as written, for the caller's own type fragment to parse.
        write!(
            macros,
            "(@walk [$callback:ident] $state:tt $original:tt [] [@ $($budget:tt)*]; :: $($rest:tt)+) => {{ $crate::{names_macro}!(@walk [$callback] $state $original [::] [$($budget)*]; $($rest)+) }};\n\
             (@walk [$callback:ident] $state:tt $original:tt [$($path:tt)*] [@ $($budget:tt)*]; $segment:ident :: $($rest:tt)+) => {{ $crate::{names_macro}!(@walk [$callback] $state $original [$($path)* $segment ::] [$($budget)*]; $($rest)+) }};\n\
             (@walk [$callback:ident] [$next:ident $plain:ident $mode:ident $done:tt] [$($original:tt)*] $path:tt $budget:tt; $($other:tt)*) => {{ $crate::$callback!(@$plain $mode $done; $($original)*) }};\n}}\n"
        )
        .expect("String output");
    }
    Ok(GeneratedLookup { rust, macros, marker })
}
