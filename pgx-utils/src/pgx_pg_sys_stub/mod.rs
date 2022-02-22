use eyre::WrapErr;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use syn::{parse_quote, ForeignItem, ForeignItemFn, ForeignItemStatic, Item};

const ADDITIONAL_FUNCTIONS_TO_STUB: [&'static str; 6] = [
    "varsize_any",
    "query_tree_walker",
    "expression_tree_walker",
    "sigsetjmp",
    "siglongjmp",
    "pg_re_throw",
];

/// A utility structure which can generate 'stubs' of the bindings generated by `pgx-pg-sys`'s build script.
///
/// These stubs can be built.
///
/// For example, this is used by `cargo-pgx` and then `dlopen`'d before the extension for SQL generation.
pub struct PgxPgSysStub {
    rewritten: syn::File,
}

impl PgxPgSysStub {
    #[tracing::instrument(level = "error", skip_all, fields(pgx_pg_sys_path = %pgx_pg_sys_path.as_ref().display()))]
    pub fn from_file(pgx_pg_sys_path: impl AsRef<Path>) -> eyre::Result<Self> {
        let pgx_pg_sys_path = pgx_pg_sys_path.as_ref();
        let mut pgx_pg_sys_file = File::open(pgx_pg_sys_path)
            .wrap_err("Could not open `pgx_pg_sys` generated bindings.")?;
        let mut buf = String::default();
        pgx_pg_sys_file
            .read_to_string(&mut buf)
            .wrap_err("Could not read `pgx_pg_sys` generated bindings.")?;

        Self::from_str(&buf)
    }

    #[tracing::instrument(level = "error", skip_all)]
    pub fn from_str(buf: impl AsRef<str>) -> eyre::Result<Self> {
        let mut working_state = syn::parse_file(buf.as_ref())
            .wrap_err("Could not parse `pgx_pg_sys` generated bindings.")?;

        let mut items_with_stubs = Vec::default();
        for mut item in working_state.items.drain(..) {
            match item {
                Item::ForeignMod(ref mut item_foreign_mod) => {
                    let mut items_without_foreign_fns = Vec::default();
                    for inner_item in item_foreign_mod.items.iter_mut() {
                        match inner_item {
                            ForeignItem::Fn(foreign_item_fn) => {
                                let stub = stub_for_fn(&foreign_item_fn);
                                items_with_stubs.push(stub);
                            }
                            ForeignItem::Static(foreign_item_static) => {
                                let stub = stub_for_static(&foreign_item_static);
                                items_with_stubs.push(stub);
                            }
                            _ => items_without_foreign_fns.push(inner_item.clone()),
                        }
                    }
                    item_foreign_mod.items = items_without_foreign_fns;
                }
                _ => (),
            }
            // items_with_stubs.push(item);
        }

        for additional_function in ADDITIONAL_FUNCTIONS_TO_STUB {
            let additional_ident =
                syn::Ident::new(additional_function, proc_macro2::Span::call_site());
            items_with_stubs.push(parse_quote! {
                #[no_mangle]
                pub extern "C" fn #additional_ident() {
                    unimplemented!(concat!(stringify!(#additional_ident), " is stubbed and cannot be used"));
                }
            });
        }

        working_state.items = items_with_stubs;

        Ok(Self {
            rewritten: working_state,
        })
    }

    #[tracing::instrument(level = "error", skip_all, fields(pgx_pg_sys_stub = %pgx_pg_sys_stub.as_ref().display()))]
    pub fn write_to_file(&self, pgx_pg_sys_stub: impl AsRef<Path>) -> eyre::Result<()> {
        let pgx_pg_sys_stub = pgx_pg_sys_stub.as_ref();
        if let Some(parent) = pgx_pg_sys_stub.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut output_file = File::create(pgx_pg_sys_stub)?;
        let content = prettyplease::unparse(&self.rewritten);
        output_file.write_all(content.as_bytes())?;
        Ok(())
    }
}

fn stub_for_fn(foreign_item_fn: &ForeignItemFn) -> Item {
    let mut attrs = foreign_item_fn
        .attrs
        .iter()
        .cloned()
        .filter(|attr| {
            if let Some(ident) = attr.path.get_ident() {
                ident.to_string() != "pg_guard" && ident.to_string() != "link_name"
            } else {
                true
            }
        })
        .collect::<Vec<_>>();
    attrs.push(parse_quote! { #[no_mangle] });
    let vis = &foreign_item_fn.vis;
    let mut sig = foreign_item_fn.sig.clone();
    sig.inputs = syn::punctuated::Punctuated::<syn::FnArg, syn::Token![,]>::new();
    sig.output = syn::ReturnType::Default;
    sig.abi = Some(parse_quote! { extern "C" });
    sig.variadic = None;
    let fn_ident = &sig.ident;
    parse_quote! {
        #(#attrs)* #vis #sig {
            unimplemented!(concat!(stringify!(#fn_ident), " is stubbed and cannot be used"));
        }
    }
}

fn stub_for_static(foreign_item_static: &ForeignItemStatic) -> Item {
    let mut attrs = foreign_item_static
        .attrs
        .iter()
        .cloned()
        .filter(|attr| {
            if let Some(ident) = attr.path.get_ident() {
                ident.to_string() != "pg_guard" && ident.to_string() != "link_name"
            } else {
                true
            }
        })
        .collect::<Vec<_>>();
    attrs.push(parse_quote! { #[no_mangle] });

    let vis = &foreign_item_static.vis;
    let static_token = &foreign_item_static.static_token;
    let mutability = &foreign_item_static.mutability;
    let ident = &foreign_item_static.ident;
    let colon_token = &foreign_item_static.colon_token;
    let semi_token = &foreign_item_static.semi_token;

    parse_quote! {
        #(#attrs)* #vis #static_token #mutability #ident #colon_token () = () #semi_token
    }
}
