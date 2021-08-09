use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Ident, ItemEnum, ItemStruct,
};

use super::{DotIdentifier, SqlGraphEntity, ToSql};

/// A parsed `#[derive(PostgresOrd)]` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a [`InventoryPostgresOrd`].
///
/// On structs:
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::pg_inventory::PostgresOrd;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PostgresOrd = parse_quote! {
///     #[derive(PostgresOrd)]
///     struct Example<'a> {
///         demo: &'a str,
///     }
/// };
/// let inventory_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
///
/// On enums:
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::pg_inventory::PostgresOrd;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PostgresOrd = parse_quote! {
///     #[derive(PostgresOrd)]
///     enum Demo {
///         Example,
///     }
/// };
/// let inventory_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PostgresOrd {
    pub name: Ident,
}

impl PostgresOrd {
    pub fn new(name: Ident) -> Self {
        Self { name }
    }

    pub fn from_derive_input(derive_input: DeriveInput) -> Result<Self, syn::Error> {
        Ok(Self::new(derive_input.ident))
    }
}

impl Parse for PostgresOrd {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let parsed_enum: Result<ItemEnum, syn::Error> = input.parse();
        let parsed_struct: Result<ItemStruct, syn::Error> = input.parse();
        let ident = parsed_enum
            .map(|x| x.ident)
            .or_else(|_| parsed_struct.map(|x| x.ident))
            .map_err(|_| syn::Error::new(input.span(), "expected enum or struct"))?;
        Ok(Self::new(ident))
    }
}

impl ToTokens for PostgresOrd {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let inv = quote! {
            pgx::pg_inventory::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PostgresOrd(pgx::pg_inventory::InventoryPostgresOrd {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    full_path: core::any::type_name::<#name>(),
                    module_path: module_path!(),
                    id: TypeId::of::<#name>(),
                })
            }
        };
        tokens.append_all(inv);
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InventoryPostgresOrd {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
}

impl<'a> Into<SqlGraphEntity<'a>> for &'a InventoryPostgresOrd {
    fn into(self) -> SqlGraphEntity<'a> {
        SqlGraphEntity::Ord(self)
    }
}

impl DotIdentifier for InventoryPostgresOrd {
    fn dot_identifier(&self) -> String {
        format!("ord {}", self.full_path.to_string())
    }
}

impl ToSql for InventoryPostgresOrd {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = self.full_path))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            CREATE OPERATOR FAMILY {name}_btree_ops USING btree;\n\
                            CREATE OPERATOR CLASS {name}_btree_ops DEFAULT FOR TYPE {name} USING btree FAMILY {name}_btree_ops AS\n\
                                  \tOPERATOR 1 <,\n\
                                  \tOPERATOR 2 <=,\n\
                                  \tOPERATOR 3 =,\n\
                                  \tOPERATOR 4 >=,\n\
                                  \tOPERATOR 5 >,\n\
                                  \tFUNCTION 1 {name}_cmp({name}, {name});\
                            ",
                          name = self.name,
                          full_path = self.full_path,
                          file = self.file,
                          line = self.line,
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}
