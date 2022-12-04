use crate::NameMacro;
use proc_macro2::TokenStream;

pub fn staticize_lifetimes_in_type_path(value: syn::TypePath) -> syn::TypePath {
    let mut ty = syn::Type::Path(value);
    staticize_lifetimes(&mut ty);
    match ty {
        syn::Type::Path(type_path) => type_path,

        // shouldn't happen
        _ => panic!("not a TypePath"),
    }
}

pub fn staticize_lifetimes(value: &mut syn::Type) {
    match value {
        syn::Type::Path(type_path) => {
            for segment in &mut type_path.path.segments {
                match &mut segment.arguments {
                    syn::PathArguments::AngleBracketed(bracketed) => {
                        for arg in &mut bracketed.args {
                            match arg {
                                // rename lifetimes to the static lifetime so the TypeIds match.
                                syn::GenericArgument::Lifetime(lifetime) => {
                                    lifetime.ident =
                                        syn::Ident::new("static", lifetime.ident.span());
                                }

                                // recurse
                                syn::GenericArgument::Type(ty) => staticize_lifetimes(ty),
                                syn::GenericArgument::Binding(binding) => {
                                    staticize_lifetimes(&mut binding.ty)
                                }
                                syn::GenericArgument::Constraint(constraint) => {
                                    for bound in constraint.bounds.iter_mut() {
                                        match bound {
                                            syn::TypeParamBound::Lifetime(lifetime) => {
                                                lifetime.ident =
                                                    syn::Ident::new("static", lifetime.ident.span())
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                // nothing to do otherwise
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        syn::Type::Reference(type_ref) => match &mut type_ref.lifetime {
            Some(ref mut lifetime) => {
                lifetime.ident = syn::Ident::new("static", lifetime.ident.span());
            }
            this @ None => *this = Some(syn::parse_quote!('static)),
        },

        syn::Type::Tuple(type_tuple) => {
            for elem in &mut type_tuple.elems {
                staticize_lifetimes(elem);
            }
        }

        syn::Type::Macro(type_macro) => {
            let mac = &type_macro.mac;
            if let Some(archetype) = mac.path.segments.last() {
                match archetype.ident.to_string().as_str() {
                    "name" => {
                        if let Ok(out) = mac.parse_body::<NameMacro>() {
                            // We don't particularly care what the identifier is, so we parse a
                            // raw TokenStream.  Specifically, it's okay for the identifier String,
                            // which we end up using as a Postgres column name, to be nearly any
                            // string, which can include Rust reserved words such as "type" or "match"
                            if let Ok(ident) = syn::parse_str::<TokenStream>(&out.ident) {
                                let mut ty = out.used_ty.resolved_ty;

                                // rewrite the name!() macro's type so that it has a static lifetime, if any
                                staticize_lifetimes(&mut ty);
                                type_macro.mac = syn::parse_quote! {name!(#ident, #ty)};
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub fn anonymize_lifetimes_in_type_path(value: syn::TypePath) -> syn::TypePath {
    let mut ty = syn::Type::Path(value);
    anonymize_lifetimes(&mut ty);
    match ty {
        syn::Type::Path(type_path) => type_path,

        // shouldn't happen
        _ => panic!("not a TypePath"),
    }
}

pub fn anonymize_lifetimes(value: &mut syn::Type) {
    match value {
        syn::Type::Path(type_path) => {
            for segment in &mut type_path.path.segments {
                match &mut segment.arguments {
                    syn::PathArguments::AngleBracketed(bracketed) => {
                        for arg in &mut bracketed.args {
                            match arg {
                                // rename lifetimes to the anonymous lifetime
                                syn::GenericArgument::Lifetime(lifetime) => {
                                    lifetime.ident = syn::Ident::new("_", lifetime.ident.span());
                                }

                                // recurse
                                syn::GenericArgument::Type(ty) => anonymize_lifetimes(ty),
                                syn::GenericArgument::Binding(binding) => {
                                    anonymize_lifetimes(&mut binding.ty)
                                }
                                syn::GenericArgument::Constraint(constraint) => {
                                    for bound in constraint.bounds.iter_mut() {
                                        match bound {
                                            syn::TypeParamBound::Lifetime(lifetime) => {
                                                lifetime.ident =
                                                    syn::Ident::new("_", lifetime.ident.span())
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                // nothing to do otherwise
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        syn::Type::Reference(type_ref) => {
            if let Some(lifetime) = type_ref.lifetime.as_mut() {
                lifetime.ident = syn::Ident::new("_", lifetime.ident.span());
            }
        }

        syn::Type::Tuple(type_tuple) => {
            for elem in &mut type_tuple.elems {
                anonymize_lifetimes(elem);
            }
        }

        _ => {}
    }
}
