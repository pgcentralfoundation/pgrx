/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[pg_extern]` related attributes for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::sql_entity_graph::positioning_ref::PositioningRef;
use crate::sql_entity_graph::to_sql::ToSqlConfig;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Attribute {
    Immutable,
    Strict,
    Stable,
    Volatile,
    Raw,
    NoGuard,
    CreateOrReplace,
    ParallelSafe,
    ParallelUnsafe,
    ParallelRestricted,
    Error(syn::LitStr),
    Schema(syn::LitStr),
    Name(syn::LitStr),
    Cost(syn::Expr),
    Requires(Punctuated<PositioningRef, Token![,]>),
    Sql(ToSqlConfig),
}

impl Attribute {
    pub(crate) fn to_sql_entity_graph_tokens(&self) -> TokenStream2 {
        match self {
            Attribute::Immutable => quote! { ::pgx::utils::ExternArgs::Immutable },
            Attribute::Strict => quote! { ::pgx::utils::ExternArgs::Strict },
            Attribute::Stable => quote! { ::pgx::utils::ExternArgs::Stable },
            Attribute::Volatile => quote! { ::pgx::utils::ExternArgs::Volatile },
            Attribute::Raw => quote! { ::pgx::utils::ExternArgs::Raw },
            Attribute::NoGuard => quote! { ::pgx::utils::ExternArgs::NoGuard },
            Attribute::CreateOrReplace => quote! { ::pgx::utils::ExternArgs::CreateOrReplace },
            Attribute::ParallelSafe => {
                quote! { ::pgx::utils::ExternArgs::ParallelSafe }
            }
            Attribute::ParallelUnsafe => {
                quote! { ::pgx::utils::ExternArgs::ParallelUnsafe }
            }
            Attribute::ParallelRestricted => {
                quote! { ::pgx::utils::ExternArgs::ParallelRestricted }
            }
            Attribute::Error(s) => {
                quote! { ::pgx::utils::ExternArgs::Error(String::from(#s)) }
            }
            Attribute::Schema(s) => {
                quote! { ::pgx::utils::ExternArgs::Schema(String::from(#s)) }
            }
            Attribute::Name(s) => {
                quote! { ::pgx::utils::ExternArgs::Name(String::from(#s)) }
            }
            Attribute::Cost(s) => {
                quote! { ::pgx::utils::ExternArgs::Cost(format!("{}", #s)) }
            }
            Attribute::Requires(items) => {
                let items_iter = items.iter().map(|x| x.to_token_stream()).collect::<Vec<_>>();
                quote! { ::pgx::utils::ExternArgs::Requires(vec![#(#items_iter),*],) }
            }
            // This attribute is handled separately
            Attribute::Sql(_) => {
                quote! {}
            }
        }
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = match self {
            Attribute::Immutable => quote! { immutable },
            Attribute::Strict => quote! { strict },
            Attribute::Stable => quote! { stable },
            Attribute::Volatile => quote! { volatile },
            Attribute::Raw => quote! { raw },
            Attribute::NoGuard => quote! { no_guard },
            Attribute::CreateOrReplace => quote! { create_or_replace },
            Attribute::ParallelSafe => {
                quote! { parallel_safe }
            }
            Attribute::ParallelUnsafe => {
                quote! { parallel_unsafe }
            }
            Attribute::ParallelRestricted => {
                quote! { parallel_restricted }
            }
            Attribute::Error(s) => {
                quote! { error = #s }
            }
            Attribute::Schema(s) => {
                quote! { schema = #s }
            }
            Attribute::Name(s) => {
                quote! { name = #s }
            }
            Attribute::Cost(s) => {
                quote! { cost = #s }
            }
            Attribute::Requires(items) => {
                let items_iter = items.iter().map(|x| x.to_token_stream()).collect::<Vec<_>>();
                quote! { requires = [#(#items_iter),*] }
            }
            // This attribute is handled separately
            Attribute::Sql(to_sql_config) => {
                quote! { sql = #to_sql_config }
            }
        };
        tokens.append_all(quoted);
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: syn::Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
            "immutable" => Self::Immutable,
            "strict" => Self::Strict,
            "stable" => Self::Stable,
            "volatile" => Self::Volatile,
            "raw" => Self::Raw,
            "no_guard" => Self::NoGuard,
            "create_or_replace" => Self::CreateOrReplace,
            "parallel_safe" => Self::ParallelSafe,
            "parallel_unsafe" => Self::ParallelUnsafe,
            "parallel_restricted" => Self::ParallelRestricted,
            "error" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::LitStr = input.parse()?;
                Self::Error(literal)
            }
            "schema" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::LitStr = input.parse()?;
                Attribute::Schema(literal)
            }
            "name" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::LitStr = input.parse()?;
                Self::Name(literal)
            }
            "cost" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::Expr = input.parse()?;
                Self::Cost(literal)
            }
            "requires" => {
                let _eq: syn::token::Eq = input.parse()?;
                let content;
                let _bracket = syn::bracketed!(content in input);
                Self::Requires(content.parse_terminated(PositioningRef::parse)?)
            }
            "sql" => {
                use crate::sql_entity_graph::pgx_attribute::ArgValue;
                use syn::Lit;

                let _eq: Token![=] = input.parse()?;
                match input.parse::<ArgValue>()? {
                    ArgValue::Path(p) => Self::Sql(ToSqlConfig::from(p)),
                    ArgValue::Lit(Lit::Bool(b)) => Self::Sql(ToSqlConfig::from(b.value)),
                    ArgValue::Lit(Lit::Str(s)) => Self::Sql(ToSqlConfig::from(s)),
                    ArgValue::Lit(other) => {
                        return Err(syn::Error::new(
                            other.span(),
                            "expected boolean, path, or string literal",
                        ))
                    }
                }
            }
            e => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    format!(
                        "Invalid option `{}` inside `{} {}`",
                        e,
                        ident.to_string(),
                        input.to_string()
                    ),
                ))
            }
        };
        Ok(found)
    }
}
