use quote::{quote, ToTokens};
use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InventoryPositioningRef {
    FullPath(String),
    Name(String),
}

impl Display for InventoryPositioningRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryPositioningRef::FullPath(i) => f.write_str(i),
            InventoryPositioningRef::Name(i) => f.write_str(i),
        }
    }
}

impl ToTokens for InventoryPositioningRef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let toks = match self {
            InventoryPositioningRef::FullPath(item) => quote! {
                pgx::inventory::InventoryPositioningRef::FullPath(String::from(#item))
            },
            InventoryPositioningRef::Name(item) => quote! {
                pgx::inventory::InventoryPositioningRef::Name(String::from(#item))
            },
        };
        toks.to_tokens(tokens);
    }
}
