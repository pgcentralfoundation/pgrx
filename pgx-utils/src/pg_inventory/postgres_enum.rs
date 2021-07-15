use std::hash::{Hasher, Hash};

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{punctuated::Punctuated, Ident, Token};

#[derive(Debug, Clone)]
pub struct PostgresEnum {
    pub name: Ident,
    pub variants: Punctuated<syn::Variant, Token![,]>,
}

impl PostgresEnum {
    pub fn new(name: Ident, variants: Punctuated<syn::Variant, Token![,]>) -> Self {
        Self { name, variants }
    }
}

impl ToTokens for PostgresEnum {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let variants = self.variants.iter();
        let inv = quote! {
            pgx_utils::pg_inventory::inventory::submit! {
                let mut mappings = std::collections::HashMap::default();
                <#name as ::pgx::datum::WithTypeIds>::register_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithSizedTypeIds::<#name>::register_sized_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithArrayTypeIds::<#name>::register_array_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithVarlenaTypeIds::<#name>::register_varlena_with_refs(&mut mappings, stringify!(#name).to_string());

                crate::__pgx_internals::PostgresEnum(pgx_utils::pg_inventory::InventoryPostgresEnum {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name>(),
                    id: *<#name as WithTypeIds>::ITEM_ID,
                    mappings,
                    variants: vec![ #(  stringify!(#variants)  ),* ],
                })
            }
        };
        tokens.append_all(inv);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryPostgresEnum {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
    pub mappings: std::collections::HashMap<core::any::TypeId, super::RustSqlMapping>,
    pub variants: Vec<&'static str>,
}

impl Hash for InventoryPostgresEnum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for InventoryPostgresEnum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for InventoryPostgresEnum {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl InventoryPostgresEnum {
    pub fn id_matches(&self, candidate: &core::any::TypeId) -> bool {
        self.mappings.iter().any(|(tester, _)| *candidate == *tester)
    }
}
