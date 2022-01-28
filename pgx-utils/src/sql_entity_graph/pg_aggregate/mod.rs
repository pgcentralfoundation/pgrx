mod aggregate_type;
mod maybe_variadic_type;

use aggregate_type::{AggregateTypeList, AggregateType};
use convert_case::{Case, Casing};
use maybe_variadic_type::MaybeNamedVariadicTypeList;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    spanned::Spanned,
    ImplItemConst, ImplItemMethod, ImplItemType, ItemFn, ItemImpl, Path, Type,
    Expr,
};

// We support only 32 tuples...
const ARG_NAMES: [&str; 32] = [
    "arg_one",
    "arg_two",
    "arg_three",
    "arg_four",
    "arg_five",
    "arg_six",
    "arg_seven",
    "arg_eight",
    "arg_nine",
    "arg_ten",
    "arg_eleven",
    "arg_twelve",
    "arg_thirteen",
    "arg_fourteen",
    "arg_fifteen",
    "arg_sixteen",
    "arg_seventeen",
    "arg_eighteen",
    "arg_nineteen",
    "arg_twenty",
    "arg_twenty_one",
    "arg_twenty_two",
    "arg_twenty_three",
    "arg_twenty_four",
    "arg_twenty_five",
    "arg_twenty_six",
    "arg_twenty_seven",
    "arg_twenty_eight",
    "arg_twenty_nine",
    "arg_thirty",
    "arg_thirty_one",
    "arg_thirty_two",
];

/** A parsed `#[pg_aggregate]` item.
*/
#[derive(Debug, Clone)]
pub struct PgAggregate {
    item_impl: ItemImpl,
    name: Expr,
    pg_externs: Vec<ItemFn>,
    // Note these should not be considered *writable*, they're snapshots from construction.
    type_args: MaybeNamedVariadicTypeList,
    type_order_by: Option<AggregateTypeList>,
    type_moving_state: Option<syn::Type>,
    type_stype: AggregateType,
    const_parallel: Option<syn::Expr>,
    const_finalize_modify: Option<syn::Expr>,
    const_moving_finalize_modify: Option<syn::Expr>,
    const_initial_condition: Option<String>,
    const_sort_operator: Option<String>,
    const_moving_intial_condition: Option<String>,
    fn_state: Ident,
    fn_finalize: Option<Ident>,
    fn_combine: Option<Ident>,
    fn_serial: Option<Ident>,
    fn_deserial: Option<Ident>,
    fn_moving_state: Option<Ident>,
    fn_moving_state_inverse: Option<Ident>,
    fn_moving_finalize: Option<Ident>,
    hypothetical: bool,
}

