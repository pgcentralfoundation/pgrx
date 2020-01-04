use quote::quote;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::process::{Command, Output};
use syn::export::TokenStream2;
use syn::ForeignItem;
use syn::{ImplItem, Item};

#[derive(Eq, Clone)]
struct SortableItem {
    item: Item,
}

impl SortableItem {
    fn new(item: Item) -> Self {
        SortableItem { item }
    }

    fn ident(&self) -> String {
        match &self.item {
            Item::Const(v) => format!("Const: {}", v.ident.to_string()),
            Item::Enum(v) => format!("Enum: {}", v.ident.to_string()),
            Item::ExternCrate(v) => format!("ExternCrate: {}", v.ident.to_string()),
            Item::Fn(v) => format!("Fn: {}", v.sig.ident.to_string()),
            Item::ForeignMod(v) => format!(
                "ForeignMod: {}",
                if v.items.is_empty() {
                    format!("{}", quote! {#v})
                } else {
                    match v.items.first().unwrap() {
                        ForeignItem::Fn(v) => format!("Fn: {}", v.sig.ident.to_string()),
                        ForeignItem::Static(v) => format!("Static: {}", v.ident.to_string()),
                        ForeignItem::Type(v) => format!("Type: {}", v.ident.to_string()),
                        ForeignItem::Macro(v) => format!("Macro: {}", quote! {#v}),
                        ForeignItem::Verbatim(v) => format!("Verbatim: {}", quote! {#v}),
                        ForeignItem::__Nonexhaustive => panic!("ForeignItem __Nonexhausstive"),
                    }
                }
            ),
            Item::Impl(v) => format!(
                "Impl: {}",
                if v.items.is_empty() {
                    format!("{}", quote! {#v})
                } else {
                    match v.items.first().unwrap() {
                        ImplItem::Const(v) => format!("Const: {}", v.ident.to_string()),
                        ImplItem::Method(v) => format!("Method: {}", v.sig.ident.to_string()),
                        ImplItem::Type(v) => format!("Type: {}", v.ident.to_string()),
                        ImplItem::Macro(v) => format!("Macro: {}", format!("{}", quote! {#v})),
                        ImplItem::Verbatim(v) => {
                            format!("Verbatim: {}", format!("{}", quote! {#v}))
                        }
                        ImplItem::__Nonexhaustive => panic!("ImplItem __Nonexhausstive"),
                    }
                }
            ),
            Item::Macro(v) => format!("Macro: {}", quote! {#v}),
            Item::Macro2(v) => format!("Macro2: {}", v.ident.to_string()),
            Item::Mod(v) => format!("Mod: {}", v.ident.to_string()),
            Item::Static(v) => format!("Static: {}", v.ident.to_string()),
            Item::Struct(v) => format!("Struct: {}", v.ident.to_string()),
            Item::Trait(v) => format!("Trait: {}", v.ident.to_string()),
            Item::TraitAlias(v) => format!("TraitAlias: {}", v.ident.to_string()),
            Item::Type(v) => format!("Type: {}", v.ident.to_string()),
            Item::Union(v) => format!("Union: {}", v.ident.to_string()),
            Item::Use(v) => format!("Use: {}", format!("{}", quote! {#v})),
            Item::Verbatim(v) => format!("Verbatim: {}", format!("{}", quote! {#v})),
            Item::__Nonexhaustive => panic!("Item __Nonexhaustive"),
        }
    }
}

impl Ord for SortableItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ident().cmp(&other.ident())
    }
}

impl PartialOrd for SortableItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ident().partial_cmp(&other.ident())
    }
}

impl PartialEq for SortableItem {
    fn eq(&self, other: &Self) -> bool {
        self.item.eq(&other.item)
    }
}

impl Hash for SortableItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item.hash(state)
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut v10 = read_source_file("pg-bridge/src/pg_sys/pg10_bindings.rs");
    let mut v11 = read_source_file("pg-bridge/src/pg_sys/pg11_bindings.rs");
    let mut v12 = read_source_file("pg-bridge/src/pg_sys/pg12_bindings.rs");

    let mut versions = vec![&mut v10, &mut v11, &mut v12];
    let common = build_common_set(&mut versions);

    eprintln!(
        "[all branches]: common={}, v10={}, v11={}, v12={}",
        common.len(),
        v10.len(),
        v11.len(),
        v12.len(),
    );

    write_common_file("pg-bridge/src/pg_sys/common.rs", common);
    write_source_file("pg-bridge/src/pg_sys/pg10_specific.rs", v10);
    write_source_file("pg-bridge/src/pg_sys/pg11_specific.rs", v11);
    write_source_file("pg-bridge/src/pg_sys/pg12_specific.rs", v12);

    Ok(())
}

fn build_common_set(
    versions: &mut Vec<&mut BTreeMap<String, SortableItem>>,
) -> BTreeSet<SortableItem> {
    let mut common = BTreeSet::new();

    for map in versions.iter() {
        for (key, value) in map.iter() {
            if common.contains(value)
                || key.contains("pub struct __BindgenUnionField")
                || key.contains("pub struct __IncompleteArrayField")
            {
                continue;
            }

            if all_contain(&versions, &key) {
                common.insert(value.clone());
            }
        }
    }

    for map in versions.iter_mut() {
        for item in &common {
            let item = &item.item;
            let key = format!("{}", quote! {#item});
            map.remove(&key);
        }
    }

    common
}

#[inline]
fn all_contain(maps: &[&mut BTreeMap<String, SortableItem>], key: &str) -> bool {
    for map in maps.iter() {
        if !map.contains_key(key) {
            return false;
        }
    }

    true
}

fn read_source_file(filename: &str) -> BTreeMap<String, SortableItem> {
    let mut file = File::open(filename).unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let source = syn::parse_file(input.as_str()).unwrap();

    let mut item_map = BTreeMap::new();
    for item in source.items.into_iter() {
        let mut stream = TokenStream2::new();
        stream.extend(quote! {#item});
        item_map.insert(format!("{}", stream), SortableItem::new(item));
    }

    item_map
}

fn write_source_file(filename: &str, items: BTreeMap<String, SortableItem>) {
    let mut stream = TokenStream2::new();
    stream.extend(quote! {
        #![allow(clippy::all)]

        use crate as pg_bridge;
        use crate::pg_sys::common::*;
        use crate::DatumCompatible;
    });
    for (_, item) in items {
        match &item.item {
            Item::Use(_) => {}
            item => stream.extend(quote! {#item}),
        }
    }
    std::fs::write(filename, stream.to_string())
        .unwrap_or_else(|_| panic!("Unable to save bindings for {}", filename));
    rustfmt(filename);
}

fn write_common_file(filename: &str, items: BTreeSet<SortableItem>) {
    let mut stream = TokenStream2::new();
    stream.extend(quote! {
        #![allow(clippy::all)]

        use crate as pg_bridge;
        use crate::datum_compatible::DatumCompatible;

        #[cfg(feature = "pg10")]
        use crate::pg_sys::pg10_specific::*;
        #[cfg(feature = "pg11")]
        use crate::pg_sys::pg11_specific::*;
        #[cfg(feature = "pg12")]
        use crate::pg_sys::pg12_specific::*;
    });
    for item in items.iter() {
        match &item.item {
            Item::Use(_) => {}
            item => stream.extend(quote! {#item}),
        }
    }
    std::fs::write(filename, stream.to_string())
        .unwrap_or_else(|_| panic!("Unable to save bindings for {}", filename));
    rustfmt(filename);
}

fn rustfmt(filename: &str) {
    run_command(
        Command::new("rustfmt").arg(filename).current_dir("."),
        "common",
    )
    .unwrap();
}

fn run_command(command: &mut Command, branch_name: &str) -> Result<Output, std::io::Error> {
    let mut dbg = String::new();

    dbg.push_str(&format!(
        "[{}]: -------- {:?} -------- \n",
        branch_name, command
    ));

    let output = command.output()?;
    let rc = output.clone();

    if !output.stdout.is_empty() {
        for line in String::from_utf8(output.stdout).unwrap().lines() {
            dbg.push_str(&format!("[{}] [stdout]: {}\n", branch_name, line));
        }
    }

    if !output.stderr.is_empty() {
        for line in String::from_utf8(output.stderr).unwrap().lines() {
            dbg.push_str(&format!("[{}] [stderr]: {}\n", branch_name, line));
        }
    }
    dbg.push_str(&format!(
        "[{}] /----------------------------------------\n",
        branch_name
    ));

    eprintln!("{}", dbg);
    Ok(rc)
}
