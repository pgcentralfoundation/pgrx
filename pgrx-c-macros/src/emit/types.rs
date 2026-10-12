//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Reconcile compiler C identities with the storage that bindgen actually emitted.
//!
//! C markers retain arithmetic rank, qualifiers, and nominal identity independently
//! from native Rust storage. This reconciliation indexes aliases, record edges, and
//! enum constructors once per catalog, rejects ambiguous storage correspondences,
//! and renders distinct paths for exported macro expansions and native support
//! items in the defining crate.

use super::{BindingCatalog, SUPPORT, marker};
use crate::{
    ArrayKind, DeclarationCatalog, EnumBinding, FieldBinding, RecordBinding, RustBindingType,
    TargetFacts, TypeCategory, TypeInfo, TypeShapeKind,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Hygienic exported-macro path to contextual C expression capabilities.
pub(super) const EXPRESSION: &str = "$crate::__pgrx_c_macros::expression";

/// Choose runtime helper paths according to whether source is an exported macro or native item.
#[derive(Clone, Copy)]
enum RuntimePath {
    /// Use hygienic $crate paths that survive dependency renaming at macro invocation.
    ExportedMacro,
    /// Use the defining module’s private runtime import for ordinary generated Rust items.
    Native,
}

/// Select helper namespaces without altering binding or semantic identity paths.
impl RuntimePath {
    /// Select the integer semantic runtime namespace for this emission context.
    fn support(self) -> &'static str {
        match self {
            Self::ExportedMacro => SUPPORT,
            Self::Native => "c",
        }
    }

    /// Select contextual value/place capability paths without rewriting binding storage paths.
    fn expression(self) -> &'static str {
        match self {
            Self::ExportedMacro => EXPRESSION,
            Self::Native => "c::expression",
        }
    }
}

/// Paired C semantic marker and actual Rust storage after identity/representation reconciliation.
pub(super) struct LoweredType {
    /// Capability type retaining C rank, qualifiers, or nominal object identity.
    pub marker: String,
    /// Native Rust representation used by verified binding storage and ABI operations.
    pub storage: String,
}

/// Exact named cast spelling paired with its actual binding type and pointee mutability fact.
struct CastStorage {
    /// Readable source typedef path preserved rather than substituted with a primitive name.
    spelling: String,
    /// Binding type checked against the independently resolved C cast target.
    ty: RustBindingType,
    /// Qualification used while reconstructing nested pointer storage.
    is_const: bool,
}

/// Immutable catalog reconciliation reused across expression rendering and native adapter passes.
///
/// Indexes connect nominal C identities to actual binding constructors and member
/// edges; ambiguous mappings stay errors instead of being resolved by equal layout.
pub(super) struct Lowering<'a> {
    /// Compiler-owned C shapes, qualifiers, identities, and layout facts.
    declarations: &'a DeclarationCatalog,
    /// Actual Rust alias, constructor, member, and capability storage facts.
    bindings: &'a BindingCatalog,
    /// Selected platform widths, ranks, and pointer ABI used for scalar marker reconciliation.
    target: &'a TargetFacts,
    /// Helper-path mode appropriate for exported expansions or ordinary native support items.
    runtime: RuntimePath,
    /// Actual record constructors indexed by normalized crate-relative binding path.
    records_by_path: BTreeMap<String, &'a RecordBinding>,
    /// All witnessed Rust storage paths for a compiler record, retaining ambiguity.
    record_paths: BTreeMap<String, BTreeSet<String>>,
    /// Compiler record/field-index edges paired with bindgen’s anonymous storage member names.
    anonymous_edges: BTreeMap<(String, usize), String>,
    /// Compiler named-member edges paired with unambiguous bindgen-renamed fields.
    named_edges: BTreeMap<(String, String), String>,
    /// Actual Rust enum constructors indexed separately from typedef aliases.
    enums_by_path: BTreeMap<String, &'a EnumBinding>,
    /// Witnessed storage constructors for each unqualified compiler enum identity.
    enum_paths: BTreeMap<String, BTreeSet<String>>,
    /// Storage paths shared by multiple C declarations, preventing nominal enum inference.
    ambiguous_enum_paths: BTreeSet<String>,
}

/// Distinguish value evaluation from object metadata without changing compiler facts.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TypeUse {
    /// Value operations must satisfy the selected floating evaluation policy.
    Value,
    /// Size and alignment queries require layout, and never evaluate floating values.
    ObjectQuery,
}

/// Reconcile nominal compiler types with actual binding edges before rendering capabilities.
impl<'a> Lowering<'a> {
    /// Build immutable reconciliation indexes with hygienic paths for exported macro rendering.
    pub fn new(
        declarations: &'a DeclarationCatalog,
        bindings: &'a BindingCatalog,
        target: &'a TargetFacts,
    ) -> Self {
        Self::with_runtime(declarations, bindings, target, RuntimePath::ExportedMacro)
    }

    /// Native items share one private runtime import in `__pgrx_c_generated`.
    /// Binding storage and capability identities retain their defining-crate paths.
    pub fn new_native(
        declarations: &'a DeclarationCatalog,
        bindings: &'a BindingCatalog,
        target: &'a TargetFacts,
    ) -> Self {
        Self::with_runtime(declarations, bindings, target, RuntimePath::Native)
    }

    /// Create path-independent storage indexes and select the helper namespace used during rendering.
    fn with_runtime(
        declarations: &'a DeclarationCatalog,
        bindings: &'a BindingCatalog,
        target: &'a TargetFacts,
        runtime: RuntimePath,
    ) -> Self {
        let records_by_path =
            bindings.records.values().map(|record| (path_key(&record.path), record)).collect();
        let enums_by_path =
            bindings.enums.values().map(|binding| (path_key(&binding.path), binding)).collect();
        let mut lowering = Self {
            declarations,
            bindings,
            target,
            runtime,
            records_by_path,
            record_paths: BTreeMap::new(),
            anonymous_edges: BTreeMap::new(),
            named_edges: BTreeMap::new(),
            enums_by_path,
            enum_paths: BTreeMap::new(),
            ambiguous_enum_paths: BTreeSet::new(),
        };
        lowering.index_records();
        lowering.index_enums();
        lowering
    }

    /// Map compiler enum identities to actual binding constructors and retain shared-path ambiguity.
    fn index_enums(&mut self) {
        let mut owners = BTreeMap::<String, BTreeSet<String>>::new();
        for (name, ty) in &self.declarations.types {
            if ty.category != TypeCategory::Enum {
                continue;
            }
            let storage =
                self.bindings.type_alias(name).map(|alias| alias.target.clone()).or_else(|| {
                    self.bindings
                        .enums
                        .get(name)
                        .map(|binding| RustBindingType::Named { path: binding.path.clone() })
                });
            if let Some(path) =
                storage.as_ref().and_then(|storage| self.storage_enum_path(storage, 0))
            {
                owners.entry(path.clone()).or_default().insert(enum_key(ty));
                self.enum_paths.entry(enum_key(ty)).or_default().insert(path);
            }
        }
        self.ambiguous_enum_paths = owners
            .into_iter()
            .filter_map(|(path, identities)| (identities.len() > 1).then_some(path))
            .collect();
    }

