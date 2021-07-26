use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Ident;

use super::{DotFormat, SqlGraphEntity, ToSql};

#[derive(Debug, Clone)]
pub struct PostgresOrd {
    pub name: Ident,
}

impl PostgresOrd {
    pub fn new(name: Ident) -> Self {
        Self { name }
    }
}

impl ToTokens for PostgresOrd {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let inv = quote! {
            pgx_utils::pg_inventory::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PostgresOrd(pgx_utils::pg_inventory::InventoryPostgresOrd {
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

impl DotFormat for InventoryPostgresOrd {
    fn dot_format(&self) -> String {
        format!("ord {}", self.full_path.to_string())
    }
}

impl ToSql for InventoryPostgresOrd {
    #[tracing::instrument(level = "debug", err, skip(self, _context))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            -- {id:?}\n\
                            CREATE OPERATOR FAMILY {name}_btree_ops USING btree;\n\
                            CREATE OPERATOR CLASS {name}_btree_ops DEFAULT FOR TYPE {name} USING btree FAMILY {name}_btree_ops AS\n\
                                  \tOPERATOR 1 <,\n\
                                  \tOPERATOR 2 <=,\n\
                                  \tOPERATOR 3 =,\n\
                                  \tOPERATOR 4 >=,\n\
                                  \tOPERATOR 5 >,\n\
                                  \tFUNCTION 1 {name}_cmp({name}, {name});\n\
                            ",
                          name = self.name,
                          full_path = self.full_path,
                          file = self.file,
                          line = self.line,
                          id = self.id,
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}

