use proc_macro2::{Ident, Span};
use quote::quote;
use rayon::prelude::*;
use std::borrow::BorrowMut;
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
use syn::{Attribute, FnArg, Item, ItemFn, Pat, ReturnType, Type};

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
    Error(String),
}

#[derive(Debug)]
enum CategorizedAttribute {
    PgGuard(Span),
    PgTest((Span, HashSet<ExternArgs>)),
    RustTest(Span),
    PgExtern((Span, HashSet<ExternArgs>)),
    Sql(Vec<String>),
    SqlFunctionName(String),
    SqlFunctionArgs(String),
    Other(Vec<(Span, String)>),
}

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
                error if att.starts_with("error") => {
                    let re = regex::Regex::new(r#"("[^"\\]*(?:\\.[^"\\]*)*")"#).unwrap();

                    let message = match re.captures(error) {
                        Some(captures) => match captures.get(0) {
                            Some(mtch) => {
                                let message = mtch.as_str().clone();
                                let message = unescape::unescape(message)
                                    .expect("improperly escaped error message");

                                // trim leading/trailing quotes
                                let message = String::from(&message[1..]);
                                let message = String::from(&message[..message.len() - 1]);

                                message
                            }
                            None => {
                                panic!("No matches found in: {}", error);
                            }
                        },
                        None => panic!("/{}/ is an invalid error= attribute", error),
                    };

                    args.insert(ExternArgs::Error(message.to_string()))
                }

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
                let string = makro.mac.tokens.to_string();
                let string = string.trim();

                if !string.starts_with("r#\"") || !string.ends_with("\"#") {
                    panic!("extension_sql!{} value isn't ia raw string");
                }

                // remove the raw string quotes
                let string = string.trim_start_matches("r#\"");
                let string = string.trim_end_matches("\"#");

                // trim off leading/trailing new lines, but preserve other whitespace
                let string = string.trim_start_matches('\n');
                let string = string.trim_end_matches('\n');

                // and remember this sql block
                sql.push(string.to_string());
            }
        } else if let Item::Fn(func) = item {
            let attributes = collect_attributes(rs_file, &func);
            let is_test_mode = std::env::var("PGX_TEST_MODE").is_ok();
            let mut function_sql = Vec::new();
            let sql_func_args = extract_funcargs_attribute(&attributes);

            for attribute in attributes {
                match attribute {
                    // only generate CREATE FUNCTION statements for #[pg_test] functions
                    // if we're in test mode, which is controlled by the PGX_TEST_MODE
                    // environment variable
                    CategorizedAttribute::PgTest((span, _)) if is_test_mode => {
                        match make_create_function_statement(&func, None, rs_file, None) {
                            Some(statement) => {
                                function_sql.push(location_comment(rs_file, span));
                                function_sql.push(statement)
                            }
                            None => {}
                        }
                    }

                    // for #[pg_extern] attributes, we only want to programatically generate
                    // a CREATE FUNCTION statement if we don't already have some
                    CategorizedAttribute::PgExtern((span, args)) if function_sql.is_empty() => {
                        match make_create_function_statement(
                            &func,
                            Some(args),
                            rs_file,
                            sql_func_args.clone(),
                        ) {
                            Some(statement) => {
                                function_sql.push(location_comment(rs_file, span));
                                function_sql.push(statement)
                            }
                            None => {} // TODO:  Emit a warning?
                        }
                    }

                    // it's user-provided SQL from doc comment blocks
                    CategorizedAttribute::Sql(mut sql_lines) => function_sql.append(&mut sql_lines),

                    // we don't care about other attributes
                    _ => {}
                }
            }

            sql.append(&mut function_sql);
        }
    }
}

