use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};

pub struct PostgresType {
    name: Ident,
    in_fn: Ident,
    out_fn: Ident,
}

impl PostgresType {
    pub fn new(name: Ident, in_fn: Ident, out_fn: Ident) -> Self {
        Self {
            name,
            in_fn,
            out_fn,
        }
    }
}

impl ToTokens for PostgresType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let in_fn = &self.in_fn;
        let out_fn = &self.out_fn;
        let inv = quote! {
            pgx::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PostgresType(pgx_utils::pg_inventory::InventoryPostgresType {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name>(),
                    id: TypeId::of::<#name>(),
                    option_id: TypeId::of::<Option<#name>>(),
                    vec_id: TypeId::of::<Vec<#name>>(),
                    in_fn: stringify!(#in_fn),
                    out_fn: stringify!(#out_fn),
                })
            }
        };
        tokens.append_all(inv);
    }
}

#[derive(Debug)]
pub struct InventoryPostgresType {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
    pub option_id: core::any::TypeId,
    pub vec_id: core::any::TypeId,
    pub in_fn: &'static str,
    pub out_fn: &'static str,
}