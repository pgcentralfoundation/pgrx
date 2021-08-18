use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr, Token,
};
use std::{io::Write, fs::{create_dir_all, File}};

/// A parsed `extension_sql_file!()` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a [`InventoryExtensionSql`].
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::pg_inventory::ExtensionSqlFile;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: Macro = parse_quote! {
///     extension_sql_file!("sql/example.sql", bootstrap)
/// };
/// let inner_tokens = parsed.tokens;
/// let inner: ExtensionSqlFile = parse_quote! {
///     #inner_tokens
/// };
/// let inventory_tokens = inner.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ExtensionSqlFile {
    pub path: LitStr,
    pub attrs: Punctuated<ExtensionSqlAttribute, Token![,]>,
}

impl ExtensionSqlFile {
    pub fn inventory_fn_name(&self) -> String {
        std::path::PathBuf::from(self.path.value())
            .file_name()
            .expect("No file name for extension_sql_file!()")
            .to_str()
            .expect("No UTF-8 file name for extension_sql_file!()")
            .to_string()
    }

    pub fn inventory(&self, inventory_dir: String) {
        create_dir_all(&inventory_dir).expect("Couldn't create inventory dir.");
        let mut fd = File::create(inventory_dir.to_string() + "/sql_" + &self.inventory_fn_name() + ".json").expect("Couldn't create inventory file");
        let inventory_fn_json = serde_json::to_string(&self.inventory_fn_name()).expect("Could not serialize inventory item.");
        write!(fd, "{}", inventory_fn_json).expect("Couldn't write to inventory file");
    }
}

impl Parse for ExtensionSqlFile {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let path = input.parse()?;
        let _after_sql_comma: Option<Token![,]> = input.parse()?;
        let attrs = input.parse_terminated(ExtensionSqlAttribute::parse)?;
        Ok(Self { path, attrs })
    }
}

impl ToTokens for ExtensionSqlFile {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let path = &self.path;
        let mut name = None;
        let mut bootstrap = false;
        let mut finalize = false;
        let mut skip_inventory = false;
        let mut before = vec![];
        let mut after = vec![];
        let mut creates = vec![];
        for attr in &self.attrs {
            match attr {
                ExtensionSqlAttribute::Creates(items) => {
                    creates.append(&mut items.iter().map(|x| x.to_token_stream()).collect());
                }
                ExtensionSqlAttribute::Before(items) => {
                    before.append(&mut items.iter().map(|x| x.to_token_stream()).collect());
                }
                ExtensionSqlAttribute::After(items) => {
                    after.append(&mut items.iter().map(|x| x.to_token_stream()).collect());
                }
                ExtensionSqlAttribute::Bootstrap => {
                    bootstrap = true;
                }
                ExtensionSqlAttribute::Finalize => {
                    finalize = true;
                }
                ExtensionSqlAttribute::SkipInventory => {
                    skip_inventory = true;
                }
                ExtensionSqlAttribute::Name(found_name) => {
                    name = Some(found_name.value());
                }
            }
        }
        name.get_or_insert(
            std::path::PathBuf::from(path.value())
                .file_name()
                .expect("No file name for extension_sql_file!()")
                .to_str()
                .expect("No UTF-8 file name for extension_sql_file!()")
                .to_string(),
        );
        let before_iter = before.iter();
        let after_iter = after.iter();
        let creates_iter = creates.iter();
        let name_iter = name.iter();
        if !skip_inventory {
            let inventory_fn_name = syn::Ident::new(
                &format!("__pgx_internals_sql_{}", name.clone().expect("Unnamed extension_sql not supported yet.")),
                Span::call_site(),
            );
            let pg_finfo_fn_name = syn::Ident::new(
                &format!("pg_finfo_{}_wrapper", inventory_fn_name),
                Span::call_site(),
            );
            let inv = quote! {
                #[no_mangle]
                pub extern "C" fn  #pg_finfo_fn_name() -> &'static pg_sys::Pg_finfo_record {
                    const V1_API: pg_sys::Pg_finfo_record = pg_sys::Pg_finfo_record { api_version: 1 };
                    &V1_API
                }
    
                #[pgx::pg_guard]
                #[no_mangle]
                pub extern "C" fn  #inventory_fn_name(fcinfo: pgx::pg_sys::FunctionCallInfo) -> pgx::pg_sys::Datum {
                    let submission = pgx::pg_inventory::InventoryExtensionSql {
                        sql: include_str!(#path),
                        module_path: module_path!(),
                        full_path: concat!(file!(), ':', line!()),
                        file: file!(),
                        line: line!(),
                        name: None#( .unwrap_or(Some(#name_iter)) )*,
                        bootstrap: #bootstrap,
                        finalize: #finalize,
                        before: vec![#(#before_iter),*],
                        after: vec![#(#after_iter),*],
                        creates: vec![#(#creates_iter),*],
                    };
                    use pgx::IntoDatum;
                    return submission.into_datum().unwrap();
                }
            };
            tokens.append_all(inv);
        }
    }
}

/// A parsed `extension_sql!()` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a [`InventoryExtensionSql`].
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::pg_inventory::ExtensionSql;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: Macro = parse_quote! {
///     extension_sql!("-- Example content", bootstrap)
/// };
/// let inner_tokens = parsed.tokens;
/// let inner: ExtensionSql = parse_quote! {
///     #inner_tokens
/// };
/// let inventory_tokens = inner.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ExtensionSql {
    pub sql: LitStr,
    pub attrs: Punctuated<ExtensionSqlAttribute, Token![,]>,
}

impl ExtensionSql {
    pub fn inventory_fn_name(&self) -> String {
        let mut name = None;
        for attr in &self.attrs {
            match attr {
                ExtensionSqlAttribute::Name(found_name) => {
                    name = Some(found_name.value());
                },
                _ => (),
            }
        }
        name.expect("No name on extension_sql")
    }