fn make_create_function_statement(
    func: &ItemFn,
    mut extern_args: Option<HashSet<ExternArgs>>,
    rs_file: &DirEntry,
    sql_func_arg: Option<String>,
) -> Option<String> {
    let exported_func_name = format!("{}_wrapper", func.sig.ident.to_string());
    let mut statement = String::new();
    let has_option_arg = func_args_have_option(func, rs_file);
    let attributes = collect_attributes(rs_file, func);
    let sql_func_name =
        extract_funcname_attribute(&attributes).unwrap_or(quote_ident(&func.sig.ident));

    statement.push_str(&format!("CREATE OR REPLACE FUNCTION {}", sql_func_name));

    if sql_func_arg.is_some() {
        statement.push_str(sql_func_arg.unwrap().as_str());
    } else {
        // function arguments
        statement.push('(');
        let mut had_none = false;
        let mut i = 0;
        for arg in &func.sig.inputs {
            match arg {
                FnArg::Receiver(_) => panic!("functions that take 'self' are not supported"),
                FnArg::Typed(ty) => match translate_type(rs_file, &ty.ty) {
                    Some((type_name, _)) => {
                        if i > 0 {
                            statement.push_str(", ");
                        }

                        statement.push_str(&arg_name(arg));
                        statement.push(' ');
                        statement.push_str(&type_name);

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
            return None;
        }
        statement.push(')');
    }

    if !has_option_arg {
        // there were no Option<T> arguments, so the function can be declared STRICT
        if let Some(extern_args) = extern_args.borrow_mut() {
            extern_args.insert(ExternArgs::Strict);
        }
    }

    // append RETURNS clause
    match match &func.sig.output {
        ReturnType::Default => Some(("void".to_string(), false)),
        ReturnType::Type(_, ty) => translate_type(rs_file, ty),
    } {
        Some((return_type, _is_option)) => statement.push_str(&format!(" RETURNS {}", return_type)),
        None => panic!("could not determine return type"),
    }

    // modifiers
    if let Some(extern_args) = extern_args {
        for extern_arg in extern_args {
            match extern_arg {
                ExternArgs::Immutable => statement.push_str(" IMMUTABLE"),
                ExternArgs::Strict => statement.push_str(" STRICT"),
                ExternArgs::Stable => statement.push_str(" STABLE"),
                ExternArgs::Volatile => statement.push_str(" VOLATILE"),
                ExternArgs::Raw => { /* noop */ }
                ExternArgs::ParallelSafe => statement.push_str(" PARALLEL SAFE"),
                ExternArgs::ParallelUnsafe => statement.push_str(" PARALLEL UNSAFE"),
                ExternArgs::ParallelRestricted => statement.push_str(" PARALLEL RESTRICTED"),
                ExternArgs::Error(_) => { /* noop */ }
            }
        }
    }

    statement.push_str(&format!(
        " LANGUAGE c AS 'MODULE_PATHNAME', '{}';",
        exported_func_name
    ));

    Some(statement)
}

fn func_args_have_option(func: &ItemFn, rs_file: &DirEntry) -> bool {
    for arg in &func.sig.inputs {
        if let FnArg::Typed(ty) = arg {
            if let Some((_, is_option)) = translate_type(rs_file, &ty.ty) {
                if is_option {
                    return true;
                }
            }
        }
    }

    false
}

fn quote_ident(ident: &Ident) -> String {
    quote_ident_string(ident.to_token_stream().to_string())
}

fn quote_ident_string(ident: String) -> String {
    let mut quoted = String::new();

    quoted.push('"');
    quoted.push_str(ident.replace("\"", "\"\"").as_str());
    quoted.push('"');

    quoted
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

    translate_type_string(rust_type, filename, &span, 0)
}

fn translate_type_string(
    rust_type: String,
    filename: &DirEntry,
    span: &proc_macro2::Span,
    depth: i32,
) -> Option<(String, bool)> {
    match rust_type.as_str() {
        "i8" => Some(("smallint".to_string(), false)), // convert i8 types into smallints as Postgres doesn't have a 1byte-sized type
        "i16" => Some(("smallint".to_string(), false)),
        "i32" => Some(("integer".to_string(), false)),
        "i64" => Some(("bigint".to_string(), false)),
        "bool" => Some(("bool".to_string(), false)),
        "char" => Some(("char".to_string(), false)),
        "f32" => Some(("real".to_string(), false)),
        "f64" => Some(("double precision".to_string(), false)),
        "& str" | "String" => Some(("text".to_string(), false)),
        "& std :: ffi :: CStr" => Some(("cstring".to_string(), false)),
        "AnyElement" => Some(("anyelement".to_string(), false)),
        "pg_sys :: Oid" => Some(("oid".to_string(), false)),
        "pg_sys :: ItemPointerData" => Some(("tid".to_string(), false)),
        "pg_sys :: FunctionCallInfo" => None,
        "pg_sys :: IndexAmRoutine" => Some(("index_am_handler".to_string(), false)),
        _array if rust_type.starts_with("Array <") => {
            let rc = translate_type_string(extract_type(&rust_type), filename, span, depth + 1);
            let mut type_string = rc.unwrap().0;
            type_string.push_str("[]");
            Some((type_string, false))
        }
        _internal if rust_type.starts_with("Internal <") => Some(("internal".to_string(), false)),
        _boxed if rust_type.starts_with("PgBox <") => {
            translate_type_string(extract_type(&rust_type), filename, span, depth + 1)
        }
        _option if rust_type.starts_with("Option <") => {
            let rc = translate_type_string(extract_type(&rust_type), filename, span, depth + 1);
            let type_string = rc.unwrap().0;
            Some((type_string, true))
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

fn extract_type(type_name: &str) -> String {
    let re = regex::Regex::new(r#"\w+ < (.*) >.*"#).unwrap();
    let capture = re.captures(type_name).unwrap().get(1);
    return capture.unwrap().as_str().to_string();
}

fn extract_funcargs_attribute(attrs: &Vec<CategorizedAttribute>) -> Option<String> {
    for a in attrs {
        if let CategorizedAttribute::SqlFunctionArgs(sql) = a {
            return Some(sql.clone());
        }
    }

    None
}

fn extract_funcname_attribute(attrs: &Vec<CategorizedAttribute>) -> Option<String> {
    for a in attrs {
        if let CategorizedAttribute::SqlFunctionName(sql) = a {
            return Some(quote_ident_string(sql.clone()));
        }
    }

    None
}

fn collect_attributes(rs_file: &DirEntry, func: &ItemFn) -> Vec<CategorizedAttribute> {
    let attrs = &func.attrs;
    let mut categorized_attributes = Vec::new();
    let mut other_attributes = Vec::new();

    //    let itr = attrs.iter();
    let mut i = 0;
    while i < attrs.len() {
        let a = attrs.get(i).unwrap();
        let span = a.span();
        let as_string = a.to_token_stream().to_string();

        if as_string == "# [ pg_guard ]" {
            categorized_attributes.push(CategorizedAttribute::PgGuard(span));
        } else if as_string.starts_with("# [ pg_extern") {
            categorized_attributes.push(CategorizedAttribute::PgExtern((
                span,
                parse_extern_args(&a),
            )));
        } else if as_string.starts_with("# [ pg_test") {
            categorized_attributes
                .push(CategorizedAttribute::PgTest((span, parse_extern_args(&a))));
        } else if as_string == "# [ test ]" {
            categorized_attributes.push(CategorizedAttribute::RustTest(span));
        } else if as_string == "# [ doc = \" ```funcname\" ]" {
            let (new_i, mut sql_statements) = collect_doc(
                rs_file,
                &func,
                attrs,
                &mut other_attributes,
                i,
                a,
                span,
                false,
            );

            if sql_statements.len() == 1 {
                categorized_attributes.push(CategorizedAttribute::SqlFunctionName(
                    sql_statements.pop().unwrap(),
                ));
            } else if sql_statements.len() == 0 {
                panic!("Found no lines for ```funcname");
            } else {
                panic!("Found more than 1 line for ```funcname");
            }

            i = new_i;
        } else if as_string == "# [ doc = \" ```funcargs\" ]" {
            let (new_i, mut sql_statements) = collect_doc(
                rs_file,
                &func,
                attrs,
                &mut other_attributes,
                i,
                a,
                span,
                false,
            );

            if sql_statements.len() == 1 {
                categorized_attributes.push(CategorizedAttribute::SqlFunctionArgs(
                    sql_statements.pop().unwrap(),
                ));
            } else if sql_statements.len() == 0 {
                panic!("Found no lines for ```funcargs");
            } else {
                panic!("Found more than 1 line for ```funcargs");
            }

            i = new_i;
        } else if as_string == "# [ doc = \" ```sql\" ]" {
            let (new_i, sql_statements) = collect_doc(
                rs_file,
                &func,
                attrs,
                &mut other_attributes,
                i,
                a,
                span,
                true,
            );

            if !sql_statements.is_empty() {
                categorized_attributes.push(CategorizedAttribute::Sql(sql_statements));
            }

            i = new_i;
        } else {
            other_attributes.push((span, as_string));
        }

        i = i + 1;
    }

    if !other_attributes.is_empty() {
        categorized_attributes.push(CategorizedAttribute::Other(other_attributes));
    }

    categorized_attributes
}

fn collect_doc(
    rs_file: &DirEntry,
    func: &&ItemFn,
    attrs: &Vec<Attribute>,
    other_attributes: &mut Vec<(Span, String)>,
    mut i: usize,
    a: &Attribute,
    span: Span,
    track_location: bool,
) -> (usize, Vec<String>) {
    let mut sql_statements = Vec::new();

    // run forward saving each line as an sql statement until we find ```

    if track_location {
        sql_statements.push(location_comment(rs_file, a.span()));
    }

    i = i + 1;
    while i < attrs.len() {
        let a = attrs.get(i).unwrap();
        let as_string = a.to_token_stream().to_string();

        if as_string == "# [ doc = \" ```\" ]" {
            // we found the end to this ```sql block of documentation
            break;
        } else if as_string.starts_with("# [ doc = \"") {
            // it's a doc line within the sql block
            let as_string = as_string.trim_start_matches("# [ doc = \"");
            let as_string = as_string.trim_end_matches("\" ]");
            let as_string = as_string.trim();
            let as_string = unescape::unescape(as_string)
                .expect(&format!("Improperly escaped:\n{}", as_string));

            // do variable substitution in the sql statement
            let as_string =
                as_string.replace("@FUNCTION_NAME@", &format!("{}_wrapper", func.sig.ident));

            // and remember it, along with its original source location
            sql_statements.push(as_string);
        } else {
            // it's not a doc line, so add it to other_attributes and get out
            other_attributes.push((span, as_string));
            break;
        }

        i = i + 1;
    }
    (i, sql_statements)
}

fn location_comment(rs_file: &DirEntry, span: Span) -> String {
    format!(
        "-- {}:{}:{}",
        rs_file.path().display(),
        span.start().line,
        span.start().column,
    )
}
