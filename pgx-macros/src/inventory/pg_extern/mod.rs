mod attribute;
mod argument;
mod returning;
mod search_path;
mod operator;

use attribute::PgxAttributes;
use argument::Argument;
use returning::Returning;
use search_path::SearchPathList;
use operator::{PgxOperator, PgxOperatorAttributeWithIdent, PgxOperatorOpName};

use syn::parse::{Parse, ParseStream};
use quote::{ToTokens, quote, TokenStreamExt};
use proc_macro2::{TokenStream as TokenStream2, Span, Ident};
use proc_macro::TokenStream;
use std::convert::TryFrom;
use syn::Meta;

#[derive(Debug)]
pub struct PgxExtern {
    attrs: PgxAttributes,
    func: syn::ItemFn,
}

impl PgxExtern {
    fn extern_attrs(&self) -> &PgxAttributes {
        &self.attrs
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
                        if !in_commented_sql_block && inner.value().trim() == "```sql" {
                            in_commented_sql_block = true;
                        } else if in_commented_sql_block && inner.value().trim() == "```" {
                            in_commented_sql_block = false;
                        } else if in_commented_sql_block {
                            let sql = retval.get_or_insert_with(String::default);
                            sql.push_str(&inner.value().trim_start());
                        }
                    }
                }
            }
        }
        retval
    }

    fn operator(&self) -> Option<PgxOperator> {
        let mut skel = Option::<PgxOperator>::default();
        for attr in &self.func.attrs {
            let last_segment = attr.path.segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                "opname" => {
                    let attr: PgxOperatorOpName = syn::parse2(attr.tokens.clone()).expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default).opname.get_or_insert(attr);
                }
                "commutator" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone()).expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default).commutator.get_or_insert(attr);
                },
                "negator" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone()).expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default).negator.get_or_insert(attr);
                },
                "restrict" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone()).expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default).restrict.get_or_insert(attr);
                },
                "hashes" => {
                    skel.get_or_insert_with(Default::default).hashes = true;
                },
                "merges" => {
                    skel.get_or_insert_with(Default::default).merges = true;
                },
                _ => (),
            }
        }
        skel
    }

    fn search_path(&self) -> Option<SearchPathList> {
        self.func.attrs.iter().find(|f| {
            f.path.segments.first().map(|f| {
                f.ident == Ident::new("search_path", Span::call_site())
            }).unwrap_or_default()
        }).and_then(|attr| {
            Some(attr.parse_args::<SearchPathList>().unwrap())
        })
    }

    fn inputs(&self) -> Vec<Argument> {
        self.func.sig.inputs.iter().flat_map(|input| {
            Argument::try_from(input.clone()).ok()
        }).collect()
    }

    fn returns(&self) -> Returning {
        Returning::try_from(&self.func.sig.output).unwrap()
    }


    pub(crate) fn new(attr: TokenStream, item: TokenStream) -> Result<Self, syn::Error> {
        let attrs = syn::parse::<PgxAttributes>(attr)?;
        let func = syn::parse::<syn::ItemFn>(item)?;
        Ok(Self {
            attrs,
            func,
        })
    }
}


impl ToTokens for PgxExtern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.func.sig.ident;
        let extern_attrs = self.extern_attrs();
        let search_path = self.search_path().into_iter();
        let inputs = self.inputs();
        let returns = self.returns();
        let operator = self.operator().into_iter();
        let overridden = self.overridden().into_iter();

        let inv = quote! {
            pgx::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PgxExtern {
                    name: stringify!(#ident),
                    file: file!(),
                    line: line!(),
                    module_path: core::module_path!(),
                    extern_attrs: #extern_attrs,
                    search_path: None#( .unwrap_or(Some(vec![#search_path])) )*,
                    fn_args: vec![#(#inputs),*],
                    fn_return: #returns,
                    operator: None#( .unwrap_or(Some(#operator)) )*,
                    overridden: None#( .unwrap_or(Some(#overridden)) )*,
                }
            }
        };
        tokens.append_all(inv);
    }
}

impl Parse for PgxExtern {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse()?,
            func: input.parse()?,
        })
    }
}
