//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compiler-established field projection capabilities for the actual Rust bindings.
//!
//! Field capability generation pairs compiler record identities and offsets with
//! actual binding field paths. Ordinary projections retain typed layout witnesses;
//! bitfields delegate storage access to C. Offset-only capabilities are planned
//! separately because computing an offset must not require reading the member or
//! constructing a fully initialized record.

use super::types::{Lowering, rust_path};
use crate::{
    ArrayKind, BindingCatalog, DeclarationCatalog, FieldBinding, FieldInfo, TargetFacts,
    TypeCategory, TypeInfo, TypeShapeKind,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Write;

/// Private runtime namespace used by generated member and record capabilities.
const EXPRESSION: &str = "c::expression";
/// Bound generated field support independently of macro expansion source size.
const MAX_ADAPTER_BYTES: usize = 16 * 1024 * 1024;

/// Named members or the full member family retained for one owner request.
#[derive(Clone, Default, PartialEq, Eq)]
struct RequestedFields {
    /// Concrete C member names selected by analyzed projections or offset paths.
    names: BTreeSet<String>,
    /// Keep every member when the caller supplies an unresolved field designator.
    all: bool,
}

/// Union concrete member demands without losing the possibility of a wildcard designator.
impl RequestedFields {
    /// Admit a member selected explicitly or by a wildcard owner request.
    fn contains(&self, name: &str) -> bool {
        self.all || self.names.contains(name)
    }

    /// Union member demands without losing an existing wildcard family.
    fn extend(&mut self, other: &Self) {
        self.all |= other.all;
        self.names.extend(other.names.iter().cloned());
    }
}

/// A concrete owner is a compiler record spelling; None retains capabilities
/// for a caller whose record identity cannot be established by the planner.
#[derive(Default, PartialEq, Eq)]
pub(super) struct FieldRequests {
    /// Load or projection demand, distinct from unevaluated offset-only use.
    fields: BTreeMap<Option<String>, RequestedFields>,
    /// Offset capability demand that does not imply a loadable member representation.
    offsets: BTreeMap<Option<String>, RequestedFields>,
}

/// Keep access and offset requests distinct while collecting their type dependencies.
impl FieldRequests {
    /// Retain a member access for a known owner, or all possible owners when unresolved.
    pub(super) fn field(&mut self, record: Option<&str>, name: Option<&str>) {
        Self::insert(&mut self.fields, record, name);
    }

    /// Retain an offsetof step independently of memory-access capabilities.
    pub(super) fn offset(&mut self, record: Option<&str>, name: Option<&str>) {
        Self::insert(&mut self.offsets, record, name);
    }

    /// Close type dependencies of requested members, including anonymous promotions.
    pub(super) fn required_types(&self, declarations: &DeclarationCatalog) -> Vec<TypeInfo> {
        let mut types = Vec::new();
        for (owner, request) in selected_records(&self.fields, declarations, false) {
            let mut pending = VecDeque::from([(owner.as_str(), 0)]);
            let mut seen = BTreeSet::new();
            while let Some((canonical, depth)) = pending.pop_front() {
                if depth > 64 || !seen.insert(canonical) {
                    continue;
                }
                let Some(record) = declarations.records.get(canonical) else { continue };
                for field in &record.fields {
                    if field.name.as_ref().is_some_and(|name| request.contains(name)) {
                        types.push(field.ty.clone());
                    } else if field.name.is_none() && field.ty.category == TypeCategory::Record {
                        pending.push_back((&field.ty.canonical_spelling, depth + 1));
                    }
                }
            }
        }
        types
    }

    /// Union concrete and wildcard requests into the selected access or offset family.
    fn insert(
        requests: &mut BTreeMap<Option<String>, RequestedFields>,
        record: Option<&str>,
        name: Option<&str>,
    ) {
        let request = requests.entry(record.map(str::to_owned)).or_default();
        if let Some(name) = name {
            request.names.insert(name.to_owned());
        } else {
            request.all = true;
        }
    }
}

/// Match requests by compiler record identity and retain qualified spellings sharing that identity.
///
/// Wildcard offset paths can traverse nested records; an unresolved owner retains
/// all possible record capabilities rather than guessing a binding type.
fn selected_records(
    requests: &BTreeMap<Option<String>, RequestedFields>,
    declarations: &DeclarationCatalog,
    nested_offsets: bool,
) -> BTreeMap<String, RequestedFields> {
    let mut identities = BTreeMap::<&str, RequestedFields>::new();
    let mut pending = VecDeque::new();
    for (owner, request) in requests {
        let Some(owner) = owner else { continue };
        let Some(record) = declarations.records.get(owner) else { continue };
        identities.entry(&record.identity).or_default().extend(request);
        if nested_offsets && request.all {
            pending.push_back((owner.as_str(), 0));
        }
    }
    // A caller-provided offsetof designator may continue through named records,
    // unions, or anonymous promotions, but never through a pointer or array.
    // Breadth-first traversal visits each record once at its shallowest depth.
    let mut seen = BTreeSet::new();
    while let Some((canonical, depth)) = pending.pop_front() {
        if depth >= 64 || !seen.insert(canonical) {
            continue;
        }
        let Some(record) = declarations.records.get(canonical) else { continue };
        for field in &record.fields {
            if field.ty.category != TypeCategory::Record {
                continue;
            }
            let Some(child) = declarations.records.get(&field.ty.canonical_spelling) else {
                continue;
            };
            identities.entry(&child.identity).or_default().all = true;
            pending.push_back((&field.ty.canonical_spelling, depth + 1));
        }
    }
    let wildcard = requests.get(&None).cloned().unwrap_or_default();
    declarations
        .records
        .iter()
        .filter_map(|(canonical, record)| {
            let mut request = wildcard.clone();
            if let Some(specific) = identities.get(record.identity.as_str()) {
                request.extend(specific);
            }
            (request.all || !request.names.is_empty()).then_some((canonical.clone(), request))
        })
        .collect()
}

/// Generated field support with independent success and rejection maps for access and offsets.
pub(super) struct FieldAdapters {
    /// Items included once in the defining crate's `__pgrx_c_generated` module.
    pub rust: String,
    /// Original-C field access primitives compiled under the inspected profile.
    pub c_source: String,
    /// C field name to the capability marker usable from a macro expansion.
    pub markers: BTreeMap<String, String>,
    /// Compiler record spelling and field name to the rejected capability's reason.
    pub unsupported: BTreeMap<String, String>,
    /// Compiler record and field to the nested record identity, or an offset-only leaf.
    pub offsets: BTreeMap<String, Option<String>>,
    /// Rejected offset capabilities, independently of field access support.
    pub offset_unsupported: BTreeMap<String, String>,
}

/// IDs depend on the complete immutable C catalog, so changing requested roots
/// or removing rejected macros cannot change an already emitted field identity.
struct FieldIds<'a> {
    /// Sorted immutable C member names assigned deterministic capability IDs.
    names: BTreeMap<&'a str, u64>,
}

/// Assign immutable catalog identities before emission demand can remove any member.
impl<'a> FieldIds<'a> {
    /// Assign sorted IDs from the complete declaration catalog before requested roots are pruned.
    fn new(declarations: &'a DeclarationCatalog) -> Result<Self, String> {
        let mut names = BTreeMap::new();
        for record in declarations.records.values() {
            for field in &record.fields {
                if let Some(name) = &field.name {
                    names.entry(name.as_str()).or_insert(0);
                }
            }
        }
        for (position, id) in names.values_mut().enumerate() {
            *id = u64::try_from(position)
                .map_err(|_| "C field identity registry exceeds u64 representation")?;
        }
        Ok(Self { names })
    }

    /// Resolve a catalog-owned member name without fabricating an identity for missing fields.
    fn marker(&self, name: &str) -> Result<String, String> {
        let id = self.names.get(name).ok_or_else(|| {
            format!("C field `{name}` has no immutable declaration-catalog identity")
        })?;
        Ok(format!("Field{id}"))
    }
}

