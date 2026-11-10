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

/// The lookup trait of each requested prefix and the types that implement it.
pub(super) struct TypeEnumeratorAdapters {
    /// Sealed lookup traits with one implementation per bound structure type.
    pub rust: String,
    /// Each requested prefix's lookup trait, as an exported macro names it, or why it has
    /// none.
    pub lookups: BTreeMap<String, Result<String, Refusal>>,
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

/// Generate the lookup of every requested prefix.
///
/// A structure type implements a lookup when the bindings hold its record and its
/// enumerator. One whose record or enumerator is not bound is left out. So is a record
/// that several C names would give different enumerators, as PostgreSQL's `OpExpr`,
/// `DistinctExpr` and `NullIfExpr` name one structure: Rust sees one type, and choosing
/// one of its enumerators would silently change what the other names mean. A name whose
/// enumerator a macro hides or the bindings lack still counts among those names.
pub(super) fn generate(
    frontend: &FrontendOutput,
    bindings: &BindingCatalog,
    lowering: &Lowering<'_>,
    prefixes: &BTreeSet<String>,
) -> Result<TypeEnumeratorAdapters, String> {
    let mut seals = String::new();
    let mut rust = String::new();
    let mut lookups = BTreeMap::new();
    for prefix in prefixes {
        let lookup = lookup(frontend, bindings, lowering, prefix).map(|implementations| {
            writeln!(seals, "pub trait Seal_{prefix} {{}}").expect("String output");
            rust.push_str(&implementations);
            format!("$crate::__pgrx_c_generated::{}", lookup_name(prefix))
        });
        lookups.insert(prefix.clone(), lookup);
        if rust.len() > SOURCE_LIMIT {
            return Err("type enumerator lookups exceed the bounded source budget".into());
        }
    }
    if !seals.is_empty() {
        rust = format!(
            "/// Seals the type enumerator lookups: only this support names these traits.\nmod {SEALS} {{\n{seals}}}\n{rust}"
        );
    }
    Ok(TypeEnumeratorAdapters { rust, lookups })
}

/// Write one prefix's lookup trait and its implementations.
fn lookup(
    frontend: &FrontendOutput,
    bindings: &BindingCatalog,
    lowering: &Lowering<'_>,
    prefix: &str,
) -> Result<String, Refusal> {
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
    // Rust sees one type where C has several names, each with its own enumerator, so no
    // one enumerator is that type's, even when a macro hides one of those names. The error
    // for such a type says so.
    let shared = records
        .values()
        .filter(|names| names.len() > 1)
        .map(|names| names.iter().map(|(name, _)| *name).collect::<Vec<_>>().join(", "))
        .collect::<Vec<_>>();
    let implemented = records
        .iter()
        .filter_map(|(record, names)| match names.as_slice() {
            [(_, Some(value))] => Some((record, value)),
            _ => None,
        })
        .collect::<Vec<_>>();
    if implemented.is_empty() {
        return Err(Refusal::Unavailable(format!(
            "no bound structure type has one name that `{prefix}` pastes into a bound enumerator of `{enumeration}`"
        )));
    }
    let name = lookup_name(prefix);
    let marker = marker(enumerators.ty.kind);
    // The attribute formats its text, in which only `{Self}` is a parameter.
    let literal = enumeration.replace('{', "{{").replace('}', "}}");
    let message =
        format!("no enumerator of `{literal}` is `{prefix}` followed by the C name of `{{Self}}`");
    let label = format!("`{prefix}` pasted onto this type's C name must name an enumerator");
    let note = if shared.is_empty() {
        String::new()
    } else {
        format!(
            ", note = {:?}",
            format!(
                "a Rust type that several C type names share, each with its own enumerator, selects none: {}",
                shared.join("; ")
            )
        )
    };
    let mut rust = format!(
        "/// The enumerator of `{enumeration}` that C names by pasting `{prefix}` onto a structure type's name.\n#[diagnostic::on_unimplemented(message = {message:?}, label = {label:?}{note})]\npub trait {name}: {SEALS}::Seal_{prefix} {{\n/// The enumerator, with its C type.\nconst VALUE: c::CValue<c::{marker}>;\n}}\n"
    );
    for (record, value) in implemented {
        writeln!(rust, "impl {SEALS}::Seal_{prefix} for {record} {{}}\nimpl {name} for {record} {{ const VALUE: c::CValue<c::{marker}> = {value}; }}")
            .expect("String output");
    }
    Ok(rust)
}