    /// Follow bounded alias chains to an actual Rust enum constructor, never guessing from integer storage.
    fn storage_enum_path(&self, storage: &RustBindingType, depth: usize) -> Option<String> {
        if depth > 64 {
            return None;
        }
        let RustBindingType::Named { path } = storage else {
            return None;
        };
        let key = path_key(path);
        if self.enums_by_path.contains_key(&key) {
            Some(key)
        } else {
            self.bindings
                .types
                .get(&key)
                .and_then(|alias| self.storage_enum_path(&alias.target, depth + 1))
        }
    }

    /// Require Clang’s compatible integer type before admitting enum arithmetic or ABI transport.
    pub fn enum_underlying(&self, ty: &TypeInfo) -> Result<&'a TypeInfo, String> {
        let shape = self
            .declarations
            .type_shapes
            .get(&ty.canonical_spelling)
            .or_else(|| self.declarations.type_shapes.get(&enum_key(ty)));
        let Some(crate::TypeShape {
            kind: TypeShapeKind::Enum { underlying: Some(underlying) },
            ..
        }) = shape
        else {
            return Err("C enum has no compiler-established compatible integer type".into());
        };
        if !matches!(underlying.category, TypeCategory::Integer(_)) {
            return Err("C enum compatible type is not a modeled integer".into());
        }
        Ok(underlying)
    }

    /// Return only a unique nominal enum storage match, rejecting identity or path ambiguity.
    pub fn enum_binding(&self, ty: &TypeInfo) -> Result<Option<&'a EnumBinding>, String> {
        let Some(paths) = self.enum_paths.get(&enum_key(ty)) else {
            return Ok(None);
        };
        if paths.len() != 1 {
            return Err("C enum maps to multiple distinct Rust enum storage types".into());
        }
        let path = paths.first().expect("nonempty indexed enum paths");
        if self.ambiguous_enum_paths.contains(path) {
            return Err("one Rust enum storage type represents multiple C declarations".into());
        }
        Ok(self.enums_by_path.get(path).copied())
    }

    /// Reconcile numeric aliases or checked Rust enum objects with the compiler enum’s layout and identity.
    fn resolve_enum(
        &self,
        ty: &TypeInfo,
        storage: &RustBindingType,
        depth: usize,
    ) -> Result<LoweredType, String> {
        let expression = self.runtime.expression();
        let underlying = self.enum_underlying(ty)?;
        if ty.size.is_none()
            || ty.alignment.is_none()
            || ty.size != underlying.size
            || ty.alignment != underlying.alignment
        {
            return Err(
                "C enum layout differs from its compiler-established compatible integer".into()
            );
        }
        let numeric = self.resolve_unqualified_at(underlying, depth + 1)?;
        let actual = self.storage_type(storage, depth + 1)?;
        if let Some(path) = self.storage_enum_path(storage, 0) {
            if let Some(reason) = self.bindings.enum_unavailable.get(&enum_key(ty)) {
                return Err(reason.clone());
            }
            let binding = self
                .enum_binding(ty)?
                .ok_or("Rust enum storage has no compiler declaration identity")?;
            if path != path_key(&binding.path) {
                return Err("Rust enum storage belongs to a different C declaration".into());
            }
            let repr = binding
                .repr
                .as_ref()
                .ok_or("Rust enum storage has no explicit integer representation")?;
            if normalize(&self.storage_type(repr, depth + 1)?) != normalize(&numeric.storage) {
                return Err(
                    "Rust enum integer representation differs from Clang's compatible type".into(),
                );
            }
        } else if normalize(&actual) != normalize(&numeric.storage) {
            return Err(
                "Rust enum object storage differs from Clang's compatible integer type".into()
            );
        }
        let identity = super::enumerations::identity_path(ty);
        let marker = format!("{expression}::CEnumObject<{identity}, {}, {actual}>", numeric.marker);
        Ok(LoweredType {
            marker: if ty.is_volatile {
                format!("{expression}::CVolatile<{marker}>")
            } else {
                marker
            },
            storage: actual,
        })
    }

    /// Record identities are anchored by compiler names/typedefs or a field of
    /// an already matched parent. Structurally similar standalone records never
    /// establish interchangeable C identities.
    fn index_records(&mut self) {
        let mut pending = VecDeque::new();
        let mut candidate_cache = BTreeMap::new();
        for (canonical, record) in &self.declarations.records {
            if let Some(binding) =
                record.name.as_ref().and_then(|name| self.bindings.records.get(name))
            {
                pending.push_back((canonical.clone(), path_key(&binding.path)));
            }
        }
        for (name, ty) in &self.declarations.types {
            if ty.category != TypeCategory::Record {
                continue;
            }
            let storage =
                self.bindings.type_alias(name).map(|alias| alias.target.clone()).or_else(|| {
                    self.bindings
                        .records
                        .get(name)
                        .map(|record| RustBindingType::Named { path: record.path.clone() })
                });
            if let Some(path) =
                storage.as_ref().and_then(|storage| self.storage_record_path(storage, 0))
            {
                pending.push_back((ty.canonical_spelling.clone(), path));
            }
        }
        while let Some((canonical, path)) = pending.pop_front() {
            if !self.record_paths.entry(canonical.clone()).or_default().insert(path.clone()) {
                continue;
            }
            let Some(record) = self.declarations.records.get(&canonical) else { continue };
            let Some(binding) = self.records_by_path.get(&path).copied() else { continue };
            if record.kind != binding.kind {
                continue;
            }
            // Each candidate is indexed once. Anonymous edges only compare the
            // anchored parent's unmatched children; recursive candidate facts
            // are memoized instead of rescanning aliases or descendant trees.
            let named = matched_named_fields(record, binding);
            let actual_names = named
                .values()
                .map(|field| field.rust_name.trim_start_matches("r#"))
                .collect::<BTreeSet<_>>();
            for (name, actual) in &named {
                self.named_edges.insert(
                    (canonical.clone(), name.clone()),
                    actual.rust_name.trim_start_matches("r#").to_owned(),
                );
            }
            let children = binding
                .fields
                .iter()
                .filter(|(name, _)| !actual_names.contains(name.as_str()))
                .filter_map(|(name, field)| {
                    Some((name.clone(), self.storage_record_path(&field.ty, 0)?))
                })
                .collect::<Vec<_>>();
            for (index, field) in record.fields.iter().enumerate() {
                let actual = if let Some(name) = &field.name {
                    named.get(name).copied()
                } else {
                    let Some(child) = self.declarations.records.get(&field.ty.canonical_spelling)
                    else {
                        continue;
                    };
                    let candidates = children
                        .iter()
                        .filter(|(_, path)| {
                            self.anonymous_candidate(child, path, 0, &mut candidate_cache)
                        })
                        .map(|(name, _)| name)
                        .collect::<Vec<_>>();
                    if candidates.len() != 1 {
                        continue;
                    }
                    let name = candidates[0];
                    self.anonymous_edges.insert((canonical.clone(), index), name.clone());
                    binding.fields.get(name)
                };
                if let Some(actual) = actual {
                    self.match_record_edges(&field.ty, &actual.ty, &mut pending, 0);
                }
            }
        }
    }

    /// Prove anonymous record shape matching through uniquely corresponding binding edges.
    ///
    /// A recursive by-value cycle or multiple possible child constructors cannot
    /// establish an anchored storage match.
    fn anonymous_candidate(
        &self,
        record: &crate::RecordInfo,
        path: &str,
        depth: usize,
        cache: &mut BTreeMap<(String, String), bool>,
    ) -> bool {
        if depth > 64 {
            return false;
        }
        let key = (record.identity.clone(), path.to_owned());
        if let Some(proven) = cache.get(&key) {
            return *proven;
        }
        // A recursive by-value cycle is not an established C object layout.
        cache.insert(key.clone(), false);
        let Some(binding) = self.records_by_path.get(path) else { return false };
        if binding.kind != record.kind
            || record.fields.iter().any(|field| field.bit_width.is_some())
        {
            return false;
        }
        let names = matched_named_fields(record, binding);
        if names.len() != record.fields.iter().filter(|field| field.name.is_some()).count() {
            return false;
        }
        let actual_names = names
            .values()
            .map(|field| field.rust_name.trim_start_matches("r#"))
            .collect::<BTreeSet<_>>();
        let extras = binding
            .fields
            .iter()
            .filter(|(name, _)| !actual_names.contains(name.as_str()))
            .collect::<Vec<_>>();
        let anonymous =
            record.fields.iter().filter(|field| field.name.is_none()).collect::<Vec<_>>();
        if extras.len() != anonymous.len() {
            return false;
        }
        let matched = anonymous.iter().all(|field| {
            let Some(child) = self.declarations.records.get(&field.ty.canonical_spelling) else {
                return false;
            };
            extras
                .iter()
                .filter(|(_, actual)| {
                    self.storage_record_path(&actual.ty, 0).is_some_and(|path| {
                        self.anonymous_candidate(child, &path, depth + 1, cache)
                    })
                })
                .count()
                == 1
        });
        cache.insert(key, matched);
        matched
    }

    /// Follow matched aliases, pointers, arrays, and record objects to discover nominal storage witnesses.
    fn match_record_edges(
        &self,
        ty: &TypeInfo,
        actual: &RustBindingType,
        pending: &mut VecDeque<(String, String)>,
        depth: usize,
    ) {
        if depth > 64 {
            return;
        }
        if let RustBindingType::ManuallyDrop { value } = actual {
            self.match_record_edges(ty, value, pending, depth + 1);
            return;
        }
        if let RustBindingType::Named { path } = actual
            && let Some(alias) = self.bindings.types.get(&path_key(path))
        {
            self.match_record_edges(ty, &alias.target, pending, depth + 1);
            return;
        }
        match (
            self.declarations.type_shapes.get(&ty.canonical_spelling).map(|shape| &shape.kind),
            actual,
        ) {
            (Some(TypeShapeKind::Record { .. }), _) => {
                if let Some(path) = self.storage_record_path(actual, depth + 1) {
                    pending.push_back((ty.canonical_spelling.clone(), path));
                }
            }
            (
                Some(TypeShapeKind::Pointer { pointee }),
                RustBindingType::Pointer { pointee: actual, .. },
            ) => self.match_record_edges(pointee, actual, pending, depth + 1),
            (
                Some(TypeShapeKind::Array { element, .. }),
                RustBindingType::Array { element: actual, .. }
                | RustBindingType::IncompleteArrayField { element: actual, .. },
            ) => self.match_record_edges(element, actual, pending, depth + 1),
            _ => {}
        }
    }

    /// Follow actual alias or ManuallyDrop storage to a known record constructor within the depth bound.
    fn storage_record_path(&self, storage: &RustBindingType, depth: usize) -> Option<String> {
        if depth > 64 {
            return None;
        }
        match storage {
            RustBindingType::ManuallyDrop { value } => self.storage_record_path(value, depth + 1),
            RustBindingType::Named { path } => {
                let key = path_key(path);
                if let Some(alias) = self.bindings.types.get(&key) {
                    self.storage_record_path(&alias.target, depth + 1)
                } else {
                    self.records_by_path.contains_key(&key).then_some(key)
                }
            }
            _ => None,
        }
    }

    /// Require exactly one compiler-anchored Rust constructor for a record identity.
    pub fn record_binding(&self, ty: &TypeInfo) -> Result<&'a RecordBinding, String> {
        let paths = self
            .record_paths
            .get(&ty.canonical_spelling)
            .ok_or("record has no compiler-anchored public Rust binding")?;
        if paths.len() != 1 {
            return Err("compiler record identity has ambiguous actual Rust storage paths".into());
        }
        self.records_by_path
            .get(paths.first().expect("one record path"))
            .copied()
            .ok_or("record path is absent from actual bindings".into())
    }

    /// Recover a uniquely anchored bindgen storage member for an anonymous C record edge.
    pub fn anonymous_field_binding(
        &self,
        canonical: &str,
        index: usize,
    ) -> Option<&'a FieldBinding> {
        let name = self.anonymous_edges.get(&(canonical.to_owned(), index))?;
        let paths = self.record_paths.get(canonical)?;
        if paths.len() != 1 {
            return None;
        }
        self.records_by_path.get(paths.first()?)?.fields.get(name)
    }

    /// Recover the unambiguous actual Rust field corresponding to a compiler-owned member name.
    pub fn named_field_binding(&self, canonical: &str, name: &str) -> Option<&'a FieldBinding> {
        let actual = self.named_edges.get(&(canonical.to_owned(), name.to_owned()))?;
        let paths = self.record_paths.get(canonical)?;
        if paths.len() != 1 {
            return None;
        }
        self.records_by_path.get(paths.first()?)?.fields.get(actual)
    }

    /// Lower compiler type facts into a marker/storage pair without caller-supplied representation guesses.
    pub fn resolve(&self, ty: &TypeInfo) -> Result<LoweredType, String> {
        self.resolve_at(ty, 0)
    }

    /// Lower an unevaluated size/alignment operand without requiring floating arithmetic support.
    /// Storage widths and recursive object shapes remain checked against the original C facts.
    pub fn resolve_object_query(&self, ty: &TypeInfo) -> Result<LoweredType, String> {
        self.resolve_for_use(ty, 0, TypeUse::ObjectQuery)
    }

    /// Use the exact source typedef, never an alias guessed from its representation.
    /// The marker still carries C rank and qualifications after Rust erases aliases.
    pub fn cast_alias(
        &self,
        name: &str,
        ty: &TypeInfo,
    ) -> Option<Result<(String, LoweredType), String>> {
        let storage = self.alias_storage(name, 0)?;
        Some(storage.and_then(|storage| {
            let lowered = self.resolve_with_storage(ty, &storage.ty)?;
            Ok((storage.spelling, lowered))
        }))
    }

    // Follow the same flat pointer/qualifier grammar admitted by resolve_type_info.
    // Every named leaf must exist in the compiler catalog and the actual bindings.
    /// Retain exact source typedef paths while following the admitted pointer/qualifier grammar.
    ///
    /// Every named leaf must exist in both compiler declarations and the current
    /// bindings; unavailable aliases remain absent or produce a reconciliation error.
    fn alias_storage(&self, name: &str, depth: usize) -> Option<Result<CastStorage, String>> {
        if depth > 64 {
            return Some(Err("named cast storage exceeds the bounded lowering depth".into()));
        }
        if let Some(ty) = self.declarations.types.get(name) {
            return Some((|| {
                let path = if let Some(alias) = self.bindings.type_alias(name) {
                    alias.path.clone()
                } else if self.bindings.integer_storage.contains_key(name) {
                    // The binding generator may replace this typedef with an
                    // imported newtype; its verified adapter owns that path.
                    name.split("::").map(str::to_owned).collect()
                } else if let Some(binding) =
                    self.bindings.records.get(name).filter(|_| ty.category == TypeCategory::Record)
                {
                    binding.path.clone()
                } else if let Some(binding) =
                    self.bindings.enums.get(name).filter(|_| ty.category == TypeCategory::Enum)
                {
                    binding.path.clone()
                } else if name.starts_with("struct ") || name.starts_with("union ") {
                    self.record_binding(ty)?.path.clone()
                } else if name.starts_with("enum ") {
                    self.enum_binding(ty)?
                        .ok_or("the C enum tag has no corresponding named Rust binding")?
                        .path
                        .clone()
                } else {
                    return Err("the C type has no corresponding named Rust binding".into());
                };
                Ok(CastStorage {
                    spelling: rust_path(&path)?,
                    ty: RustBindingType::Named { path },
                    is_const: ty.is_const,
                })
            })());
        }
        if let Some((pointee, qualifiers)) = name.rsplit_once('*') {
            if !qualifiers.split_whitespace().all(|word| matches!(word, "const" | "volatile")) {
                return None;
            }
            let pointee = pointee.trim();
            let storage = self.alias_storage(pointee, depth + 1)?;
            return Some(storage.map(|storage| {
                let mutable = !storage.is_const;
                CastStorage {
                    spelling: format!(
                        "*{} {}",
                        if mutable { "mut" } else { "const" },
                        storage.spelling
                    ),
                    ty: RustBindingType::Pointer { pointee: Box::new(storage.ty), mutable },
                    is_const: qualifiers.split_whitespace().any(|word| word == "const"),
                }
            }));
        }
        let words = name.split_whitespace().collect::<Vec<_>>();
        let unqualified = words
            .iter()
            .copied()
            .filter(|word| !matches!(*word, "const" | "volatile"))
            .collect::<Vec<_>>()
            .join(" ");
        if words.iter().any(|word| matches!(*word, "const" | "volatile")) {
            self.alias_storage(&unqualified, depth + 1).map(|storage| {
                storage.map(|mut storage| {
                    storage.is_const |= words.contains(&"const");
                    storage
                })
            })
        } else {
            None
        }
    }

    /// Macro casts can retain a typedef spelling absent from the canonical
    /// graph. Resolve that spelling through the same compiler declaration facts
    /// used by lowering, so support discovery follows the identical edge.
    pub fn pointer_pointee(&self, ty: &TypeInfo) -> Result<TypeInfo, String> {
        if let Some(crate::TypeShape { kind: TypeShapeKind::Pointer { pointee }, .. }) =
            self.declarations.type_shapes.get(&ty.canonical_spelling)
        {
            return Ok(pointee.clone());
        }
        let (name, _) = ty
            .canonical_spelling
            .rsplit_once('*')
            .ok_or("C pointer has no compiler-owned pointee shape")?;
        crate::analysis::resolve_type_info(name.trim(), self.declarations, self.target)
            .ok_or("C pointer pointee is not established".into())
    }

    /// Apply effective volatile qualification around the bounded unqualified type lowering.
    fn resolve_at(&self, ty: &TypeInfo, depth: usize) -> Result<LoweredType, String> {
        self.resolve_for_use(ty, depth, TypeUse::Value)
    }

    /// Preserve qualification while carrying the operation's evaluation contract through nested types.
    fn resolve_for_use(
        &self,
        ty: &TypeInfo,
        depth: usize,
        usage: TypeUse,
    ) -> Result<LoweredType, String> {
        let expression = self.runtime.expression();
        if depth > 64 {
            return Err("C type exceeds the bounded lowering depth".into());
        }
        let mut lowered = self.resolve_unqualified_for_use(ty, depth, usage)?;
        if ty.is_volatile {
            lowered.marker = format!("{expression}::CVolatile<{}>", lowered.marker);
        }
        Ok(lowered)
    }

    /// Choose semantic markers and native storage for compiler-proved scalar and structural types.
    fn resolve_unqualified_at(&self, ty: &TypeInfo, depth: usize) -> Result<LoweredType, String> {
        self.resolve_unqualified_for_use(ty, depth, TypeUse::Value)
    }

    /// Reconcile storage recursively, retaining stricter policy checks only where values are evaluated.
    fn resolve_unqualified_for_use(
        &self,
        ty: &TypeInfo,
        depth: usize,
        usage: TypeUse,
    ) -> Result<LoweredType, String> {
        let expression = self.runtime.expression();
        let support = self.runtime.support();
        if let Some(callback) = self.bindings.callback_capabilities.get(&ty.canonical_spelling) {
            return Ok(LoweredType {
                marker: format!("{expression}::CFunction<{}>", callback.marker),
                storage: self.storage_type(&callback.storage, 0)?,
            });
        }
        if let Some(reason) = self.bindings.callback_unavailable.get(&ty.canonical_spelling) {
            return Err(reason.clone());
        }
        let shape = self.declarations.type_shapes.get(&ty.canonical_spelling);
        if let Some(crate::TypeShape {
            kind:
                TypeShapeKind::Array { element, length: Some(length), array_kind: ArrayKind::Constant },
            ..
        }) = shape
        {
            let element = self.resolve_for_use(element, depth + 1, usage)?;
            return Ok(LoweredType {
                marker: format!("{expression}::CArray<{}, {length}>", element.marker),
                storage: format!("[{}; {length}]", element.storage),
            });
        }
        match ty.category {
            TypeCategory::Integer(kind) => {
                let integer = self
                    .target
                    .integers
                    .get(&kind)
                    .ok_or("C integer identity is absent from the target profile")?;
                Ok(LoweredType {
                    marker: format!("{support}::{}", marker(kind)),
                    storage: if kind == crate::IntegerKind::Bool {
                        "bool".into()
                    } else {
                        format!("{}{}", if integer.signed { 'i' } else { 'u' }, integer.bits)
                    },
                })
            }
            TypeCategory::Void => Ok(LoweredType {
                marker: format!("{expression}::CVoid"),
                storage: "::core::ffi::c_void".into(),
            }),
            TypeCategory::Pointer => {
                let pointee = self.pointer_pointee(ty)?;
                let lowered = self.resolve_for_use(&pointee, depth + 1, usage)?;
                Ok(LoweredType {
                    marker: format!(
                        "{expression}::CPointer<{}, {expression}::{}>",
                        lowered.marker,
                        if pointee.is_const { "ReadOnly" } else { "ReadWrite" }
                    ),
                    storage: format!(
                        "*{} {}",
                        if pointee.is_const { "const" } else { "mut" },
                        lowered.storage
                    ),
                })
            }
            TypeCategory::Record => {
                let record = self
                    .declarations
                    .records
                    .get(&ty.canonical_spelling)
                    .ok_or("C record has no compiler-owned layout")?;
                let binding = self.record_binding(ty)?;
                if record.kind != binding.kind {
                    return Err("C and Rust record kinds differ".into());
                }
                let storage = rust_path(&binding.path)?;
                let marker = if record.size.is_some() && record.alignment.is_some() {
                    "CRecord"
                } else {
                    "COpaque"
                };
                Ok(LoweredType { marker: format!("{expression}::{marker}<{storage}>"), storage })
            }
            TypeCategory::Enum => {
                let storage = if let Some(binding) = self.enum_binding(ty)? {
                    RustBindingType::Named { path: binding.path.clone() }
                } else {
                    let underlying = self.enum_underlying(ty)?;
                    let TypeCategory::Integer(kind) = underlying.category else {
                        unreachable!("enum_underlying checks integer category")
                    };
                    let integer = self
                        .target
                        .integers
                        .get(&kind)
                        .ok_or("enum integer absent from target profile")?;
                    RustBindingType::Integer { signed: integer.signed, bits: integer.bits }
                };
                let mut unqualified = ty.clone();
                unqualified.is_volatile = false;
                self.resolve_enum(&unqualified, &storage, depth)
            }
            TypeCategory::Floating => {
                if usage == TypeUse::Value {
                    self.target.supports_float_values()?;
                }
                let (name, storage) = match ty.canonical_spelling.as_str() {
                    "float" => ("CFloat", "f32"),
                    "double" => ("CDouble", "f64"),
                    _ => {
                        return Err(
                            "extended floating type has no matching Rust representation".into()
                        );
                    }
                };
                let bytes = if storage == "f32" { 4 } else { 8 };
                if ty.size != Some(bytes) {
                    return Err("floating C object width differs from Rust storage".into());
                }
                Ok(LoweredType { marker: format!("{expression}::{name}"), storage: storage.into() })
            }
            TypeCategory::Function => {
                Err("function pointer values require a generated ABI adapter".into())
            }
            TypeCategory::Other => Err(format!("{}: unmodeled C storage type", ty.spelling)),
        }
    }

    /// Reconcile a compiler type against the actual binding representation at a specific storage edge.
    pub fn resolve_with_storage(
        &self,
        ty: &TypeInfo,
        storage: &RustBindingType,
    ) -> Result<LoweredType, String> {
        self.resolve_with_storage_at(ty, storage, 0)
    }

    /// Validate nested C/Rust representation agreement without erasing nominal identity or qualifiers.
    ///
    /// Raw records, checked enums, callbacks, and flexible arrays each retain their
    /// representation-specific capabilities rather than relying on equal object size.
    fn resolve_with_storage_at(
        &self,
        ty: &TypeInfo,
        storage: &RustBindingType,
        depth: usize,
    ) -> Result<LoweredType, String> {
        let expression = self.runtime.expression();
        let support = self.runtime.support();
        if depth > 64 {
            return Err("C/Rust storage reconciliation exceeds its bounded depth".into());
        }
        if let Some(callback) = self.bindings.callback_capabilities.get(&ty.canonical_spelling) {
            let expected = self.storage_type(&callback.storage, 0)?;
            let actual = self.storage_type(storage, 0)?;
            if normalize(&expected) != normalize(&actual) {
                return Err(
                    "callback binding ABI differs from the compiler-checked signature adapter"
                        .into(),
                );
            }
            return self.resolve_at(ty, depth);
        }
        if let RustBindingType::MaybeUninit { value } = storage {
            if ty.category != TypeCategory::Record {
                return Err("raw record ABI storage requires a compiler record type".into());
            }
            let record = self
                .declarations
                .records
                .get(&ty.canonical_spelling)
                .ok_or("raw record ABI has no compiler layout")?;
            if record.size.is_none() || record.alignment.is_none() {
                return Err("incomplete record cannot cross a by-value C ABI".into());
            }
            let lowered = self.resolve_with_storage_at(ty, value, depth + 1)?;
            let marker = format!("{expression}::CRawRecord<{}>", lowered.storage);
            return Ok(LoweredType {
                marker: if ty.is_volatile {
                    format!("{expression}::CVolatile<{marker}>")
                } else {
                    marker
                },
                storage: format!("::core::mem::MaybeUninit<{}>", lowered.storage),
            });
        }
        if let RustBindingType::ManuallyDrop { value } = storage {
            return self.resolve_with_storage_at(ty, value, depth + 1);
        }
        if ty.category == TypeCategory::Enum {
            return self.resolve_enum(ty, storage, depth);
        }
        if let RustBindingType::IncompleteArrayField { path, element: actual } = storage {
            let Some(crate::TypeShape {
                kind:
                    TypeShapeKind::Array { element, length: None, array_kind: ArrayKind::Incomplete },
                ..
            }) = self.declarations.type_shapes.get(&ty.canonical_spelling)
            else {
                return Err(
                    "flexible-array storage does not match a compiler incomplete array".into()
                );
            };
            let element = self.resolve_with_storage_at(element, actual, depth + 1)?;
            let storage = format!("{}<{}>", rust_path(path)?, element.storage);
            let marker = format!("{expression}::CFlexibleArray<{}, {storage}>", element.marker);
            return Ok(LoweredType {
                marker: if ty.is_volatile {
                    format!("{expression}::CVolatile<{marker}>")
                } else {
                    marker
                },
                storage,
            });
        }
        if let RustBindingType::Named { path } = storage {
            let key = path
                .iter()
                .map(|part| part.strip_prefix("r#").unwrap_or(part))
                .collect::<Vec<_>>()
                .join("::");
            if let Some(alias) = self.bindings.types.get(&key) {
                return self.resolve_with_storage_at(ty, &alias.target, depth + 1);
            }
            if let TypeCategory::Integer(kind) = ty.category
                && self.bindings.integer_kind(path) == Some(kind)
            {
                let storage = rust_path(path)?;
                let marker = format!(
                    "{expression}::CIntegerStorage<{support}::{}, {storage}>",
                    marker(kind)
                );
                return Ok(LoweredType {
                    marker: if ty.is_volatile {
                        format!("{expression}::CVolatile<{marker}>")
                    } else {
                        marker
                    },
                    storage,
                });
            }
        }
        if let RustBindingType::Array { element: actual, length: actual_length } = storage
            && let Some(crate::TypeShape {
                kind:
                    TypeShapeKind::Array {
                        element,
                        length: Some(length),
                        array_kind: ArrayKind::Constant,
                    },
                ..
            }) = self.declarations.type_shapes.get(&ty.canonical_spelling)
        {
            if length != actual_length {
                return Err("bindgen array bound differs from the compiler declaration".into());
            }
            let element = self.resolve_with_storage_at(element, actual, depth + 1)?;
            let marker = format!("{expression}::CArray<{}, {length}>", element.marker);
            return Ok(LoweredType {
                marker: if ty.is_volatile {
                    format!("{expression}::CVolatile<{marker}>")
                } else {
                    marker
                },
                storage: format!("[{}; {length}]", element.storage),
            });
        }
        if storage == &RustBindingType::NativeChar {
            if ty.category != TypeCategory::Integer(crate::IntegerKind::Char)
                || ty.size != Some(1)
                || ty.alignment != Some(1)
                || self.target.char_bits != 8
            {
                return Err("Rust c_char storage requires a verified C plain-char byte".into());
            }
            let storage = "::core::ffi::c_char".to_owned();
            let marker = format!("{expression}::CIntegerStorage<{support}::CChar, {storage}>");
            return Ok(LoweredType {
                marker: if ty.is_volatile {
                    format!("{expression}::CVolatile<{marker}>")
                } else {
                    marker
                },
                storage,
            });
        }
        if let (
            TypeCategory::Integer(crate::IntegerKind::Char),
            RustBindingType::Integer { signed, bits: 8 },
        ) = (ty.category, storage)
            && ty.size == Some(1)
            && ty.alignment == Some(1)
            && let Some(integer) = self.target.integers.get(&crate::IntegerKind::Char)
            && integer.bits == 8
            && integer.signed != *signed
        {
            // Both Rust byte integers admit all 256 representations. Preserve
            // the binding's storage bytes while the verified plain-char marker
            // supplies the C sign and promotion rules selected by build flags.
            let storage = if *signed { "i8" } else { "u8" }.to_owned();
            let marker = format!("{expression}::CIntegerStorage<{support}::CChar, {storage}>");
            return Ok(LoweredType {
                marker: if ty.is_volatile {
                    format!("{expression}::CVolatile<{marker}>")
                } else {
                    marker
                },
                storage,
            });
        }
        if let (
            TypeCategory::Integer(kind),
            RustBindingType::PointerSizedInteger { signed, bits },
        ) = (ty.category, storage)
        {
            let integer =
                self.target.integers.get(&kind).ok_or("C integer absent from target profile")?;
            if integer.signed != *signed || integer.bits != *bits {
                return Err(
                    "pointer-sized Rust integer differs from the C object representation".into()
                );
            }
            let storage = if *signed { "isize" } else { "usize" }.to_owned();
            let marker =
                format!("{expression}::CIntegerStorage<{support}::{}, {storage}>", marker(kind));
            return Ok(LoweredType {
                marker: if ty.is_volatile {
                    format!("{expression}::CVolatile<{marker}>")
                } else {
                    marker
                },
                storage,
            });
        }
        if let (TypeCategory::Pointer, RustBindingType::Pointer { pointee: actual, mutable }) =
            (ty.category, storage)
        {
            let pointee = self.pointer_pointee(ty)?;
            if pointee.is_const == *mutable {
                return Err("bindgen pointer qualification differs from the C declaration".into());
            }
            let pointee_lowered = self.resolve_with_storage_at(&pointee, actual, depth + 1)?;
            let marker = format!(
                "{expression}::CPointer<{}, {expression}::{}>",
                pointee_lowered.marker,
                if *mutable { "ReadWrite" } else { "ReadOnly" }
            );
            return Ok(LoweredType {
                marker: if ty.is_volatile {
                    format!("{expression}::CVolatile<{marker}>")
                } else {
                    marker
                },
                storage: format!(
                    "*{} {}",
                    if *mutable { "mut" } else { "const" },
                    pointee_lowered.storage
                ),
            });
        }
        let lowered = self.resolve(ty)?;
        let actual = self.storage_type(storage, 0)?;
        if normalize(&actual) != normalize(&lowered.storage) {
            return Err(format!(
                "{}: C storage {} differs from bindgen storage {actual}",
                ty.spelling, lowered.storage
            ));
        }
        Ok(lowered)
    }

    /// Render actual Rust binding storage through bounded aliases and verified native ABI constructors.
    pub fn storage_type(&self, storage: &RustBindingType, depth: usize) -> Result<String, String> {
        if depth > 64 {
            return Err("Rust alias chain exceeds the bounded lowering depth".into());
        }
        Ok(match storage {
            RustBindingType::Unit => "()".into(),
            RustBindingType::Bool => "bool".into(),
            RustBindingType::NativeChar => "::core::ffi::c_char".into(),
            RustBindingType::Integer { signed, bits } => {
                format!("{}{bits}", if *signed { 'i' } else { 'u' })
            }
            RustBindingType::PointerSizedInteger { signed, .. } => {
                if *signed { "isize" } else { "usize" }.into()
            }
            RustBindingType::Float { bits } => format!("f{bits}"),
            RustBindingType::Pointer { pointee, mutable } => format!(
                "*{} {}",
                if *mutable { "mut" } else { "const" },
                self.storage_type(pointee, depth + 1)?
            ),
            RustBindingType::Array { element, length } => {
                format!("[{}; {length}]", self.storage_type(element, depth + 1)?)
            }
            RustBindingType::Named { path } => {
                let key = path
                    .iter()
                    .map(|part| part.strip_prefix("r#").unwrap_or(part))
                    .collect::<Vec<_>>()
                    .join("::");
                if let Some(alias) = self.bindings.types.get(&key) {
                    self.storage_type(&alias.target, depth + 1)?
                } else if path.len() >= 2
                    && matches!(path[0].as_str(), "core" | "std" | "libc")
                    && path.last().is_some_and(|part| part == "c_void")
                {
                    "::core::ffi::c_void".into()
                } else {
                    rust_path(path)?
                }
            }
            RustBindingType::Function { parameters, result, abi, unsafe_, variadic } => {
                if *variadic || !matches!(abi.as_str(), "C" | "C-unwind") {
                    return Err("callback storage requires a fixed native C ABI".into());
                }
                let parameters = parameters
                    .iter()
                    .map(|parameter| self.storage_type(parameter, depth + 1))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ");
                let result = self.storage_type(result, depth + 1)?;
                format!(
                    "{}extern {abi:?} fn({parameters}) -> {result}",
                    if *unsafe_ { "unsafe " } else { "" }
                )
            }
            RustBindingType::Option { value } => {
                format!("::core::option::Option<{}>", self.storage_type(value, depth + 1)?)
            }
            RustBindingType::ManuallyDrop { value } => {
                format!("::core::mem::ManuallyDrop<{}>", self.storage_type(value, depth + 1)?)
            }
            RustBindingType::MaybeUninit { value } => {
                format!("::core::mem::MaybeUninit<{}>", self.storage_type(value, depth + 1)?)
            }
            RustBindingType::IncompleteArrayField { path, element } => {
                format!("{}<{}>", rust_path(path)?, self.storage_type(element, depth + 1)?)
            }
        })
    }
}

