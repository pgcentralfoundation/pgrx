use std::{
    path::Path,
    io::{Read, Write},
    fs::{read_dir, File}, sync::WaitTimeoutResult,
};
use quote::ToTokens;
use syn::{parse_quote, parse_file, Item, ForeignItem, ForeignItemStatic, ForeignItemFn};
use eyre::WrapErr;

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
        pgx_pg_sys_file.read_to_string(&mut buf)
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
                            },
                            ForeignItem::Static(foreign_item_static) => {
                                let stub = stub_for_static(&foreign_item_static);
                                items_with_stubs.push(stub);
                            },
                            _ => items_without_foreign_fns.push(inner_item.clone()),
                        }
                    }
                    item_foreign_mod.items = items_without_foreign_fns;
                },
                _ => (),
            }
            // items_with_stubs.push(item);
        }

        items_with_stubs.push(parse_quote! {
            #[no_mangle]
            pub extern "C" fn pg_re_throw() {
                unimplemented!()
            }
        });

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
    let mut attrs = foreign_item_fn.attrs.iter().cloned().filter(|attr| {
        if let Some(ident) = attr.path.get_ident() {
            ident.to_string() != "pg_guard" && ident.to_string() != "link_name"
        } else {
            true
        }
    }).collect::<Vec<_>>();
    attrs.push(parse_quote! { #[no_mangle] });
    let vis = &foreign_item_fn.vis;
    let mut sig = foreign_item_fn.sig.clone();
    sig.inputs = syn::punctuated::Punctuated::<syn::FnArg, syn::Token![,]>::new();
    sig.output = syn::ReturnType::Default;
    sig.abi = Some(parse_quote!{ extern "C" });
    sig.variadic = None;
    parse_quote!{ 
        #(#attrs)* #vis #sig {
            unimplemented!()
        }
    }
}


fn stub_for_static(foreign_item_static: &ForeignItemStatic) -> Item {
    let mut attrs = foreign_item_static.attrs.iter().cloned().filter(|attr| {
        if let Some(ident) = attr.path.get_ident() {
            ident.to_string() != "pg_guard" && ident.to_string() != "link_name"
        } else {
            true
        }
    }).collect::<Vec<_>>();
    attrs.push(parse_quote! { #[no_mangle] });

    let vis = &foreign_item_static.vis;
    let static_token = &foreign_item_static.static_token;
    let mutability = &foreign_item_static.mutability;
    let ident = &foreign_item_static.ident;
    let colon_token = &foreign_item_static.colon_token;
    let ty = &foreign_item_static.ty;
    let semi_token = &foreign_item_static.semi_token;

    parse_quote!{ 
        #(#attrs)* #vis #static_token #mutability #ident #colon_token () = () #semi_token
    }
}