use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    ItemMod,
};

/// A parsed `#[pg_schema] mod example {}` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a `pgx::datum::inventory::InventorySchema`.
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::inventory::Schema;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: Schema = parse_quote! {
///     #[pg_schema] mod example {}
/// };
/// let inventory_tokens = parsed.to_token_stream();
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

        Ok(Self { module })
    }
}

use std::hash::{Hash, Hasher};
impl ToTokens for Schema {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.module.attrs;
        let vis = &self.module.vis;
        let mod_token = &self.module.mod_token;
        let ident = &self.module.ident;
        let (_content_brace, content_items) = &self
            .module
            .content
            .as_ref()
            .expect("Can only support `mod {}` right now.");

        // A hack until https://github.com/rust-lang/rust/issues/54725 is fixed.
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        content_items.hash(&mut hasher);
        let postfix = hasher.finish();
        // End of hack

        let mut updated_content = content_items.clone();
        let inventory_fn_name = syn::Ident::new(
            &format!("__pgx_internals_schema_{}_{}", ident, postfix),
            Span::call_site(),
        );
        updated_content.push(syn::parse_quote! {
                #[no_mangle]
                #[link(kind = "static")]
                pub extern "C" fn  #inventory_fn_name() -> pgx::datum::inventory::SqlGraphEntity {
                    let submission = pgx::datum::inventory::InventorySchema {
                        module_path: module_path!(),
                        name: stringify!(#ident),
                        file: file!(),
                        line: line!(),
                    };
                    pgx::datum::inventory::SqlGraphEntity::Schema(submission)
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
