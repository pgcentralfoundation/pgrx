use pgx_utils::{operator_common::*, pg_inventory};
use quote::ToTokens;
use syn::DeriveInput;

pub(crate) fn impl_postgres_eq(ast: DeriveInput) -> proc_macro2::TokenStream {
    let found_skip_inventory = ast.attrs.iter().any(|x| {
        x.path
            .get_ident()
            .map(|x| x.to_string() == "skip_inventory")
            .unwrap_or(false)
    });
    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(eq(&ast.ident, found_skip_inventory));
    stream.extend(ne(&ast.ident, found_skip_inventory));

    stream
}

pub(crate) fn impl_postgres_ord(ast: DeriveInput) -> proc_macro2::TokenStream {
    let found_skip_inventory = ast.attrs.iter().any(|x| {
        x.path
            .get_ident()
            .map(|x| x.to_string() == "skip_inventory")
            .unwrap_or(false)
    });

    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(lt(&ast.ident, found_skip_inventory));
    stream.extend(gt(&ast.ident, found_skip_inventory));
    stream.extend(le(&ast.ident, found_skip_inventory));
    stream.extend(ge(&ast.ident, found_skip_inventory));
    stream.extend(cmp(&ast.ident, found_skip_inventory));

    let inventory_item = pg_inventory::PostgresOrd::new(ast.ident.clone());
    if !found_skip_inventory {
        inventory_item.to_tokens(&mut stream);
    }

    stream
}

pub(crate) fn impl_postgres_hash(ast: DeriveInput) -> proc_macro2::TokenStream {
    let found_skip_inventory = ast.attrs.iter().any(|x| {
        x.path
            .get_ident()
            .map(|x| x.to_string() == "skip_inventory")
            .unwrap_or(false)
    });

    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(hash(&ast.ident, found_skip_inventory));

    let inventory_item = pg_inventory::PostgresHash::new(ast.ident.clone());
    if !found_skip_inventory {
        inventory_item.to_tokens(&mut stream);
    }

    stream
}
