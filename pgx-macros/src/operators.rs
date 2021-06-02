use pgx_utils::operator_common::*;
use syn::DeriveInput;
use crate::inventory;
use quote::ToTokens;

pub(crate) fn impl_postgres_eq(ast: DeriveInput) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(eq(&ast.ident));
    stream.extend(ne(&ast.ident));

    stream
}

pub(crate) fn impl_postgres_ord(ast: DeriveInput) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(lt(&ast.ident));
    stream.extend(gt(&ast.ident));
    stream.extend(le(&ast.ident));
    stream.extend(ge(&ast.ident));
    stream.extend(cmp(&ast.ident));

    inventory::PostgresOrd::new(
        ast.ident.clone()
    ).to_tokens(&mut stream);

    stream
}

pub(crate) fn impl_postgres_hash(ast: DeriveInput) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(hash(&ast.ident));

    inventory::PostgresHash::new(
        ast.ident.clone()
    ).to_tokens(&mut stream);

    stream
}
