/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
pub mod entity;

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use std::{
    fs::{create_dir_all, File},
    io::Write,
};
use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Generics, ItemStruct,
};

use crate::sql_entity_graph::ToSqlConfig;

/// A parsed `#[derive(PostgresType)]` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a [`PostgresTypeEntity`][crate::sql_entity_graph::PostgresTypeEntity].
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::sql_entity_graph::PostgresType;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PostgresType = parse_quote! {
///     #[derive(PostgresType)]
///     struct Example<'a> {
///         demo: &'a str,
///     }
/// };
/// let sql_graph_entity_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PostgresType {
    name: Ident,
    generics: Generics,
    in_fn: Ident,
    out_fn: Ident,
    to_sql_config: ToSqlConfig,
}

impl PostgresType {
    pub fn new(
        name: Ident,
        generics: Generics,
        in_fn: Ident,
        out_fn: Ident,
        to_sql_config: ToSqlConfig,
    ) -> Self {
        Self {
            generics,
            name,
            in_fn,
            out_fn,
            to_sql_config,
        }
    }

    pub fn from_derive_input(derive_input: DeriveInput) -> Result<Self, syn::Error> {
        let _data_struct = match derive_input.data {
            syn::Data::Struct(data_struct) => data_struct,
            syn::Data::Union(_) | syn::Data::Enum(_) => {
                return Err(syn::Error::new(
                    derive_input.ident.span(),
                    "expected struct",
                ))
            }
        };
        let to_sql_config =
            ToSqlConfig::from_attributes(derive_input.attrs.as_slice())?.unwrap_or_default();
        let funcname_in = Ident::new(
            &format!("{}_in", derive_input.ident).to_lowercase(),
            derive_input.ident.span(),
        );
        let funcname_out = Ident::new(
            &format!("{}_out", derive_input.ident).to_lowercase(),
            derive_input.ident.span(),
        );
        Ok(Self::new(
            derive_input.ident,
            derive_input.generics,
            funcname_in,
            funcname_out,
            to_sql_config,
        ))
    }

    pub fn inventory_fn_name(&self) -> String {
        "__inventory_type_".to_string() + &self.name.to_string()
    }

    pub fn inventory(&self, inventory_dir: String) {
        create_dir_all(&inventory_dir).expect("Couldn't create inventory dir.");
        let mut fd =
            File::create(inventory_dir.to_string() + "/" + &self.inventory_fn_name() + ".json")
                .expect("Couldn't create inventory file");
        let sql_graph_entity_fn_json = serde_json::to_string(&self.inventory_fn_name())
            .expect("Could not serialize inventory item.");
        write!(fd, "{}", sql_graph_entity_fn_json).expect("Couldn't write to inventory file");
    }
}

impl Parse for PostgresType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let parsed: ItemStruct = input.parse()?;
        let to_sql_config =
            ToSqlConfig::from_attributes(parsed.attrs.as_slice())?.unwrap_or_default();
        let funcname_in = Ident::new(
            &format!("{}_in", parsed.ident).to_lowercase(),
            parsed.ident.span(),
        );
        let funcname_out = Ident::new(
            &format!("{}_out", parsed.ident).to_lowercase(),
            parsed.ident.span(),
        );
        Ok(Self::new(
            parsed.ident,
            parsed.generics,
            funcname_in,
            funcname_out,
            to_sql_config,
        ))
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

        let sql_graph_entity_fn_name = syn::Ident::new(
            &format!("__pgx_internals_type_{}", self.name),
            Span::call_site(),
        );

        let to_sql_config = &self.to_sql_config;

        let inv = quote! {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn  #sql_graph_entity_fn_name() -> ::pgx::utils::sql_entity_graph::SqlGraphEntity {
                extern crate alloc;
                use alloc::vec::Vec;
                use alloc::vec;
                use alloc::string::{String, ToString};

                let mut mappings = Default::default();
                <#name #ty_generics as pgx::datum::WithTypeIds>::register_with_refs(
                    &mut mappings,
                    stringify!(#name).to_string()
                );
                pgx::datum::WithSizedTypeIds::<#name #ty_generics>::register_sized_with_refs(
                    &mut mappings,
                    stringify!(#name).to_string()
                );
                pgx::datum::WithArrayTypeIds::<#name #ty_generics>::register_array_with_refs(
                    &mut mappings,
                    stringify!(#name).to_string()
                );
                pgx::datum::WithVarlenaTypeIds::<#name #ty_generics>::register_varlena_with_refs(
                    &mut mappings,
                    stringify!(#name).to_string()
                );
                let submission = ::pgx::utils::sql_entity_graph::PostgresTypeEntity {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name #ty_generics>(),
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
                    },
                    to_sql_config: #to_sql_config,
                };
                ::pgx::utils::sql_entity_graph::SqlGraphEntity::Type(submission)
            }
        };
        tokens.append_all(inv);
    }
}
