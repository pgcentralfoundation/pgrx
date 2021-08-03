use std::hash::{Hash, Hasher};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Generics, ItemEnum,
};
use syn::{punctuated::Punctuated, Ident, Token};

use super::{DotIdentifier, SqlGraphEntity, ToSql};

/// A parsed `#[derive(PostgresEnum)]` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a [`InventoryPostgresEnum`].
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::pg_inventory::PostgresEnum;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PostgresEnum = parse_quote! {
///     #[derive(PostgresEnum)]
///     enum Demo {
///         Example,
///     }
/// };
/// let inventory_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PostgresEnum {
    name: Ident,
    generics: Generics,
    variants: Punctuated<syn::Variant, Token![,]>,
}

impl PostgresEnum {
    pub fn new(
        name: Ident,
        generics: Generics,
        variants: Punctuated<syn::Variant, Token![,]>,
    ) -> Self {
        Self {
            name,
            generics,
            variants,
        }
    }

    pub fn from_derive_input(derive_input: DeriveInput) -> Result<Self, syn::Error> {
        let data_enum = match derive_input.data {
            syn::Data::Enum(data_enum) => data_enum,
            syn::Data::Union(_) | syn::Data::Struct(_) => {
                return Err(syn::Error::new(derive_input.ident.span(), "expected enum"))
            }
        };
        Ok(Self::new(
            derive_input.ident,
            derive_input.generics,
            data_enum.variants,
        ))
    }
}

impl Parse for PostgresEnum {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let parsed: ItemEnum = input.parse()?;
        Ok(Self::new(parsed.ident, parsed.generics, parsed.variants))
    }
}

impl ToTokens for PostgresEnum {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // It's important we remap all lifetimes we spot to `'static` so they can be used during inventory submission.
        let name = self.name.clone();
        let mut static_generics = self.generics.clone();
        for lifetime in static_generics.lifetimes_mut() {
            lifetime.lifetime.ident = Ident::new("static", Span::call_site());
        }
        let (_impl_generics, ty_generics, _where_clauses) = static_generics.split_for_impl();

        let variants = self.variants.iter();
        let inv = quote! {
            pgx_utils::pg_inventory::inventory::submit! {
                let mut mappings = Default::default();
                <#name #ty_generics as ::pgx::datum::WithTypeIds>::register_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithSizedTypeIds::<#name #ty_generics>::register_sized_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithArrayTypeIds::<#name #ty_generics>::register_array_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithVarlenaTypeIds::<#name #ty_generics>::register_varlena_with_refs(&mut mappings, stringify!(#name).to_string());

                crate::__pgx_internals::PostgresEnum(pgx_utils::pg_inventory::InventoryPostgresEnum {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name #ty_generics>(),
                    id: *<#name #ty_generics as WithTypeIds>::ITEM_ID,
                    mappings,
                    variants: vec![ #(  stringify!(#variants)  ),* ],
                })
            }
        };
        tokens.append_all(inv);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryPostgresEnum {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
    pub mappings: std::collections::HashSet<super::RustSqlMapping>,
    pub variants: Vec<&'static str>,
}

impl Hash for InventoryPostgresEnum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for InventoryPostgresEnum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for InventoryPostgresEnum {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl InventoryPostgresEnum {
    pub fn id_matches(&self, candidate: &core::any::TypeId) -> bool {
        self.mappings.iter().any(|tester| *candidate == tester.id)
    }
}

impl<'a> Into<SqlGraphEntity<'a>> for &'a InventoryPostgresEnum {
    fn into(self) -> SqlGraphEntity<'a> {
        SqlGraphEntity::Enum(self)
    }
}

impl DotIdentifier for InventoryPostgresEnum {
    fn dot_identifier(&self) -> String {
        format!("enum {}", self.full_path.to_string())
    }
}

impl ToSql for InventoryPostgresEnum {
    #[tracing::instrument(level = "debug", err, skip(self, context))]
    fn to_sql(&self, context: &super::PgxSql) -> eyre::Result<String> {
        let self_index = context.enums[self];
        let sql = format!(
            "\n\
                    -- {file}:{line}\n\
                    -- {full_path}\n\
                    CREATE TYPE {schema}{name} AS ENUM (\n\
                        {variants}\
                    );\n\
                ",
            schema = context.schema_prefix_for(&self_index),
            full_path = self.full_path,
            file = self.file,
            line = self.line,
            name = self.name,
            variants = self
                .variants
                .iter()
                .map(|variant| format!("\t'{}'", variant))
                .collect::<Vec<_>>()
                .join(",\n")
                + "\n",
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}
