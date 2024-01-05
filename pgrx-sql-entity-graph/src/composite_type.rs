#[derive(Debug, Clone)]
pub struct CompositeTypeMacro {
    #[allow(dead_code)]
    pub(crate) lifetime: Option<syn::Lifetime>,
    pub(crate) expr: syn::Expr,
}

impl syn::parse::Parse for CompositeTypeMacro {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let lifetime: Option<syn::Lifetime> = input.parse().ok();
        let _comma: Option<syn::Token![,]> = input.parse().ok();
        let expr = input.parse()?;
        Ok(Self { lifetime, expr })
    }
}

pub fn handle_composite_type_macro(mac: &syn::Macro) -> syn::Result<CompositeTypeMacro> {
    let out: CompositeTypeMacro = mac.parse_body()?;
    Ok(out)
}

impl CompositeTypeMacro {
    /// Expands into the appropriate type, explicitly eliding the lifetime
    /// if none is actually given.
    pub fn expand_with_lifetime(&self) -> syn::Type {
        let CompositeTypeMacro { lifetime, .. } = self;
        let lifetime = lifetime.clone().unwrap_or_else(|| syn::Lifetime::new("'_", proc_macro2::Span::call_site()));
        syn::parse_quote! {
            ::pgrx::heap_tuple::PgHeapTuple<#lifetime, ::pgrx::pgbox::AllocatedByRust>
        }
    }
}