impl PgAggregate {
    pub fn new(mut item_impl: ItemImpl) -> Result<Self, syn::Error> {
        let target_path = get_target_path(&item_impl)?;
        let target_ident = get_target_ident(&target_path)?;
        let snake_case_target_ident = Ident::new(
            &target_ident.to_string().to_case(Case::Snake),
            target_ident.span(),
        );
        let mut pg_externs = Vec::default();
        // We want to avoid having multiple borrows, so we take a snapshot to scan from,
        // and mutate the actual one.
        let item_impl_snapshot = item_impl.clone();

        if let Some((_, ref path, _)) = item_impl.trait_ {
            // TODO: Consider checking the path if there is more than one segment to make sure it's pgx.
            if let Some(last) = path.segments.last() {
                if last.ident.to_string() != "Aggregate" {
                    return Err(syn::Error::new(
                        last.ident.span(),
                        "`#[pg_aggregate]` only works with the `Aggregate` trait.",
                    ));
                }
            }
        }

        let name = match get_impl_const_by_name(&item_impl_snapshot, "NAME") {
            Some(item_const) => match item_const.expr {
                syn::Expr::Lit(ref expr) => if let syn::Lit::Str(_) = expr.lit {
                    item_const.expr.clone()
                } else {
                    panic!(
                        "`NAME` must be a `&'static str` for Aggregate implementations."
                    )
                },
                _ => panic!("`NAME` must be a `&'static str` for Aggregate implementations.")
            }
            None => {
                item_impl.items.push(parse_quote! {
                    const NAME: &'static str = stringify!(Self);
                });
                parse_quote! {
                    stringify!(#target_ident)
                }
            },
        };

        // `State` is an optional value, we default to `Self`.
        let type_state = get_impl_type_by_name(&item_impl_snapshot, "State");
        let _type_state_value = type_state.map(|v| v.ty.clone());
        
        let type_state_without_self = if let Some(inner) = type_state {
            let mut remapped = inner.ty.clone();
            remap_self_to_target(&mut remapped, &target_ident);
            remapped
        } else {
            item_impl.items.push(parse_quote! {
                type State = Self;
            });
            let mut remapped = parse_quote!(Self);
            remap_self_to_target(&mut remapped, &target_ident);
            remapped
        };
        let type_stype = AggregateType { ty: type_state_without_self.clone(), };

        // `MovingState` is an optional value, we default to nothing.
        let type_moving_state = get_impl_type_by_name(&item_impl_snapshot, "MovingState");
        let type_moving_state_value = type_moving_state.map(|v| v.ty.clone());
        if type_moving_state.is_none() {
            item_impl.items.push(parse_quote! {
                type MovingState = ();
            })
        }

        // `OrderBy` is an optional value, we default to nothing.
        let type_order_by = get_impl_type_by_name(&item_impl_snapshot, "OrderBy");
        let type_order_by_value = type_order_by
            .map(|v| AggregateTypeList::new(v.ty.clone()))
            .transpose()?;
        if type_order_by.is_none() {
            item_impl.items.push(parse_quote! {
                type OrderBy = ();
            })
        }

        // `Args` is an optional value, we default to nothing.
        let type_args = get_impl_type_by_name(&item_impl_snapshot, "Args").ok_or_else(|| {
            syn::Error::new(
                item_impl_snapshot.span(),
                "`#[pg_aggregate]` requires the `Args` type defined.",
            )
        })?;
        let type_args_value = MaybeNamedVariadicTypeList::new(type_args.ty.clone())?;

        // `Finalize` is an optional value, we default to nothing.
        let type_finalize = get_impl_type_by_name(&item_impl_snapshot, "Finalize");
        if type_finalize.is_none() {
            item_impl.items.push(parse_quote! {
                type Finalize = ();
            })
        }

        let fn_state = get_impl_func_by_name(&item_impl_snapshot, "state");
        
        let fn_state_name = if let Some(found) = fn_state {
            let fn_name = Ident::new(
                &format!("{}_state", snake_case_target_ident),
                found.sig.ident.span(),
            );
            let pg_extern_attr = pg_extern_attr(found);
            let args = type_args_value
                .found
                .iter()
                .map(|x| (x.name.clone(), x.ty.clone()))
                .collect::<Vec<_>>();
            let arg_names = ARG_NAMES[0..args.len()]
                .iter()
                .zip(args.iter())
                .map(|(default_name, (custom_name, _ty))| 
                    Ident::new(&custom_name.clone().unwrap_or_else(|| default_name.to_string()), fn_state.span())
                ).collect::<Vec<_>>();
            let args_with_names = args.iter().zip(arg_names.iter()).map(|(arg, name)| {
                let arg_ty = &arg.1;
                quote! {
                    #name: #arg_ty
                }
            });

            pg_externs.push(parse_quote! {
                #[allow(non_snake_case, clippy::too_many_arguments)]
                #pg_extern_attr
                fn #fn_name(this: #type_state_without_self, #(#args_with_names),*, fcinfo: pgx::pg_sys::FunctionCallInfo) -> #type_state_without_self {
                    <#target_path as pgx::Aggregate>::in_memory_context(
                        fcinfo,
                        move |_context| <#target_path as pgx::Aggregate>::state(this, (#(#arg_names),*), fcinfo)
                    )
                }
            });
            fn_name
        } else {
            return Err(syn::Error::new(
                item_impl.span(),
                "Aggregate implementation must include state function.",
            ));
        };

