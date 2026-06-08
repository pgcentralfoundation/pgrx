//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
/*!

`#[pg_extern]` related attributes for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate] APIs, this is considered **internal**
> to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::extern_args::ExternArgs;
use crate::positioning_ref::PositioningRef;
use crate::to_sql::ToSqlConfig;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::Token;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Attribute {
    Immutable,
    Strict,
    Stable,
    Volatile,
    Raw,
    NoGuard,
    CreateOrReplace,
    SecurityDefiner,
    SecurityInvoker,
    ParallelSafe,
    ParallelUnsafe,
    ParallelRestricted,
    ShouldPanic(syn::LitStr),
    Schema(syn::LitStr),
    Support(PositioningRef),
    Name(syn::LitStr),
    Cost(Box<syn::Expr>),
    Requires(Punctuated<PositioningRef, Token![,]>),
    Sql(ToSqlConfig),
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = match self {
            Self::Immutable => quote! { immutable },
            Self::Strict => quote! { strict },
            Self::Stable => quote! { stable },
            Self::Volatile => quote! { volatile },
            Self::Raw => quote! { raw },
            Self::NoGuard => quote! { no_guard },
            Self::CreateOrReplace => quote! { create_or_replace },
            Self::SecurityDefiner => {
                quote! {security_definer}
            }
            Self::SecurityInvoker => {
                quote! {security_invoker}
            }
            Self::ParallelSafe => {
                quote! { parallel_safe }
            }
            Self::ParallelUnsafe => {
                quote! { parallel_unsafe }
            }
            Self::ParallelRestricted => {
                quote! { parallel_restricted }
            }
            Self::ShouldPanic(s) => {
                quote! { expected = #s }
            }
            Self::Schema(s) => {
                quote! { schema = #s }
            }
            Self::Support(item) => {
                quote! { support = #item }
            }
            Self::Name(s) => {
                quote! { name = #s }
            }
            Self::Cost(s) => {
                quote! { cost = #s }
            }
            Self::Requires(items) => {
                let items_iter = items.iter().map(|x| x.to_token_stream()).collect::<Vec<_>>();
                quote! { requires = [#(#items_iter),*] }
            }
            // This attribute is handled separately
            Self::Sql(to_sql_config) => {
                quote! { sql = #to_sql_config }
            }
        };
        tokens.append_all(quoted);
    }
}

impl Attribute {
    /// Convert this attribute into an [`ExternArgs`] for SQL emission.
    ///
    /// Returns `None` for attributes (currently only [`Attribute::Sql`]) that are handled outside the extern-args pipeline.
    pub fn as_extern_arg(&self) -> Option<ExternArgs> {
        Some(match self {
            Self::CreateOrReplace => ExternArgs::CreateOrReplace,
            Self::Immutable => ExternArgs::Immutable,
            Self::Strict => ExternArgs::Strict,
            Self::Stable => ExternArgs::Stable,
            Self::Volatile => ExternArgs::Volatile,
            Self::Raw => ExternArgs::Raw,
            Self::NoGuard => ExternArgs::NoGuard,
            Self::SecurityDefiner => ExternArgs::SecurityDefiner,
            Self::SecurityInvoker => ExternArgs::SecurityInvoker,
            Self::ParallelSafe => ExternArgs::ParallelSafe,
            Self::ParallelUnsafe => ExternArgs::ParallelUnsafe,
            Self::ParallelRestricted => ExternArgs::ParallelRestricted,
            Self::ShouldPanic(v) => ExternArgs::ShouldPanic(v.value()),
            Self::Schema(v) => ExternArgs::Schema(v.value()),
            Self::Support(v) => ExternArgs::Support(v.clone()),
            Self::Name(v) => ExternArgs::Name(v.value()),
            Self::Cost(v) => ExternArgs::Cost(v.to_token_stream().to_string()),
            Self::Requires(items) => ExternArgs::Requires(items.iter().cloned().collect()),
            Self::Sql(_) => return None,
        })
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: syn::Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
            "immutable" => Self::Immutable,
            "strict" => Self::Strict,
            "stable" => Self::Stable,
            "volatile" => Self::Volatile,
            "raw" => Self::Raw,
            "no_guard" => Self::NoGuard,
            "create_or_replace" => Self::CreateOrReplace,
            "security_definer" => Self::SecurityDefiner,
            "security_invoker" => Self::SecurityInvoker,
            "parallel_safe" => Self::ParallelSafe,
            "parallel_unsafe" => Self::ParallelUnsafe,
            "parallel_restricted" => Self::ParallelRestricted,
            "error" | "expected" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::LitStr = input.parse()?;
                Self::ShouldPanic(literal)
            }
            "schema" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::LitStr = input.parse()?;
                Self::Schema(literal)
            }
            "support" => {
                let _eq: Token![=] = input.parse()?;
                let item: PositioningRef = input.parse()?;
                Self::Support(item)
            }
            "name" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::LitStr = input.parse()?;
                Self::Name(literal)
            }
            "cost" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::Expr = input.parse()?;
                Self::Cost(Box::new(literal))
            }
            "requires" => {
                let _eq: syn::token::Eq = input.parse()?;
                let content;
                let _bracket = syn::bracketed!(content in input);
                Self::Requires(content.parse_terminated(PositioningRef::parse, Token![,])?)
            }
            "sql" => {
                use crate::pgrx_attribute::ArgValue;
                use syn::Lit;

                let _eq: Token![=] = input.parse()?;
                match input.parse::<ArgValue>()? {
                    ArgValue::Path(path) => {
                        return Err(syn::Error::new(
                            path.span(),
                            "expected boolean or string literal",
                        ));
                    }
                    ArgValue::Lit(Lit::Bool(b)) => Self::Sql(ToSqlConfig::from(b.value)),
                    ArgValue::Lit(Lit::Str(s)) => Self::Sql(ToSqlConfig::from(s)),
                    ArgValue::Lit(other) => {
                        // FIXME: add a ui test for this
                        return Err(syn::Error::new(
                            other.span(),
                            "expected boolean or string literal",
                        ));
                    }
                }
            }
            e => {
                // FIXME: add a UI test for this
                return Err(syn::Error::new(
                    ident.span(),
                    format!("Invalid option `{e}` inside `{ident} {input}`"),
                ));
            }
        };
        Ok(found)
    }
}

#[cfg(test)]
mod tests {

    use super::Attribute;
    use std::str::FromStr;
    use syn::parse::Parser;
    use syn::punctuated::Punctuated;

    fn parse(src: &str) -> Punctuated<Attribute, syn::Token![,]> {
        let ts = proc_macro2::TokenStream::from_str(src).expect("tokenize");
        Punctuated::<Attribute, syn::Token![,]>::parse_terminated.parse2(ts).expect("parse")
    }

    fn expected_value(attrs: &Punctuated<Attribute, syn::Token![,]>) -> Option<String> {
        attrs.iter().find_map(|a| match a {
            Attribute::ShouldPanic(lit) => Some(lit.value()),
            _ => None,
        })
    }

    #[test]
    fn plain_string_expected() {
        let attrs = parse(r#"expected = "syntax error""#);
        assert_eq!(expected_value(&attrs).as_deref(), Some("syntax error"));
    }

    #[test]
    fn escaped_quotes_in_plain_string() {
        let attrs = parse(r#"expected = "syntax error at or near \"THIS\"""#);
        assert_eq!(expected_value(&attrs).as_deref(), Some(r#"syntax error at or near "THIS""#),);
    }

    #[test]
    fn raw_string_with_embedded_quotes() {
        // The bug we are pinning: the old walker would have produced `#"foo "bar""#` (raw-string delimiters leaking into the value).
        let attrs = parse(r###"expected = r#"foo "bar""#"###);
        assert_eq!(expected_value(&attrs).as_deref(), Some(r#"foo "bar""#));
    }

    #[test]
    fn raw_string_with_nested_hashes() {
        let attrs = parse(r####"expected = r##"weird"#text"##"####);
        assert_eq!(expected_value(&attrs).as_deref(), Some(r##"weird"#text"##));
    }

    #[test]
    fn error_alias_works_like_expected() {
        let attrs = parse(r#"error = "boom""#);
        assert_eq!(expected_value(&attrs).as_deref(), Some("boom"));
    }

    #[test]
    fn other_attrs_alongside_expected_do_not_interfere() {
        let attrs = parse(r#"immutable, expected = "ok", strict"#);
        assert_eq!(expected_value(&attrs).as_deref(), Some("ok"));
        assert!(attrs.iter().any(|a| matches!(a, Attribute::Immutable)));
        assert!(attrs.iter().any(|a| matches!(a, Attribute::Strict)));
    }

    #[test]
    fn malformed_input_is_a_syn_error_not_a_panic() {
        let ts = proc_macro2::TokenStream::from_str("expected").expect("tokenize");
        let result = Punctuated::<Attribute, syn::Token![,]>::parse_terminated.parse2(ts);
        assert!(result.is_err(), "expected = is required");
    }
}
