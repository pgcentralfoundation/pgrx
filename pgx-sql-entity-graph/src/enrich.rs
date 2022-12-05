use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};

pub struct CodeEnrichment<T>(pub T);

/// Generates the rust code that pgx requires for one of its SQL interfaces such as `#[pg_extern]`
pub trait ToRustCodeTokens {
    fn to_rust_code_tokens(&self) -> TokenStream2 {
        quote! {}
    }
}

/// Generates the rust code to tie one of pgx' supported SQL interfaces into pgx' schema generator
pub trait ToEntityGraphTokens {
    fn to_entity_graph_tokens(&self) -> TokenStream2 {
        quote! {}
    }
}

impl<T> ToTokens for CodeEnrichment<T>
where
    T: ToEntityGraphTokens + ToRustCodeTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        #[cfg(not(feature = "no-schema-generation"))]
        {
            // only emit entity graph tokens when we're generating a schema, which is our default mode
            tokens.append_all(self.0.to_entity_graph_tokens());
        }

        tokens.append_all(self.0.to_rust_code_tokens());
    }
}
