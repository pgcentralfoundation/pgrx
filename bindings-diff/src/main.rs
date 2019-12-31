use quote::quote;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::process::{Command, Output};
use syn::export::TokenStream2;
use syn::Item;

fn main() {
    let mut v10 = read_source_file("pg-bridge/src/pg_sys/pg10_bindings.rs");
    let mut v11 = read_source_file("pg-bridge/src/pg_sys/pg11_bindings.rs");
    let mut v12 = read_source_file("pg-bridge/src/pg_sys/pg12_bindings.rs");

    let mut versions = vec![&mut v10, &mut v11, &mut v12];
    let common = build_common_set(&mut versions);

    println!(
        "common={}, v10={}, v11={}, v12={}",
        common.len(),
        v10.len(),
        v11.len(),
        v12.len(),
    );

    write_common_file("pg-bridge/src/pg_sys/common.rs", common);
    write_source_file("pg-bridge/src/pg_sys/pg10_specific.rs", v10);
    write_source_file("pg-bridge/src/pg_sys/pg11_specific.rs", v11);
    write_source_file("pg-bridge/src/pg_sys/pg12_specific.rs", v12);
}

fn build_common_set(versions: &mut Vec<&mut HashMap<String, Item>>) -> HashSet<Item> {
    let mut common = HashSet::new();

    for map in versions.iter() {
        for (key, value) in map.iter() {
            if all_contain(&versions, &key) {
                common.insert(value.clone() as Item);
            }
        }
    }

    for map in versions.iter_mut() {
        map.retain(|_, v| !common.contains(v))
    }

    common
}

#[inline]
fn all_contain(maps: &Vec<&mut HashMap<String, Item>>, key: &String) -> bool {
    for map in maps.iter() {
        if !map.contains_key(key) {
            return false;
        }
    }

    true
}

fn read_source_file(filename: &str) -> HashMap<String, Item> {
    let mut file = File::open(filename).unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let source = syn::parse_file(input.as_str()).unwrap();

    let mut item_map = HashMap::with_capacity(32768);
    for item in source.items.into_iter() {
        let mut stream = TokenStream2::new();
        stream.extend(quote! {#item});
        item_map.insert(format!("{}", stream), item);
    }

    item_map
}

fn write_source_file(filename: &str, items: HashMap<String, Item>) {
    let mut stream = TokenStream2::new();
    stream.extend(quote! {use crate::DatumCompatible;});
    stream.extend(quote! {use crate::pg_sys::common::*;});
    for (_, item) in items {
        match item {
            Item::Use(_) => {}
            item => stream.extend(quote! {#item}),
        }
    }
    std::fs::write(filename.clone(), stream.to_string())
        .expect(&format!("Unable to save bindings for {}", filename));
    rustfmt(filename);
}

fn write_common_file(filename: &str, items: HashSet<Item>) {
    let mut stream = TokenStream2::new();
    stream.extend(quote! {use crate::DatumCompatible;});
    stream.extend(quote! {
            #[cfg(feature = "pg10")]
            use crate::pg_sys::pg10_specific::*;
            #[cfg(feature = "pg11")]
            use crate::pg_sys::pg11_specific::*;
            #[cfg(feature = "pg12")]
            use crate::pg_sys::pg12_specific::*;
    });
    for item in items {
        match item {
            Item::Use(_) => {}
            item => stream.extend(quote! {#item}),
        }
    }
    std::fs::write(filename.clone(), stream.to_string())
        .expect(&format!("Unable to save bindings for {}", filename));
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