    pub fn inventory(&self, inventory_dir: String) {
        create_dir_all(&inventory_dir).expect("Couldn't create inventory dir.");
        let mut fd = File::create(inventory_dir.to_string() + "/sql_" + &self.inventory_fn_name() + ".json").expect("Couldn't create inventory file");
        let inventory_fn_json = serde_json::to_string(&self.inventory_fn_name()).expect("Could not serialize inventory item.");
        write!(fd, "{}", inventory_fn_json).expect("Couldn't write to inventory file");
    }
}

impl Parse for ExtensionSql {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let sql = input.parse()?;
        let _after_sql_comma: Option<Token![,]> = input.parse()?;
        let attrs = input.parse_terminated(ExtensionSqlAttribute::parse)?;
        Ok(Self { sql, attrs })
    }
}

impl ToTokens for ExtensionSql {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let sql = &self.sql;
        let mut name = None;
        let mut bootstrap = false;
        let mut finalize = false;
        let mut skip_inventory = false;
        let mut before = vec![];
        let mut creates = vec![];
        let mut after = vec![];
        for attr in &self.attrs {
            match attr {
                ExtensionSqlAttribute::Before(items) => {
                    before.append(&mut items.iter().map(|x| x.to_token_stream()).collect());
                }
                ExtensionSqlAttribute::After(items) => {
                    after.append(&mut items.iter().map(|x| x.to_token_stream()).collect());
                }
                ExtensionSqlAttribute::Creates(items) => {
                    creates.append(&mut items.iter().map(|x| x.to_token_stream()).collect());
                }
                ExtensionSqlAttribute::Bootstrap => {
                    bootstrap = true;
                }
                ExtensionSqlAttribute::Finalize => {
                    finalize = true;
                }
                ExtensionSqlAttribute::SkipInventory => {
                    skip_inventory = true;
                }
                ExtensionSqlAttribute::Name(found_name) => {
                    name = Some(found_name.value());
                }
            }
        }
        let before_iter = before.iter();
        let after_iter = after.iter();
        let creates_iter = creates.iter();
        let name_iter = name.iter();
        if !skip_inventory {
            let inv = quote! {
                pgx::pg_inventory::inventory::submit! {
                    crate::__pgx_internals::ExtensionSql(pgx::pg_inventory::InventoryExtensionSql {
                        sql: #sql,
                        module_path: module_path!(),
                        full_path: concat!(file!(), ':', line!()),
                        file: file!(),
                        line: line!(),
                        name: None#( .unwrap_or(Some(#name_iter)) )*,
                        bootstrap: #bootstrap,
                        finalize: #finalize,
                        before: vec![#(#before_iter),*],
                        after: vec![#(#after_iter),*],
                        creates: vec![#(#creates_iter),*],
                    })
                }
            };
            tokens.append_all(inv);
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExtensionSqlAttribute {
    Before(Punctuated<ExtensionSqlPositioning, Token![,]>),
    After(Punctuated<ExtensionSqlPositioning, Token![,]>),
    Creates(Punctuated<SqlDeclaredEntity, Token![,]>),
    Bootstrap,
    Finalize,
    Name(LitStr),
    SkipInventory,
}

impl Parse for ExtensionSqlAttribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
            "creates" => {
                let _eq: syn::token::Eq = input.parse()?;
                let content;
                let _bracket = syn::bracketed!(content in input);
                Self::Creates(content.parse_terminated(SqlDeclaredEntity::parse)?)
            }
            "before" => {
                let _eq: syn::token::Eq = input.parse()?;
                let content;
                let _bracket = syn::bracketed!(content in input);
                Self::Before(content.parse_terminated(ExtensionSqlPositioning::parse)?)
            }
            "after" => {
                let _eq: syn::token::Eq = input.parse()?;
                let content;
                let _bracket = syn::bracketed!(content in input);
                Self::After(content.parse_terminated(ExtensionSqlPositioning::parse)?)
            }
            "bootstrap" => Self::Bootstrap,
            "finalize" => Self::Finalize,
            "name" => {
                let _eq: syn::token::Eq = input.parse()?;
                Self::Name(input.parse()?)
            }
            "skip_inventory" => Self::SkipInventory,
            _ => {
                return Err(syn::Error::new(
                    ident.span(),
                    "Unknown extension_sql attribute",
                ))
            }
        };
        Ok(found)
    }
}

#[derive(Debug, Clone)]
pub enum ExtensionSqlPositioning {
    Expr(syn::Expr),
    Name(syn::LitStr),
}

impl Parse for ExtensionSqlPositioning {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let maybe_litstr: Option<syn::LitStr> = input.parse()?;
        let found = if let Some(litstr) = maybe_litstr {
            Self::Name(litstr)
        } else {
            let path: syn::Expr = input.parse()?;
            Self::Expr(path)
        };
        Ok(found)
    }
}

impl ToTokens for ExtensionSqlPositioning {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let toks = match self {
            ExtensionSqlPositioning::Expr(ex) => {
                let path = ex.to_token_stream().to_string().replace(" ", "");
                (quote! {
                    pgx::pg_inventory::InventoryExtensionSqlPositioningRef::FullPath(#path)
                })
                .to_token_stream()
            }
            ExtensionSqlPositioning::Name(name) => quote! {
                pgx::pg_inventory::InventoryExtensionSqlPositioningRef::Name(#name)
            },
        };
        tokens.append_all(toks);
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum SqlDeclaredEntity {
    Type(String),
    Enum(String),
    Function(String),
}

impl Parse for SqlDeclaredEntity {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variant: Ident = input.parse()?;
        let content;
        let _bracket: syn::token::Paren = syn::parenthesized!(content in input);
        let identifier_path: syn::Path = content.parse()?;
        let identifier_str = {
            let mut identifier_segments = Vec::new();
            for segment in identifier_path.segments {
                identifier_segments.push(segment.ident.to_string())
            }
            identifier_segments.join("::")
        };
        let this = match variant.to_string().as_str() {
            "Type" => SqlDeclaredEntity::Type(identifier_str),
            "Enum" => SqlDeclaredEntity::Enum(identifier_str),
            "Function" => SqlDeclaredEntity::Function(identifier_str),
            _ => return Err(syn::Error::new(
                variant.span(),
                "SQL declared entities must be `Type(ident)`, `Enum(ident)`, or `Function(ident)`",
            )),
        };
        Ok(this)
    }
}

impl ToTokens for SqlDeclaredEntity {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (variant, identifier) = match &self {
            SqlDeclaredEntity::Type(val) => ("Type", val),
            SqlDeclaredEntity::Enum(val) => ("Enum", val),
            SqlDeclaredEntity::Function(val) => ("Function", val),
        };
        let identifier_split = identifier.split("::").collect::<Vec<_>>();
        let identifier = if identifier_split.len() == 1 {
            let identifier_infer = Ident::new(
                identifier_split.last().unwrap(),
                proc_macro2::Span::call_site(),
            );
            quote! { concat!(module_path!(), "::", stringify!(#identifier_infer)) }
        } else {
            quote! { stringify!(#identifier) }
        };
        let inv = quote! {
            pgx::pg_inventory::InventorySqlDeclaredEntity::build(#variant, #identifier).unwrap()
        };
        tokens.append_all(inv);
    }
}
