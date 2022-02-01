use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Ident, ItemEnum, ItemStruct,
};

/// A parsed `#[derive(PostgresHash)]` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a `pgx::datum::sql_entity_graph::InventoryPostgresHash`.
///
/// On structs:
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::sql_entity_graph::PostgresHash;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PostgresHash = parse_quote! {
///     #[derive(PostgresHash)]
///     struct Example<'a> {
///         demo: &'a str,
///     }
/// };
/// let sql_graph_entity_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
///
/// On enums:
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::sql_entity_graph::PostgresHash;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PostgresHash = parse_quote! {
///     #[derive(PostgresHash)]
///     enum Demo {
///         Example,
///     }
/// };
/// let sql_graph_entity_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PostgresHash {
    pub name: Ident,
}

impl PostgresHash {
    pub fn new(name: Ident) -> Self {
        Self { name }
    }

    pub fn from_derive_input(derive_input: DeriveInput) -> Result<Self, syn::Error> {
        Ok(Self::new(derive_input.ident))
    }
}

impl Parse for PostgresHash {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let parsed_enum: Result<ItemEnum, syn::Error> = input.parse();
        let parsed_struct: Result<ItemStruct, syn::Error> = input.parse();
        let ident = parsed_enum
            .map(|x| x.ident)
            .or_else(|_| parsed_struct.map(|x| x.ident))
            .map_err(|_| syn::Error::new(input.span(), "expected enum or struct"))?;
        Ok(Self::new(ident))
    }
}

impl ToTokens for PostgresHash {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let sql_graph_entity_fn_name = syn::Ident::new(
            &format!("__pgx_internals_hash_{}", self.name),
            Span::call_site(),
        );
        let inv = quote! {
            #[no_mangle]
            pub extern "C" fn  #sql_graph_entity_fn_name() -> pgx::datum::sql_entity_graph::SqlGraphEntity {
                use core::any::TypeId;
                extern crate alloc;
                use alloc::vec::Vec;
                use alloc::vec;
                let submission = pgx::datum::sql_entity_graph::PostgresHashEntity {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    full_path: core::any::type_name::<#name>(),
                    module_path: module_path!(),
                    id: TypeId::of::<#name>(),
                };
                pgx::datum::sql_entity_graph::SqlGraphEntity::Hash(submission)
            }
        };
        tokens.append_all(inv);
    }
}
