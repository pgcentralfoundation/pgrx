//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use pgrx_c_macros::{
    AliasBinding, BindingCatalog, BitfieldBinding, DeclarationCatalog, EnumBinding, FieldBinding,
    FunctionBinding, IntegerBinding, IntegerBindingRepresentation, IntegerConstant, IntegerKind,
    IntegerValue, RecordBinding, RecordKind, RustBindingType, TargetFacts, VariableBinding,
};
use std::collections::{BTreeMap, BTreeSet};
use syn::{Expr, Item, Lit, Type, UnOp};

/// Discover bindgen's actual values and paths without changing its bindings.
/// The emitter compares these values with Clang before referencing them.
pub(super) fn collect_bindings(
    file: &syn::File,
    objects: &BTreeMap<String, IntegerConstant>,
    declarations: &DeclarationCatalog,
    target: &TargetFacts,
) -> BindingCatalog {
    let mut bindings = BindingCatalog::default();
    let mut duplicates = BTreeSet::new();
    collect(&file.items, &mut Vec::new(), target, &mut bindings, &mut duplicates);
    let wrappers = flexible_array_helpers(&file.items, &mut Vec::new());
    let mut structured_duplicates = StructuredDuplicates::default();
    collect_structures(
        &file.items,
        &mut Vec::new(),
        target,
        &mut bindings,
        &mut structured_duplicates,
        &wrappers,
    );
    structured_duplicates.remove(&mut bindings);
    // Rustified enums keep C enumerators as associated variants rather than
    // standalone constants. Preserve their actual paths and bindgen values so
    // the ordinary Clang disagreement check applies before symbolic emission.
    let enum_constants = bindings
        .enums
        .values()
        .flat_map(|binding| {
            binding.variants.iter().filter_map(|(variant, value)| {
                let name = if declarations.integer_constants.contains_key(variant) {
                    variant.as_str()
                } else {
                    let original = variant.strip_suffix('_')?;
                    declarations.integer_constants.contains_key(original).then_some(original)?
                };
                let mut path = binding.path.clone();
                path.push(variant.clone());
                Some((
                    name.to_owned(),
                    IntegerBinding {
                        path,
                        value: *value,
                        representation: IntegerBindingRepresentation::Primitive,
                    },
                ))
            })
        })
        .collect::<Vec<_>>();
    for (name, binding) in enum_constants {
        if bindings.integer_constants.insert(name.clone(), binding).is_some() {
            duplicates.insert(name);
        }
    }
    bindings.integer_constants.retain(|name, _| {
        !duplicates.contains(name)
            && (objects.contains_key(name) || declarations.integer_constants.contains_key(name))
    });
    collect_bitfields(&file.items, &mut Vec::new(), target, &mut bindings, &wrappers);
    bindings
}

fn collect_bitfields(
    items: &[Item],
    path: &mut Vec<String>,
    target: &TargetFacts,
    catalog: &mut BindingCatalog,
    wrappers: &BTreeSet<Vec<String>>,
) {
    let units = items
        .iter()
        .filter_map(|item| {
            let Item::Struct(item) = item else { return None };
            let fields = item
                .fields
                .iter()
                .filter_map(|field| {
                    Some((field.ident.as_ref()?.to_string(), bitfield_unit_bytes(&field.ty)?))
                })
                .collect::<BTreeMap<_, _>>();
            (!fields.is_empty()).then(|| (item_path(path, &item.ident), fields))
        })
        .collect::<BTreeMap<_, _>>();
    for item in items {
        match item {
            Item::Impl(item) if item.trait_.is_none() && item.generics.params.is_empty() => {
                let Some(RustBindingType::Named { path: record_path }) =
                    binding_type(&item.self_ty, path, target, 0, wrappers)
                else {
                    continue;
                };
                let Some(record) = record_path
                    .last()
                    .and_then(|name| catalog.records.get(name.trim_start_matches("r#")))
                    .filter(|record| record.path == record_path)
                else {
                    continue;
                };
                for method in &item.items {
                    let syn::ImplItem::Fn(method) = method else { continue };
                    if !public(&method.vis) || method.sig.inputs.len() != 1 {
                        continue;
                    }
                    let Some(syn::FnArg::Receiver(receiver)) = method.sig.inputs.first() else {
                        continue;
                    };
                    if receiver.reference.is_none() || receiver.mutability.is_some() {
                        continue;
                    }
                    let Some(ty) = return_type(&method.sig.output, path, target, 0, wrappers)
                    else {
                        continue;
                    };
                    let mut access = BitfieldAccess::default();
                    let mut body = method.block.clone();
                    syn::visit_mut::VisitMut::visit_block_mut(&mut access, &mut body);
                    if access.0.len() != 1 {
                        continue;
                    }
                    let (storage_field, offset_bits, width) =
                        access.0.pop().expect("one bitfield access");
                    // Only actual bindgen units establish bitfield layout. Other
                    // getters named `get` remain unrelated Rust method calls.
                    let unit = units
                        .get(&record_path)
                        .and_then(|fields| fields.get(&storage_field))
                        .copied();
                    if record.fields.contains_key(&storage_field)
                        || width == 0
                        || !unit.is_some_and(|bytes| {
                            offset_bits
                                .checked_add(u64::from(width))
                                .is_some_and(|end| end <= bytes.saturating_mul(8))
                        })
                    {
                        continue;
                    }
                    let rust_name = method.sig.ident.to_string();
                    let key = format!(
                        "{}::{}",
                        record_path.join("::"),
                        rust_name.trim_start_matches("r#")
                    );
                    catalog.bitfields.insert(
                        key,
                        BitfieldBinding {
                            record_path: record_path.clone(),
                            rust_name,
                            ty,
                            storage_field,
                            offset_bits,
                            width,
                        },
                    );
                }
            }
            Item::Mod(module) if public(&module.vis) => {
                if let Some((_, items)) = &module.content {
                    path.push(module.ident.to_string());
                    collect_bitfields(items, path, target, catalog, wrappers);
                    path.pop();
                }
            }
            _ => {}
        }
    }
}

