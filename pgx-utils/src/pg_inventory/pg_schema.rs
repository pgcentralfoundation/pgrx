use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    ItemMod,
};
use std::{
    io::Write,
    fs::{create_dir_all, File}
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

impl Schema {
    pub fn inventory_fn_name(&self) -> String {
        "__inventory_schema_".to_string() + &self.module.ident.to_string()
    }

    pub fn inventory(&self, inventory_dir: String) {
        create_dir_all(&inventory_dir).expect("Couldn't create inventory dir.");
        let mut fd = File::create(inventory_dir.to_string() + "/" + &self.inventory_fn_name() + ".json").expect("Couldn't create inventory file");
        let inventory_fn_json = serde_json::to_string(&self.inventory_fn_name()).expect("Could not serialize inventory item.");
        write!(fd, "{}", inventory_fn_json).expect("Couldn't write to inventory file");
    }
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
            let inventory_fn_name = syn::Ident::new(
                &format!("__pgx_internals_schema_{}", ident),
                Span::call_site(),
            );
            updated_content.push(syn::parse_quote! {
                    #[no_mangle]
                    #[link(kind = "static")]
                    pub extern "C" fn  #inventory_fn_name() -> pgx::datum::inventory::SqlGraphEntity {
                        let submission = pgx::pg_inventory::InventorySchema {
                            module_path: module_path!(),
                            name: stringify!(#ident),
                            file: file!(),
                            line: line!(),
                        };
                        pgx::datum::inventory::SqlGraphEntity::Schema(submission)
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
