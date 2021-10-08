use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Expr, GenericArgument, ImplItemConst, ImplItemMethod, ImplItemType, ItemFn, ItemImpl, PathArguments, Type, parse::{Parse, ParseStream}, parse_quote, spanned::Spanned};
use syn::{punctuated::Punctuated, Token};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PgAggregateAttrs {
    attrs: Punctuated<PgAggregateAttr, Token![,]>
}

impl Parse for PgAggregateAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse_terminated(PgAggregateAttr::parse)?,
        })
    }
}

impl ToTokens for PgAggregateAttrs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.attrs;
        let quoted = quote! {
            #attrs
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PgAggregateAttr {
    Parallel(syn::TypePath),
    InitialCondition(syn::LitStr),
    MovingInitialCondition(syn::LitStr),
    Hypothetical,
}

impl Parse for PgAggregateAttr {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: syn::Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
            "hypothetical" => Self::Hypothetical,
            "initial_condition" => {
                let condition = input.parse()?;
                Self::InitialCondition(condition)
            },
            "moving_initial_condition" => {
                let condition = input.parse()?;
                Self::MovingInitialCondition(condition)
            },
            "parallel" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::TypePath = input.parse()?;
                Self::Parallel(literal)
            },
            _ => return Err(syn::Error::new(input.span(), "Recieved unknown `pg_aggregate` attr.")),
        };
        Ok(found)
    }
}

impl ToTokens for PgAggregateAttr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = quote! {
            #self
        };
        tokens.append_all(quoted);
    }
}