/// Reconcile requested field paths with actual binding storage and emit their capability witnesses.
///
/// Record registration, ordinary projections, original-C bitfield access, and
/// offset-only paths have distinct proof requirements and rejection diagnostics.
pub(super) fn generate(
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    requests: &FieldRequests,
    target: &TargetFacts,
    required_types: &[TypeInfo],
) -> Result<FieldAdapters, String> {
    let field_ids = FieldIds::new(declarations)?;
    let lowering = Lowering::new_native(declarations, bindings, target);
    let records_by_path = bindings
        .records
        .values()
        .filter_map(|binding| {
            Some((rust_path(&binding.path).ok()?.replace("$crate", "crate"), binding))
        })
        .collect::<BTreeMap<_, _>>();
    let mut output = FieldAdapters {
        rust: String::new(),
        c_source: String::new(),
        markers: BTreeMap::new(),
        unsupported: BTreeMap::new(),
        offsets: BTreeMap::new(),
        offset_unsupported: BTreeMap::new(),
    };
    let mut layout_checks = LayoutChecks::default();
    let mut record_bridges = BTreeSet::new();
    let mut qualifiers_imported = false;
    let mut projections = BTreeMap::<(String, String), Option<OffsetIdentity>>::new();
    let mut pending_records = required_types.to_vec();
    let field_records = selected_records(&requests.fields, declarations, false);
    let offset_records = selected_records(&requests.offsets, declarations, true);
    for (canonical, record) in &declarations.records {
        let used_fields = field_records.get(canonical).cloned().unwrap_or_default();
        let used_offsets = offset_records.get(canonical).cloned().unwrap_or_default();
        let mut used_names = used_fields.clone();
        used_names.extend(&used_offsets);
        if !used_names.all && used_names.names.is_empty() {
            continue;
        }
        let mut used = Vec::new();
        collect_projections(
            canonical,
            declarations,
            &lowering,
            &used_names,
            &mut Vec::new(),
            &mut used,
            0,
        );
        if used.is_empty() {
            continue;
        }
        let shape = declarations.type_shapes.get(canonical);
        let resolved = shape
            .ok_or_else(|| "C record has no compiler-owned type shape".to_owned())
            .and_then(|shape| lowering.resolve(&shape.ty));
        let storage = match resolved {
            Ok(resolved) => resolved.storage.replace("$crate", "crate"),
            Err(reason) => {
                for field in used {
                    let name = field.field.name.as_ref().expect("used fields have names");
                    let key = format!("{canonical}::{name}");
                    if used_fields.contains(name) {
                        output.unsupported.insert(key.clone(), reason.clone());
                    }
                    if used_offsets.contains(name) {
                        output.offset_unsupported.insert(key, reason.clone());
                    }
                }
                continue;
            }
        };
        let Some(binding) = records_by_path.get(&storage).copied() else { continue };
        for projection in used {
            let field = projection.field;
            let name = field.name.as_ref().expect("used fields have names");
            let key = format!("{canonical}::{name}");
            let (first_projection, previous_offset) = match projections
                .entry((storage.clone(), name.clone()))
            {
                std::collections::btree_map::Entry::Vacant(entry) => (true, entry.insert(None)),
                std::collections::btree_map::Entry::Occupied(entry) => (false, entry.into_mut()),
            };
            if used_offsets.contains(name) {
                match offset_adapter(&projection, &lowering, declarations) {
                    Ok(adapter) => {
                        let identity = OffsetIdentity {
                            record: record.identity.clone(),
                            member: adapter
                                .member_record
                                .as_ref()
                                .map(|canonical| declarations.records[canonical].identity.clone()),
                            marker: adapter.member_marker.clone(),
                            path: adapter.offset.clone(),
                        };
                        if previous_offset.as_ref().is_some_and(|previous| *previous != identity) {
                            output.offset_unsupported.insert(
                                key.clone(),
                                "canonical record spellings disagree on the shared Rust offset capability".into(),
                            );
                            continue;
                        }
                        let field_marker = register_marker(&mut output, &field_ids, name)?;
                        if record_bridges.insert(storage.clone()) {
                            record_bridge(
                                &mut output.rust,
                                &mut layout_checks,
                                &storage,
                                record.size,
                                record.alignment,
                            );
                        }
                        for layout in adapter.layouts {
                            layout_checks.object(
                                &mut output.rust,
                                &layout.storage,
                                layout.size,
                                layout.alignment,
                            );
                            layout_checks.field(
                                &mut output.rust,
                                &layout.storage,
                                &layout.field,
                                layout.offset,
                            );
                        }
                        if previous_offset.is_none() {
                            writeln!(
                                output.rust,
                                "impl {EXPRESSION}::OffsetField<{field_marker}> for {EXPRESSION}::CRecord<{storage}> {{ type Member = {}; const OFFSET: usize = {}; }}",
                                adapter.member_marker,
                                adapter.offset,
                            )
                            .expect("String output");
                            *previous_offset = Some(identity);
                        }
                        output.offsets.insert(key.clone(), adapter.member_record);
                    }
                    Err(reason) => {
                        output.offset_unsupported.insert(key.clone(), reason);
                    }
                }
                if output.rust.len() > MAX_ADAPTER_BYTES {
                    return Err("generated field adapters exceed the 16 MiB source budget".into());
                }
            }
            if !used_fields.contains(name) {
                continue;
            }
            if field.bit_width.is_some() {
                if projection.steps.len() > 1 {
                    output.unsupported.insert(
                        key,
                        "promoted anonymous bitfield needs an original-C accessor path".into(),
                    );
                    continue;
                }
                let field_marker = field_ids.marker(name)?;
                match super::bitfields::generate(
                    canonical,
                    field,
                    binding,
                    &field_marker,
                    declarations,
                    bindings,
                    &lowering,
                ) {
                    Ok(adapter) => {
                        if let Some((size, alignment)) = record.size.zip(record.alignment) {
                            layout_checks.object(&mut output.rust, &storage, size, alignment);
                        }
                        if !first_projection {
                            continue;
                        }
                        register_marker(&mut output, &field_ids, name)?;
                        if record_bridges.insert(storage.clone()) {
                            record_bridge(
                                &mut output.rust,
                                &mut layout_checks,
                                &storage,
                                record.size,
                                record.alignment,
                            );
                        }
                        output.rust.push_str(&adapter.rust);
                        output.c_source.push_str(&adapter.c_source);
                    }
                    Err(reason) => {
                        output.unsupported.insert(key, reason);
                    }
                }
                if output.rust.len() > MAX_ADAPTER_BYTES
                    || output.c_source.len() > MAX_ADAPTER_BYTES
                {
                    return Err("generated field adapters exceed the 16 MiB source budget".into());
                }
                continue;
            }
            let validated =
                validate_projection(&projection, record.alignment, &lowering, declarations);
            let ValidatedProjection {
                mut marker,
                alignment: field_alignment,
                storage: projected_storage,
                flexible,
                offset,
            } = match validated {
                Ok(projection) => projection,
                Err(reason) => {
                    output.unsupported.insert(key, reason);
                    continue;
                }
            };
            let field_marker = register_marker(&mut output, &field_ids, name)?;
            if let Some((size, alignment)) = record.size.zip(record.alignment) {
                layout_checks.object(&mut output.rust, &storage, size, alignment);
            }
            if record_bridges.insert(storage.clone()) {
                record_bridge(
                    &mut output.rust,
                    &mut layout_checks,
                    &storage,
                    record.size,
                    record.alignment,
                );
            }
            for step in &projection.steps {
                let parent_storage = lowering
                    .resolve(&declarations.type_shapes[step.canonical].ty)?
                    .storage
                    .replace("$crate", "crate");
                let offset = step.field.offset_bits.expect("validated field offset") / 8;
                let rust_field = &step.binding.rust_name;
                if let Some((size, alignment)) = declarations.records[step.canonical]
                    .size
                    .zip(declarations.records[step.canonical].alignment)
                {
                    layout_checks.object(&mut output.rust, &parent_storage, size, alignment);
                }
                layout_checks.field(&mut output.rust, &parent_storage, rust_field, offset);
                let actual = lowering.storage_type(&step.binding.ty, 0)?.replace("$crate", "crate");
                layout_checks.projection(&mut output.rust, &parent_storage, rust_field, &actual);
                if let Some(size) = step.field.ty.size {
                    layout_checks.object(
                        &mut output.rust,
                        &actual,
                        size,
                        step.field
                            .ty
                            .alignment
                            .ok_or("projected field has no compiler alignment")?,
                    );
                }
            }
            if flexible {
                layout_checks.object(&mut output.rust, &projected_storage, 0, field_alignment);
            }
            if !first_projection {
                continue;
            }
            let qualification = if projection.steps.iter().any(|step| step.field.ty.is_const) {
                "FieldReadOnly"
            } else {
                "FieldReadWrite"
            };
            let volatile = projection.steps.iter().any(|step| step.field.ty.is_volatile);
            if volatile && !field.ty.is_volatile {
                marker = format!("{EXPRESSION}::CVolatile<{marker}>");
            }
            // Shared access metadata derives from the storage and qualifiers
            // proved above, including the flexible-array wrapper's alignment.
            if !qualifiers_imported {
                writeln!(
                    output.rust,
                    "use {EXPRESSION}::{{ReadOnly as FieldReadOnly, ReadWrite as FieldReadWrite}};"
                )
                .expect("String output");
                qualifiers_imported = true;
            }
            writeln!(output.rust,
                "// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.\n\
                 unsafe impl {EXPRESSION}::OrdinaryField<{field_marker}> for {EXPRESSION}::CRecord<{storage}> {{\n\
                   type Member = {marker};\n\
                   type Declared = {qualification};\n\
                   const OFFSET: usize = {offset};\n\
                 }}").expect("String output");
            pending_records.push(field.ty.clone());
            if output.rust.len() > MAX_ADAPTER_BYTES {
                return Err("generated field adapters exceed the 16 MiB source budget".into());
            }
        }
    }
    let mut seen_types = BTreeSet::new();
    while let Some(ty) = pending_records.pop() {
        if !seen_types.insert(ty.canonical_spelling.clone()) {
            continue;
        }
        if ty.category == TypeCategory::Pointer {
            if let Ok(pointee) = lowering.pointer_pointee(&ty) {
                pending_records.push(pointee);
            }
            continue;
        }
        if let Some(crate::TypeShape { kind: TypeShapeKind::Array { element, .. }, .. }) =
            declarations.type_shapes.get(&ty.canonical_spelling)
        {
            pending_records.push(element.clone());
            continue;
        }
        if ty.category == TypeCategory::Record {
            let Ok(lowered) = lowering.resolve(&ty) else { continue };
            let storage = lowered.storage.replace("$crate", "crate");
            if record_bridges.insert(storage.clone()) {
                if !records_by_path.contains_key(&storage) {
                    continue;
                }
                let Some(record) = declarations.records.get(&ty.canonical_spelling) else {
                    continue;
                };
                record_bridge(
                    &mut output.rust,
                    &mut layout_checks,
                    &storage,
                    record.size,
                    record.alignment,
                );
            }
        }
    }
    if output.rust.len() > MAX_ADAPTER_BYTES {
        return Err("generated field adapters exceed the 16 MiB source budget".into());
    }
    Ok(output)
}

