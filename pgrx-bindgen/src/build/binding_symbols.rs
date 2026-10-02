//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use pgrx_c_macros::{
    BindingCatalog, DeclarationCatalog, IntegerBinding, IntegerBindingRepresentation,
    IntegerConstant, IntegerKind, IntegerValue, TargetFacts,
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
    bindings.integer_constants.retain(|name, _| {
        !duplicates.contains(name)
            && (objects.contains_key(name) || declarations.integer_constants.contains_key(name))
    });
    bindings
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
            char_bits: 8,
            char_is_signed: true,
            ascii_execution_charset: true,
            byte_order: ByteOrder::Little,
            c_standard: Some(201112),
            integers: [
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
}
