use proc_macro2::Ident;
use quote::quote;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::DirEntry;
use std::io::{BufRead, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::result::Result;
use std::str::FromStr;
use std::sync::Mutex;
use syn::export::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, FnArg, Item, Pat, ReturnType, Type};

pub(crate) fn generate_schema() -> Result<(), std::io::Error> {
    let path = PathBuf::from_str("./src").unwrap();
    let files = find_rs_files(&path, Vec::new());

    let mut created = Mutex::new(Vec::new());
    let mut deleted = Mutex::new(Vec::new());
    files.par_iter().for_each(|f: &DirEntry| {
        let statemets = make_create_function_statements(f);
        let (did_write, filename) = write_sql_file(f, statemets);

        // strip the leading ./sql/ from the filenames we generated
        let mut filename = filename.display().to_string();
        filename = filename.trim_start_matches("./sql/").to_string();

        if did_write {
            created.lock().unwrap().push(filename);
        } else {
            deleted.lock().unwrap().push(filename);
        }
    });

    process_schema_load_order(created.get_mut().unwrap(), deleted.get_mut().unwrap());

    Ok(())
}

fn process_schema_load_order(created: &mut Vec<String>, deleted: &mut Vec<String>) {
    let filename = PathBuf::from_str("./sql/load-order.txt").unwrap();
    let mut load_order = read_load_order(&filename);

    // remove everything from load_order that we deleted
    for v in deleted {
        if let Some(idx) = load_order.iter().position(|r| r.eq(v)) {
            load_order.remove(idx);
        }
    }

    // created keeps only those items that aren't already in load_order
    created.retain(|v| !load_order.contains(v));

    // and finally we append whatever is left in created to load_order as they're new files
    load_order.append(created);

    // rewrite the load_order file
    let mut file = std::fs::File::create(&filename)
        .expect(&format!("couldn't open {} for writing", filename.display()));
    load_order.iter().for_each(|v| {
        let v = v.trim_start_matches("./sql/");

        file.write_all(v.as_bytes())
            .expect(&format!("failed to write to {}", filename.display()));
        file.write(&['\n' as u8])
            .expect(&format!("failed to write to {}", filename.display()));
    });
}

pub(crate) fn read_load_order(filename: &PathBuf) -> Vec<String> {
    let mut load_order = Vec::new();

    if let Ok(file) = std::fs::File::open(&filename) {
        let reader = std::io::BufReader::new(file);
        for (_, line) in reader.lines().enumerate() {
            load_order.push(line.unwrap());
        }
    }

    load_order
}

fn write_sql_file(f: &DirEntry, statements: Vec<String>) -> (bool, PathBuf) {
    let filename = make_sql_filename(f);

    if statements.is_empty() {
        // delete existing sql file if it exists
        if filename.exists() {
            std::fs::remove_file(&filename)
                .expect(&format!("unable to delete {}", filename.display()));
        }

        (false, filename)
    } else {
        // write the statements out to the sql file
        let mut file = std::fs::File::create(&filename)
            .expect(&format!("failed to open {}", filename.display()));
        for statement in statements {
            file.write_all(statement.as_bytes())
                .expect(&format!("failed to write to {}", filename.display()));
            file.write(&['\n' as u8])
                .expect(&format!("failed to write to {}", filename.display()));
        }

        (true, filename)
    }
}

fn make_sql_filename(f: &DirEntry) -> PathBuf {
    PathBuf::from_str(make_sql_filename_from_string(f.path().display().to_string()).as_str())
        .unwrap()
}

fn make_sql_filename_from_string(mut sql_filename: String) -> String {
    sql_filename = sql_filename.trim_start_matches("./src/").to_string();
    sql_filename = sql_filename.trim_start_matches("./sql/").to_string();
    sql_filename = sql_filename.trim_end_matches(".rs").to_string();
    sql_filename = sql_filename.replace("/", "_");
    sql_filename.insert_str(0, "./sql/");
    sql_filename.push_str(".generated.sql");

    sql_filename
}