/// One compiler record edge paired with the actual Rust field used to project it.
#[derive(Clone, Copy)]
struct ProjectionStep<'a> {
    /// Compiler parent record spelling anchoring the layout for this step.
    canonical: &'a str,
    /// Original field facts, including qualifiers, offsets, and any bit width.
    field: &'a FieldInfo,
    /// Actual binding field name and storage used by the typed Rust projection.
    binding: &'a FieldBinding,
}
/// A requested leaf reached through direct fields or anonymous member promotions.
struct Projection<'a> {
    /// Compiler leaf field whose declared type and access semantics must be preserved.
    field: &'a FieldInfo,
    /// Anchored parent-to-leaf path; empty paths retain an unsupported witness.
    steps: Vec<ProjectionStep<'a>>,
}

/// Validated offset expression and layout witnesses without member-load requirements.
struct OffsetAdapter {
    /// Containing layouts and per-step offsets that Rust must agree with at compilation.
    layouts: Vec<OffsetLayout>,
    /// Checked Rust offset expression for the complete promoted member path.
    offset: String,
    /// Nested record marker, or unit for an offset-only leaf.
    member_marker: String,
    /// Compiler identity allowing further offsetof steps through a complete nested record.
    member_record: Option<String>,
}

/// Qualified spellings may share one Rust implementation only when they retain
/// the compiler record identities and the same anchored storage path.
#[derive(PartialEq, Eq)]
struct OffsetIdentity {
    /// Nominal C parent record retained when sharing a qualified spelling’s implementation.
    record: String,
    /// Nested member record identity, absent for an offset-only leaf.
    member: Option<String>,
    /// Semantic member marker used by the offset capability.
    marker: String,
    /// Actual binding projection path that must match before an implementation is shared.
    path: String,
}

/// Compiler facts needed to anchor one Rust offset step to its containing record.
struct OffsetLayout {
    /// Actual Rust parent record type used in offset and layout assertions.
    storage: String,
    /// Compiler-established containing record size in bytes.
    size: u64,
    /// Compiler-established containing record alignment in bytes.
    alignment: u64,
    /// Actual Rust member name used in the offset witness.
    field: String,
    /// Compiler field offset in bytes, checked against Rust rather than substituted blindly.
    offset: u64,
}

/// Deduplicate identical witnesses while retaining every distinct or conflicting layout fact.
#[derive(Default)]
struct LayoutChecks {
    /// Emitted record size/alignment tuples; differing compiler facts remain separate assertions.
    objects: BTreeSet<(String, u64, u64)>,
    /// Emitted parent/member/offset tuples, including conflicting offsets.
    fields: BTreeSet<(String, String, u64)>,
    /// Typed field storage witnesses, preventing equal-layout type substitution.
    projections: BTreeSet<(String, String, String)>,
    /// Whether the shared projection witness alias has already been written.
    projection_imported: bool,
}

/// Share only identical layout facts and retain typed projection or disagreement witnesses.
impl LayoutChecks {
    /// Emit a size/alignment witness once for each exact storage and compiler-fact tuple.
    fn object(&mut self, rust: &mut String, storage: &str, size: u64, alignment: u64) {
        if self.objects.insert((storage.to_owned(), size, alignment)) {
            writeln!(rust, "const _: () = {{ assert!(::core::mem::size_of::<{storage}>() == {size}); assert!(::core::mem::align_of::<{storage}>() == {alignment}); }};").expect("String output");
        }
    }

    /// Emit an offset witness once per exact fact without merging disagreeing offsets.
    fn field(&mut self, rust: &mut String, storage: &str, field: &str, offset: u64) {
        if self.fields.insert((storage.to_owned(), field.to_owned(), offset)) {
            writeln!(
                rust,
                "const _: () = assert!(::core::mem::offset_of!({storage}, {field}) == {offset});"
            )
            .expect("String output");
        }
    }

    /// Prove the Rust field’s exact storage through an uncalled typed address projection.
    fn projection(&mut self, rust: &mut String, parent: &str, field: &str, storage: &str) {
        if self.projections.insert((parent.to_owned(), field.to_owned(), storage.to_owned())) {
            if !self.projection_imported {
                writeln!(rust, "use {EXPRESSION}::FieldProjection as Projection;")
                    .expect("String output");
                self.projection_imported = true;
            }
            writeln!(rust, "// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.\nconst _: Projection<{parent}, {storage}> = |base| unsafe {{ ::core::ptr::addr_of_mut!((*base).{field}) }};").expect("String output");
        }
    }
}

