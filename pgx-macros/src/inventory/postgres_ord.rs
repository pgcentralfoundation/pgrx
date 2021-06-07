use proc_macro2::{TokenStream as TokenStream2};
use syn::Ident;
use quote::{quote, ToTokens, TokenStreamExt};

pub struct PostgresOrd {
    pub name: Ident,
}

impl PostgresOrd {
    pub(crate) fn new(name: Ident) -> Self {
        Self {
            name,
        }
    }
}

impl ToTokens for PostgresOrd {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let inv = quote! {
            pgx::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PgxPostgresOrd {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    full_path: core::any::type_name::<#name>(),
                    id: TypeId::of::<#name>(),
                }
            }
        };
        tokens.append_all(inv);
    }
}