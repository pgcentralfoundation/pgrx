use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{ImplItemConst, ImplItemMethod, ImplItemType, ItemFn, ItemImpl, parse::{Parse, ParseStream}, parse_quote, spanned::Spanned};
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
    has_type_order_by: bool,
    has_type_finalize: bool,
    has_type_moving_state: bool,
    has_const_parallel: bool,
    has_const_finalize_modify: bool,
    has_const_moving_finalize_modify: bool,
    has_const_initial_condition: bool,
    has_const_sort_operator: bool,
    has_const_moving_intial_condition: bool,
    has_const_hypothetical: bool,
    has_fn_finalize: bool,
    has_fn_combine: bool,
    has_fn_serial: bool,
    has_fn_deserial: bool,
    has_fn_moving_state: bool,
    has_fn_moving_state_inverse: bool,
    has_fn_moving_finalize: bool,
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
        if type_moving_state.is_none() {
            item_impl.items.push(parse_quote! {
                type MovingState = ();
            })
        }

        // `OrderBy` is an optional value, we default to nothing.
        let type_order_by = get_impl_type_by_name(&item_impl_snapshot, "OrderBy");
        if type_order_by.is_none() {
            item_impl.items.push(parse_quote! {
                type OrderBy = ();
            })
        }

        // `Finalize` is an optional value, we default to nothing.
        let type_finalize = get_impl_type_by_name(&item_impl_snapshot, "Finalize");
        if type_finalize.is_none() {
            item_impl.items.push(parse_quote! {
                type Finalize = ();
            })
        }

        let fn_state = get_impl_func_by_name(&item_impl_snapshot, "state");
        if let Some(found) = fn_state {
            let fn_name = Ident::new(&format!("{}_state", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(this: #target_ident, v: <#target_ident as pgx::Aggregate>::Args) -> #target_ident {
                    this.state(v)
                }
            })
        } else {
            return Err(syn::Error::new(item_impl.span(), "Aggregate implementation must include state function."))
        }

        let fn_combine = get_impl_func_by_name(&item_impl_snapshot, "combine");
        if let Some(found) = fn_combine {
            let fn_name = Ident::new(&format!("{}_combine", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(this: #target_ident, v: #target_ident) -> #target_ident {
                    this.combine(v)
                }
            })
        } else {
            item_impl.items.push(parse_quote! {
                fn combine(&self, _other: Self) -> Self {
                    unimplemented!("Call to combine on an aggregate which does not support it.")
                }
            })
        }

        let fn_finalize = get_impl_func_by_name(&item_impl_snapshot, "finalize");
        if let Some(found) = fn_finalize {
            let fn_name = Ident::new(&format!("{}_finalize", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(this: #target_ident) -> <#target_ident as pgx::Aggregate>::Finalize {
                    this.finalize()
                }
            })
        } else {
            item_impl.items.push(parse_quote! {
                fn finalize(&self) -> Self::Finalize {
                    unimplemented!("Call to finalize on an aggregate which does not support it.")
                }
            })
        }

        let fn_serial = get_impl_func_by_name(&item_impl_snapshot, "serial");
        if let Some(found) = fn_serial {
            let fn_name = Ident::new(&format!("{}_serial", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(this: #target_ident) -> Vec<u8> {
                    this.serial()
                }
            })
        } else {
            item_impl.items.push(parse_quote! {
                fn serial(&self) -> Vec<u8> {
                    unimplemented!("Call to serial on an aggregate which does not support it.")
                }
            })
        }

        let fn_deserial = get_impl_func_by_name(&item_impl_snapshot, "deserial");
        if let Some(found) = fn_deserial {
            let fn_name = Ident::new(&format!("{}_deserial", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(this: #target_ident, buf: Vec<u8>, internal: pgx::PgBox<#target_ident>) -> pgx::PgBox<#target_ident> {
                    this.deserial(buf, internal)
                }
            })
        } else {
            item_impl.items.push(parse_quote! {
                fn deserial(&self, _buf: Vec<u8>, _internal: pgx::PgBox<Self>) -> pgx::PgBox<Self> {
                    unimplemented!("Call to deserial on an aggregate which does not support it.")
                }
            })
        }

        let fn_moving_state = get_impl_func_by_name(&item_impl_snapshot, "moving_state");
        if let Some(found) = fn_moving_state {
            let fn_name = Ident::new(&format!("{}_moving_state", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(
                    mstate: <#target_ident as pgx::Aggregate>::MovingState,
                    v: <#target_ident as pgx::Aggregate>::Args,
                ) -> <#target_ident as pgx::Aggregate>::MovingState {
                    <#target_ident as pgx::Aggregate>::moving_state(mstate, v)
                }
            })
        } else {
            item_impl.items.push(parse_quote! {
                fn moving_state(
                    _mstate: <#target_ident as pgx::Aggregate>::MovingState,
                    _v: Self::Args
                ) -> <#target_ident as pgx::Aggregate>::MovingState {
                    unimplemented!("Call to moving_state on an aggregate which does not support it.")
                }
            })
        }

        let fn_moving_state_inverse = get_impl_func_by_name(&item_impl_snapshot, "moving_state_inverse");
        if let Some(found) = fn_moving_state_inverse {
            let fn_name = Ident::new(&format!("{}_moving_state_inverse", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(
                    mstate: <#target_ident as pgx::Aggregate>::MovingState,
                    v: <#target_ident as pgx::Aggregate>::Args,
                ) -> <#target_ident as pgx::Aggregate>::MovingState {
                    <#target_ident as pgx::Aggregate>::moving_state(mstate, v)
                }
            })
        } else {
            item_impl.items.push(parse_quote! {
                fn moving_state_inverse(
                    _mstate: <#target_ident as pgx::Aggregate>::MovingState,
                    _v: Self::Args,
                ) -> <#target_ident as pgx::Aggregate>::MovingState {
                    unimplemented!("Call to moving_state on an aggregate which does not support it.")
                }
            })
        }

        let fn_moving_finalize = get_impl_func_by_name(&item_impl_snapshot, "moving_finalize");
        if let Some(found) = fn_moving_finalize {
            let fn_name = Ident::new(&format!("{}_moving_finalize", target_ident), found.sig.ident.span());
            pg_externs.push(parse_quote! {
                #[pg_extern]
                fn #fn_name(mstate: <#target_ident as pgx::Aggregate>::MovingState) -> <#target_ident as pgx::Aggregate>::Finalize {
                    <#target_ident as pgx::Aggregate>::moving_finalize(mstate)
                }
            })
        } else {
            item_impl.items.push(parse_quote! {
                fn moving_finalize(_mstate: Self::MovingState) -> Self::Finalize {
                    unimplemented!("Call to moving_finalize on an aggregate which does not support it.")
                }
            })
        }

        Ok(Self {
            aggregate_attrs,
            item_impl,
            pg_externs,
            has_type_order_by: type_order_by.is_some(),
            has_type_finalize: type_finalize.is_some(),
            has_type_moving_state: type_moving_state.is_some(),
            has_const_parallel: get_impl_const_by_name(&item_impl_snapshot, "PARALLEL").is_some(),
            has_const_finalize_modify: get_impl_const_by_name(&item_impl_snapshot, "FINALIZE_MODIFY").is_some(),
            has_const_moving_finalize_modify: get_impl_const_by_name(&item_impl_snapshot, "MOVING_FINALIZE_MODIFY").is_some(), // const_moving_finalize_modify.is_some(),
            has_const_initial_condition: get_impl_const_by_name(&item_impl_snapshot, "INITIAL_CONDITION").is_some(),
            has_const_sort_operator: get_impl_const_by_name(&item_impl_snapshot, "SORT_OPERATOR").is_some(),
            has_const_moving_intial_condition: get_impl_const_by_name(&item_impl_snapshot, "MOVING_INITIAL_CONDITION").is_some(),
            has_const_hypothetical:  get_impl_const_by_name(&item_impl_snapshot, "HYPOTHETICAL").is_some(),
            has_fn_finalize: fn_finalize.is_some(),
            has_fn_combine: fn_combine.is_some(),
            has_fn_serial: fn_serial.is_some(),
            has_fn_deserial: fn_deserial.is_some(),
            has_fn_moving_state: fn_moving_state.is_some(),
            has_fn_moving_state_inverse: fn_moving_state_inverse.is_some(),
            has_fn_moving_finalize: fn_moving_finalize.is_some(),
        })
    }

    fn entity_fn(&self) -> ItemFn {
        let target_ident = get_target_ident(&self.item_impl)
            .expect("Expected constructed PgAggregate to have target ident.");
        let sql_graph_entity_fn_name = syn::Ident::new(
            &format!("__pgx_internals_aggregate_{}", target_ident),
            target_ident.span(),
        );

        // TODO: Get all the params.

        let entity_item_fn: ItemFn = parse_quote! {
            #[no_mangle]
            pub extern "C" fn #sql_graph_entity_fn_name() -> pgx::datum::sql_entity_graph::SqlGraphEntity {
                let submission = pgx::datum::sql_entity_graph::PgAggregateEntity {
                    full_path: core::any::type_name::<#target_ident>(),
                    module_path: module_path!(),
                    file: file!(),
                    line: line!(),
                    ty_id: core::any::TypeId::of::<#target_ident>(),
                    args: &[],
                    order_by: None,
                    stype: "todo",
                    sfunc: "todo",
                    finalfunc: None,
                    finalfunc_modify: None,
                    initcond: None,
                    serialfunc: None,
                    deserialfunc: None,
                    msfunc: None,
                    minvfunc: None,
                    mstype: None,
                    mfinalfunc: None,
                    mfinalfunc_modify: None,
                    minitcond: None,
                    sortop: None,
                    parallel: None,
                    hypothetical: None,
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
        let entity_fn = self.entity_fn();
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