/// Emit each catalog-owned member marker once and expose its hygienic macro-facing path.
fn register_marker(
    output: &mut FieldAdapters,
    ids: &FieldIds<'_>,
    name: &str,
) -> Result<String, String> {
    let marker = ids.marker(name)?;
    if let std::collections::btree_map::Entry::Vacant(entry) = output.markers.entry(name.to_owned())
    {
        // Distinct local markers retain separate nominal trait identities and
        // the locality required by the generated field capability implementations.
        entry.insert(format!("$crate::__pgrx_c_generated::{marker}"));
        writeln!(output.rust, "#[doc(hidden)] pub struct {marker};").expect("String output");
    }
    Ok(marker)
}

/// Offsets require the containing layout and actual field path, not a loadable
/// leaf type. In particular, packing, volatility and flexible arrays do not
/// create a memory-access obligation here.
fn offset_adapter(
    projection: &Projection<'_>,
    lowering: &Lowering<'_>,
    declarations: &DeclarationCatalog,
) -> Result<OffsetAdapter, String> {
    if projection.field.bit_width.is_some() {
        return Err("C offsetof cannot name a bitfield".into());
    }
    if projection.steps.is_empty() {
        return Err("field has no extractable compiler-anchored Rust offset path".into());
    }
    let mut layouts = Vec::with_capacity(projection.steps.len());
    let mut offset = String::new();
    let mut compiler_offset = 0u64;
    for step in &projection.steps {
        let parent = declarations
            .records
            .get(step.canonical)
            .ok_or("offset parent has no compiler record layout")?;
        let (size, alignment) = parent
            .size
            .zip(parent.alignment.filter(|alignment| *alignment > 0))
            .ok_or("C offsetof requires a complete containing record")?;
        let shape = declarations
            .type_shapes
            .get(step.canonical)
            .ok_or("offset parent has no compiler-owned type shape")?;
        let storage = lowering.resolve(&shape.ty)?.storage.replace("$crate", "crate");
        let position = step.field.offset_bits.ok_or("compiler did not establish field offset")?;
        if step.field.bit_width.is_some() || position % 8 != 0 {
            return Err("C offsetof requires an ordinary field on a byte boundary".into());
        }
        let position = position / 8;
        if position > size {
            return Err("compiler field offset exceeds its containing record".into());
        }
        compiler_offset = compiler_offset
            .checked_add(position)
            .ok_or("promoted field offset exceeds the compiler representation")?;
        let rust_field = &step.binding.rust_name;
        if offset.is_empty() {
            write!(offset, "::core::mem::offset_of!({storage}, {rust_field})")
                .expect("String output");
        } else {
            write!(offset, ".checked_add(::core::mem::offset_of!({storage}, {rust_field})).expect(\"C promoted field offset exceeds usize\")").expect("String output");
        }
        layouts.push(OffsetLayout {
            storage,
            size,
            alignment,
            field: rust_field.clone(),
            offset: position,
        });
    }
    let root = declarations.records[projection.steps[0].canonical]
        .size
        .expect("each offset parent is complete");
    if compiler_offset > root {
        return Err("promoted field offset exceeds its containing record".into());
    }
    let mut member_marker = "()".to_owned();
    let mut member_record = None;
    if projection.field.ty.category == TypeCategory::Record
        && declarations
            .records
            .get(&projection.field.ty.canonical_spelling)
            .is_some_and(|record| record.size.is_some() && record.alignment.is_some())
    {
        let mut ty = projection.field.ty.clone();
        // offsetof does not access the member. Qualifiers must not impose the
        // value representation or access restrictions of a field projection.
        ty.is_volatile = false;
        if let Ok(lowered) = lowering.resolve_with_storage(
            &ty,
            &projection.steps.last().expect("nonempty offset path").binding.ty,
        ) {
            member_marker = lowered.marker.replace("$crate", "crate");
            member_record = Some(ty.canonical_spelling);
        }
    }
    Ok(OffsetAdapter { layouts, offset, member_marker, member_record })
}

/// Follow actual binding edges through anonymous promotions, retaining missing-path failures.
fn collect_projections<'a>(
    canonical: &'a str,
    declarations: &'a DeclarationCatalog,
    lowering: &Lowering<'a>,
    used: &RequestedFields,
    prefix: &mut Vec<ProjectionStep<'a>>,
    output: &mut Vec<Projection<'a>>,
    depth: usize,
) {
    if depth > 64 {
        return;
    }
    let Some(record) = declarations.records.get(canonical) else { return };
    let Some(shape) = declarations.type_shapes.get(canonical) else { return };
    let Ok(_binding) = lowering.record_binding(&shape.ty) else {
        for field in &record.fields {
            if field.name.as_ref().is_some_and(|name| used.contains(name)) {
                output.push(Projection { field, steps: Vec::new() });
            }
        }
        return;
    };
    for (index, field) in record.fields.iter().enumerate() {
        let actual = if let Some(name) = &field.name {
            lowering.named_field_binding(canonical, name)
        } else {
            lowering.anonymous_field_binding(canonical, index)
        };
        if let Some(actual) = actual {
            prefix.push(ProjectionStep { canonical, field, binding: actual });
            if field.name.as_ref().is_some_and(|name| used.contains(name)) {
                output.push(Projection { field, steps: prefix.clone() });
            }
            if field.name.is_none() && field.ty.category == TypeCategory::Record {
                collect_projections(
                    &field.ty.canonical_spelling,
                    declarations,
                    lowering,
                    used,
                    prefix,
                    output,
                    depth + 1,
                );
            }
            prefix.pop();
        } else if field.name.as_ref().is_some_and(|name| used.contains(name)) {
            output.push(Projection { field, steps: Vec::new() });
        }
    }
}

/// Access facts established before emitting a member capability or raw address projection.
struct ValidatedProjection {
    /// Declared semantic member type preserving C identity and qualification.
    marker: String,
    /// Required leaf alignment used to select aligned or unaligned access.
    alignment: u64,
    /// Exact Rust member storage reconciled with its C declaration.
    storage: String,
    /// Whether the leaf is a compiler-owned incomplete array requiring element-based access.
    flexible: bool,
    /// Complete parent-to-leaf byte offset established by compiler field facts.
    offset: u64,
}

/// Prove every path step’s storage and offset before admitting access to the leaf.
///
/// Incomplete arrays use their element alignment; unaligned volatile loads remain
/// rejected because no verified runtime access can satisfy both requirements.
fn validate_projection(
    projection: &Projection<'_>,
    record_alignment: Option<u64>,
    lowering: &Lowering<'_>,
    declarations: &DeclarationCatalog,
) -> Result<ValidatedProjection, String> {
    if projection.steps.is_empty() {
        return Err("field has no extractable compiler-anchored Rust projection path".into());
    }
    let field = projection.field;
    if field.bit_width.is_some() {
        return Err("bit-field projection requires an original-C storage adapter".into());
    }
    let mut offset = 0_u64;
    for step in &projection.steps {
        lowering.resolve_with_storage(&step.field.ty, &step.binding.ty)?;
        if step.field.ty.size.is_some() && step.field.ty.alignment.is_none() {
            return Err("projected field has no compiler alignment".into());
        }
        let position = step.field.offset_bits.ok_or("compiler did not establish field offset")?;
        if position % 8 != 0 {
            return Err("ordinary field does not begin on a C byte boundary".into());
        }
        offset = offset.checked_add(position / 8).ok_or("field offset exceeds representation")?;
    }
    let final_step = projection.steps.last().expect("nonempty projection");
    let flexible = matches!(
        declarations.type_shapes.get(&field.ty.canonical_spelling).map(|shape| &shape.kind),
        Some(TypeShapeKind::Array { array_kind: ArrayKind::Incomplete, .. })
    );
    let alignment = if flexible {
        let Some(crate::TypeShape { kind: TypeShapeKind::Array { element, .. }, .. }) =
            declarations.type_shapes.get(&field.ty.canonical_spelling)
        else {
            unreachable!("flexible array shape")
        };
        element.alignment
    } else {
        field.ty.size.ok_or("compiler did not establish field size")?;
        field.ty.alignment
    }
    .filter(|alignment| *alignment > 0)
    .ok_or("compiler did not establish field alignment")?;
    let unaligned = record_alignment.ok_or("compiler did not establish record alignment")?
        < alignment
        || !offset.is_multiple_of(alignment);
    if projection.steps.iter().any(|step| step.field.ty.is_volatile) && unaligned {
        return Err("unaligned volatile field access has no verified runtime implementation".into());
    }
    let lowered = lowering.resolve_with_storage(&field.ty, &final_step.binding.ty)?;
    Ok(ValidatedProjection {
        marker: lowered.marker.replace("$crate", "crate"),
        alignment,
        storage: lowered.storage.replace("$crate", "crate"),
        flexible,
        offset,
    })
}

