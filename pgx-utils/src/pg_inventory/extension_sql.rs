use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr, Token,
};

use super::{DotIdentifier, SqlGraphEntity, ToSql};

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

impl Parse for ExtensionSqlFile {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let path = input.parse()?;
        let _after_sql_comma: Option<Token![,]> = input.parse()?;
        let attrs = input.parse_terminated(ExtensionSqlAttribute::parse)?;
        Ok(Self {
            path,
            attrs,
        })
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
        for attr in &self.attrs {
            match attr {
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
        let name_iter = name.iter();
        if !skip_inventory {
            let inv = quote! {
                pgx_utils::pg_inventory::inventory::submit! {
                    crate::__pgx_internals::ExtensionSql(pgx_utils::pg_inventory::InventoryExtensionSql {
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
                    })
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

impl Parse for ExtensionSql {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let sql = input.parse()?;
        let _after_sql_comma: Option<Token![,]> = input.parse()?;
        let attrs = input.parse_terminated(ExtensionSqlAttribute::parse)?;
        Ok(Self {
            sql,
            attrs,
        })
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
        let mut after = vec![];
        for attr in &self.attrs {
            match attr {
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
        let before_iter = before.iter();
        let after_iter = after.iter();
        let name_iter = name.iter();
        if !skip_inventory {
            let inv = quote! {
                pgx_utils::pg_inventory::inventory::submit! {
                    crate::__pgx_internals::ExtensionSql(pgx_utils::pg_inventory::InventoryExtensionSql {
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
    Bootstrap,
    Finalize,
    Name(LitStr),
    SkipInventory,
}

impl Parse for ExtensionSqlAttribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
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
                    ::pgx_utils::pg_inventory::InventoryExtensionSqlPositioningRef::FullPath(#path)
                })
                .to_token_stream()
            }
            ExtensionSqlPositioning::Name(name) => quote! {
                ::pgx_utils::pg_inventory::InventoryExtensionSqlPositioningRef::Name(#name)
            },
        };
        tokens.append_all(toks);
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InventoryExtensionSql {
    pub module_path: &'static str,
    pub full_path: &'static str,
    pub sql: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub name: Option<&'static str>,
    pub bootstrap: bool,
    pub finalize: bool,
    pub before: Vec<InventoryExtensionSqlPositioningRef<'static>>,
    pub after: Vec<InventoryExtensionSqlPositioningRef<'static>>,
}

impl InventoryExtensionSql {
    pub fn identifier(&self) -> &str {
        self.name.unwrap_or(self.full_path)
    }
}

impl<'a> Into<SqlGraphEntity<'a>> for &'a InventoryExtensionSql {
    fn into(self) -> SqlGraphEntity<'a> {
        SqlGraphEntity::CustomSql(self)
    }
}

impl DotIdentifier for InventoryExtensionSql {
    fn dot_identifier(&self) -> String {
        format!("schema {}", self.full_path.to_string())
    }
}

impl ToSql for InventoryExtensionSql {
    #[tracing::instrument(level = "debug", err, skip(self, _context))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!(
            "\n\
                -- {file}:{line}\n\
                {sql}\
                ",
            file = self.file,
            line = self.line,
            sql = self.sql,
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InventoryExtensionSqlPositioningRef<'a> {
    FullPath(&'a str),
    Name(&'a str),
}