/// Bindgen can append an underscore to Rust keywords and primitive names.
/// The compiler's field set owns the source names; an alternate spelling is
/// accepted only when it cannot denote a different field in the same record.
fn matched_named_fields<'a>(
    record: &crate::RecordInfo,
    binding: &'a RecordBinding,
) -> BTreeMap<String, &'a FieldBinding> {
    let names =
        record.fields.iter().filter_map(|field| field.name.as_deref()).collect::<BTreeSet<_>>();
    let mut matched = BTreeMap::new();
    for name in &names {
        if let Some(base) = name.strip_suffix('_')
            && names.contains(base)
            && !binding.fields.contains_key(base)
        {
            continue;
        }
        let actual = binding.fields.get(*name).or_else(|| {
            let alternate = format!("{name}_");
            (!names.contains(alternate.as_str())).then(|| binding.fields.get(&alternate)).flatten()
        });
        if let Some(actual) = actual {
            matched.insert((*name).to_owned(), actual);
        }
    }
    matched
}

/// Normalize raw identifier prefixes for catalog lookup while retaining the complete binding path.
fn path_key(path: &[String]) -> String {
    path.iter().map(|part| part.strip_prefix("r#").unwrap_or(part)).collect::<Vec<_>>().join("::")
}

/// Remove only top-level qualifier prefixes when identifying the nominal C enum declaration.
pub(super) fn enum_key(ty: &TypeInfo) -> String {
    let mut spelling = ty.canonical_spelling.as_str();
    while let Some(rest) = ["const ", "volatile ", "restrict "]
        .iter()
        .find_map(|qualifier| spelling.strip_prefix(qualifier))
    {
        spelling = rest;
    }
    spelling.to_owned()
}

