mod extension_sql;
mod inventory_positioning_ref;
mod pg_extern;
mod pg_schema;
mod postgres_enum;
mod postgres_hash;
mod postgres_ord;
mod postgres_type;

pub use super::ExternArgs;
pub use extension_sql::{ExtensionSql, ExtensionSqlFile, SqlDeclaredEntity};
pub use inventory_positioning_ref::InventoryPositioningRef;
pub use pg_extern::PgExtern;
pub use pg_schema::Schema;
pub use postgres_enum::PostgresEnum;
pub use postgres_hash::PostgresHash;
pub use postgres_ord::PostgresOrd;
pub use postgres_type::PostgresType;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};

/// Reexports for the pgx SQL generator binaries.
#[doc(hidden)]
pub mod reexports {
    #[doc(hidden)]
    pub use clap;
    #[doc(hidden)]
    pub use color_eyre;
    #[doc(hidden)]
    pub use eyre;
    #[doc(hidden)]
    pub use inventory;
    #[doc(hidden)]
    pub use libloading;
    #[doc(hidden)]
    pub use once_cell;
    #[doc(hidden)]
    pub use tracing;
    #[doc(hidden)]
    pub use tracing_error;
    #[doc(hidden)]
    pub use tracing_subscriber;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PositioningRef {
    Expr(syn::Expr),
    Name(syn::LitStr),
}

impl Parse for PositioningRef {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let maybe_litstr: Option<syn::LitStr> = input.parse()?;
        let found = if let Some(litstr) = maybe_litstr {
            Self::Name(litstr)
        } else {
            let path: syn::Expr = input.parse()?;
            Self::Expr(path)
        };
        Ok(found)
    }
}

impl ToTokens for PositioningRef {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let toks = match self {
            PositioningRef::Expr(ex) => {
                let path = ex.to_token_stream().to_string().replace(" ", "");
                (quote! {
                    pgx::inventory::InventoryPositioningRef::FullPath(String::from(#path))
                })
                .to_token_stream()
            }
            PositioningRef::Name(name) => quote! {
                pgx::inventory::InventoryPositioningRef::Name(String::from(#name))
            },
        };
        tokens.append_all(toks);
    }
}
