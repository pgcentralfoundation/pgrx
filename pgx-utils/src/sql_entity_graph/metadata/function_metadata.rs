use super::{FunctionMetadataEntity, PhantomDataExt, SqlTranslatable};
use core::{any::TypeId, marker::PhantomData};

pub trait FunctionMetadata<Inputs, Output> {
    fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>>;
    fn retval(&self) -> Option<Box<dyn PhantomDataExt>>;
    fn path(&self) -> &'static str {
        core::any::type_name::<Self>()
    }
    fn entity(&self) -> FunctionMetadataEntity {
        FunctionMetadataEntity {
            arguments: self.arguments().into_iter().map(|v| v.entity()).collect(),
            retval: self.retval().map(|v| v.entity()),
            path: self.path(),
        }
    }
}

impl<Output, Func> FunctionMetadata<(), Output> for Func
where
    for<'a> Func: FnMut() -> Output,
    Output: SqlTranslatable + 'static,
{
    fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
        Vec::new()
    }
    fn retval(&self) -> Option<Box<dyn PhantomDataExt>> {
        Some(Box::new({
            let marker: PhantomData<Output> = PhantomData;
            marker
        }) as Box<dyn PhantomDataExt>)
    }
}

impl<Func> FunctionMetadata<(), ()> for Func
where
    for<'a> Func: FnMut(),
{
    fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
        Vec::new()
    }
    fn retval(&self) -> Option<Box<dyn PhantomDataExt>> {
        None
    }
}

seq_macro::seq!(I in 0..=32 {
    #(
        seq_macro::seq!(N in 0..=I {
            impl<#(Input~N,)* Output, Func> FunctionMetadata<(#(Input~N,)*), Output> for Func
            where
                for<'a> Func: FnMut(#(Input~N,)*) -> Output,
                #(
                    Input~N: SqlTranslatable + 'static,
                )*
                Output: SqlTranslatable + 'static,
            {
                fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
                    let mut vec = Vec::new();
                    #(
                        vec.push(Box::new({
                            let marker: PhantomData<Input~N> = PhantomData;
                            marker
                        }) as Box<dyn PhantomDataExt>);
                    )*
                    vec
                }
                fn retval(&self) -> Option<Box<dyn PhantomDataExt>> {
                    Some(Box::new({
                        let marker: PhantomData<Output> = PhantomData;
                        marker
                    }) as Box<dyn PhantomDataExt>)
                }
            }

            impl<#(Input~N,)* Func> FunctionMetadata<(#(Input~N,)*), ()> for Func
            where
                for<'a> Func: FnMut(#(Input~N,)*),
                #(
                    Input~N: SqlTranslatable + 'static,
                )*
            {
                fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
                    let mut vec = Vec::new();
                    #(
                        vec.push(Box::new({
                            let marker: PhantomData<Input~N> = PhantomData;
                            marker
                        }) as Box<dyn PhantomDataExt>);
                    )*
                    vec
                }
                fn retval(&self) -> Option<Box<dyn PhantomDataExt> >{
                    None
                }
            }
        });
    )*
});