fn find_rs_files(path: &PathBuf, mut files: Vec<DirEntry>) -> Vec<DirEntry> {
    if path.display().to_string().contains("/target/") {
        // ignore the target/ directory
        return files;
    }

    for f in std::fs::read_dir(path).unwrap() {
        if f.is_ok() {
            let f = f.unwrap();
            let filename = f.file_name().into_string().unwrap();

            if f.metadata().unwrap().is_dir() {
                // recurse
                files = find_rs_files(&f.path(), files);
            } else if filename.ends_with(".rs") {
                files.push(f);
            }
        }
    }

    files
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum ExternArgs {
    Immutable,
    Strict,
    Stable,
    Volatile,
    Raw,
    ParallelSafe,
    ParallelUnsafe,
    ParallelRestricted,
}

fn parse_extern_args(att: &Attribute) -> HashSet<ExternArgs> {
    let mut args = HashSet::<ExternArgs>::new();
    let line = att.into_token_stream().to_string();
    let mut line = line.as_str();

    if line.contains('(') {
        line = line.trim_start_matches("# [ pg_extern").trim();
        line = line.trim_start_matches("# [ pg_test").trim();
        line = line.trim_start_matches("(").trim();
        line = line.trim_end_matches(") ]").trim();
        line = line.trim_end_matches("]").trim();
        let atts: Vec<_> = line.split(',').collect();

        for att in atts {
            let att = att.trim();

            match att {
                "immutable" => args.insert(ExternArgs::Immutable),
                "strict" => args.insert(ExternArgs::Strict),
                "stable" => args.insert(ExternArgs::Stable),
                "volatile" => args.insert(ExternArgs::Volatile),
                "raw" => args.insert(ExternArgs::Raw),
                "parallel_safe" => args.insert(ExternArgs::ParallelSafe),
                "parallel_unsafe" => args.insert(ExternArgs::ParallelUnsafe),
                "parallel_restricted" => args.insert(ExternArgs::ParallelRestricted),
                _ => false,
            };
        }
    }
    args
}

fn make_create_function_statements(rs_file: &DirEntry) -> Vec<String> {
    let mut sql = Vec::new();
    let file = std::fs::read_to_string(rs_file.path()).unwrap();
    let ast = syn::parse_file(file.as_str()).unwrap();

    walk_items(rs_file, &mut sql, ast.items);

    sql
}

fn walk_items(rs_file: &DirEntry, sql: &mut Vec<String>, items: Vec<Item>) {
    for item in items {
        if let Item::Mod(modd) = item {
            match modd.content {
                Some((_, items)) => walk_items(rs_file, sql, items),
                None => {}
            }
        } else if let Item::Macro(makro) = item {
            let name = makro
                .mac
                .path
                .get_ident()
                .unwrap()
                .into_token_stream()
                .to_string();

            if "extension_sql".eq(&name) {
                sql.push(makro.mac.tokens.to_string());
            }
        } else if let Item::Fn(func) = item {
            for att in &func.attrs {
                let att_desc = att.clone().into_token_stream().to_string();
                let is_test_mode = std::env::var("PGX_TEST_MODE").is_ok();
                if att_desc.starts_with("# [ pg_extern")
                    || (att_desc.starts_with("# [ pg_test") && is_test_mode)
                {
                    let mut extern_args = parse_extern_args(att);
                    let mut def = String::new();
                    let exported_func_name = format!("{}_wrapper", func.sig.ident.to_string());

                    def.push_str(&format!(
                        "CREATE OR REPLACE FUNCTION {name}",
                        name = quote_ident(&func.sig.ident)
                    ));

                    // function arguments
                    def.push('(');
                    let mut had_none = false;
                    let mut i = 0;
                    let mut had_option = false;
                    for arg in &func.sig.inputs {
                        match arg {
                            FnArg::Receiver(_) => {
                                panic!("functions that take 'self' are not supported")
                            }
                            FnArg::Typed(ty) => match translate_type(rs_file, &ty.ty) {
                                Some((type_name, is_option)) => {
                                    if i > 0 {
                                        def.push_str(", ");
                                    }

                                    def.push_str(&arg_name(arg));
                                    def.push(' ');
                                    def.push_str(&type_name);

                                    if is_option {
                                        had_option = true;
                                    }

                                    i += 1;
                                }
                                None => had_none = true,
                            },
                        }
                    }

                    if had_none && i == 0 {
                        let span = &func.span();
                        eprintln!(
                        "{}:{}:{}: Could not generate function for {} at  -- it contains only pg_sys::FunctionCallData as its only argument",
                        rs_file.path().display(),
                        span.start().line,
                        span.start().column,
                        quote_ident(&func.sig.ident),
                    );
                        continue;
                    }

                    def.push(')');

                    if !had_option {
                        // there were no Option<T> arguments, so the function can be declared STRICT
                        extern_args.insert(ExternArgs::Strict);
                    }

                    // append RETURNS clause
                    match match &func.sig.output {
                        ReturnType::Default => Some(("void".to_string(), false)),
                        ReturnType::Type(_, ty) => translate_type(rs_file, ty),
                    } {
                        Some((return_type, _is_option)) => {
                            def.push_str(&format!(" RETURNS {}", return_type))
                        }
                        None => panic!("could not determine return type"),
                    }

                    // modifiers
                    for extern_arg in extern_args {
                        match extern_arg {
                            ExternArgs::Immutable => def.push_str(" IMMUTABLE"),
                            ExternArgs::Strict => def.push_str(" STRICT"),
                            ExternArgs::Stable => def.push_str(" STABLE"),
                            ExternArgs::Volatile => def.push_str(" VOLATILE"),
                            ExternArgs::Raw => { /* no op */ }
                            ExternArgs::ParallelSafe => def.push_str(" PARALLEL SAFE"),
                            ExternArgs::ParallelUnsafe => def.push_str(" PARALLEL UNSAFE"),
                            ExternArgs::ParallelRestricted => def.push_str(" PARALLEL RESTRICTED"),
                        }
                    }

                    def.push_str(&format!(
                        " LANGUAGE c AS 'MODULE_PATHNAME', '{}';",
                        exported_func_name
                    ));
                    sql.push(def);
                }
            }
        }
    }
}

fn quote_ident(ident: &Ident) -> String {
    let mut name = format!("{}", quote! {#ident});
    name = name.replace("\"", "\"\"");
    format!("\"{}\"", name)
}

fn arg_name(arg: &FnArg) -> String {
    if let FnArg::Typed(ty) = arg {
        if let Pat::Ident(ident) = ty.pat.deref() {
            return quote_ident(&ident.ident);
        }

        panic!("Can't figure out argument name");
    }

    panic!("functions that take 'self' are not supported")
}

fn translate_type(filename: &DirEntry, ty: &Box<Type>) -> Option<(String, bool)> {
    let rust_type;
    let span;
    if let Type::Path(path) = ty.deref() {
        rust_type = format!("{}", quote! {#path});
        span = path.span().clone();
    } else if let Type::Reference(tref) = ty.deref() {
        let elem = &tref.elem;
        rust_type = format!("{}", quote! {&#elem});
        span = tref.span().clone();
    } else {
        panic!("Unsupported type: {}", quote! {#ty});
    }

    translate_type_string(rust_type, filename, &span, ty)
}

fn translate_type_string(
    rust_type: String,
    filename: &DirEntry,
    span: &proc_macro2::Span,
    ty: &Box<Type>,
) -> Option<(String, bool)> {
    match rust_type.as_str() {
        "i8" => Some(("smallint".to_string(), false)), // convert i8 types into smallints as Postgres doesn't have a 1byte-sized type
        "i16" => Some(("smallint".to_string(), false)),
        "i32" => Some(("int".to_string(), false)),
        "i64" => Some(("bigint".to_string(), false)),
        "bool" => Some(("bool".to_string(), false)),
        "char" => Some(("char".to_string(), false)),
        "f32" => Some(("real".to_string(), false)),
        "f64" => Some(("double precision".to_string(), false)),
        "& str" | "String" => Some(("text".to_string(), false)),
        "& std :: ffi :: CStr" => Some(("cstring".to_string(), false)),
        "pg_sys :: ItemPointerData" => Some(("tid".to_string(), false)),
        "pg_sys :: FunctionCallInfo" => None,
        "pg_sys :: IndexAmRoutine" => Some(("index_am_handler".to_string(), false)),
        _boxed if rust_type.starts_with("PgBox <") => {
            translate_type_string(extract_type(ty), filename, span, ty)
        }
        _option if rust_type.starts_with("Option <") => {
            let rc = translate_type_string(extract_type(ty), filename, span, ty);
            Some((rc.unwrap().0, true))
        }
        mut unknown => {
            if std::env::var("DEBUG").is_ok() {
                eprintln!(
                    "{}:{}:{}: Unrecognized type: {}",
                    filename.path().display(),
                    span.start().line,
                    span.start().column,
                    unknown,
                );
            }

            unknown = unknown.trim_start_matches("pg_sys :: ");
            Some((unknown.to_string(), false))
        }
    }
}

fn extract_type(ty: &Box<Type>) -> String {
    match ty.deref() {
        Type::Path(path) => {
            for segment in &path.path.segments {
                let arguments = &segment.arguments;
                let mut type_name = &format!("{}", quote! {#arguments}) as &str;

                type_name = type_name.trim();
                type_name = type_name.trim_start_matches('<');
                type_name = type_name.trim_end_matches('>');

                while type_name.contains('<') {
                    // trim off type
                    type_name = type_name.trim();
                    type_name = &type_name[type_name.find(' ').unwrap()..];
                    type_name = type_name.trim_start_matches('<');
                    type_name = type_name.trim_end_matches('>');
                }

                return type_name.trim().to_string();
            }
            panic!("No type found inside Option");
        }
        _ => panic!("No type found inside Option"),
    }
}
