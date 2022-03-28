/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
pub mod entity;

use crate::sql_entity_graph::ToSqlConfig;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Generics, ItemEnum,
};
use syn::{punctuated::Punctuated, Ident, Token};

/// A parsed `#[derive(PostgresEnum)]` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a `pgx::datum::sql_entity_graph::PostgresEnumEntity`.
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::sql_entity_graph::PostgresEnum;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PostgresEnum = parse_quote! {
///     #[derive(PostgresEnum)]
///     enum Demo {
///         Example,
///     }
/// };
/// let sql_graph_entity_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PostgresEnum {
    name: Ident,
    generics: Generics,
    variants: Punctuated<syn::Variant, Token![,]>,
    to_sql_config: ToSqlConfig,
}

impl PostgresEnum {
    pub fn new(
        name: Ident,
        generics: Generics,
        variants: Punctuated<syn::Variant, Token![,]>,
        to_sql_config: ToSqlConfig,
    ) -> Self {
        Self {
            name,
            generics,
            variants,
            to_sql_config,
        }
    }

    pub fn from_derive_input(derive_input: DeriveInput) -> Result<Self, syn::Error> {
        let to_sql_config =
            ToSqlConfig::from_attributes(derive_input.attrs.as_slice())?.unwrap_or_default();
        let data_enum = match derive_input.data {
            syn::Data::Enum(data_enum) => data_enum,
            syn::Data::Union(_) | syn::Data::Struct(_) => {
                return Err(syn::Error::new(derive_input.ident.span(), "expected enum"))
            }
        };
        Ok(Self::new(
            derive_input.ident,
            derive_input.generics,
            data_enum.variants,
            to_sql_config,
        ))
    }
}

impl Parse for PostgresEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let parsed: ItemEnum = input.parse()?;
        let to_sql_config =
            ToSqlConfig::from_attributes(parsed.attrs.as_slice())?.unwrap_or_default();
        Ok(Self::new(
            parsed.ident,
            parsed.generics,
            parsed.variants,
            to_sql_config,
        ))
    }
}

impl ToTokens for PostgresEnum {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // It's important we remap all lifetimes we spot to `'static` so they can be used during inventory submission.
        let name = self.name.clone();
        let mut static_generics = self.generics.clone();
        for lifetime in static_generics.lifetimes_mut() {
            lifetime.lifetime.ident = Ident::new("static", Span::call_site());
        }
        let (_impl_generics, ty_generics, _where_clauses) = static_generics.split_for_impl();

        let variants = self.variants.iter();
        let sql_graph_entity_fn_name =
            syn::Ident::new(&format!("__pgx_internals_enum_{}", name), Span::call_site());

        let to_sql_config = &self.to_sql_config;

        let inv = quote! {
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn  #sql_graph_entity_fn_name() -> ::pgx::utils::sql_entity_graph::SqlGraphEntity {
                extern crate alloc;
                use alloc::vec::Vec;
                use alloc::vec;
                let mut mappings = Default::default();
                <#name #ty_generics as ::pgx::datum::WithTypeIds>::register_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithSizedTypeIds::<#name #ty_generics>::register_sized_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithArrayTypeIds::<#name #ty_generics>::register_array_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithVarlenaTypeIds::<#name #ty_generics>::register_varlena_with_refs(&mut mappings, stringify!(#name).to_string());

                let submission = ::pgx::utils::sql_entity_graph::PostgresEnumEntity {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name #ty_generics>(),
                    mappings,
                    variants: vec![ #(  stringify!(#variants)  ),* ],
                    to_sql_config: #to_sql_config,
                };
                ::pgx::utils::sql_entity_graph::SqlGraphEntity::Enum(submission)
            }
        };
        tokens.append_all(inv);
    }
}
