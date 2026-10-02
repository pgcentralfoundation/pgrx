//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compiler-established field projection capabilities for the actual Rust bindings.

use super::types::{Lowering, rust_path};
use crate::{
    ArrayKind, BindingCatalog, DeclarationCatalog, FieldBinding, FieldInfo, RecordBinding,
    RustBindingType, TargetFacts, TypeCategory, TypeInfo, TypeShapeKind,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

const EXPRESSION: &str = "crate::__pgrx_c_macros::expression";
const MAX_ADAPTER_BYTES: usize = 16 * 1024 * 1024;

pub(super) struct FieldAdapters {
    /// Items included once in the defining crate's `__pgrx_c_generated` module.
    pub rust: String,
    /// Original-C field access primitives compiled under the inspected profile.
    pub c_source: String,
    /// C field name to the capability marker usable from a macro expansion.
    pub markers: BTreeMap<String, String>,
    /// Compiler record spelling and field name to the rejected capability's reason.
    pub unsupported: BTreeMap<String, String>,
}

/// Field names can coincide with Rust keywords or generated identifiers. Encoding
/// all bytes keeps capability names independent of those language namespaces.
pub(super) fn marker_name(field: &str) -> String {
    let mut marker = String::from("Field_");
    for byte in field.bytes() {
        write!(marker, "{byte:02x}").expect("writing a field marker to String cannot fail");
    }
    marker
}

pub(super) fn generate(
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    used_fields: &BTreeSet<String>,
    target: &TargetFacts,
    required_types: &[TypeInfo],
) -> Result<FieldAdapters, String> {
    let lowering = Lowering::new(declarations, bindings, target);
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
    };
    let mut record_bridges = BTreeSet::new();
    let mut projections = BTreeSet::new();
    let mut pending_records = required_types.to_vec();
    for (canonical, record) in &declarations.records {
        let mut used = Vec::new();
        collect_projections(
            canonical,
            declarations,
            &lowering,
            used_fields,
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
                    output.unsupported.insert(
                        format!(
                            "{canonical}::{}",
                            field.field.name.as_ref().expect("used fields have names")
                        ),
                        reason.clone(),
                    );
                }
                continue;
            }
        };
        let Some(binding) = records_by_path.get(&storage).copied() else { continue };
        for projection in used {
            let field = projection.field;
            let name = field.name.as_ref().expect("used fields have names");
            let key = format!("{canonical}::{name}");
            if !projections.insert((storage.clone(), name.clone())) {
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
                let field_marker = marker_name(name);
                match super::bitfields::generate(
                    canonical,
                    field,
                    binding,
                    &storage,
                    declarations,
                    bindings,
                    &lowering,
                ) {
                    Ok(adapter) => {
                        if output
                            .markers
                            .insert(
                                name.clone(),
                                format!("$crate::__pgrx_c_generated::{field_marker}"),
                            )
                            .is_none()
                        {
                            writeln!(output.rust, "#[doc(hidden)] pub struct {field_marker};")
                                .expect("String output");
                        }
                        if record_bridges.insert(storage.clone()) {
                            record_bridge(
                                &mut output.rust,
                                &storage,
                                binding,
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
            let (mut marker, unaligned, field_alignment, projected_storage, flexible) =
                match validated {
                    Ok(projection) => projection,
                    Err(reason) => {
                        output.unsupported.insert(key, reason);
                        continue;
                    }
                };
            let field_marker = marker_name(name);
            if output
                .markers
                .insert(name.clone(), format!("$crate::__pgrx_c_generated::{field_marker}"))
                .is_none()
            {
                writeln!(output.rust, "#[doc(hidden)] pub struct {field_marker};")
                    .expect("String output");
            }
            if record_bridges.insert(storage.clone()) {
                record_bridge(&mut output.rust, &storage, binding, record.size, record.alignment);
            }
            let mut addresses = String::new();
            let mut previous = "base.pointer().as_mut_address()".to_owned();
            for (index, step) in projection.steps.iter().enumerate() {
                let step_storage = lowering
                    .resolve_with_storage(&step.field.ty, &step.binding.ty)?
                    .storage
                    .replace("$crate", "crate");
                let parent_storage = lowering
                    .resolve(&declarations.type_shapes[step.canonical].ty)?
                    .storage
                    .replace("$crate", "crate");
                let offset = step.field.offset_bits.expect("validated field offset") / 8;
                let rust_field = &step.binding.rust_name;
                writeln!(output.rust, "const _: () = assert!(::core::mem::offset_of!({parent_storage}, {rust_field}) == {offset});").expect("String output");
                let address = format!("address{index}");
                let cast = if matches!(step.binding.ty, RustBindingType::ManuallyDrop { .. }) {
                    format!(".cast::<{step_storage}>()")
                } else {
                    String::new()
                };
                writeln!(addresses, "let {address} = unsafe {{ ::core::ptr::addr_of_mut!((*{previous}).{rust_field}) }}{cast};").expect("String output");
                if let Some(size) = step.field.ty.size {
                    let actual =
                        lowering.storage_type(&step.binding.ty, 0)?.replace("$crate", "crate");
                    writeln!(output.rust, "const _: () = {{ assert!(::core::mem::size_of::<{actual}>() == {size}); assert!(::core::mem::align_of::<{actual}>() == {}); }};", step.field.ty.alignment.ok_or("projected field has no compiler alignment")?).expect("String output");
                }
                previous = address;
            }
            if flexible {
                writeln!(output.rust, "const _: () = {{ assert!(::core::mem::size_of::<{projected_storage}>() == 0); assert!(::core::mem::align_of::<{projected_storage}>() == {field_alignment}); }};").expect("String output");
            }
            let qualification = if projection.steps.iter().any(|step| step.field.ty.is_const) {
                format!("{EXPRESSION}::ReadOnly")
            } else {
                "Q".into()
            };
            let volatile = projection.steps.iter().any(|step| step.field.ty.is_volatile);
            if volatile && !field.ty.is_volatile {
                marker = format!("{EXPRESSION}::CVolatile<{marker}>");
            }
            let inherited_alignment =
                if field_alignment == 1 { "false" } else { "base.access().unaligned" };
            writeln!(output.rust,
                "// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.\n\
                 unsafe impl<Q: {EXPRESSION}::Qualifier> {EXPRESSION}::Field<{field_marker}, Q> for {EXPRESSION}::CRecord<{storage}> {{\n\
                   type Output = {EXPRESSION}::Place<{marker}, {qualification}>;\n\
                   unsafe fn project(base: {EXPRESSION}::Place<Self, Q>) -> Self::Output {{\n\
                     // SAFETY: The caller establishes in-bounds projection; no containing record is loaded and no reference is created.\n\
                     {addresses}\n\
                     {EXPRESSION}::Place::new(<{qualification} as {EXPRESSION}::Qualifier>::from_mut({previous}), {EXPRESSION}::Access {{ volatile: base.access().volatile || {}, unaligned: {inherited_alignment} || {unaligned} }})\n\
                   }}\n\
                 }}", volatile).expect("String output");
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
                let Some(binding) = records_by_path.get(&storage).copied() else { continue };
                let Some(record) = declarations.records.get(&ty.canonical_spelling) else {
                    continue;
                };
                record_bridge(&mut output.rust, &storage, binding, record.size, record.alignment);
            }
        }
    }
    if output.rust.len() > MAX_ADAPTER_BYTES {
        return Err("generated field adapters exceed the 16 MiB source budget".into());
    }
    Ok(output)
}

#[derive(Clone, Copy)]
struct ProjectionStep<'a> {
    canonical: &'a str,
    field: &'a FieldInfo,
    binding: &'a FieldBinding,
}
struct Projection<'a> {
    field: &'a FieldInfo,
    steps: Vec<ProjectionStep<'a>>,
}