        let fn_combine = get_impl_func_by_name(&item_impl_snapshot, "combine");
        let fn_combine_name = if let Some(found) = fn_combine {
            let fn_name = Ident::new(
                &format!("{}_combine", snake_case_target_ident),
                found.sig.ident.span(),
            );
            let pg_extern_attr = pg_extern_attr(found);
            pg_externs.push(parse_quote! {
                #[allow(non_snake_case, clippy::too_many_arguments)]
                #pg_extern_attr
                fn #fn_name(this: #type_state_without_self, v: #type_state_without_self, fcinfo: pgx::pg_sys::FunctionCallInfo) -> #type_state_without_self {
                    <#target_path as pgx::Aggregate>::in_memory_context(
                        fcinfo,
                        move |_context| <#target_path as pgx::Aggregate>::combine(this, v, fcinfo)
                    )
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn combine(current: #type_state_without_self, _other: #type_state_without_self, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> #type_state_without_self {
                    unimplemented!("Call to combine on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_finalize = get_impl_func_by_name(&item_impl_snapshot, "finalize");
        let fn_finalize_name = if let Some(found) = fn_finalize {
            let fn_name = Ident::new(
                &format!("{}_finalize", snake_case_target_ident),
                found.sig.ident.span(),
            );
            let pg_extern_attr = pg_extern_attr(found);
            pg_externs.push(parse_quote! {
                #[allow(non_snake_case, clippy::too_many_arguments)]
                #pg_extern_attr
                fn #fn_name(this: #type_state_without_self, fcinfo: pgx::pg_sys::FunctionCallInfo) -> <#target_path as pgx::Aggregate>::Finalize {
                    <#target_path as pgx::Aggregate>::in_memory_context(
                        fcinfo,
                        move |_context| <#target_path as pgx::Aggregate>::finalize(this, fcinfo)
                    )
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn finalize(current: #type_state_without_self, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> Self::Finalize {
                    unimplemented!("Call to finalize on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_serial = get_impl_func_by_name(&item_impl_snapshot, "serial");
        let fn_serial_name = if let Some(found) = fn_serial {
            let fn_name = Ident::new(
                &format!("{}_serial", snake_case_target_ident),
                found.sig.ident.span(),
            );
            let pg_extern_attr = pg_extern_attr(found);
            pg_externs.push(parse_quote! {
                #[allow(non_snake_case, clippy::too_many_arguments)]
                #pg_extern_attr
                fn #fn_name(this: #type_state_without_self, fcinfo: pgx::pg_sys::FunctionCallInfo) -> Vec<u8> {
                    <#target_path as pgx::Aggregate>::in_memory_context(
                        fcinfo,
                        move |_context| <#target_path as pgx::Aggregate>::serial(this, fcinfo)
                    )
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn serial(current: #type_state_without_self, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> Vec<u8> {
                    unimplemented!("Call to serial on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_deserial = get_impl_func_by_name(&item_impl_snapshot, "deserial");
        let fn_deserial_name = if let Some(found) = fn_deserial {
            let fn_name = Ident::new(
                &format!("{}_deserial", snake_case_target_ident),
                found.sig.ident.span(),
            );
            let pg_extern_attr = pg_extern_attr(found);
            pg_externs.push(parse_quote! {
                #[allow(non_snake_case, clippy::too_many_arguments)]
                #pg_extern_attr
                fn #fn_name(this: #type_state_without_self, buf: Vec<u8>, internal: pgx::PgBox<#type_state_without_self>, fcinfo: pgx::pg_sys::FunctionCallInfo) -> pgx::PgBox<#type_state_without_self> {
                    <#target_path as pgx::Aggregate>::in_memory_context(
                        fcinfo,
                        move |_context| <#target_path as pgx::Aggregate>::deserial(this, buf, internal, fcinfo)
                    )
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn deserial(current: #type_state_without_self, _buf: Vec<u8>, _internal: pgx::PgBox<#type_state_without_self>, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> pgx::PgBox<#type_state_without_self> {
                    unimplemented!("Call to deserial on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_moving_state = get_impl_func_by_name(&item_impl_snapshot, "moving_state");
        let fn_moving_state_name = if let Some(found) = fn_moving_state {
            let fn_name = Ident::new(
                &format!("{}_moving_state", snake_case_target_ident),
                found.sig.ident.span(),
            );
            let pg_extern_attr = pg_extern_attr(found);
            let args = type_args_value
                .found
                .iter()
                .map(|x| x.variadic_ty.clone().unwrap_or(x.ty.clone()))
                .collect::<Vec<_>>();
            let args_with_names = args.iter().zip(ARG_NAMES.iter()).map(|(arg, name)| {
                let name_ident = Ident::new(name, Span::call_site());
                quote! {
                    #name_ident: #arg
                }
            });
            let arg_names = ARG_NAMES[0..args.len()]
                .iter()
                .map(|name| Ident::new(name, fn_state.span()));
            pg_externs.push(parse_quote! {
                #[allow(non_snake_case, clippy::too_many_arguments)]
                #pg_extern_attr
                fn #fn_name(
                    mstate: <#target_path as pgx::Aggregate>::MovingState,
                    #(#args_with_names),*,
                    fcinfo: pgx::pg_sys::FunctionCallInfo,
                ) -> <#target_path as pgx::Aggregate>::MovingState {
                    <#target_path as pgx::Aggregate>::in_memory_context(
                        fcinfo,
                        move |_context| <#target_path as pgx::Aggregate>::moving_state(mstate, (#(#arg_names),*), fcinfo)
                    )
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn moving_state(
                    _mstate: <#target_path as pgx::Aggregate>::MovingState,
                    _v: Self::Args,
                    _fcinfo: pgx::pg_sys::FunctionCallInfo,
                ) -> <#target_path as pgx::Aggregate>::MovingState {
                    unimplemented!("Call to moving_state on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_moving_state_inverse =
            get_impl_func_by_name(&item_impl_snapshot, "moving_state_inverse");
        let fn_moving_state_inverse_name = if let Some(found) = fn_moving_state_inverse {
            let fn_name = Ident::new(
                &format!("{}_moving_state_inverse", snake_case_target_ident),
                found.sig.ident.span(),
            );
            let pg_extern_attr = pg_extern_attr(found);
            pg_externs.push(parse_quote! {
                #[allow(non_snake_case, clippy::too_many_arguments)]
                #pg_extern_attr
                fn #fn_name(
                    mstate: <#target_path as pgx::Aggregate>::MovingState,
                    v: <#target_path as pgx::Aggregate>::Args,
                    fcinfo: pgx::pg_sys::FunctionCallInfo,
                ) -> <#target_path as pgx::Aggregate>::MovingState {
                    <#target_path as pgx::Aggregate>::in_memory_context(
                        fcinfo,
                        move |_context| <#target_path as pgx::Aggregate>::moving_state(mstate, v, fcinfo)
                    )
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn moving_state_inverse(
                    _mstate: <#target_path as pgx::Aggregate>::MovingState,
                    _v: Self::Args,
                    _fcinfo: pgx::pg_sys::FunctionCallInfo,
                ) -> <#target_path as pgx::Aggregate>::MovingState {
                    unimplemented!("Call to moving_state on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_moving_finalize = get_impl_func_by_name(&item_impl_snapshot, "moving_finalize");
        let fn_moving_finalize_name = if let Some(found) = fn_moving_finalize {
            let fn_name = Ident::new(
                &format!("{}_moving_finalize", snake_case_target_ident),
                found.sig.ident.span(),
            );
            let pg_extern_attr = pg_extern_attr(found);
            pg_externs.push(parse_quote! {
                #[allow(non_snake_case, clippy::too_many_arguments)]
                #pg_extern_attr
                fn #fn_name(mstate: <#target_path as pgx::Aggregate>::MovingState, fcinfo: pgx::pg_sys::FunctionCallInfo) -> <#target_path as pgx::Aggregate>::Finalize {
                    <#target_path as pgx::Aggregate>::in_memory_context(
                        fcinfo,
                        move |_context| <#target_path as pgx::Aggregate>::moving_finalize(mstate, fcinfo)
                    )
                    
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn moving_finalize(_mstate: Self::MovingState, _fcinfo: pgx::pg_sys::FunctionCallInfo) -> Self::Finalize {
                    unimplemented!("Call to moving_finalize on an aggregate which does not support it.")
                }
            });
            None
        };

        Ok(Self {
            item_impl,
            pg_externs,
            name,
            type_args: type_args_value,
            type_order_by: type_order_by_value,
            type_moving_state: type_moving_state_value,
            type_stype: type_stype,
            const_parallel: get_impl_const_by_name(&item_impl_snapshot, "PARALLEL")
                .map(|x| x.expr.clone()),
            const_finalize_modify: get_impl_const_by_name(&item_impl_snapshot, "FINALIZE_MODIFY")
                .map(|x| x.expr.clone()),
            const_moving_finalize_modify: get_impl_const_by_name(
                &item_impl_snapshot,
                "MOVING_FINALIZE_MODIFY",
            )
            .map(|x| x.expr.clone()),
            const_initial_condition: get_impl_const_by_name(
                &item_impl_snapshot,
                "INITIAL_CONDITION",
            )
            .and_then(get_const_litstr),
            const_sort_operator: get_impl_const_by_name(&item_impl_snapshot, "SORT_OPERATOR")
                .and_then(get_const_litstr),
            const_moving_intial_condition: get_impl_const_by_name(
                &item_impl_snapshot,
                "MOVING_INITIAL_CONDITION",
            )
            .and_then(get_const_litstr),
            fn_state: fn_state_name,
            fn_finalize: fn_finalize_name,
            fn_combine: fn_combine_name,
            fn_serial: fn_serial_name,
            fn_deserial: fn_deserial_name,
            fn_moving_state: fn_moving_state_name,
            fn_moving_state_inverse: fn_moving_state_inverse_name,
            fn_moving_finalize: fn_moving_finalize_name,
            hypothetical: if let Some(value) =
                get_impl_const_by_name(&item_impl_snapshot, "HYPOTHETICAL")
            {
                match &value.expr {
                    syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                        syn::Lit::Bool(lit) => lit.value,
                        _ => return Err(syn::Error::new(value.span(), "`#[pg_aggregate]` required the `HYPOTHETICAL` value to be a literal boolean.")),
                    },
                    _ => return Err(syn::Error::new(value.span(), "`#[pg_aggregate]` required the `HYPOTHETICAL` value to be a literal boolean.")),
                }
            } else {
                false
            },
        })
    }

    fn entity_tokens(&self) -> ItemFn {
        let target_path = get_target_path(&self.item_impl)
            .expect("Expected constructed PgAggregate to have target path.");
        let target_ident = get_target_ident(&target_path)
            .expect("Expected constructed PgAggregate to have target ident.");
        let snake_case_target_ident = Ident::new(
            &target_ident.to_string().to_case(Case::Snake),
            target_ident.span(),
        );
        let sql_graph_entity_fn_name = syn::Ident::new(
            &format!("__pgx_internals_aggregate_{}", snake_case_target_ident),
            target_ident.span(),
        );


        let name = &self.name;
        let type_args_iter = &self.type_args.entity_tokens();
        let type_order_by_iter = self.type_order_by.iter().map(|x| x.entity_tokens());
        let type_moving_state_iter = self.type_moving_state.iter();
        let type_moving_state_string = self.type_moving_state.as_ref().map(|t| { t.to_token_stream().to_string().replace(" ", "") });
        let type_stype = self.type_stype.entity_tokens();
        let const_parallel_iter = self.const_parallel.iter();
        let const_finalize_modify_iter = self.const_finalize_modify.iter();
        let const_moving_finalize_modify_iter = self.const_moving_finalize_modify.iter();
        let const_initial_condition_iter = self.const_initial_condition.iter();
        let const_sort_operator_iter = self.const_sort_operator.iter();
        let const_moving_intial_condition_iter = self.const_moving_intial_condition.iter();
        let hypothetical = self.hypothetical;
        let fn_state = &self.fn_state;
        let fn_finalize_iter = self.fn_finalize.iter();
        let fn_combine_iter = self.fn_combine.iter();
        let fn_serial_iter = self.fn_serial.iter();
        let fn_deserial_iter = self.fn_deserial.iter();
        let fn_moving_state_iter = self.fn_moving_state.iter();
        let fn_moving_state_inverse_iter = self.fn_moving_state_inverse.iter();
        let fn_moving_finalize_iter = self.fn_moving_finalize.iter();

        let entity_item_fn: ItemFn = parse_quote! {
            #[no_mangle]
            pub extern "C" fn #sql_graph_entity_fn_name() -> pgx::datum::sql_entity_graph::SqlGraphEntity {
                let submission = pgx::datum::sql_entity_graph::aggregate::PgAggregateEntity {
                    full_path: core::any::type_name::<#target_ident>(),
                    module_path: module_path!(),
                    file: file!(),
                    line: line!(),
                    name: #name,
                    ty_id: core::any::TypeId::of::<#target_ident>(),
                    args: #type_args_iter,
                    order_by: None#( .unwrap_or(Some(#type_order_by_iter)) )*,
                    stype: #type_stype,
                    sfunc: stringify!(#fn_state),
                    combinefunc: None#( .unwrap_or(Some(stringify!(#fn_combine_iter))) )*,
                    finalfunc: None#( .unwrap_or(Some(stringify!(#fn_finalize_iter))) )*,
                    finalfunc_modify: None#( .unwrap_or(#const_finalize_modify_iter) )*,
                    initcond: None#( .unwrap_or(Some(#const_initial_condition_iter)) )*,
                    serialfunc: None#( .unwrap_or(Some(stringify!(#fn_serial_iter))) )*,
                    deserialfunc: None#( .unwrap_or(Some(stringify!(#fn_deserial_iter))) )*,
                    msfunc: None#( .unwrap_or(Some(stringify!(#fn_moving_state_iter))) )*,
                    minvfunc: None#( .unwrap_or(Some(stringify!(#fn_moving_state_inverse_iter))) )*,
                    mstype: None#( .unwrap_or(Some(pgx::datum::sql_entity_graph::aggregate::AggregateType {
                        ty_source: #type_moving_state_string,
                        ty_id: core::any::TypeId::of::<#type_moving_state_iter>(),
                        full_path: core::any::type_name::<#type_moving_state_iter>(),
                    })) )*,
                    mfinalfunc: None#( .unwrap_or(Some(stringify!(#fn_moving_finalize_iter))) )*,
                    mfinalfunc_modify: None#( .unwrap_or(#const_moving_finalize_modify_iter) )*,
                    minitcond: None#( .unwrap_or(Some(#const_moving_intial_condition_iter)) )*,
                    sortop: None#( .unwrap_or(Some(#const_sort_operator_iter)) )*,
                    parallel: None#( .unwrap_or(#const_parallel_iter) )*,
                    hypothetical: #hypothetical,
                };
                pgx::datum::sql_entity_graph::SqlGraphEntity::Aggregate(submission)
            }
        };
        entity_item_fn
    }
}

impl Parse for PgAggregate {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}

impl ToTokens for PgAggregate {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let entity_fn = self.entity_tokens();
        let impl_item = &self.item_impl;
        let pg_externs = self.pg_externs.iter();
        let inv = quote! {
            #impl_item

            #(#pg_externs)*

            #entity_fn
        };
        tokens.append_all(inv);
    }
}

fn get_target_ident(path: &Path) -> Result<Ident, syn::Error> {
    let last = path.segments.last().ok_or_else(|| {
        syn::Error::new(
            path.span(),
            "`#[pg_aggregate]` only works with types whose path have a final segment.",
        )
    })?;
    Ok(last.ident.clone())
}

fn get_target_path(item_impl: &ItemImpl) -> Result<Path, syn::Error> {
    let target_ident = match &*item_impl.self_ty {
        syn::Type::Path(ref type_path) => {
            let last_segment = type_path.path.segments.last().ok_or_else(|| {
                syn::Error::new(
                    type_path.span(),
                    "`#[pg_aggregate]` only works with types whose path have a final segment.",
                )
            })?;
            if last_segment.ident.to_string() == "PgVarlena" {
                match &last_segment.arguments {
                    syn::PathArguments::AngleBracketed(angled) => {
                        let first = angled.args.first().ok_or_else(|| syn::Error::new(
                            type_path.span(),
                            "`#[pg_aggregate]` only works with `PgVarlena` declarations if they have a type contained.",
                        ))?;
                        match &first {
                            syn::GenericArgument::Type(Type::Path(ty_path)) => ty_path.path.clone(),
                            _ => return Err(syn::Error::new(
                                type_path.span(),
                                "`#[pg_aggregate]` only works with `PgVarlena` declarations if they have a type path contained.",
                            )),
                        }
                    },
                    _ => return Err(syn::Error::new(
                        type_path.span(),
                        "`#[pg_aggregate]` only works with `PgVarlena` declarations if they have a type contained.",
                    )),
                }
            } else {
                type_path.path.clone()
            }
        }
        something_else => {
            return Err(syn::Error::new(
                something_else.span(),
                "`#[pg_aggregate]` only works with types.",
            ))
        }
    };
    Ok(target_ident)
}

fn pg_extern_attr(item: &ImplItemMethod) -> syn::Attribute {
    let mut found = None;
    for attr in item.attrs.iter() {
        match attr.path.segments.last() {
            Some(segment) if segment.ident.to_string() == "pgx" => {
                found = Some(attr.tokens.clone());
                break;
            },
            _ => (),
        };
    };
    match found {
        Some(args) => parse_quote! {
            #[pg_extern #args]
        },
        None => parse_quote! {
            #[pg_extern]
        },
    }
}

fn get_impl_type_by_name<'a>(item_impl: &'a ItemImpl, name: &str) -> Option<&'a ImplItemType> {
    let mut needle = None;
    for impl_item in item_impl.items.iter() {
        match impl_item {
            syn::ImplItem::Type(impl_item_type) => {
                let ident_string = impl_item_type.ident.to_string();
                if ident_string == name {
                    needle = Some(impl_item_type);
                }
            }
            _ => (),
        }
    }
    needle
}

fn get_impl_func_by_name<'a>(item_impl: &'a ItemImpl, name: &str) -> Option<&'a ImplItemMethod> {
    let mut needle = None;
    for impl_item in item_impl.items.iter() {
        match impl_item {
            syn::ImplItem::Method(impl_item_method) => {
                let ident_string = impl_item_method.sig.ident.to_string();
                if ident_string == name {
                    needle = Some(impl_item_method);
                }
            }
            _ => (),
        }
    }
    needle
}

fn get_impl_const_by_name<'a>(item_impl: &'a ItemImpl, name: &str) -> Option<&'a ImplItemConst> {
    let mut needle = None;
    for impl_item in item_impl.items.iter() {
        match impl_item {
            syn::ImplItem::Const(impl_item_const) => {
                let ident_string = impl_item_const.ident.to_string();
                if ident_string == name {
                    needle = Some(impl_item_const);
                }
            }
            _ => (),
        }
    }
    needle
}

fn get_const_litstr<'a>(item: &'a ImplItemConst) -> Option<String> {
    match &item.expr {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Str(lit) => Some(lit.value()),
            _ => None,
        },
        syn::Expr::Call(expr_call) => match &*expr_call.func {
            syn::Expr::Path(expr_path) => {
                if expr_path.path.segments.last()?.ident.to_string() == "Some" {
                    match expr_call.args.first()? {
                        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                            syn::Lit::Str(lit) => Some(lit.value()),
                            _ => None,
                        },
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => panic!("Got {:?}", item.expr),
    }
}

fn remap_self_to_target(ty: &mut syn::Type, target: &syn::Ident) {
    match ty {
        Type::Path(ref mut ty_path) => {
            for segment in ty_path.path.segments.iter_mut() {
                if segment.ident.to_string() == "Self" {
                    segment.ident = target.clone()
                }
                use syn::{PathArguments, GenericArgument};
                match segment.arguments {
                    PathArguments::AngleBracketed(ref mut angle_args) => {
                        for arg in angle_args.args.iter_mut() {
                            match arg {
                                GenericArgument::Type(inner_ty) => remap_self_to_target(inner_ty, target),
                                _ => (),
                            }
                        }
                    },
                    PathArguments::Parenthesized(_) => (),
                    PathArguments::None => (),
                }
            }  
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::PgAggregate;
    use eyre::Result;
    use syn::{parse_quote, ItemImpl};

    #[test]
    fn agg_required_only() -> Result<()> {
        let tokens: ItemImpl = parse_quote! {
            #[pg_aggregate]
            impl Aggregate for DemoAgg {
                type State = PgVarlena<Self>;
                type Args = i32;
                const NAME: &'static str = "DEMO";

                fn state(mut current: Self::State, arg: Self::Args) -> Self::State {
                    todo!()
                }
            }
        };
        // It should not error, as it's valid.
        let agg = PgAggregate::new(tokens);
        assert!(agg.is_ok());
        // It should create 1 extern, the state.
        let agg = agg.unwrap();
        assert_eq!(agg.pg_externs.len(), 1);
        // That extern should be named specifically:
        let extern_fn = &agg.pg_externs[0];
        assert_eq!(extern_fn.sig.ident.to_string(), "demo_agg_state");
        // It should be possible to generate entity tokens.
        let _ = agg.entity_tokens();
        Ok(())
    }

    #[test]
    fn agg_all_options() -> Result<()> {
        let tokens: ItemImpl = parse_quote! {
            #[pg_aggregate]
            impl Aggregate for DemoAgg {
                type State = PgVarlena<Self>;
                type Args = i32;
                type OrderBy = i32;
                type MovingState = i32;

                const NAME: &'static str = "DEMO";

                const PARALLEL: Option<ParallelOption> = Some(ParallelOption::Safe);
                const FINALIZE_MODIFY: Option<FinalizeModify> = Some(FinalizeModify::ReadWrite);
                const MOVING_FINALIZE_MODIFY: Option<FinalizeModify> = Some(FinalizeModify::ReadWrite);
                const SORT_OPERATOR: Option<&'static str> = Some("sortop");
                const MOVING_INITIAL_CONDITION: Option<&'static str> = Some("1,1");
                const HYPOTHETICAL: bool = true;

                fn state(current: Self::State, v: Self::Args) -> Self::State {
                    todo!()
                }

                fn finalize(current: Self::State) -> Self::Finalize {
                    todo!()
                }

                fn combine(current: Self::State, _other: Self::State) -> Self::State {
                    todo!()
                }

                fn serial(current: Self::State) -> Vec<u8> {
                    todo!()
                }

                fn deserial(current: Self::State, _buf: Vec<u8>, _internal: PgBox<Self>) -> PgBox<Self> {
                    todo!()
                }

                fn moving_state(_mstate: Self::MovingState, _v: Self::Args) -> Self::MovingState {
                    todo!()
                }

                fn moving_state_inverse(_mstate: Self::MovingState, _v: Self::Args) -> Self::MovingState {
                    todo!()
                }

                fn moving_finalize(_mstate: Self::MovingState) -> Self::Finalize {
                    todo!()
                }
            }
        };
        // It should not error, as it's valid.
        let agg = PgAggregate::new(tokens);
        assert!(agg.is_ok());
        // It should create 8 externs!
        let agg = agg.unwrap();
        assert_eq!(agg.pg_externs.len(), 8);
        // That extern should be named specifically:
        let extern_fn = &agg.pg_externs[0];
        assert_eq!(extern_fn.sig.ident.to_string(), "demo_agg_state");
        // It should be possible to generate entity tokens.
        let _ = agg.entity_tokens();
        Ok(())
    }

    #[test]
    fn agg_missing_required() -> Result<()> {
        // This is not valid as it is missing required types/consts.
        let tokens: ItemImpl = parse_quote! {
            #[pg_aggregate]
            impl Aggregate for IntegerAvgState {
            }
        };
        let agg = PgAggregate::new(tokens);
        assert!(agg.is_err());
        Ok(())
    }
}
