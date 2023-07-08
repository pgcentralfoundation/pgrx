/*!

`#[pg_trigger]` attribute related macro expansion for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgrx_sql_entity_graph] APIs, this is considered **internal**
to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::ToSqlConfig;
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::Token;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PgTriggerAttribute {
    Sql(ToSqlConfig),
}

impl Parse for PgTriggerAttribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: syn::Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
            "sql" => {
                use crate::pgrx_attribute::ArgValue;
                use syn::Lit;

                let _eq: Token![=] = input.parse()?;
                match input.parse::<ArgValue>()? {
                    ArgValue::Path(p) => Self::Sql(ToSqlConfig::from(p)),
                    ArgValue::Lit(Lit::Bool(b)) => Self::Sql(ToSqlConfig::from(b.value)),
                    ArgValue::Lit(Lit::Str(s)) => Self::Sql(ToSqlConfig::from(s)),
                    ArgValue::Lit(other) => {
                        return Err(syn::Error::new(
                            other.span(),
                            "expected boolean, path, or string literal",
                        ))
                    }
                }
            }
            e => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    format!(
                        "Invalid option `{}` inside `{} {}`",
                        e,
                        ident.to_string(),
                        input.to_string()
                    ),
                ))
            }
        };
        Ok(found)
    }
}
