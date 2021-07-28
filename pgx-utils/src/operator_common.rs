use proc_macro2::Ident;
use quote::quote;

pub fn eq(type_name: &Ident, skip_inventory: bool) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(
        &format!("{}_eq", type_name).to_lowercase(),
        type_name.span(),
    );
    let maybe_skip_inventory_attr = if skip_inventory {
        quote! { , skip_inventory }
    } else {
        quote! {}
    };
    quote! {
        #[allow(non_snake_case)]
        #[pg_operator(immutable, parallel_safe #maybe_skip_inventory_attr)]
        #[opname(=)]
        #[negator(<>)]
        #[restrict(eqsel)]
        #[join(eqjoinsel)]
        #[merges]
        #[hashes]
        fn #pg_name(left: #type_name, right: #type_name) -> bool {
            left == right
        }
    }
}

pub fn ne(type_name: &Ident, skip_inventory: bool) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(
        &format!("{}_ne", type_name).to_lowercase(),
        type_name.span(),
    );
    let maybe_skip_inventory_attr = if skip_inventory {
        quote! { , skip_inventory }
    } else {
        quote! {}
    };
    quote! {
        #[allow(non_snake_case)]
        #[pg_operator(immutable, parallel_safe #maybe_skip_inventory_attr)]
        #[opname(<>)]
        #[negator(=)]
        #[restrict(neqsel)]
        #[join(neqjoinsel)]
        fn #pg_name(left: #type_name, right: #type_name) -> bool {
            left != right
        }
    }
}

pub fn lt(type_name: &Ident, skip_inventory: bool) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(
        &format!("{}_lt", type_name).to_lowercase(),
        type_name.span(),
    );
    let maybe_skip_inventory_attr = if skip_inventory {
        quote! { , skip_inventory }
    } else {
        quote! {}
    };
    quote! {
        #[allow(non_snake_case)]
        #[pg_operator(immutable, parallel_safe #maybe_skip_inventory_attr)]
        #[opname(<)]
        #[negator(>=)]
        #[commutator(>)]
        #[restrict(scalarltsel)]
        #[join(scalarltjoinsel)]
        fn #pg_name(left: #type_name, right: #type_name) -> bool {
            left < right
        }

    }
}

pub fn gt(type_name: &Ident, skip_inventory: bool) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(
        &format!("{}_gt", type_name).to_lowercase(),
        type_name.span(),
    );
    let maybe_skip_inventory_attr = if skip_inventory {
        quote! { , skip_inventory }
    } else {
        quote! {}
    };
    quote! {
        #[allow(non_snake_case)]
        #[pg_operator(immutable, parallel_safe #maybe_skip_inventory_attr)]
        #[opname(>)]
        #[negator(<=)]
        #[commutator(<)]
        #[restrict(scalargtsel)]
        #[join(scalargtjoinsel)]
        fn #pg_name(left: #type_name, right: #type_name) -> bool {
            left > right
        }
    }
}

pub fn le(type_name: &Ident, skip_inventory: bool) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(
        &format!("{}_le", type_name).to_lowercase(),
        type_name.span(),
    );
    let maybe_skip_inventory_attr = if skip_inventory {
        quote! { , skip_inventory }
    } else {
        quote! {}
    };
    quote! {
        #[allow(non_snake_case)]
        #[pg_operator(immutable, parallel_safe #maybe_skip_inventory_attr)]
        #[opname(<=)]
        #[negator(>)]
        #[commutator(>=)]
        #[restrict(scalarlesel)]
        #[join(scalarlejoinsel)]
        fn #pg_name(left: #type_name, right: #type_name) -> bool {
            left <= right
        }
    }
}

pub fn ge(type_name: &Ident, skip_inventory: bool) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(
        &format!("{}_ge", type_name).to_lowercase(),
        type_name.span(),
    );
    let maybe_skip_inventory_attr = if skip_inventory {
        quote! { , skip_inventory }
    } else {
        quote! {}
    };
    quote! {
        #[allow(non_snake_case)]
        #[pg_operator(immutable, parallel_safe #maybe_skip_inventory_attr)]
        #[opname(>=)]
        #[negator(<)]
        #[commutator(<=)]
        #[restrict(scalargesel)]
        #[join(scalargejoinsel)]
        fn #pg_name(left: #type_name, right: #type_name) -> bool {
            left >= right
        }
    }
}

pub fn cmp(type_name: &Ident, skip_inventory: bool) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(
        &format!("{}_cmp", type_name).to_lowercase(),
        type_name.span(),
    );
    let maybe_skip_inventory_attr = if skip_inventory {
        quote! { , skip_inventory }
    } else {
        quote! {}
    };
    quote! {
        #[allow(non_snake_case)]
        #[pg_extern(immutable, parallel_safe #maybe_skip_inventory_attr)]
        fn #pg_name(left: #type_name, right: #type_name) -> i32 {
            left.cmp(&right) as i32
        }
    }
}

pub fn hash(type_name: &Ident, skip_inventory: bool) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(
        &format!("{}_hash", type_name).to_lowercase(),
        type_name.span(),
    );
    let maybe_skip_inventory_attr = if skip_inventory {
        quote! { , skip_inventory }
    } else {
        quote! {}
    };
    quote! {
        #[allow(non_snake_case)]
        #[pg_extern(immutable, parallel_safe #maybe_skip_inventory_attr)]
        fn #pg_name(value: #type_name) -> i32 {
            pgx::misc::pgx_seahash(&value) as i32
        }
    }
}