fn collect_projections<'a>(
    canonical: &'a str,
    declarations: &'a DeclarationCatalog,
    lowering: &Lowering<'a>,
    used: &BTreeSet<String>,
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

fn validate_projection(
    projection: &Projection<'_>,
    record_alignment: Option<u64>,
    lowering: &Lowering<'_>,
    declarations: &DeclarationCatalog,
) -> Result<(String, bool, u64, String, bool), String> {
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
    Ok((
        lowered.marker.replace("$crate", "crate"),
        unaligned,
        alignment,
        lowered.storage.replace("$crate", "crate"),
        flexible,
    ))
}

fn record_bridge(
    rust: &mut String,
    storage: &str,
    binding: &RecordBinding,
    size: Option<u64>,
    alignment: Option<u64>,
) {
    if size.is_none() || alignment.is_none() {
        writeln!(rust, "impl crate::__pgrx_c_macros::sealed::Sealed for {storage} {{}}\nimpl {EXPRESSION}::NativeType for {storage} {{ type Marker = {EXPRESSION}::COpaque<Self>; }}").expect("String output");
        return;
    }
    let size = size.expect("complete record size");
    let alignment = alignment.expect("complete record alignment");
    writeln!(rust,
        "const _: () = {{ assert!(::core::mem::size_of::<{storage}>() == {size}); assert!(::core::mem::align_of::<{storage}>() == {alignment}); }};\n\
         impl crate::__pgrx_c_macros::sealed::Sealed for {storage} {{}}\n\
         impl {EXPRESSION}::NativeType for {storage} {{ type Marker = {EXPRESSION}::CRecord<Self>; }}").expect("String output");
    if binding.copy {
        writeln!(
            rust,
            "impl {EXPRESSION}::IntoExpression for {storage} {{\n\
               type Value = {EXPRESSION}::RecordValue<Self>;\n\
               fn into_expression(self) -> Self::Value {{ {EXPRESSION}::RecordValue::new(self) }}\n\
             }}"
        )
        .expect("String output");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FieldBinding, MacroScanner, RecordKind, RustBindingType, inspect};
    use std::path::PathBuf;
    use std::sync::Mutex;

    static SCANNER_LOCK: Mutex<()> = Mutex::new(());

    mod oracle {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/oracle.rs"));
    }
    mod rust_oracle {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/rust_oracle.rs"));
    }

    fn fixture() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/field_adapters.h")
    }

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

    #[test]
    fn generated_field_projections_match_original_c_without_whole_record_reads() {
        let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = MacroScanner::new().unwrap();
        let arguments = arguments();
        let frontend = inspect(&scanner, &fixture(), &arguments, None).unwrap();
        let names = ["count", "value", "child", "next", "frozen", "signal", "array"]
            .into_iter()
            .map(str::to_owned)
            .collect();
        let generated =
            generate(frontend.declarations(), &bindings(), &names, &frontend.profile().target, &[])
                .unwrap();
        assert!(generated.unsupported.values().any(|reason| reason.contains("unaligned volatile")));
        assert_eq!(generated.rust.matches("Sealed for crate::Child").count(), 1);
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
            "#[path = {support:?}] pub mod __pgrx_c_macros;\n{rust_bindings}\npub mod __pgrx_c_generated {{ {} }}\n",
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
"#, count=marker_name("count"), child=marker_name("child"), value=marker_name("value"), next=marker_name("next"), signal=marker_name("signal"), frozen=marker_name("frozen"), array=marker_name("array")));
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
        assert_eq!(actual, expected);
    }

    #[test]
    fn field_capabilities_reject_storage_disagreements_and_deferred_layouts() {
        let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let scanner = MacroScanner::new().unwrap();
        let frontend = inspect(&scanner, &fixture(), &arguments(), None).unwrap();
        let mut bindings = bindings();
        bindings.records.get_mut("Outer").unwrap().fields.get_mut("count").unwrap().ty =
            RustBindingType::Integer { signed: true, bits: 32 };
        let names = ["count", "bits", "array"].into_iter().map(str::to_owned).collect();
        let generated =
            generate(frontend.declarations(), &bindings, &names, &frontend.profile().target, &[])
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
    }
}
