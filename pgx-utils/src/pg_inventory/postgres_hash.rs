use std::{io::Write, fs::{create_dir_all, File}};

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
/// Using [`quote::ToTokens`] will output the declaration for a [`InventoryPostgresHash`].
///
/// On structs:
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::pg_inventory::PostgresHash;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PostgresHash = parse_quote! {
///     #[derive(PostgresHash)]
///     struct Example<'a> {
///         demo: &'a str,
///     }
/// };
/// let inventory_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
///
/// On enums:
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::pg_inventory::PostgresHash;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PostgresHash = parse_quote! {
///     #[derive(PostgresHash)]
///     enum Demo {
///         Example,
///     }
/// };
/// let inventory_tokens = parsed.to_token_stream();
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
    
    pub fn inventory_fn_name(&self) -> String {
        "__inventory_hash_".to_string() + &self.name.to_string()
    }

    pub fn inventory(&self, inventory_dir: String) {
        create_dir_all(&inventory_dir).expect("Couldn't create inventory dir.");
        let mut fd = File::create(inventory_dir.to_string() + "/" + &self.inventory_fn_name() + ".json").expect("Couldn't create inventory file");
        let inventory_fn_json = serde_json::to_string(&self.inventory_fn_name()).expect("Could not serialize inventory item.");
        write!(fd, "{}", inventory_fn_json).expect("Couldn't write to inventory file");
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
        let inventory_fn_name = syn::Ident::new(
            &format!("__pgx_internals_hash_{}", self.name),
            Span::call_site(),
        );
        let inv = quote! {
            #[no_mangle]
            #[link(kind = "static")]
            pub extern "C" fn  #inventory_fn_name() -> pgx::datum::inventory::SqlGraphEntity {
                use core::any::TypeId;
                let submission = pgx::pg_inventory::InventoryPostgresHash {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    full_path: core::any::type_name::<#name>(),
                    module_path: module_path!(),
                    id: TypeId::of::<#name>(),
                };
                pgx::datum::inventory::SqlGraphEntity::Hash(submission)
            }
        };
        tokens.append_all(inv);
    }
}
