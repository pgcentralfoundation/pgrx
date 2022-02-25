use quote::{quote, ToTokens};
use std::fmt::Display;
use syn::parse::{Parse, ParseStream};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PositioningRef {
    FullPath(String),
    Name(String),
}

impl Display for PositioningRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositioningRef::FullPath(i) => f.write_str(i),
            PositioningRef::Name(i) => f.write_str(i),
        }
    }
}

impl Parse for PositioningRef {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let maybe_litstr: Option<syn::LitStr> = input.parse()?;
        let found = if let Some(litstr) = maybe_litstr {
            Self::Name(litstr.value())
        } else {
            let path: syn::Path = input.parse()?;
            let path_str = path.to_token_stream().to_string().replace(" ", "");
            Self::FullPath(path_str)
        };
        Ok(found)
    }
}

impl ToTokens for PositioningRef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let toks = match self {
            PositioningRef::FullPath(item) => quote! {
                ::pgx::utils::sql_entity_graph::PositioningRef::FullPath(String::from(#item))
            },
            PositioningRef::Name(item) => quote! {
                ::pgx::utils::sql_entity_graph::PositioningRef::Name(String::from(#item))
            },
        };
        toks.to_tokens(tokens);
    }
}
