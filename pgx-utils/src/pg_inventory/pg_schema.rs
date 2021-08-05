use super::{DotIdentifier, SqlGraphEntity, ToSql};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    ItemMod,
};

/// A parsed `#[pg_schema] mod example {}` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a [`InventorySchema`].
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::pg_inventory::Schema;
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
        Ok(Self {
            module: input.parse()?,
        })
    }
}

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
        let found_skip_inventory = self.module.attrs.iter().any(|x| {
            x.path
                .get_ident()
                .map(|x| x.to_string() == "skip_inventory")
                .unwrap_or(false)
        });

        let mut updated_content = content_items.clone();
        if !found_skip_inventory {
            updated_content.push(syn::parse_quote! {
                use pgx_utils::pg_inventory::inventory;
            });
            updated_content.push(syn::parse_quote! {
                pgx_utils::pg_inventory::inventory::submit! {
                    crate::__pgx_internals::Schema(pgx_utils::pg_inventory::InventorySchema {
                        module_path: module_path!(),
                        name: stringify!(#ident),
                        file: file!(),
                        line: line!(),
                    })
                }
            });
        }
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InventorySchema {
    pub module_path: &'static str,
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
}

impl<'a> Into<SqlGraphEntity<'a>> for &'a InventorySchema {
    fn into(self) -> SqlGraphEntity<'a> {
        SqlGraphEntity::Schema(self)
    }
}

impl DotIdentifier for InventorySchema {
    fn dot_identifier(&self) -> String {
        format!("schema {}", self.module_path.to_string())
    }
}

impl ToSql for InventorySchema {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = self.module_path))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!(
            "\n\
                    -- {file}:{line}\n\
                    CREATE SCHEMA IF NOT EXISTS {name}; /* {module_path} */\
                ",
            name = self.name,
            file = self.file,
            line = self.line,
            module_path = self.module_path,
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}
