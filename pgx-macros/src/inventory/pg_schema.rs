use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::{Parse, ParseStream}, punctuated::Punctuated, Ident, Token, ItemMod};

#[derive(Debug)]
pub struct PgxSchema {
    pub module: ItemMod,

}

impl Parse for PgxSchema {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            module: input.parse()?,
        })
    }
}


impl ToTokens for PgxSchema {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.module.attrs;
        let vis = &self.module.vis;
        let mod_token = &self.module.mod_token;
        let ident = &self.module.ident;
        let (_content_brace, content_items) = &self.module.content.as_ref().expect("Can only support `mod {}` right now.");

        let mut updated_content = content_items.clone();
        updated_content.push(syn::parse_quote! {
            struct __marker;
        });
        updated_content.push(syn::parse_quote! {
            pgx::inventory::submit! {
                crate::__pgx_internals::PgxSchema {
                    module_path: core::any::type_name::<__marker>(),
                    name: stringify!(#ident),
                    file: file!(),
                    line: line!(),
                }
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
