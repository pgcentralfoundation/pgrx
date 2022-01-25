mod argument;
mod attribute;
mod operator;
mod returning;
mod search_path;

pub use argument::Argument;
use attribute::{Attribute, PgxAttributes};
pub use operator::PgOperator;
use operator::{PgxOperatorAttributeWithIdent, PgxOperatorOpName};
use returning::Returning;
pub(crate) use returning::NameMacro;
use search_path::SearchPathList;

use eyre::WrapErr;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::Meta;

/// A parsed `#[pg_extern]` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a `pgx::datum::sql_entity_graph::InventoryPgExtern`.
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::sql_entity_graph::PgExtern;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PgExtern = parse_quote! {
///     fn example(x: Option<str>) -> Option<&'a str> {
///         unimplemented!()
///     }
/// };
/// let sql_graph_entity_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PgExtern {
    attrs: Option<PgxAttributes>,
    attr_tokens: proc_macro2::TokenStream,
    func: syn::ItemFn,
}

impl PgExtern {
    fn name(&self) -> String {
        self.attrs
            .as_ref()
            .and_then(|a| {
                a.attrs.iter().find_map(|candidate| match candidate {
                    Attribute::Name(name) => Some(name.value()),
                    _ => None,
                })
            })
            .unwrap_or_else(|| self.func.sig.ident.to_string())
    }

    fn schema(&self) -> Option<String> {
        self.attrs.as_ref().and_then(|a| {
            a.attrs.iter().find_map(|candidate| match candidate {
                Attribute::Schema(name) => Some(name.value()),
                _ => None,
            })
        })
    }

    fn extern_attrs(&self) -> Option<&PgxAttributes> {
        self.attrs.as_ref()
    }

    pub fn extern_attr_tokens(&self) -> &proc_macro2::TokenStream {
        &self.attr_tokens
    }

    fn overridden(&self) -> Option<String> {
        let mut retval = None;
        let mut in_commented_sql_block = false;
        for attr in &self.func.attrs {
            let meta = attr.parse_meta().ok();
            if let Some(meta) = meta {
                if meta.path().is_ident("doc") {
                    let content = match meta {
                        Meta::Path(_) | Meta::List(_) => continue,
                        Meta::NameValue(mnv) => mnv,
                    };
                    if let syn::Lit::Str(inner) = content.lit {
                        if !in_commented_sql_block && inner.value().trim() == "```pgxsql" {
                            in_commented_sql_block = true;
                        } else if in_commented_sql_block && inner.value().trim() == "```" {
                            in_commented_sql_block = false;
                        } else if in_commented_sql_block {
                            let sql = retval.get_or_insert_with(String::default);
                            let line = inner.value().trim_start().replace(
                                "@FUNCTION_NAME@",
                                &*(self.func.sig.ident.to_string() + "_wrapper"),
                            ) + "\n";
                            sql.push_str(&*line);
                        }
                    }
                }
            }
        }
        retval
    }

    fn operator(&self) -> Option<PgOperator> {
        let mut skel = Option::<PgOperator>::default();
        for attr in &self.func.attrs {
            let last_segment = attr.path.segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                "opname" => {
                    let attr: PgxOperatorOpName = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .opname
                        .get_or_insert(attr);
                }
                "commutator" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .commutator
                        .get_or_insert(attr);
                }
                "negator" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .negator
                        .get_or_insert(attr);
                }
                "join" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .join
                        .get_or_insert(attr);
                }
                "restrict" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .restrict
                        .get_or_insert(attr);
                }
                "hashes" => {
                    skel.get_or_insert_with(Default::default).hashes = true;
                }
                "merges" => {
                    skel.get_or_insert_with(Default::default).merges = true;
                }
                _ => (),
            }
        }
        skel
    }

    fn search_path(&self) -> Option<SearchPathList> {
        self.func
            .attrs
            .iter()
            .find(|f| {
                f.path
                    .segments
                    .first()
                    .map(|f| f.ident == Ident::new("search_path", Span::call_site()))
                    .unwrap_or_default()
            })
            .and_then(|attr| Some(attr.parse_args::<SearchPathList>().unwrap()))
    }

    fn inputs(&self) -> eyre::Result<Vec<Argument>> {
        let mut args = Vec::default();
        for input in &self.func.sig.inputs {
            let arg = Argument::build(input.clone())
                .wrap_err_with(|| format!("Could not map {:?}", input))?;
            if let Some(arg) = arg {
                args.push(arg);
            }
        }
        Ok(args)
    }

    fn returns(&self) -> Result<Returning, eyre::Error> {
        Returning::try_from(&self.func.sig.output)
    }

    pub fn new(attr: TokenStream2, item: TokenStream2) -> Result<Self, syn::Error> {
        let attrs = syn::parse2::<PgxAttributes>(attr.clone()).ok();
        let func = syn::parse2::<syn::ItemFn>(item)?;
        Ok(Self {
            attrs: attrs,
            attr_tokens: attr,
            func: func,
        })
    }
}

impl ToTokens for PgExtern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.func.sig.ident;
        let name = self.name();
        let schema = self.schema();
        let schema_iter = schema.iter();
        let extern_attrs = self.extern_attrs();
        let search_path = self.search_path().into_iter();
        let inputs = self.inputs().unwrap();
        let returns = match self.returns() {
            Ok(returns) => returns,
            Err(e) => {
                let msg = e.to_string();
                tokens.append_all(quote! {
                    std::compile_error!(#msg);
                });
                return;
            }
        };
        let operator = self.operator().into_iter();
        let overridden = self.overridden().into_iter();

        let sql_graph_entity_fn_name =
            syn::Ident::new(&format!("__pgx_internals_fn_{}", ident), Span::call_site());
        let inv = quote! {
            #[no_mangle]
            pub extern "C" fn  #sql_graph_entity_fn_name() -> pgx::datum::sql_entity_graph::SqlGraphEntity {
                use core::any::TypeId;
                let submission = pgx::datum::sql_entity_graph::PgExternEntity {
                    name: #name,
                    unaliased_name: stringify!(#ident),
                    schema: None#( .unwrap_or(Some(#schema_iter)) )*,
                    file: file!(),
                    line: line!(),
                    module_path: core::module_path!(),
                    full_path: concat!(core::module_path!(), "::", stringify!(#ident)),
                    extern_attrs: #extern_attrs,
                    search_path: None#( .unwrap_or(Some(vec![#search_path])) )*,
                    fn_args: vec![#(#inputs),*],
                    fn_return: #returns,
                    operator: None#( .unwrap_or(Some(#operator)) )*,
                    overridden: None#( .unwrap_or(Some(#overridden)) )*,
                };
                pgx::datum::sql_entity_graph::SqlGraphEntity::Function(submission)
            }
        };
        tokens.append_all(inv);
    }
}

impl Parse for PgExtern {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let attrs: Option<PgxAttributes> = input.parse().ok();
        let func = input.parse()?;
        let attr_tokens: proc_macro2::TokenStream = attrs.clone().into_token_stream();
        Ok(Self {
            attrs,
            attr_tokens,
            func,
        })
    }
}