/// Ignore formatting whitespace when comparing already-rendered native storage spellings.
fn normalize(storage: &str) -> String {
    storage.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Validate a binding path and choose absolute standard-library or hygienic defining-crate resolution.
pub(super) fn rust_path(path: &[String]) -> Result<String, String> {
    if path.is_empty()
        || path
            .iter()
            .any(|part| super::rust_identifier(part.strip_prefix("r#").unwrap_or(part)).is_none())
    {
        return Err("binding has no usable public Rust path".into());
    }
    let prefix =
        if matches!(path[0].as_str(), "core" | "std" | "libc") { "::" } else { "$crate::" };
    Ok(format!("{prefix}{}", path.join("::")))
}

/// Regressions for runtime path hygiene and unambiguous C-to-bindgen member-name reconciliation.
#[cfg(test)]
mod tests {
    use super::*;

    /// A genuine x87 evaluation profile still permits unevaluated double layout
    /// queries; arithmetic stays refused and incompatible storage widths stay errors.
    #[test]
    fn object_queries_do_not_require_floating_value_evaluation() {
        let _lock = crate::SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = crate::MacroScanner::new().unwrap();
        let header = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/native_profile.h");
        let frontend =
            crate::inspect(&scanner, &header, &["--target=i686-unknown-openbsd".into()], None)
                .unwrap();
        assert_eq!(frontend.profile().target.floating_point.evaluation_method, Some(2));
        let bindings = BindingCatalog::default();
        let lowering =
            Lowering::new(frontend.declarations(), &bindings, &frontend.profile().target);
        let ty = crate::analysis::resolve_type_info(
            "double",
            frontend.declarations(),
            &frontend.profile().target,
        )
        .unwrap();
        assert!(lowering.resolve(&ty).is_err());
        assert!(lowering.resolve_object_query(&ty).unwrap().marker.ends_with("::CDouble"));
        let incompatible = TypeInfo { size: Some(4), ..ty };
        assert!(lowering.resolve_object_query(&incompatible).is_err());
    }

    /// Admit only byte-compatible plain-char storage when compiler flags change
    /// signedness; ordinary signed-char and wider mismatches remain errors.
    #[test]
    fn plain_char_storage_keeps_binding_bytes_and_inspected_c_identity() {
        let _lock = crate::SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = crate::MacroScanner::new().unwrap();
        let header =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/input_types.h");
        let frontend =
            crate::inspect(&scanner, &header, &["-funsigned-char".into()], None).unwrap();
        let bindings = BindingCatalog::default();
        let lowering =
            Lowering::new(frontend.declarations(), &bindings, &frontend.profile().target);
        let ty = TypeInfo {
            spelling: "char".into(),
            canonical_spelling: "char".into(),
            category: TypeCategory::Integer(crate::IntegerKind::Char),
            size: Some(1),
            alignment: Some(1),
            is_const: false,
            is_volatile: false,
        };
        let actual = RustBindingType::Integer { signed: true, bits: 8 };
        let lowered = lowering.resolve_with_storage(&ty, &actual).unwrap();
        assert_eq!(lowered.storage, "i8");
        assert!(lowered.marker.ends_with("::CIntegerStorage<$crate::__pgrx_c_macros::CChar, i8>"));
        let symbolic = lowering.resolve_with_storage(&ty, &RustBindingType::NativeChar).unwrap();
        assert_eq!(symbolic.storage, "::core::ffi::c_char");
        assert!(symbolic.marker.ends_with("::CChar, ::core::ffi::c_char>"));
        assert!(
            lowering
                .resolve_with_storage(&ty, &RustBindingType::Integer { signed: true, bits: 16 })
                .is_err()
        );
        let signed_char =
            TypeInfo { category: TypeCategory::Integer(crate::IntegerKind::SignedChar), ..ty };
        assert!(
            lowering
                .resolve_with_storage(
                    &signed_char,
                    &RustBindingType::Integer { signed: false, bits: 8 }
                )
                .is_err()
        );
    }

    /// Check native helper imports remain local while exported macros and binding storage retain correct crate paths.
    #[test]
    fn native_runtime_paths_do_not_escape_into_macro_or_storage_paths() {
        let kind = crate::IntegerKind::UnsignedInt;
        let target = TargetFacts {
            triple: "fixture".into(),
            pointer_bits: 64,
            function_pointer: crate::PointerLayout { size: 8, alignment: 8 },
            size_type: kind,
            ptrdiff_type: crate::IntegerKind::Long,
            preferred_alignments: Default::default(),
            arm_float_abi: None,
            ppc64_elf_abi: None,
            offsetof_supported: true,
            char_bits: 8,
            char_is_signed: true,
            ascii_execution_charset: true,
            byte_order: crate::ByteOrder::Little,
            c_standard: Some(201710),
            integers: [(kind, crate::IntegerType { kind, bits: 32, signed: false, rank: 3 })]
                .into(),
            floating_point: Default::default(),
        };
        let integer = TypeInfo {
            spelling: "unsigned int".into(),
            canonical_spelling: "unsigned int".into(),
            category: TypeCategory::Integer(kind),
            size: Some(4),
            alignment: Some(4),
            is_const: false,
            is_volatile: false,
        };
        let mut declarations = DeclarationCatalog::default();
        declarations.types.insert("CountAlias".into(), integer.clone());
        let mut bindings = BindingCatalog::default();
        bindings.types.insert(
            "CountAlias".into(),
            crate::AliasBinding {
                path: vec!["CountAlias".into()],
                target: RustBindingType::Integer { signed: false, bits: 32 },
            },
        );
        let signature = "$crate::__pgrx_c_generated::Signature_verified";
        bindings.callback_capabilities.insert(
            "callback".into(),
            crate::CallbackBinding {
                marker: signature.into(),
                storage: RustBindingType::Option {
                    value: Box::new(RustBindingType::Function {
                        parameters: vec![RustBindingType::Integer { signed: false, bits: 32 }],
                        result: Box::new(RustBindingType::Unit),
                        abi: "C-unwind".into(),
                        unsafe_: true,
                        variadic: false,
                    }),
                },
            },
        );
        let exported = Lowering::new(&declarations, &bindings, &target);
        let native = Lowering::new_native(&declarations, &bindings, &target);
        let (_, exported_alias) = exported.cast_alias("CountAlias", &integer).unwrap().unwrap();
        let (spelling, native_alias) = native.cast_alias("CountAlias", &integer).unwrap().unwrap();
        assert_eq!(spelling, "$crate::CountAlias");
        assert_eq!(exported_alias.marker, "$crate::__pgrx_c_macros::CUnsignedInt");
        assert_eq!(native_alias.marker, "c::CUnsignedInt");
        assert_eq!(exported_alias.storage, native_alias.storage);
        let callback = TypeInfo {
            spelling: "callback".into(),
            canonical_spelling: "callback".into(),
            category: TypeCategory::Function,
            ..integer.clone()
        };
        for ty in [integer.clone(), TypeInfo { is_volatile: true, ..integer }, callback] {
            let macro_type = exported.resolve(&ty).unwrap();
            let native_type = native.resolve(&ty).unwrap();
            assert!(macro_type.marker.starts_with("$crate::__pgrx_c_macros::"));
            assert!(native_type.marker.starts_with("c::"));
            assert_eq!(macro_type.storage, native_type.storage);
            assert_eq!(
                macro_type.marker.contains(signature),
                native_type.marker.contains(signature),
            );
        }
        // A binding named `c` remains a defining-crate path, not the private import.
        let storage = RustBindingType::Named { path: vec!["c".into()] };
        assert_eq!(exported.storage_type(&storage, 0).unwrap(), "$crate::c");
        assert_eq!(native.storage_type(&storage, 0).unwrap(), "$crate::c");
        assert_eq!(bindings.callback_capabilities["callback"].marker, signature);
    }

    /// Check bindgen underscore renames are accepted only when no distinct C member could own that spelling.
    #[test]
    fn keyword_and_primitive_field_renames_require_unambiguous_compiler_names() {
        let ty = TypeInfo {
            spelling: "int".into(),
            canonical_spelling: "int".into(),
            category: TypeCategory::Integer(crate::IntegerKind::Int),
            size: Some(4),
            alignment: Some(4),
            is_const: false,
            is_volatile: false,
        };
        let record = |names: &[&str]| crate::RecordInfo {
            identity: "compiler record".into(),
            name: Some("Record".into()),
            kind: crate::RecordKind::Struct,
            fields: names
                .iter()
                .enumerate()
                .map(|(index, name)| crate::FieldInfo {
                    name: Some((*name).into()),
                    ty: ty.clone(),
                    offset_bits: Some(index as u64 * 32),
                    bit_width: None,
                    is_anonymous: false,
                })
                .collect(),
            size: Some(names.len() as u64 * 4),
            alignment: Some(4),
            is_anonymous: false,
        };
        let binding = RecordBinding {
            path: vec!["Record".into()],
            kind: crate::RecordKind::Struct,
            fields: ["type_", "u32_", "r#match"]
                .into_iter()
                .map(|name| {
                    (
                        name.trim_start_matches("r#").to_owned(),
                        FieldBinding {
                            rust_name: name.into(),
                            ty: RustBindingType::Integer { signed: true, bits: 32 },
                        },
                    )
                })
                .collect(),
            packed: false,
            copy: true,
        };
        let matched = matched_named_fields(&record(&["type", "u32", "match"]), &binding);
        assert_eq!(matched["type"].rust_name, "type_");
        assert_eq!(matched["u32"].rust_name, "u32_");
        assert_eq!(matched["match"].rust_name, "r#match");
        let collision = matched_named_fields(&record(&["type", "type_"]), &binding);
        assert!(
            collision.is_empty(),
            "a suffix collision must never choose one C field by accident"
        );
    }
}