#[derive(Default)]
struct BitfieldAccess(Vec<(String, u64, u32)>);
impl syn::visit_mut::VisitMut for BitfieldAccess {
    fn visit_expr_method_call_mut(&mut self, call: &mut syn::ExprMethodCall) {
        if call.method == "get"
            && call.args.len() == 2
            && let Expr::Field(receiver) = call.receiver.as_ref()
            && let Expr::Path(base) = receiver.base.as_ref()
            && base.path.is_ident("self")
            && let syn::Member::Named(storage) = &receiver.member
            && let Some(offset) = literal_u64(&call.args[0])
            && let Some(width) =
                literal_u64(&call.args[1]).and_then(|width| u32::try_from(width).ok())
        {
            self.0.push((storage.to_string(), offset, width));
        }
        syn::visit_mut::visit_expr_method_call_mut(self, call);
    }
}

fn literal_u64(expression: &Expr) -> Option<u64> {
    let Expr::Lit(literal) = expression else { return None };
    let Lit::Int(literal) = &literal.lit else { return None };
    literal.base10_parse().ok()
}

fn bitfield_unit_bytes(ty: &Type) -> Option<u64> {
    let Type::Path(ty) = ty else { return None };
    let segment = ty.path.segments.last()?;
    if segment.ident != "__BindgenBitfieldUnit" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else { return None };
    let Some(syn::GenericArgument::Type(Type::Array(array))) = arguments.args.first() else {
        return None;
    };
    if arguments.args.len() != 1 {
        return None;
    }
    let Type::Path(element) = array.elem.as_ref() else { return None };
    if !element.path.is_ident("u8") {
        return None;
    }
    literal_u64(&array.len)
}

/// Separate namespaces retain the same-named Rust type and foreign function, while
/// collisions between public modules cannot accidentally select one C declaration.
#[derive(Default)]
struct StructuredDuplicates {
    functions: BTreeSet<String>,
    records: BTreeSet<String>,
    types: BTreeSet<String>,
    enums: BTreeSet<String>,
    variables: BTreeSet<String>,
}

impl StructuredDuplicates {
    fn remove(self, bindings: &mut BindingCatalog) {
        for name in self.functions {
            bindings.functions.remove(&name);
        }
        for name in self.records {
            bindings.records.remove(&name);
        }
        for name in self.types {
            bindings.types.remove(&name);
        }
        for name in self.enums {
            bindings.enums.remove(&name);
        }
        for name in self.variables {
            bindings.variables.remove(&name);
        }
    }
}

fn insert_unique<T>(
    map: &mut BTreeMap<String, T>,
    duplicates: &mut BTreeSet<String>,
    name: String,
    value: T,
) {
    if map.insert(name.clone(), value).is_some() {
        duplicates.insert(name);
    }
}

fn public(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}

fn rust_name(ident: &syn::Ident) -> String {
    ident.to_string().trim_start_matches("r#").to_owned()
}

fn item_path(path: &[String], ident: &syn::Ident) -> Vec<String> {
    let mut result = path.to_vec();
    result.push(ident.to_string());
    result
}

/// Establish the zero-length storage layout from the emitted definition, rather
/// than accepting a generic helper solely because of its conventional name.
fn flexible_array_helpers(items: &[Item], path: &mut Vec<String>) -> BTreeSet<Vec<String>> {
    let mut verified = BTreeSet::new();
    for item in items {
        match item {
            Item::Struct(item) if public(&item.vis) => {
                let syn::Fields::Unnamed(fields) = &item.fields else { continue };
                let Some(syn::GenericParam::Type(parameter)) = item.generics.params.first() else {
                    continue;
                };
                if item.generics.params.len() != 1
                    || !parameter.bounds.is_empty()
                    || parameter.default.is_some()
                    || item.generics.where_clause.is_some()
                    || fields.unnamed.len() != 2
                    || attribute_arguments(&item.attrs, "repr").len() != 1
                    || !attribute_arguments(&item.attrs, "repr")[0].path().is_ident("C")
                {
                    continue;
                }
                let Type::Path(phantom) = &fields.unnamed[0].ty else { continue };
                let parts = phantom
                    .path
                    .segments
                    .iter()
                    .map(|part| part.ident.to_string())
                    .collect::<Vec<_>>();
                if phantom.qself.is_some()
                    || !matches!(parts.as_slice(), [root, marker, name] if (root == "core" || root == "std") && marker == "marker" && name == "PhantomData")
                {
                    continue;
                }
                let syn::PathArguments::AngleBracketed(arguments) =
                    &phantom.path.segments.last().expect("three segments").arguments
                else {
                    continue;
                };
                if arguments.args.len() != 1 {
                    continue;
                }
                let Some(syn::GenericArgument::Type(Type::Path(element))) = arguments.args.first()
                else {
                    continue;
                };
                let Type::Array(array) = &fields.unnamed[1].ty else { continue };
                let Type::Path(array_element) = array.elem.as_ref() else { continue };
                if element.qself.is_none()
                    && element.path.is_ident(&parameter.ident)
                    && array_element.qself.is_none()
                    && array_element.path.is_ident(&parameter.ident)
                    && literal_u64(&array.len) == Some(0)
                {
                    verified.insert(item_path(path, &item.ident));
                }
            }
            Item::Mod(module) if public(&module.vis) => {
                if let Some((_, items)) = &module.content {
                    path.push(module.ident.to_string());
                    verified.extend(flexible_array_helpers(items, path));
                    path.pop();
                }
            }
            _ => {}
        }
    }
    verified
}