/** A parsed `#[pg_aggregate]` item.
*/
#[derive(Debug, Clone)]
pub struct PgAggregate {
    // Options relevant to the aggregate's final implementation or SQL generation.
    aggregate_attrs: Option<PgAggregateAttrs>,
    item_impl: ItemImpl,
    pg_externs: Vec<ItemFn>,
    // Note these should not be considered *writable*, they're snapshots from construction.
    type_args: MaybeVariadicTypeList,
    type_order_by: Option<AggregateTypeList>,
    type_finalize: Option<syn::Type>,
    type_moving_state: Option<syn::Type>,
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
    pub fn new(
        mut item_impl: ItemImpl,
    ) -> Result<Self, syn::Error> {
        let target_ident = get_target_ident(&item_impl)?;
        let mut pg_externs = Vec::default(); 
        // We want to avoid having multiple borrows, so we take a snapshot to scan from,
        // and mutate the actual one.
        let item_impl_snapshot = item_impl.clone();
        
        if let Some((_, ref path, _)) = item_impl.trait_ {
            // TODO: Consider checking the path if there is more than one segment to make sure it's pgx.
            if let Some(last) = path.segments.last() {
                if last.ident.to_string() != "Aggregate" {
                    return Err(syn::Error::new(last.ident.span(), "`#[pg_aggregate]` only works with the `Aggregate` trait."))
                }
            }
        }

        let mut aggregate_attrs = None;
        for attr in item_impl.attrs.clone() {
            // TODO: Consider checking the path if there is more than one segment to make sure it's pgx.
            let attr_path = attr.path.segments.last();
            if let Some(candidate_path) = attr_path {
                if candidate_path.ident.to_string() == "pg_aggregate" {
                    let parsed: PgAggregateAttrs = syn::parse2(attr.tokens)?;
                    aggregate_attrs = Some(parsed);
                }
            }
        }

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
        let type_order_by_value = type_order_by.map(|v| AggregateTypeList::new(v.ty.clone()))
            .transpose()?;
        if type_order_by.is_none() {
            item_impl.items.push(parse_quote! {
                type OrderBy = ();
            })
        }

        // `Args` is an optional value, we default to nothing.
        let type_args = get_impl_type_by_name(&item_impl_snapshot, "OrderBy").ok_or_else(||
            syn::Error::new(item_impl_snapshot.span(), "`#[pg_aggregate]` requires the `Args` type defined.")
        )?;
        let type_args_value = MaybeVariadicTypeList::new(type_args.ty.clone())?;

        // `Finalize` is an optional value, we default to nothing.
        let type_finalize = get_impl_type_by_name(&item_impl_snapshot, "Finalize");
        let type_finalize_value = type_finalize.map(|v| v.ty.clone());
        if type_finalize.is_none() {
            item_impl.items.push(parse_quote! {
                type Finalize = ();
            })
        }

        let fn_state = get_impl_func_by_name(&item_impl_snapshot, "state");
        let fn_state_name = if let Some(found) = fn_state {
            let fn_name = Ident::new(&format!("{}_state", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(this: #target_ident, v: <#target_ident as pgx::Aggregate>::Args) -> #target_ident {
                    this.state(v)
                }
            });
            fn_name
        } else {
            return Err(syn::Error::new(item_impl.span(), "Aggregate implementation must include state function."))
        };

        let fn_combine = get_impl_func_by_name(&item_impl_snapshot, "combine");
        let fn_combine_name = if let Some(found) = fn_combine {
            let fn_name = Ident::new(&format!("{}_combine", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(this: #target_ident, v: #target_ident) -> #target_ident {
                    this.combine(v)
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn combine(&self, _other: Self) -> Self {
                    unimplemented!("Call to combine on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_finalize = get_impl_func_by_name(&item_impl_snapshot, "finalize");
        let fn_finalize_name = if let Some(found) = fn_finalize {
            let fn_name = Ident::new(&format!("{}_finalize", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(this: #target_ident) -> <#target_ident as pgx::Aggregate>::Finalize {
                    this.finalize()
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn finalize(&self) -> Self::Finalize {
                    unimplemented!("Call to finalize on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_serial = get_impl_func_by_name(&item_impl_snapshot, "serial");
        let fn_serial_name = if let Some(found) = fn_serial {
            let fn_name = Ident::new(&format!("{}_serial", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(this: #target_ident) -> Vec<u8> {
                    this.serial()
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn serial(&self) -> Vec<u8> {
                    unimplemented!("Call to serial on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_deserial = get_impl_func_by_name(&item_impl_snapshot, "deserial");
        let fn_deserial_name = if let Some(found) = fn_deserial {
            let fn_name = Ident::new(&format!("{}_deserial", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(this: #target_ident, buf: Vec<u8>, internal: pgx::PgBox<#target_ident>) -> pgx::PgBox<#target_ident> {
                    this.deserial(buf, internal)
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn deserial(&self, _buf: Vec<u8>, _internal: pgx::PgBox<Self>) -> pgx::PgBox<Self> {
                    unimplemented!("Call to deserial on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_moving_state = get_impl_func_by_name(&item_impl_snapshot, "moving_state");
        let fn_moving_state_name = if let Some(found) = fn_moving_state {
            let fn_name = Ident::new(&format!("{}_moving_state", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(
                    mstate: <#target_ident as pgx::Aggregate>::MovingState,
                    v: <#target_ident as pgx::Aggregate>::Args,
                ) -> <#target_ident as pgx::Aggregate>::MovingState {
                    <#target_ident as pgx::Aggregate>::moving_state(mstate, v)
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn moving_state(
                    _mstate: <#target_ident as pgx::Aggregate>::MovingState,
                    _v: Self::Args
                ) -> <#target_ident as pgx::Aggregate>::MovingState {
                    unimplemented!("Call to moving_state on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_moving_state_inverse = get_impl_func_by_name(&item_impl_snapshot, "moving_state_inverse");
        let fn_moving_state_inverse_name = if let Some(found) = fn_moving_state_inverse {
            let fn_name = Ident::new(&format!("{}_moving_state_inverse", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(
                    mstate: <#target_ident as pgx::Aggregate>::MovingState,
                    v: <#target_ident as pgx::Aggregate>::Args,
                ) -> <#target_ident as pgx::Aggregate>::MovingState {
                    <#target_ident as pgx::Aggregate>::moving_state(mstate, v)
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn moving_state_inverse(
                    _mstate: <#target_ident as pgx::Aggregate>::MovingState,
                    _v: Self::Args,
                ) -> <#target_ident as pgx::Aggregate>::MovingState {
                    unimplemented!("Call to moving_state on an aggregate which does not support it.")
                }
            });
            None
        };

        let fn_moving_finalize = get_impl_func_by_name(&item_impl_snapshot, "moving_finalize");
        let fn_moving_finalize_name = if let Some(found) = fn_moving_finalize {
            let fn_name = Ident::new(&format!("{}_moving_finalize", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(mstate: <#target_ident as pgx::Aggregate>::MovingState) -> <#target_ident as pgx::Aggregate>::Finalize {
                    <#target_ident as pgx::Aggregate>::moving_finalize(mstate)
                }
            });
            Some(fn_name)
        } else {
            item_impl.items.push(parse_quote! {
                fn moving_finalize(_mstate: Self::MovingState) -> Self::Finalize {
                    unimplemented!("Call to moving_finalize on an aggregate which does not support it.")
                }
            });
            None
        };

        Ok(Self {
            aggregate_attrs,
            item_impl,
            pg_externs,
            type_args: type_args_value,
            type_order_by: type_order_by_value,
            type_finalize: type_finalize_value,
            type_moving_state: type_moving_state_value,
            const_parallel: get_impl_const_by_name(&item_impl_snapshot, "PARALLEL").map(|x| x.expr.clone()),
            const_finalize_modify: get_impl_const_by_name(&item_impl_snapshot, "FINALIZE_MODIFY").map(|x| x.expr.clone()),
            const_moving_finalize_modify: get_impl_const_by_name(&item_impl_snapshot, "MOVING_FINALIZE_MODIFY").map(|x| x.expr.clone()),
            const_initial_condition: get_impl_const_by_name(&item_impl_snapshot, "INITIAL_CONDITION").and_then(get_const_litstr),
            const_sort_operator: get_impl_const_by_name(&item_impl_snapshot, "SORT_OPERATOR").and_then(get_const_litstr),
            const_moving_intial_condition: get_impl_const_by_name(&item_impl_snapshot, "MOVING_INITIAL_CONDITION").and_then(get_const_litstr),
            fn_state: fn_state_name,
            fn_finalize: fn_finalize_name,
            fn_combine: fn_combine_name,
            fn_serial: fn_serial_name,
            fn_deserial: fn_deserial_name,
            fn_moving_state: fn_moving_state_name,
            fn_moving_state_inverse: fn_moving_state_inverse_name,
            fn_moving_finalize: fn_moving_finalize_name,
            hypothetical: if let Some(value) = get_impl_const_by_name(&item_impl_snapshot, "HYPOTHETICAL") {
                match &value.expr {
                    syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                        syn::Lit::Bool(lit) => lit.value,
                        _ => return Err(syn::Error::new(value.span(), "`#[pg_aggregate]` required the `HYPOTHETICAL` value to be a literal boolean.")),
                    },
                    _ => return Err(syn::Error::new(value.span(), "`#[pg_aggregate]` required the `HYPOTHETICAL` value to be a literal boolean.")),
                }
            } else { false },
        })
    }

    fn entity_tokens(&self) -> ItemFn {
        let target_ident = get_target_ident(&self.item_impl)
            .expect("Expected constructed PgAggregate to have target ident.");
        let sql_graph_entity_fn_name = syn::Ident::new(
            &format!("__pgx_internals_aggregate_{}", target_ident),
            target_ident.span(),
        );

        // TODO: Get all the params.
        let name = match get_impl_const_by_name(&self.item_impl, "NAME")
            .expect("`NAME` is a required const for Aggregate implementations.")
            .expr {
                syn::Expr::Lit(ref expr) => if let syn::Lit::Str(ref litstr) = expr.lit {
                    litstr.clone()
                } else {
                    panic!("`NAME: &'static str` is a required const for Aggregate implementations.")
                },
                _ => panic!("`NAME: &'static str` is a required const for Aggregate implementations."),
            };

        let type_args_iter = &self.type_args.entity_tokens();
        let type_order_by_iter = self.type_order_by.iter().map(|x| x.entity_tokens());
        let type_moving_state_iter = self.type_moving_state.iter();
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
                    name: stringify!(#name),
                    ty_id: core::any::TypeId::of::<#target_ident>(),
                    args: #type_args_iter,
                    order_by: None#( .unwrap_or(Some(#type_order_by_iter)) )*,
                    stype: stringify!(#target_ident),
                    sfunc: stringify!(#fn_state),
                    combinefunc: None#( .unwrap_or(Some(stringify!(#fn_combine_iter))) )*,
                    finalfunc: None#( .unwrap_or(Some(stringify!(#fn_finalize_iter))) )*,
                    finalfunc_modify: None#( .unwrap_or(#const_finalize_modify_iter) )*,
                    initcond: None#( .unwrap_or(Some(#const_initial_condition_iter)) )*,
                    serialfunc: None#( .unwrap_or(Some(stringify!(#fn_serial_iter))) )*,
                    deserialfunc: None#( .unwrap_or(Some(stringify!(#fn_deserial_iter))) )*,
                    msfunc: None#( .unwrap_or(Some(stringify!(#fn_moving_state_iter))) )*,
                    minvfunc: None#( .unwrap_or(Some(stringify!(#fn_moving_state_inverse_iter))) )*,
                    mstype: None#( .unwrap_or(Some(stringify!(#type_moving_state_iter))) )*,
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

fn get_target_ident(item_impl: &ItemImpl) -> Result<Ident, syn::Error> {
    let target_ident = match &*item_impl.self_ty {
        syn::Type::Path(ref type_path) => {
            // TODO: Consider checking the path if there is more than one segment to make sure it's pgx.
            let last_segment = type_path.path.segments.last().ok_or_else(|| 
                syn::Error::new(type_path.span(), "`#[pg_aggregate]` only works with types whose path have a final segment.")
            )?;
            last_segment.ident.clone()
        },
        something_else => return Err(syn::Error::new(something_else.span(), "`#[pg_aggregate]` only works with types."))
    };
    Ok(target_ident)
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

fn get_impl_type_by_name<'a>(item_impl: &'a ItemImpl, name: &str) -> Option<&'a ImplItemType> {
    let mut needle = None;
    for impl_item in item_impl.items.iter() {
        match impl_item {
            syn::ImplItem::Type(impl_item_type) => {
                let ident_string = impl_item_type.ident.to_string();
                if ident_string == name {
                    needle = Some(impl_item_type);
                }
            },
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
            },
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
            },
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
        _ => None,
    }
}

#[derive(Debug, Clone)]
struct MaybeVariadicTypeList {
    found: Vec<MaybeVariadicType>,
    original: syn::Type,
}

impl MaybeVariadicTypeList {
    fn new(maybe_type_list: syn::Type) -> Result<Self, syn::Error> {
        match &maybe_type_list {
            Type::Tuple(tuple) => {
                let mut coll = Vec::new();
                for elem in &tuple.elems {
                    let parsed_elem = MaybeVariadicType::new(elem.clone())?;
                    coll.push(parsed_elem);
                }
                Ok(Self {
                    found: coll,
                    original: maybe_type_list,
                })
            },
            ty => Ok(Self {
                found: vec![ MaybeVariadicType::new(ty.clone())?, ],
                original: maybe_type_list,
            })
        }
    }

    fn entity_tokens(&self) -> Expr {
        let found = self.found.iter().map(|x| x.entity_tokens());
        parse_quote! {
            vec![#(#found),*]
        }
    }
}

impl Parse for MaybeVariadicTypeList {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}

impl ToTokens for MaybeVariadicTypeList {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.original.to_tokens(tokens)
    }
}

#[derive(Debug, Clone)]
struct MaybeVariadicType {
    ty: Type,
    /// The inner of a variadic, if it exists.
    variadic_ty: Option<Type>,
}

impl MaybeVariadicType {
    fn new(ty: syn::Type) -> Result<Self, syn::Error> {
        let variadic_ty = match &ty {
            syn::Type::Path(ty_path) => {
                let mut found_pgx = false;
                let mut found_variadic = false;
                // We don't actually have type resolution here, this is a "Best guess".
                for (idx, segment) in ty_path.path.segments.iter().enumerate() {
                    match segment.ident.to_string().as_str() {
                        "pgx" if idx == 1 => found_pgx = true,
                        "Variadic" => found_variadic = true,
                        _ => (),
                    }
                }
                if (ty_path.path.segments.len() == 1 && found_variadic) || (found_pgx && found_variadic) {
                    if let Some(last_seg) = ty_path.path.segments.last() {
                        match &last_seg.arguments {
                            PathArguments::AngleBracketed(angle_args) => {
                                match angle_args.args.last() {
                                    Some(GenericArgument::Type(ty)) => Some(ty.clone()),
                                    _ => None,
                                }
                            },
                            _ => None,
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
        let retval = Self {
            ty,
            variadic_ty,
        };
        Ok(retval)
    }

    fn entity_tokens(&self) -> Expr {
        let ty = self.variadic_ty.as_ref().unwrap_or(&self.ty);
        let variadic = self.variadic_ty.is_some();
        parse_quote! {
            pgx::datum::sql_entity_graph::aggregate::MaybeVariadicAggregateArgType {
                agg_ty: pgx::datum::sql_entity_graph::aggregate::AggregateArgType {
                    ty_source: stringify!(#ty),
                    ty_id: core::any::TypeId::of::<#ty>(),
                    full_path: core::any::type_name::<#ty>(),
                },
                variadic: #variadic,
            }
        }
    }
}

impl ToTokens for MaybeVariadicType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty.to_tokens(tokens)
    }
}

impl Parse for MaybeVariadicType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}


#[derive(Debug, Clone)]
struct AggregateTypeList {
    found: Vec<AggregateType>,
    original: syn::Type,
}

impl AggregateTypeList {
    fn new(maybe_type_list: syn::Type) -> Result<Self, syn::Error> {
        match &maybe_type_list {
            Type::Tuple(tuple) => {
                let mut coll = Vec::new();
                for elem in &tuple.elems {
                    let parsed_elem = AggregateType::new(elem.clone())?;
                    coll.push(parsed_elem);
                }
                Ok(Self {
                    found: coll,
                    original: maybe_type_list,
                })
            },
            ty => Ok(Self {
                found: vec![ AggregateType::new(ty.clone())?, ],
                original: maybe_type_list,
            })
        }
    }

    fn entity_tokens(&self) -> Expr {
        let found = self.found.iter().map(|x| x.entity_tokens());
        parse_quote! {
            vec![#(#found),*]
        }
    }
}

impl Parse for AggregateTypeList {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}

impl ToTokens for AggregateTypeList {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.original.to_tokens(tokens)
    }
}

#[derive(Debug, Clone)]
struct AggregateType {
    ty: Type,
}

impl AggregateType {
    fn new(ty: syn::Type) -> Result<Self, syn::Error> {
        let retval = Self {
            ty,
        };
        Ok(retval)
    }

    fn entity_tokens(&self) -> Expr {
        let ty = &self.ty;
        parse_quote! {
            pgx::datum::sql_entity_graph::aggregate::AggregateArgType {
                ty_source: stringify!(#ty),
                ty_id: core::any::TypeId::of::<#ty>(),
                full_path: core::any::type_name::<#ty>(),
            }
        }
    }
}

impl ToTokens for AggregateType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty.to_tokens(tokens)
    }
}

impl Parse for AggregateType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}