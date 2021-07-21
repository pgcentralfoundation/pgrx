use proc_macro2::{Span, Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use std::hash::{Hasher, Hash};
use syn::Generics;

#[derive(Debug, Clone)]
pub struct PostgresType {
    name: Ident,
    generics: Generics,
    in_fn: Ident,
    out_fn: Ident,
}

impl PostgresType {
    pub fn new(name: Ident, generics: Generics, in_fn: Ident, out_fn: Ident) -> Self {
        Self {
            generics,
            name,
            in_fn,
            out_fn,
        }
    }
}

impl ToTokens for PostgresType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let mut static_generics = self.generics.clone();
        for lifetime in static_generics.lifetimes_mut() {
            lifetime.lifetime.ident = Ident::new("static", Span::call_site());
        }
        let (_impl_generics, ty_generics, _where_clauses) = static_generics.split_for_impl();

        let in_fn = &self.in_fn;
        let out_fn = &self.out_fn;
        let inv = quote! {
            pgx_utils::pg_inventory::inventory::submit! {
                let mut mappings = std::collections::HashMap::default();
                <#name #ty_generics as ::pgx::datum::WithTypeIds>::register_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithSizedTypeIds::<#name #ty_generics>::register_sized_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithArrayTypeIds::<#name #ty_generics>::register_array_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithVarlenaTypeIds::<#name #ty_generics>::register_varlena_with_refs(&mut mappings, stringify!(#name).to_string());

                crate::__pgx_internals::PostgresType(pgx_utils::pg_inventory::InventoryPostgresType {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name #ty_generics>(),
                    id: *<#name  #ty_generics as WithTypeIds>::ITEM_ID,
                    mappings,
                    in_fn: stringify!(#in_fn),
                    in_fn_module_path: {
                        let in_fn = stringify!(#in_fn);
                        let mut path_items: Vec<_> = in_fn.split("::").collect();
                        let _ = path_items.pop(); // Drop the one we don't want.
                        path_items.join("::")
                    },
                    out_fn: stringify!(#out_fn),
                    out_fn_module_path: {
                        let out_fn = stringify!(#out_fn);
                        let mut path_items: Vec<_> = out_fn.split("::").collect();
                        let _ = path_items.pop(); // Drop the one we don't want.
                        path_items.join("::")
                    }
                })
            }
        };
        tokens.append_all(inv);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryPostgresType {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
    pub mappings: std::collections::HashMap<core::any::TypeId, super::RustSqlMapping>,
    pub in_fn: &'static str,
    pub in_fn_module_path: String,
    pub out_fn: &'static str,
    pub out_fn_module_path: String,
}


impl Hash for InventoryPostgresType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for InventoryPostgresType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for InventoryPostgresType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl InventoryPostgresType {
    pub fn id_matches(&self, candidate: &core::any::TypeId) -> bool {
        self.mappings.iter().any(|(tester, _)| *candidate == *tester)
    }
}