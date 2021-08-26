use pgx_utils::{operator_common::*, inventory};
use quote::ToTokens;
use syn::DeriveInput;

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

    let inventory_item = inventory::PostgresOrd::new(ast.ident.clone());
    inventory_item.to_tokens(&mut stream);

    stream
}

pub(crate) fn impl_postgres_hash(ast: DeriveInput) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(hash(&ast.ident));

    let inventory_item = inventory::PostgresHash::new(ast.ident.clone());
    inventory_item.to_tokens(&mut stream);

    stream
}