fn collect_structures(
    items: &[Item],
    path: &mut Vec<String>,
    target: &TargetFacts,
    catalog: &mut BindingCatalog,
    duplicates: &mut StructuredDuplicates,
    wrappers: &BTreeSet<Vec<String>>,
) {
    for item in items {
        match item {
            Item::ForeignMod(block) => {
                let abi = abi_name(Some(&block.abi));
                for item in &block.items {
                    match item {
                        syn::ForeignItem::Fn(function)
                            if public(&function.vis) && function.sig.generics.params.is_empty() =>
                        {
                            let parameters = function
                                .sig
                                .inputs
                                .iter()
                                .map(|argument| match argument {
                                    syn::FnArg::Typed(argument) => {
                                        binding_type(&argument.ty, path, target, 0, wrappers)
                                    }
                                    syn::FnArg::Receiver(_) => None,
                                })
                                .collect::<Option<Vec<_>>>();
                            let Some(parameters) = parameters else { continue };
                            let Some(result) =
                                return_type(&function.sig.output, path, target, 0, wrappers)
                            else {
                                continue;
                            };
                            let name = rust_name(&function.sig.ident);
                            let link_name =
                                link_name(&function.attrs).unwrap_or_else(|| name.clone());
                            let variadic = function.sig.variadic.is_some();
                            let uses_cshim = link_name.ends_with("__pgrx_cshim");
                            insert_unique(
                                &mut catalog.functions,
                                &mut duplicates.functions,
                                name,
                                FunctionBinding {
                                    path: item_path(path, &function.sig.ident),
                                    parameters,
                                    result,
                                    abi: abi.clone(),
                                    variadic,
                                    link_name,
                                    guarded: !variadic,
                                    uses_cshim,
                                },
                            );
                        }
                        syn::ForeignItem::Static(variable) if public(&variable.vis) => {
                            let Some(ty) = binding_type(&variable.ty, path, target, 0, wrappers)
                            else {
                                continue;
                            };
                            insert_unique(
                                &mut catalog.variables,
                                &mut duplicates.variables,
                                rust_name(&variable.ident),
                                VariableBinding {
                                    path: item_path(path, &variable.ident),
                                    ty,
                                    mutable: matches!(
                                        variable.mutability,
                                        syn::StaticMutability::Mut(_)
                                    ),
                                },
                            );
                        }
                        _ => {}
                    }
                }
            }
            Item::Struct(record) if public(&record.vis) && record.generics.params.is_empty() => {
                if let syn::Fields::Named(fields) = &record.fields {
                    insert_unique(
                        &mut catalog.records,
                        &mut duplicates.records,
                        rust_name(&record.ident),
                        record_binding(
                            &record.ident,
                            &record.attrs,
                            &fields.named,
                            RecordKind::Struct,
                            path,
                            target,
                            wrappers,
                        ),
                    );
                }
            }
            Item::Union(record) if public(&record.vis) && record.generics.params.is_empty() => {
                insert_unique(
                    &mut catalog.records,
                    &mut duplicates.records,
                    rust_name(&record.ident),
                    record_binding(
                        &record.ident,
                        &record.attrs,
                        &record.fields.named,
                        RecordKind::Union,
                        path,
                        target,
                        wrappers,
                    ),
                );
            }
            Item::Type(alias) if public(&alias.vis) && alias.generics.params.is_empty() => {
                if let Some(target_type) = binding_type(&alias.ty, path, target, 0, wrappers) {
                    let alias_path = item_path(path, &alias.ident);
                    insert_unique(
                        &mut catalog.types,
                        &mut duplicates.types,
                        alias_path
                            .iter()
                            .map(|part| part.trim_start_matches("r#"))
                            .collect::<Vec<_>>()
                            .join("::"),
                        AliasBinding { path: alias_path, target: target_type },
                    );
                }
            }
            Item::Enum(item) if public(&item.vis) && item.generics.params.is_empty() => {
                if let Some(binding) = enum_binding(item, path, target, wrappers) {
                    insert_unique(
                        &mut catalog.enums,
                        &mut duplicates.enums,
                        rust_name(&item.ident),
                        binding,
                    );
                }
            }
            Item::Mod(module) if public(&module.vis) => {
                if let Some((_, items)) = &module.content {
                    path.push(module.ident.to_string());
                    collect_structures(items, path, target, catalog, duplicates, wrappers);
                    path.pop();
                }
            }
            _ => {}
        }
    }
}

fn record_binding(
    ident: &syn::Ident,
    attrs: &[syn::Attribute],
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    kind: RecordKind,
    path: &[String],
    target: &TargetFacts,
    wrappers: &BTreeSet<Vec<String>>,
) -> RecordBinding {
    let fields = fields
        .iter()
        .filter(|field| public(&field.vis))
        .filter_map(|field| {
            let ident = field.ident.as_ref()?;
            Some((
                rust_name(ident),
                FieldBinding {
                    rust_name: ident.to_string(),
                    ty: binding_type(&field.ty, path, target, 0, wrappers)?,
                },
            ))
        })
        .collect();
    let packed =
        attribute_arguments(attrs, "repr").iter().any(|meta| meta.path().is_ident("packed"));
    let copy = attribute_arguments(attrs, "derive").iter().any(|meta| meta.path().is_ident("Copy"));
    RecordBinding { path: item_path(path, ident), kind, fields, packed, copy }
}

fn attribute_arguments(attrs: &[syn::Attribute], name: &str) -> Vec<syn::Meta> {
    attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident(name))
        .filter_map(|attribute| {
            attribute
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .ok()
        })
        .flatten()
        .collect()
}

fn enum_binding(
    item: &syn::ItemEnum,
    path: &[String],
    target: &TargetFacts,
    wrappers: &BTreeSet<Vec<String>>,
) -> Option<EnumBinding> {
    let repr = attribute_arguments(&item.attrs, "repr").iter().find_map(|meta| {
        let syn::Meta::Path(path_meta) = meta else { return None };
        let ty = Type::Path(syn::TypePath { qself: None, path: path_meta.clone() });
        let value = binding_type(&ty, path, target, 0, wrappers)?;
        matches!(value, RustBindingType::Integer { .. }).then_some(value)
    });
    let mut variants = BTreeMap::new();
    let mut next = Some(IntegerValue::Unsigned(0));
    for variant in &item.variants {
        if !matches!(variant.fields, syn::Fields::Unit) {
            return None;
        }
        let value = match &variant.discriminant {
            Some((_, expr)) => rust_value(expr)?,
            None => next?,
        };
        next = match value {
            IntegerValue::Signed(value) => value.checked_add(1).map(IntegerValue::Signed),
            IntegerValue::Unsigned(value) => value.checked_add(1).map(IntegerValue::Unsigned),
        };
        variants.insert(rust_name(&variant.ident), value);
    }
    Some(EnumBinding { path: item_path(path, &item.ident), repr, variants })
}

