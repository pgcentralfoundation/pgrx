/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[pg_schema]` related macro expansion for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
pub mod entity;

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use std::hash::{Hash, Hasher};
use syn::parse::{Parse, ParseStream};
use syn::ItemMod;

/// A parsed `#[pg_schema] mod example {}` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a `pgx::datum::pgx_sql_entity_graph::InventorySchema`.
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_sql_entity_graph::Schema;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: Schema = parse_quote! {
///     #[pg_schema] mod example {}
/// };
/// let entity_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Schema {
    pub module: ItemMod,
}

impl Parse for Schema {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let module: ItemMod = input.parse()?;
        crate::ident_is_acceptable_to_postgres(&module.ident)?;
        Ok(Self { module })
    }
}

impl ToTokens for Schema {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.module.attrs;
        let vis = &self.module.vis;
        let mod_token = &self.module.mod_token;
        let ident = &self.module.ident;

        let (_content_brace, content_items) =
            &self.module.content.as_ref().expect("Can only support `mod {}` right now.");

        // A hack until https://github.com/rust-lang/rust/issues/54725 is fixed.
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        content_items.hash(&mut hasher);
        let postfix = hasher.finish();
        // End of hack

        let mut updated_content = content_items.clone();
        let sql_graph_entity_fn_name = syn::Ident::new(
            &format!("__pgx_internals_schema_{}_{}", ident, postfix),
            Span::call_site(),
        );
        updated_content.push(syn::parse_quote! {
                #[no_mangle]
                #[doc(hidden)]
                pub extern "Rust" fn  #sql_graph_entity_fn_name() -> pgx::pgx_sql_entity_graph::SqlGraphEntity {
                    extern crate alloc;
                    use alloc::vec::Vec;
                    use alloc::vec;
                    let submission = pgx::pgx_sql_entity_graph::SchemaEntity {
                            module_path: module_path!(),
                            name: stringify!(#ident),
                            file: file!(),
                            line: line!(),
                        };
                    pgx::pgx_sql_entity_graph::SqlGraphEntity::Schema(submission)
                }
        });
        let _semi = &self.module.semi;

        let inv = quote! {
            #(#attrs)*
            #vis #mod_token #ident {
                #(#updated_content)*
            }
        };
        tokens.append_all(inv);
    }
}
