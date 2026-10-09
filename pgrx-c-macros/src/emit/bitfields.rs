//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Generate original-C access primitives where Rust cannot model initialized bits.
//!
//! Rust cannot directly project a C bitfield as an addressable field. These adapters
//! leave width conversion and neighboring-bit preservation to the original C
//! compiler, while Rust capability markers track mutability, volatility, and access
//! alignment. Bindgen and Clang layout witnesses must agree before emission.

use super::types::{Lowering, rust_path};
use crate::{BindingCatalog, DeclarationCatalog, FieldInfo, RecordBinding, TypeCategory};
use sha2::{Digest, Sha256};
use std::fmt::Write;

/// Private runtime namespace used by same-crate bitfield support items.
const EXPRESSION: &str = "c::expression";

/// Rust place capabilities paired with original-C primitives for one verified bitfield.
pub(super) struct BitfieldAdapter {
    /// Typed place descriptors and explicit read/write capability implementations.
    pub rust: String,
    /// Compiler-owned getters and setters preserving bit width and adjacent storage.
    pub c_source: String,
}

/// Verify the compiler and bindgen storage witness before producing one bitfield capability.
///
/// The declared arithmetic type and promotion remain separate, while generated C
/// performs width narrowing and volatile or unaligned accesses.
pub(super) fn generate(
    canonical: &str,
    field: &FieldInfo,
    record: &RecordBinding,
    field_marker: &str,
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    lowering: &Lowering<'_>,
) -> Result<BitfieldAdapter, String> {
    let name = field.name.as_ref().ok_or("bitfield has no addressable name")?;
    let storage = rust_path(&record.path)?.replace("$crate", "crate");
    if field.is_anonymous {
        return Err("anonymous bitfields require a verified promoted member path".into());
    }
    let width = field
        .bit_width
        .filter(|width| *width != 0)
        .ok_or("zero-width bitfields are not expressions")?;
    let offset = field.offset_bits.ok_or("compiler did not establish the bitfield offset")?;
    let facts = declarations
        .bitfields
        .get(&format!("{canonical}::{name}"))
        .ok_or("compiler-owned bitfield expression type facts are unavailable")?;
    if !facts.unaligned_volatile_access {
        return Err("compiler did not prove matching volatile bitfield access units for unaligned and containing-record projections".into());
    }
    let actual = bindings
        .bitfields
        .get(&format!("{}::{name}", record.path.join("::")))
        .ok_or("bindgen did not emit an extractable bitfield storage accessor")?;
    if actual.width != width {
        return Err("bitfield width differs between Clang and bindgen".into());
    }
    if !matches!(field.ty.category, TypeCategory::Integer(_))
        || !matches!(facts.promoted.category, TypeCategory::Integer(_))
    {
        return Err("bitfield declared and promoted types require a proven integer family".into());
    }
    // The generated accessor verifies the existing binding's scalar identity;
    // original C helpers use the compiler's canonical primitive ABI, not Rust
    // getters that require every neighboring storage byte to be initialized.
    lowering.resolve_with_storage(&field.ty, &actual.ty)?;
    let mut declared = field.ty.clone();
    declared.is_const = false;
    declared.is_volatile = false;
    declared.canonical_spelling =
        declared.canonical_spelling.replace("const ", "").replace("volatile ", "");
    declared.spelling = declared.canonical_spelling.clone();
    let object = lowering.resolve(&declared)?;
    let promoted = lowering.resolve(&facts.promoted)?;
    let writable = !field.ty.is_const;
    if writable
        && (facts.assignment.is_none()
            || facts
                .assignment_promoted
                .as_ref()
                .is_none_or(|ty| ty.canonical_spelling != facts.promoted.canonical_spelling))
    {
        return Err("bitfield assignment promotion lacks a matching compiler proof".into());
    }
    if writable
        && facts.postfix.as_ref().is_none_or(|ty| {
            ty.canonical_spelling
                != field.ty.canonical_spelling.replace("const ", "").replace("volatile ", "")
        })
    {
        return Err("bitfield postfix result lacks a matching declared-base compiler proof".into());
    }
    let object_marker = object.marker.replace("$crate", "crate");
    let promoted_marker = promoted.marker.replace("$crate", "crate");
    let expression_marker = format!("{EXPRESSION}::CBitfield<{object_marker},{promoted_marker}>");
    let object_storage = object.storage.replace("$crate", "crate");
    let promoted_storage = &object_storage;
    let layout =
        declarations.records.get(canonical).ok_or("bitfield record layout is unavailable")?;
    let layout = serde_json::to_string(layout)
        .map_err(|error| format!("cannot fingerprint bitfield record layout: {error}"))?;
    let hash = Sha256::digest(format!(
        "{canonical}\0{}\0{name}\0{offset}\0{width}\0{layout}\0{}\0{}",
        record.path.join("::"),
        field.ty.canonical_spelling,
        facts.promoted.canonical_spelling
    ));
    let suffix = hash[..16].iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let place = format!("Bitfield_{suffix}");
    let prefix = format!("__pgrx_bitfield_{suffix}");
    let record_type = &facts.record_type;
    let alias = format!("{prefix}_unaligned_type");
    let declared_c = field.ty.canonical_spelling.replace("const ", "").replace("volatile ", "");
    let promoted_c = &declared_c;
    let mut c_source = format!(
        "typedef {record_type} {alias} __attribute__((aligned(1),may_alias));\n_Static_assert(_Alignof({alias}) == 1, \"bitfield unaligned alias\");\n_Static_assert(sizeof({alias}) == sizeof({record_type}), \"bitfield alias layout\");\n"
    );
    let mut declarations_rust = String::new();
    for (mode, c_type, volatile) in [
        ("", record_type.as_str(), ""),
        ("_volatile", record_type.as_str(), "volatile "),
        ("_unaligned", alias.as_str(), ""),
        ("_unaligned_volatile", alias.as_str(), "volatile "),
    ] {
        writeln!(c_source, "{promoted_c} {prefix}_get{mode}(const {volatile}{c_type} *p) {{ return +(p->{name}); }}").expect("String output");
        writeln!(
            declarations_rust,
            "fn {prefix}_get{mode}(p: *const {storage}) -> {promoted_storage};"
        )
        .expect("String output");
        if writable {
            writeln!(c_source, "{promoted_c} {prefix}_set{mode}({volatile}{c_type} *p, {declared_c} value) {{ return +(p->{name} = value); }}").expect("String output");
            writeln!(declarations_rust, "fn {prefix}_set{mode}(p: *mut {storage}, value: {object_storage}) -> {promoted_storage};").expect("String output");
        }
    }
    let qualification =
        if field.ty.is_const { format!("{EXPRESSION}::ReadOnly") } else { "Q".into() };
    let mut rust = format!(
        "const _: () = {{ assert!(::core::mem::offset_of!({storage}, {}) * 8 + {} == {offset}); }};\nunsafe extern \"C\" {{\n{declarations_rust}}}\n#[doc(hidden)] pub struct {place}<Q: {EXPRESSION}::Qualifier> {{ address: Q::Raw<{storage}>, access: {EXPRESSION}::Access }}\nimpl<Q: {EXPRESSION}::Qualifier> Copy for {place}<Q> {{}}\nimpl<Q: {EXPRESSION}::Qualifier> Clone for {place}<Q> {{ fn clone(&self) -> Self {{ *self }} }}\n",
        actual.storage_field, actual.offset_bits
    );
    writeln!(rust, "// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.\nunsafe impl<Q: {EXPRESSION}::Qualifier> {EXPRESSION}::Field<{field_marker}, Q> for {EXPRESSION}::CRecord<{storage}> {{ type Output = {place}<{qualification}>; unsafe fn project(base: {EXPRESSION}::Place<Self,Q>) -> Self::Output {{ {place}::<{qualification}> {{ address: <{qualification} as {EXPRESSION}::Qualifier>::from_mut(base.pointer().as_mut_address()), access: {EXPRESSION}::Access {{ volatile: base.access().volatile || {}, unaligned: base.access().unaligned }} }} }} }}", field.ty.is_volatile).expect("String output");
    writeln!(rust,"impl<Q: {EXPRESSION}::Qualifier> {EXPRESSION}::VolatilePlace for {place}<Q> {{ type Qualified = Self; fn qualify_volatile(mut self) -> Self::Qualified {{ self.access.volatile = true; self }} }}").expect("String output");
    let select_get = select(&format!("{prefix}_get"), "pointer", "self.access");
    writeln!(rust, "// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.\nunsafe impl<Q: {EXPRESSION}::Qualifier> {EXPRESSION}::ReadPlace for {place}<Q> {{ type Object = {object_marker}; type Type = {expression_marker}; unsafe fn load(self) -> <Self::Type as {EXPRESSION}::CType>::Value {{ let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.\nlet storage = unsafe {{ {select_get} }}; <Self::Type as {EXPRESSION}::CType>::from_storage(storage) }} }}").expect("String output");
    if writable {
        let select_set = select(&format!("{prefix}_set"), "self.address, value", "self.access");
        writeln!(rust, "// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.\nunsafe impl {EXPRESSION}::WritePlace for {place}<{EXPRESSION}::ReadWrite> {{ type Assignment = {object_marker}; unsafe fn store(self, value: <Self::Assignment as {EXPRESSION}::CType>::Value) -> <Self::Type as {EXPRESSION}::CType>::Value {{ let value = <Self::Assignment as {EXPRESSION}::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.\nlet stored = unsafe {{ {select_set} }}; <Self::Type as {EXPRESSION}::CType>::from_storage(stored) }} }}").expect("String output");
    }
    Ok(BitfieldAdapter { rust, c_source })
}

/// Choose the exact native primitive for the inherited volatile and alignment access flags.
fn select(function: &str, arguments: &str, access: &str) -> String {
    format!(
        "if {access}.unaligned {{ if {access}.volatile {{ {function}_unaligned_volatile({arguments}) }} else {{ {function}_unaligned({arguments}) }} }} else if {access}.volatile {{ {function}_volatile({arguments}) }} else {{ {function}({arguments}) }}"
    )
}