fn abi_name(abi: Option<&syn::Abi>) -> String {
    abi.map(|abi| abi.name.as_ref().map_or_else(|| "C".into(), syn::LitStr::value))
        .unwrap_or_else(|| "Rust".into())
}

fn link_name(attrs: &[syn::Attribute]) -> Option<String> {
    attrs.iter().find_map(|attribute| {
        let syn::Meta::NameValue(meta) = &attribute.meta else { return None };
        if !meta.path.is_ident("link_name") {
            return None;
        }
        let Expr::Lit(value) = &meta.value else { return None };
        let Lit::Str(value) = &value.lit else { return None };
        Some(value.value())
    })
}

fn return_type(
    output: &syn::ReturnType,
    path: &[String],
    target: &TargetFacts,
    depth: usize,
    wrappers: &BTreeSet<Vec<String>>,
) -> Option<RustBindingType> {
    match output {
        syn::ReturnType::Default => Some(RustBindingType::Unit),
        syn::ReturnType::Type(_, ty) => binding_type(ty, path, target, depth + 1, wrappers),
    }
}

fn binding_type(
    ty: &Type,
    path: &[String],
    target: &TargetFacts,
    depth: usize,
    wrappers: &BTreeSet<Vec<String>>,
) -> Option<RustBindingType> {
    if depth > 64 {
        return None;
    }
    match ty {
        Type::Tuple(tuple) if tuple.elems.is_empty() => Some(RustBindingType::Unit),
        Type::Ptr(pointer) => Some(RustBindingType::Pointer {
            pointee: Box::new(binding_type(&pointer.elem, path, target, depth + 1, wrappers)?),
            mutable: pointer.mutability.is_some(),
        }),
        Type::Array(array) => {
            let Expr::Lit(length) = &array.len else { return None };
            let Lit::Int(length) = &length.lit else { return None };
            Some(RustBindingType::Array {
                element: Box::new(binding_type(&array.elem, path, target, depth + 1, wrappers)?),
                length: length.base10_parse().ok()?,
            })
        }
        Type::BareFn(function) if function.lifetimes.is_none() => Some(RustBindingType::Function {
            parameters: function
                .inputs
                .iter()
                .map(|argument| binding_type(&argument.ty, path, target, depth + 1, wrappers))
                .collect::<Option<Vec<_>>>()?,
            result: Box::new(return_type(&function.output, path, target, depth + 1, wrappers)?),
            abi: abi_name(function.abi.as_ref()),
            unsafe_: function.unsafety.is_some(),
            variadic: function.variadic.is_some(),
        }),
        Type::Path(named) if named.qself.is_none() => {
            let parts =
                named.path.segments.iter().map(|part| part.ident.to_string()).collect::<Vec<_>>();
            let last = named.path.segments.last()?;
            if matches!(parts.as_slice(), [name] if name == "Option")
                || matches!(parts.as_slice(), [root, option, name] if (root == "core" || root == "std") && option == "option" && name == "Option")
            {
                let syn::PathArguments::AngleBracketed(arguments) = &last.arguments else {
                    return None;
                };
                if arguments.args.len() != 1 {
                    return None;
                }
                let syn::GenericArgument::Type(value) = arguments.args.first()? else {
                    return None;
                };
                return Some(RustBindingType::Option {
                    value: Box::new(binding_type(value, path, target, depth + 1, wrappers)?),
                });
            }
            if matches!(parts.as_slice(), [root, memory, name] if (root == "core" || root == "std") && memory == "mem" && (name == "ManuallyDrop" || name == "MaybeUninit"))
                || wrappers.contains(&resolve_path(&parts, path)?)
            {
                if named
                    .path
                    .segments
                    .iter()
                    .rev()
                    .skip(1)
                    .any(|segment| !segment.arguments.is_empty())
                {
                    return None;
                }
                let syn::PathArguments::AngleBracketed(arguments) = &last.arguments else {
                    return None;
                };
                if arguments.args.len() != 1 {
                    return None;
                }
                let syn::GenericArgument::Type(value) = arguments.args.first()? else {
                    return None;
                };
                let value = Box::new(binding_type(value, path, target, depth + 1, wrappers)?);
                return Some(if last.ident == "ManuallyDrop" {
                    RustBindingType::ManuallyDrop { value }
                } else if last.ident == "MaybeUninit" {
                    RustBindingType::MaybeUninit { value }
                } else {
                    RustBindingType::IncompleteArrayField {
                        path: resolve_path(&parts, path)?,
                        element: value,
                    }
                });
            }
            if named.path.segments.iter().any(|segment| !segment.arguments.is_empty()) {
                return None;
            }
            if parts.len() == 1
                || matches!(parts.as_slice(), [root, primitive, _] if (root == "core" || root == "std") && primitive == "primitive")
            {
                let primitive = match last.ident.to_string().as_str() {
                    "bool" => return Some(RustBindingType::Bool),
                    "u8" => Some((false, 8)),
                    "u16" => Some((false, 16)),
                    "u32" => Some((false, 32)),
                    "u64" => Some((false, 64)),
                    "u128" => Some((false, 128)),
                    "i8" => Some((true, 8)),
                    "i16" => Some((true, 16)),
                    "i32" => Some((true, 32)),
                    "i64" => Some((true, 64)),
                    "i128" => Some((true, 128)),
                    "usize" | "isize" => {
                        return Some(RustBindingType::PointerSizedInteger {
                            signed: last.ident == "isize",
                            bits: target.pointer_bits,
                        });
                    }
                    "f32" => return Some(RustBindingType::Float { bits: 32 }),
                    "f64" => return Some(RustBindingType::Float { bits: 64 }),
                    _ => None,
                };
                if let Some((signed, bits)) = primitive {
                    return Some(RustBindingType::Integer { signed, bits });
                }
            }
            if matches!(parts.as_slice(), [root, ffi, _] if (root == "core" || root == "std") && ffi == "ffi")
                || matches!(parts.as_slice(), [root, os, raw, _] if root == "std" && os == "os" && raw == "raw")
                || matches!(parts.as_slice(), [root, _] if root == "libc")
            {
                if last.ident == "c_float" {
                    return Some(RustBindingType::Float { bits: 32 });
                }
                if last.ident == "c_double" {
                    return Some(RustBindingType::Float { bits: 64 });
                }
                let standard_type = Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::parse_quote!(::core::ffi::#last),
                });
                if let Some(storage) = storage(&standard_type, &BTreeMap::new(), target, 0) {
                    return Some(RustBindingType::Integer {
                        signed: storage.signed,
                        bits: storage.bits,
                    });
                }
            }
            Some(RustBindingType::Named { path: resolve_path(&parts, path)? })
        }
        Type::Group(group) => binding_type(&group.elem, path, target, depth + 1, wrappers),
        Type::Paren(group) => binding_type(&group.elem, path, target, depth + 1, wrappers),
        _ => None,
    }
}

