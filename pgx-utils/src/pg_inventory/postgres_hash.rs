use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Ident;

use super::{DotFormat, SqlGraphEntity, ToSql};

#[derive(Debug, Clone)]
pub struct PostgresHash {
    pub name: Ident,
}

impl PostgresHash {
    pub fn new(name: Ident) -> Self {
        Self { name }
    }
}

impl ToTokens for PostgresHash {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let inv = quote! {
            pgx_utils::pg_inventory::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PostgresHash(pgx_utils::pg_inventory::InventoryPostgresHash {
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
pub struct InventoryPostgresHash {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
}

impl<'a> Into<SqlGraphEntity<'a>> for &'a InventoryPostgresHash {
    fn into(self) -> SqlGraphEntity<'a> {
        SqlGraphEntity::Hash(self)
    }
}

impl DotFormat for InventoryPostgresHash {
    fn dot_format(&self) -> String {
        format!("hash {}", self.full_path.to_string())
    }
}

impl ToSql for InventoryPostgresHash {
    #[tracing::instrument(level = "debug", err, skip(self, _context))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            -- {id:?}\n\
                            CREATE OPERATOR FAMILY {name}_hash_ops USING hash;\n\
                            CREATE OPERATOR CLASS {name}_hash_ops DEFAULT FOR TYPE {name} USING hash FAMILY {name}_hash_ops AS\n\
                                \tOPERATOR    1   =  ({name}, {name}),\n\
                                \tFUNCTION    1   {name}_hash({name});\
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