/// Register complete native records with layout checks, or incomplete records as opaque storage.
fn record_bridge(
    rust: &mut String,
    checks: &mut LayoutChecks,
    storage: &str,
    size: Option<u64>,
    alignment: Option<u64>,
) {
    if size.is_none() || alignment.is_none() {
        writeln!(rust, "impl c::sealed::Sealed for {storage} {{}}\nimpl {EXPRESSION}::NativeType for {storage} {{ type Marker = {EXPRESSION}::COpaque<Self>; }}").expect("String output");
        return;
    }
    let size = size.expect("complete record size");
    let alignment = alignment.expect("complete record alignment");
    checks.object(rust, storage, size, alignment);
    writeln!(rust, "impl {EXPRESSION}::NativeRecord for {storage} {{}}").expect("String output");
}

/// Layout, nominal field identity, and C-oracle checks for projections and offset-only capabilities.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::SCANNER_LOCK;
    use crate::{FieldBinding, MacroScanner, RecordBinding, RecordKind, RustBindingType, inspect};
    use std::path::PathBuf;

    /// C oracle compilation support used to compare generated access against the original header.
    mod oracle {
        //! Execute original-header C witnesses independently of field lowering.
        //!
        //! The shared harness owns compiler artifacts and bounds process runtime
        //! and output. Its native observations establish layout and access behavior;
        //! translated Rust or existing pgrx ports never supply the expected result.
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/oracle.rs"));
    }
    /// Rust consumer compilation support used to exercise capability and negative type checks.
    mod rust_oracle {
        //! Compile actual generated field adapters in independent Rust consumers.
        //!
        //! Positive consumers are compared with the original C witnesses, while
        //! negative consumers must fail type checking before linking. The shared
        //! harness isolates artifacts and enforces the same process limits as C.
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/rust_oracle.rs"));
    }

    /// Build immutable record/member catalogs for field identity stability tests.
    fn registry_catalog(records: &[(&str, &[&str])]) -> DeclarationCatalog {
        let mut catalog = DeclarationCatalog::default();
        for (record, names) in records {
            catalog.records.insert(
                (*record).into(),
                crate::RecordInfo {
                    identity: (*record).into(),
                    name: Some((*record).into()),
                    kind: RecordKind::Struct,
                    fields: names
                        .iter()
                        .map(|name| FieldInfo {
                            name: Some((*name).into()),
                            ty: TypeInfo {
                                spelling: "unsupported".into(),
                                canonical_spelling: "unsupported".into(),
                                category: TypeCategory::Other,
                                size: None,
                                alignment: None,
                                is_const: false,
                                is_volatile: false,
                            },
                            offset_bits: None,
                            bit_width: None,
                            is_anonymous: false,
                        })
                        .collect(),
                    size: None,
                    alignment: None,
                    is_anonymous: false,
                },
            );
        }
        catalog
    }

    /// Construct an empty output accumulator to test local marker registration independently.
    fn empty_adapters() -> FieldAdapters {
        FieldAdapters {
            rust: String::new(),
            c_source: String::new(),
            markers: BTreeMap::new(),
            unsupported: BTreeMap::new(),
            offsets: BTreeMap::new(),
            offset_unsupported: BTreeMap::new(),
        }
    }

    /// Extract the locally defined marker name from its hygienic crate-relative path.
    fn local_marker<'a>(adapters: &'a FieldAdapters, name: &str) -> &'a str {
        adapters.markers[name]
            .strip_prefix("$crate::__pgrx_c_generated::")
            .expect("generated markers refer to the defining crate's local types")
    }

    /// Prove field IDs are deterministic, unique, and independent of record insertion order.
    #[test]
    fn field_ids_are_sorted_unique_and_independent_of_record_order() {
        let original = registry_catalog(&[
            ("Second", &["z", "a", "self", "type"]),
            ("First", &["z", "_", "a_", "a"]),
        ]);
        let reordered = registry_catalog(&[
            ("First", &["a", "a_", "_", "z"]),
            ("Second", &["type", "self", "a", "z"]),
        ]);
        let ids = FieldIds::new(&original).unwrap();
        let reordered = FieldIds::new(&reordered).unwrap();
        assert_eq!(ids.names, reordered.names);
        assert_eq!(
            ids.names,
            BTreeMap::from([("_", 0), ("a", 1), ("a_", 2), ("self", 3), ("type", 4), ("z", 5)])
        );
        assert_eq!(ids.marker("self").unwrap(), "Field3");
        assert_ne!(ids.marker("a").unwrap(), ids.marker("a_").unwrap());
    }

    /// Check pruning keeps stable field IDs and repeated requests emit one nominal marker.
    #[test]
    fn field_ids_preserve_subset_identity_and_emit_each_local_marker_once() {
        let catalog = registry_catalog(&[("Owner", &["unrequested", "z", "a", "self"])]);
        let ids = FieldIds::new(&catalog).unwrap();
        let mut original = empty_adapters();
        register_marker(&mut original, &ids, "z").unwrap();
        register_marker(&mut original, &ids, "self").unwrap();
        register_marker(&mut original, &ids, "z").unwrap();
        let mut subset = empty_adapters();
        register_marker(&mut subset, &FieldIds::new(&catalog).unwrap(), "z").unwrap();
        assert_eq!(original.markers["z"], subset.markers["z"]);
        assert_eq!(local_marker(&original, "z"), "Field3");
        assert_ne!(original.markers["z"], original.markers["self"]);
        assert_eq!(original.rust.matches("pub struct ").count(), 2);
        assert_eq!(original.rust.matches("pub struct Field3;").count(), 1);
        assert_eq!(original.rust.matches("pub struct Field1;").count(), 1);
        assert_eq!(subset.rust.matches("pub struct Field3;").count(), 1);
        assert!(!subset.rust.contains("pub struct Field1;"));
        assert_eq!(original.markers.len(), 2);
        assert!(!original.markers.contains_key("unrequested"));
        syn::parse_file(&original.rust).unwrap();
    }

    /// Check empty catalogs and missing fields cannot create unanchored capability identities.
    #[test]
    fn empty_and_missing_field_ids_emit_no_markers_or_fabricated_identity() {
        let catalog = DeclarationCatalog::default();
        let ids = FieldIds::new(&catalog).unwrap();
        assert!(ids.names.is_empty());
        let mut output = empty_adapters();
        let error = register_marker(&mut output, &ids, "missing").unwrap_err();
        assert!(error.contains("missing") && error.contains("declaration-catalog identity"));
        assert!(output.rust.is_empty());
        assert!(output.markers.is_empty());
    }

    /// Prove deduplication removes only identical layout witnesses and retains conflicting assertions.
    #[test]
    fn layout_checks_share_identical_facts_but_retain_disagreements() {
        let mut checks = LayoutChecks::default();
        let mut rust = String::new();
        for _ in 0..2 {
            checks.object(&mut rust, "crate::Parent", 16, 8);
            checks.field(&mut rust, "crate::Parent", "member", 8);
            checks.projection(&mut rust, "crate::Parent", "member", "u32");
        }
        assert_eq!(rust.matches("size_of::<crate::Parent>").count(), 1);
        assert_eq!(rust.matches("offset_of!(crate::Parent, member)").count(), 1);
        assert_eq!(rust.matches("Projection<crate::Parent, u32>").count(), 1);
        checks.object(&mut rust, "crate::Parent", 24, 8);
        checks.field(&mut rust, "crate::Parent", "member", 16);
        checks.projection(&mut rust, "crate::Parent", "member", "i32");
        assert_eq!(rust.matches("size_of::<crate::Parent>").count(), 2);
        assert_eq!(rust.matches("offset_of!(crate::Parent, member)").count(), 2);
        assert_eq!(rust.matches("Projection<crate::Parent, i32>").count(), 1);
        assert_eq!(rust.matches("use c::expression::FieldProjection").count(), 1);
    }

    /// Check the shared projection alias reduces repeated pointer/function AST shapes in layout witnesses.
    #[test]
    fn projection_alias_removes_repeated_function_shape_types() {
        use syn::visit_mut::{self, VisitMut};
        /// AST counter measuring witness type-shape duplication without compiling generated snippets.
        #[derive(Default)]
        struct Types {
            /// Total visited Rust type nodes in the parsed projection witness.
            total: usize,
            /// Function type nodes retained by the witness representation.
            functions: usize,
            /// Raw pointer type nodes retained by the witness representation.
            pointers: usize,
        }
        /// Traverse parsed witness types to measure structural duplication removed by the projection alias.
        impl VisitMut for Types {
            /// Count nested type shapes while delegating recursion to syn’s standard visitor.
            fn visit_type_mut(&mut self, ty: &mut syn::Type) {
                self.total += 1;
                self.functions += usize::from(matches!(ty, syn::Type::BareFn(_)));
                self.pointers += usize::from(matches!(ty, syn::Type::Ptr(_)));
                visit_mut::visit_type_mut(self, ty);
            }
        }
        let mut old = syn::parse_file("const _: unsafe fn(*mut Parent) -> *mut u32 = |base| unsafe { core::ptr::addr_of_mut!((*base).member) };").unwrap();
        let mut checks = LayoutChecks::default();
        let mut rust = String::new();
        checks.projection(&mut rust, "Parent", "member", "u32");
        let mut new = syn::parse_file(&rust).unwrap();
        let (mut before, mut after) = (Types::default(), Types::default());
        before.visit_file_mut(&mut old);
        after.visit_file_mut(&mut new);
        assert_eq!(before.total - after.total, 2);
        assert_eq!(before.functions, 1);
        assert_eq!(before.pointers, 2);
        assert_eq!(after.functions, 0);
        assert_eq!(after.pointers, 0);
    }

    /// Locate the original C field-oracle header used for layout and access comparisons.
    fn fixture() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/field_adapters.h")
    }

    /// Supply the fixture’s include path and compiler profile for field analysis.
    fn arguments() -> Vec<String> {
        let mut arguments = vec!["-std=c11".into()];
        #[cfg(target_os = "macos")]
        {
            let sdk = rust_oracle::run_tool(
                std::process::Command::new("xcrun").arg("--show-sdk-path"),
                "locate the C oracle SDK",
            );
            arguments.extend(["-isysroot".into(), sdk.trim().into()]);
        }
        arguments
    }

    /// Check concrete owner demand retains equivalent compiler spellings and required layout witnesses.
    #[test]
    fn concrete_owner_requests_preserve_equivalent_layout_witnesses() {
        let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = MacroScanner::new().unwrap();
        let arguments = arguments();
        let frontend = inspect(&scanner, &fixture(), &arguments, None).unwrap();
        let mut declarations = frontend.declarations().clone();
        let mut qualified = declarations.records["struct Outer"].clone();
        let original_size = qualified.size.unwrap();
        qualified.size = Some(original_size + qualified.alignment.unwrap());
        declarations.records.insert("const struct Outer".into(), qualified);
        let mut shape = declarations.type_shapes["struct Outer"].clone();
        shape.ty.canonical_spelling = "const struct Outer".into();
        shape.ty.spelling = "const struct Outer".into();
        shape.ty.is_const = true;
        declarations.type_shapes.insert("const struct Outer".into(), shape);
        let mut requests = FieldRequests::default();
        requests.field(Some("struct Outer"), Some("count"));
        let generated =
            generate(&declarations, &bindings(), &requests, &frontend.profile().target, &[])
                .unwrap();
        assert!(generated.unsupported.is_empty());
        assert_eq!(generated.rust.matches("size_of::<crate::Outer>").count(), 2);
        assert_eq!(
            generated
                .rust
                .matches(&format!("::OrdinaryField<{}", local_marker(&generated, "count")))
                .count(),
            1
        );
        assert!(!generated.rust.contains("crate::Packed"));
        let required = requests.required_types(&declarations);
        assert!(!required.is_empty(), "selected field types must reach dependent capabilities");
        assert!(required.iter().all(|ty| {
            ty.canonical_spelling
                == declarations.records["struct Outer"].fields[2].ty.canonical_spelling
        }));

        let rust_bindings = bindgen::Builder::default()
            .header(fixture().to_str().unwrap())
            .clang_args(&arguments)
            .allowlist_type("Child|Outer")
            .layout_tests(false)
            .generate_comments(false)
            .generate()
            .unwrap()
            .to_string();
        let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap();
        let diagnostics = rust_oracle::reject_rust(&format!(
            "#[path = {support:?}] pub mod __pgrx_c_macros;\n{rust_bindings}\npub mod __pgrx_c_generated {{ use crate::__pgrx_c_macros as c; {} }}\nfn main() {{}}",
            generated.rust
        ));
        assert!(
            diagnostics.contains("E0080") && diagnostics.contains("assertion failed"),
            "{diagnostics}"
        );
    }

    /// Prove typed projections reject equal-sized wrong member types while accepting true storage aliases.
    #[test]
    fn typed_projection_rejects_equal_layout_field_substitution_but_accepts_aliases() {
        let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = MacroScanner::new().unwrap();
        let arguments = arguments();
        let frontend = inspect(&scanner, &fixture(), &arguments, None).unwrap();
        let mut requests = FieldRequests::default();
        requests.field(Some("struct Outer"), Some("count"));
        let generated = generate(
            frontend.declarations(),
            &bindings(),
            &requests,
            &frontend.profile().target,
            &[],
        )
        .unwrap();
        assert!(generated.unsupported.is_empty());
        let bindings = bindgen::Builder::default()
            .header(fixture().to_str().unwrap())
            .clang_args(&arguments)
            .allowlist_type("Child|Outer")
            .layout_tests(false)
            .generate_comments(false)
            .generate()
            .unwrap()
            .to_string();
        let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap();
        for (ty, accepted) in
            [(syn::parse_quote!(i32), false), (syn::parse_quote!(CountAlias), true)]
        {
            let mut edited = syn::parse_file(&bindings).unwrap();
            let record = edited
                .items
                .iter_mut()
                .find_map(|item| match item {
                    syn::Item::Struct(record) if record.ident == "Outer" => Some(record),
                    _ => None,
                })
                .unwrap();
            record
                .fields
                .iter_mut()
                .find(|field| field.ident.as_ref().unwrap() == "count")
                .unwrap()
                .ty = ty;
            let program = format!(
                "#[path={support:?}] pub mod __pgrx_c_macros;\ntype CountAlias=u32;\n{}\npub mod __pgrx_c_generated {{ use crate::__pgrx_c_macros as c; {} }}\nfn main() {{ println!(\"ok\"); }}",
                quote::quote!(#edited),
                generated.rust,
            );
            if accepted {
                assert_eq!(rust_oracle::run_rust(&program), "ok\n");
            } else {
                let error = rust_oracle::reject_rust(&program);
                assert!(error.contains("E0308"), "exact field type must be checked: {error}");
                assert!(
                    !error.contains("E0080"),
                    "layout agrees; the typed witness must reject it"
                );
            }
        }
    }

    /// Check ManuallyDrop fields preserve the inner member storage in their projection witness.
    #[test]
    fn typed_projection_witness_retains_manually_drop_inner_storage() {
        let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap();
        let mut checks = LayoutChecks::default();
        let mut rust = String::new();
        checks.object(&mut rust, "crate::Parent", 4, 4);
        checks.field(&mut rust, "crate::Parent", "member", 0);
        checks.projection(&mut rust, "crate::Parent", "member", "core::mem::ManuallyDrop<u32>");
        for (inner, accepted) in [("u32", true), ("i32", false)] {
            let program = format!(
                "#[path={support:?}] pub mod __pgrx_c_macros;\n#[repr(C)]pub struct Parent {{ member:core::mem::ManuallyDrop<{inner}> }}\nuse crate::__pgrx_c_macros as c;\n{rust}\nfn main() {{ println!(\"ok\"); }}"
            );
            if accepted {
                assert_eq!(rust_oracle::run_rust(&program), "ok\n");
            } else {
                let error = rust_oracle::reject_rust(&program);
                assert!(error.contains("E0308"), "the wrapper's inner storage must match: {error}");
                assert!(!error.contains("E0080"), "equal wrapper layout alone is insufficient");
            }
        }
    }

    /// Describe fixture Rust records and fields independently of the compiler’s C layout facts.
    fn bindings() -> BindingCatalog {
        let unsigned = RustBindingType::Integer { signed: false, bits: 32 };
        let signed = RustBindingType::Integer { signed: true, bits: 32 };
        let child = RustBindingType::Named { path: vec!["Child".into()] };
        let records = [
            (
                "Child",
                false,
                RecordKind::Struct,
                vec![("value", unsigned.clone()), ("frozen", unsigned.clone())],
            ),
            (
                "Outer",
                false,
                RecordKind::Struct,
                vec![
                    ("child", child.clone()),
                    ("next", RustBindingType::Pointer { pointee: Box::new(child), mutable: true }),
                    ("count", unsigned.clone()),
                    ("other", RustBindingType::Bool),
                ],
            ),
            (
                "Packed",
                true,
                RecordKind::Struct,
                vec![
                    ("lead", RustBindingType::Integer { signed: true, bits: 8 }),
                    ("count", unsigned.clone()),
                    ("signal", RustBindingType::Integer { signed: false, bits: 8 }),
                ],
            ),
            ("Volatile", false, RecordKind::Struct, vec![("signal", unsigned.clone())]),
            (
                "Value",
                false,
                RecordKind::Union,
                vec![("count", unsigned.clone()), ("signed_value", signed)],
            ),
            (
                "Limitations",
                false,
                RecordKind::Struct,
                vec![(
                    "array",
                    RustBindingType::Array { element: Box::new(unsigned.clone()), length: 2 },
                )],
            ),
            ("VolatilePacked", true, RecordKind::Struct, vec![("signal", unsigned)]),
        ];
        let mut catalog = BindingCatalog::default();
        for (name, packed, kind, fields) in records {
            catalog.records.insert(
                name.into(),
                RecordBinding {
                    path: vec![name.into()],
                    kind,
                    packed,
                    copy: true,
                    fields: fields
                        .into_iter()
                        .map(|(name, ty)| {
                            (name.into(), FieldBinding { rust_name: name.into(), ty })
                        })
                        .collect(),
                },
            );
        }
        catalog
    }

    /// Compare generated field access with C using partially initialized records and promoted members.
    #[test]
    fn generated_field_projections_match_original_c_without_whole_record_reads() {
        let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = MacroScanner::new().unwrap();
        let arguments = arguments();
        let frontend = inspect(&scanner, &fixture(), &arguments, None).unwrap();
        let mut requests = FieldRequests::default();
        for name in ["count", "value", "child", "next", "frozen", "signal", "array"] {
            requests.field(None, Some(name));
        }
        let generated = generate(
            frontend.declarations(),
            &bindings(),
            &requests,
            &frontend.profile().target,
            &[],
        )
        .unwrap();
        assert!(generated.unsupported.values().any(|reason| reason.contains("unaligned volatile")));
        assert_eq!(generated.rust.matches("NativeRecord for crate::Child").count(), 1);
        let rust_bindings = bindgen::Builder::default()
            .header(fixture().to_str().unwrap())
            .clang_args(&arguments)
            .allowlist_type("Child|Outer|Packed|Volatile|Value|Limitations")
            .derive_default(false)
            .layout_tests(false)
            .generate()
            .unwrap()
            .to_string();
        let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap();
        let mut rust = format!(
            "#[path = {support:?}] pub mod __pgrx_c_macros;\n{rust_bindings}\npub mod __pgrx_c_generated {{ use crate::__pgrx_c_macros as c; {} }}\n",
            generated.rust
        );
        rust.push_str(&format!(r#"
use __pgrx_c_macros::expression::*;
fn main() {{
    let mut outer = core::mem::MaybeUninit::<Outer>::uninit();
    let base = native_place(outer.as_mut_ptr());
    unsafe {{
        let count = project::<__pgrx_c_generated::{count}, _, _>(base);
        assign(count, input(259_i32));
        let child = project::<__pgrx_c_generated::{child}, _, _>(base);
        let value = project::<__pgrx_c_generated::{value}, _, _>(child);
        assign(value, input(11_i32));
        let next = project::<__pgrx_c_generated::{next}, _, _>(base);
        assign(next, input(core::ptr::addr_of_mut!((*outer.as_mut_ptr()).child)));
        let indirect = project::<__pgrx_c_generated::{value}, _, _>(pointee(load(next)));
        let mut packed = core::mem::MaybeUninit::<Packed>::uninit();
        let packed = native_place(packed.as_mut_ptr());
        let packed_count = project::<__pgrx_c_generated::{count}, _, _>(packed);
        assert!(packed_count.access().unaligned);
        assign(packed_count, input(641_i32));
        let packed_signal = project::<__pgrx_c_generated::{signal}, _, _>(packed);
        assert!(packed_signal.access().volatile && !packed_signal.access().unaligned);
        assign(packed_signal, input(7_i32));
        let mut volatile = core::mem::MaybeUninit::<Volatile>::uninit();
        let signal = project::<__pgrx_c_generated::{signal}, _, _>(native_place(volatile.as_mut_ptr()));
        assert!(signal.access().volatile);
        assign(signal, input(19_i32));
        let mut union = core::mem::MaybeUninit::<Value>::uninit();
        let union_count = project::<__pgrx_c_generated::{count}, _, _>(native_place(union.as_mut_ptr()));
        assign(union_count, input(23_i32));
        let mut limitations = core::mem::MaybeUninit::<Limitations>::uninit();
        let array = project::<__pgrx_c_generated::{array}, _, _>(native_place(limitations.as_mut_ptr()));
        let element = index(load(array), input(1_i32));
        assign(element, input(-1_i32));
        let _: Place<__pgrx_c_macros::CUnsignedInt, ReadOnly> = project::<__pgrx_c_generated::{frozen}, _, _>(child);
        let const_count = project::<__pgrx_c_generated::{count}, _, _>(native_const_place(outer.as_ptr()));
        let _: Place<__pgrx_c_macros::CUnsignedInt, ReadOnly> = const_count;
        println!("{{}},{{}},{{}},{{}},{{}},{{}},{{}},{{}}", load(count).get(), load(value).get(), load(indirect).get(), load(packed_count).get(), load(packed_signal).get(), load(signal).get(), load(union_count).get(), load(element).get());
    }}
}}
"#, count=local_marker(&generated, "count"), child=local_marker(&generated, "child"), value=local_marker(&generated, "value"), next=local_marker(&generated, "next"), signal=local_marker(&generated, "signal"), frozen=local_marker(&generated, "frozen"), array=local_marker(&generated, "array")));
        let actual = rust_oracle::run_rust(&rust);
        let expected = oracle::run_c(
            &frontend.profile().compiler.executable,
            &fixture(),
            r#"
#include <stdio.h>
int main(void) {
    struct Outer outer;
    SET_COUNT(&outer, 259);
    outer.child.value = 11;
    outer.next = &outer.child;
    struct Packed packed;
    SET_COUNT(&packed, 641);
    packed.signal = 7;
    struct Volatile observable;
    observable.signal = 19;
    union Value value;
    SET_COUNT(&value, 23);
    struct Limitations limitations;
    limitations.array[1] = -1;
    printf("%u,%u,%u,%u,%u,%u,%u,%u\n", READ_COUNT(&outer), READ_CHILD(&outer), READ_NEXT(&outer), READ_COUNT(&packed), READ_SIGNAL(&packed), READ_SIGNAL(&observable), READ_COUNT(&value), limitations.array[1]);
    return 0;
}
"#,
            &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            true,
        );
        // Windows C stdio translates line endings; numeric oracle data must
        // match Rust's output without treating that stream convention as C semantics.
        assert_eq!(actual, expected.replace("\r\n", "\n"));
    }

    /// Check mismatched field storage and incomplete access layouts yield explicit capability rejections.
    #[test]
    fn field_capabilities_reject_storage_disagreements_and_deferred_layouts() {
        let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = MacroScanner::new().unwrap();
        let frontend = inspect(&scanner, &fixture(), &arguments(), None).unwrap();
        let mut bindings = bindings();
        bindings.records.get_mut("Outer").unwrap().fields.get_mut("count").unwrap().ty =
            RustBindingType::Integer { signed: true, bits: 32 };
        let mut requests = FieldRequests::default();
        for name in ["count", "bits", "array"] {
            requests.field(None, Some(name));
        }
        let generated = generate(
            frontend.declarations(),
            &bindings,
            &requests,
            &frontend.profile().target,
            &[],
        )
        .unwrap();
        assert!(
            generated.unsupported["struct Outer::count"].contains("differs from bindgen storage")
        );
        assert!(
            generated.unsupported["struct Limitations::bits"].contains("bitfield storage accessor")
        );
        assert!(!generated.markers.contains_key("bits"));
        assert!(generated.markers.contains_key("array"));
        assert!(!generated.rust.contains("CRecord<crate::Outer>"));
        let ids = FieldIds::new(frontend.declarations()).unwrap();
        assert!(ids.marker("bits").is_ok(), "rejected fields retain their immutable IDs");
        let mut subset = FieldRequests::default();
        subset.field(None, Some("count"));
        let subset =
            generate(frontend.declarations(), &bindings, &subset, &frontend.profile().target, &[])
                .unwrap();
        assert_eq!(generated.markers["count"], subset.markers["count"]);
    }

    /// Prove offsetof support remains available when the leaf cannot safely expose a load capability.
    #[test]
    fn offset_capabilities_are_independent_of_loadable_leaf_storage() {
        let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = MacroScanner::new().unwrap();
        let arguments = arguments();
        let frontend = inspect(&scanner, &fixture(), &arguments, None).unwrap();
        let mut bindings = bindings();
        bindings.records.get_mut("Outer").unwrap().fields.get_mut("count").unwrap().ty =
            RustBindingType::Integer { signed: true, bits: 32 };
        let mut requests = FieldRequests::default();
        for name in ["count", "child", "frozen", "signal", "array", "bits"] {
            requests.offset(None, Some(name));
        }
        let generated = generate(
            frontend.declarations(),
            &bindings,
            &requests,
            &frontend.profile().target,
            &[],
        )
        .unwrap();
        assert_eq!(generated.offsets["struct Outer::child"].as_deref(), Some("struct Child"));
        assert_eq!(generated.offsets["struct Outer::count"], None);
        assert_eq!(generated.offsets["struct VolatilePacked::signal"], None);
        assert_eq!(generated.offsets["struct Limitations::array"], None);
        assert!(generated.offset_unsupported["struct Limitations::bits"].contains("bitfield"));
        assert!(generated.unsupported.is_empty());
        assert!(generated.c_source.is_empty());
        assert!(!generated.rust.contains("unsafe"));
        assert_eq!(generated.rust.matches("NativeRecord for crate::Outer").count(), 1);

        let rust_bindings = bindgen::Builder::default()
            .header(fixture().to_str().unwrap())
            .clang_args(&arguments)
            .allowlist_type("Child|Outer|Packed|Volatile|Value|Limitations|VolatilePacked")
            .derive_default(false)
            .layout_tests(false)
            .generate()
            .unwrap()
            .to_string();
        let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap();
        let rust = format!(
            r#"
#[path = {support:?}] pub mod __pgrx_c_macros;
{rust_bindings}
pub mod __pgrx_c_generated {{ use crate::__pgrx_c_macros as c; {adapters} }}
use __pgrx_c_macros::expression::*;
fn main() {{
    type Frozen = OffsetStep<__pgrx_c_generated::{child}, OffsetStep<__pgrx_c_generated::{frozen}, OffsetEnd>>;
    type Count = OffsetStep<__pgrx_c_generated::{count}, OffsetEnd>;
    type Signal = OffsetStep<__pgrx_c_generated::{signal}, OffsetEnd>;
    type Array = OffsetStep<__pgrx_c_generated::{array}, OffsetEnd>;
    println!("{{}},{{}},{{}},{{}}", offset_of::<CRecord<Outer>, Frozen>().get(), offset_of::<CRecord<Outer>, Count>().get(), offset_of::<CRecord<VolatilePacked>, Signal>().get(), offset_of::<CRecord<Limitations>, Array>().get());
}}
"#,
            adapters = generated.rust,
            child = local_marker(&generated, "child"),
            frozen = local_marker(&generated, "frozen"),
            count = local_marker(&generated, "count"),
            signal = local_marker(&generated, "signal"),
            array = local_marker(&generated, "array"),
        );
        let actual = rust_oracle::run_rust(&rust);
        let expected = oracle::run_c(
            &frontend.profile().compiler.executable,
            &fixture(),
            r#"
#include <stddef.h>
#include <stdio.h>
int main(void) {
    printf("%zu,%zu,%zu,%zu\n", offsetof(struct Outer, child.frozen), offsetof(struct Outer, count), offsetof(struct VolatilePacked, signal), offsetof(struct Limitations, array));
}
"#,
            &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            true,
        );
        // Preserve every offset and separator while normalizing C stdio's
        // Windows text-mode line endings to Rust's LF output.
        assert_eq!(actual, expected.replace("\r\n", "\n"));
    }

    /// Check Rust type checking rejects overflowing offset sums and overlong designator paths.
    #[test]
    fn offset_paths_reject_overflow_and_excess_depth_during_type_checking() {
        let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap();
        for (offset, depth, reason) in [
            ("usize::MAX", 2, "C offsetof path offset exceeds usize"),
            ("0", 65, "C offsetof path exceeds the 64-field bound"),
        ] {
            let path = format!("{}OffsetEnd{}", "OffsetStep<F,".repeat(depth), ">".repeat(depth));
            let source = format!(
                r#"
#![recursion_limit = "512"]
#[path = {support:?}] pub mod __pgrx_c_macros;
use __pgrx_c_macros::expression::*;
struct F;
impl OffsetField<F> for CRecord<()> {{ type Member = Self; const OFFSET: usize = {offset}; }}
const INVALID: u64 = offset_of::<CRecord<()>, {path}>().get();
fn main() {{ let _ = INVALID; }}
"#,
            );
            let diagnostics = rust_oracle::reject_rust(&source);
            assert!(diagnostics.contains(reason), "{diagnostics}");
        }
    }
}