fn resolve_path(parts: &[String], current: &[String]) -> Option<Vec<String>> {
    let mut resolved = current.to_vec();
    let mut position = 0;
    match parts.first()?.as_str() {
        "core" | "std" | "libc" => return Some(parts.to_vec()),
        "crate" => {
            resolved.clear();
            position = 1;
        }
        "self" => {
            position = 1;
        }
        "super" => {
            while parts.get(position).is_some_and(|part| part == "super") {
                resolved.pop()?;
                position += 1;
            }
        }
        _ => {}
    }
    resolved.extend_from_slice(&parts[position..]);
    Some(resolved)
}

fn collect(
    items: &[Item],
    path: &mut Vec<String>,
    target: &TargetFacts,
    catalog: &mut BindingCatalog,
    duplicates: &mut BTreeSet<String>,
) {
    let aliases = items
        .iter()
        .filter_map(|item| match item {
            Item::Type(item) => Some((item.ident.to_string(), item.ty.as_ref())),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    for item in items {
        match item {
            Item::Const(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                let Some(storage) = storage(&item.ty, &aliases, target, 0) else { continue };
                let Some(value) = rust_value(&item.expr) else { continue };
                if !storage.contains(numeric(value)) {
                    continue;
                }
                let name = item.ident.to_string().trim_start_matches("r#").to_owned();
                let representation = if path.is_empty()
                    && super::is_builtin_oid(&name)
                    && matches!(item.ty.as_ref(), Type::Path(ty) if ty.path.is_ident("u32"))
                {
                    IntegerBindingRepresentation::Oid
                } else {
                    IntegerBindingRepresentation::Primitive
                };
                path.push(name.clone());
                if catalog
                    .integer_constants
                    .insert(
                        name.clone(),
                        IntegerBinding { path: path.clone(), value, representation },
                    )
                    .is_some()
                {
                    duplicates.insert(name);
                }
                path.pop();
            }
            Item::Mod(module) if matches!(module.vis, syn::Visibility::Public(_)) => {
                if let Some((_, items)) = &module.content {
                    path.push(module.ident.to_string().trim_start_matches("r#").to_owned());
                    collect(items, path, target, catalog, duplicates);
                    path.pop();
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy)]
struct Storage {
    signed: bool,
    bits: u32,
    boolean: bool,
}

impl Storage {
    fn contains(self, value: i128) -> bool {
        if self.boolean {
            return (0..=1).contains(&value);
        }
        if self.signed {
            let bound = 1i128 << (self.bits - 1);
            (-bound..bound).contains(&value)
        } else {
            (0..(1i128 << self.bits)).contains(&value)
        }
    }
}

fn storage(
    ty: &Type,
    aliases: &BTreeMap<String, &Type>,
    target: &TargetFacts,
    depth: usize,
) -> Option<Storage> {
    if depth > 16 {
        return None;
    }
    let Type::Path(ty) = ty else { return None };
    if ty.qself.is_some() || ty.path.segments.iter().any(|part| !part.arguments.is_empty()) {
        return None;
    }
    let name = ty.path.segments.last()?.ident.to_string();
    if ty.path.segments.len() == 1
        && let Some(alias) = aliases.get(&name)
    {
        return storage(alias, aliases, target, depth + 1);
    }
    let parts = ty.path.segments.iter().map(|part| part.ident.to_string()).collect::<Vec<_>>();
    let is_primitive = parts.len() == 1
        || matches!(parts.as_slice(), [root, primitive, _] if (root == "core" || root == "std") && primitive == "primitive");
    let primitive = if is_primitive {
        match name.as_str() {
            "bool" => return Some(Storage { signed: false, bits: 1, boolean: true }),
            "usize" => Some((false, target.pointer_bits)),
            "isize" => Some((true, target.pointer_bits)),
            "u8" => Some((false, 8)),
            "u16" => Some((false, 16)),
            "u32" => Some((false, 32)),
            "u64" => Some((false, 64)),
            "i8" => Some((true, 8)),
            "i16" => Some((true, 16)),
            "i32" => Some((true, 32)),
            "i64" => Some((true, 64)),
            _ => None,
        }
    } else {
        None
    };
    if let Some((signed, bits)) = primitive {
        return Some(Storage { signed, bits, boolean: false });
    }
    // Only standard library C aliases are accepted; an unrelated type with the
    // same final identifier does not establish primitive storage.
    if !matches!(parts.as_slice(), [root, ffi, _] if (root == "core" || root == "std") && ffi == "ffi")
    {
        return None;
    }
    let kind = match name.as_str() {
        "c_char" => IntegerKind::Char,
        "c_schar" => IntegerKind::SignedChar,
        "c_uchar" => IntegerKind::UnsignedChar,
        "c_short" => IntegerKind::Short,
        "c_ushort" => IntegerKind::UnsignedShort,
        "c_int" => IntegerKind::Int,
        "c_uint" => IntegerKind::UnsignedInt,
        "c_long" => IntegerKind::Long,
        "c_ulong" => IntegerKind::UnsignedLong,
        "c_longlong" => IntegerKind::LongLong,
        "c_ulonglong" => IntegerKind::UnsignedLongLong,
        _ => return None,
    };
    let integer = target.integers.get(&kind)?;
    (integer.bits <= 64).then_some(Storage {
        signed: integer.signed,
        bits: integer.bits,
        boolean: false,
    })
}

fn numeric(value: IntegerValue) -> i128 {
    match value {
        IntegerValue::Signed(value) => value.into(),
        IntegerValue::Unsigned(value) => value.into(),
    }
}

fn rust_value(expr: &Expr) -> Option<IntegerValue> {
    match expr {
        Expr::Lit(expr) => match &expr.lit {
            Lit::Int(value) => value.base10_parse::<u64>().ok().map(IntegerValue::Unsigned),
            Lit::Bool(value) => Some(IntegerValue::Unsigned(u64::from(value.value))),
            _ => None,
        },
        Expr::Unary(expr) if matches!(expr.op, UnOp::Neg(_)) => {
            i64::try_from(-numeric(rust_value(&expr.expr)?)).ok().map(IntegerValue::Signed)
        }
        Expr::Paren(expr) => rust_value(&expr.expr),
        Expr::Group(expr) => rust_value(&expr.expr),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pgrx_c_macros::{ByteOrder, IntegerType, TypeCategory, TypeInfo};
    use quote::ToTokens;

    fn target() -> TargetFacts {
        TargetFacts {
            triple: "x86_64-unknown-linux-gnu".into(),
            pointer_bits: 64,
            function_pointer: pgrx_c_macros::PointerLayout { size: 8, alignment: 8 },
            char_bits: 8,
            char_is_signed: true,
            ascii_execution_charset: true,
            byte_order: ByteOrder::Little,
            c_standard: Some(201112),
            floating_point: Default::default(),
            integers: [
                (IntegerKind::UnsignedChar, false, 8, 1),
                (IntegerKind::Int, true, 32, 3),
                (IntegerKind::UnsignedInt, false, 32, 3),
                (IntegerKind::UnsignedLong, false, 64, 4),
            ]
            .into_iter()
            .map(|(kind, signed, bits, rank)| (kind, IntegerType { kind, signed, bits, rank }))
            .collect(),
        }
    }

    fn constant(kind: IntegerKind, value: IntegerValue) -> IntegerConstant {
        IntegerConstant {
            ty: TypeInfo {
                spelling: "test integer".into(),
                canonical_spelling: "test integer".into(),
                category: TypeCategory::Integer(kind),
                size: None,
                alignment: None,
                is_const: false,
                is_volatile: false,
            },
            value,
            literal: None,
        }
    }

    #[test]
    fn preserves_bindgen_values_storage_and_enum_oid_paths() {
        let file = syn::parse_file(
            r#"
pub const Limit: u32 = 0;
pub const Stable: u32 = 17;
pub const TYPEOID: u32 = 42;
pub const HEAP_HASOID: u32 = 8;
pub mod Choice { pub type Type = ::core::ffi::c_uint; pub const PICK: Type = 7; }
pub mod Negative { pub type Type = ::core::ffi::c_int; pub const NEG: Type = -1; }
"#,
        )
        .unwrap();
        let objects = BTreeMap::from([
            (
                "Limit".into(),
                constant(IntegerKind::UnsignedLong, IntegerValue::Unsigned(u64::MAX / 2)),
            ),
            ("Stable".into(), constant(IntegerKind::Int, IntegerValue::Signed(17))),
            ("TYPEOID".into(), constant(IntegerKind::Int, IntegerValue::Signed(42))),
            ("HEAP_HASOID".into(), constant(IntegerKind::Int, IntegerValue::Signed(8))),
        ]);
        let declarations = DeclarationCatalog {
            integer_constants: BTreeMap::from([
                ("PICK".into(), constant(IntegerKind::Int, IntegerValue::Signed(7))),
                ("NEG".into(), constant(IntegerKind::Int, IntegerValue::Signed(-1))),
            ]),
            ..DeclarationCatalog::default()
        };
        let original = file.to_token_stream().to_string();
        let bindings = collect_bindings(&file, &objects, &declarations, &target());
        assert_eq!(
            file.to_token_stream().to_string(),
            original,
            "collection cannot change bindings"
        );
        let Item::Const(limit) = &file.items[0] else { panic!("constant") };
        assert!(matches!(limit.ty.as_ref(), Type::Path(ty) if ty.path.is_ident("u32")));
        assert_eq!(rust_value(&limit.expr), Some(IntegerValue::Unsigned(0)));
        assert_eq!(bindings.integer_constants["Limit"].path, ["Limit"]);
        assert_eq!(bindings.integer_constants["Limit"].value, IntegerValue::Unsigned(0));
        let Item::Const(stable) = &file.items[1] else { panic!("constant") };
        assert!(
            matches!(stable.ty.as_ref(), Type::Path(ty) if ty.path.is_ident("u32")),
            "retain bindgen storage for OID rewrites and existing callers"
        );
        assert_eq!(bindings.integer_constants["PICK"].path, ["Choice", "PICK"]);
        assert_eq!(bindings.integer_constants["NEG"].value, IntegerValue::Signed(-1));
        assert!(matches!(
            bindings.integer_constants["TYPEOID"].representation,
            IntegerBindingRepresentation::Oid
        ));
        assert!(matches!(
            bindings.integer_constants["HEAP_HASOID"].representation,
            IntegerBindingRepresentation::Primitive
        ));
    }

    #[test]
    fn collects_rustified_enum_symbols_without_replacing_bindgen_values() {
        let file = syn::parse_file(
            r#"
#[repr(u32)] pub enum Choice { PICK = 7, type_ = 12 }
#[repr(i32)] pub enum Negative { NEG = -1 }
pub mod A { #[repr(u32)] pub enum First { DUP = 1 } }
pub mod B { #[repr(u32)] pub enum Second { DUP = 1 } }
"#,
        )
        .unwrap();
        let declarations = DeclarationCatalog {
            integer_constants: BTreeMap::from([
                ("PICK".into(), constant(IntegerKind::Int, IntegerValue::Signed(7))),
                ("type".into(), constant(IntegerKind::Int, IntegerValue::Signed(13))),
                ("NEG".into(), constant(IntegerKind::Int, IntegerValue::Signed(-1))),
                ("DUP".into(), constant(IntegerKind::Int, IntegerValue::Signed(1))),
            ]),
            ..DeclarationCatalog::default()
        };
        let original = file.to_token_stream().to_string();
        let bindings = collect_bindings(&file, &BTreeMap::new(), &declarations, &target());
        assert_eq!(bindings.integer_constants["PICK"].path, ["Choice", "PICK"]);
        assert_eq!(bindings.integer_constants["type"].path, ["Choice", "type_"]);
        assert_eq!(bindings.integer_constants["type"].value, IntegerValue::Unsigned(12));
        assert_eq!(bindings.integer_constants["NEG"].value, IntegerValue::Signed(-1));
        assert!(!bindings.integer_constants.contains_key("DUP"));
        assert_eq!(file.to_token_stream().to_string(), original);
    }

    #[test]
    fn rejects_ambiguous_paths_unresolved_storage_and_nonintegral_constants() {
        let file = syn::parse_file(
            r#"
pub mod A { pub const DUP: u32 = 1; }
pub mod B { pub const DUP: u32 = 1; }
pub const WRONG: unrelated::u32 = 5;
pub const FLOAT: f64 = 1.0;
pub type Loop = Loop;
pub const CYCLE: Loop = 7;
"#,
        )
        .unwrap();
        let objects = ["DUP", "WRONG", "FLOAT", "CYCLE"]
            .into_iter()
            .map(|name| (name.into(), constant(IntegerKind::Int, IntegerValue::Signed(1))))
            .collect();
        let bindings = collect_bindings(&file, &objects, &DeclarationCatalog::default(), &target());
        assert!(bindings.integer_constants.is_empty());
        assert_eq!(
            rust_value(&syn::parse_str::<Expr>("-9223372036854775808i64").unwrap()),
            Some(IntegerValue::Signed(i64::MIN))
        );
        assert_eq!(
            rust_value(&syn::parse_str::<Expr>("0xFFFFFFFFFFFFFFFFu64").unwrap()),
            Some(IntegerValue::Unsigned(u64::MAX))
        );
    }

    #[test]
    fn catalogs_actual_function_abi_aliases_records_and_callback_types() {
        let file = syn::parse_file(
            r#"
pub type Count = ::core::ffi::c_uint;
pub type r#type = ::libc::c_uint;
pub type Callback = ::core::option::Option<unsafe extern "C" fn(Count) -> Count>;
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct Packet { pub r#type: Count, pub bytes: [u8; 8], hidden: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub union Value { pub signed: i32, pub unsigned: u32 }
#[repr(u32)]
pub enum Tag { First = 5, Next, Last = 17 }
pub mod Direction { pub type Type = ::core::ffi::c_int; }
pub mod Mode { pub type Type = ::core::ffi::c_uint; }
unsafe extern "C" {
    pub fn transform(input: *const Packet, output: *mut Packet, count: Count, callback: Callback) -> Count;
    #[link_name = "clear__pgrx_cshim"]
    pub fn clear(packet: *mut Packet);
    pub fn logging(first: i32, ...) -> i32;
    pub static mut Current: *mut Packet;
    pub static Frozen: *const Packet;
    fn private_call();
}
"#,
        )
        .unwrap();
        let original = file.to_token_stream().to_string();
        let catalog =
            collect_bindings(&file, &BTreeMap::new(), &DeclarationCatalog::default(), &target());
        assert_eq!(file.to_token_stream().to_string(), original);
        let transform = &catalog.functions["transform"];
        assert_eq!(transform.path, ["transform"]);
        assert_eq!(transform.abi, "C");
        assert!(transform.guarded && !transform.uses_cshim && !transform.variadic);
        assert_eq!(transform.link_name, "transform");
        assert_eq!(transform.parameters.len(), 4);
        assert!(
            matches!(&transform.parameters[0], RustBindingType::Pointer { mutable: false, pointee } if **pointee == RustBindingType::Named { path: vec!["Packet".into()] })
        );
        assert!(matches!(&transform.parameters[1], RustBindingType::Pointer { mutable: true, .. }));
        assert_eq!(transform.result, RustBindingType::Named { path: vec!["Count".into()] });
        let clear = &catalog.functions["clear"];
        assert_eq!(clear.link_name, "clear__pgrx_cshim");
        assert_eq!(clear.result, RustBindingType::Unit);
        assert!(clear.guarded && clear.uses_cshim);
        assert!(catalog.functions["logging"].variadic);
        assert!(!catalog.functions["logging"].guarded);
        assert!(!catalog.functions.contains_key("private_call"));
        let RustBindingType::Option { value } = &catalog.types["Callback"].target else {
            panic!("nullable callback")
        };
        assert!(
            matches!(value.as_ref(), RustBindingType::Function { abi, unsafe_: true, variadic: false, parameters, .. } if abi == "C" && parameters.len() == 1)
        );
        assert_eq!(
            catalog.types["Count"].target,
            RustBindingType::Integer { signed: false, bits: 32 }
        );
        assert_eq!(catalog.types["type"].path, ["r#type"]);
        assert_eq!(
            catalog.types["type"].target,
            RustBindingType::Integer { signed: false, bits: 32 }
        );
        assert_eq!(
            catalog.types["Direction::Type"].target,
            RustBindingType::Integer { signed: true, bits: 32 }
        );
        assert_eq!(
            catalog.types["Mode::Type"].target,
            RustBindingType::Integer { signed: false, bits: 32 }
        );
        let packet = &catalog.records["Packet"];
        assert_eq!(packet.kind, RecordKind::Struct);
        assert!(packet.packed && packet.copy);
        assert_eq!(packet.fields["type"].rust_name, "r#type");
        assert_eq!(
            packet.fields["bytes"].ty,
            RustBindingType::Array {
                element: Box::new(RustBindingType::Integer { signed: false, bits: 8 }),
                length: 8
            }
        );
        assert!(!packet.fields.contains_key("hidden"));
        assert_eq!(catalog.records["Value"].kind, RecordKind::Union);
        let tag = &catalog.enums["Tag"];
        assert_eq!(tag.repr, Some(RustBindingType::Integer { signed: false, bits: 32 }));
        assert_eq!(tag.variants["Next"], IntegerValue::Unsigned(6));
        assert!(catalog.variables["Current"].mutable);
        assert!(!catalog.variables["Frozen"].mutable);
    }

    #[test]
    fn recovers_bitfield_layout_from_actual_bindgen_getter_bodies() {
        let file = syn::parse_file(
            r#"
pub struct Bits { pub lead: u32, pub storage: __BindgenBitfieldUnit<[u8;4usize]> }
impl Bits {
    pub fn flag(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self.storage.get(3usize,7u8) as u32) }
    }
    pub fn exceeds_storage(&self) -> ::core::ffi::c_uint { self.storage.get(31usize,7u8) as u32 }
    pub fn ordinary_getter(&self) -> u32 { self.lead.get(0usize,1u8) }
}
"#,
        )
        .unwrap();
        let catalog =
            collect_bindings(&file, &BTreeMap::new(), &DeclarationCatalog::default(), &target());
        assert_eq!(catalog.bitfields.len(), 1);
        let flag = &catalog.bitfields["Bits::flag"];
        assert_eq!(flag.record_path, ["Bits"]);
        assert_eq!(flag.storage_field, "storage");
        assert_eq!(flag.offset_bits, 3);
        assert_eq!(flag.width, 7);
        assert_eq!(flag.ty, RustBindingType::Integer { signed: false, bits: 32 });
    }

    #[test]
    fn collects_actual_std_os_raw_function_and_field_storage() {
        let file = syn::parse_file(
            r#"
pub struct Bytes { pub byte: ::std::os::raw::c_uchar }
unsafe extern "C" {
    pub fn access(byte: ::std::os::raw::c_uchar, offset: usize) -> ::std::os::raw::c_int;
}
"#,
        )
        .unwrap();
        let catalog =
            collect_bindings(&file, &BTreeMap::new(), &DeclarationCatalog::default(), &target());
        let unsigned_byte = RustBindingType::Integer { signed: false, bits: 8 };
        assert_eq!(catalog.records["Bytes"].fields["byte"].ty, unsigned_byte);
        assert_eq!(catalog.functions["access"].parameters[0], unsigned_byte);
        assert_eq!(
            catalog.functions["access"].parameters[1],
            RustBindingType::PointerSizedInteger { signed: false, bits: target().pointer_bits }
        );
        assert_eq!(
            catalog.functions["access"].result,
            RustBindingType::Integer { signed: true, bits: 32 }
        );
    }

    #[test]
    fn rejects_ambiguous_and_unrepresentable_structured_bindings() {
        let file = syn::parse_file(
            r#"
pub mod A { pub struct Dup { pub value: u32 } unsafe extern "C" { pub fn same(); } }
pub mod B { pub struct Dup { pub value: u32 } unsafe extern "C" { pub fn same(); } }
pub struct Generic<T> { pub value: T }
pub enum Data { Variant(u32) }
pub enum UnknownValue { Variant = OTHER_VALUE }
unsafe extern "C" {
    pub fn borrowed(value: &u32);
    pub fn unsupported(value: impl Copy);
    pub fn never() -> !;
    pub fn good(value: Option<unsafe extern "C" fn(i32) -> i32>) -> *mut ::core::ffi::c_void;
}
"#,
        )
        .unwrap();
        let catalog =
            collect_bindings(&file, &BTreeMap::new(), &DeclarationCatalog::default(), &target());
        assert!(!catalog.functions.contains_key("same"));
        assert!(!catalog.records.contains_key("Dup"));
        assert!(!catalog.records.contains_key("Generic"));
        assert!(!catalog.enums.contains_key("Data"));
        assert!(!catalog.enums.contains_key("UnknownValue"));
        assert_eq!(catalog.functions.len(), 1);
        assert!(
            matches!(&catalog.functions["good"].result, RustBindingType::Pointer { pointee, .. } if **pointee == RustBindingType::Named { path: vec!["core".into(), "ffi".into(), "c_void".into()] })
        );
    }
    #[test]
    fn verifies_generic_union_and_flexible_array_storage_definitions() {
        let file = syn::parse_file(r#"
pub mod storage {
    #[repr(C)] pub struct Tail<T>(::core::marker::PhantomData<T>, [T; 0]);
    #[repr(C)] pub struct Wrong<T>(::core::marker::PhantomData<T>, [T; 1]);
    pub struct Unproved<T>(::core::marker::PhantomData<T>, [T; 0]);
    #[repr(C)] pub struct Record { pub tail: Tail<u32>, pub wrong: Wrong<u32>, pub unproved: Unproved<u32> }
}
#[repr(C)] pub union Union { pub record: ::core::mem::ManuallyDrop<storage::Record> }
"#).unwrap();
        let catalog =
            collect_bindings(&file, &BTreeMap::new(), &DeclarationCatalog::default(), &target());
        let fields = &catalog.records["Record"].fields;
        assert_eq!(
            fields.len(),
            1,
            "the helper's definition establishes its zero-length representation"
        );
        assert_eq!(
            fields["tail"].ty,
            RustBindingType::IncompleteArrayField {
                path: vec!["storage".into(), "Tail".into()],
                element: Box::new(RustBindingType::Integer { signed: false, bits: 32 }),
            }
        );
        assert_eq!(
            catalog.records["Union"].fields["record"].ty,
            RustBindingType::ManuallyDrop {
                value: Box::new(RustBindingType::Named {
                    path: vec!["storage".into(), "Record".into()]
                }),
            }
        );
    }
}
