use proc_macro2::Ident;
use quote::quote;
use rayon::prelude::*;
use std::any::Any;
use std::fs::DirEntry;
use std::ops::Deref;
use std::path::PathBuf;
use std::result::Result;
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::{FnArg, Item, Pat, ReturnType, Type};

pub(crate) fn generate_schema() -> Result<(), std::io::Error> {
    let path = PathBuf::from_str("./src").unwrap();
    let files = find_rs_files(&path, Vec::new());

    std::panic::set_hook(Box::new(|_| {}));

    files.par_iter().for_each(|f| {
        let result = std::panic::catch_unwind(|| make_create_function_statements(f));

        match result {
            Ok(statements) => {
                // TODO:  write statements to disk
            }
            Err(e) => {
                eprintln!("ERROR:  {}", downcast_err(e));
            }
        }
    });

    Ok(())
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

fn make_create_function_statements(rs_file: &DirEntry) -> Vec<String> {
    let mut sql = Vec::new();
    let file = std::fs::read_to_string(rs_file.path()).unwrap();
    let ast = syn::parse_file(file.as_str()).unwrap();

    for item in ast.items {
        if let Item::Fn(func) = item {
            for att in &func.attrs {
                if let "# [ pg_extern ]" = &format!("{}", quote! {#att}) as &str {
                    // TODO:  pick out: strict, immutable, volatile, parallel safe

                    let mut def = String::new();

                    def.push_str(&format!(
                        "CREATE OR REPLACE FUNCTION {name}",
                        name = quote_ident(&func.sig.ident)
                    ));

                    def.push('(');
                    let mut had_none = false;
                    let mut i = 0;
                    for arg in &func.sig.inputs {
                        match arg {
                            FnArg::Receiver(_) => {
                                panic!("functions that take 'self' are not supported")
                            }
                            FnArg::Typed(ty) => {
                                let type_name = translate_type(rs_file, &ty.ty);

                                match type_name {
                                    Some(type_name) => {
                                        if i > 0 {
                                            def.push_str(", ");
                                        }

                                        def.push_str(&arg_name(arg));
                                        def.push(' ');
                                        def.push_str(&type_name);

                                        i += 1;
                                    }
                                    None => had_none = true,
                                }
                            }
                        }
                    }

                    if had_none && i == 0 {
                        let span = &func.span();
                        eprintln!(
                            "WARNING:  Could not generate function for {} at {}:{}:{} -- it contains only pg_sys::FunctionCallData as its only argument",
                            quote_ident(&func.sig.ident),
                            rs_file.path().display(),
                            span.start().line,
                            span.start().column
                        );
                        continue;
                    }

                    def.push(')');

                    match match &func.sig.output {
                        ReturnType::Default => Some("void".to_string()),
                        ReturnType::Type(_, ty) => translate_type(rs_file, ty),
                    } {
                        Some(return_type) => def.push_str(&format!(" RETURNS {}", return_type)),
                        None => panic!("could not determine return type"),
                    }

                    def.push_str(" LANGUAGE c AS 'MODULE_PATHNAME';");

                    println!("{}", def);
                    sql.push(def);
                }
            }
        }
    }

    sql
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

fn translate_type(filename: &DirEntry, ty: &Box<Type>) -> Option<String> {
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
) -> Option<String> {
    match rust_type.as_str() {
        "i16" => Some("smallint".to_string()),
        "i32" => Some("int".to_string()),
        "i64" => Some("bigint".to_string()),
        "bool" => Some("bool".to_string()),
        "f32" => Some("real".to_string()),
        "f64" => Some("double precision".to_string()),
        "& str" | "String" => Some("text".to_string()),
        "& std :: ffi :: CStr" => Some("cstring".to_string()),
        "pg_sys :: ItemPointerData" => Some("tid".to_string()),
        "pg_sys :: FunctionCallInfo" => None,
        "pg_sys :: IndexAmRoutine" => Some("index_am_handler".to_string()),
        _boxed if rust_type.starts_with("PgBox <") => {
            translate_type_string(extract_type(ty), filename, span, ty)
        }
        _option if rust_type.starts_with("Option <") => {
            translate_type_string(extract_type(ty), filename, span, ty)
        }
        mut unknown => {
            eprintln!(
                "WARNING:  Unsupported rust type: {} at {}:{}:{}",
                unknown,
                filename.path().display(),
                span.start().line,
                span.start().column
            );

            unknown = unknown.trim_start_matches("pg_sys :: ");
            Some(unknown.to_string())
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

fn downcast_err(e: Box<dyn Any + Send>) -> String {
    if let Some(s) = e.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = e.downcast_ref::<String>() {
        s.to_string()
    } else {
        // not a type we understand, so use a generic string
        "Box<Any>".to_string()
    }
}
